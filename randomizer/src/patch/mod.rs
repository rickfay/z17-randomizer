use std::{array, collections::HashMap, fs, iter, path::Path};
use std::io::{Read, stdin, stdout, Write};

use albw::{course, demo::Timed, flow::FlowMut, Demo, File, Game, IntoBytes, Item, Language, Scene};
use fs_extra::dir::CopyOptions;
use log::{error, info};
use serde::Serialize;
use tempfile::tempdir;
use try_insert_ext::*;
use albw::scene::Arg;

use crate::{Error, Result, Settings};

use self::code::Code;

mod code;
mod flow;
mod scenes;

#[derive(Debug)]
pub struct Patcher {
    game: Game,
    boot: Language,
    rentals: [Item; 9],
    merchant: [Item; 3],
    courses: HashMap<course::Id, Course>,
}

impl Patcher {
    pub fn new(game: Game) -> Result<Self> {
        let boot = game.boot()?;
        Ok(Self {
            game,
            boot,
            rentals: [Item::KeySmall; 9],
            merchant: [Item::KeySmall; 3],
            courses: Default::default(),
        })
    }

    fn course(&mut self, course: course::Id) -> Result<&mut Course> {
        let Self {
            game,
            ref mut courses,
            ..
        } = self;
        courses
            .entry(course)
            .or_try_insert_with(|| {
                game.course(course).language().map(|load| Course {
                    language: load,
                    scenes: Default::default(),
                })
            })
            .map_err(Into::into)
    }

    fn scene(&mut self, course: course::Id, stage: u16) -> Result<&mut Scene> {
        let Self {
            game,
            ref mut courses,
            ..
        } = self;
        courses
            .entry(course)
            .or_try_insert_with(|| {
                game.course(course).language().map(|load| Course {
                    language: load,
                    scenes: Default::default(),
                })
            })?
            .scenes
            .entry(stage)
            .or_try_insert_with(|| game.course(course).scene(stage))
            .map_err(Into::into)
    }

    fn language<C>(&mut self, course: C) -> Result<&mut Language>
        where
            C: Into<Option<course::Id>>,
    {
        Ok(if let Some(course) = course.into() {
            &mut self.course(course)?.language
        } else {
            &mut self.boot
        })
    }

    fn flow<C>(&mut self, course: C) -> Result<albw::language::LoadedMut<FlowMut>>
        where
            C: Into<Option<course::Id>>,
    {
        Ok(self.language(course)?.flow_mut())
    }

    fn parse_args(&mut self, course: course::Id, stage: u16, unq: u16) -> &mut Arg {
        self.scene(course, stage).unwrap()
            .stage_mut()
            .get_mut()
            .get_mut(unq)
            .ok_or_else(|| {
                Error::game(format!(
                    "{}{} [{}] not found",
                    course.as_str(),
                    stage + 1,
                    unq
                ))
            }).unwrap().arg_mut()
    }

    fn apply(&mut self, patch: Patch, item: Item) -> Result<()> {
        match patch {
            Patch::Chest { course, stage, unq } => {
                self.parse_args(course, stage, unq).0 = item as i32;
            }
            Patch::Heart { course, scene, unq } |
            Patch::Key { course, scene, unq } => {
                self.parse_args(course, scene, unq).1 = item as i32;
            }
            // Patch::Maiamai { course, scene, unq } => {
            //     self.parse_args(course, scene, unq).2 = item as i32;
            // }
            Patch::Event {
                course,
                name,
                index,
            } => {
                self.flow(course)?
                    .get_mut(name)
                    .ok_or_else(|| Error::game("File not found."))??
                    .get_mut()
                    .get_mut(index)
                    .ok_or_else(|| {
                        Error::game(format!(
                            "{}/{} [{}] not found",
                            course.as_ref().map(course::Id::as_str).unwrap_or("Boot"),
                            name,
                            index
                        ))
                    })?
                    .into_action()
                    .ok_or_else(|| Error::game("Not an action."))?
                    .set_value(item as u32);
            }
            Patch::Shop(Shop::Ravio(index)) => {
                self.rentals[index as usize] = item;
            }
            Patch::Shop(Shop::Merchant(index)) => {
                self.merchant[index as usize] = item;
            }
            // Patch::Multi(patches) => {
            //     for patch in patches {
            //         self.apply(patch, item)?;
            //     }
            // }
            Patch::None => {}
        }
        Ok(())
    }

    pub fn prepare(mut self, settings: &Settings) -> Result<Patches> {
        let mut item_actors = HashMap::new();
        for (item, get_item) in self.game.get_item() {
            let name = get_item.actor_name()?;
            let mut actor = get_item.actor(&self.game)?;
            actor.rename(format!("World/Actor/{}.bch", name));
            item_actors.insert(item, actor);
        }
        // Add chest to pedestal scene
        let chest = self
            .scene(course::Id::DungeonHera, 0)?
            .actors()
            .get("TreasureBoxS")?;
        self.scene(course::Id::FieldLight, 33)?
            .actors_mut()
            .add(chest)?;
        scenes::apply(&mut self, settings)?;
        let free = self.rentals[8];
        flow::apply(&mut self, free)?;
        {
            let Self {
                ref rentals,
                ref merchant,
                ref mut courses,
                ..
            } = self;
            let your_house_actors = courses
                .get_mut(&course::Id::IndoorLight)
                .unwrap()
                .scenes
                .get_mut(&0)
                .unwrap()
                .actors_mut();
            for actor in rentals.iter().filter_map(|item| item_actors.get(item)) {
                your_house_actors.add(actor.clone())?;
            }
            let kakariko_actors = courses
                .get_mut(&course::Id::FieldLight)
                .unwrap()
                .scenes
                .get_mut(&15)
                .unwrap()
                .actors_mut();
            kakariko_actors.add(item_actors.get(&merchant[0]).unwrap().clone())?;
            kakariko_actors.add(item_actors.get(&merchant[2]).unwrap().clone())?;
        }
        let code = code::create(&self);
        let Self {
            game,
            boot,
            courses,
            ..
        } = self;
        let mut romfs = Files(vec![]);
        romfs.add(boot.into_archive());
        for (_, Course { language, scenes }) in courses {
            romfs.add(language.into_archive());
            for (_, scene) in scenes {
                let (actors, stage) = scene.into_files();
                if let Some(archive) = actors {
                    romfs.add(archive);
                }
                romfs.add_serialize(stage);
            }
        }
        for cutscene in cutscenes(&game, settings) {
            romfs.add(cutscene?);
        }
        Ok(Patches { game, code, romfs })
    }
}

#[derive(Debug)]
pub struct Course {
    language: Language,
    scenes: HashMap<u16, Scene>,
}

#[derive(Clone, Debug)]
pub enum Patch {
    Chest {
        course: course::Id,
        stage: u16,
        unq: u16,
    },
    Event {
        course: Option<course::Id>,
        name: &'static str,
        index: u16,
    },
    Heart {
        course: course::Id,
        scene: u16,
        unq: u16,
    },
    Key {
        course: course::Id,
        scene: u16,
        unq: u16,
    },
    // Maiamai {
    //     course: course::Id,
    //     scene: u16,
    //     unq: u16,
    // },
    Shop(Shop),
    // Multi(Vec<Patch>),
    None, // Workaround until everything is shufflable
}

impl Patch {
    pub fn apply(self, patcher: &mut Patcher, item: Item) -> Result<()> {
        patcher.apply(self, item)
    }
}

#[derive(Clone, Debug)]
pub enum Shop {
    Ravio(u8),
    Merchant(u8),
}

#[derive(Debug)]
pub struct Patches {
    game: Game,
    code: Code,
    romfs: Files,
}

// FIXME unnecssary duplicate
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

impl Patches {
    pub fn dump<P>(self, path: P) -> Result<()>
        where
            P: AsRef<Path>,
    {
        let temp = tempdir()?;
        let moddir = temp.path().join(&format!("{:016X}", self.game.id()));
        let romfs = moddir.join("romfs");
        fs::create_dir_all(&romfs)?;
        self.code.dump(&moddir, self.game.exheader())?;
        for file in self.romfs.0 {
            file.dump(&romfs)?;
        }
        let path = path.as_ref();
        info!("Writing patch to:               {}\\{:016X}", path.display(), self.game.id());

        match fs_extra::copy_items(
            &[moddir],
            path,
            &CopyOptions {
                overwrite: true,
                ..Default::default()
            },
        ).map_err(Error::io) {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Couldn't write to:              {}", path.display());
                error!("Please check that config.toml points to a valid output destination.");
                pause();
                std::process::exit(1);
            }
        }
    }
}

#[derive(Debug)]
struct Files(Vec<File<Box<[u8]>>>);

impl Files {
    pub fn add<T>(&mut self, file: File<T>)
        where
            T: IntoBytes,
    {
        self.0.push(file.into_bytes());
    }

    pub fn add_serialize<T>(&mut self, file: File<T>)
        where
            T: Serialize,
    {
        self.0.push(file.serialize());
    }
}

/// Removes extraneous events from all important cutscenes.
fn cutscenes<'game, 'settings>(
    game: &'game Game,
    settings: &'settings Settings,
) -> impl Iterator<Item=Result<File<Demo>>> + 'game {

    let Settings { logic, options, .. } = settings.clone();
    let early = iter::once_with(move || {
        let mut opening = game.demo(0)?.map(truncate_cutscene);
        {
            let opening = opening.get_mut();
            for flag in array::IntoIter::new([
                7, 9, 10, // Skip Gulley in prologue
                11, // Fix Hyrule lighting, skip Gulley dialogue at Blacksmith
                20,  // Disable Gulley's callback
                26, // Skip Blacksmith Package Sword
                55, // ?
                84, // Enable Dampe + Seres conversation
                107, // Spawn enemies
                110, // Post Sanctuary
                131, // Suppress Ravio's Gift
                210, // Skip Thanks item
                222, 223, // Skip Hyrule Castle events
                224, // Skip Zelda dialogue
                225, // Correct field music
                231, // Skip Hyrule Castle events
                232, // Enable Ravio's freebie
                233, // Ravio's Shop fully opened
                //235, // Suppress Ravio's Signs, Huh? Not Interested? text, but also Freebie =\
                236, // Enable Stamina bar
                239, // Ravio Sign Trigger
                241, // Skip Osfala intro
                246, // Skip Irene, make Hyrule Hotfoot appear, spawns certain enemies
                248, // Skip Yuga killing Osfala
                315, // Shop open???
                320, // Shady Guy Trigger
                321, 322, // Skip first Oren cutscenes
                415, // Skip Yuga capturing Zelda
                430, // Fix Chamber of Sages Softlock
                510, // Open Portals, Activate Hyrule Castle Midway
                522, // Hilda Blacksmith Text + get Map Swap icon on lower screen
                523, // Hilda Graveyard Text
                524, // Hilda ??? Text
                525, // Skip Sahasrahla outside Link's House, make Hyrule Hotfoot appear
                542, 543, // Skip Bomb-Shop Man dialogue
                560, // Hilda ??? Text
                599, // Disable Sand Rod return
                600, // Hilda ??? Text
                620, // Hilda ??? Text
                640, // Hilda ??? Text
                // 828 // Seems (?) identical to 829/830. This flag is being repurposed to control the Sanctuary doors.
                829, // Respawn in Ravio's Shop after visiting Lorule.
                899, // Enable Quick Equip
                902, // StreetPass Tree
                906, // Monster Guts
                907, // Monster Tail
                908, // Monster Horn
                920, // Link's House Weather Vane
                940, // Vacant House Weather Vane
                950, // Maiamai
                960, // Blacksmith's Wife
                965, // Suppress Energy Potion
            ]) {
                opening.add_event_flag(flag);
            }

            if logic.swordless_mode {
                opening.add_event_flag(410); // Tear down Barrier in Swordless Mode
            }

            if options.night_mode {
                opening.add_event_flag(964); // Night Mode
            }
        }

        Ok(opening)
    })
        .chain((1..4).map(move |i| Ok(game.demo(i)?.map(truncate_cutscene))));
    let late = iter::once_with(move || {
        let mut midgame = game.demo(4)?.map(truncate_cutscene);
        {
            let opening = midgame.get_mut();
            for flag in array::IntoIter::new([
                510, // Skip Lorule Blacksmith's Wife dialogue
                524, 560, 600, 620, 640, // Skip Hilda telepathy
            ]) {
                opening.add_event_flag(flag);
            }
        }
        Ok(midgame)
    })
        .chain((5..8).map(move |i| Ok(game.demo(i)?.map(truncate_cutscene))));
    early.chain(late)
}

/// Removes all extraneous events and sets the Finish timestamp to 0.
fn truncate_cutscene(mut demo: Demo) -> Demo {
    demo.retain(Timed::is_known);
    demo.finish_mut().set_timestamp(0);
    demo
}

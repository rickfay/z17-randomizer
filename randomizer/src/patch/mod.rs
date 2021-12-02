use std::{array, collections::HashMap, fs, iter, path::Path};

use albw::{
    course, demo::Timed, flow::FlowMut, Demo, File, Game, IntoBytes, Item, Language, Scene,
};
use fs_extra::dir::CopyOptions;
use log::info;
use serde::Serialize;
use tempfile::tempdir;
use try_insert_ext::*;

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
    hash: u32,
}

impl Patcher {
    pub fn new(game: Game, hash: u32) -> Result<Self> {
        let boot = game.boot()?;
        Ok(Self {
            game,
            boot,
            rentals: [Item::KeySmall; 9],
            merchant: [Item::KeySmall; 3],
            courses: Default::default(),
            hash,
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

    fn apply(&mut self, patch: Patch, item: Item) -> Result<()> {
        match patch {
            Patch::Chest { course, stage, unq } => {
                self.scene(course, stage)?
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
                    })?
                    .arg_mut()
                    .0 = item as i32;
            }
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
            Patch::Heart { course, scene, unq } | Patch::Key { course, scene, unq } => {
                self.scene(course, scene)?
                    .stage_mut()
                    .get_mut()
                    .get_mut(unq)
                    .ok_or_else(|| {
                        Error::game(format!(
                            "{}{} [{}] not found",
                            course.as_str(),
                            scene + 1,
                            unq
                        ))
                    })?
                    .arg_mut()
                    .1 = item as i32;
            }
            Patch::Shop(Shop::Ravio(index)) => {
                self.rentals[index as usize] = item;
            }
            Patch::Shop(Shop::Merchant(index)) => {
                self.merchant[index as usize] = item;
            }
            Patch::Multi(patches) => {
                for patch in patches {
                    self.apply(patch, item)?;
                }
            }
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
        let code = code::create(&self, settings);
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
    Shop(Shop),
    Multi(Vec<Patch>),
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
        info!("Copying files to {}", path.display());
        fs_extra::copy_items(
            &[moddir],
            path,
            &CopyOptions {
                overwrite: true,
                ..Default::default()
            },
        )
        .map_err(Error::io)?;
        Ok(())
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
) -> impl Iterator<Item = Result<File<Demo>>> + 'game {
    let open = settings.behavior.open;
    let Settings { items, .. } = settings.clone();
    let early = iter::once_with(move || {
        let mut opening = game.demo(0)?.map(truncate_cutscene);
        {
            let opening = opening.get_mut();
            for flag in array::IntoIter::new([
                7, 9, 10, 11, // Skip Gulley in prologue
                55, // ?
                222, 223, 231, // Skip Hyrule Castle events
                236, // Enable Stamina bar
                241, // Skip Osfala intro
                248, // Skip Yuga killing Osfala
                315, // Shop open???
                321, 322, // Skip first Oren cutscenes
                415, // Skip Yuga capturing Zelda
                542, 543, // Skip Bomb-Shop Man dialogue
                599, // Disable Sand Rod return
                899, // Enable Quick Equip
            ]) {
                opening.add_event_flag(flag);
            }
            if open {
                for flag in array::IntoIter::new([
                    20,  // Disable Gulley's callback
                    107, // Spawn enemies
                    110, // Post Sanctuary
                    224, // Skip Zelda dialogue
                    225, // Correct field music
                    232, // Enable Ravio's freebie
                ]) {
                    opening.add_event_flag(flag);
                }
            }
            if items.captains_sword.is_skipped() {
                opening.add_event_flag(26); // Got delivery sword
                opening.add_event_flag(84); // Enable Seres/Dampe conversation
            }
            if items.first_bracelet.is_skipped() {
                opening.add_event_flag(210); // Skip Ravio giving bracelet
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

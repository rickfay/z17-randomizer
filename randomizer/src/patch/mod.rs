use std::{collections::HashMap, fs, iter, path::Path};

use fs_extra::dir::CopyOptions;
use game::{
    world::{self, Location, Shop},
    Course::*,
    Item,
};
use log::{debug, error, info};
use modd::{ItemExt, Settings};
use path_absolutize::*;
use rom::{
    demo::Timed,
    flow::FlowMut,
    scene::{Arg, Obj, Rail, SceneMeta},
    Demo, File, IntoBytes, Language, Rom, Scene,
};
use serde::Serialize;
use tempfile::tempdir;
use try_insert_ext::EntryInsertExt;

use crate::{patch::util::*, Error, Layout, Result, SeedInfo};

use code::Code;

mod code;
mod flow;
mod maps;
mod messages;
mod prizes;
mod scenes;
pub mod util;

#[non_exhaustive]
pub struct DungeonPrizes {
    ep_prize: Item,
    hg_prize: Item,
    th_prize: Item,
    hc_prize: Item,
    pd_prize: Item,
    sp_prize: Item,
    sw_prize: Item,
    tt_prize: Item,
    tr_prize: Item,
    dp_prize: Item,
    ir_prize: Item,
}

#[derive(Debug)]
pub struct Patcher {
    game: Rom,
    boot: Language,
    rentals: [Item; 9],
    merchant: [Item; 3],
    courses: HashMap<game::Course, Course>,
}

impl Patcher {
    pub fn new(game: Rom) -> Result<Self> {
        let boot = game.boot()?;
        Ok(Self {
            game,
            boot,
            rentals: [Item::KeySmall; 9],
            merchant: [Item::KeySmall; 3],
            courses: Default::default(),
        })
    }

    fn add_obj(&mut self, id: game::Course, stage_index: u16, obj: Obj) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_obj(obj);
    }

    fn add_rail(&mut self, id: game::Course, stage_index: u16, rail: Rail) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_rail(rail);
    }

    #[allow(unused)]
    fn add_system(&mut self, id: game::Course, stage_index: u16, obj: Obj) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_system(obj);
    }

    fn modify_objs(
        &mut self, id: game::Course, stage_index: u16, actions: &[(u16, Box<dyn Fn(&mut Obj)>)],
    ) {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions {
            action(
                stage
                    .get_obj_mut(*unq)
                    .ok_or_else(|| {
                        Error::new(format!(
                            "Could not find [Objs] UNQ {} in {}{}",
                            unq,
                            id.as_ref(),
                            stage_index
                        ))
                    })
                    .unwrap(),
            );
        }
    }

    fn modify_rails(
        &mut self, id: game::Course, stage_index: u16, actions: &[(u16, Box<dyn Fn(&mut Rail)>)],
    ) {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions {
            action(
                stage
                    .get_rails_mut(*unq)
                    .ok_or_else(|| {
                        Error::new(format!(
                            "Could not find [Rails] UNQ {} in {}{}",
                            unq,
                            id.as_ref(),
                            stage_index
                        ))
                    })
                    .unwrap(),
            );
        }
    }

    fn modify_system(
        &mut self, id: game::Course, stage_index: u16, actions: &[(u16, Box<dyn Fn(&mut Obj)>)],
    ) {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions {
            action(
                stage
                    .get_system_mut(*unq)
                    .ok_or_else(|| {
                        Error::new(format!(
                            "Could not find [System] UNQ {} in {}{}",
                            unq,
                            id.as_ref(),
                            stage_index
                        ))
                    })
                    .unwrap(),
            );
        }
    }

    fn load_course(game: &mut Rom, course: game::Course) -> Course {
        game.course(course)
            .language()
            .map(|load| Course {
                language: load,
                scenes: Default::default(),
                scene_meta: game.course(course).scene_meta(),
            })
            .unwrap()
    }

    fn course(&mut self, course: game::Course) -> Result<&mut Course> {
        let Self { game, ref mut courses, .. } = self;
        Ok(courses.entry(course).or_insert(Self::load_course(game, course)))
    }

    fn scene(&mut self, course: game::Course, stage: u16) -> Result<&mut Scene> {
        let Self { game, ref mut courses, .. } = self;
        courses
            .entry(course)
            .or_insert(Self::load_course(game, course))
            .scenes
            .entry(stage)
            .or_try_insert_with(|| game.course(course).scene(stage))
            .map_err(Into::into)
    }

    fn scene_meta(&mut self, course: game::Course) -> &mut SceneMeta {
        let Self { game, ref mut courses, .. } = self;
        let Course { ref mut scene_meta, .. } =
            courses.entry(course).or_insert(Self::load_course(game, course));
        scene_meta.as_mut().unwrap()
    }

    fn update(&mut self, (course, file): (game::Course, File<Vec<u8>>)) -> Result<()> {
        self.language(course)?.update(file)?;
        Ok(())
    }

    fn inject_msbf(
        &mut self, course: game::Course, msbf: Option<&(&str, File<Box<[u8]>>)>,
    ) -> Result<()> {
        if let Some((msbf_key, msbf_file)) = msbf {
            self.language(course)?.flow_inject(msbf_key, msbf_file.clone())?;
        }

        Ok(())
    }

    fn language<C>(&mut self, course: C) -> Result<&mut Language>
    where
        C: Into<Option<game::Course>>,
    {
        Ok(if let Some(course) = course.into() {
            &mut self.course(course)?.language
        } else {
            &mut self.boot
        })
    }

    fn flow<C>(&mut self, course: C) -> Result<rom::language::LoadedMut<FlowMut>>
    where
        C: Into<Option<game::Course>>,
    {
        Ok(self.language(course)?.flow_mut())
    }

    fn parse_args(&mut self, course: game::Course, stage: u16, unq: u16) -> &mut Arg {
        self.scene(course, stage)
            .unwrap()
            .stage_mut()
            .get_mut()
            .get_obj_mut(unq)
            .ok_or_else(|| {
                Error::new(format!("{}{} [{}] not found", course.as_ref(), stage + 1, unq))
            })
            .unwrap()
            .arg_mut()
    }

    fn prep_chest(
        &mut self, item: Item, course: game::Course, stage: u16, unq: u16, is_big: bool,
        settings: &Settings,
    ) -> Result<()> {
        // Set contents
        self.parse_args(course, stage, unq).0 = item as i32;

        let small_chest = (35, "TreasureBoxS");
        let large_chest = (34, "TreasureBoxL");

        let chest_data = if settings.options.chest_size_matches_contents {
            if item.goes_in_csmc_large_chest() {
                large_chest
            } else {
                small_chest
            }
        } else if is_big {
            large_chest
        } else {
            small_chest
        };

        // Forcibly set ID
        self.scene(course, stage)
            .unwrap()
            .stage_mut()
            .get_mut()
            .get_obj_mut(unq)
            .unwrap()
            .set_id(chest_data.0);

        // Add Actor if scene doesn't already have it
        if !self.scene(course, stage).unwrap().actors().contains(chest_data.1) {
            debug!("Adding {} to {}{}", chest_data.1, course.as_ref(), stage + 1);
            let actor = self.scene(DungeonHera, 0)?.actors().get_actor_bch(chest_data.1)?;
            self.scene(course, stage).unwrap().actors_mut().add(actor)?;
        }

        Ok(())
    }

    pub fn patch_locations(&mut self, layout: &Layout, settings: &Settings) -> Result<()> {
        let regions = world::dungeons::regions()
            .chain(world::hyrule::regions())
            .chain(world::lorule::regions());
        for locations in regions {
            for (key, location) in locations {
                if let Some(item) = layout.get(&key) {
                    self.apply(location, item, settings)?;
                }
            }
        }
        Ok(())
    }

    fn apply(&mut self, location: Location, item: Item, settings: &Settings) -> Result<()> {
        match location {
            Location::Chest { course, stage, unq } => {
                self.prep_chest(item, course, stage, unq, false, settings)?;
            }
            Location::BigChest { course, stage, unq } => {
                self.prep_chest(item, course, stage, unq, true, settings)?;
            }
            Location::Heart { course, scene, unq }
            | Location::Key { course, scene, unq }
            | Location::SilverRupee { course, scene, unq }
            | Location::GoldRupee { course, scene, unq } => {
                self.parse_args(course, scene, unq).1 = item as i32;
            }
            Location::Maiamai { course, scene, unq } => {
                self.parse_args(course, scene, unq).2 = item as i32;
            }
            Location::Event { course, name, index } => {
                self.flow(course)?
                    .get_mut(name)
                    .ok_or_else(|| Error::new("File not found."))??
                    .get_mut()
                    .get_mut(index)
                    .ok_or_else(|| {
                        Error::new(format!(
                            "{}/{} [{}] not found",
                            course.as_ref().map(AsRef::as_ref).unwrap_or("Boot"),
                            name,
                            index
                        ))
                    })?
                    .into_action()
                    .ok_or_else(|| Error::new("Not an action."))?
                    .set_value(item as u32);
            }
            Location::Shop(Shop::Ravio(index)) => {
                self.rentals[index as usize] = item;
            }
            Location::Shop(Shop::Merchant(index)) => {
                self.merchant[index as usize] = item;
            }
            Location::Multi(patches) => {
                for patch in patches {
                    self.apply(patch, item, settings)?;
                }
            }
            Location::None => {}
        }
        Ok(())
    }

    pub fn prepare(mut self, seed_info: &SeedInfo) -> Result<Patches> {
        let common_archive = self.game.common()?;
        let mut item_actors = HashMap::new();

        for (item, get_item) in self.game.match_items_to_get_items() {
            if Item::SpecialMove.as_ref().eq(&get_item.0) {
                // fixme hacky and gross
                let mut actor = common_archive.get_actor_bch("SwordD")?.clone();
                actor.rename(String::from("World/Actor/SwordD.bch"));
                item_actors.insert(item, actor);
            } else if let Some(mut actor) = get_item.actor(&self.game) {
                actor.rename(format!("World/Actor/{}.bch", get_item.actor_name()?));
                item_actors.insert(item, actor);
            }
        }

        // Add Warp Tiles to scenes for softlock prevention
        let warp_tile = self.scene(DungeonHera, 0)?.actors().get_actor_bch("WarpTile")?;
        self.scene(DungeonWind, 0)?.actors_mut().add(warp_tile.clone())?; // Gales 1F
        self.scene(FieldDark, 19)?.actors_mut().add(warp_tile.clone())?; // Dark Maze
        self.scene(DungeonWater, 1)?.actors_mut().add(warp_tile.clone())?; // Swamp Palace B1
        self.scene(DungeonDokuro, 1)?.actors_mut().add(warp_tile)?; // Skull Woods B2

        // Add Ravio to Hilda's Study to give out Bow of Light Hint
        let hint_ghost = self.scene(IndoorDark, 15)?.actors().get_actor_bch("HintGhost")?;
        self.scene(IndoorDark, 4)?.actors_mut().add(hint_ghost)?;

        // Add Heart Piece actor to vanilla Letter in a Bottle area
        let heart_piece = self.scene(FieldLight, 29)?.actors().get_actor_bch("HeartPiece")?;
        self.scene(FieldLight, 35)?.actors_mut().add(heart_piece)?;

        // Debug stuff
        // let step_switch = self.scene(DungeonDark, 0)?.actors().get_actor_bch("SwitchStep")?;
        // self.scene(DungeonKame, 2)?.actors_mut().add(step_switch.clone())?; // Turtle Rock Boss
        // self.scene(DungeonWind, 2)?.actors_mut().add(step_switch.clone())?; // Gales Boss
        // self.scene(DungeonHera, 0)?.actors_mut().add(step_switch.clone())?; // Hera Boss
        // self.scene(FieldDark, 30)?.actors_mut().add(step_switch.clone())?; // Desert Boss

        // TODO Bow of Light Fix
        //self.scene(course::Id::DungeonWater, 2)?.actors_mut().add(fresco_arrow.clone())?;
        //self.scene(course::Id::IndoorLight, 0)?.actors_mut().add(fresco_arrow.clone())?;

        // just testin'
        // let sarc = jack::open_szs(&self.game, "Archive/ActorProfile.szs");

        let prizes = get_dungeon_prizes(&seed_info.layout);
        let free = self.rentals[8];
        flow::apply(&mut self, free, seed_info.settings)?;
        messages::patch_messages(&mut self, seed_info)?;
        prizes::patch_dungeon_prizes(&mut self, &prizes, seed_info.settings)?;
        maps::patch_maps(&mut self, &prizes)?;
        scenes::patch_byaml_files(&mut self, seed_info.settings)?;

        {
            let Self { ref rentals, ref merchant, ref mut courses, .. } = self;
            let your_house_actors =
                courses.get_mut(&IndoorLight).unwrap().scenes.get_mut(&0).unwrap().actors_mut();
            for actor in rentals.iter().filter_map(|item| item_actors.get(item)) {
                your_house_actors.add(actor.clone())?;
            }
            let kakariko_actors =
                courses.get_mut(&FieldLight).unwrap().scenes.get_mut(&15).unwrap().actors_mut();
            kakariko_actors.add(item_actors.get(&merchant[0]).unwrap().clone())?;
            kakariko_actors.add(item_actors.get(&merchant[2]).unwrap().clone())?;
        }
        let code = code::create(&self, seed_info)?;
        let Self { game, boot, courses, .. } = self;
        let mut romfs = Files(vec![]);

        // Add Actors to Common Archive
        //let mut common = game.common().unwrap();
        //common.add(chest_large)?;
        //common.add(fresco_arrow)?; // Sorta works... but not really...
        //romfs.add(common.into_archive().unwrap());

        romfs.add(boot.into_archive());
        for (_, Course { language, scenes, scene_meta }) in courses {
            romfs.add(language.into_archive());
            if let Some(scene_meta) = scene_meta {
                romfs.add_serialize(scene_meta.into_file());
            }
            for (_, scene) in scenes {
                let (actors, stage) = scene.into_files();
                if let Some(archive) = actors {
                    romfs.add(archive);
                }
                romfs.add_serialize(stage);
            }
        }
        for cutscene in cutscenes(&game, seed_info.settings) {
            romfs.add(cutscene?);
        }
        Ok(Patches { game, code, romfs })
    }
}

#[derive(Debug)]
pub struct Course {
    language: Language,
    scenes: HashMap<u16, Scene>,
    scene_meta: Option<SceneMeta>,
}

#[derive(Debug)]
pub struct Patches {
    game: Rom,
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
        println!();
        info!(
            "Writing Patch Files to:         {}\\{:016X}",
            &path.absolutize()?.display(),
            self.game.id()
        );

        match fs_extra::copy_items(
            &[moddir],
            path,
            &CopyOptions { overwrite: true, ..Default::default() },
        )
        .map_err(|err| Error::new(err.to_string()))
        {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Couldn't write to:              {}", path.display());
                error!("Please check that config.json points to a valid output destination.");
                Err(err)
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
fn cutscenes<'game>(
    game: &'game Rom, settings: &Settings,
) -> impl Iterator<Item = Result<File<Demo>>> + 'game {
    info!("Patching Cutscenes...");
    let Settings { logic, options, .. } = settings.clone();
    let early = iter::once_with(move || {
        let mut opening = game.demo(0)?.map(truncate_cutscene);
        {
            let opening = opening.get_mut();

            for flag in IntoIterator::into_iter([
                7, 9, 10,  // Skip Gulley in prologue
                11,  // Fix Hyrule lighting, skip Gulley dialogue at Blacksmith
                20,  // Disable Gulley's callback
                55,  // ?
                84,  // Enable Dampe + Seres conversation
                107, // Spawn enemies
                110, // Post Sanctuary
                131, // Suppress Ravio's Gift
                210, // Skip Thanks item
                223, // Skip Hyrule Castle Art Gallery Event
                225, // Correct field music
                231, // Skip Hyrule Castle events
                232, // Enable Ravio's freebie
                233, // Ravio's Shop fully opened
                //235, // Suppress Ravio's Signs, Huh? Not Interested? text, but also Freebie =\
                236, // Enable Stamina bar
                239, // Ravio Sign Trigger
                241, // Skip Osfala intro
                //246, // Skip Irene, make Hyrule Hotfoot appear, spawns certain enemies
                248, // Skip Yuga killing Osfala
                //250, // Yuga 1 Defeated
                //251, // Set in Post-EP FieldLight20 cutscene, being used as PoC Flag
                310, // Watched HC Post-EP cutscene, fixes overworld music issues
                315, // Shop open???
                // 320, // Shady Guy Trigger
                321, 322, // Skip first Oren cutscenes
                374, // Fix Post-Gales and Post-Hera music by marking Sahasrahla telepathy as seen
                415, // Skip Yuga capturing Zelda
                430, // Fix Chamber of Sages Softlock
                510, // Open Portals, Activate Hyrule Castle Midway
                522, // Blacksmith Hilda Text, enable Map Swap icon, skip introductory Lorule music
                524, 560, 600, 620, 640, // Skip Hilda Text, enable Lorule overworld music
                525, // Skip Sahasrahla outside Link's House, make Hyrule Hotfoot appear
                // 536, 537, // Gulley Flags
                // 556, 557, // Oren Flags
                // 576, 577, // Seres Flags
                // 596, 597, // Osfala Flags
                // 616, 617, // Rosso Flags
                // 636, 637, // Irene Flags
                // 656, 657, // Impa Flags
                542, 543, // Skip Bomb-Shop Man dialogue
                599, // Disable Sand Rod return
                897, // Big Special Something
                899, // Enable Quick Equip
                902, // StreetPass Tree
                906, // Monster Guts
                907, // Monster Tail
                908, // Monster Horn
                919, // Skip Hint Ghost tutorial
                920, // Link's House Weather Vane
                940, // Vacant House Weather Vane
                950, // Maiamai
                960, // Blacksmith's Wife
                965, // Suppress Energy Potion
            ]) {
                opening.add_event_flag(flag);
            }

            // Enable opening Lorule Castle from start
            if logic.lc_requirement == 0 {
                opening.add_event_flag(670);
            }

            // Night Mode
            if options.night_mode {
                opening.add_event_flag(964);
            }

            if logic.weather_vanes_activated {
                for flag in IntoIterator::into_iter([
                    920, //	Your House Weather Vane
                    921, //	Kakariko Village Weather Vane
                    922, //	Eastern Palace Weather Vane
                    923, //	House of Gales Weather Vane
                    924, //	Tower of Hera Weather Vane
                    925, //	Witch's House Weather Vane
                    926, //	Death Mountain (Hyrule) Weather Vane
                    927, //	Desert Palace Weather Vane
                    928, //	Sanctuary Weather Vane
                    932, //	Skull Woods Weather Vane
                    933, //	Treacherous Tower Weather Vane
                    934, //	Ice Ruins Weather Vane
                    935, //	Lorule Castle Weather Vane
                    936, //	Graveyard Weather Vane
                    937, //	Thieves' Town Weather Vane
                    938, //	Dark Palace Weather Vane
                    939, //	Blacksmith Weather Vane
                    940, //	Vacant House Weather Vane
                    941, //	Misery Mire Weather Vane
                    942, //	Swamp Palace Weather Vane
                    943, //	Turtle Rock Weather Vane
                    944, //	Death Mountain (Lorule) Weather Vane
                ]) {
                    opening.add_event_flag(flag);
                }
            }

            // Swordless Mode - Tear down Barrier at game start
            if logic.swordless_mode {
                opening.add_event_flag(410);
            }

            // Trial's Skip
            // Set flags to auto-complete the trials, advanced LC music, and show doors opening.
            // Intentionally not setting 713 (lower-right square) so door will do check and open itself.
            if logic.skip_trials {
                for flag in
                    IntoIterator::into_iter([710, 711, 712, /*713,*/ 714, 715, 716, 717])
                {
                    opening.add_event_flag(flag);
                }
            }

            // Big Bomb Flower Skip
            // Removes the Big Rock (FieldDark33) to drain the water (CaveDark1)
            if logic.skip_big_bomb_flower {
                opening.add_event_flag(541);
            }

            // Reverse Sage Events
            if !logic.reverse_sage_events {
                opening.add_event_flag(222); // Open Hyrule Castle Front Door
            }
        }

        Ok(opening)
    })
    .chain((1..4).map(move |i| Ok(game.demo(i)?.map(truncate_cutscene))));
    let late = iter::once_with(move || {
        let mut midgame = game.demo(4)?.map(truncate_cutscene);
        {
            let opening = midgame.get_mut();
            for flag in IntoIterator::into_iter([
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

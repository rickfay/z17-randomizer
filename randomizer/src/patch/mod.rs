use std::{array, collections::HashMap, fs, iter, path::Path};
use std::io::{Read, stdin, stdout, Write};

use albw::{demo::Timed, flow::FlowMut, Demo, File, Game, IntoBytes, Item, Language, Scene, byaml, Actor, GetItem};
use fs_extra::dir::CopyOptions;
use log::{error, info};
use serde::Serialize;
use tempfile::tempdir;
use try_insert_ext::*;
use albw::course::Id;
use albw::course::Id::*;
use albw::scene::{Arg, Obj, Rail};
use albw::Item::*;
use albw::language::FlowChart;

use crate::{Error, ItemExt, Layout, LocationInfo, MsbfKey, regions, Result, Settings};
use crate::patch::r#ref::MsbfInfo;
use crate::patch::scenes::is_pendant;

use self::code::Code;

mod code;
mod flow;
mod scenes;
pub mod r#ref;

#[non_exhaustive]
pub struct DungeonPrizes {
    ep_prize: Item,
    hg_prize: Item,
    th_prize: Item,
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
    game: Game,
    boot: Language,
    rentals: [Item; 9],
    merchant: [Item; 3],
    courses: HashMap<Id, Course>,
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

    fn add_obj(&mut self, id: Id, stage_index: u16, obj: Obj) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_obj(obj);
    }

    fn add_rail(&mut self, id: Id, stage_index: u16, rail: Rail) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_rail(rail);
    }

    fn add_system(&mut self, id: Id, stage_index: u16, obj: Obj) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_system(obj);
    }

    fn modify_objs(&mut self, id: Id, stage_index: u16, actions: &[(u16, Box<dyn Fn(&mut Obj)>)])
    {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions {
            action(stage.get_obj_mut(*unq)
                .ok_or_else(|| Error::game(format!("Could not find [Objs] UNQ {} in {}{}", unq, id.as_str(), stage_index)))
                .unwrap());
        }
    }

    fn modify_rails(&mut self, id: Id, stage_index: u16, actions: &[(u16, Box<dyn Fn(&mut Rail)>)])
    {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions {
            action(stage.get_rails_mut(*unq)
                .ok_or_else(|| Error::game(format!("Could not find [Rails] UNQ {} in {}{}", unq, id.as_str(), stage_index)))
                .unwrap());
        }
    }

    fn modify_system(&mut self, id: Id, stage_index: u16, actions: &[(u16, Box<dyn Fn(&mut Obj)>)])
    {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions {
            action(stage.get_system_mut(*unq)
                .ok_or_else(|| Error::game(format!("Could not find [System] UNQ {} in {}{}", unq, id.as_str(), stage_index)))
                .unwrap());
        }
    }

    fn course(&mut self, course: Id) -> Result<&mut Course> {
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

    fn scene(&mut self, course: Id, stage: u16) -> Result<&mut Scene> {
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

    fn inject_msbf(&mut self, course: Id, msbf: Option<&(&str, File<Box<[u8]>>)>) -> Result<()> {
        if let Some((msbf_key, msbf_file)) = msbf {
            self.language(course)?.flow_inject(msbf_key, msbf_file.clone())?;
        }

        Ok(())
    }

    fn language<C>(&mut self, course: C) -> Result<&mut Language>
        where
            C: Into<Option<Id>>,
    {
        Ok(if let Some(course) = course.into() {
            &mut self.course(course)?.language
        } else {
            &mut self.boot
        })
    }

    fn flow<C>(&mut self, course: C) -> Result<albw::language::LoadedMut<FlowMut>>
        where
            C: Into<Option<Id>>,
    {
        Ok(self.language(course)?.flow_mut())
    }

    fn parse_args(&mut self, course: Id, stage: u16, unq: u16) -> &mut Arg {
        self.scene(course, stage).unwrap()
            .stage_mut()
            .get_mut()
            .get_obj_mut(unq)
            .ok_or_else(|| {
                Error::game(format!(
                    "{}{} [{}] not found",
                    course.as_str(),
                    stage + 1,
                    unq
                ))
            }).unwrap().arg_mut()
    }

    fn apply(&mut self, patch: Patch, item: Item, settings: &Settings) -> Result<()> {
        match patch {
            Patch::Chest { course, stage, unq } => {
                self.modify_objs(course, stage + 1, &[
                    (unq, Box::new(move |obj| {
                        obj.arg_mut().0 = item as i32;
                    })),
                ]);

                // Alter chests actors to match their contents
                if settings.options.chest_size_matches_contents {
                    let chest_data = if item.is_progression() {
                        (34, self.scene(DungeonHera, 0)?.actors().get("TreasureBoxL")?)
                    } else {
                        (35, self.scene(DungeonHera, 0)?.actors().get("TreasureBoxS")?)
                    };

                    let scene = self.scene(course, stage).unwrap();
                    scene.actors_mut().add(chest_data.1)?;
                    scene.stage_mut().get_mut().get_obj_mut(unq).unwrap().set_id(chest_data.0);
                }
            }
            Patch::Heart { course, scene, unq } |
            Patch::Key { course, scene, unq } |
            Patch::SilverRupee { course, scene, unq } |
            Patch::GoldRupee { course, scene, unq } => {
                self.parse_args(course, scene, unq).1 = item as i32;
            }
            Patch::Maiamai { course, scene, unq } => {
                self.parse_args(course, scene, unq).2 = item as i32;
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
                            course.as_ref().map(Id::as_str).unwrap_or("Boot"),
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
            Patch::Multi(patches) => {
                for patch in patches {
                    self.apply(patch, item, settings)?;
                }
            }
            Patch::None => {}
        }
        Ok(())
    }


    fn patch_dungeon_prizes(&mut self, layout: &Layout, settings: &Settings) -> Result<()> {

        // Fetch the placed Dungeon Rewards
        // TODO really need to clean up the Layout data structure...
        let prizes = DungeonPrizes {
            ep_prize: layout.get(&LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "Eastern Palace Prize")).unwrap(),
            hg_prize: layout.get(&LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "House of Gales Prize")).unwrap(),
            th_prize: layout.get(&LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "Tower of Hera Prize")).unwrap(),
            pd_prize: layout.get(&LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "Dark Palace Prize")).unwrap(),
            sp_prize: layout.get(&LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "Swamp Palace Prize")).unwrap(),
            sw_prize: layout.get(&LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "Skull Woods Prize")).unwrap(),
            tt_prize: layout.get(&LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "Thieves' Hideout Prize")).unwrap(),
            tr_prize: layout.get(&LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "Turtle Rock Prize")).unwrap(),
            dp_prize: layout.get(&LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "Desert Palace Prize")).unwrap(),
            ir_prize: layout.get(&LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "Ice Ruins Prize")).unwrap(),
        };

        self.patch_flowchart(&prizes)?;
        self.patch_msbf_files(&prizes)?;
        self.patch_dungeon_prize_actors(&prizes)?;
        scenes::patch_prize_byaml(self, &prizes, settings);

        Ok(())
    }

    /// Get msbf event files and inject them into scenes
    fn patch_msbf_files(&mut self, prizes: &DungeonPrizes) -> Result<()> {
        let prize_msbf_map: HashMap<Item, (&str, File<Box<[u8]>>)> = HashMap::from([
            (SageGulley, (MsbfKey::Dark, self.language(DungeonDark)?.flow().extract("World/Flow/Dark.msbf").unwrap())),
            (SageOren, (MsbfKey::Water, self.language(DungeonWater)?.flow().extract("World/Flow/Water.msbf").unwrap())),
            (SageSeres, (MsbfKey::Dokuro, self.language(FieldDark)?.flow().extract("World/Flow/Dokuro.msbf").unwrap())),
            (SageOsfala, (MsbfKey::Hagure, self.language(IndoorDark)?.flow().extract("World/Flow/Hagure.msbf").unwrap())),
            /* No Impa */
            (SageIrene, (MsbfKey::Sand, self.language(FieldDark)?.flow().extract("World/Flow/Sand.msbf").unwrap())),
            (SageRosso, (MsbfKey::Ice, self.language(DungeonIce)?.flow().extract("World/Flow/Ice.msbf").unwrap())),
        ]);

        self.inject_msbf(DungeonEast, prize_msbf_map.get(&prizes.ep_prize))?;
        self.inject_msbf(DungeonWind, prize_msbf_map.get(&prizes.hg_prize))?;
        self.inject_msbf(DungeonHera, prize_msbf_map.get(&prizes.th_prize))?;
        self.inject_msbf(DungeonDark, prize_msbf_map.get(&prizes.pd_prize))?;
        self.inject_msbf(DungeonWater, prize_msbf_map.get(&prizes.sp_prize))?;
        self.inject_msbf(FieldDark, prize_msbf_map.get(&prizes.sw_prize))?;
        self.inject_msbf(IndoorDark, prize_msbf_map.get(&prizes.tt_prize))?;
        self.inject_msbf(DungeonKame, prize_msbf_map.get(&prizes.tr_prize))?;
        self.inject_msbf(FieldDark, prize_msbf_map.get(&prizes.dp_prize))?;
        self.inject_msbf(DungeonIce, prize_msbf_map.get(&prizes.ir_prize))?;

        Ok(())
    }

    fn patch_dungeon_prize_actors(&mut self, prizes: &DungeonPrizes) -> Result<()> {

        // Fetch and map Actors to their dungeon prizes
        let pendant = self.scene(DungeonWind, 2)?.actors().get("Pendant")?;
        let actor_map: HashMap<Item, Actor> = HashMap::from([
            (PendantCourage, pendant.clone()),
            (PendantWisdom, pendant.clone()),
            (PendantPower, pendant),
            (SageGulley, self.scene(DungeonDark, 0)?.actors().get("PictureBlacksmithBoy")?),
            (SageOren, self.scene(DungeonWater, 2)?.actors().get("PictureZoraQueen")?),
            (SageSeres, self.scene(FieldDark, 0)?.actors().get("PicturePriestGirl")?),
            (SageOsfala, self.scene(IndoorDark, 14)?.actors().get("PictureSahasPupil")?),
            (SageImpa, self.scene(DungeonKame, 2)?.actors().get("PictureInpa")?),
            (SageIrene, self.scene(FieldDark, 30)?.actors().get("PictureMaple")?),
            (SageRosso, self.scene(DungeonIce, 0)?.actors().get("PictureMountaineer")?),
        ]);

        // Add Actors to relevant scenes
        self.scene(DungeonEast, 0)?.actors_mut().add(actor_map.get(&prizes.ep_prize).unwrap().clone())?;
        self.scene(DungeonWind, 2)?.actors_mut().add(actor_map.get(&prizes.hg_prize).unwrap().clone())?;
        self.scene(DungeonHera, 0)?.actors_mut().add(actor_map.get(&prizes.th_prize).unwrap().clone())?;
        self.scene(DungeonDark, 0)?.actors_mut().add(actor_map.get(&prizes.pd_prize).unwrap().clone())?;
        self.scene(DungeonWater, 2)?.actors_mut().add(actor_map.get(&prizes.sp_prize).unwrap().clone())?;
        self.scene(FieldDark, 0)?.actors_mut().add(actor_map.get(&prizes.sw_prize).unwrap().clone())?;
        self.scene(IndoorDark, 14)?.actors_mut().add(actor_map.get(&prizes.tt_prize).unwrap().clone())?;
        self.scene(DungeonKame, 2)?.actors_mut().add(actor_map.get(&prizes.tr_prize).unwrap().clone())?;
        self.scene(FieldDark, 30)?.actors_mut().add(actor_map.get(&prizes.dp_prize).unwrap().clone())?;
        self.scene(DungeonIce, 0)?.actors_mut().add(actor_map.get(&prizes.ir_prize).unwrap().clone())?;

        // Inject Small Chests into scenes that don't have them for Pendants
        // TODO Remove after Pendants can be redirected
        let chest_small = self.scene(DungeonHera, 0)?.actors().get("TreasureBoxS")?;
        if is_pendant(prizes.sp_prize) {
            let warp_tile = self.scene(DungeonHera, 0)?.actors().get("WarpTile")?;
            self.scene(DungeonWater, 2)?.actors_mut().add(warp_tile)?;
            self.scene(DungeonWater, 2)?.actors_mut().add(chest_small.clone())?;
        }
        if is_pendant(prizes.sw_prize) {
            self.scene(FieldDark, 0)?.actors_mut().add(chest_small.clone())?;
        }
        if is_pendant(prizes.tt_prize) {
            self.scene(IndoorDark, 14)?.actors_mut().add(chest_small.clone())?;
        }
        if is_pendant(prizes.dp_prize) {
            self.scene(FieldDark, 30)?.actors_mut().add(chest_small.clone())?;
        }
        if is_pendant(prizes.tr_prize) {
            self.scene(DungeonKame, 2)?.actors_mut().add(chest_small)?;
        }

        Ok(())
    }

    fn patch_flowchart(&mut self, prizes: &DungeonPrizes) -> Result<()> {

        // Map dungeon MsbfInfo to the randomized prizes
        let dungeon_msbf_mapping: Vec<(MsbfInfo, Option<&'static str>)> = Vec::from([
            (MsbfInfo::EP, prizes.ep_prize.msbf_key()),
            (MsbfInfo::HG, prizes.hg_prize.msbf_key()),
            (MsbfInfo::TH, prizes.th_prize.msbf_key()),
            (MsbfInfo::PD, prizes.pd_prize.msbf_key()),
            (MsbfInfo::SP, prizes.sp_prize.msbf_key()),
            (MsbfInfo::SW, prizes.sw_prize.msbf_key()),
            (MsbfInfo::TT, prizes.tt_prize.msbf_key()),
            (MsbfInfo::TR, prizes.tr_prize.msbf_key()),
            (MsbfInfo::DP, prizes.dp_prize.msbf_key()),
            (MsbfInfo::IR, prizes.ir_prize.msbf_key()),
        ]);

        // Read and deserialize the FlowChart from RegionBoot
        let raw = self.boot.archive.get_mut().read("World/Byaml/FlowChart.byaml")?;
        let mut flow_chart: File<FlowChart> = raw.try_map(|data| byaml::from_bytes(&data))?;


        // Remove vanilla msbf entries
        // NOTE: Skull + Desert share FieldDark, so this must be done separately from adding
        // for (dungeon_info, _) in &dungeon_msbf_mapping {
        //     if dungeon_info.has_msbf() {
        //         flow_chart.get_mut().load_mut().remove_entry(
        //             dungeon_info.get_course().as_str(), dungeon_info.get_vanilla_msbf().unwrap());
        //     }
        // }

        // Add msbf for dungeon prize
        for (dungeon_info, new_msbf) in &dungeon_msbf_mapping {
            if new_msbf.is_some() {
                flow_chart.get_mut().load_mut().add_entry(
                    dungeon_info.get_course().as_str(), new_msbf.unwrap());
            }
        }

        // Serialize the FlowChart and update the boot archive
        let serialized = flow_chart.serialize();
        self.boot.archive.get_mut().update(serialized)?;

        Ok(())
    }

    fn patch_actor_profile(&mut self, _layout: &Layout, _settings: &Settings) -> Result<()> {

        // let tr_prize = layout.get(&LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "Turtle Rock Prize")).unwrap();
        //
        // let smol_byaml = match tr_prize {
        //     SageGulley => "ObjPictureBlacksmithBoy.byaml",
        //     SageOren => "ObjPictureZoraQueen.byaml",
        //     SageSeres => "ObjPicturePriestGirl.byaml",
        //     SageOsfala => "ObjPictureSahasPupil.byaml",
        //     SageIrene => "ObjPictureMaple.byaml",
        //     SageRosso => "ObjPictureMountaineer.byaml",
        //     SageImpa | PendantPower | PendantWisdom | PendantCourage => { return Ok(()); },
        //     _ => panic!()
        // };
        //
        // // Read and deserialize the FlowChart from RegionBoot
        // let mut szs = self.game.actor_profile();
        //
        // // Make the collision of the TR Portrait larger so Link can 'Touch' it
        // // let smol_raw = szs.get_mut().read(smol_byaml)?;
        // // let mut smol_profile: File<ActorProfile> = smol_raw.try_map(|data| byaml::from_bytes(&data))?;
        // // info!("read smol file");
        // // smol_profile.get_mut().collision.get_mut(0).unwrap().scale = String::from("{X: 3.00000, Y: 3.00000, Z: 3.00000}");
        //
        // // Reduce the collision of Impa to match normal sages when not in TR
        // let impa_raw = szs.get_mut().read("ObjPictureInpa.byaml")?;
        // let mut impa_profile: File<ActorProfile> = impa_raw.try_map(|data| byaml::from_bytes(&data))?;
        //
        // info!("Being ActorProfile Serializing...");
        //
        // // Serialize and update the archive
        // //self.boot.archive.get_mut().add(smol_profile.serialize())?;
        // self.boot.archive.get_mut().add(impa_profile.serialize())?;

        Ok(())
    }

    pub fn prepare(mut self, layout: &Layout, settings: &Settings) -> Result<Patches> {
        self.patch_dungeon_prizes(layout, settings)?;
        self.patch_actor_profile(layout, settings)?;

        let mut item_actors = HashMap::new();

        for (item, get_item) in self.game.match_items_to_get_items() {
            let actor_opt = get_item.actor(&self.game);
            if actor_opt.is_some() {
                let mut actor = actor_opt.unwrap();
                actor.rename(format!("World/Actor/{}.bch", get_item.actor_name()?));
                item_actors.insert(item, actor);
            }
        }

        // let mut heart_container = self.scene(IndoorDark, 14)?.actors().get("HeartContainer")?;
        // heart_container.rename(String::from("World/Actor/HeartPiece.bch"));
        // self.scene(FieldLight, 16)?.actors_mut().update(heart_container.clone())?;

        // Add chest to pedestal scene
        let chest_small = self.scene(DungeonHera, 0)?.actors().get("TreasureBoxS")?;
        self.scene(FieldLight, 33)?.actors_mut().add(chest_small.clone())?; // Master Sword Pedestal

        // Add Warp Tiles to scenes for softlock prevention
        let warp_tile = self.scene(DungeonHera, 0)?.actors().get("WarpTile")?;
        self.scene(DungeonWind, 0)?.actors_mut().add(warp_tile.clone())?; // Gales 1F
        self.scene(FieldDark, 19)?.actors_mut().add(warp_tile.clone())?; // Dark Maze
        self.scene(DungeonWater, 1)?.actors_mut().add(warp_tile.clone())?; // Swamp Palace B1
        self.scene(DungeonDokuro, 1)?.actors_mut().add(warp_tile.clone())?; // Skull Woods B2

        // Debug stuff
        //let step_switch = self.scene(DungeonDark, 0)?.actors().get("SwitchStep")?;
        // self.scene(DungeonKame, 2)?.actors_mut().add(step_switch.clone())?; // Turtle Rock Boss
        //self.scene(DungeonWind, 2)?.actors_mut().add(step_switch.clone())?; // Gales Boss
        // self.scene(DungeonHera, 0)?.actors_mut().add(step_switch.clone())?; // Hera Boss
        // self.scene(FieldDark, 30)?.actors_mut().add(step_switch.clone())?; // Desert Boss

        // TODO Bow of Light Fix
        //self.scene(course::Id::DungeonWater, 2)?.actors_mut().add(fresco_arrow.clone())?;
        //self.scene(course::Id::IndoorLight, 0)?.actors_mut().add(fresco_arrow.clone())?;


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
                .get_mut(&IndoorLight)
                .unwrap()
                .scenes
                .get_mut(&0)
                .unwrap()
                .actors_mut();
            for actor in rentals.iter().filter_map(|item| item_actors.get(item)) {
                your_house_actors.add(actor.clone())?;
            }
            let kakariko_actors = courses
                .get_mut(&FieldLight)
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

        // Add Actors to Common Archive
        //let mut common = game.common().unwrap();
        //common.add(chest_large)?;
        //common.add(fresco_arrow)?; // Sorta works... but not really...
        //romfs.add(common.into_archive().unwrap());

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
        course: Id,
        stage: u16,
        unq: u16,
    },
    Event {
        course: Option<Id>,
        name: &'static str,
        index: u16,
    },
    Heart {
        course: Id,
        scene: u16,
        unq: u16,
    },
    Key {
        course: Id,
        scene: u16,
        unq: u16,
    },
    Maiamai {
        course: Id,
        scene: u16,
        unq: u16,
    },
    SilverRupee {
        course: Id,
        scene: u16,
        unq: u16,
    },
    GoldRupee {
        course: Id,
        scene: u16,
        unq: u16,
    },
    Shop(Shop),
    Multi(Vec<Patch>),
    None, // Workaround until everything is shufflable
}

impl Patch {
    pub fn apply(self, patcher: &mut Patcher, item: Item, settings: &Settings) -> Result<()> {
        patcher.apply(self, item, settings)
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
                //246, // Skip Irene, make Hyrule Hotfoot appear, spawns certain enemies
                248, // Skip Yuga killing Osfala
                //250, // Yuga 1 Defeated
                //251, // Set in Post-EP FieldLight20 cutscene / Effectively Green Pendant
                //310, // Watched HC Post-EP cutscene
                315, // Shop open???
                // 320, // Shady Guy Trigger
                321, 322, // Skip first Oren cutscenes
                415, // Skip Yuga capturing Zelda
                430, // Fix Chamber of Sages Softlock
                510, // Open Portals, Activate Hyrule Castle Midway
                522, // Hilda Blacksmith Text + get Map Swap icon on lower screen
                523, // Hilda Graveyard Text
                524, // Hilda ??? Text
                525, // Skip Sahasrahla outside Link's House, make Hyrule Hotfoot appear

                // 536, 537, // Gulley
                // 556, 557, // Oren Flags
                // 576, 577, // Seres Flags
                // 596, 597, // Osfala Flags
                // 616, 617, // Rosso Flags
                // 636, 637, // Irene Flags
                // 656, 657, // Impa Flags

                542, 543, // Skip Bomb-Shop Man dialogue
                560, // Hilda ??? Text
                599, // Disable Sand Rod return
                600, // Hilda ??? Text
                620, // Hilda ??? Text
                640, // Hilda ??? Text
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

            if logic.vanes_activated {
                for flag in array::IntoIter::new([
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

use crate::filler::filler_item::{Randomizable, Vane};
use crate::filler::portals::Portal;
use crate::{patch::util::*, Error, PortalMap, Result, SeedInfo};
use code::Code;
use fs_extra::dir::CopyOptions;
use game::{
    Course::{self as CourseId, *},
    Item, World,
};
use log::{debug, error, info};
use macros::fail;
use modinfo::settings::portal_shuffle::PortalShuffle;
use modinfo::settings::portals::Portals;
use modinfo::settings::weather_vanes::WeatherVanes::*;
use path_absolutize::*;
use rom::byaml::scene_env::SceneEnvFile;
use rom::flag::Flag;
use rom::scene::{Transform, Vec3};
use rom::{
    flow::FlowMut,
    scene::{Arg, Obj, Rail, SceneMeta},
    File, IntoBytes, Language, Rom, Scene,
};
use serde::Serialize;
use std::{collections::HashMap, fs, path::Path};
use tempfile::tempdir;
use try_insert_ext::EntryInsertExt;

mod actors;
mod byaml;
mod code;
mod demo;
pub mod lms;
mod messages;
mod prizes;
pub mod util;

#[non_exhaustive]
pub struct DungeonPrizes {
    ep_prize: Randomizable,
    hg_prize: Randomizable,
    th_prize: Randomizable,
    pd_prize: Randomizable,
    sp_prize: Randomizable,
    sw_prize: Randomizable,
    tt_prize: Randomizable,
    tr_prize: Randomizable,
    dp_prize: Randomizable,
    ir_prize: Randomizable,
}

#[derive(Debug)]
pub struct Patcher {
    game: Rom,
    boot: Language,
    rentals: [Item; 9],
    merchant: [Item; 3],
    courses: HashMap<CourseId, Course>,
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

    fn add_obj(&mut self, id: CourseId, stage_index: u16, obj: Obj) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_obj(obj);
    }

    fn add_rail(&mut self, id: CourseId, stage_index: u16, rail: Rail) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_rail(rail);
    }

    #[allow(unused)]
    fn add_system(&mut self, id: CourseId, stage_index: u16, obj: Obj) {
        self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut().add_system(obj);
    }

    /// Finds the lowest UNQ and SER
    fn find_objs_unq_ser(&mut self, id: CourseId, stage_index: u16) -> (u16, Option<u16>) {
        let unq_ser = (self.find_objs_unq(id, stage_index), self.find_objs_ser(id, stage_index));
        debug!("Unused {:?}{} Objs UNQ: {}, SER: {:?}", id, stage_index, unq_ser.0, unq_ser.1);
        unq_ser
    }

    /// Finds the lowest currently unused UNQ
    fn find_objs_unq(&mut self, id: CourseId, stage_index: u16) -> u16 {
        self.scene(id, stage_index - 1).unwrap().stage().get().find_objs_unq()
    }

    /// Finds the lowest currently unused SER
    fn find_objs_ser(&mut self, id: CourseId, stage_index: u16) -> Option<u16> {
        Some(self.scene(id, stage_index - 1).unwrap().stage().get().find_objs_ser())
    }

    /// Finds the lowest currently unused Rails UNQ
    #[allow(unused)]
    fn find_rails_unq(&mut self, id: CourseId, stage_index: u16) -> u16 {
        let unq = self.scene(id, stage_index - 1).unwrap().stage().get().find_rails_unq();
        debug!("Unused {:?}{} Rails UNQ: {}", id, stage_index, unq);
        unq
    }

    /// Finds the lowest UNQ and SER
    #[allow(unused)]
    fn find_system_unq_ser(&mut self, id: CourseId, stage_index: u16) -> (u16, Option<u16>) {
        let unq_ser = (self.find_system_unq(id, stage_index), self.find_system_ser(id, stage_index));
        debug!("Unused {:?}{} System UNQ: {}, SER: {:?}", id, stage_index, unq_ser.0, unq_ser.1);
        unq_ser
    }

    /// Finds the lowest currently unused UNQ
    #[allow(unused)]
    fn find_system_unq(&mut self, id: CourseId, stage_index: u16) -> u16 {
        self.scene(id, stage_index - 1).unwrap().stage().get().find_system_unq()
    }

    /// Finds the lowest currently unused SER
    #[allow(unused)]
    fn find_system_ser(&mut self, id: CourseId, stage_index: u16) -> Option<u16> {
        Some(self.scene(id, stage_index - 1).unwrap().stage().get().find_system_ser())
    }

    fn read_obj(&mut self, id: CourseId, stage_index: u16, unq: u16) -> &Obj {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        stage.get_obj(unq).expect(&format!(
            "Failed to read Portal Objs entry with UNQ: {} from World/Byaml/{:?}{}_stage.byaml",
            unq, id, stage_index
        ))
    }

    fn modify_objs<A>(&mut self, course_id: CourseId, stage_index: u16, actions: A)
    where
        A: Into<Vec<(u16, Box<dyn Fn(&mut Obj)>)>>,
    {
        let stage = self.scene(course_id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions.into() {
            action(
                stage
                    .get_obj_mut(unq)
                    .ok_or_else(|| {
                        Error::game(format!(
                            "Could not find [Objs] UNQ {} in {}{}",
                            unq,
                            course_id.as_str(),
                            stage_index
                        ))
                    })
                    .unwrap(),
            );
        }
    }

    fn modify_rails<A>(&mut self, id: CourseId, stage_index: u16, actions: A)
    where
        A: Into<Vec<(u16, Box<dyn Fn(&mut Rail)>)>>,
    {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions.into() {
            action(
                stage
                    .get_rails_mut(unq)
                    .ok_or_else(|| {
                        Error::game(format!("Could not find [Rails] UNQ {} in {}{}", unq, id.as_str(), stage_index))
                    })
                    .unwrap(),
            );
        }
    }

    fn modify_system<A>(&mut self, id: CourseId, stage_index: u16, actions: A)
    where
        A: Into<Vec<(u16, Box<dyn Fn(&mut Obj)>)>>,
    {
        let stage = self.scene(id, stage_index - 1).unwrap().stage_mut().get_mut();
        for (unq, action) in actions.into() {
            action(
                stage
                    .get_system_mut(unq)
                    .ok_or_else(|| {
                        Error::game(format!("Could not find [System] UNQ {} in {}{}", unq, id.as_str(), stage_index))
                    })
                    .unwrap(),
            );
        }
    }

    fn load_course(game: &mut Rom, course: CourseId) -> Course {
        game.course(course)
            .language()
            .map(|load| Course {
                language: load,
                scenes: Default::default(),
                scene_meta: game.course(course).scene_meta(),
            })
            .unwrap()
    }

    fn course(&mut self, course: CourseId) -> Result<&mut Course> {
        let Self { game, ref mut courses, .. } = self;
        Ok(courses.entry(course).or_insert(Self::load_course(game, course)))
    }

    /// Subtract 1 from stage
    fn scene(&mut self, course: CourseId, stage: u16) -> Result<&mut Scene> {
        let Self { game, ref mut courses, .. } = self;
        courses
            .entry(course)
            .or_insert(Self::load_course(game, course))
            .scenes
            .entry(stage)
            .or_try_insert_with(|| game.course(course).scene(stage))
            .map_err(Into::into)
    }

    fn scene_meta(&mut self, course: CourseId) -> &mut SceneMeta {
        let Self { game, ref mut courses, .. } = self;
        let Course { ref mut scene_meta, .. } = courses.entry(course).or_insert(Self::load_course(game, course));
        scene_meta.as_mut().unwrap()
    }

    fn scene_env(&mut self) -> rom::Result<SceneEnvFile> {
        self.game.scene_env()
    }

    fn update(&mut self, (course, file): (CourseId, File<Vec<u8>>)) -> Result<()> {
        self.language(course)?.update(file)?;
        Ok(())
    }

    fn inject_msbf(&mut self, course: CourseId, msbf: Option<&(&str, File<Box<[u8]>>)>) -> Result<()> {
        if let Some((msbf_key, msbf_file)) = msbf {
            self.language(course)?.flow_inject(msbf_key, msbf_file.clone())?;
        }

        Ok(())
    }

    fn language<C>(&mut self, course: C) -> Result<&mut Language>
    where
        C: Into<Option<CourseId>>,
    {
        Ok(if let Some(course) = course.into() { &mut self.course(course)?.language } else { &mut self.boot })
    }

    fn flow<C>(&mut self, course: C) -> Result<rom::language::LoadedMut<FlowMut>>
    where
        C: Into<Option<CourseId>>,
    {
        Ok(self.language(course)?.flow_mut())
    }

    /// Perform patching operations for each patch, depending on what type it is.
    fn apply<F>(&mut self, patch: Patch, seed_info: &SeedInfo, filler_item: F) -> Result<()>
    where
        F: Into<Option<Randomizable>> + Clone,
    {
        match patch {
            Patch::Chest { course, stage, unq } => {
                self.prep_chest(filler_item.into().unwrap(), course, stage, unq, false, seed_info)?;
            },
            Patch::BigChest { course, stage, unq } => {
                self.prep_chest(filler_item.into().unwrap(), course, stage, unq, true, seed_info)?;
            },
            Patch::Heart { course, scene, unq }
            | Patch::Key { course, scene, unq }
            | Patch::SilverRupee { course, scene, unq }
            | Patch::GoldRupee { course, scene, unq } => {
                self.parse_args(course, scene, unq).1 = filler_item.into().unwrap().as_item_index() as i32;
            },
            Patch::Portal { course, scene, unq, portal } => {
                self.patch_portal(&seed_info.portal_map, course, scene + 1, unq, portal, seed_info)?;
            },
            Patch::WeatherVane { course, scene, unq, vane } => {
                self.patch_weather_vane(filler_item.into().unwrap(), course, scene, unq, vane, seed_info)?;
            },
            Patch::Maiamai { course, scene, unq } => {
                self.parse_args(course, scene, unq).2 = filler_item.into().unwrap().as_item_index() as i32;
            },
            Patch::Event { course, name, index } => {
                self.flow(course)?
                    .get_mut(name)
                    .ok_or_else(|| Error::game(format!("File not found: {name}")))??
                    .get_mut()
                    .get_mut(index)
                    .ok_or_else(|| {
                        Error::game(format!(
                            "{}/{} [{}] not found",
                            course.as_ref().map(CourseId::as_str).unwrap_or("Boot"),
                            name,
                            index
                        ))
                    })?
                    .into_action()
                    .ok_or_else(|| Error::game("Not an action."))?
                    .set_value(filler_item.into().unwrap().as_item_index());
            },
            Patch::Shop(Shop::Ravio(index)) => {
                self.rentals[index as usize] = filler_item.into().unwrap().as_item().unwrap().to_game_item();
            },
            Patch::Shop(Shop::Merchant(index)) => {
                self.merchant[index as usize] = filler_item.into().unwrap().as_item().unwrap().to_game_item();
            },
            Patch::Multi(patches) => {
                for patch in patches {
                    self.apply(patch, seed_info, filler_item.clone())?;
                }
            },
            Patch::None => {},
        }
        Ok(())
    }

    fn parse_args(&mut self, course: CourseId, stage: u16, unq: u16) -> &mut Arg {
        self.scene(course, stage)
            .unwrap()
            .stage_mut()
            .get_mut()
            .get_obj_mut(unq)
            .ok_or_else(|| Error::game(format!("{}{} [{}] not found", course.as_str(), stage + 1, unq)))
            .unwrap()
            .arg_mut()
    }

    /// Patch Chest actors and swap their size if needed for CSMC.
    fn prep_chest(
        &mut self, item: Randomizable, course: CourseId, stage: u16, unq: u16, is_big: bool,
        SeedInfo { settings, .. }: &SeedInfo,
    ) -> Result<()> {
        // Set contents
        self.parse_args(course, stage, unq).0 = item.as_item_index() as i32;

        let small_chest = (35, "TreasureBoxS");
        let large_chest = (34, "TreasureBoxL");

        let chest_data = if settings.chest_size_matches_contents {
            if item.goes_in_csmc_large_chest(settings) {
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
        self.scene(course, stage).unwrap().stage_mut().get_mut().get_obj_mut(unq).unwrap().set_id(chest_data.0);

        // Add Actor if scene doesn't already have it
        if !self.scene(course, stage).unwrap().actors().contains(chest_data.1) {
            debug!("Adding {} to {}{}", chest_data.1, course.as_str(), stage + 1);
            let actor = self.scene(DungeonHera, 0)?.actors().get_actor_bch(chest_data.1)?;
            self.scene(course, stage).unwrap().actors_mut().add(actor)?;
        }

        Ok(())
    }

    /// Portals!
    fn patch_portal(
        &mut self, portal_map: &PortalMap, course: CourseId, scene: u16, unq: u16, here_portal: Portal,
        seed_info: &SeedInfo,
    ) -> Result<()> {
        if seed_info.settings.portal_shuffle == PortalShuffle::Off {
            return Ok(());
        }

        let there_portal = portal_map.get(&here_portal).expect(&format!("No portal_map entry for: {:?}", here_portal));
        let there_flag = there_portal.get_flag();
        let there_sp = there_portal.get_spawn_point();

        let here_arg2 = if here_portal.get_world() == there_portal.get_world() {
            here_portal.get_reverse_type()
        } else {
            here_portal.get_type()
        };

        // Redirect Portal to new destination, and set correct flag to update destination icon on lower screen
        self.modify_objs(
            course,
            scene,
            [call(unq, move |obj| {
                obj.redirect(there_sp);
                obj.arg.2 = here_arg2;
                obj.set_active_flag(there_flag);
                obj.set_inactive_flag(here_portal.get_flag());
            })],
        );

        if here_portal == Portal::HyruleCastle {
            // Hyrule Castle Portal
            self.modify_objs(
                IndoorLight,
                7,
                [
                    set_46_args(26, there_flag),      // Curtain
                    set_46_args(29, there_flag),      // AreaDisableWallIn
                    set_disable_flag(29, there_flag), // AreaDisableWallIn
                ],
            );
        } else if seed_info.settings.portals == Portals::Closed {
            // Portal paired with Hyrule Castle is always kept open
            if here_portal == *seed_info.portal_map.get(&Portal::HyruleCastle).unwrap() {
                self.modify_objs(course, scene, [clear_enable_flag(unq)]);
            } else {
                // Lock all Portals (except HC + its pair) behind Flag 510
                self.modify_objs(course, scene, [set_enable_flag(unq, Flag::QUAKE)]);
            }
        }

        // TODO angle of Portal Blockages can't seem to be changed, no point in this function until the game respects
        // the x-component of the MojWallBreakFieldLight/MojWallBreakFieldDark's rotation.
        //self.block_portals(course, scene, unq, here_portal, *there_portal)?;

        Ok(())
    }

    /// For portals that now lead to blocked portals, add matching blockages on that side of the portal
    #[allow(unused)]
    fn block_portals(
        &mut self, course: CourseId, scene: u16, unq: u16, here_portal: Portal, there_portal: Portal,
    ) -> Result<()> {
        // Already blocked Portals can be left alone
        match here_portal {
            Portal::DesertNorth
            | Portal::EasternRuinsSE
            | Portal::DarkRuinsSE
            | Portal::GraveyardLedgeLorule
            | Portal::HyruleCastle => {
                return Ok(());
            },
            _ => {},
        }

        match there_portal {
            // Blocked portals
            Portal::DesertNorth | Portal::EasternRuinsSE | Portal::GraveyardLedgeLorule | Portal::DarkRuinsSE => {
                // Read vanilla Portal data, use it to add blockages relative to Portals
                let obj_portal = self.read_obj(course, scene, unq);
                let clp = obj_portal.clp;
                let translate = obj_portal.srt.translate;
                let (wall_unq, wall_ser) = self.find_objs_unq_ser(course, scene);

                // Attach blockage to Portal (keeps light/dark fog from appearing until breakage is gone)
                self.modify_objs(course, scene, [call(unq, move |obj| obj.lnk = vec![(wall_unq, 0, 0)])]);

                // Different actors for WallBreakFieldLight and WallBreakFieldDark
                match there_portal {
                    Portal::DesertNorth | Portal::EasternRuinsSE => {
                        self.add_obj(
                            course,
                            scene,
                            Obj::wall_break_field_light(here_portal.get_flag(), clp, wall_ser, wall_unq, translate),
                        );
                        self.copy_bch("WallBreakFieldLight", (FieldLight, 30), (course, scene))?;
                    },
                    Portal::GraveyardLedgeLorule | Portal::DarkRuinsSE => {
                        self.add_obj(
                            course,
                            scene,
                            Obj::wall_break_field_dark(here_portal.get_flag(), clp, wall_ser, wall_unq, translate),
                        );
                        self.copy_bch("WallBreakFieldDark", (FieldDark, 30), (course, scene))?;
                    },
                    _ => unreachable!(),
                }
            },
            // Hyrule Castle Curtain
            Portal::HyruleCastle => {
                // Read vanilla Portal data, use it to add the Curtain and WallDisableIn actors
                let obj_portal = self.read_obj(course, scene, unq);
                let clp = obj_portal.clp;
                let t_curtain = obj_portal.srt.translate.add(Vec3 { x: 0.0, y: 0.0, z: 1.0 });
                let t_wall = obj_portal.srt.translate.add(Vec3 { x: -0.02731, y: -0.00001, z: 0.96672 });
                let (curtain_unq, curtain_ser) = self.find_objs_unq_ser(course, scene);

                // Attach Curtain to Portal (keeps light from appearing until Curtain is gone)
                self.modify_objs(course, scene, [call(unq, move |obj| obj.lnk = vec![(curtain_unq, 0, 0)])]);

                let flag = here_portal.get_flag();

                // Curtain
                self.add_obj(
                    course,
                    scene,
                    Obj {
                        arg: Arg(0, 0, 0, 0, flag.get_type(), 0, flag.get_value(), 0, 0, 0, 0, 0, 0, 0.0),
                        clp,
                        flg: (0, 0, 0, 0),
                        id: 550,
                        lnk: vec![],
                        nme: None,
                        ril: vec![],
                        ser: curtain_ser,
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 16.0, z: 1.0 },
                            rotate: Vec3 { x: 339.37350, y: 0.0, z: 0.0 },
                            translate: t_curtain,
                        },
                        typ: 1,
                        unq: curtain_unq,
                    },
                );

                // WallDisableIn
                let (disable_merge_unq, disable_merge_ser) = self.find_objs_unq_ser(course, scene);
                self.add_obj(
                    course,
                    scene,
                    Obj {
                        arg: Arg(0, 0, 0, 0, flag.get_type(), 0, flag.get_value(), 0, 0, 0, 0, 0, 0, 0.0),
                        clp,
                        flg: (0, flag.get_type(), 0, flag.get_value()),
                        id: 568,
                        lnk: vec![],
                        nme: None,
                        ril: vec![],
                        ser: disable_merge_ser,
                        srt: Transform {
                            scale: Vec3 { x: 5.25577, y: 1.0, z: 2.43051 },
                            rotate: Vec3::ZERO,
                            translate: t_wall,
                        },
                        typ: 6,
                        unq: disable_merge_unq,
                    },
                );

                self.copy_bch("Curtain", (IndoorLight, 7), (course, scene))?;
            },
            _ => return Ok(()),
        };

        Ok(())
    }

    /// Copies an actor .bch file from a given stage to a different stage, if it doesn't already have it.
    fn copy_bch(
        &mut self, actor_name: &str, (from_course, from_stage): (CourseId, u16), (to_course, to_stage): (CourseId, u16),
    ) -> Result<()> {
        if !self.scene(to_course, to_stage - 1).unwrap().actors().contains(actor_name) {
            debug!(
                "Copying {} from {}{} to {}{}",
                actor_name,
                from_course.as_str(),
                from_stage,
                to_course.as_str(),
                to_stage
            );
            let actor = self.scene(from_course, from_stage - 1)?.actors().get_actor_bch(actor_name)?;
            self.scene(to_course, to_stage - 1).unwrap().actors_mut().add(actor)?;
        }

        Ok(())
    }

    /// Weather Vanes
    fn patch_weather_vane(
        &mut self, item: Randomizable, course: CourseId, scene: u16, unq: u16, vane: Vane,
        SeedInfo { settings, .. }: &SeedInfo,
    ) -> Result<()> {
        if settings.weather_vanes != Shuffled {
            return Ok(());
        }

        let wv_flag = match item {
            Randomizable::Vane(vane) => vane.flag().get_value(),
            _ => unreachable!(),
        };

        // Set Weather Vane flag to randomized value
        self.parse_args(course, scene, unq).6 = wv_flag;

        let wv_world_dest = Vane::get_world(item.into());
        let wv_world_vane = Vane::get_world(vane);

        // Actor info for model swaps
        let wv_config_info = match wv_world_dest {
            World::Hyrule => (165, "Telephone", FieldLight),
            World::Lorule => (464, "TelephoneDark", FieldDark),
        };

        // Forcibly set ID
        self.scene(course, scene).unwrap().stage_mut().get_mut().get_obj_mut(unq).unwrap().set_id(wv_config_info.0);

        // Swap the Weather Vane model if it's from the opposite world
        if wv_world_vane != wv_world_dest {
            Self::copy_bch(self, wv_config_info.1, (wv_config_info.2, 16), (course, scene + 1))?;
        }
        //     Add Actor if scene doesn't already have it
        //     if !self.scene(course, scene).unwrap().actors().contains(wv_config_info.1) {
        //         debug!("Adding {} to {}{}", wv_config_info.1, course.as_str(), scene + 1);
        //         let actor =
        //             self.scene(wv_config_info.2, 15)?.actors().get_actor_bch(wv_config_info.1)?;
        //         self.scene(course, scene).unwrap().actors_mut().add(actor)?;
        //     }
        // }

        Ok(())
    }

    pub fn prepare(mut self, seed_info: &SeedInfo) -> Result<Patches> {
        actors::patch(&mut self, seed_info)?;
        lms::msbf::patch(&mut self, seed_info)?;
        messages::patch_messages(&mut self, seed_info)?;
        let prizes = get_dungeon_prizes(&seed_info.layout);
        prizes::patch_dungeon_prizes(&mut self, &prizes);
        // byaml::get_item::patch(&mut self)?;
        byaml::course::patch(&mut self, &prizes, seed_info);
        byaml::stage::patch(&mut self, seed_info)?;
        let scene_env_file = byaml::scene_env::patch(&mut self, &seed_info.settings);
        let cutscenes = demo::build_replacement_cutscenes(seed_info)?;

        let common_archive = self.game.common()?;
        let mut item_actors = HashMap::new();

        for (item, get_item) in self.game.match_items_to_get_items() {
            if Item::SpecialMove.as_str().eq(&get_item.0) {
                // fixme hacky and gross
                let mut actor = common_archive.get_actor_bch("SwordD")?.clone();
                actor.rename(String::from("World/Actor/SwordD.bch"));
                item_actors.insert(item, actor);
            } else if let Some(mut actor) = get_item.actor(&self.game) {
                actor.rename(format!("World/Actor/{}.bch", get_item.actor_name()?));
                item_actors.insert(item, actor);
            }
        }

        {
            let Self { ref rentals, ref merchant, ref mut courses, .. } = self;
            let your_house_actors = courses.get_mut(&IndoorLight).unwrap().scenes.get_mut(&0).unwrap().actors_mut();
            for actor in rentals.iter().filter_map(|item| item_actors.get(&item)) {
                your_house_actors.add(actor.clone())?;
            }
            let kakariko_actors = courses.get_mut(&FieldLight).unwrap().scenes.get_mut(&15).unwrap().actors_mut();
            kakariko_actors.add(item_actors.get(&merchant[0]).unwrap().clone())?;
            kakariko_actors.add(item_actors.get(&merchant[2]).unwrap().clone())?;
        }
        let code = code::create(&self, seed_info);
        let Self { game, boot, courses, .. } = self;
        let mut romfs = Files(vec![]);

        // Add Actors to Common Archive
        //let mut common = game.common().unwrap();
        //common.add(chest_large)?;
        //common.add(fresco_arrow)?; // Sorta works... but not really...
        //romfs.add(common.into_archive().unwrap());

        romfs.add(boot.into_archive());
        if let Some(scene_env_file) = scene_env_file {
            romfs.add_serialize(scene_env_file.into_file());
        };
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
        for cutscene in cutscenes {
            romfs.add(cutscene);
        }
        Ok(Patches { game, code, romfs })
    }
}

/// Research MSBF and MSBT Files
#[allow(unused)]
#[deprecated]
pub fn research_msbf_msbt<C>(
    patcher: &mut Patcher, msbf_course: C, msbf_file: &str, msbt_course: game::Course, msbt_file: &str, edotor: bool,
) where
    C: Into<Option<game::Course>>,
{
    let labels = messages::research(patcher, msbt_course, msbt_file, edotor);
    lms::msbf::research(patcher, msbf_course, msbf_file, labels, edotor);

    info!("Early Debug Exit");
    std::process::exit(0);
}

#[derive(Debug)]
pub struct Course {
    language: Language,
    scenes: HashMap<u16, Scene>,
    scene_meta: Option<SceneMeta>,
}

#[derive(Clone, Debug)]
pub enum Patch {
    Chest { course: CourseId, stage: u16, unq: u16 },
    BigChest { course: CourseId, stage: u16, unq: u16 },
    Event { course: Option<CourseId>, name: &'static str, index: u16 },
    Heart { course: CourseId, scene: u16, unq: u16 },
    Key { course: CourseId, scene: u16, unq: u16 },
    Maiamai { course: CourseId, scene: u16, unq: u16 },
    SilverRupee { course: CourseId, scene: u16, unq: u16 },
    GoldRupee { course: CourseId, scene: u16, unq: u16 },
    Portal { course: CourseId, scene: u16, unq: u16, portal: Portal },
    WeatherVane { course: CourseId, scene: u16, unq: u16, vane: Vane },
    Shop(Shop),
    Multi(Vec<Patch>),
    None, // Workaround until everything is shufflable
}

impl Patch {
    pub fn apply<F>(self, patcher: &mut Patcher, seed_info: &SeedInfo, filler_item: F) -> Result<()>
    where
        F: Into<Option<Randomizable>>,
    {
        patcher.apply(self, seed_info, filler_item.into())
    }
}

#[derive(Clone, Debug)]
pub enum Shop {
    Ravio(u8),
    Merchant(u8),
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
        info!("Writing Patch Files to:         {}\\{:016X}", &path.absolutize()?.display(), self.game.id());

        match fs_extra::copy_items(&[moddir], path, &CopyOptions { overwrite: true, ..Default::default() })
            .map_err(Error::io)
        {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Couldn't write to:              {}", path.display());
                error!("Please check that config.json points to a valid output destination.");
                fail!();
            },
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

use crate::filler::filler_item::Item::*;
use crate::filler::filler_item::Item::{SageOren, SageSeres};
use crate::filler::filler_item::Randomizable;
use crate::filler::filler_item::Randomizable::Item;
use crate::{
    patch::{util::*, DungeonPrizes},
    MsbfKey, Patcher,
};
use game::Course::{self, *};
use log::info;
use macros::fail;
use rom::flag::Flag;
use rom::{
    byaml,
    language::FlowChart,
    scene::{Arg, Obj, Point, Rail, SpawnPoint, Transform, Vec3},
    Actor, File,
};
use std::collections::HashMap;

pub(crate) fn patch_dungeon_prizes(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    info!("Patching Dungeon Prizes...");
    patch_flowchart(patcher, prizes);
    patch_msbf_files(patcher, prizes);
    patch_dungeon_prize_actors(patcher, prizes);
    patch_prize_byaml(patcher, prizes);
}

/// Adds entries to the FlowChart for the MSBF files related to each Portrait
fn patch_flowchart(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    // Map dungeon MsbfInfo to the randomized prizes
    let dungeon_msbf_mapping: Vec<(Option<&'static str>, Course)> = Vec::from([
        (prizes.ep_prize.msbf_key(), DungeonEast),
        (prizes.hg_prize.msbf_key(), DungeonWind),
        (prizes.th_prize.msbf_key(), DungeonHera),
        (prizes.pd_prize.msbf_key(), DungeonDark),
        (prizes.sp_prize.msbf_key(), DungeonWater),
        (prizes.sw_prize.msbf_key(), FieldDark),
        (prizes.tt_prize.msbf_key(), IndoorDark),
        (prizes.tr_prize.msbf_key(), DungeonKame),
        (prizes.dp_prize.msbf_key(), FieldDark),
        (prizes.ir_prize.msbf_key(), DungeonIce),
    ]);

    // Read and deserialize the FlowChart from RegionBoot
    let raw = patcher.boot.archive.get_mut().read("World/Byaml/FlowChart.byaml").unwrap();
    let mut flow_chart: File<FlowChart> = raw.try_map(|data| byaml::from_bytes(&data)).unwrap();

    // Remove vanilla msbf entries
    // NOTE: Skull + Desert share FieldDark, so this must be done separately from adding
    // for (dungeon_info, _) in &dungeon_msbf_mapping {
    //     if dungeon_info.has_msbf() {
    //         flow_chart.get_mut().load_mut().remove_entry(
    //             dungeon_info.get_course().as_str(), dungeon_info.get_vanilla_msbf().unwrap());
    //     }
    // }

    // Add msbf for dungeon prize
    for (new_msbf, course) in &dungeon_msbf_mapping {
        if new_msbf.is_some() {
            flow_chart.get_mut().load_mut().add_entry(course.as_str(), new_msbf.unwrap());
        }
    }

    // Serialize the FlowChart and update the boot archive
    let serialized = flow_chart.serialize();
    patcher.boot.archive.get_mut().update(serialized).unwrap();
}

/// Get msbf event files and inject them into scenes
#[rustfmt::skip]
fn patch_msbf_files(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    let prize_msbf_map: HashMap<Randomizable, (&str, File<Box<[u8]>>)> = HashMap::from([
        (Item(SageGulley), (MsbfKey::Dark, patcher.language(DungeonDark).unwrap().flow().extract("World/Flow/Dark.msbf").unwrap())),
        (Item(SageOren), (MsbfKey::Water, patcher.language(DungeonWater).unwrap().flow().extract("World/Flow/Water.msbf").unwrap())),
        (Item(SageSeres), (MsbfKey::Dokuro, patcher.language(FieldDark).unwrap().flow().extract("World/Flow/Dokuro.msbf").unwrap())),
        (Item(SageOsfala), (MsbfKey::Hagure, patcher.language(IndoorDark).unwrap().flow().extract("World/Flow/Hagure.msbf").unwrap())),
        /* No Impa */
        (Item(SageIrene), (MsbfKey::Sand, patcher.language(FieldDark).unwrap().flow().extract("World/Flow/Sand.msbf").unwrap())),
        (Item(SageRosso), (MsbfKey::Ice, patcher.language(DungeonIce).unwrap().flow().extract("World/Flow/Ice.msbf").unwrap())),
    ]);

    patcher.inject_msbf(DungeonEast, prize_msbf_map.get(&prizes.ep_prize)).unwrap();
    patcher.inject_msbf(DungeonWind, prize_msbf_map.get(&prizes.hg_prize)).unwrap();
    patcher.inject_msbf(DungeonHera, prize_msbf_map.get(&prizes.th_prize)).unwrap();
    patcher.inject_msbf(DungeonDark, prize_msbf_map.get(&prizes.pd_prize)).unwrap();
    patcher.inject_msbf(DungeonWater, prize_msbf_map.get(&prizes.sp_prize)).unwrap();
    patcher.inject_msbf(FieldDark, prize_msbf_map.get(&prizes.sw_prize)).unwrap();
    patcher.inject_msbf(IndoorDark, prize_msbf_map.get(&prizes.tt_prize)).unwrap();
    patcher.inject_msbf(DungeonKame, prize_msbf_map.get(&prizes.tr_prize)).unwrap();
    patcher.inject_msbf(FieldDark, prize_msbf_map.get(&prizes.dp_prize)).unwrap();
    patcher.inject_msbf(DungeonIce, prize_msbf_map.get(&prizes.ir_prize)).unwrap();
}

/// Inject the prize actors into the relevant scenes
#[rustfmt::skip]
fn patch_dungeon_prize_actors(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    // Fetch and map Actors to their dungeon prizes
    let pendant = patcher.scene(DungeonWind, 2).unwrap().actors().get_actor_bch("Pendant").unwrap();
    let actor_map: HashMap<Randomizable, Actor> = HashMap::from([
        (Item(PendantOfPower), pendant.clone()),
        (Item(PendantOfWisdom), pendant.clone()),
        (Item(PendantOfCourage), pendant),
        (Item(SageGulley), patcher.scene(DungeonDark, 0).unwrap().actors().get_actor_bch("PictureBlacksmithBoy").unwrap()),
        (Item(SageOren), patcher.scene(DungeonWater, 2).unwrap().actors().get_actor_bch("PictureZoraQueen").unwrap()),
        (Item(SageSeres), patcher.scene(FieldDark, 0).unwrap().actors().get_actor_bch("PicturePriestGirl").unwrap()),
        (Item(SageOsfala), patcher.scene(IndoorDark, 14).unwrap().actors().get_actor_bch("PictureSahasPupil").unwrap()),
        (Item(SageImpa), patcher.scene(DungeonKame, 2).unwrap().actors().get_actor_bch("PictureInpa").unwrap()),
        (Item(SageIrene), patcher.scene(FieldDark, 30).unwrap().actors().get_actor_bch("PictureMaple").unwrap()),
        (Item(SageRosso), patcher.scene(DungeonIce, 0).unwrap().actors().get_actor_bch("PictureMountaineer").unwrap()),
    ]);

    // Add Actors to relevant scenes
    let actor_scene_map = vec![
        (DungeonEast, 0, prizes.ep_prize),
        (DungeonWind, 2, prizes.hg_prize),
        (DungeonHera, 0, prizes.th_prize),
        (DungeonDark, 0, prizes.pd_prize),
        (DungeonWater, 2, prizes.sp_prize),
        (FieldDark, 0, prizes.sw_prize),
        (IndoorDark, 14, prizes.tt_prize),
        (DungeonKame, 2, prizes.tr_prize),
        (FieldDark, 30, prizes.dp_prize),
        (DungeonIce, 0, prizes.ir_prize),
    ];
    for (course, scene, prize) in actor_scene_map {
        if let Some(prize) = actor_map.get(&prize) {
            patcher.scene(course, scene).unwrap().actors_mut().add(prize.clone()).unwrap();
        }
    }
}

/// Patches the BYAML files to shuffle Dungeon Prizes
fn patch_prize_byaml(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    patch_eastern(patcher, prizes.ep_prize);
    patch_gales(patcher, prizes.hg_prize);
    patch_hera(patcher, prizes.th_prize);
    patch_dark(patcher, prizes.pd_prize);
    patch_swamp(patcher, prizes.sp_prize);
    patch_skull(patcher, prizes.sw_prize);
    patch_thieves(patcher, prizes.tt_prize);
    patch_turtle(patcher, prizes.tr_prize);
    patch_desert(patcher, prizes.dp_prize);
    patch_ice(patcher, prizes.ir_prize);

    patch_checks_unlocked_by_prizes(patcher);
}

/// Patch Checks unlocked by specific Dungeon Prizes
fn patch_checks_unlocked_by_prizes(patcher: &mut Patcher) {
    patch_oren(patcher);
    patch_impa(patcher);
    patch_irene(patcher);
    patch_rosso(patcher);
}

/// Oren
fn patch_oren(patcher: &mut Patcher) {
    let credits_flag = Flag::Event(730);

    // Zora's Domain
    patcher.modify_objs(
        CaveLight,
        7,
        [
            set_disable_flag(116, credits_flag), // Thin Oren
            set_disable_flag(119, credits_flag), // Zora Attendant
            set_disable_flag(127, credits_flag), // Zora Attendant
            set_disable_flag(134, credits_flag), // Thicc Oren
        ],
    );

    // Shady Guy Trigger
    patcher.modify_objs(
        FieldLight,
        7,
        [
            set_enable_flag(14, Flag::SAGE_OREN), // Cutscene trigger
            set_enable_flag(16, Flag::SAGE_OREN), // Shady Guy
        ],
    );

    // Zora's Domain
    patcher.modify_objs(
        CaveLight,
        7,
        [
            // Hide Oren + Attendants until Oren is saved
            set_enable_flag(116, Flag::SAGE_OREN), // Thin Oren
            set_enable_flag(119, Flag::SAGE_OREN), // Zora Attendant
            set_enable_flag(127, Flag::SAGE_OREN), // Zora Attendant
            set_enable_flag(134, Flag::SAGE_OREN), // Thicc Oren
            // Require saving Oren to turn in Smooth Gem
            set_enable_flag(131, Flag::SAGE_OREN), // AreaSwitchCube
            set_enable_flag(132, Flag::SAGE_OREN), // Throw Smooth Gem textbox trigger
        ],
    );
}

/// Impa
fn patch_impa(patcher: &mut Patcher) {
    patcher.modify_objs(
        IndoorLight,
        12,
        [
            // Show Impa
            call(36, move |obj| {
                obj.set_enable_flag(Flag::Event(1));
                obj.clear_disable_flag();
            }),
        ],
    );
}

/// Irene
fn patch_irene(patcher: &mut Patcher) {
    // Bridge
    patcher.modify_objs(
        FieldLight,
        28,
        [
            set_46_args(55, Flag::SAGE_IRENE),     // Trigger - NpcMaple_BellGet_2D
            set_enable_flag(56, Flag::SAGE_IRENE), // Irene
        ],
    );

    // Fortune-Teller
    patcher.modify_objs(
        FieldLight,
        9,
        [
            set_46_args(83, Flag::SAGE_IRENE),     // Trigger - NpcMaple_BellGet_11
            set_enable_flag(85, Flag::SAGE_IRENE), // Irene
        ],
    );

    // Small Pond
    patcher.modify_objs(
        FieldLight,
        10,
        [
            set_46_args(65, Flag::SAGE_IRENE),     // Trigger - NpcMaple_BellGet_12_00
            set_enable_flag(67, Flag::SAGE_IRENE), // Irene
            set_46_args(68, Flag::SAGE_IRENE),     // Trigger - NpcMaple_BellGet_12_01
        ],
    );
}

/// Rosso
fn patch_rosso(patcher: &mut Patcher) {
    // Outside Rosso's House
    patcher.modify_objs(
        FieldLight,
        2,
        [
            set_enable_flag(11, Flag::SAGE_ROSSO),   // Small Rock (controller, see below)
            disable(88),                             // early game LZ to Rosso's House
            clear_disable_flag(100),                 // Keep Entry_KikoriMan3 from disappearing
            clear_disable_flag(100),                 // NpcMountaineer
            clear_enable_flag(101),                  // Rosso
            set_disable_flag(101, Flag::CREDITS),    // Rosso
            set_disable_flag(128, Flag::SAGE_ROSSO), // "Not in right now." signboard
            set_46_args(132, Flag::SAGE_ROSSO),      // Door
            disable(135),                            // Disable LZ to IndoorLight4 cutscene
            set_enable_flag(136, Flag::SAGE_ROSSO),  // LZ to Rosso's House
        ],
    );

    // Rosso's House
    patcher.modify_objs(
        IndoorLight,
        10,
        [call(7, move |obj| {
            obj.set_enable_flag(Flag::SAGE_ROSSO);
            obj.clear_disable_flag();
        })],
    );

    // Rosso Rocks
    patcher.modify_system(
        FieldLight,
        2,
        [
            set_enable_flag(11, Flag::SAGE_ROSSO), // controller
            set_enable_flag(12, Flag::SAGE_ROSSO),
            set_enable_flag(14, Flag::SAGE_ROSSO),
            set_enable_flag(15, Flag::SAGE_ROSSO),
            set_enable_flag(16, Flag::SAGE_ROSSO),
            set_enable_flag(18, Flag::SAGE_ROSSO),
            set_enable_flag(19, Flag::SAGE_ROSSO),
            set_enable_flag(20, Flag::SAGE_ROSSO),
            set_enable_flag(21, Flag::SAGE_ROSSO),
            set_enable_flag(93, Flag::SAGE_ROSSO),
            set_enable_flag(94, Flag::SAGE_ROSSO),
            set_enable_flag(102, Flag::SAGE_ROSSO),
            set_enable_flag(103, Flag::SAGE_ROSSO),
            set_enable_flag(104, Flag::SAGE_ROSSO),
            set_enable_flag(105, Flag::SAGE_ROSSO),
            set_enable_flag(106, Flag::SAGE_ROSSO),
            set_enable_flag(107, Flag::SAGE_ROSSO),
            set_enable_flag(108, Flag::SAGE_ROSSO),
            set_enable_flag(109, Flag::SAGE_ROSSO),
            set_enable_flag(110, Flag::SAGE_ROSSO),
            set_enable_flag(111, Flag::SAGE_ROSSO),
            set_enable_flag(112, Flag::SAGE_ROSSO),
            set_enable_flag(118, Flag::SAGE_ROSSO),
            set_enable_flag(119, Flag::SAGE_ROSSO),
            set_enable_flag(120, Flag::SAGE_ROSSO),
            set_enable_flag(121, Flag::SAGE_ROSSO),
            set_enable_flag(122, Flag::SAGE_ROSSO),
            set_enable_flag(123, Flag::SAGE_ROSSO),
            set_enable_flag(124, Flag::SAGE_ROSSO),
            set_enable_flag(125, Flag::SAGE_ROSSO),
            set_enable_flag(126, Flag::SAGE_ROSSO),
        ],
    );
}

/// Eastern Palace
fn patch_eastern(patcher: &mut Patcher, prize: Randomizable) {
    let data = PrizePatchData::get(prize);
    let outside_hyrule_castle = SpawnPoint::new(FieldLight, 18, 5);

    // Eastern Palace 1F - Add Dungeon Reward
    patcher.add_obj(
        DungeonEast,
        1,
        Obj {
            arg: Arg(
                outside_hyrule_castle.spawn,
                data.arg1,
                0,
                0,
                4,
                data.flag.get_type(),
                250,
                data.flag.get_value(),
                0,
                data.arg9,
                outside_hyrule_castle.course as i32,
                outside_hyrule_castle.scene - 1,
                data.arg12,
                0.0,
            ),
            clp: 0,
            flg: (4, data.flag.get_type(), 250, data.flag.get_value()),
            id: data.actor_id,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser: Some(129),
            srt: Transform {
                scale: Vec3::UNIT,
                rotate: Vec3 { x: data.rot_x, y: 0.0, z: 0.0 },
                translate: Vec3 { x: 0.0, y: 2.5, z: -5.75 },
            },
            typ: 4,
            unq: 301,
        },
    );

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, outside_hyrule_castle);
    }

    // Eastern Ruins - Disable Post-EP cutscene
    patcher.modify_objs(
        FieldLight,
        20,
        [
            enable(214), // Paint Heart
            enable(215), // Paint Heart
            disable(83), // Sahasrahla
            disable(84), // Text box
            disable(85), // Loading Zone to FL18
        ],
    );

    // Outside Hyrule Castle
    patcher.modify_objs(
        FieldLight,
        18,
        [
            // Sahasrahla
            call(200, |obj| {
                obj.set_rotate(0.0, 0.0, 0.0);
                obj.set_translate(0.0, 0.0, 13.5);
                obj.arg.3 = 2;
                obj.set_active_flag(Flag::Event(1));
                obj.enable();
            }),
            disable(208), // Textbox trigger
            disable(264), // lgt_NpcSoldier_Field1B_04_broke - idk what this is, but now it's nothing
            disable(529), // AreaSwitchCube
            disable(502), // Sahasrahla
        ],
    );

    // patcher.modify_system(FieldLight, 18, &[call(199, |obj| {
    //     obj.srt.translate.z = 12.75; // move to where cutscene normally ends
    // })]);
}

/// House of Gales
fn patch_gales(patcher: &mut Patcher, prize: Randomizable) {
    // Debug stuff
    // patcher.modify_objs(DungeonWind, 3, &[
    //     disable(436), // Margomill
    //     disable(457), // Holocaust
    // ]);
    // patcher.add_obj(DungeonWind, 3,
    //                 Obj::step_switch(Flag::Event(340), 0, 102, 564,
    //                                  Vec3 { x: 0.0, y: 0.0, z: -39.5 }));

    const UNQ_PRIZE: u16 = 459;

    // Insta-spawn Pendants
    if is_pendant(prize) {
        patcher.modify_objs(
            DungeonWind,
            3,
            [call(UNQ_PRIZE, |obj| {
                obj.lnk.clear();
            })],
        );
    }

    modify_dungeon_reward(patcher, prize, UNQ_PRIZE, DungeonWind, 3, false, SpawnPoint::new(FieldLight, 35, 0));

    let prize_flag = prize_flag(prize);
    patcher.modify_objs(
        DungeonWind,
        3,
        [
            set_enable_flag(490, prize_flag), // Warp to leave boss room
            set_enable_flag(543, prize_flag), // Destination Warp
        ],
    );

    // Gulley will fall down on his own, other Sages need to be put on a Rail to appear
    if is_sage(prize) && prize != Item(SageGulley) {
        patcher.modify_objs(DungeonWind, 3, [add_rail(UNQ_PRIZE, (12, 0))]);

        let (end_y, end_z) = if prize == Item(SageImpa) { (2.0, -47.0) } else { (0.0, -46.5) };
        patcher.add_rail(
            DungeonWind,
            3,
            Rail {
                arg: (0, 0, 0, 0, 0.0, 0.0),
                pnt: vec![
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [0.0, end_y, end_z - 15.0, 0.0, end_y, end_z - 15.0],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3::UNIT,
                            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z - 15.0 },
                        },
                    },
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [0.0, end_y, end_z, 0.0, end_y, end_z],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3::UNIT,
                            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z },
                        },
                    },
                ],
                rng: false,
                unq: 12,
            },
        );
    }
}

/// Tower of Hera
fn patch_hera(patcher: &mut Patcher, prize: Randomizable) {
    // Debug stuff
    // patcher.modify_objs(DungeonHera, 1, &[
    //     disable(737), // Moldorm
    //     disable(738), // Holocaust
    // ]);
    // patcher.add_obj(DungeonHera, 1,
    //                 Obj::step_switch(Flag::Event(370), 20, 313, 920,
    //                                  Vec3 { x: 0.0, y: 101.0, z: -1.5 }));

    const UNQ_PRIZE: u16 = 829;

    // Insta-spawn Pendants
    if is_pendant(prize) {
        patcher.modify_objs(
            DungeonHera,
            1,
            [call(UNQ_PRIZE, |obj| {
                obj.lnk.clear();
            })],
        );
    }

    modify_dungeon_reward(patcher, prize, UNQ_PRIZE, DungeonHera, 1, false, SpawnPoint::new(FieldLight, 3, 3));

    // Gulley will fall down on his own, other Sages need to be put on a Rail to appear
    if is_sage(prize) && prize != Item(SageGulley) {
        patcher.modify_objs(DungeonHera, 1, [add_rail(UNQ_PRIZE, (56, 0))]);

        let (end_y, end_z) = if prize == Item(SageImpa) { (103.0, -6.0) } else { (101.0, -5.5) };
        patcher.add_rail(
            DungeonHera,
            1,
            Rail {
                arg: (0, 0, 0, 0, 0.0, 0.0),
                pnt: vec![
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [0.0, end_y, end_z - 15.0, 0.0, end_y, end_z - 15.0],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3::UNIT,
                            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z - 15.0 },
                        },
                    },
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [0.0, end_y, end_z, 0.0, end_y, end_z],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3::UNIT,
                            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z },
                        },
                    },
                ],
                rng: false,
                unq: 56,
            },
        );
    }
}

/// Dark Palace
fn patch_dark(patcher: &mut Patcher, prize: Randomizable) {
    // Debug stuff
    // patcher.modify_objs(DungeonDark, 1, &[
    //     disable(118), // Gemesaur
    //     disable(122), // Holocaust
    // ]);
    // patcher.add_obj(DungeonDark, 1,
    //                 Obj::step_switch(Flag::Course(21), 4, 400, 400,
    //                                  Vec3 { x: 0.0, y: 0.0, z: -44.75 }));

    if prize == Item(SageGulley) {
        return;
    }

    modify_dungeon_reward(patcher, prize, 262, DungeonDark, 1, false, SpawnPoint::new(FieldDark, 20, 5));

    if is_pendant(prize) {
        // Don't take camera control away from player to watch Mask break and reveal... nothing...
        patcher.modify_objs(
            DungeonDark,
            1,
            [
                disable(121), // ObjJewelMask Camera
            ],
        );
    } else {
        // Put non-Gulley Portraits on a Rail so they drop down after the boss
        // TODO Figure out how to attach skeletal animation to portraits so they drop non-jankily
        patcher.modify_objs(DungeonDark, 1, [add_rail(262, (14, 0))]);
        let (end_y, end_z) = if prize == Item(SageImpa) { (2.0, -48.0) } else { (0.0, -47.5) };
        patcher.add_rail(
            DungeonDark,
            1,
            Rail {
                arg: (0, 0, 0, 0, 0.0, 0.0),
                pnt: vec![
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [0.0, 6.7700896, -48.948_357, 0.0, 6.7700896, -48.948_357],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                            rotate: Vec3 { x: 335.99999945441, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: 6.7700896, z: -48.948_357 },
                        },
                    },
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [0.0, end_y, end_z, 0.0, end_y, end_z],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                            rotate: Vec3 { x: 331.999999459, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z },
                        },
                    },
                ],
                rng: false,
                unq: 14,
            },
        );
    }

    // Remove Maze Guards after Dark Palace
    // let prize_flag = prize_flag(prize);
    // patcher.modify_objs(FieldDark, 20, &[
    //     set_disable_flag(73, prize_flag),
    //     set_disable_flag(82, prize_flag),
    //     set_disable_flag(83, prize_flag),
    //     set_disable_flag(84, prize_flag),
    //     set_disable_flag(113, prize_flag),
    //     set_disable_flag(123, prize_flag),
    //     set_disable_flag(135, prize_flag),
    //     set_disable_flag(136, prize_flag),
    //     set_disable_flag(143, prize_flag),
    //     set_disable_flag(171, prize_flag),
    //     set_disable_flag(176, prize_flag),
    //     set_disable_flag(177, prize_flag),
    //     set_disable_flag(178, prize_flag),
    //     set_disable_flag(179, prize_flag),
    //     set_disable_flag(197, prize_flag),
    // ]);
}

/// Swamp Palace
fn patch_swamp(patcher: &mut Patcher, prize: Randomizable) {
    if prize == Item(SageOren) {
        return;
    }

    modify_dungeon_reward(patcher, prize, 13, DungeonWater, 3, true, SpawnPoint::new(FieldDark, 33, 0));
}

/// Skull Woods
fn patch_skull(patcher: &mut Patcher, prize: Randomizable) {
    if prize == Item(SageSeres) {
        return;
    }

    modify_dungeon_reward(patcher, prize, 273, FieldDark, 1, true, SpawnPoint::new(FieldDark, 1, 10));
}

/// Thieves' Hideout
fn patch_thieves(patcher: &mut Patcher, prize: Randomizable) {
    if prize == Item(SageOsfala) {
        return;
    }

    modify_dungeon_reward(patcher, prize, 3, IndoorDark, 15, true, SpawnPoint::new(FieldDark, 16, 14));
}

/// Turtle Rock
fn patch_turtle(patcher: &mut Patcher, prize: Randomizable) {
    // Debug stuff
    // patcher.modify_objs(DungeonKame, 3, &[
    //     disable(8), // Grinexx
    //     disable(11), // Holocaust
    // ]);
    // patcher.add_obj(DungeonKame, 3,
    //                 Obj::step_switch(Flag::Course(130), 0, 32, 100,
    //                                  Vec3 { x: 0.0, y: 5.0, z: -39.0 }));

    if prize == Item(SageImpa) {
        return;
    }

    const UNQ_PRIZE: u16 = 56;
    const DY: f32 = -2.0;
    const DZ: f32 = 0.5;

    modify_dungeon_reward(patcher, prize, UNQ_PRIZE, DungeonKame, 3, false, SpawnPoint::new(FieldDark, 35, 6));

    if is_pendant(prize) {
        // Pendants don't ride on the pillar, so manually move them and remove the pillar
        patcher.modify_objs(
            DungeonKame,
            3,
            [
                call(UNQ_PRIZE, move |obj| {
                    obj.srt.translate.z = -44.0;
                    obj.srt.translate.y = 5.0;
                }),
                disable(9), // dgn_Kame_Pillar
            ],
        );
    } else {
        // Gulley is a difficult child
        if prize == Item(SageGulley) {
            patcher.modify_objs(
                DungeonKame,
                3,
                [call(UNQ_PRIZE, move |obj| {
                    obj.srt.translate.z += DZ;
                    obj.srt.translate.y += 15.0 + DY;
                })],
            );
        }

        // Modify Rails so that non-Impa Portraits are reachable
        // TODO modify ActorProfile collision entries so this won't be needed

        let fn_extend_rails = move |rail: &mut Rail| {
            let rails_len = rail.pnt.len();
            rail.pnt.get_mut(rails_len - 1).unwrap().srt.translate.y += DY;

            let mut p = rail.pnt.last().unwrap().clone();
            p.srt.translate.z += DZ;
            rail.pnt.push(p);
        };

        patcher.modify_rails(
            DungeonKame,
            3,
            [
                call_rail(1, fn_extend_rails),  // dgn_Kame_Pillar
                call_rail(17, fn_extend_rails), // Impa
            ],
        );
    }
}

/// Desert Palace
fn patch_desert(patcher: &mut Patcher, prize: Randomizable) {
    // Debug stuff
    // patcher.modify_objs(FieldDark, 31, &[
    //     disable(67), // Zaganaga
    //     disable(74), // Holocaust
    // ]);
    // patcher.add_obj(FieldDark, 31,
    //                 Obj::step_switch(Flag::Course(252), 0, 58, 137,
    //                                  Vec3 { x: -19.0, y: 0.0, z: -19.0 }));

    if prize == Item(SageIrene) {
        return;
    }

    modify_dungeon_reward(patcher, prize, 76, FieldDark, 31, true, SpawnPoint::new(FieldDark, 31, 30));

    let prize_flag = prize_flag(prize);

    // Add blue warp to allow going from Zaga to Mire repeatedly, not just when first picking up the dungeon prize
    let (unq, ser) = patcher.find_objs_unq_ser(FieldDark, 31);
    let mut warp_tile = Obj::blue_warp(
        prize_flag,
        0,
        ser,
        unq,
        SpawnPoint::new(FieldDark, 31, 30),
        Vec3 { x: -13.0, y: 0.0, z: -24.0 },
    );
    warp_tile.set_enable_flag(prize_flag);
    patcher.add_obj(FieldDark, 31, warp_tile);

    patcher.modify_objs(
        FieldDark,
        31,
        [
            set_enable_flag(132, prize_flag), // Warp to leave boss area
            set_enable_flag(133, prize_flag), // Destination Warp
        ],
    );
}

/// Ice Ruins
fn patch_ice(patcher: &mut Patcher, prize: Randomizable) {
    // Debug stuff
    // patcher.modify_system(DungeonIce, 1, &[
    //     call(68, |obj| {
    //         obj.srt.translate.z = -31.6 + 3.0;
    //     }),
    // ]);

    if prize == Item(SageRosso) {
        return;
    }

    modify_dungeon_reward(patcher, prize, 16, DungeonIce, 1, true, SpawnPoint::new(FieldDark, 5, 0));
}

struct PrizePatchData {
    actor_id: i16,
    flag: Flag,
    rot_x: f32,
    arg1: i32,
    arg9: i32,
    arg12: i32,
}

impl PrizePatchData {
    fn new(actor_id: i16, flag: Flag, rot_x: f32, arg1: i32, arg9: i32, arg12: i32) -> Self {
        Self { actor_id, flag, rot_x, arg1, arg9, arg12 }
    }

    fn get(prize: Randomizable) -> Self {
        match prize {
            Item(SageGulley) => Self::new(418, Flag::Event(536), 0.0, 0, 1, 60),
            Item(SageOren) => Self::new(423, Flag::Event(556), 330.0, 0, 0, 30),
            Item(SageSeres) => Self::new(420, Flag::Event(576), 330.0, 0, 0, 30),
            Item(SageOsfala) => Self::new(419, Flag::Event(596), 330.0, 0, 0, 30),
            Item(SageRosso) => Self::new(422, Flag::Event(616), 330.0, 0, 0, 30),
            Item(SageIrene) => Self::new(417, Flag::Event(636), 330.0, 0, 0, 30),
            Item(SageImpa) => Self::new(421, Flag::Event(656), 330.0, 0, 0, 120),
            Item(PendantOfPower) => Self::new(173, Flag::Event(372), 0.0, 0, 0, 0),
            Item(PendantOfWisdom) => Self::new(173, Flag::Event(342), 0.0, 1, 0, 0),
            Item(PendantOfCourage) => Self::new(173, Flag::Event(251), 0.0, 2, 0, 0),
            _ => fail!("\"{}\" is not a dungeon prize.", prize.as_str()),
        }
    }
}

fn reroute_sage_warp(patcher: &mut Patcher, prize: Randomizable, sp: SpawnPoint) {
    // Get UNQ of warp object in the Chamber of Sages
    let unq_sage_warp = match prize {
        Item(SageGulley) => Some(73),
        Item(SageOren) => Some(72),
        Item(SageSeres) => Some(71),
        Item(SageOsfala) => Some(67),
        Item(SageRosso) => Some(69),
        Item(SageIrene) => Some(70),
        Item(SageImpa) => Some(68),
        Item(PendantOfPower) | Item(PendantOfWisdom) | Item(PendantOfCourage) => None,
        _ => fail!("\"{}\" is not a dungeon prize.", prize.as_str()),
    };

    // Reroute
    if let Some(unq_sage_warp) = unq_sage_warp {
        patcher.modify_objs(CaveDark, 10, [redirect(unq_sage_warp, sp)]);
    }
}

fn modify_dungeon_reward(
    patcher: &mut Patcher, prize: Randomizable, unq: u16, scene: Course, scene_index: u16, activate: bool,
    sp: SpawnPoint,
) {
    let data = PrizePatchData::get(prize);
    patcher.modify_objs(
        scene,
        scene_index,
        [call(unq, move |obj| {
            obj.set_id(data.actor_id);
            obj.arg.1 = data.arg1;
            if activate {
                obj.set_active_flag(Flag::Event(1));
            }
            obj.set_inactive_flag(data.flag);
            obj.set_rotate(data.rot_x, 0.0, 0.0);
            obj.set_disable_flag(data.flag);
            if is_pendant(prize) {
                obj.redirect(sp);
            }
        })],
    );
    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, sp);
    }
}

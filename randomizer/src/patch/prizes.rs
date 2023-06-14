use {
    crate::{
        patch::{util::*, DungeonPrizes},
        ItemExt, MsbfKey, Patcher, Settings,
    },
    log::info,
    macros::fail,
    std::collections::HashMap,
};
use jack::byaml::course::CourseId::{self, *};
use jack::byaml::{Dest, Flag, Transform, Vec3};
use jack::byaml::stage::{Arg, Obj, Point, Rail};
use jack::item::Item::{self, *};
use jack::JackFile;
use jack::lms::msbf::MsbfFile;

pub(crate) fn patch_dungeon_prizes(
    patcher: &mut Patcher, prizes: &DungeonPrizes, settings: &Settings,
) {
    info!("Patching Dungeon Prizes...");
    patch_flowchart(patcher, &prizes);
    patch_msbf_files(patcher, &prizes);
    patch_dungeon_prize_actors(patcher, &prizes);
    patch_prize_byaml(patcher, settings, &prizes);
}

/// Adds entries to the FlowChart for the MSBF files related to each Portrait
fn patch_flowchart(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    // Map dungeon MsbfInfo to the randomized prizes
    let dungeon_msbf_mapping: Vec<(Option<&'static str>, Id)> = Vec::from([
        (prizes.ep_prize.msbf_key(), DungeonEast),
        (prizes.hg_prize.msbf_key(), DungeonWind),
        (prizes.th_prize.msbf_key(), DungeonHera),
        (prizes.hc_prize.msbf_key(), IndoorLight),
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
    let prize_msbf_map: HashMap<Item, (&str, JackFile<MsbfFile>)> = HashMap::from([
        (SageGulley, (MsbfKey::Dark, patcher.language(DungeonDark).unwrap().flow().extract("World/Flow/Dark.msbf").unwrap())),
        (SageOren, (MsbfKey::Water, patcher.language(DungeonWater).unwrap().flow().extract("World/Flow/Water.msbf").unwrap())),
        (SageSeres, (MsbfKey::Dokuro, patcher.language(FieldDark).unwrap().flow().extract("World/Flow/Dokuro.msbf").unwrap())),
        (SageOsfala, (MsbfKey::Hagure, patcher.language(IndoorDark).unwrap().flow().extract("World/Flow/Hagure.msbf").unwrap())),
        /* No Impa */
        (SageIrene, (MsbfKey::Sand, patcher.language(FieldDark).unwrap().flow().extract("World/Flow/Sand.msbf").unwrap())),
        (SageRosso, (MsbfKey::Ice, patcher.language(DungeonIce).unwrap().flow().extract("World/Flow/Ice.msbf").unwrap())),
    ]);

    patcher.inject_msbf(DungeonEast, prize_msbf_map.get(&prizes.ep_prize)).unwrap();
    patcher.inject_msbf(DungeonWind, prize_msbf_map.get(&prizes.hg_prize)).unwrap();
    patcher.inject_msbf(DungeonHera, prize_msbf_map.get(&prizes.th_prize)).unwrap();
    patcher.inject_msbf(IndoorLight, prize_msbf_map.get(&prizes.hc_prize)).unwrap();
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
    let actor_map: HashMap<Item, Actor> = HashMap::from([
        (PendantPower, pendant.clone()),
        (PendantWisdom, pendant.clone()),
        (PendantCourage, pendant.clone()),
        (ZeldaAmulet, pendant),
        (SageGulley, patcher.scene(DungeonDark, 0).unwrap().actors().get_actor_bch("PictureBlacksmithBoy").unwrap()),
        (SageOren, patcher.scene(DungeonWater, 2).unwrap().actors().get_actor_bch("PictureZoraQueen").unwrap()),
        (SageSeres, patcher.scene(FieldDark, 0).unwrap().actors().get_actor_bch("PicturePriestGirl").unwrap()),
        (SageOsfala, patcher.scene(IndoorDark, 14).unwrap().actors().get_actor_bch("PictureSahasPupil").unwrap()),
        (SageImpa, patcher.scene(DungeonKame, 2).unwrap().actors().get_actor_bch("PictureInpa").unwrap()),
        (SageIrene, patcher.scene(FieldDark, 30).unwrap().actors().get_actor_bch("PictureMaple").unwrap()),
        (SageRosso, patcher.scene(DungeonIce, 0).unwrap().actors().get_actor_bch("PictureMountaineer").unwrap()),
    ]);

    // Add Actors to relevant scenes
    patcher.scene(DungeonEast, 0).unwrap().actors_mut().add(actor_map.get(&prizes.ep_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonWind, 2).unwrap().actors_mut().add(actor_map.get(&prizes.hg_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonHera, 0).unwrap().actors_mut().add(actor_map.get(&prizes.th_prize).unwrap().clone()).unwrap();
    patcher.scene(IndoorLight, 11).unwrap().actors_mut().add(actor_map.get(&prizes.hc_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonDark, 0).unwrap().actors_mut().add(actor_map.get(&prizes.pd_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonWater, 2).unwrap().actors_mut().add(actor_map.get(&prizes.sp_prize).unwrap().clone()).unwrap();
    patcher.scene(FieldDark, 0).unwrap().actors_mut().add(actor_map.get(&prizes.sw_prize).unwrap().clone()).unwrap();
    patcher.scene(IndoorDark, 14).unwrap().actors_mut().add(actor_map.get(&prizes.tt_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonKame, 2).unwrap().actors_mut().add(actor_map.get(&prizes.tr_prize).unwrap().clone()).unwrap();
    patcher.scene(FieldDark, 30).unwrap().actors_mut().add(actor_map.get(&prizes.dp_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonIce, 0).unwrap().actors_mut().add(actor_map.get(&prizes.ir_prize).unwrap().clone()).unwrap();

    //
    if is_pendant(prizes.sp_prize) {
        let warp_tile = patcher.scene(DungeonHera, 0).unwrap().actors().get_actor_bch("WarpTile").unwrap();
        patcher.scene(DungeonWater, 2).unwrap().actors_mut().add(warp_tile).unwrap();
    }

}

/// Patches the BYAML files to shuffle Dungeon Prizes
fn patch_prize_byaml(patcher: &mut Patcher, settings: &Settings, prizes: &DungeonPrizes) {
    patch_eastern(patcher, prizes.ep_prize);
    patch_gales(patcher, prizes.hg_prize);
    patch_hera(patcher, prizes.th_prize);
    patch_hyrule_castle(patcher, prizes.hc_prize);
    patch_dark(patcher, prizes.pd_prize);
    patch_swamp(patcher, prizes.sp_prize);
    patch_skull(patcher, prizes.sw_prize);
    patch_thieves(patcher, prizes.tt_prize);
    patch_turtle(patcher, prizes.tr_prize);
    patch_desert(patcher, prizes.dp_prize);
    patch_ice(patcher, prizes.ir_prize);

    patch_checks_unlocked_by_prizes(patcher, settings);
}

/// Patch Checks unlocked by specific Dungeon Prizes
fn patch_checks_unlocked_by_prizes(patcher: &mut Patcher, settings: &Settings) {
    patch_oren(patcher, settings);
    patch_impa(patcher, settings);
    patch_irene(patcher, settings);
    patch_rosso(patcher, settings);
}

/// Oren
fn patch_oren(patcher: &mut Patcher, settings: &Settings) {
    let credits_flag = Flag::Event(730);

    // Zora's Domain
    patcher.modify_objs(CaveLight, 7, &[
        set_disable_flag(116, credits_flag), // Thin Oren
        set_disable_flag(119, credits_flag), // Zora Attendant
        set_disable_flag(127, credits_flag), // Zora Attendant
        set_disable_flag(134, credits_flag), // Thicc Oren
    ]);

    if settings.logic.reverse_sage_events {
        let oren_flag = prize_flag(SageOren);

        // Shady Guy Trigger
        patcher.modify_objs(FieldLight, 7, &[
            set_enable_flag(14, oren_flag), // Cutscene trigger
            set_enable_flag(16, oren_flag), // Shady Guy
        ]);

        // Zora's Domain
        patcher.modify_objs(CaveLight, 7, &[
            // Hide Oren + Attendants until Oren is saved
            set_enable_flag(116, oren_flag), // Thin Oren
            set_enable_flag(119, oren_flag), // Zora Attendant
            set_enable_flag(127, oren_flag), // Zora Attendant
            set_enable_flag(134, oren_flag), // Thicc Oren
            // Require saving Oren to turn in Smooth Gem
            set_enable_flag(131, oren_flag), // AreaSwitchCube
            set_enable_flag(132, oren_flag), // Throw Smooth Gem textbox trigger
        ]);
    } else {
        // Zora's Domain
        patcher.modify_objs(CaveLight, 7, &[
            // Always allow turning in Smooth Gem
            clear_enable_flag(131), // AreaSwitchCube, fix for not being able to turn in Smooth Gem
            clear_enable_flag(132), // Throw Smooth Gem textbox trigger
        ]);
    }
}

/// Impa
fn patch_impa(patcher: &mut Patcher, settings: &Settings) {
    let impa_flag = prize_flag(SageImpa);
    if !settings.logic.reverse_sage_events {
        // Remove HC Impa when not RSE
        patcher.modify_objs(IndoorLight, 12, &[disable(36)]);
        return;
    }

    // Show Impa in Hyrule Castle's Throne Room after she's been rescued
    patcher.modify_objs(IndoorLight, 12, &[
        // Impa
        call(36, move |obj| {
            obj.set_enable_flag(impa_flag);
            obj.clear_disable_flag();
        }),
    ]);

    // RSE - Make Impa required to enter HC Front Door
    if settings.logic.reverse_sage_events {
        patcher.modify_objs(FieldLight, 18, &[
            // Front Door Soldier
            call(269, move |obj| {
                obj.clear_disable_flag();
                obj.set_enable_flag(impa_flag);
            }),
            set_enable_flag(270, impa_flag), // Impa
        ]);
    }
}

/// Irene
fn patch_irene(patcher: &mut Patcher, settings: &Settings) {
    if !settings.logic.reverse_sage_events {
        return;
    }

    let irene_flag = prize_flag(SageIrene);

    // Bridge
    patcher.modify_objs(FieldLight, 28, &[
        set_46_args(55, irene_flag),     // Trigger - NpcMaple_BellGet_2D
        set_enable_flag(56, irene_flag), // Irene
    ]);

    // Fortune-Teller
    patcher.modify_objs(FieldLight, 9, &[
        set_46_args(83, irene_flag),     // Trigger - NpcMaple_BellGet_11
        set_enable_flag(85, irene_flag), // Irene
    ]);

    // Small Pond
    patcher.modify_objs(FieldLight, 10, &[
        set_46_args(65, irene_flag),     // Trigger - NpcMaple_BellGet_12_00
        set_enable_flag(67, irene_flag), // Irene
        set_46_args(68, irene_flag),     // Trigger - NpcMaple_BellGet_12_01
    ]);
}

/// Rosso
fn patch_rosso(patcher: &mut Patcher, settings: &Settings) {
    let rosso_flag = if settings.logic.reverse_sage_events {
        prize_flag(SageRosso)
    } else {
        prize_flag(PendantCourage)
    };

    // Outside Rosso's House
    patcher.modify_objs(FieldLight, 2, &[
        set_enable_flag(11, rosso_flag), // Small Rock (controller, see below)
        disable(88),                     // early game LZ to Rosso's House
        clear_disable_flag(100),         // Keep Entry_KikoriMan3 from disappearing
        clear_disable_flag(100),         // NpcMountaineer
        set_disable_flag(128, rosso_flag), // "Not in right now." signboard
        set_46_args(132, rosso_flag),    // Door
        disable(135),                    // Disable LZ to IndoorLight4 cutscene
        set_enable_flag(136, rosso_flag), // LZ to Rosso's House
    ]);

    // Rosso's House
    patcher.modify_objs(IndoorLight, 10, &[call(7, move |obj| {
        obj.set_inactive_flag(Flag::Event(282));
        obj.set_enable_flag(rosso_flag);
        obj.clear_disable_flag();
        obj.set_typ(1);
    })]);

    // Rosso Rocks
    patcher.modify_system(FieldLight, 2, &[
        set_enable_flag(11, rosso_flag), // controller
        set_enable_flag(12, rosso_flag),
        set_enable_flag(14, rosso_flag),
        set_enable_flag(15, rosso_flag),
        set_enable_flag(16, rosso_flag),
        set_enable_flag(18, rosso_flag),
        set_enable_flag(19, rosso_flag),
        set_enable_flag(20, rosso_flag),
        set_enable_flag(21, rosso_flag),
        set_enable_flag(93, rosso_flag),
        set_enable_flag(94, rosso_flag),
        set_enable_flag(102, rosso_flag),
        set_enable_flag(103, rosso_flag),
        set_enable_flag(104, rosso_flag),
        set_enable_flag(105, rosso_flag),
        set_enable_flag(106, rosso_flag),
        set_enable_flag(107, rosso_flag),
        set_enable_flag(108, rosso_flag),
        set_enable_flag(109, rosso_flag),
        set_enable_flag(110, rosso_flag),
        set_enable_flag(111, rosso_flag),
        set_enable_flag(112, rosso_flag),
        set_enable_flag(118, rosso_flag),
        set_enable_flag(119, rosso_flag),
        set_enable_flag(120, rosso_flag),
        set_enable_flag(121, rosso_flag),
        set_enable_flag(122, rosso_flag),
        set_enable_flag(123, rosso_flag),
        set_enable_flag(124, rosso_flag),
        set_enable_flag(125, rosso_flag),
        set_enable_flag(126, rosso_flag),
    ]);
}

/// Eastern Palace
fn patch_eastern(patcher: &mut Patcher, prize: Item) {
    let data = PrizePatchData::get(prize);
    let outside_hyrule_castle = Dest::new(FieldLight, 18, 5);

    // Eastern Palace 1F - Add Dungeon Reward
    patcher.add_obj(DungeonEast, 1, Obj {
        arg: Arg(
            outside_hyrule_castle.spawn_point,
            data.arg1,
            0,
            0,
            4,
            data.flag.get_type(),
            250,
            data.flag.get_value(),
            0,
            data.arg9,
            outside_hyrule_castle.scene as i32,
            outside_hyrule_castle.scene_index - 1,
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
    });

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, outside_hyrule_castle);
    }

    // Eastern Ruins - Disable Post-EP cutscene
    patcher.modify_objs(FieldLight, 20, &[
        enable(214), // Paint Heart
        enable(215), // Paint Heart
        disable(83), // Sahasrahla
        disable(84), // Text box
        disable(85), // Loading Zone to FL18
    ]);

    // Outside Hyrule Castle
    patcher.modify_objs(FieldLight, 18, &[
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
    ]);

    // patcher.modify_system(FieldLight, 18, &[call(199, |obj| {
    //     obj.srt.translate.z = 12.75; // move to where cutscene normally ends
    // })]);
}

/// House of Gales
fn patch_gales(patcher: &mut Patcher, prize: Item) {
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
        patcher.modify_objs(DungeonWind, 3, &[call(UNQ_PRIZE, |obj| {
            obj.lnk.clear();
        })]);
    }

    if prize == PendantWisdom {
        return;
    }

    modify_dungeon_reward(
        patcher,
        prize,
        UNQ_PRIZE,
        DungeonWind,
        3,
        false,
        Dest::new(FieldLight, 35, 0),
    );

    let prize_flag = prize_flag(prize);
    patcher.modify_objs(DungeonWind, 3, &[
        set_enable_flag(490, prize_flag), // Warp to leave boss room
        set_enable_flag(543, prize_flag), // Destination Warp
    ]);

    // Gulley will fall down on his own, other Sages need to be put on a Rail to appear
    if is_sage(prize) {
        if prize != SageGulley {
            patcher.modify_objs(DungeonWind, 3, &[add_rail(UNQ_PRIZE, (12, 0))]);

            let (end_y, end_z) = if prize == SageImpa { (2.0, -47.0) } else { (0.0, -46.5) };
            patcher.add_rail(DungeonWind, 3, Rail {
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
            });
        }
    }
}

/// Tower of Hera
fn patch_hera(patcher: &mut Patcher, prize: Item) {
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
        patcher.modify_objs(DungeonHera, 1, &[call(UNQ_PRIZE, |obj| {
            obj.lnk.clear();
        })]);
    }

    if prize == PendantPower {
        return;
    }

    modify_dungeon_reward(
        patcher,
        prize,
        UNQ_PRIZE,
        DungeonHera,
        1,
        false,
        Dest::new(FieldLight, 3, 3),
    );

    // Gulley will fall down on his own, other Sages need to be put on a Rail to appear
    if is_sage(prize) {
        if prize != SageGulley {
            patcher.modify_objs(DungeonHera, 1, &[add_rail(UNQ_PRIZE, (56, 0))]);

            let (end_y, end_z) = if prize == SageImpa { (103.0, -6.0) } else { (101.0, -5.5) };
            patcher.add_rail(DungeonHera, 1, Rail {
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
            });
        }
    }
}

/// Hyrule Castle
fn patch_hyrule_castle(patcher: &mut Patcher, prize: Item) {
    // Convert Zelda herself into the dungeon prize
    const UNQ_ZELDA: u16 = 23;
    modify_dungeon_reward(
        patcher,
        prize,
        UNQ_ZELDA,
        IndoorLight,
        12,
        true,
        Dest::new(IndoorLight, 12, 1),
    );

    // Set TYP to 4 to make sure prize disappears when collected
    patcher.modify_objs(IndoorLight, 12, &[call(UNQ_ZELDA, |obj| {
        obj.set_typ(4);
    })]);
}

/// Dark Palace
fn patch_dark(patcher: &mut Patcher, prize: Item) {
    // Debug stuff
    // patcher.modify_objs(DungeonDark, 1, &[
    //     disable(118), // Gemesaur
    //     disable(122), // Holocaust
    // ]);
    // patcher.add_obj(DungeonDark, 1,
    //                 Obj::step_switch(Flag::Course(21), 4, 400, 400,
    //                                  Vec3 { x: 0.0, y: 0.0, z: -44.75 }));

    if prize == SageGulley {
        return;
    }

    modify_dungeon_reward(patcher, prize, 262, DungeonDark, 1, false, Dest::new(FieldDark, 20, 5));

    if is_pendant(prize) {
        // Don't take camera control away from player to watch Mask break and reveal... nothing...
        patcher.modify_objs(DungeonDark, 1, &[
            disable(121), // ObjJewelMask Camera
        ]);
    } else {
        // Put non-Gulley Portraits on a Rail so they drop down after the boss
        // TODO Figure out how to attach skeletal animation to portraits so they drop non-jankily
        patcher.modify_objs(DungeonDark, 1, &[add_rail(262, (14, 0))]);
        let (end_y, end_z) = if prize == SageImpa { (2.0, -48.0) } else { (0.0, -47.5) };
        patcher.add_rail(DungeonDark, 1, Rail {
            arg: (0, 0, 0, 0, 0.0, 0.0),
            pnt: vec![
                Point {
                    arg: (0, 0, 0, 0, 0.0, 0.0),
                    ctl: [0.0, 6.7700896, -48.9483577, 0.0, 6.7700896, -48.9483577],
                    lnk: vec![],
                    srt: Transform {
                        scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                        rotate: Vec3 { x: 335.99999945441, y: 0.0, z: 0.0 },
                        translate: Vec3 { x: 0.0, y: 6.7700896, z: -48.9483577 },
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
        });
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
fn patch_swamp(patcher: &mut Patcher, prize: Item) {
    if prize == SageOren {
        return;
    }

    modify_dungeon_reward(patcher, prize, 13, DungeonWater, 3, true, Dest::new(FieldDark, 33, 0));
}

/// Skull Woods
fn patch_skull(patcher: &mut Patcher, prize: Item) {
    if prize == SageSeres {
        return;
    }

    modify_dungeon_reward(patcher, prize, 273, FieldDark, 1, true, Dest::new(FieldDark, 1, 10));
}

/// Thieves' Hideout
fn patch_thieves(patcher: &mut Patcher, prize: Item) {
    if prize == SageOsfala {
        return;
    }

    modify_dungeon_reward(patcher, prize, 3, IndoorDark, 15, true, Dest::new(FieldDark, 16, 14));
}

/// Turtle Rock
fn patch_turtle(patcher: &mut Patcher, prize: Item) {
    // Debug stuff
    // patcher.modify_objs(DungeonKame, 3, &[
    //     disable(8), // Grinexx
    //     disable(11), // Holocaust
    // ]);
    // patcher.add_obj(DungeonKame, 3,
    //                 Obj::step_switch(Flag::Course(130), 0, 32, 100,
    //                                  Vec3 { x: 0.0, y: 5.0, z: -39.0 }));

    if prize == SageImpa {
        return;
    }

    const UNQ_PRIZE: u16 = 56;
    const DY: f32 = -2.0;
    const DZ: f32 = 0.5;

    modify_dungeon_reward(
        patcher,
        prize,
        UNQ_PRIZE,
        DungeonKame,
        3,
        false,
        Dest::new(FieldDark, 35, 6),
    );

    if is_pendant(prize) {
        // Pendants don't ride on the pillar, so manually move them and remove the pillar
        patcher.modify_objs(DungeonKame, 3, &[
            call(UNQ_PRIZE, move |obj| {
                obj.srt.translate.z = -44.0;
                obj.srt.translate.y = 5.0;
            }),
            disable(9), // dgn_Kame_Pillar
        ]);
    } else {
        // Gulley is a difficult child
        if prize == SageGulley {
            patcher.modify_objs(DungeonKame, 3, &[call(UNQ_PRIZE, move |obj| {
                obj.srt.translate.z += DZ;
                obj.srt.translate.y += 15.0 + DY;
            })]);
        }

        // Modify Rails so that non-Impa Portraits are reachable
        // TODO modify ActorProfile collision entries so this won't be needed

        let fn_extend_rails = move |rail: &mut Rail| {
            let rails_len = rail.pnt.len();
            rail.pnt.get_mut(rails_len - 1).unwrap().srt.translate.y += DY;

            let mut p = rail.pnt.get(rail.pnt.len() - 1).unwrap().clone();
            p.srt.translate.z += DZ;
            rail.pnt.push(p);
        };

        patcher.modify_rails(DungeonKame, 3, &[
            call_rail(1, fn_extend_rails),  // dgn_Kame_Pillar
            call_rail(17, fn_extend_rails), // Impa
        ]);
    }
}

/// Desert Palace
fn patch_desert(patcher: &mut Patcher, prize: Item) {
    // Debug stuff
    // patcher.modify_objs(FieldDark, 31, &[
    //     disable(67), // Zaganaga
    //     disable(74), // Holocaust
    // ]);
    // patcher.add_obj(FieldDark, 31,
    //                 Obj::step_switch(Flag::Course(252), 0, 58, 137,
    //                                  Vec3 { x: -19.0, y: 0.0, z: -19.0 }));

    if prize == SageIrene {
        return;
    }

    modify_dungeon_reward(patcher, prize, 76, FieldDark, 31, true, Dest::new(FieldDark, 31, 30));

    let prize_flag = prize_flag(prize);
    patcher.modify_objs(FieldDark, 31, &[
        set_enable_flag(132, prize_flag), // Warp to leave boss area
        set_enable_flag(133, prize_flag), // Destination Warp
    ]);
}

/// Ice Ruins
fn patch_ice(patcher: &mut Patcher, prize: Item) {
    // Debug stuff
    // patcher.modify_system(DungeonIce, 1, &[
    //     call(68, |obj| {
    //         obj.srt.translate.z = -31.6 + 3.0;
    //     }),
    // ]);

    if prize == SageRosso {
        return;
    }

    modify_dungeon_reward(patcher, prize, 16, DungeonIce, 1, true, Dest::new(FieldDark, 5, 0));
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

    fn get(prize: Item) -> Self {
        match prize {
            SageGulley => Self::new(418, Flag::Event(536), 0.0, 0, 1, 60),
            SageOren => Self::new(423, Flag::Event(556), 330.0, 0, 0, 30),
            SageSeres => Self::new(420, Flag::Event(576), 330.0, 0, 0, 30),
            SageOsfala => Self::new(419, Flag::Event(596), 330.0, 0, 0, 30),
            SageRosso => Self::new(422, Flag::Event(616), 330.0, 0, 0, 30),
            SageIrene => Self::new(417, Flag::Event(636), 330.0, 0, 0, 30),
            SageImpa => Self::new(421, Flag::Event(656), 330.0, 0, 0, 120),
            PendantPower => Self::new(173, Flag::Event(372), 0.0, 0, 0, 0),
            PendantWisdom => Self::new(173, Flag::Event(342), 0.0, 1, 0, 0),
            PendantCourage => Self::new(173, Flag::Course(500), 0.0, 2, 0, 0),
            ZeldaAmulet => Self::new(173, Flag::Course(501), 0.0, 2, 0, 0),
            _ => fail!("\"{}\" is not a dungeon prize.", prize.as_str()),
        }
    }
}

fn reroute_sage_warp(patcher: &mut Patcher, prize: Item, dest: Dest) {
    // Get UNQ of warp object in the Chamber of Sages
    let unq_sage_warp = match prize {
        SageGulley => Some(73),
        SageOren => Some(72),
        SageSeres => Some(71),
        SageOsfala => Some(67),
        SageRosso => Some(69),
        SageIrene => Some(70),
        SageImpa => Some(68),
        PendantPower | PendantWisdom | PendantCourage | ZeldaAmulet => None,
        _ => fail!("\"{}\" is not a dungeon prize.", prize.as_str()),
    };

    // Reroute
    if let Some(unq_sage_warp) = unq_sage_warp {
        patcher.modify_objs(CaveDark, 10, &[redirect(unq_sage_warp, dest)]);
    }
}

fn modify_dungeon_reward(
    patcher: &mut Patcher, prize: Item, unq: u16, scene: Id, scene_index: u16, activate: bool,
    dest: Dest,
) {
    let data = PrizePatchData::get(prize);
    patcher.modify_objs(scene, scene_index, &[call(unq, move |obj| {
        obj.set_id(data.actor_id);
        obj.arg.1 = data.arg1;
        if activate {
            obj.set_active_flag(Flag::Event(1));
        }
        obj.set_inactive_flag(data.flag);
        obj.set_rotate(data.rot_x, 0.0, 0.0);
        obj.set_disable_flag(data.flag);
        if is_pendant(prize) {
            obj.redirect(dest);
        }
    })]);
    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, dest);
    }
}

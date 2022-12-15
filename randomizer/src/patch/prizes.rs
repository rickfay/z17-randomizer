use std::collections::HashMap;
use albw::course::Id::*;
use albw::{Actor, byaml, File, Item};
use albw::course::Id;
use albw::Item::*;
use albw::language::FlowChart;
use albw::scene::{Arg, Flag, Obj, Point, Rail, Transform, Vec3};
use crate::patch::DungeonPrizes;
use crate::{ItemExt, MsbfKey, Patcher, Settings};
use crate::patch::util::*;

pub(crate) fn patch_dungeon_prizes(patcher: &mut Patcher, prizes: &DungeonPrizes, settings: &Settings) {
    patch_flowchart(patcher, &prizes);
    patch_msbf_files(patcher, &prizes);
    patch_dungeon_prize_actors(patcher, &prizes);
    patch_actor_profile(patcher, &prizes);
    patch_prize_byaml(patcher, &prizes, settings);
}

/// Adds entries to the FlowChart for the MSBF files related to each Portrait
fn patch_flowchart(patcher: &mut Patcher, prizes: &DungeonPrizes) {

    // Map dungeon MsbfInfo to the randomized prizes
    let dungeon_msbf_mapping: Vec<(Option<&'static str>, Id)> = Vec::from([
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
            flow_chart.get_mut().load_mut().add_entry(
                course.as_str(), new_msbf.unwrap());
        }
    }

    // Serialize the FlowChart and update the boot archive
    let serialized = flow_chart.serialize();
    patcher.boot.archive.get_mut().update(serialized).unwrap();
}

/// Get msbf event files and inject them into scenes
fn patch_msbf_files(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    let prize_msbf_map: HashMap<Item, (&str, File<Box<[u8]>>)> = HashMap::from([
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
    patcher.inject_msbf(DungeonDark, prize_msbf_map.get(&prizes.pd_prize)).unwrap();
    patcher.inject_msbf(DungeonWater, prize_msbf_map.get(&prizes.sp_prize)).unwrap();
    patcher.inject_msbf(FieldDark, prize_msbf_map.get(&prizes.sw_prize)).unwrap();
    patcher.inject_msbf(IndoorDark, prize_msbf_map.get(&prizes.tt_prize)).unwrap();
    patcher.inject_msbf(DungeonKame, prize_msbf_map.get(&prizes.tr_prize)).unwrap();
    patcher.inject_msbf(FieldDark, prize_msbf_map.get(&prizes.dp_prize)).unwrap();
    patcher.inject_msbf(DungeonIce, prize_msbf_map.get(&prizes.ir_prize)).unwrap();
}

/// Inject the prize actors into the relevant scenes
fn patch_dungeon_prize_actors(patcher: &mut Patcher, prizes: &DungeonPrizes) {

    // Fetch and map Actors to their dungeon prizes
    let pendant = patcher.scene(DungeonWind, 2).unwrap().actors().get_actor_bch("Pendant").unwrap();
    let actor_map: HashMap<Item, Actor> = HashMap::from([
        (PendantCourage, pendant.clone()),
        (PendantWisdom, pendant.clone()),
        (PendantPower, pendant),
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
    patcher.scene(DungeonDark, 0).unwrap().actors_mut().add(actor_map.get(&prizes.pd_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonWater, 2).unwrap().actors_mut().add(actor_map.get(&prizes.sp_prize).unwrap().clone()).unwrap();
    patcher.scene(FieldDark, 0).unwrap().actors_mut().add(actor_map.get(&prizes.sw_prize).unwrap().clone()).unwrap();
    patcher.scene(IndoorDark, 14).unwrap().actors_mut().add(actor_map.get(&prizes.tt_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonKame, 2).unwrap().actors_mut().add(actor_map.get(&prizes.tr_prize).unwrap().clone()).unwrap();
    patcher.scene(FieldDark, 30).unwrap().actors_mut().add(actor_map.get(&prizes.dp_prize).unwrap().clone()).unwrap();
    patcher.scene(DungeonIce, 0).unwrap().actors_mut().add(actor_map.get(&prizes.ir_prize).unwrap().clone()).unwrap();

    // Inject Small Chests into scenes that don't have them for Pendants
    // TODO Remove after Pendants can be redirected
    let chest_small = patcher.scene(DungeonHera, 0).unwrap().actors().get_actor_bch("TreasureBoxS").unwrap();
    if is_pendant(prizes.sp_prize) {
        let warp_tile = patcher.scene(DungeonHera, 0).unwrap().actors().get_actor_bch("WarpTile").unwrap();
        patcher.scene(DungeonWater, 2).unwrap().actors_mut().add(warp_tile).unwrap();
        patcher.scene(DungeonWater, 2).unwrap().actors_mut().add(chest_small.clone()).unwrap();
    }
    if is_pendant(prizes.sw_prize) {
        patcher.scene(FieldDark, 0).unwrap().actors_mut().add(chest_small.clone()).unwrap();
    }
    if is_pendant(prizes.tt_prize) {
        patcher.scene(IndoorDark, 14).unwrap().actors_mut().add(chest_small.clone()).unwrap();
    }
    if is_pendant(prizes.dp_prize) {
        patcher.scene(FieldDark, 30).unwrap().actors_mut().add(chest_small.clone()).unwrap();
    }
    if is_pendant(prizes.tr_prize) {
        patcher.scene(DungeonKame, 2).unwrap().actors_mut().add(chest_small).unwrap();
    }
}

/// TODO
fn patch_actor_profile(_patcher: &mut Patcher, _prizes: &DungeonPrizes) {

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
}

fn patch_prize_byaml(patcher: &mut Patcher, prizes: &DungeonPrizes, settings: &Settings) {
    patch_eastern(patcher, prizes.ep_prize, settings);
    patch_gales(patcher, prizes.hg_prize, settings);
    patch_hera(patcher, prizes.th_prize, settings);
    patch_dark(patcher, prizes.pd_prize, settings);
    patch_swamp(patcher, prizes.sp_prize, settings);
    patch_skull(patcher, prizes.sw_prize, settings);
    patch_thieves(patcher, prizes.tt_prize, settings);
    patch_turtle(patcher, prizes.tr_prize, settings);
    patch_desert(patcher, prizes.dp_prize, settings);
    patch_ice(patcher, prizes.ir_prize, settings);
}

/// Eastern Palace
fn patch_eastern(patcher: &mut Patcher, prize: Item, _: &Settings) {
    let data = PrizePatchData::get(prize);

    // Eastern Palace 1F - Add Dungeon Reward
    patcher.add_obj(DungeonEast, 1, Obj {
        arg: Arg(0, data.arg1, 0, 0, 4, 4, 250, data.flag, 0, data.arg9, data.arg10, 0, data.arg12, 0.0),
        clp: 0,
        flg: (4, 4, 250, data.flag),
        id: data.actor_id,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(129),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: data.rot_x, y: 0.0, z: 0.0 },
            translate: Vec3 { x: 0.0, y: 2.5, z: -5.75 },
        },
        typ: 4,
        unq: 301,
    });

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, 5, 0, 17); // Outside Hyrule Castle
    } else {
        patcher.modify_objs(DungeonEast, 1, &[
            remove_collision(301),
        ]);

        let pendant_flag = prize_flag(prize);
        patcher.add_obj(DungeonEast, 1,
                        Obj::pendant_chest(prize, Flag::Event(250), pendant_flag,
                                           0, 130, 302,
                                           Vec3 { x: 0.0, y: 2.5, z: -5.75 }));
        patcher.add_obj(DungeonEast, 1,
                        Obj::blue_warp(pendant_flag,
                                       0, 131, 303,
                                       5, 0, 17,
                                       Vec3 { x: -3.5, y: 2.5, z: -3.0 }));
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
        disable(200), // Sahasrahla
        disable(208), // Textbox trigger
        disable(264), // lgt_NpcSoldier_Field1B_04_broke - idk what this is, but now it's nothing
        disable(529), // AreaSwitchCube
        // enable(502), // Sahasrahla
    ]);
    patcher.modify_system(FieldLight, 18, &[
        call(199, |obj| {
            obj.srt.translate.z = 12.75; // move to where cutscene normally ends
        }),
    ]);

    // Rewire Post-EP checks to require PoC
    let green_pendant_flag = prize_flag(PendantCourage);

    // Haunted Grove
    patcher.modify_objs(FieldLight, 25, &[
        set_enable_flag(122, green_pendant_flag), // Pouch
        set_disable_flag(123, green_pendant_flag), // Gulley
    ]);

    // Irene (bridge)
    patcher.modify_objs(FieldLight, 28, &[
        set_46_args(55, green_pendant_flag), // Trigger - NpcMaple_BellGet_2D
        set_enable_flag(56, green_pendant_flag), // Irene
    ]);

    // Irene (Fortune-Teller)
    patcher.modify_objs(FieldLight, 9, &[
        set_46_args(83, green_pendant_flag), // Trigger - NpcMaple_BellGet_11
        set_enable_flag(85, green_pendant_flag), // Irene
    ]);

    // Irene (small pond)
    patcher.modify_objs(FieldLight, 10, &[
        set_46_args(65, green_pendant_flag), // Trigger - NpcMaple_BellGet_12_00
        set_enable_flag(67, green_pendant_flag), // Irene
        set_46_args(68, green_pendant_flag), // Trigger - NpcMaple_BellGet_12_01
    ]);

    // Outside Rosso's House
    patcher.modify_objs(FieldLight, 2, &[
        set_enable_flag(11, green_pendant_flag), // Small Rock (controller, see below)
        disable(88), // early game LZ to Rosso's House
        clear_disable_flag(100), // Keep Entry_KikoriMan3 from disappearing
        clear_disable_flag(100), // NpcMountaineer
        set_disable_flag(128, green_pendant_flag), // "Not in right now." signboard
        set_46_args(132, green_pendant_flag), // Door
        disable(135), // Disable LZ to IndoorLight4 cutscene
        set_enable_flag(136, green_pendant_flag), // LZ to Rosso's House
    ]);

    // Rosso's House
    patcher.modify_objs(IndoorLight, 10, &[
        call(7, |obj| {
            obj.set_inactive_flag(Flag::Event(282));
            obj.enable();
        }),
    ]);

    // Small Rocks
    patcher.modify_system(FieldLight, 2, &[
        set_enable_flag(11, green_pendant_flag), // controller
        set_enable_flag(12, green_pendant_flag),
        set_enable_flag(14, green_pendant_flag),
        set_enable_flag(15, green_pendant_flag),
        set_enable_flag(16, green_pendant_flag),
        set_enable_flag(18, green_pendant_flag),
        set_enable_flag(19, green_pendant_flag),
        set_enable_flag(20, green_pendant_flag),
        set_enable_flag(21, green_pendant_flag),
        set_enable_flag(93, green_pendant_flag),
        set_enable_flag(94, green_pendant_flag),
        set_enable_flag(102, green_pendant_flag),
        set_enable_flag(103, green_pendant_flag),
        set_enable_flag(104, green_pendant_flag),
        set_enable_flag(105, green_pendant_flag),
        set_enable_flag(106, green_pendant_flag),
        set_enable_flag(107, green_pendant_flag),
        set_enable_flag(108, green_pendant_flag),
        set_enable_flag(109, green_pendant_flag),
        set_enable_flag(110, green_pendant_flag),
        set_enable_flag(111, green_pendant_flag),
        set_enable_flag(112, green_pendant_flag),
        set_enable_flag(118, green_pendant_flag),
        set_enable_flag(119, green_pendant_flag),
        set_enable_flag(120, green_pendant_flag),
        set_enable_flag(121, green_pendant_flag),
        set_enable_flag(122, green_pendant_flag),
        set_enable_flag(123, green_pendant_flag),
        set_enable_flag(124, green_pendant_flag),
        set_enable_flag(125, green_pendant_flag),
        set_enable_flag(126, green_pendant_flag),
    ]);
}

/// House of Gales
fn patch_gales(patcher: &mut Patcher, prize: Item, _: &Settings) {

    // Debug stuff
    // patcher.modify_objs(DungeonWind, 3, &[
    //     disable(436), // Margomill
    //     disable(457), // Holocaust
    // ]);
    // patcher.add_obj(DungeonWind, 3,
    //                 Obj::step_switch(Flag::Event(340), 0, 102, 564,
    //                                  Vec3 { x: 0.0, y: 0.0, z: -39.5 }));

    if prize == PendantWisdom {
        return;
    }

    let prize_flag = prize_flag(prize);
    patcher.modify_objs(DungeonWind, 3, &[
        modify_reward(459, prize, false),
        set_enable_flag(490, prize_flag), // Warp to leave boss room
        set_enable_flag(543, prize_flag), // Destination Warp
    ]);

    if is_pendant(prize) {
        patcher.modify_objs(DungeonWind, 3, &[
            remove_collision(459),
            call(459, |obj| {
                obj.lnk.clear(); // insta-spawn since chest will already be there
            }),
        ]);

        patcher.add_obj(DungeonWind, 3,
                        Obj::pendant_chest(prize, Flag::Event(340), prize_flag,
                                           0, 100, 562,
                                           Vec3 { x: 0.0, y: 0.0, z: -46.5 }));
        patcher.add_obj(DungeonWind, 3,
                        Obj::blue_warp(prize_flag,
                                       0, 101, 563,
                                       0, 0, 34,
                                       Vec3 { x: 0.0, y: 0.0, z: -44.75 }));
    } else {
        reroute_sage_warp(patcher, prize, 0, 0, 34);

        if prize != SageGulley {
            patcher.modify_objs(DungeonWind, 3, &[
                add_rail(459, (12, 0)),
            ]);

            let (end_y, end_z) = if prize == SageImpa { (2.0, -47.0) } else { (0.0, -46.5) };
            patcher.add_rail(DungeonWind, 3, Rail {
                arg: (0, 0, 0, 0, 0.0, 0.0),
                pnt: vec![
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [
                            0.0, end_y, end_z - 15.0,
                            0.0, end_y, end_z - 15.0
                        ],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z - 15.0 },
                        },
                    },
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [
                            0.0, end_y, end_z,
                            0.0, end_y, end_z
                        ],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
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
fn patch_hera(patcher: &mut Patcher, prize: Item, _: &Settings) {

    // Debug stuff
    // patcher.modify_objs(DungeonHera, 1, &[
    //     disable(737), // Moldorm
    //     disable(738), // Holocaust
    // ]);
    // patcher.add_obj(DungeonHera, 1,
    //                 Obj::step_switch(Flag::Event(370), 20, 313, 920,
    //                                  Vec3 { x: 0.0, y: 101.0, z: -1.5 }));

    if prize == PendantPower {
        return;
    }

    patcher.modify_objs(DungeonHera, 1, &[
        modify_reward(829, prize, false),
    ]);

    if is_pendant(prize) {
        patcher.modify_objs(DungeonHera, 1, &[
            remove_collision(829),
            call(829, |obj| {
                obj.lnk.clear(); // insta-spawn since chest will already be there
            }),
        ]);

        let pendant_flag = prize_flag(prize);
        patcher.add_obj(DungeonHera, 1,
                        Obj::pendant_chest(prize, Flag::Event(370), pendant_flag,
                                           20, 314, 921,
                                           Vec3 { x: 0.0, y: 101.0, z: -5.5 }));
        patcher.add_obj(DungeonHera, 1,
                        Obj::blue_warp(pendant_flag,
                                       20, 315, 922,
                                       3, 0, 2,
                                       Vec3 { x: 0.0, y: 101.0, z: -1.5 }));
    } else {
        reroute_sage_warp(patcher, prize, 3, 0, 2);

        if prize != SageGulley {
            patcher.modify_objs(DungeonHera, 1, &[
                add_rail(829, (56, 0)),
            ]);

            let (end_y, end_z) = if prize == SageImpa { (103.0, -6.0) } else { (101.0, -5.5) };
            patcher.add_rail(DungeonHera, 1, Rail {
                arg: (0, 0, 0, 0, 0.0, 0.0),
                pnt: vec![
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [
                            0.0, end_y, end_z - 15.0,
                            0.0, end_y, end_z - 15.0
                        ],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
                            translate: Vec3 { x: 0.0, y: end_y, z: end_z - 15.0 },
                        },
                    },
                    Point {
                        arg: (0, 0, 0, 0, 0.0, 0.0),
                        ctl: [
                            0.0, end_y, end_z,
                            0.0, end_y, end_z
                        ],
                        lnk: vec![],
                        srt: Transform {
                            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
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

/// Dark Palace
fn patch_dark(patcher: &mut Patcher, prize: Item, _: &Settings) {

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

    let prize_flag = prize_flag(prize);

    patcher.modify_objs(DungeonDark, 1, &[
        modify_reward(262, prize, false),
    ]);

    if is_pendant(prize) {
        patcher.modify_objs(DungeonDark, 1, &[
            call(262, |obj| {
                obj.srt.scale = Vec3 { x: 0.0, y: 0.0, z: 0.0 }; // remove collision
            }),
            disable(121), // ObjJewelMask Camera
        ]);


        patcher.add_obj(DungeonDark, 1,
                        Obj::pendant_chest(prize, Flag::Course(21), prize_flag,
                                           4, 155, 300,
                                           Vec3 { x: 0.0, y: 0.0, z: -47.5 }));
        patcher.add_obj(DungeonDark, 1,
                        Obj::blue_warp(prize_flag,
                                       4, 156, 301,
                                       5, 1, 19,
                                       Vec3 { x: 0.0, y: 0.0, z: -44.75 }));
    } else { // is_sage(prize)

        reroute_sage_warp(patcher, prize, 5, 1, 19);

        patcher.modify_objs(DungeonDark, 1, &[
            add_rail(262, (14, 0)),
        ]);

        let (end_y, end_z) = if prize == SageImpa { (2.0, -48.0) } else { (0.0, -47.5) };
        patcher.add_rail(DungeonDark, 1, Rail {
            arg: (0, 0, 0, 0, 0.0, 0.0),
            pnt: vec![
                Point {
                    arg: (0, 0, 0, 0, 0.0, 0.0),
                    ctl: [
                        0.0, 6.7700896, -48.9483577,
                        0.0, 6.7700896, -48.9483577
                    ],
                    lnk: vec![],
                    srt: Transform {
                        scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                        rotate: Vec3 { x: 335.99999945441, y: 0.0, z: 0.0 },
                        translate: Vec3 { x: 0.0, y: 6.7700896, z: -48.9483577 },
                    },
                },
                Point {
                    arg: (0, 0, 0, 0, 0.0, 0.0),
                    ctl: [
                        0.0, end_y, end_z,
                        0.0, end_y, end_z
                    ],
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

    // // Remove Maze Guards after Dark Palace
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
fn patch_swamp(patcher: &mut Patcher, prize: Item, _: &Settings) {
    if prize == SageOren {
        return;
    }

    patcher.modify_objs(DungeonWater, 3, &[
        modify_reward(13, prize, true),
    ]);

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, 0, 1, 32);
    } else {
        patcher.modify_objs(DungeonWater, 3, &[
            remove_collision(13),
        ]);

        let pendant_flag = prize_flag(prize);
        patcher.add_obj(DungeonWater, 3,
                        Obj::pendant_chest(prize, Flag::Event(1), pendant_flag,
                                           0, 14, 25,
                                           Vec3 { x: 0.0, y: 2.5, z: -40.15 }));
        patcher.add_obj(DungeonWater, 3,
                        Obj::warp_tile(pendant_flag,
                                       0, 15, 26,
                                       0, 1, 32,
                                       Vec3 { x: 0.0, y: 0.0, z: -32.0 }));
    }
}

/// Skull Woods
fn patch_skull(patcher: &mut Patcher, prize: Item, _: &Settings) {
    if prize == SageSeres {
        return;
    }

    patcher.modify_objs(FieldDark, 1, &[
        modify_reward(273, prize, true),
    ]);

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, 10, 1, 0);
    } else {
        patcher.modify_objs(FieldDark, 1, &[
            remove_collision(273),
        ]);

        let pendant_flag = prize_flag(prize);
        patcher.add_obj(FieldDark, 1,
                        Obj::pendant_chest(prize, Flag::Event(1), pendant_flag,
                                           0, 76, 533,
                                           Vec3 { x: -6.0, y: 0.0, z: -16.5 }));
        patcher.add_obj(FieldDark, 1,
                        Obj::blue_warp(pendant_flag,
                                       0, 77, 534,
                                       10, 1, 0,
                                       Vec3 { x: -6.0, y: 0.0, z: -15.0 }));
    }
}

/// Thieves' Hideout
fn patch_thieves(patcher: &mut Patcher, prize: Item, _: &Settings) {
    if prize == SageOsfala {
        return;
    }

    patcher.modify_objs(IndoorDark, 15, &[
        modify_reward(3, prize, true),
    ]);

    let prize_flag = prize_flag(prize);
    if is_pendant(prize) {
        patcher.modify_objs(IndoorDark, 15, &[
            remove_collision(3),
        ]);


        patcher.add_obj(IndoorDark, 15,
                        Obj::pendant_chest(prize, Flag::Event(1), prize_flag,
                                           0, 7, 13,
                                           Vec3 { x: 0.0, y: 0.0, z: -8.0 }));
    } else {
        reroute_sage_warp(patcher, prize, 14, 1, 15);
    }
}

/// Turtle Rock
fn patch_turtle(patcher: &mut Patcher, prize: Item, _: &Settings) {

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

    patcher.modify_objs(DungeonKame, 3, &[
        modify_reward(56, prize, false),
    ]);

    let dy = -2.0;
    let dz = 0.5;

    if is_pendant(prize) {
        patcher.modify_objs(DungeonKame, 3, &[
            remove_collision(56),
            call(56, move |obj| {
                obj.srt.translate.z = -44.0;
                obj.srt.translate.y = 5.0;
            }),
            disable(9), // dgn_Kame_Pillar
        ]);

        let pendant_flag = prize_flag(prize);
        patcher.add_obj(DungeonKame, 3,
                        Obj::pendant_chest(prize, Flag::Course(130), pendant_flag,
                                           0, 33, 72,
                                           Vec3 { x: 0.0, y: 5.0, z: -44.0 }));
        patcher.add_obj(DungeonKame, 3,
                        Obj::blue_warp(pendant_flag,
                                       0, 34, 73,
                                       6, 1, 34,
                                       Vec3 { x: 0.0, y: 5.0, z: -39.0 }));
    } else {
        reroute_sage_warp(patcher, prize, 6, 1, 34);

        if prize == SageGulley {
            patcher.modify_objs(DungeonKame, 3, &[
                call(56, move |obj| {
                    obj.srt.translate.z += dz;
                    obj.srt.translate.y += 15.0 + dy;
                }),
            ]);
        }

        // Modify Rails so that non-Impa Portraits are reachable TODO modify ActorProfile
        let fn_extend_rails = move |rail: &mut Rail| {
            let rails_len = rail.pnt.len();
            rail.pnt.get_mut(rails_len - 1).unwrap().srt.translate.y += dy;

            let mut p = rail.pnt.get(rail.pnt.len() - 1).unwrap().clone();
            p.srt.translate.z += dz;
            rail.pnt.push(p);
        };

        patcher.modify_rails(DungeonKame, 3, &[
            call_rail(1, fn_extend_rails), // dgn_Kame_Pillar
            call_rail(17, fn_extend_rails), // Impa
        ]);
    }
}

/// Desert Palace
fn patch_desert(patcher: &mut Patcher, prize: Item, _: &Settings) {

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

    let prize_flag = prize_flag(prize);
    patcher.modify_objs(FieldDark, 31, &[
        modify_reward(76, prize, true),
        set_enable_flag(132, prize_flag), // Warp to leave boss area
        set_enable_flag(133, prize_flag), // Destination Warp
    ]);

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, 30, 1, 30);
    } else {
        patcher.modify_objs(FieldDark, 31, &[
            remove_collision(76),
        ]);


        patcher.add_obj(FieldDark, 31,
                        Obj::pendant_chest(prize, Flag::Event(1), prize_flag,
                                           0, 56, 135,
                                           Vec3 { x: -13.0, y: 0.0, z: -24.0 }));
        patcher.add_obj(FieldDark, 31,
                        Obj::blue_warp(prize_flag,
                                       0, 57, 136,
                                       30, 1, 30,
                                       Vec3 { x: -10.5, y: 0.0, z: -21.25 }));
    }
}

/// Ice Ruins
fn patch_ice(patcher: &mut Patcher, prize: Item, _: &Settings) {

    // Debug stuff
    // patcher.modify_system(DungeonIce, 1, &[
    //     call(68, |obj| {
    //         obj.srt.translate.z = -31.6 + 3.0;
    //     }),
    // ]);

    if prize == SageRosso {
        return;
    }

    patcher.modify_objs(DungeonIce, 1, &[
        modify_reward(16, prize, true),
    ]);

    if is_sage(prize) {
        reroute_sage_warp(patcher, prize, 0, 1, 4);
    } else {
        patcher.modify_objs(DungeonIce, 1, &[
            remove_collision(16),
        ]);

        let pendant_flag = prize_flag(prize);
        patcher.add_obj(DungeonIce, 1,
                        Obj::pendant_chest(prize, Flag::Event(1), pendant_flag,
                                           6, 408, 1214,
                                           Vec3 { x: 0.0, y: 0.0, z: -31.6 }));
        patcher.add_obj(DungeonIce, 1,
                        Obj::blue_warp(pendant_flag,
                                       6, 409, 1215,
                                       0, 1, 4,
                                       Vec3 { x: 0.0, y: 0.0, z: -30.1 }));
    }
}

struct PrizePatchData {
    actor_id: i16,
    flag: u16,
    rot_x: f32,
    arg1: i32,
    arg9: i32,
    arg10: i32,
    arg12: i32,
}

impl PrizePatchData {
    fn new(actor_id: i16, flag: u16, rot_x: f32, arg1: i32, arg9: i32, arg10: i32, arg12: i32) -> Self {
        Self { actor_id, flag, rot_x, arg1, arg9, arg10, arg12 }
    }

    fn get(prize: Item) -> Self {
        match prize {
            SageGulley => Self::new(418, 536, 0.0, 0, 1, 180, 60),
            SageOren => Self::new(423, 556, 330.0, 0, 0, 0, 30),
            SageSeres => Self::new(420, 576, 330.0, 0, 0, 0, 30),
            SageOsfala => Self::new(419, 596, 330.0, 0, 0, 0, 30),
            SageRosso => Self::new(422, 616, 330.0, 0, 0, 0, 30),
            SageIrene => Self::new(417, 636, 330.0, 0, 0, 0, 30),
            SageImpa => Self::new(421, 656, 330.0, 0, 0, 0, 120),
            PendantPower => Self::new(173, 372, 0.0, 0, 0, 0, 0),
            PendantWisdom => Self::new(173, 342, 0.0, 1, 0, 0, 0),
            PendantCourage => Self::new(173, 310, 0.0, 2, 0, 0, 0),
            _ => panic!("\"{}\" is not a dungeon prize.", prize.as_str())
        }
    }
}

fn reroute_sage_warp(patcher: &mut Patcher, prize: Item, spawn_point: i32, scene: i32, scene_index: i32) {

    // Get UNQ of warp object in the Chamber of Sages
    let unq_sage_warp = match prize {
        SageGulley => Some(73),
        SageOren => Some(72),
        SageSeres => Some(71),
        SageOsfala => Some(67),
        SageRosso => Some(69),
        SageIrene => Some(70),
        SageImpa => Some(68),
        PendantPower | PendantWisdom | PendantCourage => None,
        _ => panic!("\"{}\" is not a dungeon prize.", prize.as_str())
    };

    // Reroute
    if let Some(unq_sage_warp) = unq_sage_warp {
        patcher.modify_objs(CaveDark, 10, &[
            redirect(unq_sage_warp, spawn_point, scene, scene_index),
        ]);
    }
}

fn modify_reward(unq: u16, prize: Item, activate: bool) -> (u16, Box<dyn Fn(&mut Obj)>) {
    let data = PrizePatchData::get(prize);
    (unq, Box::new(move |obj: &mut Obj| {
        obj.set_id(data.actor_id);
        obj.arg.1 = data.arg1;
        if activate {
            obj.set_active_flag(Flag::Event(1));
        }
        obj.set_inactive_flag(Flag::Event(data.flag));
        obj.set_rotate(data.rot_x, 0.0, 0.0);
        obj.set_disable_flag(Flag::Event(data.flag));
    }))
}
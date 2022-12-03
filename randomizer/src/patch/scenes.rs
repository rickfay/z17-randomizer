
use albw::{course, Item, scene::{Flag, Obj}};
use albw::Item::*;
use albw::course::Id::*;
use albw::scene::{Arg, Point, Rail, Transform, Vec3};

use super::Patcher;
use crate::{Result, Settings};
use crate::logic_mode::LogicMode;
use crate::patch::DungeonPrizes;

macro_rules! apply {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(course::Id::$course, $stage - 1)?.stage_mut().get_mut();
            $(action!((stage
                .get_obj_mut($unq)
                .ok_or_else(|| $crate::Error::game("Could not find scene."))?
            ).$action $value);)+
        })+
    };
}

macro_rules! apply_system {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(course::Id::$course, $stage - 1)?.stage_mut().get_mut();
            $(action!((stage
                .get_system_mut($unq)
                .ok_or_else(|| $crate::Error::game("Could not find scene."))?
            ).$action $value);)+
        })+
    };
}

macro_rules! action {
    ($unq:tt.id($id:literal)) => {
        $unq.set_id($id);
    };
    ($unq:tt.each [$($action:ident $value:tt,)+]) => {
        $(action!($unq.$action $value);)+
    };
    ($unq:tt.active(0)) => {
        $unq.set_active_flag(None);
    };
    ($unq:tt.active($flag:literal)) => {
        $unq.set_active_flag(Flag::Event($flag));
    };
    ($unq:tt.inactive($flag:literal)) => {
        $unq.set_inactive_flag(Flag::Event($flag));
    };
    ($unq:tt.enable($flag:expr)) => {
        $unq.set_enable_flag($flag);
    };
    ($unq:tt.disable($flag:expr)) => {
        $unq.set_disable_flag($flag);
    };
    ($unq:tt.clear_enable_flag()) => {
        $unq.clear_enable_flag();
    };
    ($unq:tt.clear_disable_flag()) => {
        $unq.clear_disable_flag();
    };
    ($unq:tt.clear_active_args()) => {
        $unq.clear_active_args();
    };
    ($unq:tt.clear_inactive_args()) => {
        $unq.clear_inactive_args();
    };
    ($unq:tt.enable()) => {
        $unq.enable();
    };
    ($unq:tt.disable()) => {
        $unq.disable();
    };
    ($unq:tt.call $fn:block) => {
        ($fn)($unq);
    };
    ($unq:tt.set_translate($x:literal, $y:literal, $z:literal)) => {
        $unq.set_translate($x, $y, $z);
    };
    ($unq:tt.redirect($spawn_point:literal, $scene:literal, $scene_index:literal)) => {
        $unq.redirect($spawn_point, $scene, $scene_index);
    };
}

fn call<F>(unq: u16, action: F) -> (u16, Box<dyn Fn(&mut Obj)>)
    where
        F: Fn(&mut Obj) + 'static
{
    (unq, Box::new(action))
}

fn call_rail<F>(unq: u16, action: F) -> (u16, Box<dyn Fn(&mut Rail)>)
    where
        F: Fn(&mut Rail) + 'static
{
    (unq, Box::new(action))
}

fn set_46_args(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_active_flag(flag) }))
}

#[allow(unused)]
fn set_57_args(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_inactive_flag(flag) }))
}

fn enable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.enable() }))
}

fn disable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.disable() }))
}

fn set_enable_flag(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_enable_flag(flag) }))
}

fn set_disable_flag(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_disable_flag(flag) }))
}

#[allow(unused)]
fn clear_enable_flag(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_enable_flag(None) }))
}

fn clear_disable_flag(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_disable_flag(None) }))
}

fn redirect(unq: u16, spawn_point: i32, scene: i32, scene_index: i32) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.redirect(spawn_point, scene, scene_index) }))
}

fn add_rail(unq: u16, rail: (i32, i32)) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.ril.push(rail) }))
}

fn remove_collision(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.srt.scale = Vec3 { x: 0.0, y: 0.0, z: 0.0 } }))
}

fn is_sage(item: Item) -> bool {
    match item {
        SageGulley | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso => true,
        _ => false
    }
}

pub fn is_pendant(item: Item) -> bool {
    match item {
        PendantPower | PendantWisdom | PendantCourage => true,
        _ => false
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

fn prize_flag(pendant: Item) -> Flag {
    match pendant {
        PendantPower => Flag::Event(372),
        PendantWisdom => Flag::Event(342),
        PendantCourage => Flag::Event(310),
        SageGulley => Flag::Event(536),
        SageOren => Flag::Event(556),
        SageSeres => Flag::Event(576),
        SageOsfala => Flag::Event(596),
        SageRosso => Flag::Event(616),
        SageIrene => Flag::Event(636),
        SageImpa => Flag::Event(656),
        _ => panic!("{} is not a Dungeon Prize", pendant.as_str())
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

pub fn patch_prize_byaml(patcher: &mut Patcher, prizes: &DungeonPrizes, settings: &Settings) {
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

fn patch_hyrule_castle_dungeon(patcher: &mut Patcher, _: &Settings) {
    let yuga2_defeated_flag = Flag::Course(31);

    // 2F
    patcher.modify_objs(DungeonCastle, 1, &[
        set_disable_flag(19, yuga2_defeated_flag),  // Armos Statue
        call(35, move |obj| { // Warp
            obj.set_active_flag(yuga2_defeated_flag);
            obj.set_enable_flag(yuga2_defeated_flag);
        }),
    ]);

    // 4F (sic)
    patcher.modify_objs(DungeonCastle, 7, &[
        set_enable_flag(19, Flag::Event(415)),
        set_enable_flag(20, Flag::Event(415)),
        set_enable_flag(21, Flag::Event(415)),
        set_enable_flag(22, Flag::Event(415)),
    ]);

    // 7F
    patcher.modify_objs(DungeonCastle, 5, &[
        call(18, move |obj| {
            obj.set_active_flag(yuga2_defeated_flag);
            obj.set_enable_flag(yuga2_defeated_flag);
        }),
    ]);

    // 8F
    patcher.modify_objs(DungeonCastle, 6, &[
        set_disable_flag(20, yuga2_defeated_flag), // Rewire entrance door to stay open with course flag 31 (Yuga defeated)
        disable(25), // victory door
        enable(28), // no revisits door
    ]);
    patcher.add_obj(DungeonCastle, 6,
                    Obj::blue_warp(yuga2_defeated_flag,
                                   0, 19, 30,
                                   1, 3, 3,
                                   Vec3 { x: 8.0, y: 0.0, z: -19.75 }));

    // Blacksmith (Hyrule)
    patcher.add_obj(IndoorLight, 19,
                    Obj::green_warp(Flag::Event(1),
                                    0, 15, 23,
                                    3, 3, 3,
                                    Vec3 { x: 3.25, y: 0.0, z: -3.5 }));
    patcher.add_system(IndoorLight, 19,
                       Obj::spawn_point(1, 0, 16, 24,
                                        Vec3 { x: 3.25, y: 0.0, z: -3.5 }));

    // Blacksmith (Lorule)
    patcher.add_obj(IndoorDark, 4,
                    Obj::green_warp(Flag::Event(1),
                                    0, 13, 22,
                                    1, 2, 18,
                                    Vec3 { x: 3.25, y: 0.0, z: -3.5 }));
    patcher.add_system(IndoorDark, 4,
                       Obj::spawn_point(3, 0, 14, 23,
                                        Vec3 { x: 3.25, y: 0.0, z: -3.5 }));

    // Zelda's Study
    patcher.modify_objs(IndoorLight, 7, &[
        call(10, |obj| {
            obj.arg.3 = 0; // Prevent Long Portal Transition
        }),
        disable(26),  // Disable Curtain
        disable(29),  // Disable AreaDisableWallIn
    ]);
}

fn patch_master_sword(patcher: &mut Patcher, _: &Settings) {
    patcher.modify_objs(FieldLight, 34, &[
        call(71, |obj| {
            obj.clear_active_args();
            obj.set_inactive_flag(Flag::Course(150));
            obj.enable();
        }),
    ]);
}

fn patch_dark_maze(patcher: &mut Patcher, settings: &Settings) {

    // Remove dialog
    patcher.modify_objs(FieldDark, 20, &[
        disable(63), // AreaEventTalk
        disable(115), // AreaEventTalk
        disable(116), // AreaEventTalk
        disable(119), // AreaEventTalk
        disable(122), // AreaEventTalk
        disable(188), // AreaEventTalk
        disable(195), // NpcGuardMan
        disable(196), // NpcGuardMan
        disable(231), // AreaEventTalk
        disable(235), // Hilda Text
    ]);

    // Softlock Prevention for modes where Dark Maze can be reached without Merge
    if settings.logic.vanes_activated {
        // 1st Prison Cell softlock prevention
        patcher.add_obj(FieldDark, 20, Obj::warp_tile(Flag::Event(1),
                                                      0, 66, 245,
                                                      0, 1, 19,
                                                      Vec3 { x: 1.0 + 2.0, y: 0.5, z: 23.0 }));

        // 2nd Prison Cell softlock prevention
        patcher.add_obj(FieldDark, 20, Obj::warp_tile(Flag::Event(1),
                                                      0, 67, 246,
                                                      0, 1, 19,
                                                      Vec3 { x: -17.0 + 2.5, y: 0.5, z: -17.0 }));
    }
}

fn patch_thief_girl_cave(patcher: &mut Patcher, _: &Settings) {
    patcher.modify_objs(CaveDark, 15, &[

        // Thief Girl w/ Mask
        call(8, move |obj| {
            //obj.set_enable_flag(prize_flag);
            obj.srt.rotate.y = 0.0;
        }),
        //set_enable_flag(9, prize_flag), // Chest
        disable(10), // Entrance text
        disable(11), // AreaSwitchCube
        disable(13), // It's a secret to everybody
    ]);
}

// TODO figure out how to reduce coupling with patcher
pub fn apply(patcher: &mut Patcher, settings: &Settings) -> Result<()> {

    // Ravio's Shop
    patcher.modify_objs(IndoorLight, 1, &[
        call(24, |obj| {
            obj.redirect(
                5, 0, 26,   // No Redirect
                // 0, 0, 33,   // Master Sword Pedestal
                // 0, 2, 9,    // Rosso House
                // 0, 14, 2,   // Swamp Palace 2F
                // 0, 0, 1,    // FieldLight 2
                // 0, 0, 6,    // Outside Zora's Domain
                // 4, 0, 8,    // Outside Fortune-Teller
                // 0, 12, 5,   // Yuga 2 Boss
                // 1, 3, 3,    // Lorule Blacksmith
                // 0, 12, 0,   // Hyrule Castle Dungeon
                // 2, 1, 30,   // Zaganaga Portal
                // 0, 1, 30,   // Misery Mire
                // 0, 3, 14,   // Osfala Portrait
                // 0, 5, 2,    // Swamp Cave
                // 0, 5, 13,   // Great Rupee Fairy Cave
                // 1, 17, 0,   // Ice Ruins Boss
                // 0, 17, 0,   // Ice Ruins Boss
                // 0, 19, 2,   // Turtle Rock Boss
                // 0, 5, 9,    // Chamber of Sages
                // 0, 5, 14,   // Thief Girl Cave
                // 0, 0, 19,   // Eastern Ruins Cutscene
                // 5, 0, 17,   // Pendant of Courage cutscene
                // 0, 0, 24,   // Haunted Grove
                // 12, 13, 0,  // Dark Palace Boss
                // 5, 1, 19,   // Outside Dark Palace
                // 6, 10, 2,   // Gales Boss
                // 0, 9, 2,    // Eastern Palace Boss
                // 0, 9, 0,    // Eastern Palace Entrance
                // 5, 0, 19    // Eastern Ruins WV
                // 0, 9, 0     // Eastern Palace Lobby
                // 20, 1, 0,   // Seres Portrait
                // 0, 4, 3     // Kak Well Lower
                // 1, 4, 3     // Kak Well Upper
                // 10, 11, 0   // Tower of Hera Boss
            );
        }),
    ]);

    patch_master_sword(patcher, settings);
    patch_hyrule_castle_dungeon(patcher, settings);
    patch_dark_maze(patcher, settings);
    patch_thief_girl_cave(patcher, settings);

    // Chamber of Sages
    patcher.modify_objs(CaveDark, 10, &[
        set_46_args(74, Flag::Event(0)), // Staircase
    ]);

    // Old way:
    apply!(patcher,

        // Eastern Ruins Treasure Dungeon
        AttractionLight 1 {
            [15].disable(), // Skip Cutscene
        },
        // Southern Ruins Treasure Dungeon
        AttractionLight 2 {
            [54].disable(), // Skip Cutscene
        },
        // Haunted Grove Treasure Dungeon
        AttractionLight 3 {
            [47].disable(), // Skip Cutscene
        },
        // Death Mountain Treasure Dungeon
        AttractionLight 4 {
            [118].disable(), // Skip Cutscene
        },
        // Sanctuary Treasure Dungeon
        AttractionLight 5 {
            [26].disable(), // Skip Cutscene
        },



        // Lost Woods
        FieldLight 1 {
            [34].active(375), // Skip Poes
        },

        // East Death Mountain
        FieldLight 4 {
            [36].disable(), // Remove Bouldering Guy (pre-Letter in a Bottle)
            [157].clear_active_args(), // Not 100% sure what this does, but removing the association to the 916 flag
            [157].enable(), // Keep Bouldering Guy around
        },

        // Outside Fortune-Teller
        FieldLight 9 {
            [86].disable(), // Buzz Blob
            [87].disable(), // Buzz Blob
            [88].disable(), // Buzz Blob
            [89].disable(), // Buzz Blob
        },

        // Small Pond
        FieldLight 10 {
            [70].disable(), // Buzz Blob
            [71].disable(), // Buzz Blob
            [72].disable(), // Buzz Blob
        },

        // Outside Sanctuary
        FieldLight 11 {
            [81].disable(), // Buzz Blob
            [82].disable(), // Buzz Blob
            [83].disable(), // Buzz Blob
            [84].disable(), // Buzz Blob
            [85].enable(), // Green Spear Soldier
            [86].enable(), // Green Spear Soldier
            [87].enable(), // Green Spear Soldier

            [101].disable(), // Dampe
            [102].disable(), // Seres
            [133].active(1), // Close Church Door by default
            [133].disable(Flag::Event(1200)), // Church Door rigged to open when Sanc left switch pulled

            [144].disable(), // Buzz Blob
            [145].enable(), // Buzz Blob
            [146].enable(), // Buzz Blob
            [147].enable(), // Buzz Blob
        },

        // Sanctuary Dungeon
        CaveLight 18 {
            // 1200 is a newly created Flag to control this
            [35].active(1200), // Pull Switch
            [37].inactive(1200), // Door
            [107].active(1200), // TagCameraFocus
            [107].disable(Flag::Event(1200)), // TagCameraFocus
        },

        // Sanctuary Church
        IndoorLight 11 {
            [14].clear_enable_flag(), // Church Door
            [14].disable(Flag::Event(1200)), // Church Door
            [16].disable(), // Early game Priest
            [20].active(1200),
        },

        // Graveyard
        FieldLight 12 {
            [89].disable(), // Crow
            [91].disable(), // Buzz Blob
            [92].disable(), // Buzz Blob
            [93].enable(), // Arrow Soldier
            [94].enable(), // Arrow Soldier
            [162].disable(), // Crow
        },

        // Outside witch's house
        FieldLight 14 {
            [123].disable(), // Disable surprised Zora
        },
        // Kakariko Village
        FieldLight 16 {
            [259].disable(), // Papa
            [416].disable(), // Papa

            [260].disable(), // Girl
            [415].disable(), // Girl

            [241].disable(), // Cucco
            [242].disable(), // Cucco
            [413].disable(), // Cucco
            [414].disable(), // Cucco

            [197].disable(), // Disable merchant's Smooth Gem text
            [265].disable(), // Disable girl/dad text
            [299].disable(), // Disable merchant's bottle text
        },
        // Behind Blacksmith's House
        FieldLight 17 {
            [47].disable(), // Buzz Blob
            [48].disable(), // Buzz Blob
            [49].disable(), // Buzz Blob
            [58].disable(), // Buzz Blob
            [59].disable(), // Buzz Blob
            [60].disable(), // Buzz Blob
            [61].disable(), // Buzz Blob
        },
        // Hyrule Castle
        FieldLight 18 {

            [263].enable(), // Red Spear Soldier
            [536].enable(), // Red Spear Soldier

            [167].disable(), // Crow
            [168].disable(), // Crow
            [175].disable(), // Buzz Blob
            [177].disable(), // Buzz Blob
            [178].disable(), // Buzz Blob
            [179].disable(), // Buzz Blob
            [186].clear_enable_flag(), // Blue Soldier, removed after Flag 390 ?
            [187].clear_enable_flag(), // Dagger Soldier
            [189].clear_enable_flag(), // Dagger Soldier, removed after Flag 390 ?
            [190].clear_enable_flag(), // Blue Soldier
            [194].disable(), // NPC Soldier
            [195].disable(), // NPC Soldier
            [198].disable(), // NPC Soldier
            [204].clear_enable_flag(), // Arrow Soldier
            [207].clear_enable_flag(), // Blue Soldier
            [225].disable(), // Paint Soldier
            [234].disable(), // Scarecrow
            [235].disable(), // Scarecrow
            [258].clear_enable_flag(), // Bomb Soldier
            [260].clear_enable_flag(), // Bomb Soldier
            [274].disable(), // NPC Soldier
            [278].disable(), // NPC Soldier
            [279].disable(), // NPC Soldier
            [280].disable(), // NPC Soldier
            [281].disable(), // Paint Soldier
            [282].disable(), // Paint Soldier
            [301].disable(), // Paint Soldier
            [302].disable(), // Paint Soldier
            [303].disable(), // Paint Soldier
            [308].enable(), // Paint Soldier
            [309].disable(), // Paint Soldier
            [369].disable(), // Scarecrow
            [370].disable(), // Scarecrow
            [371].disable(), // NPC Soldier
            [372].disable(), // NPC Soldier
            [373].disable(), // NPC Soldier
            [395].disable(), // AreaSimpleTalk - Hekiga_Green_Soldier
            [401].disable(), // AreaSimpleTalk - Hekiga_fueta_Red
            [402].disable(), // AreaSimpleTalk - Hekiga_fueta_Green
            [403].disable(), // AreaSimpleTalk - Hekiga_Green_Soldier
            [404].disable(), // AreaSimpleTalk - Hekiga_fueta_Green
            [488].disable(), // Paint Soldier
            [491].enable(), // Paint Soldier
            [492].enable(), // Paint Soldier
            [493].enable(), // Paint Soldier
            [495].enable(), // Paint Soldier
            [496].enable(), // Paint Soldier
            [497].enable(), // Paint Soldier
            [498].enable(), // Paint Soldier
            [501].clear_enable_flag(), // TagDisableWallIn, prevent merging into barrier
            [532].disable(), // Buzz Blob
            [533].disable(), // AreaSimpleTalk - Hekiga_fueta_Green
            [534].disable(), // AreaSimpleTalk - Hekiga_Blue_Soldier
            [535].disable(), // AreaSimpleTalk - Hekiga_Blue_Soldier

            [155].enable(), // HC dungeon loading zone
            [165].active(1), // MojBarrier
            [393].disable(), // Open door to Inside Hyrule Castle
            [505].disable(), // Barrier "would you like to save?" text
        },
        // Wooden Bridge
        FieldLight 19 {
            [27].disable(), // Buzz Blob
            [28].disable(), // Buzz Blob
            [29].disable(), // Buzz Blob
            [30].disable(), // Buzz Blob
            [32].disable(), // Buzz Blob
            [35].enable(), // Arrow Solider
            [36].enable(), // Arrow Solider
            [37].enable(), // Green Spear Solider
        },

        // Cucco Ranch
        FieldLight 24 {
            [32].disable(), // Buzz Blob
            [33].disable(), // Buzz Blob
            [34].disable(), // Buzz Blob
            [38].enable(), // Dagger Soldier
            [40].enable(), // Blue Soldier
            [194].disable(), // Buzz Blob
        },

        // StreetPass Tree
        FieldLight 26 {
            [83].disable(), // Buzz Blob
            [84].disable(), // Buzz Blob
        },

        // Outside Link's house
        FieldLight 27 {
            [158].disable(), // Disable Blacksmith's Wife
        },
        // Irene Bridge
        FieldLight 28 {
            [58].disable(), // Buzz Blob
            [59].disable(), // Buzz Blob
            [60].disable(), // Buzz Blob
            [61].disable(), // Octorok
            [62].disable(), // Octorok

        },
        // Outside woods
        FieldLight 32 {
            [47].disable(), // Buzz Blob
            [48].disable(), // Buzz Blob
            [49].disable(), // Buzz Blob
            [50].disable(), // Buzz Blob
            [51].disable(), // Buzz Blob
            [76].disable(), // Disable Blacksmith's Wife
        },
        // Southern Ruins
        FieldLight 33 {
            [69].enable(), // Blue Soldier
            [70].enable(), // Blue Soldier
            [128].enable(), // Blue Soldier
            [206].disable(), // Buzz Blob
            [208].disable(), // Buzz Blob
            [342].disable(), // Buzz Blob
            [344].disable(), // Buzz Blob
            [345].disable(), // Buzz Blob
            [346].disable(), // Buzz Blob
        },

        // // Lake Hylia - Special stuff for Flora REMOVE ME
        // FieldLight 35 {
        //     [39].disable(), // Remove EnemyZora
        //     [40].disable(), // Remove EnemyZora
        //     [41].disable(), // Remove EnemyZora
        //
        //     [45].set_translate(25.0, 0.0, -24.0), // Move Weather Vane for Flora (5.5, 0.0, -3.0)
        // },

        // Hyrule Hotfoot Area
        FieldLight 36 {
            [43].disable(), // Disable Letter in a Bottle text
        },

        // Sacred Realm
        FieldLight 43 {
            [23].disable(), // seichi - "Sanctuary" - Initial text
            [26].disable(), // zelda_talk - Chat after standing up
            [32].disable(), // Remove Clouds
            [33].disable(), // zelda_talk_b - Wait for Zelda
            [34].disable(), // zelda_talk_c - Last chat before triangles
        },

        // Link's House
        IndoorLight 1 {

            // Convert standing Ravio into shopkeeper Ravio
            // [56].call {|obj: &mut Obj| {
            //     obj.arg_mut().3 = 0;
            //
            //     obj.set_active_flag(Flag::Event(233));
            //     obj.set_inactive_flag(Flag::Event(597));
            //
            //     obj.set_enable_flag(Flag::Event(233));
            //     obj.set_disable_flag(None);
            //
            //     obj.set_translate(-1.0, 0.0, -5.5);
            // }},

            // Double Sheerow
            // [57].call {|obj: &mut Obj| {
            //     obj.set_active_flag(None);
            //     obj.set_enable_flag(Flag::Event(233));
            //
            //     obj.set_disable_flag(None);
            //     obj.set_translate(-2.0, 0.0, -6.0)
            // }},

            [56].disable(), // Disable second Ravio
            [57].disable(), // Disable second Sheerow

            [46].disable(), // Disable Ravio's bye-bye
            [54].disable(), // Disable Ravio's welcome
            [55].disable(Flag::Course(244)),
            [58].disable(), // Disable Ravio's welcome
            [59].disable(), // Disable Ravio's welcome
        },

        // Hyrule Castle
        IndoorLight 12 {
            [23].disable(), // Zelda
            [26].disable(), // NPC Soldier
            [28].disable(), // NPC Soldier
            [29].disable(), // NPC Soldier
            [36].disable(), // Impa
            [37].disable(), // NPC Soldier
            [38].disable(), // NPC Soldier
            [39].disable(), // NPC Soldier
            [46].disable(), // NPC Soldier
            [47].disable(), // NPC Soldier
            [53].clear_enable_flag(), // Blue Soldier
            [54].clear_enable_flag(), // Arrow Soldier
            [56].clear_enable_flag(), // Arrow Soldier
            [57].clear_enable_flag(), // Shooter Spear
            [58].clear_enable_flag(), // Red Spear Soldier
            [60].clear_enable_flag(), // Green Spear Soldier
            [61].clear_enable_flag(), // Green Soldier
            [63].clear_enable_flag(), // Dagger Soldier
            [77].clear_enable_flag(), // Red Spear Soldier
            [78].clear_enable_flag(), // Green Soldier
            [79].clear_enable_flag(), // Blue Soldier
            [80].clear_enable_flag(), // Dagger Soldier
            [81].clear_enable_flag(), // Green Spear Soldier
            [82].clear_enable_flag(), // Red Spear Soldier
            [94].disable(), // Scholar
            [103].clear_enable_flag(), // Hyrule Paint Soldier
            [104].clear_enable_flag(), // Hyrule Paint Soldier
            [105].clear_enable_flag(), // Hyrule Paint Soldier
            [106].clear_enable_flag(), // Hyrule Paint Soldier
            [107].clear_enable_flag(), // Hyrule Paint Soldier
            [108].clear_enable_flag(), // Hyrule Paint Soldier
            [109].clear_enable_flag(), // Hyrule Paint Soldier
            [110].clear_enable_flag(), // Hyrule Paint Soldier
            [131].disable(), // NPC Soldier ACT 3
            [132].disable(), // NPC Soldier ACT 3
            [133].disable(), // NPC Soldier
            [134].disable(), // NPC Soldier
            [135].disable(), // NPC Soldier
            [136].disable(), // NPC Soldier
            [137].clear_enable_flag(), // Hyrule Paint Soldier
            [138].clear_enable_flag(), // Hyrule Paint Soldier
            [139].clear_enable_flag(), // Hyrule Paint Soldier
            [140].clear_enable_flag(), // Hyrule Paint Soldier
            [141].clear_enable_flag(), // Hyrule Paint Soldier
            [142].clear_enable_flag(), // Hyrule Paint Soldier
            [143].clear_enable_flag(), // Hyrule Paint Soldier
            [146].clear_enable_flag(), // Blue Soldier

            // Fix chest to not respawn
            [48].call {|obj: &mut Obj| {
                obj.arg_mut().5 = 3;
            }},
        },

        // Milk Bar
        IndoorLight 15 {
            [12].disable(), // Bouldering Guy stays on the mountain, so remove him from here
            [15].disable(), // Disable post Climber dialogue?
        },
        // Blacksmith's House
        IndoorLight 19 {
            [0x10].disable(), // Remove captain
        },

        // Donkey Cave
        CaveLight 1 {
            [84].disable(), // Remove a MojVolcanicRock to fix a vanilla softlock
        },
        // Zora's Domain
        CaveLight 7 {
            [116].enable(), // Thin Oren
            [119].enable(), // Zora Attendant
            [127].enable(), // Zora Attendant
            [131].clear_enable_flag(), // AreaSwitchCube, fix for not being able to turn in Smooth Gem
            [132].clear_enable_flag(), // Enable Zora Queen event always
            [134].enable(), // Thicc Oren
        },

        // Eastern Palace
        DungeonEast 3 {
            // Open door after defeating Yuga
            [0x5D].each [
                inactive(250),
                enable(),
            ],
        },

        // Skull Woods B2
        DungeonDokuro 2 {
            [363].disable(), // Remove door that can softlock player
        },

        // Thieves' Hideout
        DungeonHagure 1 {
            [541].enable(), // Thief Girl - Keep her from despawning after dungeon clear
            [1371].disable(), // Spear Boy AreaEventTalk
            [1372].disable(), // Spear Boy
        },

        // Swamp Palace 1F
        DungeonWater 1 {
            [326].disable(), // SE Room shutter door, removed for softlock prevention
            [385].disable(), // SW Room shutter door, removed for softlock prevention
        },

        // Swamp Palace B1
        DungeonWater 2 {
            [255].disable(), // Remove crystal switch, forces merge requirement to complete room to prevent softlock
        },

        // Lorule Castle
        DungeonGanon 1 {
            [1193].call {|obj: &mut Obj| { // Respawn Trial's Skip big rock upon leaving the room
                obj.arg_mut().4 = 0;
                obj.arg_mut().6 = 0;
            }},
        },
    );

    // Open Maiamai Cave only on non-glitch logics
    match settings.logic.mode {
        LogicMode::Normal | LogicMode::Hard | LogicMode::NoLogic => {
            apply!(patcher,
                // Lake Hylia
                FieldLight 35 {
                    [233].disable(), // Open Maiamai Cave
                    [235].disable(), // Remove the Sign
                },
            );
        }
        _ => {}
    }

    // Skip Trials Option
    if settings.logic.skip_trials {
        apply!(patcher,
            DungeonGanon 1 {
                [158].disable(),
            },
        );
    }

    // Change 'System' properties
    apply_system!(patcher,
        // Link's House
        IndoorLight 1 {
            // Default Spawn Point
            [47].call {|obj: &mut Obj| {
                obj.srt_mut().rotate.y = 0.0;
                obj.set_translate(0.0, 0.0, -6.5);
            }},
        },
    );

    Ok(())
}

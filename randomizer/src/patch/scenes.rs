use albw::{
    course,
    scene::{Flag, Obj},
};
use albw::course::Id::*;
use albw::scene::{Arg, Transform, Vec3};

use super::Patcher;
use crate::{Result, Settings};
use crate::logic_mode::LogicMode;

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


fn enable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.enable() }))
}

fn disable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.disable() }))
}


fn patch_eastern(patcher: &mut Patcher, settings: &Settings) {
    let eastern_flag = 536; // Gulley TODO

    // Eastern Ruins
    patcher.modify_objs(FieldLight, 20, &[

        // Remove Post-Eastern cutscene
        disable(83), // Sahasrahla
        // call(84, |obj| {
        //     obj.set_active_flag(Flag::Event(250));
        // }),
        disable(84), // Text box
        disable(85), // Loading Zone to FL18

        // Enable painted hearts always
        enable(214), // Paint Heart
        enable(215), // Paint Heart
    ]);

    // Eastern Ruins (system)
    // patcher.modify_system(FieldLight, 20, &[
    //     call(135, |obj| {
    //         obj.set_disable_flag(Flag::Event(310)); // Probably not needed but...
    //     }),
    // ]);


    // Eastern Palace 1F - Add Dungeon Reward
    patcher.add_obj(DungeonEast, 1, Obj {
        arg: Arg(0, 0, 0, 0, 4, 0, 1, 0, 0, 1, 180, 0, 0 /*60*/, 0.0),
        clp: 0,
        flg: (0, 4, 0, eastern_flag),
        id: 418,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(129),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: 0.0, y: 2.5, z: -5.75 },
        },
        typ: 4,
        unq: 301,
    });

    // Reroute Chamber of Sages warp
    patcher.modify_objs(CaveDark, 10, &[
        call(73, |obj| {
            obj.redirect(5, 0, 17); // EP drop off at HC
        })
    ]);


    // Eastern Palace 1F - Add sneaky loading zone over regular exit to go straight to dungeon reward
    // patcher.add_obj(DungeonEast, 1, Obj {
    //     arg: Arg(5, 6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0.0),
    //     clp: 0,
    //     flg: (4, 4, 250, 310), // Appears after Yuga 1 defeated, disappears after reward claimed
    //     id: 8, // Loading Zone
    //     lnk: vec![],
    //     nme: None,
    //     ril: vec![],
    //     ser: Some(129),
    //     srt: Transform {
    //         scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
    //         rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    //         translate: Vec3 { x: 0.0, y: 2.5, z: -2.0 }, // 1 unit higher than normal exit
    //     },
    //     typ: 6,
    //     unq: 301,
    // });

    // Outside Hyrule Castle - Enable reward cutscene after Yuga 1 defeated (flag 250)
    patcher.modify_objs(FieldLight, 18, &[

        // Sahasrahla
        disable(200),
        // call(200, move |obj| {
        //     obj.set_enable_flag(Flag::Event(250));
        // }),

        // Textbox trigger
        disable(208),
        // call(208, |obj| {
        //     obj.set_active_flag(Flag::Event(250));
        //     obj.set_enable_flag(Flag::Event(250));
        // }),

        disable(264), // lgt_NpcSoldier_Field1B_04_broke - idk what this is, but now it's nothing
    ]);

    // Add trigger at cutscene spawn to give dungeon reward flag
    /*patcher.add_obj(FieldLight, 18, Obj {
        arg: Arg(0, 0, 0, 0, 4, 0, eastern_flag, 0, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (4, 0, 250, 0), // Appear only after Yuga 1 defeated
        id: 14,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(164),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: 1.0, y: 0.0, z: 18.0 },
        },
        typ: 6,
        unq: 537,
    });*/
}

fn patch_gales(patcher: &mut Patcher, settings: &Settings) {
    let gales_flag = 537;

    patcher.modify_objs(DungeonWind, 3, &[

        // Change Pendant of Wisdom into chest
        call(459, move |obj| {
            obj.set_id(418);
            obj.arg.1 = 0;
            obj.arg.9 = 1;
            obj.arg.10 = 180;
            obj.arg.12 = 60;
            obj.set_inactive_flag(Flag::Event(gales_flag));
        })
    ]);

    // Reroute Chamber of Sages warp
    patcher.modify_objs(CaveDark, 10, &[
        call(73, |obj| {
            obj.redirect(0, 0, 34); // HoG Entrance
        })
    ]);
}

fn patch_hera(patcher: &mut Patcher, settings: &Settings) {
    patcher.modify_objs(DungeonHera, 1, &[

        // Change Pendant of Power into chest
        call(829, |obj| {
            obj.set_id(34);
            obj.arg.0 = 93;
            obj.set_inactive_flag(Flag::Event(597));
            obj.clear_disable_flag();
        })
    ]);
    patcher.add_obj(DungeonHera, 1,
                    Obj::warp(597, 20, 313,
                              Vec3 { x: 0.0, y: 101.5, z: -4.0 },
                              920,
                              3, 0, 2));
}

fn patch_dark(patcher: &mut Patcher, settings: &Settings) {
    let dark_flag = 597;

    patcher.modify_objs(DungeonDark, 1, &[
        disable(262),
    ]);

    // Add reward chest
    patcher.add_obj(DungeonDark, 1, Obj {
        arg: Arg(0, 0, 0, 0, 3, 4, 21, dark_flag, 0, 0, 0, 0, 0, 0.0),
        clp: 4,
        flg: (0, 0, 0, 0),
        id: 34,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(155),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: 0.0, y: 0.0, z: -47.5 },
        },
        typ: 1,
        unq: 300,
    });
    patcher.add_obj(DungeonDark, 1,
                    Obj::warp(dark_flag, 4, 156,
                              Vec3 { x: 0.0, y: 0.0, z: -46.0 },
                              301,
                              5, 1, 19));
}

// Swamp Palace
fn patch_swamp(patcher: &mut Patcher, settings: &Settings) {
    let swamp_flag = 342;

    //patcher.add_obj(DungeonWater, 3, Obj::dungeon_reward(616, 0, 14, Vec3 { x: 0.0, y: 2.5, z: -40.15 }, 6, 25));
    patcher.modify_objs(DungeonWater, 3, &[

        // Change Oren Portrait into chest
        call(13, move |obj| {
            obj.set_id(173);
            obj.redirect(0, 1, 32);
            obj.arg_mut().1 = 0; // 0 = Power, 1 = Wisdom, 2 = Courage
            obj.set_active_flag(Flag::Event(1));
            obj.set_inactive_flag(Flag::Event(swamp_flag));
            obj.arg_mut().12 = 0;
            //obj.set_typ(1);
            //obj.set_disable_flag(Flag::Event(swamp_flag));
            obj.clear_disable_flag();
        })
    ]);
    // patcher.add_obj(DungeonWater, 3,
    //                 Obj::warp(swamp_flag, 0, 14,
    //                           Vec3 { x: 0.0, y: 2.5, z: 1.5 + -40.15 }, // TODO try raising y to fix z-fighting
    //                           25,
    //                           0, 1, 32));
}

fn patch_skull(patcher: &mut Patcher, settings: &Settings) {
    let skull_flag = 617;

    // Skull Woods Overworld
    patcher.modify_objs(FieldDark, 1, &[
        call(273, move |obj| {
            obj.set_id(422);
            obj.set_disable_flag(Flag::Event(skull_flag));
            //obj.arg_mut().1 = 1;
            //obj.set_active_flag(Flag::Event(1));
            //obj.set_inactive_flag(Flag::Event(skull_flag));
            //obj.arg_mut().12 = 0;
            //obj.set_scale(0.0, 0.0, 0.0);
        })
    ]);

    /*patcher.add_obj(FieldDark, 1, Obj {
        arg: Arg(PendantCourage as i32, 0, 0, 0, 0, 4, 0, skull_flag, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 35,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(76),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: -6.0, y: 0.0, z: -16.5 },
        },
        typ: 1,
        unq: 533,
    });*/
    /*patcher.add_obj(FieldDark, 1,
                    Obj::warp(skull_flag, 0, 77,
                              Vec3 { x: -6.0, y: 0.0, z: 1.5 + -16.5 },
                              534,
                              10, 1, 0));*/
    //patcher.add_obj(FieldDark, 1, Obj::dungeon_reward(577, 0, 76, Vec3 { x: -6.0, y: 0.0, z: -16.5 }, 6, 533));
}

fn patch_thieves(patcher: &mut Patcher, settings: &Settings) {}

fn patch_ice(patcher: &mut Patcher, settings: &Settings) {}

fn patch_desert(patcher: &mut Patcher, settings: &Settings) {}

fn patch_turtle(patcher: &mut Patcher, settings: &Settings) {}


fn exhibition(patcher: &mut Patcher, settings: &Settings) {
    patcher.modify_objs(FieldLight, 25, &[
        disable(122), // Pouch
        disable(123), // Gulley
    ]);

    let shift = 2.0;

    // Gulley
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 4, 0, 1, 0, 0, 1, 0, 0, 60, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 418,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(75),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: (shift * -3.0) + -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 208,
    });

    // Oren
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 423,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(76),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: (shift * -2.0) + -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 209,
    });

    // Seres
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 420,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(77),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: -shift + -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 210,
    });

    // Osfala
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 419,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(78),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 211,
    });

    // Impa
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 421,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(79),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: shift + -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 212,
    });

    // Irene
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 417,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(80),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: (shift * 2.0) + -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 213,
    });

    // Rosso
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 422,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(81),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: (shift * 3.0) + -1.0, y: 0.0, z: -shift + -1.0 },
        },
        typ: 4,
        unq: 214,
    });

    // Power
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 0, 0, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 173,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(82),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: (shift * -1.5) + -1.0, y: 0.0, z: -1.0 },
        },
        typ: 4,
        unq: 215,
    });

    // Wisdom
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 1, 0, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 173,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(83),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: (shift * 1.5) + -1.0, y: 0.0, z: -1.0 },
        },
        typ: 4,
        unq: 216,
    });

    // Courage
    patcher.add_obj(FieldLight, 25, Obj {
        arg: Arg(0, 2, 0, 0, 4, 0, 1, 0, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (0, 0, 0, 0),
        id: 173,
        lnk: vec![],
        nme: None,
        ril: vec![],
        ser: Some(84),
        srt: Transform {
            scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            rotate: Vec3 { x: 330.0, y: 0.0, z: 0.0 },
            translate: Vec3 { x: -1.0, y: 0.5, z: -1.0 },
        },
        typ: 4,
        unq: 217,
    });
}


pub fn apply(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    patch_eastern(patcher, settings);
    patch_gales(patcher, settings);
    patch_hera(patcher, settings);

    patch_dark(patcher, settings);
    //patch_swamp(patcher, settings);
    patch_skull(patcher, settings);
    patch_thieves(patcher, settings);
    patch_ice(patcher, settings);
    patch_desert(patcher, settings);
    patch_turtle(patcher, settings);

    //exhibition(patcher, settings);

    // Ravio's Shop
    patcher.modify_objs(IndoorLight, 1, &[
        call(24, |obj| {
            obj.redirect(
                // 5, 0, 26, // No Redirect
                // 0, 5, 9, // Chamber of Sages
                // 0, 0, 19, // Eastern Ruins Cutscene
                // 5, 0, 17, // Pendant of Courage cutscene
                // 0, 0, 24,    // Haunted Grove
                // 12, 13, 0,   // Dark Palace Boss
                // 0, 65535, 65535,    // File Select Screen? lol
                // 6, 10, 2,    // Gales Boss
                0, 9, 0,     // Eastern Palace Entrance
                // 5, 0, 19     // Eastern Ruins WV
                // 0, 9, 0      // Eastern Palace Lobby
                // 20, 1, 0,     // Seres Portrait
                // 0, 4, 3      // Kak Well Lower
                // 10, 11, 0    // Tower of Hera Top
                // 0, 14, 2     // Swamp Palace 2F
            );
        }),
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
        // Outside Rosso's house
        FieldLight 2 {
            [11].clear_enable_flag(), // Small Rock (controller for other rocks, see the System section)
            [100].disable(None), // Keep Entry_KikoriMan3 from disappearing
            [101].disable(None),
            [128].disable(), // Remove "Not in right now." signboard
            [132].active(1), // Unlock Rosso's Front Door
            [135].disable(), // Disable LZ to IndoorLight4 Cutscene
            [136].enable(Flag::Event(250)), // Replace with IndoorLight10
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
            [133].disable(Flag::Event(828)), // Church Door rigged to open when Sanc left switch pulled

            [144].disable(), // Buzz Blob
            [145].enable(), // Buzz Blob
            [146].enable(), // Buzz Blob
            [147].enable(), // Buzz Blob
        },

        // Sanctuary Dungeon
        CaveLight 18 {
            [35].active(828), // Pull Switch
            [37].inactive(828), // Door
            [107].active(828), // TagCameraFocus
            [107].disable(Flag::Event(828)), // TagCameraFocus
        },

        // Sanctuary Church
        IndoorLight 11 {
            [14].clear_enable_flag(), // Church Door
            [14].disable(Flag::Event(828)), // Church Door
            [16].disable(), // Early game Priest
            [20].active(828),
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
        // Master Sword
        FieldLight 34 {
            [71].each [
                active(0),
                call {|obj: &mut Obj| {
                    let arg = obj.arg_mut();
                    arg.5 = 3;
                    arg.7 = 150;
                }},
                enable(),
                id(0x23), // replace with chest
            ],
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



        // Dark Maze
        FieldDark 20 {
            [235].disable(), // Hilda Text

            [63].disable(),  // AreaEventTalk
            [115].disable(), // AreaEventTalk
            [116].disable(), // AreaEventTalk
            [119].disable(), // AreaEventTalk
            [122].disable(), // AreaEventTalk
            [188].disable(), // AreaEventTalk
            [231].disable(), // AreaEventTalk

            [195].disable(), // NpcGuardMan
            [196].disable(), // NpcGuardMan
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
        // Zelda's Study
        IndoorLight 7 {
            [10].call {|obj: &mut Obj| {
                obj.arg_mut().3 = 0; // Prevent Long Portal Transition
            }},
            [26].disable(), // Disable Curtain
            [29].disable(), // Disable AreaDisableWallIn
        },
        // Rosso's house
        IndoorLight 10 {
            [7].each [
                id(35),
                inactive(282),
                enable(),
            ],
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

        // Thief Girl Cave
        CaveDark 15 {
            [10].disable(), // Entrance text
            [13].disable(), // It's a secret to everybody
        },

        // Eastern Palace
        DungeonEast 3 {
            // Open door after defeating Yuga
            [0x5D].each [
                inactive(250),
                enable(),
            ],
        },

        // Inside Hyrule Castle 2F
        DungeonCastle 1 {
            // Rewire Warp to use course flag 31 (Yuga defeated)
            [19].disable(Flag::Course(31)), // Armos Statue
            [35].call {|obj: &mut Obj| {
                obj.set_active_flag(Flag::Course(31));
                obj.set_enable_flag(Flag::Course(31));
            }},
        },

        // Inside Hyrule Castle 7F
        DungeonCastle 5 {
            // Rewire Warp to use course flag 31 (Yuga defeated)
            [18].call {|obj: &mut Obj| {
                obj.set_active_flag(Flag::Course(31));
                obj.set_enable_flag(Flag::Course(31));
            }},
        },

        // Inside Hyrule Castle 8F
        DungeonCastle 6 {
            // Redirect loading zone to Lorule Blacksmith
            [13].call {|obj: &mut Obj| {
                obj.redirect(2, 3, 3);
                obj.arg_mut().1 = 1; // Hole entrance
            }},

            // Rewire entrance door to stay open with course flag 31 (Yuga defeated)
            [20].disable(Flag::Course(31)),

            // Disable door normally shut after global flag 510 activated
            [28].disable(),
        },

        // Inside Hyrule Castle 4F
        DungeonCastle 7 {
            [19].enable(Flag::Event(415)),
            [20].enable(Flag::Event(415)),
            [21].enable(Flag::Event(415)),
            [22].enable(Flag::Event(415)),
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

        // Outside Rosso's house
        FieldLight 2 {
            [11].clear_enable_flag(), // Small Rock (controller for the other rocks)
            [12].clear_enable_flag(), // Small Rock
            [14].clear_enable_flag(), // Small Rock
            [15].clear_enable_flag(), // Small Rock
            [16].clear_enable_flag(), // Small Rock
            [18].clear_enable_flag(), // Small Rock
            [19].clear_enable_flag(), // Small Rock
            [20].clear_enable_flag(), // Small Rock
            [21].clear_enable_flag(), // Small Rock
            [93].clear_enable_flag(), // Small Rock
            [94].clear_enable_flag(), // Small Rock
            [102].clear_enable_flag(), // Small Rock
            [103].clear_enable_flag(), // Small Rock
            [104].clear_enable_flag(), // Small Rock
            [105].clear_enable_flag(), // Small Rock
            [106].clear_enable_flag(), // Small Rock
            [107].clear_enable_flag(), // Small Rock
            [108].clear_enable_flag(), // Small Rock
            [109].clear_enable_flag(), // Small Rock
            [110].clear_enable_flag(), // Small Rock
            [111].clear_enable_flag(), // Small Rock
            [112].clear_enable_flag(), // Small Rock
            [118].clear_enable_flag(), // Small Rock
            [119].clear_enable_flag(), // Small Rock
            [120].clear_enable_flag(), // Small Rock
            [121].clear_enable_flag(), // Small Rock
            [122].clear_enable_flag(), // Small Rock
            [123].clear_enable_flag(), // Small Rock
            [124].clear_enable_flag(), // Small Rock
            [125].clear_enable_flag(), // Small Rock
            [126].clear_enable_flag(), // Small Rock
        },

        // Dark Maze
        FieldDark 20 {
            [215].set_translate(1.0, 3.0, 23.0),    // Raise cage spawn point. Original: (  1.0, 0.5,  23.0)
            [217].set_translate(-17.0, 3.0, -17.0), // Raise cage spawn point. Original: (-17.0, 0.5, -17.0)
        },

        // Link's House
        IndoorLight 1 {
            // Default Spawn Point
            [47].call {|obj: &mut Obj| {
                obj.srt_mut().rotate.y = 0.0;
                obj.set_translate(0.0, 0.0, -6.5);
            }},
        },

        // Lorule Blacksmith's House
        IndoorDark 4 {
            // Raise spawn point for Link to fall into the house
            [21].call {|obj: &mut Obj| {
                obj.srt_mut().rotate.y = 90.0;
                obj.set_translate(-0.5, 7.5, -6.0);
            }},
        },
    );

    Ok(())
}

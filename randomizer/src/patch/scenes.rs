use game::{
    Course::{self, *},
    Item::*,
};
use log::info;
use macros::fail;
use modinfo::settings::{hyrule_castle::HyruleCastleSetting, logic::LogicMode, Settings};
use rom::flag::Flag;
use rom::scene::{Arg, Dest, Obj, Transform, Vec3};

use super::Patcher;
use crate::{patch::util::*, Result};

macro_rules! apply {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(::game::Course::$course, $stage - 1)?.stage_mut().get_mut();
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
            let stage = $patcher.scene(::game::Course::$course, $stage - 1)?.stage_mut().get_mut();
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

/// Patch Scene BYAML Files
pub fn patch_byaml_files(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    info!("Patching BYAML Files...");

    do_dev_stuff(patcher, settings);
    patch_big_problem_chests(patcher, settings);
    patch_blacksmith_hyrule(patcher);
    patch_castles(patcher, settings);
    patch_chamber_of_sages(patcher);
    patch_dark_maze(patcher);
    patch_kus_domain(patcher);
    patch_letter_in_a_bottle(patcher);
    patch_master_sword(patcher);
    patch_softlock_prevention(patcher, settings);
    patch_thief_girl_cave(patcher);
    patch_treasure_dungeons(patcher, settings);
    patch_zora(patcher);
    patch_swamp_palace(patcher);
    patch_hint_ghosts_overworld(patcher);
    patch_hint_ghosts_dungeons(patcher);
    patch_ghost_into_hildas_study(patcher);

    patch_nice_mode(patcher, settings);
    patch_big_bomb_flower_skip(patcher, settings);
    patch_no_progression_enemies(patcher, settings);
    patch_open_lost_woods(patcher);
    patch_magic_shop(patcher);

    patcher.modify_objs(FieldLight, 18, &[disable(529)]);

    // TODO convert to new approach
    apply!(patcher,

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
            [133].disable(Flag::Event(523)), // Church Door rigged to open when Sanc left switch pulled

            [144].disable(), // Buzz Blob
            [145].enable(), // Buzz Blob
            [146].enable(), // Buzz Blob
            [147].enable(), // Buzz Blob
        },

        // Sanctuary Dungeon
        CaveLight 18 {
            // 523 is a repurposed flag to control this
            [35].active(523), // Pull Switch
            [37].inactive(523), // Door
            [107].active(523), // TagCameraFocus
            [107].disable(Flag::Event(523)), // TagCameraFocus
        },

        // Sanctuary Church
        IndoorLight 11 {
            [14].clear_enable_flag(), // Church Door
            [14].disable(Flag::Event(523)), // Church Door
            [16].disable(), // Early game Priest
            [20].active(523),
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

        // Hyrule Hotfoot Area
        FieldLight 36 {
            [43].disable(), // Disable Letter in a Bottle text
        },

        // Sacred Realm
        FieldLight 43 {
            //[23].disable(), // seichi - "Sanctuary" - Initial text
            //[32].disable(), // Remove Clouds
            [26].disable(), // zelda_talk - Chat after standing up
            [33].disable(), // zelda_talk_b - Wait for Zelda
            [34].disable(), // zelda_talk_c - Last chat before triangles
        },

        // Link's House
        IndoorLight 1 {

            // Bow Slot - Keep at sale price of 10 Rupees always
            [17].call {|obj: &mut Obj| {
                obj.arg.3 = 10;
            }},

            // Tornado Rod Slot - Set to 20 Rupee sale price
            [15].call {|obj: &mut Obj| {
                obj.arg.3 = 20;
            }},

            // Hammer Slot - Set to 20 Rupee sale price
            [19].call {|obj: &mut Obj| {
                obj.arg.3 = 20;
            }},

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

            [31].disable(), // Disable first time goodbye text
            [46].disable(), // Disable Ravio's bye-bye
            [54].disable(), // Disable Ravio's welcome
            [55].disable(Flag::Course(244)),
            [58].disable(), // Disable Ravio's welcome
            [59].disable(), // Disable Ravio's welcome
        },

        // Hyrule Castle
        IndoorLight 12 {
            //[24].disable(), // Entry Impa
            [26].disable(), // NPC Soldier
            [28].disable(), // NPC Soldier
            [29].disable(), // NPC Soldier
            [37].disable(), // NPC Soldier
            [38].disable(), // NPC Soldier
            [39].disable(), // NPC Soldier

            // [40].disable(), // Textbox trigger FieldLight_1B_Impa_ACT03_01 (left)
            // [41].disable(), // Textbox trigger FieldLight_1B_Impa_ACT03_02 (right)
            // [43].disable(), // Textbox trigger FieldLight_1B_Impa_ACT03_00 (main exit)
            [45].disable(), // Disable ZeldaFirstTimeEvent_01 (Get Charm)
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
            //[92].disable(), // NPC Soldier (lower right)
            //[93].disable(), // NPC Soldier (lower left)
            [94].disable(), // Scholar
            //[99].disable(), // Text box trigger FieldLight_1B_Impa_ACT_03_05
            [100].disable(), // NpcZeldaDemo
            //[101].disable(), // TIMER
            [103].clear_enable_flag(), // Hyrule Paint Soldier
            [104].clear_enable_flag(), // Hyrule Paint Soldier
            [105].clear_enable_flag(), // Hyrule Paint Soldier
            [106].clear_enable_flag(), // Hyrule Paint Soldier
            [107].clear_enable_flag(), // Hyrule Paint Soldier
            [108].clear_enable_flag(), // Hyrule Paint Soldier
            [109].clear_enable_flag(), // Hyrule Paint Soldier
            [110].clear_enable_flag(), // Hyrule Paint Soldier

            [125].disable(), // NPC Solider (upper right)
            [126].disable(), // NPC Solider (upper left)
            [127].disable(), // FieldLight_Right_Soldier_Area
            [128].disable(), // FieldLight_Left_Soldier_Area


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
            // [145].disable(), // Impa stops makes you wait and lets you go see Zelda
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
            [1371].disable(), // Spear Boy AreaEventTalk
            [1372].disable(), // Spear Boy
            [1345].disable(), // Thief Girl Text - 1st Zazak Fight
        },

        // Swamp Palace 1F
        // DungeonWater 1 {
        //     [326].disable(), // SE Room shutter door, removed for softlock prevention
        //     [385].disable(), // SW Room shutter door, removed for softlock prevention
        // },

        // Swamp Palace B1
        // DungeonWater 2 {
        //     [255].disable(), // Remove crystal switch, forces merge requirement to complete room to prevent softlock
        // },
    );

    // Open Maiamai Cave only on non-glitch logics
    match settings.logic.logic_mode {
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

fn patch_open_lost_woods(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldLight,
        1,
        &[
            disable(34), // Keep Lost Woods Maze from disappearing after getting Pedestal
        ],
    );

    patcher.modify_objs(
        FieldLight,
        38,
        &[
            // Allow entry to maze without All Pendants Flag (375) set
            redirect(259, Dest::new(FieldLight, 38, 5)),
            // 1st Fork - Enable all Loading Zones
            clear_active_args(137), // North
            clear_active_args(138), // West
            clear_active_args(139), // East
            // 2nd Fork - Enable all Loading Zones
            clear_active_args(168), // North
            clear_active_args(91),  // West
            clear_active_args(89),  // South
            // 3rd Fork - Make all Loading Zones correct
            redirect(110, Dest::new(FieldLight, 38, 6)), // West
            redirect(111, Dest::new(FieldLight, 38, 6)), // East
            redirect(112, Dest::new(FieldLight, 38, 6)), // North
            // 1st Poes
            disable(132),
            disable(133),
            // 2nd Poes
            disable(170),
            disable(185),
            // 3rd Poes
            disable(175),
            disable(186),
            // Redirect normal loading zone to Pedestal to kick player out
            call(127, |obj| {
                obj.redirect(Dest::new(FieldLight, 38, 0));
                obj.set_translate(-80.25, -1.5, -200.5); // move back slightly
            }),
            // Repurpose Flag 375 loading zone to appear at end of maze, allowing Pedestal access
            call(134, |obj| {
                obj.redirect(Dest::new(FieldLight, 34, 0));
                obj.set_translate(-80.25, -1.5, -200.0); // take position of OG loading zone
                obj.clp = 5;
            }),
        ],
    );
}

/// Witch's House
fn patch_magic_shop(patcher: &mut Patcher) {
    patcher.modify_objs(
        IndoorLight,
        2,
        &[
            disable(19), // Entry_FieldLight16_Obaba_MissingMaple_00
            disable(20), // MagicShopKeeper_StoneBeauty
            disable(21), // Entry_FieldLight16_Obaba_HelpMaple
        ],
    );
}

// Hyrule Blacksmith
fn patch_blacksmith_hyrule(patcher: &mut Patcher) {
    patcher.modify_objs(
        IndoorLight,
        19,
        &[
            // Make PackageSword a Chest
            call(12, |obj| {
                obj.clear_active_args();
                obj.set_inactive_flag(Flag::Event(26));
                //obj.clear_disable_flag();
                obj.set_typ(1);
                obj.srt.translate.x = -1.957;
                obj.srt.translate.y = 0.6;
                obj.srt.scale = match obj.id {
                    35 => Vec3 { x: 1.00000, y: 2.00000, z: 2.22222 },
                    34 => Vec3 { x: 0.52632, y: 2.00000, z: 1.66667 },
                    _ => {
                        fail!("PackageSword wasn't a chest")
                    }
                }
            }),
            disable(19), // Map attention
        ],
    );
}

// Chamber of Sages
fn patch_chamber_of_sages(patcher: &mut Patcher) {
    patcher.modify_objs(
        CaveDark,
        10,
        &[
            set_46_args(74, Flag::Event(0)), // Staircase
        ],
    );
}

// Ku's Domain
fn patch_kus_domain(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldDark,
        7,
        &[
            call(55, |obj| {
                obj.set_typ(4); // changed to chest automatically, set typ here
            }),
            disable(66), // rupee throw camera
        ],
    );
}

// Treasure Dungeons
fn patch_treasure_dungeons(patcher: &mut Patcher, settings: &Settings) {
    // Remove Treasure Dungeon mini-cutscenes only when CSMC is off (since they show the chests)
    if !settings.options.chest_size_matches_contents {
        patcher.modify_objs(AttractionLight, 1, &[disable(15)]);
        patcher.modify_objs(AttractionLight, 2, &[disable(54)]);
        patcher.modify_objs(AttractionLight, 3, &[disable(47)]);
        patcher.modify_objs(AttractionLight, 4, &[disable(118)]);
        patcher.modify_objs(AttractionLight, 5, &[disable(26)]);
    }
}

// Zora
fn patch_zora(patcher: &mut Patcher) {
    // Lake Hylia
    patcher.modify_objs(
        FieldLight,
        35,
        &[
            enable(151), // Zora outside House of Gales
        ],
    );
}

// Swamp Palace
fn patch_swamp_palace(patcher: &mut Patcher) {
    patcher.modify_objs(
        DungeonWater,
        2,
        &[call(633, |obj| {
            obj.clp = 3; // Fix the impossible Rupee
        })],
    );
}

// Enable All Overworld Hint Ghosts
fn patch_hint_ghosts_overworld(patcher: &mut Patcher) {
    patcher.modify_objs(FieldLight, 14, &[enable(126)]); // Witch's House
    patcher.modify_objs(FieldLight, 16, &[enable(407)]); // Shady Guy (Kakariko)
    patcher.modify_objs(FieldLight, 17, &[enable(96)]); // Behind Blacksmith
    patcher.modify_objs(FieldDark, 35, &[enable(205)]); // Bullied Turtle
}

// Hide All Dungeon Hint Ghosts
fn patch_hint_ghosts_dungeons(patcher: &mut Patcher) {
    // Eastern
    patcher.modify_objs(
        DungeonEast,
        1,
        &[
            disable(251),
            disable(252),
            disable(253),
            disable(254),
            disable(255),
            disable(256),
            disable(257),
        ],
    );
    patcher.modify_objs(
        DungeonEast,
        2,
        &[
            disable(235),
            disable(236),
            disable(237),
            disable(238),
            disable(239),
            disable(240),
            disable(241),
            disable(243),
        ],
    );
    patcher.modify_objs(DungeonEast, 3, &[disable(92)]);

    // Gales
    patcher.modify_objs(
        DungeonWind,
        1,
        &[disable(390), disable(391), disable(392), disable(393), disable(394)],
    );
    patcher.modify_objs(DungeonWind, 2, &[disable(327), disable(328), disable(329), disable(474)]);
    patcher.modify_objs(DungeonWind, 3, &[disable(509), disable(510), disable(511), disable(512)]);

    // Hera
    patcher.modify_objs(
        DungeonHera,
        1,
        &[
            disable(862),
            disable(863),
            disable(864),
            disable(865),
            disable(866),
            disable(867),
            disable(868),
            disable(869),
            disable(870),
            disable(871),
        ],
    );

    // Hyrule Castle
    patcher.modify_objs(DungeonCastle, 2, &[disable(64)]);

    // Dark
    patcher.modify_objs(
        DungeonDark,
        1,
        &[
            disable(208),
            disable(209),
            disable(210),
            disable(211),
            disable(212),
            disable(213),
            disable(214),
            disable(216),
            disable(217),
            disable(218),
        ],
    );
    patcher.modify_objs(
        DungeonDark,
        2,
        &[
            disable(170),
            disable(171),
            disable(172),
            disable(173),
            disable(174),
            disable(175),
            disable(176),
            disable(177),
            disable(204),
        ],
    );
    patcher.modify_objs(
        DungeonDark,
        3,
        &[
            disable(225),
            disable(226),
            disable(227),
            disable(228),
            disable(229),
            disable(230),
            disable(231),
        ],
    );

    // Swamp
    patcher.modify_objs(DungeonWater, 1, &[disable(446), disable(447), disable(448), disable(449)]);
    patcher.modify_objs(
        DungeonWater,
        2,
        &[disable(565), disable(566), disable(567), disable(589), disable(660)],
    );

    // Skull
    patcher.modify_objs(
        DungeonDokuro,
        1,
        &[disable(765), disable(766), disable(767), disable(768), disable(776)],
    );
    patcher.modify_objs(DungeonDokuro, 2, &[disable(480), disable(481)]);

    // Thieves'
    patcher.modify_objs(
        DungeonHagure,
        1,
        &[disable(1364), disable(1365), disable(1366), disable(1367), disable(1368), disable(1416)],
    );

    // Turtle
    patcher.modify_objs(DungeonKame, 1, &[disable(247), disable(248), disable(249), disable(250)]);
    patcher.modify_objs(
        DungeonKame,
        2,
        &[disable(234), disable(235), disable(236), disable(237), disable(263)],
    );

    // Desert
    patcher.modify_objs(
        DungeonSand,
        1,
        &[disable(598), disable(599), disable(600), disable(601), disable(602), disable(616)],
    );
    patcher.modify_objs(DungeonSand, 2, &[disable(668), disable(669), disable(670), disable(671)]);
    patcher.modify_objs(DungeonSand, 3, &[disable(293), disable(294)]);

    // Ice
    patcher.modify_objs(
        DungeonIce,
        1,
        &[
            disable(900),
            disable(901),
            disable(902),
            disable(903),
            disable(904),
            disable(906),
            disable(907),
            disable(908),
            disable(909),
            disable(910),
            disable(911),
            disable(1145),
        ],
    );

    // Lorule Castle
    patcher.modify_objs(
        DungeonGanon,
        1,
        &[
            disable(1230),
            disable(1232),
            disable(1233),
            disable(1234),
            disable(1235),
            disable(1236),
            disable(1237),
            disable(1238),
            disable(1239),
            disable(1241),
            disable(1242),
            disable(1371),
            disable(1602),
            disable(1607),
        ],
    );
}

fn patch_ghost_into_hildas_study(patcher: &mut Patcher) {
    patcher.add_obj(
        IndoorDark,
        5,
        Obj {
            arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0),
            clp: 1,
            flg: (0, 0, 0, 0),
            id: 235,
            lnk: vec![],
            nme: Some("HintGhostDark/HintGhost_FieldDark_2C_014".to_owned()),
            ril: vec![],
            ser: Some(14),
            srt: Transform {
                scale: Vec3::UNIT,
                rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                translate: Vec3 { x: 67.0, y: 0.0, z: -14.5 },
            },
            typ: 3,
            unq: 48,
        },
    );
}

fn patch_castles(patcher: &mut Patcher, settings: &Settings) {
    let green_pendant_flag = prize_flag(PendantCourage);
    let yuga_defeated = Flag::Event(420); // Set after Yuga 2 defeated
    let hc_31 = Flag::Course(31); // Also set after Yuga 2 defeated
    let hacky_flag = Flag::Event(421); // Repurposed for Curtain/Trial's Door removal
    let can_fight_yuganon_flag = Flag::Event(670); // TODO separate from LC requirement

    // Hyrule Castle (exterior)
    patcher.modify_objs(
        FieldLight,
        18,
        &[
            // Barrier
            set_46_args(165, Flag::Event(1)), // Enable Barrier from game start
            disable(505),                     // Barrier "would you like to save?" text
        ],
    );
    match settings.logic.hyrule_castle_setting {
        HyruleCastleSetting::EarlyLoruleCastle => {
            patcher.modify_objs(
                FieldLight,
                18,
                &[
                    // Pendant of Courage opens the Hyrule Castle Dungeon
                    set_enable_flag(155, green_pendant_flag), // HC dungeon loading zone
                    set_disable_flag(393, green_pendant_flag), // HC dungeon door
                ],
            );
        }
        HyruleCastleSetting::Closed => {
            patcher.modify_objs(
                FieldLight,
                18,
                &[
                    // Forcibly close entrance to Hyrule Castle Dungeon
                    disable(155), // HC dungeon loading zone
                    enable(393),  // HC dungeon door
                ],
            );
        }
    }

    // 2F (there is no 1F of the dungeon)
    patcher.modify_objs(
        DungeonCastle,
        1,
        &[
            set_disable_flag(19, hc_31), // Armos Statue
            call(35, move |obj| {
                // Warp
                obj.set_active_flag(hc_31);
                obj.set_enable_flag(hc_31);
            }),
        ],
    );

    // 4F
    patcher.modify_objs(
        DungeonCastle,
        7,
        &[
            enable(19), // Green Soldier
            enable(20), // Green Soldier
            enable(21), // Red Spear Soldier
            enable(22), // Red Spear Soldier
        ],
    );

    // 7F
    patcher.modify_objs(
        DungeonCastle,
        5,
        &[call(18, move |obj| {
            // warp
            obj.set_active_flag(hc_31);
            obj.set_enable_flag(hc_31);
        })],
    );

    // 8F
    patcher.modify_objs(
        DungeonCastle,
        6,
        &[
            set_disable_flag(20, hc_31), // Rewire entrance door to stay open
            disable(28),                 // no revisits door
        ],
    );

    // Zelda's Study
    patcher.modify_objs(
        IndoorLight,
        7,
        &[
            // No backtracking door
            call(27, move |obj| {
                obj.clear_enable_flag();
                obj.set_disable_flag(yuga_defeated);
            }),
            set_disable_flag(26, hacky_flag), // Curtain
            set_disable_flag(29, hacky_flag), // AreaDisableWallIn
            // Portal
            call(10, move |obj| {
                obj.arg.3 = 0; // Prevent Long Portal Transition
                obj.set_active_flag(hacky_flag); // Open Trials Door
            }),
            // Fairies
            set_enable_flag(18, can_fight_yuganon_flag),
            set_enable_flag(19, can_fight_yuganon_flag),
            set_enable_flag(20, can_fight_yuganon_flag),
            set_enable_flag(21, can_fight_yuganon_flag),
            // Hearts (Painted)
            set_disable_flag(36, can_fight_yuganon_flag),
            set_disable_flag(41, can_fight_yuganon_flag),
            set_disable_flag(42, can_fight_yuganon_flag),
            set_disable_flag(43, can_fight_yuganon_flag),
        ],
    );

    // Hilda's Study
    patcher.modify_objs(
        IndoorDark,
        5,
        &[
            disable(4),                                  // Trial's Door
            disable(12),                                 // Yuga revives Ganon cutscene
            enable(34),                                  // Throne Room Loading Zone
            set_enable_flag(23, can_fight_yuganon_flag), // Skull (top right, controller obj)
            set_46_args(14, hacky_flag),                 // Portal - Set Flag to remove curtain
        ],
    );

    // Hilda's Study (system)
    patcher.modify_system(
        IndoorDark,
        5,
        &[
            set_enable_flag(23, can_fight_yuganon_flag), // Skull (top right, controller system obj)
            set_enable_flag(24, can_fight_yuganon_flag), // Skull (middle right)
            set_enable_flag(25, can_fight_yuganon_flag), // Skull (bottom right)
            set_enable_flag(41, can_fight_yuganon_flag), // Skull (bottom left)
            set_enable_flag(46, can_fight_yuganon_flag), // Skull (middle left)
            set_enable_flag(47, can_fight_yuganon_flag), // Skull (top left)
        ],
    );

    // Lorule Castle
    patcher.modify_objs(
        DungeonGanon,
        1,
        &[
            clear_enable_flag(1193), // Respawn Trial's Skip big rock upon leaving the room
            set_disable_flag(158, hacky_flag), // Trial's Door
            disable(265),            // Trial's Door camera pan
        ],
    );

    // Throne Room
    patcher.modify_objs(
        DungeonBoss,
        1,
        &[
            // fight start trigger
            call(10, move |obj| {
                obj.set_enable_flag(can_fight_yuganon_flag);
                obj.set_active_flag(can_fight_yuganon_flag);
            }),
            clear_enable_flag(27), // Hilda
            clear_enable_flag(41), // camera offset
            clear_enable_flag(43), // NpcAttention1
            clear_enable_flag(48), // ObjPictureZelda
        ],
    );
}

// Change Letter in a Bottle to a Heart Piece object
fn patch_letter_in_a_bottle(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldLight,
        36,
        &[call(38, |obj| {
            obj.clear_disable_flag();
            obj.set_inactive_flag(Flag::Event(916));
            obj.set_id(99);
            obj.set_typ(1);
        })],
    );
}

fn patch_master_sword(patcher: &mut Patcher) {
    // Master Sword Pedestal
    patcher.modify_objs(
        FieldLight,
        34,
        &[call(71, |obj| {
            obj.clear_active_args();
            obj.set_inactive_flag(Flag::Course(150));
            obj.enable();
        })],
    );
}

fn patch_dark_maze(patcher: &mut Patcher) {
    // Remove dialog
    patcher.modify_objs(
        FieldDark,
        20,
        &[
            disable(63),  // AreaEventTalk
            disable(115), // AreaEventTalk
            disable(116), // AreaEventTalk
            disable(119), // AreaEventTalk
            disable(122), // AreaEventTalk
            disable(188), // AreaEventTalk
            disable(195), // NpcGuardMan
            disable(196), // NpcGuardMan
            disable(231), // AreaEventTalk
            disable(235), // Hilda Text
        ],
    );
}

fn patch_thief_girl_cave(patcher: &mut Patcher) {
    patcher.modify_objs(
        CaveDark,
        15,
        &[
            // Thief Girl w/ Mask
            call(8, move |obj| {
                //obj.set_enable_flag(prize_flag);
                obj.srt.rotate.y = 0.0;
            }),
            //set_enable_flag(9, prize_flag), // Chest
            disable(10), // Entrance text
            disable(11), // AreaSwitchCube
            disable(13), // It's a secret to everybody
        ],
    );
}

/// Modify the hitboxes of select big chests that could negatively affect gameplay
fn patch_big_problem_chests(patcher: &mut Patcher, settings: &Settings) {
    if !settings.options.chest_size_matches_contents {
        return;
    }

    const BIG_PROBLEM_CHESTS: [(Course, u16, u16); 21] = [
        (FieldLight, 3, 303),  // Death Mountain West Ledge
        (FieldLight, 34, 71),  // Master Sword Pedestal
        (FieldLight, 35, 155), // Lake Hylia Ledge
        (FieldLight, 33, 320), // Southern Ruins Ledge
        // (FieldLight, 1, 133),  // Lost Woods Big Rock
        (AttractionLight, 2, 33), // Southern Ruins Treasure Dungeon
        (DungeonEast, 2, 52),     // Eastern 2F 4 Switches
        (DungeonDark, 2, 127),    // Dark 1F Fall from 2F
        (DungeonDark, 3, 269),    // Dark 2F East
        (DungeonWater, 1, 170),   // Swamp 1F West Room
        (DungeonWater, 1, 299),   // Swamp 1F East Room
        (DungeonWater, 1, 373),   // Swamp 1F SW Room
        (DungeonWater, 2, 620),   // Swamp B1 Raft Room (Left)
        (DungeonWater, 2, 621),   // Swamp B1 Raft Room (Right)
        (DungeonDokuro, 2, 105),  // Skull B2 Moving Platform Room
        (FieldDark, 1, 515),      // Skull Outdoor Chest
        (DungeonKame, 1, 173),    // Turtle 1F SE Room
        (DungeonKame, 2, 183),    // Turtle B1 East Platform
        (DungeonSand, 1, 78),     // Desert 1F Entrance
        (DungeonSand, 1, 565),    // Desert 1F South Sand Room
        (DungeonSand, 2, 462),    // Desert 2F Below Big Chest
        // (DungeonIce, 1, 1122), // Ice Ruins B4 SW Fall
        (DungeonGanon, 1, 882), // Lorule Castle Ball Trial #2
    ];

    // Change collision scaling to effectively match the small chests
    for (stage, stage_index, unq) in BIG_PROBLEM_CHESTS {
        patcher.modify_objs(
            stage,
            stage_index,
            &[call(unq, |obj| {
                if obj.id == 34 {
                    obj.srt.scale.x = 0.52632; // 0.52632 * 1.9 (actor profile) ~= 1.0
                    obj.srt.scale.z = 0.75; // 0.75 * 1.2 (actor profile) = 0.9
                }
            })],
        );
    }
}

fn patch_softlock_prevention(patcher: &mut Patcher, _settings: &Settings) {
    // Gales 1F - Add trigger to drop wall if player entered miniboss without hitting switch
    patcher.add_obj(
        DungeonWind,
        1,
        Obj::trigger_cube(Flag::Course(60), 2, 146, 454, Vec3 { x: 16.5, y: 2.5, z: -19.0 }),
    );

    // Dark Maze w/o Merge
    // match settings.logic.active_weather_vanes {
    //     ActiveWeatherVanes::Lorule | ActiveWeatherVanes::All => {
    //         // 1st Prison Cell softlock prevention
    //         patcher.add_obj(
    //             FieldDark,
    //             20,
    //             Obj::warp_tile(
    //                 Flag::Event(1),
    //                 0,
    //                 66,
    //                 245,
    //                 0,
    //                 1,
    //                 19,
    //                 Vec3 { x: 1.0 + 2.0, y: 0.5, z: 23.0 },
    //             ),
    //         );
    //
    //         // 2nd Prison Cell softlock prevention
    //         patcher.add_obj(
    //             FieldDark,
    //             20,
    //             Obj::warp_tile(
    //                 Flag::Event(1),
    //                 0,
    //                 67,
    //                 246,
    //                 0,
    //                 1,
    //                 19,
    //                 Vec3 { x: -17.0 + 2.5, y: 0.5, z: -17.0 },
    //             ),
    //         );
    //     }
    //     _ => {}
    // };

    // Swamp Palace SE Room w/o Merge
    // Swamp Palace SW Room w/o Merge
    // Skull Woods B2 Boss Hallway w/o Fire
}

/// Nice Mode
fn patch_nice_mode(patcher: &mut Patcher, settings: &Settings) {
    if !settings.logic.nice_mode {
        return;
    }

    // Make Maiamai Cave Reject Player - TODO Undo this when Maiamai Rewards are shuffled
    patcher.modify_objs(FieldLight, 35, &[redirect(140, Dest::new(FieldLight, 35, 8))]);
}

/// Big Bomb Flower Skip
fn patch_big_bomb_flower_skip(patcher: &mut Patcher, settings: &Settings) {
    if !settings.logic.skip_big_bomb_flower {
        return;
    }

    // Big Bomb Flower Field
    patcher.modify_objs(
        FieldDark,
        24,
        &[
            disable(86), // Unlock Big Bomb Flower
            disable(93), // Great Rupee Fairy
        ],
    );

    // South of Octoball Derby
    patcher.modify_objs(
        FieldDark,
        32,
        &[
            disable(89), // Boulder of Destiny
        ],
    );

    // Lorule Southern Ruins
    patcher.modify_objs(
        FieldDark,
        33,
        &[
            /* Swamp Palace gets drained by setting Flag 541 */
            disable(201), // Swamp Cave
        ],
    );
}

/// No Progression Enemies
fn patch_no_progression_enemies(patcher: &mut Patcher, settings: &Settings) {
    if !settings.logic.no_progression_enemies {
        return;
    }

    // Swamp
    patcher.modify_objs(
        DungeonWater,
        1,
        &[
            disable(451), // Bawb (west)
            disable(452), // Bawb (east)
        ],
    );

    // Skull
    patcher.modify_objs(
        DungeonDokuro,
        1,
        &[
            disable(271), // Wall Master (North B1)
        ],
    );

    // Thieves'
    patcher.modify_objs(
        DungeonHagure,
        1,
        &[
            disable(707),  // Bawb (center)
            disable(1057), // Bawb (west)
            disable(1133), // Sluggula
        ],
    );

    // Desert
    patcher.modify_objs(
        DungeonSand,
        3,
        &[
            disable(234), // Bawb
            disable(240), // Bawb
            disable(252), // Bawb
        ],
    );

    // Ice
    patcher.modify_objs(
        DungeonIce,
        1,
        &[
            disable(234), // Keelon
            disable(235), // Keelon
        ],
    );
}

//noinspection ALL
#[rustfmt::skip]
#[allow(unused)]
/// Development Sandbox
/// Make changes here for dev & testing we don't want to risk making it into the actual release.
fn do_dev_stuff(patcher: &mut Patcher, settings: &Settings) {
    if !settings.dev_mode {
        return;
    }

    // Ravio's Shop
    patcher.modify_objs(IndoorLight, 1, &[call(24, |obj| {
        obj.redirect(Dest::new(
            FieldLight, 27, 5,  // No Redirect
            // IndoorLight, 17, 0, // Bee Guy's House
            // CaveLight, 30, 0, // Witch Cave
            // DungeonHagure, 1, 0,  // Thieves' Hideout
            // FieldDark, 3, 0, // Lorule Death Mountain West
            // IndoorLight, 2, 0, // Witch's House
            // IndoorLight, 14, 0, // Stylish Woman's House
            // CaveLight, 15, 0, // Maiamai Cave
            // IndoorLight, 10, 0, // Rosso's House
            // FieldLight, 43, 0, // Sacred Realm
            // FieldLight, 36, 0,  // Hotfoot Area
            // FieldLight, 4, 3,
            // FieldLight, 18, 10, // Hyrule Castle Front Door
            // CaveLight, 7, 0, // Zora's Domain
            // IndoorLight, 15, 0, // Osfala Portrait
            // DungeonGanon, 1, 18, // LC 3F Center Warp Tile
            // CaveDark, 8, 0,     // Mysterious Man Cave
            // FieldDark, 31, 0, // Misery Mire
            // DungeonCastle, 6, 0, // Yuga 2 Boss
        ));
    })]);

    // Osfala Portrait House
    // patcher.modify_objs(IndoorDark, 15, &[
    //     redirect(6, 20, 1, 0), // Seres Portrait
    // ]);
}

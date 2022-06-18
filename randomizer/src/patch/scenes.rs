use albw::{
    course,
    scene::{Flag, Obj},
};

use super::Patcher;
use crate::{Result, Settings};

macro_rules! apply {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(course::Id::$course, $stage - 1)?.stage_mut().get_mut();
            $(action!((stage
                .get_mut($unq)
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
                .get_mut_system($unq)
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
    ($unq:tt.enable()) => {
        $unq.enable();
    };
    ($unq:tt.disable()) => {
        $unq.disable();
    };
    ($unq:tt.call $fn:block) => {
        ($fn)($unq);
    };
    ($unq:tt.redirect($spawn_point:literal, $scene:literal, $scene_index:literal)) => {
        $unq.redirect($spawn_point, $scene, $scene_index);
    };
}

pub fn apply(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
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
            [100].disable(None), // Keep Entry_KikoriMan3 from disappearing
            [101].disable(None),
            [135].disable(), // Disable IndoorLight4
            [136].enable(Flag::Event(250)), // Replace with IndoorLight10
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
            [144].disable(), // Buzz Blob
            [145].enable(), // Buzz Blob
            [146].enable(), // Buzz Blob
            [147].enable(), // Buzz Blob

            [102].disable(), // Bye Seres
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

            // TODO - What is Flag 390 ?
            // TODO - Where are rooftop Red Spear Soldiers?

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
            [496].enable(), // Paint Soldier
            [497].enable(), // Paint Soldier
            [498].enable(), // Paint Soldier
            [501].clear_enable_flag(), // TagDisableWallIn, prevent merging into barrier
            [532].disable(), // Buzz Blob
            [533].disable(), // AreaSimpleTalk - Hekiga_fueta_Green
            [534].disable(), // AreaSimpleTalk - Hekiga_Blue_Soldier
            [535].disable(), // AreaSimpleTalk - Hekiga_Blue_Soldier

            [155].enable(), // AreaChangeScene
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
        FieldLight 21 {
            [154].disable(), // Blacksmith's Wife
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
        // Lake Hylia
        FieldLight 35 {
            [233].disable(), // Open Maiamai Cave
            [235].disable(), // Remove the Sign
        },
        // Sacred Realm
        FieldLight 43 {
            [23].disable(), // seichi - "Sanctuary" - Initial text
            [26].disable(), // zelda_talk - Chat after standing up
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

            // Debug redirection
            // [24].call {|obj: &mut Obj| {
            //     obj.redirect(2, 3, 3);
            //     obj.arg_mut().1 = 1;
            // }},

            // Convert standing Ravio into shopkeeper Ravio
            [56].call {|obj: &mut Obj| {
                obj.arg_mut().3 = 0;

                obj.set_active_flag(Flag::Event(233));
                obj.set_inactive_flag(Flag::Event(597));

                obj.set_enable_flag(Flag::Event(233));
                obj.set_disable_flag(None);

                obj.set_translate(-1.0, 0.0, -5.5);
            }},

            // Double Sheerow
            [57].call {|obj: &mut Obj| {
                obj.set_active_flag(None);
                obj.set_enable_flag(Flag::Event(233));

                obj.set_disable_flag(None);
                obj.set_translate(-2.0, 0.0, -6.0)
            }},

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
        // Bar
        IndoorLight 15 {
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
            [0x84].enable(), // Enable Zora Queen event always
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
        // Thieves' Hideout
        DungeonHagure 1 {
            [541].enable(), // Thief Girl - Keep her from despawning after dungeon clear
            [1371].disable(), // Spear Boy AreaEventTalk
            [1372].disable(), // Spear Boy
        },
    );

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

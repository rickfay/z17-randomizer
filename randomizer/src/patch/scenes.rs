use {
    super::Patcher,
    crate::{logic_mode::LogicMode, patch::util::*, Result, Settings},
    albw::{
        course,
        course::{Id, Id::*},
        scene::{Arg, Flag, Obj, Transform, Vec3},
        Item::*,
    },
};

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

pub fn apply(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    debug_stuff(patcher, settings); // FIXME don't leave on

    patch_softlock_prevention(patcher, settings);
    patch_big_problem_chests(patcher, settings);
    patch_master_sword(patcher, settings);
    patch_castles(patcher, settings);
    patch_dark_maze(patcher, settings);
    patch_thief_girl_cave(patcher, settings);

    // Chamber of Sages
    patcher.modify_objs(CaveDark, 10, &[
        set_46_args(74, Flag::Event(0)), // Staircase
    ]);

    // Blacksmith Package Sword - Change to chest
    patcher.modify_objs(IndoorLight, 19, &[
        call(12, |obj| {
            obj.clear_active_args();
            obj.set_inactive_flag(Flag::Event(26));
            //obj.clear_disable_flag();
            obj.set_typ(1);
            obj.srt.translate.x = -1.957;
            obj.srt.translate.y = 0.6; //0.75;
            obj.srt.scale = match obj.id {
                35 => Vec3 { x: 1.00000, y: 2.00000, z: 2.22222 },
                // 35 => Vec3 { x: 1.00000, y: 1.50000, z: 2.22222 },
                34 => Vec3 { x: 0.52632, y: 2.00000, z: 1.66667 },
                // 34 => Vec3 { x: 0.52632, y: 1.50000, z: 1.66667 },
                _ => {
                    panic!("oh no")
                }
            }
        }),
        disable(19), // Map attention
    ]);

    // Ku's Domain
    patcher.modify_objs(FieldDark, 7, &[
        call(55, |obj| {
            obj.set_typ(4); // changed to chest automatically, set typ here
        }),
        disable(66), // rupee throw camera
    ]);

    apply!(patcher, // TODO convert to new approach

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
            //[23].disable(), // seichi - "Sanctuary" - Initial text
            //[32].disable(), // Remove Clouds
            [26].disable(), // zelda_talk - Chat after standing up
            [33].disable(), // zelda_talk_b - Wait for Zelda
            [34].disable(), // zelda_talk_c - Last chat before triangles
        },

        // Philosopher's Cave (outside)
        FieldDark 11 {
            [81].disable(), // Hilda text (frees Flag 523 for use)
        },

        // Lorule Blacksmith (outside)
        FieldDark 21 {
            [19].disable(), // Hilda Text (frees Flag 522 to be used to give Map Toggle)
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

fn patch_castles(patcher: &mut Patcher, settings: &Settings) {
    let green_pendant_flag = prize_flag(PendantCourage);
    let hc_31 = Flag::Course(31); // Set after yuga2 defeated
    let curtain_flag = Flag::Event(420); // repurposed
    let open_trials_flag = Flag::Event(421); // repurposed
    let can_fight_yuganon_flag = Flag::Event(670); // TODO separate from LC requirement

    // Hyrule Castle 1F (not the dungeon)
    patcher.modify_objs(IndoorLight, 12, &[
        // Convert Zelda to chest
        call(23, |obj| {
            obj.set_inactive_flag(Flag::Course(500)); // fake hopefully non-conflicting ID
            obj.clear_disable_flag();
            obj.set_typ(1);
        }),
    ]);

    // Hyrule Castle (exterior)
    patcher.modify_objs(FieldLight, 18, &[
        // Barrier
        set_46_args(165, Flag::Event(1)), // Enable Barrier from game start
        disable(505),                     // Barrier "would you like to save?" text
        // Pendant of Courage opens the Hyrule Castle Dungeon
        set_enable_flag(155, green_pendant_flag), // HC dungeon loading zone
        set_disable_flag(393, green_pendant_flag), // HC dungeon door
    ]);

    // 2F (there is no 1F of the dungeon)
    patcher.modify_objs(DungeonCastle, 1, &[
        set_disable_flag(19, hc_31), // Armos Statue
        call(35, move |obj| {
            // Warp
            obj.set_active_flag(hc_31);
            obj.set_enable_flag(hc_31);
        }),
    ]);

    // 4F
    patcher.modify_objs(DungeonCastle, 7, &[
        // set_disable_flag(10, yuga2_421_flag), // Yuga
        // set_disable_flag(12, yuga2_421_flag), // Zelda
        // set_disable_flag(13, yuga2_421_flag), // Zelda Portrait

        // set_enable_flag(19, yuga2_defeated_after_flag),
        // set_enable_flag(20, yuga2_defeated_after_flag),
        // set_enable_flag(21, yuga2_defeated_after_flag),
        // set_enable_flag(22, yuga2_defeated_after_flag),

        // Cutscene Trigger
        call(9, move |obj| {
            obj.set_active_flag(Flag::Event(1)); // activate regardless of barrier status
            // obj.set_inactive_flag(yuga2_421_flag);
            // obj.set_disable_flag(yuga2_421_flag);
        }),
    ]);

    // 7F
    patcher.modify_objs(DungeonCastle, 5, &[call(18, move |obj| {
        // warp
        obj.set_active_flag(hc_31);
        obj.set_enable_flag(hc_31);
    })]);

    // 8F
    patcher.modify_objs(DungeonCastle, 6, &[
        set_disable_flag(20, hc_31), // Rewire entrance door to stay open
        // disable(25), // victory door
        disable(28), // no revisits door
    ]);

    // Zelda's Study
    patcher.modify_objs(IndoorLight, 7, &[
        disable(27),                        // No backtracking Door
        set_disable_flag(26, curtain_flag), // Curtain
        set_disable_flag(29, curtain_flag), // AreaDisableWallIn
        // Portal
        call(10, move |obj| {
            obj.arg.3 = 0; // Prevent Long Portal Transition
            obj.set_active_flag(open_trials_flag); // Open Trials Door
        }),
    ]);

    // Hilda's Study
    patcher.modify_objs(IndoorDark, 5, &[
        disable(4),  // Trial's Door
        disable(12), // Yuga revives Ganon cutscene
        enable(34),  // Throne Room Loading Zone
        enable(23),  // Skull (top right, controller obj)
        // Portal
        set_46_args(14, curtain_flag), // Set Flag to remove curtain
    ]);

    // Hilda's Study (system)
    patcher.modify_system(IndoorDark, 5, &[
        enable(23), // Skull (top right, controller system obj)
        enable(24), // Skull (middle right)
        enable(25), // Skull (bottom right)
        enable(41), // Skull (bottom left)
        enable(46), // Skull (middle left)
        enable(47), // Skull (top left)
    ]);

    // Lorule Castle
    let skip_trials = settings.logic.skip_trials;
    patcher.modify_objs(DungeonGanon, 1, &[
        clear_enable_flag(1193), // Respawn Trial's Skip big rock upon leaving the room
        call(158, move |obj| {
            // Trial's Door
            if skip_trials {
                obj.disable();
            } else {
                obj.set_disable_flag(open_trials_flag);
            }
        }),
        disable(265), // Trial's Door camera pan
    ]);

    // Throne Room
    patcher.modify_objs(DungeonBoss, 1, &[
        // fight start trigger
        call(10, move |obj| {
            obj.set_enable_flag(can_fight_yuganon_flag);
            obj.set_active_flag(can_fight_yuganon_flag);
        }),
        clear_enable_flag(27), // Hilda
        clear_enable_flag(41), // camera offset
        clear_enable_flag(43), // NpcAttention1
        clear_enable_flag(48), // ObjPictureZelda
    ]);
}

fn patch_master_sword(patcher: &mut Patcher, _: &Settings) {
    // Lost Woods
    patcher.modify_objs(FieldLight, 1, &[
        // Rear LZ will always be present and lead to Master Sword area
        redirect(17, 16, 0, 33),
        // Repurpose front LZ for PoC, it'll block the Master Sword until obtained
        call(34, |obj| {
            obj.redirect(0, 0, 0);
            obj.set_active_flag(Flag::Event(1)); // might not need this
            obj.set_disable_flag(prize_flag(PendantCourage))
        }),
    ]);

    // Wisdom blocker LZ, disappears with PoW Flag
    let (flg1, flg3) = prize_flag(PendantWisdom).into_pair();
    patcher.add_obj(FieldLight, 1, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (0, flg1, 0, flg3),
        id: 8,
        lnk: vec![],
        nme: Some(String::from("Invalid")),
        ril: vec![],
        ser: Some(57),
        srt: Transform {
            scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 },
            rotate: Vec3 { x: 0.0, y: 180.0, z: 0.0 },
            translate: Vec3 { x: -20.0, y: 0.0, z: -25.5 },
        },
        typ: 6,
        unq: 327,
    });

    // Power blocker LZ, disappears with PoP Flag
    let (flg1, flg3) = prize_flag(PendantPower).into_pair();
    patcher.add_obj(FieldLight, 1, Obj {
        arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0),
        clp: 0,
        flg: (0, flg1, 0, flg3),
        id: 8,
        lnk: vec![],
        nme: Some(String::from("Invalid")),
        ril: vec![],
        ser: Some(58),
        srt: Transform {
            scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 },
            rotate: Vec3 { x: 0.0, y: 180.0, z: 0.0 },
            translate: Vec3 { x: -20.0, y: 0.0, z: -25.5 },
        },
        typ: 6,
        unq: 328,
    });

    // Master Sword Pedestal
    patcher.modify_objs(FieldLight, 34, &[call(71, |obj| {
        obj.clear_active_args();
        obj.set_inactive_flag(Flag::Course(150));
        obj.enable();
    })]);
}

fn patch_dark_maze(patcher: &mut Patcher, _: &Settings) {
    // Remove dialog
    patcher.modify_objs(FieldDark, 20, &[
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
    ]);
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

/// Modify the hitboxes of select big chests that could negatively affect gameplay
fn patch_big_problem_chests(patcher: &mut Patcher, settings: &Settings) {
    if !settings.options.chest_size_matches_contents {
        return;
    }

    const BIG_PROBLEM_CHESTS: [(Id, u16, u16); 19] = [
        (FieldLight, 3, 303),  // Death Mountain West Ledge
        (FieldLight, 34, 71),  // Master Sword Pedestal
        (FieldLight, 35, 155), // Lake Hylia Ledge
        (FieldLight, 33, 320), // Southern Ruins Ledge
        // (FieldLight, 1, 133),  // Lost Woods Big Rock
        (AttractionLight, 2, 33), // Southern Ruins Treasure Dungeon
        (DungeonDark, 2, 127),    // Dark 1F Fall from 2F
        (DungeonDark, 3, 269),    // Dark 2F East
        (DungeonWater, 1, 170),   // Swamp 1F West Room
        (DungeonWater, 1, 299),   // Swamp 1F East Room
        (DungeonWater, 1, 373),   // Swamp 1F SW Room
        (DungeonWater, 2, 620),   // Swamp B1 Raft Room (Left)
        (DungeonWater, 2, 621),   // Swamp B1 Raft Room (Right)
        (DungeonDokuro, 2, 105),  // Skull B2 Moving Platform Room
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
        patcher.modify_objs(stage, stage_index, &[call(unq, |obj| {
            if obj.id == 34 {
                obj.srt.scale.x = 0.52632; // 0.52632 * 1.9 (actor profile) ~= 1.0
                obj.srt.scale.z = 0.75; // 0.75 * 1.2 (actor profile) = 0.9
            }
        })]);
    }
}

fn patch_softlock_prevention(patcher: &mut Patcher, settings: &Settings) {
    // Gales 1F - Add trigger to drop wall if player entered miniboss without hitting switch
    patcher.add_obj(
        DungeonWind,
        1,
        Obj::trigger_cube(Flag::Course(60), 2, 146, 454, Vec3 { x: 16.5, y: 2.5, z: -19.0 }),
    );

    // Dark Maze w/o Merge
    if settings.logic.vanes_activated {
        // 1st Prison Cell softlock prevention
        patcher.add_obj(
            FieldDark,
            20,
            Obj::warp_tile(Flag::Event(1), 0, 66, 245, 0, 1, 19, Vec3 {
                x: 1.0 + 2.0,
                y: 0.5,
                z: 23.0,
            }),
        );

        // 2nd Prison Cell softlock prevention
        patcher.add_obj(
            FieldDark,
            20,
            Obj::warp_tile(Flag::Event(1), 0, 67, 246, 0, 1, 19, Vec3 {
                x: -17.0 + 2.5,
                y: 0.5,
                z: -17.0,
            }),
        );
    }

    // Swamp Palace SE Room w/o Merge
    // Swamp Palace SW Room w/o Merge
    // Skull Woods B2 Boss Hallway w/o Fire
}

#[rustfmt::skip]
fn debug_stuff(patcher: &mut Patcher, settings: &Settings) {
    // Ravio's Shop
    patcher.modify_objs(IndoorLight, 1, &[call(24, |obj| {
        obj.redirect(
            5, 0, 26, // No Redirect
            // 0, 22, 2, // Cucco Dungeon
            // 0, 1, 6, // Ku's Domain
            // 0, 0, 0,    // Lost Woods
            // 3, 0, 37,   // Lost Woods Maze - Unreachable Spawn Point
            // 1, 0, 37,   // Lost Woods Maze - Left 1st Poes
            // 20, 0, 17,  // HC Roof
            // 0, 2, 11, // Hyrule Castle 1F
            // 0, 5, 2,    // Swamp Cave
            // 0,21,0,     // Throne Room
            // 0, 8, 7,    // After final boss cutscene
            // 0, 0, 42,   // Sacred Realm
            // 0, 3, 4,    // Hilda's Study
            // 9, 0, 32,   // Southern Ruins Pillar Cave
            // 0, 2, 6,    // Zelda's Study (glitched for some reason)
            // 0, 3, 14,   // Osfala Portrait
            // 0, 1, 17,   // FieldDark 18
            // 7, 4, 17,   // Sanctuary Dungeon End
            // 0, 0, 33,   // Master Sword Pedestal
            // 0, 2, 9,    // Rosso House
            // 0, 14, 2,   // Swamp Palace 2F
            // 24, 14, 1,  // Swamp Palace River Room
            // 0, 0, 1,    // FieldLight 2
            // 0, 0, 6,    // Outside Zora's Domain
            // 4, 0, 8,    // Outside Fortune-Teller
            // 0, 12, 5,   // Yuga 2 Boss
            // 0, 12, 6,   // HC 4th Floor
            // 1, 3, 3,    // Lorule Blacksmith
            // 0, 12, 0,   // Hyrule Castle Dungeon
            // 2, 1, 30,   // Zaganaga Portal
            // 0, 1, 30,   // Misery Mire
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
            // 0, 10, 0,   // Gales Entrance
            // 0, 9, 2,    // Eastern Palace Boss
            // 0, 9, 0,    // Eastern Palace Entrance
            // 5, 0, 19    // Eastern Ruins WV
            // 0, 9, 0     // Eastern Palace Lobby
            // 20, 1, 0,   // Seres Portrait
            // 0, 4, 3     // Kak Well Lower
            // 1, 4, 3     // Kak Well Upper
            // 10, 11, 0   // Tower of Hera Boss
            // 0, 11, 0   // Tower of Hera Entrance
            // 0, 13, 0   // Dark Entrance
        );
    })]);
}

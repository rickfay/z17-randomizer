use albw::Item;

use super::Patcher;
use crate::Result;

macro_rules! apply {
    ($patcher:expr, $($course:ident/$name:ident {
        $([$index:literal $($kind:ident)?] $op:tt $rhs:tt,)*
    },)+) => {
        $({
            let mut flow = $patcher
                .flow(course!($course))?;
            let mut flow = flow
                .get_mut(stringify!($name))
                .ok_or_else(|| $crate::Error::game("File not found."))??;
            $({
                let step = flow.get_mut()
                    .get_mut($index)
                    .ok_or_else(|| $crate::Error::game(format!(
                        "Could not find command {:02X} in '{}/{}'.",
                        $index,
                        stringify!($course),
                        stringify!($name),
                    )))?;
                let mut step = kind!(step, $($kind)?)
                    .ok_or_else(|| $crate::Error::game(format!(
                        "Step {}/{} [0x{:X}] did not match expected type '{}'",
                        stringify!($course),
                        stringify!($name),
                        $index,
                        stringify!($($kind)?),
                    )))?;
                action!(step $op $rhs);
            })*
        })+
    };
}

macro_rules! kind {
    ($step:tt,) => {
        $step.into_action()
    };
    ($step:tt, $kind:ident) => {
        $step.$kind()
    };
}

macro_rules! course {
    (Boot) => {
        None
    };
    ($course:ident) => {
        ::albw::course::Id::$course
    };
}

macro_rules! action {
    ($command:tt = $kind:expr) => {
        $command.set_kind($kind);
    };
    ($command:tt value($value:expr)) => {
        $command.set_value($value);
    };
    ($command:tt => $next:expr) => {
        $command.set_next($next);
    };
    ($command:tt switch [
        $([$index:literal] => $next:expr,)*
    ]) => {
        $($command.set_branch($index, $next)?;)*
    };
    ($command:tt each [$($op:tt $rhs:tt,)+]) => {
        $(action!($command $op $rhs);)+
    };
}

pub fn apply(patcher: &mut Patcher, free: Item) -> Result<()> {

    // Debugging TODO Comment out
    // let mut flow = patcher.flow(FieldDark)?;
    // let flow = flow.get_mut(stringify!(FieldDark_1B_Hilda)).ok_or_else(|| Error::game("File not found."))??;
    // {
    //     flow.get().debug();
    // }


    apply!(patcher,

        // FieldDark/FieldDark_1B_Hilda {
        //     // FieldDark_1B_Hilda_ACT7_00
        //
        // },

        DungeonDark/Dark {
            //[0 into_start] => 5, // Goes to CoS, Gulley dialog but not model or warp to leave??
            // [0 into_start] => 7, // Crash
            // [6 into_start] => 5, // Nothing


            // [2 into_branch] each [
            //     value(6), // set ToC cutscene to require only 6 sages
            // ],

            [4] => 5, // skip Triforce of Courage cutscene! If Gulley is last
        },

        // CaveDark/CaveDark10 {
        //     [187] => 38, // skip 185
        //     [188] => 38, // skip 186
        //     //[] => 45, // skip 192 - none
        //     //[] => 46, // skip 196 - none
        //     //[] => 47, // skip 200 - none
        //     //[] => 48, // skip 213 - none
        //     //[] => 49, // skip 219 - none
        //     //[] => 50, // skip 223 - none
        //     [258] => 53, // skip 17
        //     [39] => 62, // skip 1
        //     [19] => 68, // skip 10
        //     [270] => 72, // skip 13
        //     //[] => 75, // skip 330 - too big ?
        //     //[] => 78, // skip 333 - too big ?
        //     [41] => 82, // skip 5
        //     //[] => 87, // skip 22 - none
        //     //[] => 92, // skip 26 - none
        //     [310] => 173, // skip 174
        //     //[] => 187, // skip 97 - none
        //     //[] => 188, // skip 99 - none
        //     //[] => 205, // skip 204 - none
        //     //[] => 241, // skip 308,225,245 - too big ?
        // },

        // Irene (bridge)
        FieldLight/FieldLight_2D_Maple {
            // NpcMaple_BellGet_2D
            [35] => 16,
            [21] => 7,
            [31] => 17,
            [28] => 18,
        },

        // Eastern Ruins
        FieldLight/FieldLight_1E_Sahasrahla {
            // lgt_NpcSahasrahla_Field1E_03
            //[0 into_start] => 0x6A,

            // 0,127,128,105,126,21,2,114,70,75,112,113
            [107] => 23, // skip 1
            // 23,24,29,26,68,
            [68] => 129, // skip 25,124
            // 130,131,132
            [103] => 104, // skip 102
            // 14,27,65,66,3,16,17,15,67,18
            [28] => 8, // skip 4
            // 22,69,5,9,13,6,
            //[13] => 106, // skip 6 - Leaving in so Link doesn't bumrush Sahas
            // 12,19,11,33,32,10,20,7 - END
        },

        // Hyrule Castle
        FieldLight/FieldLight_1B_Sahasrahla {
            // lgt_NpcSahasrahla_Field1B_00
            // 0,24,26,
            [35] =>  5, // skip 1
            [43] => 57, // skip 7
            [49] => 65, // skip 44
            [64] => 66, // skip 63
            [66] => 51, // skip 9
            [68] => 85, // skip 50
            // 81,82,83,86,84,20,54,53

            [67] => 55, // skip 10, 52
            //[55] => 23, // skip 22

            // 23 gives out item
            // FIXME Gift given too early b/c Sahasrahla moves during 10,52,22
            // 12,13,69,62,111
            [113] => 56, // skip 103, 91(goto), 105
            [56] => 73, // skip 11
            // 77,76,114,72 - FIN
        },
    );


    apply!(patcher,
        // Runaway Item Seller
        Boot/FieldLight_33_Douguya {
            // Entry_DouguyaOtto_00
            [5 into_start] => 0x3D, // Skip to Scoot Fruit choice
        },
        // Bird statues
        Boot/Telephone {
            // TelephoneCall
            [2] => 3, // Skip activation dialogue
            [0x0C into_branch] switch [
                [1] => None, // Skip break dialogue
            ],
        },
        // Rosso
        FieldLight/FieldLight_02_KikoriMan {
            // Entry_KikoriMan3
            [0x17 into_start] => 0x23,
        },
        // Climber
        FieldLight/FieldLight_05_Climber {
            // FieldLight_05_Climber
            [3 into_branch] each [
                = 0xA,
                value(0x395),
                switch [
                    [0] => 4,
                    [1] => 6,
                ],
            ],
        },
        // Shady Guy (Zora's Domain)
        FieldLight/FieldLight_0F_Zora {
            // Entry_ZoraEVT_TZK
            [2 into_start] => 0x102, // Skip to next event
            [0x102 into_start] => 0x7D,
            [0x7D] => 0xB3, // Set flag, then display text
        },
        // Irene (outside Fortune Teller)
        FieldLight/FieldLight_11_Maple {
            // NpcMaple_BellGet_11
            [0 into_start] => 6, // Skip to item get
            [6] => None,
        },
        // Irene (outside pond)
        FieldLight/FieldLight_12_Maple {
            // NpcMaple_BellGet_12_00
            [0 into_start] => 8, // Skip to item get
            [8] => None,
            // NpcMaple_BellGet_12_01
            [0x1E into_start] => 0x26, // Skip to item get
            [0x26] => None,
        },
        // Prologue (Seres and Dampe)
        FieldLight/FieldLight_13_Sister {
            // FieldLight_13_Sister_ACT2_heisicho
            [2 into_start] => 0x13, // Skip to door close
            [0x13] => 0x1D, // Skip to item get
            [0x1D] => 0x31, // Skip to music start
            [0x31] => 0x1F, // Skip to flags
            [0x20] => 0x6D,
        },
        // Merchant
        FieldLight/FieldLight_18_StandItem {
            // Stand_ZoraTreasure
            [0x4A into_branch] each [
                = 0xA, // Change to event flag
                value(0xD1),
                switch [
                    [0] => 0x10,
                    [1] => 0x4B,
                ],
            ],
            [0xF] each [
                = 0xE, // Change to event flag
                value(0xD1),
            ],
        },
        // Shady Guy (Kakariko)
        FieldLight/FieldLight_18_Touzoku {
            // Entry_TownTouzoku
            [0 into_start] => 2, // Skip to flag check
            [3 into_branch] switch [
                [1] => 0x12, // Skip to item get
            ],
            [0x1C] => None, // Set event flag then end
        },


        // Cucco Ranch
        FieldLight/FieldLight_29_Kokko {
            // kokko_game
            [0 into_start] => 0x58, // Skip directly to Rooster difficulty
        },
        // Pouch
        FieldLight/FieldLight_2A_BlacksmithWife {
            // BlacksmithWife_Pouch
            [0 into_start] => 0x15, // Skip to item get
            [0x15] => 0x0F, // Skip to event flag
        },
        // Under bridge
        FieldLight/FieldLight_2D_UnderBridgeStranger {
            // Entry_UnderBridgeStranger
            [1 into_branch] switch [
                [0] => 0x2A, // Skip to item get
            ],
        },
        // Fortune Teller
        IndoorLight/FieldLight_11_FortuneGirl {
            // HintGlassesGet
            [3 into_start] => 4, // Skip to item get
            [4] => None,
        },
        // Bee Guy
        IndoorLight/FieldLight_18_InsectNet {
            // Entry_MushitoriMan
            [9] => 0xE, // Skip text on first visit
            [0x10 into_branch] each [
                = 0xE,
                value(0xBE8), // Use scene flag
                switch [ // Swap branches
                    [0] => 0x11,
                    [1] => 2,
                ],
            ],
            [0xC convert_into_action] each [
                = 0x1E,
                value(0xBE8),
            ],
        },
        // Bar
        IndoorLight/FieldLight_18_MilkbarMaster {
            // FieldLight_18_MilkbarMaster
            [0x14 into_branch] each [ // Check message bottle flag
                value(0x394),
                switch [
                    [0] => 7,
                    [1] => 5,
                ],
            ],
            [0x0B convert_into_action] each [ // Unset message bottle flag
                = 1,
                value(0x394),
            ],
        },
        // Zelda
        IndoorLight/FieldLight_1B_Zelda {
            // ZeldaFirstTimeEvent_01
            [3 into_start] => 0x0C, // Skip to event flag
            [0x3E] each [
                value(0x81),
                => 0x24, // Skip to item get
            ],
            [0x24] => None,
        },
        // Blacksmith (Hyrule)
        IndoorLight/FieldLight_22_BlackSmith {
            [2] => None,
            [8] each [
                value(0x31),
                => None,
            ],
            [0x3E into_start] => 8,
            // FieldLight_22_BlackSmith_ACT6_SwordLvUP
            [0xC into_start] => 0xD, // skip Gulley dialogue
            [0xAD into_branch] switch [
                [0] => 0x16, // skip to item get
            ],
            [0xE9] => None,
        },
        // Ravio
        IndoorLight/FieldLight_2C_Rental {
            [0x2F] => 0xEE,
            [0xB7 into_start] => 0x229,
            [0xC0 into_start] => 0x13F,
            [0xCC into_start] => 0x276,
            [0xED] => None,
            [0xEE] => 0xED,
            [0x229] value(free as u32),
            [0x22A] each [
                = 0x1E,
                value(0xCAC),
                => 0xC7,
            ],
        },
        // Oren
        CaveLight/FieldLight_0F_Zora {
            [0x6B] => 0x7E,
            [0x158] => None,
            [0x4C into_branch] switch [
                [0] => 0x6F, // skip to Smooth Gem option
            ],
            [0x70 into_branch] switch [
                [0] => 0x6B, // skip to item get
            ],
        },
        // Blacksmith (Lorule)
        // IndoorDark/FiledDark_22_BlackSmithUra {
        // },
        // Chamber of the Sages
        // CaveDark/CaveDark10 {
        //     [0xF2] => 0xF3, // Skip Osfala giving Sand Rod
        // },
        // Hinox
        CaveDark/FieldDark_17_NpcHinox {
            // NpcHinox_event
            [0x1D] => 2,
            [2] => 6,
            [6] => 0x4C,
            [0x4C] => 0xA,
            [0xA] => 0x11,
            [0x11] => 0x16,
            [0x25] => 0x37,
        },
    );
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use albw::Item;
//
//     use super::apply;
//     use crate::{patch::Patcher, test_game, Result};
//
//     #[test]
//     fn it_works() -> Result<()> {
//         let mut patcher = Patcher::new(test_game()?, 0)?;
//         apply(&mut patcher, Item::KeySmall)
//     }
// }

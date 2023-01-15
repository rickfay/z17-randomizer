use {
    super::Patcher,
    crate::{Result, Settings},
    albw::Item,
};

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
    ($command:tt command($new_command:expr)) => {
        $command.set_command($new_command);
    };
    ($command:tt count($new_command:expr)) => {
        $command.set_count($new_command);
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

fn patch_portrait_requirements(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    let lc_requirement = settings.logic.lc_requirement as u32;
    //let yg_requirement = settings.logic.yuganon_requirement as u32;

    // TODO
    // let mut thing = patcher.flow(DungeonDark)?;
    // let mut stuff = thing.get_mut("Dark").unwrap().unwrap();
    // let flow = stuff.get_mut();
    //
    // let mut step = flow.get_mut(5).unwrap().convert_into_branch(2, 2).unwrap();
    // step.set_value(yg_requirement);
    // step.set_kind(15); // count sages function

    apply!(patcher,

        // Final Hilda before LC
        FieldDark/FieldDark_1B_Hilda {
            [3] => 5, // Skip text
        },

        // Set custom number of Portraits to enter LC
        DungeonDark/Dark   {[2 into_branch] value(lc_requirement),},
        DungeonWater/Water {[3 into_branch] value(lc_requirement),},
        FieldDark/Dokuro   {[3 into_branch] value(lc_requirement),},
        IndoorDark/Hagure  {[4 into_branch] value(lc_requirement),},
        Boot/Kame          {[3 into_branch] value(lc_requirement),},
        FieldDark/Sand     {[3 into_branch] value(lc_requirement),},
        DungeonIce/Ice     {[3 into_branch] value(lc_requirement),},
    );

    Ok(())
}

pub fn apply(patcher: &mut Patcher, free: Item, settings: &Settings) -> Result<()> {
    patch_portrait_requirements(patcher, settings)?;

    // Debugging
    // patcher
    //     .flow(albw::course::Id::IndoorLight)?
    //     .get_mut(stringify!(FieldLight_22_BlackSmith))
    //     .ok_or_else(|| crate::Error::game("File not found."))??
    //     .get()
    //     .debug();

    apply!(patcher,

        // Eastern Palace
        DungeonEast/East {
            [8] => None, // skip 15 - Don't set Flag 251
        },

        // Sacred Realm
        FieldLight/Ending {
            [226] => 82, // skip 21
            [27] => 202, // skip 23
            [227] => 98, // skip 201
            [94] => 25, // skip 24
            [101] => 135, // skip 99
        },

        // Master Sword Pedestal
        FieldLight/FieldLight_00_Mayoinomori {
            // [89 into_start] => 100, // don't skip Sahas noise or weird bottom screen bug happens
            [130] => 100, // skip Sahasrahla text
        },

        // Chamber of Sages
        CaveDark/CaveDark10 {

            // Skip Hilda intercepts for having 2/4/6 Sages
            // Skip flag 629 check to skip Triforce of Courage

            [266] => 18, // Skip 191,  1,  62, 63, 206 - Gulley?
            [263] => 69, // Skip 195, 10,  68, 67, 207 - Oren?
            [260] => 71, // Skip 199, 13,  72, 73, 208 - Seres?

            // Osfala?
            [258] => 309, // skip 17
            [309] => 241, // skip 308, 225, 245
            [2]   => 76, // Skip  77, 78, 209

            [254] => 81, // Skip 212,  5,  82, 83, 214 - Rosso?
            [253] => 86, // Skip 218, 22,  87, 88, 216 - Irene?
            [251] => 91, // Skip 222, 26,  92, 93, 221 - Impa?
        },

        // Hyrule Castle Dungeon
        DungeonCastle/Castle {
            [276] => 62,  // [274]
            [315] => 70,  // [ 19]
            [236] => 74,  // [ 37]
            [290] => 75,  // [284]
            [234] => 79,  // [ 18]
            [311] => 97,  // [255]
            [184] => 107, // [106]
            [107] => 110, // [109]
            [312] => 175, // [ 87]
            [313] => 176, // [ 88]
            [227] => 177, // [ 89]
            [281] => 178, // [ 16]
            [228] => 179, // [ 14]
            [229] => 180, // [ 17]
            [314] => 181, // [ 15]
            [237] => 183, // [104] // "what?"
            [171] => 184, // [ 29]
            [172] => 185, // [ 39]
            [131] => 222, // [127]
            [206] => 223, // [128]
            [135] => 224, // [129]
            [137] => 225, // [130]
            [282] => 269, // [270]
            [138] => 275, // [  1]
            [303] => 286, // [ 76] "I wish only to possess..."

            // After fight
            [5]   => 247, // skip 149
            //[155] => 249, // skip 40
            [40 convert_into_action] each [ // set Course Flag 1, if not already set, to spawn 4F enemies
                command(30),
                value(3001),
                count(0),
            ],
            [11] each [
                value(369), // set Flag 415, if not already set, to clear 4F cutscene
                => None, // End, skip setting Flag 420 and 421, freeing them for use
            ],
        },

        // Shady Guy (Zora's Domain)
        FieldLight/FieldLight_0F_Zora {
            [229] => 244, // skip 225 Youch!
            [227] => 238, // skip 261(branch),226
            [236] => 234, // skip 237
            [200] => 101, // skip 93
            [101] => 103, // skip 100
            [103] => 104, // skip 102
            [104] => 294, // skip 105
            [108] => 126, // skip 109
            // 173 END
            [125] => None, // Skip extra textbox
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

        // Great Rupee Fairy
        CaveDark/Cave {
            // CaveDark29_LuckyFairy_00
            // 5 start
            // 46
            // 35 checks 948 Flag to see if we've already gotten reward
            // 45
            [45] => 47, // Skip Would you like to throw text
            [47 into_branch] each [ // 47 checks that we have enough rupees
                value(3000),
            ],
            // 44 deposits 200 rupees as 10x Red Rupees
            [44] => 43,
            // 43 deposits 50 rupees as 10x Blue Rupees
            [43] => 25,
            // 25 Checks if 3000 have been deposited
            [27 convert_into_action] each [
                => 44, // create loop, depositing 200 then 50 until we hit 3000
            ],
            [32 convert_into_action] each [ // remove dialog
                command(0), // clear command
                => 36, // skip to item get
            ],
        },



        // Irene (bridge)
        FieldLight/FieldLight_2D_Maple {
            // NpcMaple_BellGet_2D
            [35] => 16,
            [21] => 7,
            [31] => 17,
            [28] => 18,
        },

        // Irene (outside Fortune Teller)
        FieldLight/FieldLight_11_Maple {
            // NpcMaple_BellGet_11
            [39] => 15, // skip 18
            [20] => 6, // skip 1
            [30] => 16, // skip 2,3,4
            [27] => 17, // skip 32,34,35,33 (including pan to Fortune-Teller)
        },

        // Irene (outside pond)
        FieldLight/FieldLight_12_Maple {
            // NpcMaple_BellGet_12_00
            [62] => 18, // skip 2,3,4
            [22] => 8, // skip 1
            [70] => 17, // skip 20
            [29] => 19, // skip 5

            // NpcMaple_BellGet_12_01
            [66] => 48, // skip 32,33,34
            [52] => 38, // skip 31
            [71] => 47, // skip 50
            [59] => 49, // skip 35
        },



        // Eastern Ruins
        // FieldLight/FieldLight_1E_Sahasrahla {
        //     // lgt_NpcSahasrahla_Field1E_03
        //     //[0 into_start] => 0x6A,
        //
        //     // 0,127,128,105,126,21,2,114,70,75,112,113
        //     [107] => 23, // skip 1
        //     // 23,24,29,26,68
        //     [68] => 129, // skip 25,124
        //     // 130,131,132
        //     [103] => 104, // skip 102
        //     // 14,27,65,66,3,16,17,15,67,18
        //     [28] => 8, // skip 4
        //     // 22,69,5,9,13,6
        //     //[13] => 106, // skip 6 - Leaving in so Link doesn't bumrush Sahas
        //     // 12,19,11,33,32,10,20,7 - END
        // },

        // Hyrule Castle
        // FieldLight/FieldLight_1B_Sahasrahla {
        //     // lgt_NpcSahasrahla_Field1B_00
        //     // 0,24,26,
        //     [35] =>  5, // skip 1
        //     [43] => 57, // skip 7
        //     [49] => 65, // skip 44
        //     [64] => 66, // skip 63
        //     [66] => 51, // skip 9
        //     [68] => 85, // skip 50
        //     // 81,82,83,86,84,20,54,53
        //
        //     [67] => 55, // skip 10, 52
        //     //[55] => 23, // skip 22
        //
        //     // 23 gives out item
        //     // fix - Gift given too early b/c Sahasrahla moves during 10,52,22
        //     // 12,13,69,62,111
        //     [113] => 56, // skip 103, 91(goto), 105
        //     [56] => 73, // skip 11
        //     // 77,76,114,72 - FIN
        // },
    );

    // untouched
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

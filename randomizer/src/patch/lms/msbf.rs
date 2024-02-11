use crate::patch::Patcher;
use crate::{Result, SeedInfo};
use game::Course;
use log::info;
use modinfo::settings::keysy::Keysy;
use modinfo::Settings;
use rom::string_constants;

pub fn patch(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    info!("Patching MSBF Files...");

    // research(patcher, None, "GameOver", None, true)?;

    let settings = &seed_info.settings;

    patch_ravio_shop(patcher)?;
    patch_thieves_hideout(patcher, settings)?;
    patch_turtles(patcher)?;
    patch_hyrule_castle_zelda(patcher)?;
    patch_lorule_castle_requirements(patcher, settings)?;
    patch_castle_connection(patcher, settings)?;
    patch_final_boss(patcher)?;
    patch_hint_ghosts(patcher)?;
    patch_weather_vanes(patcher)?;
    patch_rosso(patcher)?;
    patch_mother_maiamai(patcher)?;
    // patch_gameover(patcher)?;
    patch_stylish_woman(patcher)?;
    patch_woman(patcher)?;
    patch_treacherous_tower(patcher, seed_info)?;
    patch_bee_guy(patcher)?;
    patch_impa(patcher)?;

    legacy_patches(patcher)
}

/// Dev research, prints the contents of an MSBF file in a format for spreadsheets. Don't leave this on.
pub fn research<C, M>(patcher: &mut Patcher, course: C, file_name: &str, labels: M, edotor: bool) -> Result<()>
where
    C: Into<Option<Course>>,
    M: Into<Option<Vec<(String, String)>>>,
{
    let course = course.into();

    if let Some(file) = patcher.flow(course)?.get_mut(file_name) {
        if edotor {
            file?.get().edotor(labels.into().expect("No MSBT Message Info provided"));
        } else {
            file?.get().research();
        }
    } else {
        macros::fail!(
            "File not found: US{}.szs -> World/Flow/{}.msbf",
            if course.is_some() { "_English/".to_owned() + course.unwrap().as_str() } else { "/RegionBoot".to_owned() },
            file_name
        );
    };

    info!("Finished MSBF Research");
    Ok(())
}

/*
 * TODO - Want to rewrite this entire subsystem
 */

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
        ::game::Course::$course
    };
}

macro_rules! action {
    ($command:tt = $kind:expr) => {
        $command.set_kind($kind);
    };
    ($command:tt value($value:expr)) => {
        $command.set_value($value);
    };
    ($command:tt arg1($value:expr)) => {
        $command.set_arg1($value);
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

/// Ravio's Shop
fn patch_ravio_shop(patcher: &mut Patcher) -> Result<()> {
    let bow_slot_item = patcher.rentals[8];

    apply!(patcher,
        IndoorLight/FieldLight_2C_Rental {

            // Skip right to text
            [123] => 818,
            // Show mildly annoyed text if 2nd time speaking to him
            [817 into_text] => 806,

            // String together a bunch of text nodes to use to give out Sage locations
            [811] => 2,
            [2 into_text] => 812,
            [809] => 314,
            [314 into_text] => 810,
            [815] => 823,
            [823 into_text] => 816,
            [813] => 361,
            [361 into_text] => 814,

            // ---------------------------------------------------------------------------------------------------------

            [766 into_start] => 312, // 312 starts music
            [312] => 237, // 237 gives item
            [237] => None,

            // ???
            [192 into_start] => 319,
            [204 into_start] => 630,

            // ???
            [183 into_start] => 553,
            [553] value(bow_slot_item as u32),
            [554] each [
                = 30,
                value(3244),
                => 199,
            ],
        },
    );

    Ok(())
}

fn patch_thieves_hideout(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    // Small Keysy patch for Thief Girl. She will look at the Flag for the locked door (course flag 67) to determine if
    // she should follow Link or continue to sass him. Because Small Keysy removes the door altogether, the flag is
    // never actually set. This adjusts the flowchart to skip Flag 3067 checks and assume it's been set.
    // TODO Devise a mechanism to preset Course Flags.
    match settings.keysy {
        Keysy::SmallKeysy | Keysy::AllKeysy => {
            apply!(patcher,
                DungeonHagure / Hagure {
                    [6 into_branch] switch [[1] => 35,],
                    [7] => 70,
                },
            );
        },
        _ => {},
    }

    apply!(patcher,
        DungeonHagure / Hagure {
            [17] => 18, // Skip 16 "We're locked in!"
            [100] => 93, // Skip 91 "We're locked in!"

            // [17] => None, // Skip 16, 18 "We're locked in!" and camera pan
            // [100] => None, // Skip 91, 93 "We're locked in!" and camera pan

            [23] => None, // Skip 25, 24 "We're cut off!" and camera pan
            [113] => None, // Skip 105, 149, 153 "We're cut off!" and camera pan

            [50] => 49, // Skip 48 Thief Girl's "Gyaaah! What gives?!" when entering boss (not needed?)
            [156] => 157, // Skip 45 Thief Girl's "Gyaaah! What gives?!" when entering boss
            // Intentionally keeping Stalblind's textbox 49 / 46

            // Skip Post-boss text, instead set flag to activate shield to reach boss chest
            [40 convert_into_action] each [
                arg1(3),
                value(3006),
                command(30),
                count(0),
            ],
        },



        FieldDark/FieldDark_16_HagureHouse {
            [46] => 41, // Skip 42 text in Thieves' Town "You're looking for that painting, yah?"
        },

        IndoorDark/FieldDark_16_HagureHouse {
            [27] => 30, // Skip 25 text in front of painting "This is the one, right?"
        },
    );

    Ok(())
}

/// Turtle Rock Turtles
fn patch_turtles(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        FieldDark/FieldDark_35_Kame {
            [40] => 41, // Skip Jillo text (Bullied Turtle)
            [42] => 43, // Skip Tallo text (Wall Turtle)
            [44] => 45, // Skip Sabro text (Flipped Turtle)
            [29] => 46, // Skip Mama Turtle "You found all my babies!..." text
        },
    );

    Ok(())
}

/// Mother Maiamai Cave and Rewards
fn patch_mother_maiamai(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        CaveLight/FieldLight_35_Kinsta {

            // --------------------------------------------------------------------
            // 16 START
            // --------------------------------------------------------------------

            //[16 into_start] => 15, // Skip 137 (?)
            // [109] => 20, // Hard lock (don't do this)
            [63] => 19, // Skip 17
            [58] => 68, // Skip 54

            // --------------------------------------------------------------------
            // 30 START
            // --------------------------------------------------------------------

            [31 into_branch] switch [[0] => 24,],
            [35 into_branch] switch [[1] => 94,],
            [25] => 94,

            // --------------------------------------------------------------------
            // 39 START
            // --------------------------------------------------------------------

            // Skip various "I can make your item nicer" texts
            [89] => 45,
            [90 into_branch] switch [[1] => 45,],
            [72] => 45,
            [73 into_branch] switch [[1] => 45,],

            // Item Selection Cancel Text
            [43 into_branch] switch [
                [1] => 46,
                [2] => 46,
                [3] => 46,
            ],

            // Cancel quicker, Skips going back down 77 START
            [42] => 69,
            [76 into_branch] switch [[1] => 69,],

            [133 into_branch] switch [[0] => 47,], // Skip 139,50,48
            [47] => 83, // Skip 51,49
            //[47] => 81, // Skip 51,49,83,84 (10x Maiamai jump to water)

            // --------------------------------------------------------------------
            // 77 START
            // --------------------------------------------------------------------

            // Skip advice on finding Maiamai
            [140 into_branch] switch [[1] => 80,],
            [141] => 80,
            [111 into_branch] switch [[0] => 80,],
            [110 into_branch] switch [[1] => 80,],
        },
    );

    Ok(())
}

fn patch_hyrule_castle_zelda(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        IndoorLight/FieldLight_1B_Zelda {
            [18] => 16, // Skip 23 - Fancy cutscene of Zelda spinning around

            [17] => 57,

            // [17] => 72, // Skip 9 "I bid you fondest..."
            // [70] => 19, // Skip 24, 25 "Ah, while your name..."
            // [41] => 42, // Skip 10 "Seres has been..."
            // [42] => 39, // Skip 68 "I sense a terrible..."
            // [43] => 46, // Skip 11 "Oh, Impa, I fear..."
            // [32] => 78, // Skip 26 "Fret not, Princess. I'd..."
            // [44] => 80, // Skip 27, 31 "Yes, of course. That's..."
            // [81] => 12, // Skip 72 "I am certain he will..."
            // // 12 sets Event Flag 225
            // [45] => 54, // Skip 29 "Now there's just one more..."
            // [54] => 48, // Skip 53 "It's a rather special charm."
            // [34] => 49, // Skip 28 "Are you sure about..."
            // [35] => 56, // Skip 30 "Quit sure..."
            // [56] => 57, // Skip 55 "This has been in my safekeeping..."

            // 36 gives Charm
            [66] => 88, // Skip 37 "Please, tell Sahasrahla..."
        },
    );

    Ok(())
}

fn patch_lorule_castle_requirements(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    let lc_requirement = settings.lc_requirement as u32;

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

fn patch_castle_connection(patcher: &mut Patcher, _settings: &Settings) -> Result<()> {
    apply!(patcher,

        // Hyrule Castle Dungeon
        DungeonCastle/Castle {
            // Text skip
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
            [5]   => 247,  // skip 149 - "Gah! I bore of this fight.\0"
            [155] => 249,  // skip 40  - "Once I have released Ganon..."
            [11]  => None, // Set 420 but then skip setting 421, freeing it for use
        },
    );

    Ok(())
}

/// Hint Ghosts. Skip straight to hint, skipping as much as possible.
fn patch_hint_ghosts(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        Boot/HintGhost {
            // Most Hint Ghosts
            [0 into_start] => 8,
            [8] => None,

            // Lost Woods Hint Ghosts
            [19 into_start] => 35,
            [35] => None,
        },
    );

    Ok(())
}

fn patch_final_boss(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        DungeonBoss/Ganon {
            [69] => 72, // Skip 1st Zelda text
            [90] => 91, // Skip 2nd Zelda text
        },
    );

    Ok(())
}

fn patch_weather_vanes(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        Boot/Telephone {

            // Cut out as much as possible from 1st time activation
            //[2] => None, FIXME DONT LEAVE LIEK THS
            //[19 into_text] => None, // bad - leaves WV screen open unable to be closed

            // Never show "You've been playing for a while..." text
            [4 into_branch] switch [[1] => None,],
            [10 into_text] => None,
        },
    );

    Ok(())
}

fn patch_rosso(patcher: &mut Patcher) -> Result<()> {
    // Rosso's House
    apply!(patcher,
        IndoorLight/FieldLight_02_KikoriMan {

            // Rearrange...
            [3 into_start] => 4,
            // Check if we've received our item yet
            [4 into_branch] each [
                value(261), // Flag 344
                switch [
                    [0] => 13, // Give out item path
                    [1] => 40, // Repeat visits text
                ],
            ],

            // Has Smashed Rocks check
            [40 into_branch] switch [
                [0] => 16, // 19,
                // [1] - "Glad to share what's in that chest with you. You earned it, kid!"
            ],

            // Has Power Glove check
            [16 into_branch] switch [
                [0] => 19, // Has glove, long text about picking up rocks
                [1] => 7, // No glove
            ],

            // No Glove path "Urggh! These rocks! Real pain in the neck!"
            [7 into_text] => None,

            // Give out item path - skip text
            [12] => 69,
            [68] => 14,
            [15] => 21,
            [96] => None,
        },
    );

    // Outside Rosso's House
    apply!(patcher,
        FieldLight/FieldLight_02_KikoriMan {
            // After all rock cleanup, remove text from Rosso exiting house
            [77] => 28,
            [78] => 32,
            [31] => 34,
            [83] => 80,
        },
    );

    Ok(())
}

/// Stylish Woman
fn patch_stylish_woman(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        IndoorLight/FieldLight_18_ClosedHouse {
            [12] => 41, // Skip 1
            [26] => 20, // Skip 3
            [28] => 44, // Skip 21
            [48] => 23, // Skip 46
        },
    );

    Ok(())
}

/// Woman
fn patch_woman(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        IndoorLight/FieldLight_18_MiddleLady {
            [12 into_branch] switch [[0] => 15,], // Skip 11
        },
    );

    Ok(())
}

/// Treacherous Tower
fn patch_treacherous_tower(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let first_course = seed_info.treacherous_tower_floors.get(0).unwrap().course as u32;

    apply!(patcher,
        FieldDark / FieldDark_05_GameTower {
            // --- Before Game dialog --- //
            [14] => 203, // Skip 62, 202
            // 66 is Question dialog
            [71 into_branch] each [
                count(2), // Only allow 2 options
                switch [
                    [1] => 201, // 2nd option becomes "I'll pass", Skip 72
                ],
            ],
            [92 into_branch] each [
                value(200), // Check intermediate price of 200 (advanced is 300)
            ],
            // 91 tells how many floors
            [99] each [
                value(-200i32 as u32), // Charge intermediate price of 200 (advanced is 300)
            ],
            [104] => 108, // Skip 100
            [98] => 97, // Skip 94
            [97] => 110, // Skip 95, 112, 153
            [96] each [
                value(first_course),
            ],
            [102] each [
                value(first_course),
            ],

            // --- After Game dialog --- //
            [19] => 118,
            [142 into_branch] switch [
                [0] => 141, // Skip 137
            ],
            // 141 is ItemKandelaarLv2
            [141] => 143, // Skip 146
            [136 into_branch] switch [
                [1] => 143, // Skip Super Net stuff
            ],
            [140] => 143, // Skip Super Net stuff
            [209 into_start] => 210, // Skip 208
        },
    );

    Ok(())
}

/// Bee Guy
fn patch_bee_guy(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        // Bee Guy
        IndoorLight/FieldLight_18_InsectNet {
            [1 into_start] => 7,
            [16 into_branch] each [
                command(14), // Check Course Flag 48
                value(3048),
                switch [
                    [0] => 11,
                    [1] => 29,
                ],
            ],
            [10 convert_into_action] each [
                command(30), // Set Course Flag 48
                value(3048),
            ],
            [30 into_branch] switch [[0] => 51,], // Skip 53
            [51] => 31, // Skip 28
            [52] => 33, // Skip 32
            [38 into_branch] switch [[1] => 2,],
        },
    );

    Ok(())
}

/// Impa's conversation with the soldier
fn patch_impa(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        FieldLight/FieldLight_1B_Soldier {
            [84] => 34, // Skip 29, 59
            [58] => 62, // Skip 30
            [97] => 63, // Skip 31
            [63] => 38, // Skip 60
            [139] => 95, // Skip 32
            [140] => 96, // Skip 61, 65
            [73] => 102, // Skip 72
            [103] => 57, // Skip 99
        },
    );

    Ok(())
}

/// Quit / Game Over dialog
#[allow(unused)]
fn patch_gameover(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,
        Boot/GameOver {

            // 9 => Fairy Revival? Only if player has fairy (actually caused Fairy revival but no text, stuck in dialog with no dialog)
            // 12 => He just lies there...?

            [10 into_branch] switch [
                [0] => 13,
                [1] => 13,
            ],

            [13 into_branch] switch [
                [0] => 4,
                [1] => 4,
            ],

            [4 into_branch] switch [
                [0] => 5,
                [1] => 5,
            ],

            [14 into_branch] switch [
                [0] => 2,
                [1] => 2,
            ],

            [2 into_branch] switch [
                [0] => 15,
                [1] => 15,
            ],

            // 9-10-13-4-5-14-8 = Rental Items drop, [Quit, Continue], Hit continue => Nothing
            // 9-10-13-4-5-14-2 = Rental Items drop, [Quit, Continue], Hit Quit, "Sorry buddy...", take items, success respawn (dungeon death also works as expected)


        },
    );

    Ok(())
}

/// Mostly a bucket for patches from the OG rando with some misc. unsorted patches mixed in.
pub fn legacy_patches(patcher: &mut Patcher) -> Result<()> {
    apply!(patcher,

        // Irene Bell Text
        FieldLight/FieldLight_WarpEvent { [0 into_start] => None, },
        FieldDark/FieldLight_WarpEvent  { [0 into_start] => None, },

        // Sahasrahla
        FieldLight/FieldLight_1B_Sahasrahla {
            [14 into_start] => 22,
            [22 into_text] => None,
        },

        // Runaway Item Seller
        Boot/FieldLight_33_Douguya {
            // Entry_DouguyaOtto_00
            [5 into_start] => 71, // Skip to check obtained Flag (3/168)
            [71 into_branch] switch [
                [0] => 32, // Skip to hand in Scoot Fruit dialog
            ],
            [73] => 70, // Don't lose Scoot Fruit
            [70] => None, // End after setting obtained Flag (3/168)
        },

        // // Hyrule Hotfoot
        // FieldLight/FieldLight_HyruleRace {
        //
        //     // skip initial branching text describing which race it is
        //     // keep in textbox telling players the price
        //     [71] => 50,
        //
        //     // Win race - skip to reward
        //     [28 into_branch] switch [
        //         [0] => 57, // skip 12
        //     ],
        //     [57] => 18, // skip 34
        //
        //     // Move reward to first race, silver rupee to 2nd
        //     [18 into_branch] switch [
        //         [0] => 20,
        //         [1] => 33,
        //     ],
        //
        //     // Remove post-reward text
        //     [37] => None,
        //     [33] => None,
        //     [21] => None,
        // },

        // Mysterious Man Cave
        CaveDark/FieldDark_00_GoldenBeeShop {
            [0 into_start] => 9, // allow repeated purchases
        },

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
            [242] => 243, // Skip 226 (ItemRentalSandRodFirst)
            [2]   => 76, // Skip  77, 78, 209

            [254] => 81, // Skip 212,  5,  82, 83, 214 - Rosso?
            [253] => 86, // Skip 218, 22,  87, 88, 216 - Irene?
            [251] => 91, // Skip 222, 26,  92, 93, 221 - Impa?
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

            /*
             Change options:
             - "Throw 50" => "Throw 3000"
             - "Throw 200" => "Don't throw any"
             - Remove third option (done by changing [6]'s message text)
            */
            [8 into_branch] each [
                count(2),
                switch [
                    [0] => 47, // "Throw 3000" option will appear to throw 10 Reds
                    [1] => 9, // "Don't throw any" option is now second
                ],
            ],

            [47 into_branch] each [ // 47 checks that we have enough rupees
                value(3000),
            ],

            // 44 deposits 200 rupees as 10x Red Rupees
            [44] => 43,

            // Deduct 2800 rupees so the player has effectively given 3000
            [43] each [
                value(0xFFFFF510), // Negative 2800
                command(37),
                => 41,
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
        // Bouldering Guy
        FieldLight/FieldLight_05_Climber {
            [3 into_branch] each [
                switch [
                    [0] => 18,
                ],
            ],
            [18] => 28,
            [30] => 52,
            [52] => 21,
            [21] => 51,
            [51 convert_into_action] each [
                arg1(6), // give out item
                value(0x13), // Static empty bottle
                => 6, // goto actual prize
                = 0xB, // give out item
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

string_constants! {
    #[allow(non_upper_case_globals)]
    MsbfKey {
        Castle,
        CatchInsect,
        Cave,
        CaveDark10,
        cl_Church_UG,
        CrossBattle,
        CrossBoard,
        CrossForceTalk,
        CrossOldMan,
        Dark,
        Dokuro,
        DoorHouse,
        E3_flow,
        East,
        Ending,
        FieldDark_00_GoldenBeeShop,
        FieldDark_05_GameTower,
        FieldDark_0F_Namazu,
        FieldDark_13_Sinpu,
        FieldDark_14_Danpei,
        FieldDark_16_HagureHouse,
        FieldDark_16_MagicShop,
        FieldDark_17_NpcHinox,
        FieldDark_18_BakudanTouzoku,
        FieldDark_18_BoxManDark,
        FieldDark_18_ItemShop,
        FieldDark_1A_FortuneGirlUra,
        FieldDark_1B_Bakudanya,
        FieldDark_1B_Hilda,
        FieldDark_1E_Sennyukun,
        FieldDark_28_Minigame,
        FieldDark_29_BakudanShop,
        FieldDark_29_HappyFairy,
        FieldDark_2A_GameMaster,
        FieldDark_2C_RaviosDiary,
        FieldDark_33_Daibakudankabe,
        FieldDark_33_Touzoku,
        FieldDark_35_ItemShop,
        FieldDark_35_Kame,
        FieldDark_3A_CrazyMan,
        FieldDark_Tennokoe,
        FieldLight_00_JyohoShop,
        FieldLight_00_Mayoinomori,
        FieldLight_02_KikoriMan,
        FieldLight_03_Kanban,
        FieldLight_05_Climber,
        FieldLight_0A_Kanban,
        FieldLight_0F_Kanban,
        FieldLight_0F_Zora,
        FieldLight_11_FortuneGirl,
        FieldLight_11_Maple,
        FieldLight_12_Maple,
        FieldLight_12_SignBoard,
        FieldLight_13_Danpei,
        FieldLight_13_Medium,
        FieldLight_13_SignBoard,
        FieldLight_13_Sinpu,
        FieldLight_13_Sister,
        FieldLight_14_Danpei,
        FieldLight_14_Maple,
        FieldLight_16_Ending,
        FieldLight_16_MagicShop,
        FieldLight_16_Obaba,
        FieldLight_16_SignBoard,
        FieldLight_17_Kanban,
        FieldLight_18_Bard,
        FieldLight_18_BoxMan,
        FieldLight_18_ClosedHouse,
        FieldLight_18_InsectNet,
        FieldLight_18_ItemShop,
        FieldLight_18_Kakarikoboy,
        FieldLight_18_KakarikoGirl,
        FieldLight_18_MaidSahasulala,
        FieldLight_18_MiddleLady,
        FieldLight_18_MiddleMan,
        FieldLight_18_MilkbarMaster,
        FieldLight_18_MilkbarSoldier,
        FieldLight_18_Rotenshonin,
        FieldLight_18_SahasPupil,
        FieldLight_18_SignBoard,
        FieldLight_18_Soldier,
        FieldLight_18_StandItem,
        FieldLight_18_Touzoku,
        FieldLight_1A_Maple,
        FieldLight_1A_SignBoard,
        FieldLight_1B_BlackSmithKid,
        FieldLight_1B_Commander,
        FieldLight_1B_Hekiga,
        FieldLight_1B_Impa,
        FieldLight_1B_Rakcha,
        FieldLight_1B_Sahasrahla,
        FieldLight_1B_Soldier,
        FieldLight_1B_Zelda,
        FieldLight_1E_Sahasrahla,
        FieldLight_22_BlackSmith,
        FieldLight_22_BlackSmithKid,
        FieldLight_22_BlackSmithWife,
        FieldLight_22_Dwarf,
        FieldLight_22_Maple,
        FieldLight_28_Minigame,
        FieldLight_29_Kokko,
        FieldLight_2A_BlacksmithKid,
        FieldLight_2A_BlacksmithWife,
        FieldLight_2B_AppleTree,
        FieldLight_2B_BlackSmithKid,
        FieldLight_2B_Maple,
        FieldLight_2C_BlackSmithKid,
        FieldLight_2C_GanbariTutorial,
        FieldLight_2C_Rental,
        FieldLight_2C_RentalItem,
        FieldLight_2C_SahasPupil,
        FieldLight_2C_Sahasrahla,
        FieldLight_2C_SignBoard,
        FieldLight_2C_Soldier,
        FieldLight_2D_Maple,
        FieldLight_2D_UnderBridgeStranger,
        FieldLight_2E_Maple,
        FieldLight_33_Douguya,
        FieldLight_35_Douguya,
        FieldLight_35_ItemShop,
        FieldLight_35_Kinsta,
        FieldLight_35_Marutakun,
        FieldLight_35_Zora,
        FieldLight_37_MessageBottle,
        FieldLight_BlacksmithWife,
        FieldLight_HyruleRace,
        FieldLight_Tennokoe,
        FieldLight_WarpEvent,
        FiledDark_22_BlackSmithUra,
        FiledDark_22_BlackSmithWifeUra,
        GameOver,
        Ganon,
        GirigiriGameTest,
        Hagure,
        Hera,
        HintGhost,
        Ice,
        IndoorDark1_ZoraQueen,
        IndoorDark2_Demo080,
        Kame,
        MessageBoard,
        MiniDungeon_FieldDark_2B,
        MiniDungeon_FieldLight_07,
        MiniDungeon_FieldLight_15,
        MiniDungeon_FieldLight_1E,
        MiniDungeon_FieldLight_32,
        MiniDungeon_FieldLight_33,
        NpcClimberTest,
        NpcHinox,
        NpcShadowLink,
        NpcStand,
        npcTest00,
        NpcTestIwata,
        NpcTownEtc,
        Sand,
        Telephone,
        test,
        ToRentalShopBoard,
        Water,
        Wind,
        yamazaki,
        yamazaki2,
    }
}

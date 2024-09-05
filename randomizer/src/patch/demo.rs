use crate::SeedInfo;
use game::Course::*;
use log::info;
use modinfo::settings::cracks::Cracks;
use modinfo::settings::cracksanity::Cracksanity;
use modinfo::settings::trials_door::TrialsDoor;
use modinfo::settings::weather_vanes::WeatherVanes::*;
use rom::flag::Flag;
use rom::scene::SpawnPoint;
use rom::{Demo, File};

/// Cutscene file recreation.
/// Files are not read from the ROM but instead created from scratch given their (mostly) short lengths.
pub(crate) fn build_replacement_cutscenes(seed_info: &SeedInfo) -> crate::Result<Vec<File<Demo>>> {
    info!("Building Replacement Cutscenes...");

    const INITIAL_SPAWN: SpawnPoint = SpawnPoint { course: IndoorLight, scene: 1, spawn: 1 };

    // Demo1 - Link's Nightmare cutscene (goes to Link's House 0)
    let mut demo1 = Demo::new();
    get_initial_flags_to_set(seed_info).iter().for_each(|&flag| demo1.set_event_flag(0, flag));
    demo1.finish(0, INITIAL_SPAWN);

    // Demo2 - Sanctuary cutscene (goes to Link's House 2)
    let mut demo2 = Demo::new();
    demo2.set_event_flag(0, 110);
    demo2.finish(0, SpawnPoint::new(IndoorLight, 1, 2));

    // Demo3 - Eastern Palace Yuga cutscene (goes to Eastern Palace 3F)
    let mut demo3 = Demo::new();
    demo3.set_event_flag(0, 250); // todo
    demo3.finish(0, SpawnPoint::new(DungeonEast, 3, 1));

    // Demo4 - Yuga revives Ganon (after IHC) cutscene (goes to Lorule Blacksmith)
    let mut demo4 = Demo::new();
    demo4.set_event_flag(0, 430);
    demo4.finish(0, SpawnPoint::new(IndoorDark, 4, 1));

    // Demo5 - Lorule Sacred Realm cutscene (goes to Hyrule's Sacred Realm)
    let mut demo5 = Demo::new();
    demo5.finish(0, SpawnPoint::new(FieldLight, 43, 0));

    // Demo6 - Final Boss start cutscene (goes to Yuganon Phase 1)
    let mut demo6 = Demo::new();
    demo6.set_event_flag(0, 718);
    demo6.finish(0, SpawnPoint::new(DungeonBoss, 1, 1));

    // Demo7 - Final Boss halfway cutscene (goes to Yuganon Phase 2)
    let mut demo7 = Demo::new();
    demo7.set_event_flag(0, 722);
    demo7.finish(0, SpawnPoint::new(DungeonBoss, 1, 2));

    // Demo8 - Zelda saved cutscene after Final Boss (goes to Demo5)
    let mut demo8 = Demo::new();
    demo8.finish(0, SpawnPoint::new(Demo, 5, 0));

    // Demo9 - Touch the Triforce cutscene (goes to FieldLight41)
    // let mut demo9 = Demo::new();
    // demo9.finish(0, SpawnPoint::new(FieldLight, 41, 0));

    // Demo10 - Title Screen story slideshow
    // let mut demo10 = Demo::new();
    // demo10.finish(0, SpawnPoint::new(FieldLight, 0, 0));

    let demo_files = vec![
        File::new("World/Demo/Demo1.csv".to_owned(), demo1),
        File::new("World/Demo/Demo2.csv".to_owned(), demo2),
        File::new("World/Demo/Demo3.csv".to_owned(), demo3),
        File::new("World/Demo/Demo4.csv".to_owned(), demo4),
        File::new("World/Demo/Demo5.csv".to_owned(), demo5),
        File::new("World/Demo/Demo6.csv".to_owned(), demo6),
        File::new("World/Demo/Demo7.csv".to_owned(), demo7),
        File::new("World/Demo/Demo8.csv".to_owned(), demo8),
        // File::new("World/Demo/Demo9.csv".to_owned(), demo9),
        // File::new("World/Demo/Demo10.csv".to_owned(), demo10),
    ];

    Ok(demo_files)
}

fn get_initial_flags_to_set(SeedInfo { trials_config, settings, .. }: &SeedInfo) -> Vec<u16> {
    let mut flags = vec![
        1, 7, 9, 10,  // Skip Gulley in prologue
        11,  // Fix Hyrule lighting, skip Gulley dialogue at Blacksmith
        20,  // Disable Gulley's callback
        55,  // ?
        84,  // Enable Dampe + Seres conversation
        107, // Spawn enemies
        110, // Post Sanctuary
        222, // Open Hyrule Castle Front Door
        223, // Skip Hyrule Castle Art Gallery Event
        225, // Correct field music
        231, // Skip Hyrule Castle events
        232, // Enable Ravio's freebie
        //235, // Suppress Ravio's Signs, Huh? Not Interested? text, but also Freebie =\
        236, // Enable Stamina bar
        241, // Skip Osfala intro
        //246, // Skip Irene, make Hyrule Hotfoot appear, spawns certain enemies
        248, // Skip Yuga killing Osfala
        //250, // Yuga 1 Defeated
        //251, // Set in Post-EP FieldLight20 cutscene, being used as PoC Flag
        310, // Watched HC Post-EP cutscene, fixes overworld music issues
        315, // Shop open???
        // 320, // Shady Guy Trigger
        321, 322, // Skip first Oren cutscenes
        374, // Fix Post-Gales and Post-Hera music by marking Sahasrahla telepathy as seen
        415, // Skip Yuga capturing Zelda
        // 410, // Hyrule Castle Barrier
        // 420, // Yuga 2 defeated in HC
        522, // Blacksmith Hilda Text, enable Map Swap icon, skip introductory Lorule music
        523, 524, 560, 600, 620, 640, // Skip Hilda Text, enable Lorule overworld music
        525, // Skip Sahasrahla outside Link's House, make Hyrule Hotfoot appear
        // 536, 537, // Gulley Flags
        // 556, 557, // Oren Flags
        // 576, 577, // Seres Flags
        // 596, 597, // Osfala Flags
        // 616, 617, // Rosso Flags
        // 636, 637, // Irene Flags
        // 656, 657, // Impa Flags
        542, 543, // Skip Bomb-Shop Man dialogue
        599, // Disable Sand Rod return
        897, // Big Special Something
        899, // Enable Quick Equip
        902, // StreetPass Tree
        906, // Monster Guts
        907, // Monster Tail
        908, // Monster Horn
        919, // Skip Hint Ghost tutorial
        // 920, // Link's House Weather Vane
        // 940, // Vacant House Weather Vane
        950, // Maiamai
        955, // Master Ore Icon
        960, // Blacksmith's Wife
        965, // Suppress Energy Potion
    ];

    // Cracks Open/Closed + Quake
    if settings.cracks == Cracks::Open {
        flags.push(510);
    }

    // Ravio's Shop Open/Closed
    if settings.ravios_shop == modinfo::settings::ravios_shop::RaviosShop::Open {
        flags.extend(vec![
            131, // Suppress Ravio's Gift
            233, // Ravio's Shop fully opened
            239, // Ravio Sign Trigger
        ]);
    }

    // Enable opening Lorule Castle from start
    if settings.lc_requirement == 0 {
        flags.push(670);
    }

    // Night Mode
    if settings.night_mode {
        flags.push(964);
    }

    // Weather Vanes
    let wv_flags = match settings.weather_vanes {
        Standard => Flag::get_standard_weather_vane_flags(settings.cracksanity != Cracksanity::Off),
        Shuffled => None,
        Convenient => Flag::get_convenient_weather_vane_flags(settings.cracksanity != Cracksanity::Off),
        Hyrule => Flag::get_hyrule_weather_vane_flags(),
        Lorule => Flag::get_lorule_weather_vane_flags(),
        All => Flag::get_all_weather_vane_flags(),
    };
    wv_flags.iter().flatten().for_each(|flag| flags.push(flag.get_value()));

    // Swordless Mode - Tear down Barrier at game start
    if settings.swordless_mode {
        flags.push(410);
    }

    // Trial's Door
    match settings.trials_door {
        TrialsDoor::OpenFromInsideOnly => {
            // Set flags to auto-complete 3 of the 4 trials, defeat minibosses, and advance LC music.
            //
            // Flags 712 and 713 (lower right square) are intentionally not set so that they can
            // instead be set by the player when they reach Lorule Castle (the randomizer adds an
            // invisible trigger to set 712 so the trials will still be effectively skipped).
            flags.extend(vec![710, 711, /*712, 713,*/ 714, 715, 716, 717]);
        },
        TrialsDoor::OpenFromBothSides => {
            // Set flags to auto-complete the trials, open the trials door, defeat minibosses, and advance LC music.
            //
            // Flag 713 is intentionally not set so that when the door is first encountered it will fill in the bottom
            // right square and open the door. The scene will play in either the dungeon or Hilda's Study, whichever
            // the player encounters first.
            //
            // Do NOT try to shortcut this and just remove the door. We *NEED* that cutscene to play in Hilda's Study
            // situationally to draw attention to the fact that there's a path there.
            flags.extend(vec![710, 711, 712, /*713,*/ 714, 715, 716, 717]);
        },
        TrialsDoor::OneTrialRequired | TrialsDoor::TwoTrialsRequired | TrialsDoor::ThreeTrialsRequired => {
            if !trials_config.hook_trial {
                flags.extend(vec![710, 711]);
            }
            if !trials_config.tile_trial {
                flags.extend(vec![712, 713]);
            }
            if !trials_config.lamp_trial {
                flags.extend(vec![714, 715]);
            }
            if !trials_config.bomb_trial {
                flags.extend(vec![716, 717]);
            }
        },
        TrialsDoor::AllTrialsRequired => {
            // vanilla behavior
        },
    }

    // Big Bomb Flower Skip
    // Removes the Big Rock (FieldDark33) to drain the water (CaveDark1)
    if settings.skip_big_bomb_flower {
        flags.push(541);
    }

    flags.sort(); // Not needed just makes debugging easier...

    flags
}

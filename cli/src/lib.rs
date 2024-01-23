use log::info;
use modinfo::settings::keysy::Keysy;
use modinfo::settings::portal_shuffle::PortalShuffle;
use modinfo::settings::ravios_shop::RaviosShop;
use modinfo::settings::weather_vanes::WeatherVanes;
use modinfo::settings::{logic::LogicMode, pedestal::PedestalSetting, Settings};
use std::{
    io::{stdin, stdout, Read, Write},
    str::FromStr,
};

/// Pauses program execution
pub fn pause() {
    println!("\nPress Enter to continue...");
    stdin().read_exact(&mut [0]).unwrap();
}

/// Prompt the user for Seed Settings on the CLI
pub fn get_seed_settings() -> Result<Settings, String> {
    info!("No preset has been specified. Seed Settings UI will be used instead.\n");
    println!("\n--- Seed Settings ---");

    let logic_mode = prompt_logic_mode();

    let dungeon_prize_shuffle = prompt_bool(
        "Randomize Dungeon Prizes",
        "This shuffles all Sage Portraits, Pendants, and the Charm among themselves.",
    );

    let lc_requirement = prompt_u8_in_range(
        "Lorule Castle Requirement",
        "Choose how many Portraits are needed to enter Lorule Castle and fight Yuganon:",
        0,
        7,
    );
    //let yuganon_requirement = prompt_u8_in_range("Choose how many Portraits are needed to fight Yuganon:", 0, 7);

    let ped_requirement = PedestalSetting::try_from(prompt_u8_in_range(
        "Pedestal Requirement",
        "Choose which Pendants are required to reach the Master Sword Pedestal:\n\
        [2] Vanilla  - Requires only the Pendants of Power and Wisdom.\n\
        [3] Standard - Requires the Pendants of Power, Wisdom, and Courage.",
        2,
        3,
    ))?;

    let nice_mode = prompt_bool(
        "Shuffle Nice Items",
        "This shuffles a second progressive copy of each Ravio Item into the general item pool.",
    );
    let super_mode = prompt_bool(
        "Shuffle Super Items",
        "This shuffles a second progressive copy of the Lamp and Net into the general item pool.",
    );

    let reverse_sage_events = prompt_bool(
        "Reverse Sage Events",
        "Ties Sage-related checks and events to actually rescuing that Sage.\n\
        Makes the following changes:\n\
        - Irene => Unlocks the Irene check (instead of Pendant of Courage)\n\
        - Rosso => Unlocks Rosso's House and his two checks (instead of Pendant of Courage)\n\
        - Oren  => Unlocks the Smooth Gem check and the Shady Guy Event\n\
        - Impa  => Unlocks the front door to Hyrule Castle",
    );

    let no_progression_enemies = prompt_bool(
        "No Progression Enemies",
        "Removes Enemies from dungeons that are themselves Progression (e.g.: Bawbs, the bomb enemy).\n\
        Logic will be adjusted to require the player's items instead.",
    );

    let start_with_merge =
        prompt_bool("Start with Merge", "Start with the ability to Merge into walls, without Ravio's Bracelet.");

    let start_with_pouch = prompt_bool("Start with Pouch", "Start with the Pouch and a usable X Button.");

    let bell_in_shop = prompt_bool("Bell in Shop", "If enabled the Bell will be placed in Ravio's Shop.");

    let sword_in_shop = prompt_bool(
        "Sword in Shop",
        "If enabled at least one Sword will be placed in Ravio's Shop.\n\
        Note: This option is incompatible with Swordless Mode, which removes all Swords from the game.",
    );

    let boots_in_shop = prompt_bool("Boots in Shop", "If enabled the Pegasus Boots will be placed in Ravio's Shop.");

    let assured_weapon = if !&sword_in_shop && !&boots_in_shop {
        prompt_bool(
            "Assured Weapon in Shop",
            "If enabled at least one weapon is guaranteed to be placed in Ravio's Shop.",
        )
    } else {
        false
    };

    let maiamai_madness =
        prompt_bool("Maiamai Madness", "This shuffles Maiamai into the pool, adding 100 more locations.");

    let portal_shuffle = PortalShuffle::try_from(prompt_u8_in_range(
        "Portal Shuffle",
        "Choose how to shuffle Portals:\n\
        [0] Off                        - Portals are not shuffled.\n\
        [1] Cross World Pairs          - Portals are shuffled, but remain in Hyrule/Lorule pairs.\n\
        [2] Any World Pairs            - Portals are shuffled freely, and can lead to the same or opposite world.\n\
        [3] Mirrored Cross World Pairs - Same as Cross World Pairs, but each pair's vanilla counterparts will be in a matching pair.\n\
        [4] Mirrored Any World Pairs   - Same as Any World Pairs, but each pair's vanilla counterparts will be in a matching pair.",
        0,
        4,
    ))?;

    let minigames_excluded = prompt_bool(
        "Exclude Minigames",
        "Excludes the following: Octoball Derby, Dodge the Cuccos, Hyrule Hotfoot, Treacherous Tower, and both Rupee Rushes",
    );

    let skip_big_bomb_flower = prompt_bool(
        "Skip Big Bomb Flower",
        "Skips the Big Bomb Flower by removing the 5 Big Rocks in Lorule Field.\n\
        (Does not affect Lorule Castle Bomb Trial)",
    );

    let skip_trials = prompt_bool("Skip Trials", "Automatically opens the Lorule Castle Trials door.");

    let progressive_bow_of_light = prompt_bool(
        "Progressive Bow of Light",
        "Replaces the Bow of Light with a third copy of the Bow. Obtaining all 3 Bows will reward the Bow of Light.\n\
        Note 1: There will *NOT* be a Bow of Light Hint in Hilda's Study if this is enabled.\n\
        Note 2: This option is incompatible with the option to force Bow of Light in Lorule Castle.",
    );

    let bow_of_light_in_castle = prompt_bool(
        "Bow of Light in Castle",
        "Limits the Bow of Light's placement to somewhere in Lorule Castle (including possibly Zelda).",
    );

    let weather_vanes = WeatherVanes::try_from(prompt_u8_in_range(
        "Weather Vanes",
        "Choose Weather Vanes behavior. Logic may require using them to progress.\n\
        [0] Standard   - Only the standard complimentary Weather Vanes (Link's House & Vacant House)\n\
        [1] Shuffled   - Weather Vane destinations are shuffled into random pairs\n\
        [2] Convenient - Only convenient Weather Vanes that don't affect logic\n\
        [3] Hyrule     - Only the  9 Hyrule Weather Vanes (and Vacant House)\n\
        [4] Lorule     - Only the 13 Lorule Weather Vanes (and Link's House)\n\
        [5] All        - All 22 Weather Vanes\n",
        0,
        5,
    ))?;

    let ravios_shop = RaviosShop::try_from(prompt_u8_in_range(
        "Ravio's Shop",
        "Choose whether Ravio's Shop is Closed or Open at the start of the game.\n\
        [0] Closed\n\
        [1] Open\n",
        0,
        1,
    ))?;

    let dark_rooms_lampless = prompt_bool(
        "Dark Room Crossing (advanced)",
        "If enabled the logic may expect players to cross Dark Rooms without the Lamp.\n\
        Not for beginners and those who like being able to see things.",
    );

    let swordless_mode = if !&sword_in_shop {
        prompt_bool(
            "Swordless Mode (advanced)",
            "Removes *ALL* Swords from the game.\n\
        The Bug Net becomes a required item to play Dead Man's Volley against Yuga Ganon.",
        )
    } else {
        false
    };

    let chest_size_matches_contents = prompt_bool(
        "Chest Size Matches Contents",
        "All chests containing progression or unique items will become large, and others will be made small.\n\
        Note: Some large chests will have a reduced hitbox to prevent negative gameplay interference.",
    );

    let hint_ghost_price = prompt_u16_in_range(
        "Hint Ghost Price",
        "Set the price of Hints from a Hint Ghost:\nRecommended Price: 30",
        0,
        9999,
    );

    if hint_ghost_price == 69 {
        print!("\nNice.\n");
    }

    let keysy = Keysy::try_from(prompt_u8_in_range(
        "Keysy",
        "This setting removes locked keys and doors from dungeons if enabled.\n\
        [0] Off         - Key doors remain as they are in vanilla.\n\
        [1] Small Keysy - Small Keys and locked doors are removed from all dungeons.\n\
        [2] Big Keysy   - Big Keys and huge doors are removed from all dungeons.\n\
        [3] All Keysy   - All Keys and their doors are removed from all dungeons.\n",
        0,
        3,
    ))?;

    println!();
    info!("Starting seed generation...\n");

    Ok(Settings {
        dev_mode: false,
        exclusions: Default::default(),
        lc_requirement,
        yuganon_requirement: lc_requirement,
        ped_requirement,
        logic_mode,
        reverse_sage_events,
        dark_rooms_lampless,
        dungeon_prize_shuffle,
        maiamai_madness,
        nice_mode,
        super_mode,
        portal_shuffle,
        weather_vanes,
        ravios_shop,
        bow_of_light_in_castle,
        no_progression_enemies,
        keysy,
        progressive_bow_of_light,
        swordless_mode,
        start_with_merge,
        start_with_pouch,
        bell_in_shop,
        sword_in_shop,
        boots_in_shop,
        assured_weapon,
        chest_size_matches_contents,
        minigames_excluded,
        skip_big_bomb_flower,
        skip_trials,
        hint_ghost_price,
        night_mode: false,
    })
}

#[rustfmt::skip]
pub fn prompt_logic_mode() -> LogicMode {
    println!("\n[Logic Mode]");
    println!("[1] Normal        - Standard gameplay, no tricky item use or glitches. If unsure, choose this.");
    println!("[2] Hard          - Adds tricks that aren't technically glitches. Lamp + Net considered as weapons. No glitches.");
    println!("[3] Glitched      - Includes the above plus a selection of easy-to-learn glitches.");
    println!("[4] Adv. Glitched - Includes the above plus \"advanced\" glitches that may be a challenge to master.");
    println!("[5] Hell          - Includes every known RTA-viable glitch, including the insane ones. Don't choose this.");
    println!("[6] No Logic      - Items are placed with no logic at all. Seeds are likely to not be completable.");

    loop {
        print!("\nEnter a number (1-6): ");

        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        return match input.as_str() {
            "1" => LogicMode::Normal,
            "2" => LogicMode::Hard,
            "3" => LogicMode::Glitched,
            "4" => LogicMode::AdvGlitched,
            "5" => LogicMode::Hell,
            "6" => LogicMode::NoLogic,
            _ => {
                eprintln!("\nPlease enter 1, 2, 3, 4, 5, or 6");
                continue;
            }
        };
    }
}

pub fn prompt_u8_in_range(title: &str, description: &str, range_start: u8, range_end: u8) -> u8 {
    print!("\n[{}]\n{}", title, description);
    loop {
        print!("\nEnter a number ({}-{}): ", range_start, range_end);

        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match u8::from_str(input.trim()) {
            Err(_) => {},
            Ok(result) => {
                if (range_start..=range_end).contains(&result) {
                    return result;
                }
            },
        }

        eprintln!("Invalid input.");
    }
}

pub fn prompt_u16_in_range(title: &str, description: &str, range_start: u16, range_end: u16) -> u16 {
    print!("\n[{}]\n{}", title, description);
    loop {
        print!("\nEnter a number ({}-{}): ", range_start, range_end);

        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match u16::from_str(input.trim()) {
            Err(_) => {},
            Ok(result) => {
                if (range_start..=range_end).contains(&result) {
                    return result;
                }
            },
        }

        eprintln!("Invalid input.");
    }
}

pub fn prompt_bool(title: &str, description: &str) -> bool {
    loop {
        print!("\n[{}]\n{}\nEnable? (y/n): ", title, description);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if "y".eq_ignore_ascii_case(&input) {
            break true;
        } else if "n".eq_ignore_ascii_case(&input) {
            break false;
        } else {
            eprintln!("\nPlease enter either 'y' or 'n'");
        }
    }
}

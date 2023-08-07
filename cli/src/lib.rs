use std::{
    io::{stdin, stdout, Read, Write},
    str::FromStr,
};

use log::info;
use modinfo::{
    hyrule_castle_setting::HyruleCastleSetting, logic::Logic, logic_mode::LogicMode,
    pedestal_setting::PedestalSetting, Options, Settings,
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

    let randomize_dungeon_prizes = prompt_bool(
        "Randomize Dungeon Prizes",
        "This shuffles all Sage Portraits, Pendants, and the Charm among themselves.",
    );

    let vanilla_charm = if randomize_dungeon_prizes {
        prompt_bool(
            "Vanilla Charm",
            "Enabling this forces one of the two Pendant of Courage Upgrades to be in Zelda's Throne Room.\n\
            Otherwise, a random Sage Portrait or Pendant will be placed in Zelda's Throne Room.",
        )
    } else {
        false
    };

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
        [2] Vanilla  - Only the Pendants of Power and Wisdom are required\n\
        [3] Charmed  - All three Pendants are required, but Charm may substitute for the Pendant of Courage\n\
        [4] Standard - All Pendants are required\n",
        2,
        4,
    ))?;

    let hyrule_castle_setting = HyruleCastleSetting::try_from(prompt_u8_in_range(
        "Hyrule Castle Setting",
        "Choose how the Dungeon portion of Hyrule Castle should be handled:\n\
        [1] Early Lorule Castle - Completing Hyrule Castle allows early access to Lorule Castle via the Trial's Door.\n\
        [2] Closed              - The Dungeon is closed off completely, and removed from all logic.\n",
        1,
        2,
    ))?;

    let nice_mode = prompt_bool(
        "Shuffle Nice Items",
        "This shuffles a second progressive copy of each Ravio Item into the general item pool.",
    );
    let super_items = prompt_bool(
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

    let start_with_merge = prompt_bool(
        "Start with Merge",
        "Start with the ability to Merge into walls, without Ravio's Bracelet.",
    );

    let bell_in_shop =
        prompt_bool("Bell in Shop", "If enabled the Bell will be placed in Ravio's Shop.");

    let pouch_in_shop =
        prompt_bool("Pouch in Shop", "If enabled the Pouch will be placed in Ravio's Shop.");

    let sword_in_shop = prompt_bool(
        "Sword in Shop",
        "If enabled at least one Sword will be placed in Ravio's Shop.\n\
        Note: This option is incompatible with Swordless Mode, which removes all Swords from the game.",
    );

    let boots_in_shop = prompt_bool(
        "Boots in Shop",
        "If enabled the Pegasus Boots will be placed in Ravio's Shop.",
    );

    let assured_weapon = if !sword_in_shop && !boots_in_shop {
        prompt_bool(
            "Assured Weapon in Shop",
            "If enabled at least one weapon is guaranteed to be placed in Ravio's Shop.",
        )
    } else {
        false
    };

    let maiamai_madness = prompt_bool(
        "Maiamai Madness",
        "This shuffles Maiamai into the pool, adding 100 more locations.",
    );

    let minigames_excluded = prompt_bool(
        "Exclude Minigames",
        "Excludes the following: Octoball Derby, Dodge the Cuccos, Hyrule Hotfoot, Treacherous Tower, and both Rupee Rushes",
    );

    let skip_big_bomb_flower = prompt_bool(
        "Skip Big Bomb Flower",
        "Skips the Big Bomb Flower by removing the 5 Big Rocks in Lorule Field.\n\
        (Does not affect Lorule Castle Bomb Trial)",
    );

    let skip_trials =
        prompt_bool("Skip Trials", "Automatically opens the Lorule Castle Trials door.");

    let bow_of_light_in_castle = prompt_bool(
        "Bow of Light in Castle",
        "Limits the Bow of Light's placement to somewhere in Lorule Castle (including possibly Zelda).",
    );

    let weather_vanes_activated = prompt_bool(
        "Pre-Activated Weather Vanes",
        "Begin the game with all Weather Vanes activated.\n\
        The logic may expect players to use the Bell to reach otherwise unreachable locations this way.\n\
        Note: Trackers do not currently support this feature.",
    );

    let dark_rooms_lampless = prompt_bool(
        "Dark Room Crossing (advanced)",
        "If enabled the logic may expect players to cross Dark Rooms without the Lamp.\n\
        Not for beginners and those who like being able to see things.",
    );

    let swordless_mode = if !sword_in_shop {
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

    println!();
    info!("Starting seed generation...\n");

    Ok(Settings {
        logic: Logic {
            logic_mode,
            randomize_dungeon_prizes,
            vanilla_charm,
            lc_requirement,
            yuganon_requirement: lc_requirement,
            ped_requirement,
            hyrule_castle_setting,
            nice_mode,
            super_items,
            reverse_sage_events,
            no_progression_enemies,
            start_with_merge,
            bell_in_shop,
            pouch_in_shop,
            sword_in_shop,
            boots_in_shop,
            assured_weapon,
            maiamai_madness,
            weather_vanes_activated,
            minigames_excluded,
            skip_big_bomb_flower,
            skip_trials,
            bow_of_light_in_castle,
            dark_rooms_lampless,
            swordless_mode,
            hint_ghost_price,
            ..Default::default()
        },
        options: Options { chest_size_matches_contents, ..Default::default() },
        ..Default::default()
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
            Err(_) => {}
            Ok(result) => {
                if (range_start..=range_end).contains(&result) {
                    return result;
                }
            }
        }

        eprintln!("Invalid input.");
    }
}

pub fn prompt_u16_in_range(
    title: &str, description: &str, range_start: u16, range_end: u16,
) -> u16 {
    print!("\n[{}]\n{}", title, description);
    loop {
        print!("\nEnter a number ({}-{}): ", range_start, range_end);

        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match u16::from_str(input.trim()) {
            Err(_) => {}
            Ok(result) => {
                if (range_start..=range_end).contains(&result) {
                    return result;
                }
            }
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

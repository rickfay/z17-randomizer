use {
    crate::{
        settings::{
            logic::Logic, logic_mode::LogicMode, pedestal_setting::PedestalSetting,
            settings::Options,
        },
        system, Settings,
    },
    log::info,
    std::{
        io::{stdin, stdout, Write},
        str::FromStr,
    },
};

pub fn seed_settings_ui() -> Settings {
    info!("No preset has been specified. Seed Settings UI will be used instead.\n");
    println!("\n--- Seed Settings ---");

    let mode = prompt_logic_mode();
    let randomize_dungeon_prizes = prompt_bool(
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

    let ped_requirement = PedestalSetting::from(prompt_u8_in_range(
        "Pedestal Requirement",
        "Choose which Pendants are required to reach the Master Sword Pedestal:\n\
        [2] Vanilla  - Only the Pendants of Power and Wisdom are required\n\
        [3] Charmed  - All three Pendants are required, but Charm may substitute for the Pendant of Courage\n\
        [4] Standard - All Pendants are required\n",
        2,
        4,
    ));

    let nice_mode = prompt_bool(
        "Shuffle Nice Items",
        "This shuffles a second progressive copy of each Ravio Item into the general item pool.",
    );
    let super_items = prompt_bool(
        "Shuffle Super Items",
        "This shuffles a second progressive copy of the Lamp and Net into the general item pool.",
    );

    //let start_with_bracelet = prompt_until_bool("Start with Ravio's Bracelet?");
    let assured_weapon = prompt_bool(
        "Guaranteed Weapon",
        "If enabled a weapon is guarantee to be placed in Ravio's Shop.",
    );
    let bell_in_shop =
        prompt_bool("Bell in Shop", "If enabled the Bell will be placed in Ravio's Shop.");

    let pouch_in_shop =
        prompt_bool("Pouch in Shop", "If enabled the Pouch will be placed in Ravio's Shop.");

    let boots_in_shop = prompt_bool(
        "Boots in Shop",
        "If enabled the Pegasus Boots will be placed in Ravio's Shop.",
    );
    let maiamai_madness = prompt_bool(
        "Maiamai Madness",
        "This shuffles Maiamai into the pool, adding 100 more locations.",
    );
    let minigames_excluded = prompt_bool(
        "Exclude Minigames",
        "Excludes the following: Octoball Derby, Cucco Ranch, Hyrule Hotfoot, Treacherous Tower, and both Rupee Rushes",
    );
    let skip_big_bomb_flower = prompt_bool(
        "Skip Big Bomb Flower",
        "Skips the Big Bomb Flower by removing the 5 Big Rocks in Lorule Field.\n\
        (Does not affect Lorule Castle Bomb Trial)",
    );
    let skip_trials =
        prompt_bool("Skip Trials", "Automatically opens the Lorule Castle Trials door.");

    let bow_of_light_in_castle = prompt_bool(
        "Tournament Bow of Light",
        "Forces the Bow of Light to be placed in Lorule Castle (including possibly Zelda).",
    );
    let vanes_activated = prompt_bool(
        "Pre-Activated Weather Vanes",
        "Begin the game with all Weather Vanes activated.\n\
        The logic may expect players to use the Bell to reach otherwise unreachable locations this way.\n\
        NOTE: Trackers do not currently support this feature.",
    );
    let lampless = prompt_bool(
        "Dark Room Crossing (advanced)",
        "If enabled the logic may expect players to cross Dark Rooms without the Lamp.\n\
        Not for beginners and those who like being able to see things.",
    );
    let swordless_mode = prompt_bool(
        "Swordless Mode (advanced)",
        "Removes all Swords from the item pool.\n\
        The Bug Net becomes a required item to play Dead Man's Volley against Yuga Ganon.",
    );
    let chest_size_matches_contents = prompt_bool(
        "Chest Size Matches Contents",
        "All chests containing progression or unique items will become large, and others will be made small.\n\
        Note: Some large chests will have altered collision to prevent negative gameplay interference.",
    );

    println!();
    info!("Starting seed generation...\n");

    Settings {
        logic: Logic {
            mode,
            randomize_dungeon_prizes,
            lc_requirement,
            yuganon_requirement: lc_requirement,
            ped_requirement,
            nice_mode,
            assured_weapon,
            bell_in_shop,
            pouch_in_shop,
            boots_in_shop,
            maiamai_madness,
            vanes_activated,
            super_items,
            minigames_excluded,
            skip_big_bomb_flower,
            skip_trials,
            bow_of_light_in_castle,
            lampless,
            swordless_mode,
            ..Default::default()
        },
        options: Options { chest_size_matches_contents, ..Default::default() },
        ..Default::default()
    }
}

#[rustfmt::skip]
fn prompt_logic_mode() -> LogicMode {
    print!("\n[Logic Mode]\n");
    print!("[1] Normal        - Standard gameplay, no tricky item use or glitches. If unsure, choose this.\n");
    print!("[2] Hard          - Adds tricks that aren't technically glitches. Lamp + Net considered as weapons. No glitches.\n");
    print!("[3] Glitched      - Includes the above plus a selection of easy-to-learn glitches.\n");
    print!("[4] Adv. Glitched - Includes the above plus \"advanced\" glitches that may be a challenge to master.\n");
    print!("[5] Hell          - Includes every known RTA-viable glitch, including the insane ones. Don't choose this.\n");
    print!("[6] No Logic      - Items are placed with no logic at all. Seeds are likely to not be completable.\n");

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

fn prompt_u8_in_range(title: &str, description: &str, range_start: u8, range_end: u8) -> u8 {
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

fn prompt_bool(title: &str, description: &str) -> bool {
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

pub fn prompt_until<F>(prompt: &str, until: F, error: &str) -> system::Result<String>
where
    F: Fn(&str) -> bool,
{
    loop {
        print!("{}: ", prompt);
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        if until(&input) {
            break Ok(input);
        } else {
            eprintln!("{}", error);
        }
    }
}

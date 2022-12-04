use std::{fs, panic};
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use log::{error, info};

use randomizer::{Seed, Settings, plando, filler_new};
use simplelog::{LevelFilter, SimpleLogger};
use structopt::StructOpt;
use albw::Game;
use randomizer::logic_mode::LogicMode;
use randomizer::settings::{Logic, Options};
use sys::Paths;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    seed: Option<Seed>,
    #[structopt(long)]
    preset: Option<String>,
    #[structopt(long)]
    no_patch: bool,
    #[structopt(long)]
    no_spoiler: bool,
}

fn prompt_logic_mode() -> LogicMode
{
    print!("\nChoose Logic Mode:\n");
    print!("[1] Normal              - Standard gameplay, no tricky item use or glitches. If unsure, choose this.\n");
    print!("[2] Hard                - Adds tricks that aren't technically glitches. Lamp + Net considered as weapons. No glitches.\n");
    print!("[3] Glitched (Basic)    - Includes the above plus \"basic\", easy-to-learn glitches.\n");
    print!("[4] Glitched (Advanced) - Includes the above plus \"advanced\" glitches that may be a challenge to master.\n");
    print!("[5] Glitched (Hell)     - Includes every known RTA-viable glitch, including the insane ones. DO NOT CHOOSE THIS.\n");
    print!("[6] No Logic            - Items are placed with no logic at all. Seeds may not be completable.\n");

    loop {
        print!("\nEnter a number (1-6): ");

        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        return match input.as_str() {
            "1" => LogicMode::Normal,
            "2" => LogicMode::Hard,
            "3" => LogicMode::GlitchBasic,
            "4" => LogicMode::GlitchAdvanced,
            "5" => LogicMode::GlitchHell,
            "6" => LogicMode::NoLogic,
            _ => {
                eprintln!("\nPlease enter 1, 2, 3, 4, 5, or 6");
                continue;
            }
        }
    }
}

fn prompt_until_bool(prompt: &str) -> bool
{
    loop {
        print!("\n{}\nEnter (y/n): ", prompt);
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

fn prompt_until<F>(prompt: &str, until: F, error: &str) -> sys::Result<String>
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

fn create_paths() -> sys::Result<Paths> {
    let rom = prompt_until(
        "Path to ROM",
        |rom| Game::load(&rom).is_ok(),
        "The provided path does not point to a valid ROM.",
    )?;
    let output = prompt_until(
        "Output directory",
        |output| Path::new(output).exists() || fs::create_dir_all(&output).is_ok(),
        "The provided path could not be created.",
    )?;

    Ok(Paths::new(rom.into(), output.into()))
}

fn preset_ui() -> Settings {

    info!("No preset has been specified. Seed Options UI will be used instead.\n");
    println!("--- Seed Options ---");

    let mode = prompt_logic_mode();
    let randomize_dungeon_prizes = prompt_until_bool("Randomize Dungeon Prizes?");
    //let start_with_bracelet = prompt_until_bool("Start with Ravio's Bracelet?");
    let assured_weapon = prompt_until_bool("Guarantee a Weapon is placed in Ravio's Shop?");
    let bell_in_shop = prompt_until_bool("Guarantee Bell in Ravio's Shop?");
    let pouch_in_shop = prompt_until_bool("Guarantee Pouch in Ravio's Shop?");
    let boots_in_shop = prompt_until_bool("Guarantee Pegasus Boots in Ravio's Shop?");
    let maiamai_madness = prompt_until_bool("Enable Maiamai Madness? This shuffles Maiamai into the pool, adding 100 more locations.");
    let super_items = prompt_until_bool("Include the Super Lamp and Super Net?");
    let minigames_excluded = prompt_until_bool("Exclude all minigames?");
    let skip_trials = prompt_until_bool("Skip the Lorule Castle Trials?");
    let bow_of_light_in_castle = prompt_until_bool("Guarantee Bow of Light in Lorule Castle?");
    let lampless = prompt_until_bool("Don't require Lamp? (advanced)\nIf \"y\", the player may have to cross dark rooms without a light source.\nIf you're not sure, select \"n\".");
    let vanes_activated = prompt_until_bool("Pre-activate all Weather Vanes? (EXPERIMENTAL)\nThis may cause Ravio's Bracelet(s) to be placed in any region of Lorule.");
    let swordless_mode = prompt_until_bool("Play in Swordless Mode? (advanced)");
    let chest_size_matches_contents = prompt_until_bool("Make chest sizes match their contents?");

    println!();
    info!("Starting seed generation...\n");

    Settings {
        logic: Logic {
            mode,
            randomize_dungeon_prizes,
            assured_weapon,
            bell_in_shop,
            pouch_in_shop,
            boots_in_shop,
            maiamai_madness,
            vanes_activated,
            super_items,
            minigames_excluded,
            skip_trials,
            bow_of_light_in_castle,
            lampless,
            swordless_mode,
            ..Default::default()
        },
        options: Options {
            chest_size_matches_contents,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() -> randomizer::Result<()> {
    let opt = Opt::from_args();

    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("Could not initialize logger.");

    info!("Initializing ALBW Randomizer...");

    let is_plando = false; // TODO expose this eventually so people can make their own, for now it's for testing

    if is_plando {
        plando()
    } else {
        let system = randomizer::system()?;

        let preset = if let Some(ref preset) = opt.preset {
            system.preset(&preset)?
        } else {
            preset_ui()
        };

        const MAX_RETRIES: u16 = 100;
        let mut result = Ok(());

        for x in 0..MAX_RETRIES {
            let seed = opt.seed.unwrap_or_else(rand::random);


            info!("Attempt:                        #{}", x + 1);
            info!("Preset:                         {}", opt.preset.as_ref().unwrap_or(&String::from("<None>")));
            info!("Version:                        0.3.0 - Dev Build #1");

            //let randomizer = Generator::new(&preset, seed);
            let spoiler = panic::catch_unwind(|| filler_new(&preset, seed));

            if spoiler.is_ok() {
                println!();
                info!("Seed generated. Patching...");
                result = spoiler.unwrap().patch(
                    system.get_or_create_paths(create_paths)?,
                    !opt.no_patch,
                    !opt.no_spoiler,
                );

                break;
            } else if x >= MAX_RETRIES - 1 {
                // FIXME I hate this, but I'm struggling with Rust error handling so leaving it for now
                panic!("Too many retry attempts have failed. Aborting...");
            } else {
                info!("Seed was not completable (this is normal). Retrying...\n");
            }
        }

        match result {
            Ok(_) => info!("Successfully generated ALBWR seed :D"),
            Err(_) => error!("Failed to generate ALBWR seed D:"),
        }

        pause();

        result
    }
}

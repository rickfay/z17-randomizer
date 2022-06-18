use std::{fs, panic};
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::process::exit;
use log::{error, info};

use randomizer::{Seed, Generator, Settings, plando, filler_new};
use simplelog::{LevelFilter, SimpleLogger};
use structopt::StructOpt;
use albw::Game;
use randomizer::settings::Logic;
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

fn prompt_until<F>(prompt: &str, until: F, error: &str) -> ::sys::Result<String>
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

fn create_paths() -> ::sys::Result<Paths> {
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

    let start_with_bracelet = prompt_until_bool("Start with Ravio's Bracelet?");
    let bell_in_shop = prompt_until_bool("Place Bell in Ravio's Shop?");
    let pouch_in_shop = prompt_until_bool("Place Pouch in Ravio's Shop?");
    let boots_in_shop = prompt_until_bool("Place Pegasus Boots in Ravio's Shop?");
    let super_items = prompt_until_bool("Include the Super Lamp and Super Net?");
    let minigames_excluded = prompt_until_bool("Exclude all minigames?");
    let skip_trials = prompt_until_bool("Skip the Lorule Castle Trials?");
    let glitched_logic = prompt_until_bool("Use Glitched Logic? (advanced)");
    let swordless_mode = prompt_until_bool("Play in Swordless Mode? (advanced)");

    println!();
    info!("Starting seed generation...\n");

    Settings {
        logic: Logic {
            bell_in_shop,
            pouch_in_shop,
            boots_in_shop,
            super_items,
            glitched_logic,
            start_with_bracelet,
            minigames_excluded,
            swordless_mode,
            skip_trials,
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

    info!("Initializing Z17 Randomizer...");

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

        let use_new_filler = true;
        let max_retries = 100;
        let mut result = Ok(());

        for x in 0..max_retries {
            let seed = opt.seed.unwrap_or_else(rand::random);

            if use_new_filler {
                filler_new(&preset, seed as u64)?;
                pause();
                exit(0);
            } else {
                info!("Attempt:                        #{}", x + 1);
                info!("Preset:                         {}", opt.preset.as_ref().unwrap_or(&String::from("<None>")));
                info!("Version:                        0.0.4");

                let randomizer = Generator::new(&preset, seed);
                let spoiler = panic::catch_unwind(|| randomizer.randomize());

                if spoiler.is_ok() {
                    println!();
                    info!("Seed generated. Patching...");
                    result = spoiler.unwrap().patch(
                        system.get_or_create_paths(create_paths)?,
                        !opt.no_patch,
                        !opt.no_spoiler,
                    );

                    break;
                } else if x >= max_retries - 1 {
                    // FIXME I hate this, but I'm struggling with Rust error handling so leaving it for now
                    panic!("Too many retry attempts have failed. Aborting...");
                } else {
                    info!("Seed was not completable (this is normal). Retrying...\n");
                }
            }
        }

        match result {
            Ok(_) => info!("Successfully generated Z17R seed :D"),
            Err(_) => error!("Failed to generate Z17R seed D:"),
        }

        pause();

        result
    }
}

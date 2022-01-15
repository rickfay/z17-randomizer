use std::{fs, panic};
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use log::{error, info};

use randomizer::{Seed, Generator, Error, plando, Settings};
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

fn prompt_until_bool(prompt: &str) -> ::sys::Result<bool>
{
    loop {
        print!("{}: ", prompt);
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if "y".eq_ignore_ascii_case(&input) {
            break Ok(true);
        } else if "n".eq_ignore_ascii_case(&input) {
            break Ok(false);
        } else {
            eprintln!("Please enter either 'y' or 'n'");
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

    info!("No preset has been specified. Options UI will be used instead.");
    println!("\nSeed Options\nFor the following questions please answer either 'y' for yes or 'n' for no:");

    let start_with_bracelet = prompt_until_bool(
        "Start with Ravio's Bracelet?"
    ).unwrap();

    let bell_in_shop = prompt_until_bool(
        "Place Bell in Ravio's Shop?"
    ).unwrap();

    let pouch_in_shop = prompt_until_bool(
        "Place Pouch in Ravio's Shop?"
    ).unwrap();

    let glitched_logic = prompt_until_bool(
        "Use Glitched Logic?"
    ).unwrap();

    let dont_require_lamp_for_darkness = prompt_until_bool(
        "Don't require Lamp for dark rooms?"
    ).unwrap();

    let minigames_excluded = prompt_until_bool(
        "Exclude all minigames?"
    ).unwrap();

    println!("Starting seed generation...\n");

    Settings {
        logic: Logic {
            dont_require_lamp_for_darkness,
            bell_in_shop,
            pouch_in_shop,
            //unsafe_key_placement,
            glitched_logic,
            start_with_bracelet,
            minigames_excluded,
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

    let system = randomizer::system()?;

    let preset = if let Some(ref preset) = opt.preset {
        system.preset(&preset)?
    } else {
        preset_ui()
        //Default::default()
    };

    // plando();
    // Ok(())

    let max_retries = 50;
    let mut result = Ok(());

    for x in 0..max_retries {
        let seed = opt.seed.unwrap_or_else(rand::random);

        info!("Attempt #{}", x + 1);
        info!("Preset: {}", opt.preset.as_ref().unwrap_or(&String::from("<None>")));

        let randomizer = Generator::new(&preset, seed);
        let spoiler = panic::catch_unwind(|| randomizer.randomize());

        if spoiler.is_ok() {
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
            info!("Failed to generate completable seed, retrying...\n");
        }
    }

    match result {
        Ok(_) => info!("Successfully generated Z17R seed :D"),
        Err(_) => error!("Failed to generate Z17R seed D:"),
    }

    pause();

    result
}

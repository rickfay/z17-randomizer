use std::{fs, panic};
use std::io::{stdin, stdout, Write};
use std::path::Path;

use randomizer::{Seed, plando, Generator};
use simplelog::{LevelFilter, SimpleLogger};
use structopt::StructOpt;
use albw::Game;
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
    #[structopt(short, long)]
    verbose: bool,
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


fn main() -> randomizer::Result<()> {
    let opt = Opt::from_args();
    let filter = if opt.verbose {
        LOG_FILTER
    } else {
        LevelFilter::Off
    };
    panic::catch_unwind(|| {
        SimpleLogger::init(filter, Default::default()).expect("Could not initialize logger.");
        let system = randomizer::system()?;
        let preset = if let Some(preset) = opt.preset {
            system.preset(&preset)?
        } else {
            Default::default()
        };
        let seed = opt.seed.unwrap_or_else(rand::random);

        Generator::new(&preset, seed).randomize().patch(
            system.get_or_create_paths(create_paths)?,
            !opt.no_patch,
            !opt.no_spoiler,
        )
    })
        .expect("A fatal error occurred. Please report this bug to the developers.")
}

// fn main() -> () {
//     panic::catch_unwind(|| {
//         SimpleLogger::init(LevelFilter::Info, Default::default()).expect("Could not initialize logger.");
//         ..plando();
//     }).expect("Oh no it poo'd itself")
// }

#[cfg(debug_assertions)]
const LOG_FILTER: LevelFilter = LevelFilter::Debug;

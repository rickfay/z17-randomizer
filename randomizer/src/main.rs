use {
    log::{error, info},
    randomizer::{cli::seed_settings_ui, constants::VERSION, fail, filler_new, pause, Seed},
    simplelog::{LevelFilter, SimpleLogger},
    std::panic::catch_unwind,
    structopt::StructOpt,
};

/**
 * THE LEGEND OF ZELDA: A LINK BETWEEN WORLDS RANDOMIZER
 */
fn main() -> randomizer::Result<()> {
    let opt = Opt::from_args();

    SimpleLogger::init(LevelFilter::Info, Default::default())
        .expect("Could not initialize logger.");

    info!("Initializing ALBW Randomizer...");

    let system = randomizer::system()?;

    let mut preset = if let Some(ref preset) = opt.preset {
        system.preset(&preset)?
    } else {
        seed_settings_ui()
    };

    // FIXME Temporary: Force Yuganon Requirement to be equal to LC Requirement
    preset.logic.yuganon_requirement = preset.logic.lc_requirement;

    const MAX_RETRIES: u16 = 100;
    let mut result = Ok(());
    let mut seed = opt.seed.unwrap_or_else(rand::random);

    for x in 0..MAX_RETRIES {
        let no_preset = String::from("<None>");
        let preset_str = opt.preset.as_ref().unwrap_or(&no_preset);

        info!("Attempt:                        #{}", x + 1);
        info!("Preset:                         {}", preset_str);
        info!("Version:                        {}", VERSION);

        let spoiler = catch_unwind(|| filler_new(VERSION, &preset, seed));

        if spoiler.is_ok() {
            println!();
            info!("All seed information has been successfully generated.");
            println!();
            let path_config = system.load_config().unwrap_or_else(|error| {
                fail!("Failed to parse configuration file: config.json\n\
                Commonly Fixed By: Replace any single backslash characters '\\' with a forward slash '/' or double backslash '\\\\'.\n\
                Full Error: {}\n", error);
            });
            result = spoiler.unwrap().patch(path_config, !opt.no_patch, !opt.no_spoiler, true);

            break;
        } else if x >= MAX_RETRIES - 1 {
            fail!("Too many retry attempts have failed. Aborting...");
        } else {
            info!("Seed was not completable (this is normal). Retrying...\n");
            seed = opt.seed.unwrap_or_else(rand::random);
        }
    }

    println!();

    match result {
        Ok(_) => info!("Successfully generated ALBW Randomizer Seed: {}", seed),
        Err(_) => {
            println!();
            error!("An unknown error occurred while generating the seed D:\n");

            error!("If you're seeing this error, there is likely an issue with your ROM.");
            error!("Verify your ROM is (1) a North American copy of ALBW, and (2) decrypted.\n");

            info!(
                "For assistance, visit the #help-and-strats channel on the ALBW Randomizer Discord."
            );
        }
    }

    pause();

    result
}

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

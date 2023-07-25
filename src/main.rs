use {
    log::{error, info},
    randomizer::{
        constants::VERSION,
        system::{System, UserConfig},
    },
    settings::Settings,
    simplelog::{LevelFilter, SimpleLogger},
    structopt::StructOpt,
};

use albw_randomizer::fail;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    seed: Option<u32>,

    #[structopt(long)]
    preset: Option<String>,

    #[structopt(long)]
    no_patch: bool,

    #[structopt(long)]
    no_spoiler: bool,
}

/**
 * THE LEGEND OF ZELDA: A LINK BETWEEN WORLDS RANDOMIZER
 */
fn main() {
    let opt = Opt::from_args();

    SimpleLogger::init(LevelFilter::Info, Default::default())
        .expect("Could not initialize logger.");

    info!("Initializing ALBW Randomizer...");

    // Get Settings, either from a preset or the CLI
    let (preset_name, mut settings): (&str, Settings) =
        if let Some(preset_name) = opt.preset.as_ref() {
            (
                preset_name,
                System::load_preset(preset_name).unwrap_or_else(|_| {
                    fail!("Failed to load preset: {}", preset_name);
                }),
            )
        } else {
            (
                "<None>",
                cli::get_seed_settings().unwrap_or_else(|err| {
                    fail!("Failed to create Settings: {}", err);
                }),
            )
        };
    settings.logic.yuganon_requirement = settings.logic.lc_requirement; // FIXME Temporary: Force Yuganon Requirement to be equal to LC Requirement

    // Determine Seed
    let (seeded, mut seed): (bool, u32) =
        if let Some(seed) = opt.seed { (true, seed) } else { (false, rand::random()) };

    // Load User Config
    let user_config: UserConfig = System::load_config().unwrap_or_else(|error| {
        fail!("Failed to parse configuration file: config.json\n\
                Commonly Fixed By: Replace any single backslash characters '\\' with a forward slash '/' or double backslash '\\\\'.\n\
                Full Error: {}\n", error);
    });

    // Generate Seed in a retryable manner
    const MAX_RETRIES: u16 = 100;
    for x in 0..MAX_RETRIES {
        info!("Attempt:                        #{}", x + 1);
        info!("Preset:                         {}", preset_name);
        info!("Version:                        {}", VERSION);

        match randomizer::generate_seed(seed, &settings, &user_config, opt.no_patch, opt.no_spoiler)
        {
            Ok(_) => {
                println!();
                info!("Successfully Generated ALBWR Seed: {}", seed);
                break;
            }
            Err(err) => {
                error!("{:?}", err);
                if x < MAX_RETRIES {
                    if !seeded {
                        info!("Seed was not completable (this is normal). Retrying...\n");
                        seed = rand::random();
                    } else {
                        fail!("Couldn't generate Seed: \"{}\" with the given settings.", seed);
                    }
                } else {
                    fail!("Too many retry attempts have failed. Aborting...");
                }
            }
        }
    }

    println!();
    cli::pause();
}

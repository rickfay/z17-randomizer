use {
    log::{error, info},
    macros::fail,
    randomizer::{
        constants,
        system::{System, UserConfig},
    },
    seed::settings::Settings,
    simplelog::{LevelFilter, SimpleLogger},
    structopt::StructOpt,
};

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

/// THE LEGEND OF ZELDA: A LINK BETWEEN WORLDS RANDOMIZER
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
    let (seeded, mut seed_num): (bool, u32) =
        if let Some(seed) = opt.seed { (true, seed) } else { (false, rand::random()) };

    // Load User Config
    let user_config: UserConfig = System::load_config().unwrap_or_else(|error| {
        fail!("Failed to parse configuration file: {}\n\
                Commonly Fixed By: Replace any single backslash characters '\\' with a forward slash '/' or double backslash '\\\\'.\n\
                Full Error: {}\n", constants::CONFIG_FILE_NAME, error);
    });

    // Generate Seed in a retryable manner
    const MAX_RETRIES: u16 = 100;
    let generated_seed;
    for x in 0..MAX_RETRIES {
        info!("Attempt:                        #{}", x + 1);
        info!("Preset:                         {}", preset_name);
        info!("Version:                        {}", constants::VERSION);

        match randomizer::generate_seed(seed_num, &settings) {
            Ok(seed) => {
                println!();
                info!("Successfully Generated ALBWR Seed: {}", seed_num);
                generated_seed = seed;
                break;
            }
            Err(err) => {
                error!("{:?}", err);
                if x < MAX_RETRIES {
                    if !seeded {
                        info!("Seed was not completable (this is normal). Retrying...\n");
                        seed_num = rand::random();
                    } else {
                        fail!("Couldn't generate Seed: \"{}\" with the given settings.", seed_num);
                    }
                } else {
                    fail!("Too many retry attempts have failed. Aborting...");
                }
            }
        }
    }

    if !opt.no_patch {
        patcher::generate_patch(&generated_seed, user_config.output().into_path_buf())
    }

    if !opt.no_spoiler {
        // spoiler::generate_spoiler(&seed, user_config) // todo
    }

    println!();
    cli::pause();
}

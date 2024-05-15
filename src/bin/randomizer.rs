// use log::warn;
// use randomizer::SeedInfo;
// use {
//     log::{error, info},
//     macros::fail,
//     randomizer::{constants::VERSION, system::System},
//     simplelog::{LevelFilter, SimpleLogger},
//     structopt::StructOpt,
// };
//
// #[derive(Debug, StructOpt, Clone)]
// struct Opt {
//     #[structopt(long)]
//     seed: Option<u32>,
//
//     #[structopt(long)]
//     preset: Option<String>,
// }
//
// /// THE LEGEND OF ZELDA: A LINK BETWEEN WORLDS RANDOMIZER
// fn main() {
//     let opt = Opt::from_args();
//
//     SimpleLogger::init(LevelFilter::Info, Default::default()).expect("Could not initialize logger.");
//
//     info!("Initializing ALBW Randomizer...");
//
//     let (preset_name, seeded, SeedInfo { mut seed, mut settings, .. }) = determine_settings(opt.preset, opt.seed);
//
//     settings.yuganon_requirement = settings.lc_requirement; // FIXME Temporary: Force Yuganon Requirement to be equal to LC Requirement
//
//     // Load User Config
//     // let user_config: UserConfig = System::load_config().unwrap_or_else(|error| {
//     //     fail!("Failed to parse configuration file: config.json\n\
//     //             Commonly Fixed By: Replace any single backslash characters '\\' with a forward slash '/' or double backslash '\\\\'.\n\
//     //             Full Error: {}\n", error);
//     // });
//
//     // Generate Seed in a retryable manner
//     const MAX_RETRIES: u16 = 100;
//     for x in 0..MAX_RETRIES {
//         info!("Attempt:                        #{}", x + 1);
//         info!(
//             "Preset File:                    {}",
//             if let Some(preset_name) = preset_name.clone() { preset_name } else { "<None>".to_owned() }
//         );
//         info!("Version:                        {}", VERSION);
//         info!("Seed:                           {:0>10}", seed);
//
//         let stopwatch = std::time::Instant::now();
//         match randomizer::generate_seed(seed, settings.clone()) {
//             Ok(_) => {
//                 println!();
//                 info!("Successfully Generated ALBWR Seed {} in {} seconds! :D", seed, stopwatch.elapsed().as_secs());
//                 println!();
//                 info!("For help installing this seed: https://github.com/rickfay/z17-randomizer#setup");
//                 info!("List of known issues: https://github.com/rickfay/z17-randomizer#known-issues");
//                 info!("Visit us on Discord: https://discord.gg/dmAJh2uY7M");
//                 break;
//             },
//             Err(err) => {
//                 error!("{:?}", err);
//                 if x < MAX_RETRIES {
//                     if !seeded {
//                         info!("A retryable error was encountered.\n");
//                         seed = rand::random();
//                     } else {
//                         fail!("Couldn't generate Seed: \"{}\" with the given settings.", seed);
//                     }
//                 } else {
//                     fail!("Too many retry attempts have failed. Aborting...");
//                 }
//             },
//         }
//     }
//
//     println!();
//     cli::pause();
// }
//
// /// Get Settings, either from a preset or the CLI
// fn determine_settings(opt_preset: Option<String>, opt_seed: Option<u32>) -> (Option<String>, bool, SeedInfo) {
//     if let Some(preset_name) = opt_preset {
//         let preset_name = preset_name.clone();
//         let mut seed_info = System::load_preset(&(preset_name.clone())).unwrap_or_else(|err| {
//             fail!("Failed to load preset: {}\nError: {}", preset_name, err);
//         });
//
//         if seed_info.version != VERSION {
//             fail!(
//                 "There is a Version mismatch between the Randomizer and the Preset file.\n\
//                 Randomizer Version: \"{}\"\n\
//                 Preset Version:     \"{}\"",
//                 VERSION,
//                 seed_info.version
//             );
//         }
//
//         let mut seeded = false;
//         if let Some(seed) = opt_seed {
//             if seed_info.seed != 0 {
//                 println!();
//                 warn!("Two seed numbers provided! Defaulting to the command line argument.\n");
//             }
//
//             seed_info.seed = seed;
//             seeded = true;
//         } else if seed_info.seed != 0 {
//             seeded = true;
//         } else {
//             seed_info.seed = rand::random();
//         }
//
//         (Some(preset_name), seeded, seed_info)
//     } else {
//         let (seeded, seed): (bool, u32) =
//             if let Some(seed) = opt_seed { (true, seed) } else { (false, rand::random()) };
//
//         (
//             None,
//             seeded,
//             SeedInfo {
//                 seed,
//                 version: VERSION.to_owned(),
//                 settings: cli::get_seed_settings().unwrap_or_else(|err| {
//                     fail!("Failed to create Settings: {}", err);
//                 }),
//                 ..Default::default()
//             },
//         )
//     }
// }

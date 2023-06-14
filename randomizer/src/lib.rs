use {
    log::info,
    rand::{prelude::StdRng, SeedableRng},
    seed::{settings::Settings, SeedHash, SeedInfo},
    std::error,
};
use seed::Seed;

pub mod constants;
mod entrance_rando;
mod filler;
pub mod model;
pub mod system;
#[rustfmt::skip]
mod old_world;

/// Main entry point to generate one ALBWR Seed.
pub fn generate_seed(seed_num: u32, settings: &Settings) -> Result<Seed, Box<dyn error::Error>> {
    let rng = &mut StdRng::seed_from_u64(seed_num as u64);
    let hash = SeedHash::new(seed_num, settings);
    info!("Hash:                           {}\n", hash.get_text_hash());

    settings.log_settings();
    settings.validate()?;

    filler::generate_seed(seed_num, settings, hash, rng)
}

pub fn patch_seed(seed: &Seed) -> Result<(), Box<dyn error::Error>> {
    println!();

    // if !no_patch {
    //     info!("Starting Patch Process...");
    //
    //     let game = Game::load(user_config.rom())?;
    //     let mut patcher = Patcher::new(game)?;
    //
    //     info!("ROM Loaded.\n");
    //
    //     regions::patch(&mut patcher, &seed_info.layout, &seed_info.settings)?;
    //     let patches = patcher.prepare(seed_info)?;
    //     patches.dump(user_config.output())?;
    // }
    // if !no_spoiler {
    //     let path = user_config.output().join(format!("{:0>10}_spoiler.json", seed_info.seed));
    //     info!("Writing Spoiler Log to:         {}", &path.absolutize()?.display());
    //
    //     //let spoiler = Spoiler::from(seed_info);
    //
    //     let mut serialized = serde_json::to_string_pretty(&seed_info).unwrap();
    //     align_json_values(&mut serialized);
    //
    //     write!(File::create(path)?, "{}", serialized).expect("Could not write the spoiler log.");
    // }

    Ok(())
}

use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
};

use game::{
    world::LocationKey,
    Item::{self},
};
use log::{error, info};
use modd::{
    filler_item::{convert, FillerItem},
    hints::formatting::*,
    Layout, Mod, Settings,
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Serialize, Serializer};

use crate::{constants::VERSION, metrics::Metrics};

pub mod constants;
mod entrance_rando;
mod filler;
mod filler_util;
mod hints;
mod item_pools;
mod metrics;
pub mod model;
#[rustfmt::skip]
mod world;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
}

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

#[derive(Serialize)]
pub struct SeedInfo {
    pub seed: u32,
    pub version: &'static str,
    pub mod_: Mod,
    pub metrics: Metrics,
}

/// Main entry point to generate one ALBWR Seed.
pub fn generate_seed(seed: u32, settings: Settings) -> Result<SeedInfo> {
    validate_settings(&settings)?;

    let rng = &mut StdRng::seed_from_u64(seed as u64);
    let hash = SeedHash::new(seed, &settings);

    info!("Hash:                           {}\n", hash.text_hash);
    settings.log_settings();

    calculate_seed_info(seed, settings, hash, rng)
}

/// A hash used in-game to quickly verify that two players are playing the same seed.
///
/// The hash is calculated as `u64`, truncated to `u16` (5 digits), then converted to a Symbolic form that can be
/// displayed in-game as well as in the spoiler log.
pub struct SeedHash {
    item_hash: String,
    text_hash: String,
}

impl SeedHash {
    pub fn new(seed: u32, settings: &Settings) -> Self {
        // Calculate underlying Hash
        let mut hasher = DefaultHasher::new();
        (seed, settings, VERSION).hash(&mut hasher);
        let mut hash = hasher.finish() % 100_000;

        // Convert to Item Hash
        let hash_item_lut: Vec<(String, &str)> = vec![
            (a_button(), "(A)"),
            (b_button(), "(B)"),
            (x_button(), "(X)"),
            (y_button(), "(Y)"),
            (l_button(), "(L)"),
            (r_button(), "(R)"),
            (ravio(), "(Ravio)"),
            (bow(), "(Bow)"),
            (bombs(), "(Bombs)"),
            (fire_rod(), "(Fire Rod)"),
        ];

        const HASH_LEN: usize = 5;
        let mut digit = Vec::with_capacity(HASH_LEN);
        for _ in 0..HASH_LEN {
            digit.push(hash_item_lut.get((hash % 10) as usize).unwrap());
            hash /= 10;
        }

        let item_hash =
            format!("{} {} {} {} {}", digit[4].0, digit[3].0, digit[2].0, digit[1].0, digit[0].0);
        let text_hash =
            format!("{} {} {} {} {}", digit[4].1, digit[3].1, digit[2].1, digit[1].1, digit[0].1);

        Self { item_hash, text_hash }
    }
}

impl Serialize for SeedHash {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.text_hash.as_str())
    }
}

/// Validates the Settings to make sure the user hasn't made incompatible selections
fn validate_settings(settings: &Settings) -> Result<()> {
    // LC Requirement
    if !(0..=7).contains(&settings.logic.lc_requirement) {
        return Err(Error::new(format!(
            "Invalid Lorule Castle Requirement: \"{}\" was not between 0-7, inclusive.",
            settings.logic.lc_requirement
        )));
    }

    // Yuganon Requirement
    // if !(0..=7).contains(&settings.logic.yuganon_requirement) {
    //     fail!("Invalid Yuga Ganon Requirement: \"{}\" was not between 0-7, inclusive.", settings.logic.yuganon_requirement);
    // }

    if settings.logic.yuganon_requirement != settings.logic.lc_requirement {
        return Err(Error::new(format!(
            "Yuga Ganon Requirement: \"{}\" is different than Lorule Castle Requirement: \"{}\"\n\
        Different values for these settings are not yet supported!",
            settings.logic.yuganon_requirement, settings.logic.lc_requirement
        )));
    }

    // Swords
    if settings.logic.sword_in_shop && settings.logic.swordless_mode {
        return Err(Error::new(
            "The sword_in_shop and swordless_mode settings cannot both be enabled.",
        ));
    }

    // Assured Weapons
    if settings.logic.assured_weapon
        && (settings.logic.sword_in_shop || settings.logic.boots_in_shop)
    {
        return Err(Error::new(
            "The assured_weapon setting cannot be enabled when either sword_in_shop or boots_in_shop is also enabled."
        ));
    }

    Ok(())
}

pub type CheckMap = BTreeMap<String, Option<FillerItem>>;

fn calculate_seed_info(
    seed: u32, settings: Settings, hash: SeedHash, rng: &mut StdRng,
) -> Result<SeedInfo> {
    println!();
    info!("Calculating Seed Info...");

    // Build World Graph
    let world_graph = &mut world::build_world_graph();
    let check_map = &mut filler::prefill_check_map(world_graph)?;
    let (mut progression, mut junk) = item_pools::get_item_pools(&settings, rng);

    // Filler Algorithm
    let filled: Vec<(LocationKey, Item)> = filler::fill_all_locations_reachable(
        world_graph, check_map, &mut progression, &mut junk, &settings, rng,
    )?;

    // Build legacy Layout object
    let mut layout = Layout::default();
    for (location_info, item) in filled {
        layout.set(location_info, item);
    }

    let metrics = metrics::calculate_metrics(world_graph, check_map, &settings)?;
    let hints = hints::generate_hints(world_graph, check_map, &settings, rng)?;

    let mod_ = Mod { name: hash.text_hash, hash: Some(hash.item_hash), settings, layout, hints };
    Ok(SeedInfo { seed, version: VERSION, mod_, metrics })
}

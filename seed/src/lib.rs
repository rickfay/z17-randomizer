#![feature(variant_count)]

use {
    crate::{
        filler::filler_item::FillerItem,
        hints::Hints,
        world::{check::Check, location::Location, region::Region, LocationId, RegionId},
    },
    jack::{item::Item, lms::msbt::formatting::*},
    macros::fail,
    rand::prelude::StdRng,
    serde::{Serialize, Serializer},
    settings::Settings,
    std::{
        collections::{hash_map::DefaultHasher, BTreeMap, HashMap},
        hash::{Hash, Hasher},
        ops::Deref,
    },
    world::check::CheckId,
};

mod csmc;
pub mod filler;
pub mod hints;
pub mod settings;
pub mod world;

/// Manages various mappings for the seed and provides a convenient lookup API.
#[derive(Serialize)]
pub struct Seed<'s> {
    seed_num: u32,
    hash: SeedHash,
    settings: &'s Settings,

    hints: Hints,
    metrics: Metrics,

    #[serde(skip_serializing)]
    region_map: HashMap<RegionId, Region>,
    #[serde(skip_serializing)]
    location_map: HashMap<LocationId, Location>,
    #[serde(skip_serializing)]
    check_map: HashMap<CheckId, Check>,
}

impl<'s> Seed<'s> {
    pub fn init(seed_num: u32, hash: SeedHash, settings: &Settings, _rng: &mut StdRng) -> Self {
        Self {
            seed_num,
            hash,
            settings,
            region_map: world::region_map(),
            location_map: world::location_map(),
            check_map: world::check_map(),
            hints: Default::default(),   // todo
            metrics: Default::default(), // todo
        }
    }

    pub fn get_check_map(&mut self) -> &mut HashMap<CheckId, Check> {
        &mut self.check_map
    }

    pub fn set_hints(&mut self, hints: Hints) {
        self.hints = hints;
    }

    pub fn set_metrics(&mut self, metrics: Metrics) {
        self.metrics = metrics;
    }

    /// Finds the first [`Check`] with the specified [`FillerItem`].
    pub fn find_item(&self, item: FillerItem) -> Option<Check> {
        for (_, check) in &self.check_map {
            if let Some(check_item) = check.get_item() {
                if item.eq(&check_item) {
                    return Some(check.to_owned());
                }
            }
        }

        None
    }

    /// Finds all [`Check`]s with the specified [`FillerItem`].
    pub fn find_items(&self, item: FillerItem) -> Vec<Check> {
        let mut checks = Vec::new();

        for (_, check) in &self.check_map {
            if let Some(check_item) = check.get_item() {
                if item.eq(&check_item) {
                    checks.push(check.to_owned());
                }
            }
        }

        checks
    }

    /// Gets the actual [`Region`] from its [`RegionId`]
    pub fn get_region(&self, region: RegionId) -> &Region {
        self.region_map.get(&region).expect(&format!("Region lookup for ID: {:?}", region))
    }

    /// Gets the actual [`Location`] from its [`LocationId`]
    pub fn get_location(&self, location: LocationId) -> &Location {
        self.location_map.get(&location).expect(&format!("Location lookup for ID: {:?}", location))
    }

    /// Gets the actual [`Check`] from its [`CheckId`]
    pub fn get_check(&self, check: &CheckId) -> &Check {
        self.check_map.get(&check).expect(&format!("Check lookup for ID: {:?}", check))
    }

    /// Gets the [`Region`] where [`LocationId`] can be found
    pub fn get_location_region(&self, location: LocationId) -> Region {
        for (_, region) in self.region_map {
            if region.get_locations().contains(&location) {
                return region;
            }
        }

        fail!("Location {:?} didn't belong to any Region.", location);
    }

    /// Gets the [`Location`] of a given [`CheckId`]
    pub fn get_check_location(&self, check: CheckId) -> Location {
        for (_, location) in self.location_map {
            if let Some(checks) = location.get_checks() {
                if checks.contains(&check) {
                    return location;
                }
            }
        }

        fail!("Check {:?} didn't belong to any Location.", check);
    }

    /// Gets the [`Region`] where the given [`CheckId`] can be found
    pub fn get_check_region(&self, check: CheckId) -> Region {
        self.get_location_region(self.get_check_location(check).get_id())
    }

    /// Returns the [`Region`] where a [`FillerItem`] can be found
    pub fn find_item_region(&self, item: FillerItem) -> Option<Region> {
        if let Some(check) = self.find_item(item) {
            return Some(self.get_check_region(check.get_check_id()));
        }

        None
    }
}

pub type Playthrough = BTreeMap<String, BTreeMap<&'static str, &'static str>>;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Metrics {
    spheres: usize,
    playthrough: Playthrough,
}

impl Metrics {
    pub fn new(playthrough: Playthrough) -> Self {
        Self { spheres: playthrough.len(), playthrough }
    }
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
    pub fn new(seed_num: u32, settings: &Settings) -> Self {
        // Calculate underlying Hash
        let mut hasher = DefaultHasher::new();
        (seed_num, settings).hash(&mut hasher);
        let mut hash = hasher.finish() % 100_000;

        // Convert to Item Hash
        let hash_item_lut: Vec<(&String, &str)> = vec![
            (A_BUTTON.deref(), "(A)"),
            (B_BUTTON.deref(), "(B)"),
            (X_BUTTON.deref(), "(X)"),
            (Y_BUTTON.deref(), "(Y)"),
            (L_BUTTON.deref(), "(L)"),
            (R_BUTTON.deref(), "(R)"),
            (RAVIO.deref(), "(Ravio)"),
            (BOW.deref(), "(Bow)"),
            (BOMBS.deref(), "(Bombs)"),
            (FIRE_ROD.deref(), "(Fire Rod)"),
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

    pub fn get_item_hash(&self) -> &String {
        &self.item_hash
    }

    pub fn get_text_hash(&self) -> &String {
        &self.text_hash
    }
}

impl Serialize for SeedHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.text_hash.as_str())
    }
}

pub type World = BTreeMap<&'static str, BTreeMap<&'static str, Item>>;

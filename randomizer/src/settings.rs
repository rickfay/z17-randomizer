use {
    crate::{entrance_rando::EntranceShuffleSetting, logic_mode::LogicMode, regions, LocationInfo},
    serde::{Deserialize, Serialize},
    std::{
        collections::{HashMap, HashSet},
        hash::{Hash, Hasher},
    },
};

/// Logic and behavior settings.
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub logic: Logic,
    pub options: Options,
    pub exclusions: Exclusion,
    #[serde(skip_serializing_if = "Exclude::is_empty")]
    pub exclude: Exclude,
}

impl Settings {
    pub fn is_excluded(&self, location: &LocationInfo) -> bool {
        let world = match location.world() {
            regions::World::Hyrule => &self.exclude.hyrule,
            regions::World::Lorule => &self.exclude.lorule,
            regions::World::Dungeons => &self.exclude.dungeons,
        };
        world
            .0
            .get(location.region())
            .map(|region| region.contains(location.name()))
            .unwrap_or(false)
    }
}

const fn seven() -> u8 {
    7
}
const fn r#true() -> bool {
    true
}

/// Settings to change the randomizer's logic checks.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Logic {
    /// Logic to use for item placement (Normal, Hard, Glitched (Basic, Advanced, Hell), No Logic)
    #[serde(default)]
    pub mode: LogicMode,

    /// Randomizes the Pendants and Portraits between Hyrule and Lorule dungeons
    #[serde(default = "r#true")]
    pub randomize_dungeon_prizes: bool,

    /// The number of Portraits needed to trigger the Hilda cutscene to open Lorule Castle
    #[serde(default = "seven")]
    pub lc_requirement: u8,

    /// The number of Portraits needed to fight Yuga Ganon
    #[serde(default = "seven")]
    pub yuganon_requirement: u8,

    /// todo
    #[serde(default)]
    pub entrance_rando: EntranceShuffleSetting,

    /// Guarantees a Weapon is placed in Ravio's Shop
    #[serde(default)]
    pub assured_weapon: bool,

    /// Places the Bell in Ravio's Shop
    #[serde(default)]
    pub bell_in_shop: bool,

    /// Places the Pouch in Ravio's Shop
    #[serde(default)]
    pub pouch_in_shop: bool,

    /// Places the Pegasus Boots in Ravio's Shop
    #[serde(default)]
    pub boots_in_shop: bool,

    /// Excludes Cucco Ranch, both Rupee Rushes, Treacherous Tower, Octoball Derby, and Hyrule Hotfoot (both races)
    #[serde(default)]
    pub minigames_excluded: bool,

    /// Swordless Mode
    #[serde(default)]
    pub swordless_mode: bool,

    /// Shuffle Super Lamp and Super Net
    #[serde(default)]
    pub super_items: bool,

    /// Skip Trials Door in Lorule Castle
    #[serde(default)]
    pub skip_trials: bool,

    /// Guarantees Bow of Light will be placed in Lorule Castle
    #[serde(default)]
    pub bow_of_light_in_castle: bool,

    /// Lamp Requirement. If enabled, the player may have to cross dark rooms without Lamp
    #[serde(default)]
    pub lampless: bool,

    /// Maiamai Madness
    #[serde(default)]
    pub maiamai_madness: bool,

    /// Pre-activates Weather Vanes, allowing the Bell to travel anywhere from game start
    #[serde(default)]
    pub vanes_activated: bool,
}

/// Settings to change the randomizer's logic checks.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Options {
    /// Alters treasure chest sizes depending on their contents: Large for Progression items, Small for everything else.
    pub chest_size_matches_contents: bool,
    /// Experimental: Change Hyrule to the nighttime color scheme (until visiting Lorule)
    pub night_mode: bool,
}

/// A setting for useless items.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Skippable {
    Unchanged,
    Shuffled,
    Skip,
}

impl Skippable {
    pub fn is_shuffled(&self) -> bool {
        *self == Self::Shuffled
    }

    pub fn is_skipped(&self) -> bool {
        *self == Self::Skip
    }
}

impl Default for Skippable {
    fn default() -> Self {
        Self::Unchanged
    }
}

/// A setting for progression items.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Progression {
    Unchanged,
    Shuffled,
}

impl Progression {
    pub fn is_shuffled(&self) -> bool {
        *self == Self::Shuffled
    }
}

impl Default for Progression {
    fn default() -> Self {
        Self::Unchanged
    }
}

/// A setting for the castle barrier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Barrier {
    Unchanged,
    Start,
}

impl Barrier {
    pub fn is_start(&self) -> bool {
        *self == Self::Start
    }
}

impl Default for Barrier {
    fn default() -> Self {
        Self::Unchanged
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Exclude {
    #[serde(rename = "Hyrule", skip_serializing_if = "World::is_empty")]
    hyrule: World,
    #[serde(rename = "Lorule", skip_serializing_if = "World::is_empty")]
    lorule: World,
    #[serde(rename = "Dungeons", skip_serializing_if = "World::is_empty")]
    dungeons: World,
}

impl Exclude {
    fn is_empty(&self) -> bool {
        self.hyrule.is_empty() && self.lorule.is_empty() && self.dungeons.is_empty()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Exclusion(pub(crate) HashMap<String, HashSet<String>>);

impl Hash for Exclusion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.0.iter() {
            key.hash(state);
            for location in value.iter() {
                location.hash(state);
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct World(HashMap<String, HashSet<String>>);

impl World {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Hash for World {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in self.0.iter() {
            key.hash(state);
            for location in value.iter() {
                location.hash(state);
            }
        }
    }
}

pub fn open_default() -> Settings {
    Settings { ..Default::default() }
}

pub fn plando_settings() -> Settings {
    Settings {
        logic: Logic {
            lc_requirement: 4,
            yuganon_requirement: 3,
            vanes_activated: true,
            ..Default::default()
        },
        options: Options { chest_size_matches_contents: true, night_mode: false },
        ..Default::default()
    }
}

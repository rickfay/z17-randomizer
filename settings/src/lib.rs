use {
    crate::{logic::Logic, logic_mode::LogicMode::*},
    log::info,
    serde::{Deserialize, Serialize},
    std::{
        collections::{HashMap, HashSet},
        hash::{Hash, Hasher},
    },
};

pub mod entrance_shuffle_setting;
pub mod logic;
pub mod logic_mode;
pub mod pedestal_setting;

/// Logic and behavior settings.
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(default)]
pub struct Settings {
    #[serde(skip_serializing_if = "is_false")]
    pub dev_mode: bool,
    pub logic: Logic,
    pub options: Options,
    pub exclusions: Exclusion,
    #[serde(skip_serializing_if = "Exclude::is_empty")]
    pub exclude: Exclude,
}

impl Settings {
    pub fn log_settings(&self) {
        let Settings { logic, options, .. } = self;

        info!("Logic Mode:                     {}", match logic.logic_mode {
            Normal => "Normal",
            Hard => "Hard",
            Glitched => "Glitched",
            AdvGlitched => "Adv. Glitched",
            Hell => "Hell - Did you really mean to choose this?",
            NoLogic => "No Logic",
        });
        info!(
            "Dungeon Prizes:                 {}",
            if logic.randomize_dungeon_prizes { "Randomized" } else { "Not Randomized" }
        );
        info!("Lorule Castle Requirement:      {} Portraits", logic.lc_requirement);
        info!("Yuga Ganon Requirement:         {} Portraits", logic.yuganon_requirement);
        info!("Pedestal Requirement:           {}", logic.ped_requirement);

        info!("Nice Mode:                      {}", if logic.nice_mode { "ON" } else { "OFF" });
        info!(
            "Super Items:                    {}",
            if logic.super_items { "Shuffled" } else { "Not Shuffled" }
        );
        info!(
            "Reverse Sage Events:            {}",
            if logic.reverse_sage_events { "ON" } else { "OFF" }
        );
        info!(
            "Progression-Granting Enemies:   {}",
            if logic.no_progression_enemies { "Removed" } else { "Vanilla" }
        );

        info!(
            "Maiamai:                        {}",
            if logic.maiamai_madness { "Madness" } else { "Not Randomized" }
        );

        info!(
            "Start with Merge:               {}",
            if logic.start_with_merge { "Yes" } else { "No" }
        );
        let shop_items = vec![
            (logic.bell_in_shop, "Bell"),
            (logic.pouch_in_shop, "Pouch"),
            (logic.sword_in_shop, "Sword"),
            (logic.boots_in_shop, "Pegasus Boots"),
            (logic.assured_weapon, "Weapon"),
        ]
        .iter()
        .flat_map(|(setting, str)| if *setting { Some(*str) } else { None })
        .collect::<Vec<_>>()
        .join(", ");
        if !shop_items.is_empty() {
            info!("Starting Shop Items:            {}", shop_items);
        }
        info!(
            "Minigames:                      {}",
            if logic.minigames_excluded { "Excluded" } else { "Included" }
        );
        info!(
            "Trials:                         {}",
            if logic.skip_trials { "Skipped" } else { "Normal" }
        );
        info!(
            "Bow of Light:                   {}",
            if logic.bow_of_light_in_castle { "Tournament" } else { "Normal" }
        );
        info!(
            "Weather Vanes:                  {}",
            if logic.weather_vanes_activated { "All Activated" } else { "Normal" }
        );
        info!(
            "Dark Room Crossing:             {}",
            if logic.dark_rooms_lampless { "Lamp Not Required" } else { "Lamp Required" }
        );
        info!(
            "Swords:                         {}",
            if logic.swordless_mode { "Swordless Mode - NO SWORDS" } else { "Normal" }
        );
        info!(
            "Chest Size:                     {}",
            if options.chest_size_matches_contents { "Matches Contents" } else { "Normal" }
        );
        info!("Hint Ghost Price:               {}", logic.hint_ghost_price);
        println!();
    }
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
    pub hyrule: World,
    #[serde(rename = "Lorule", skip_serializing_if = "World::is_empty")]
    pub lorule: World,
    #[serde(rename = "Dungeons", skip_serializing_if = "World::is_empty")]
    pub dungeons: World,
}

impl Exclude {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    fn is_empty(&self) -> bool {
        self.hyrule.is_empty() && self.lorule.is_empty() && self.dungeons.is_empty()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Exclusion(pub HashMap<String, HashSet<String>>);

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
pub struct World(pub(crate) HashMap<String, HashSet<String>>);

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

pub(crate) const fn is_false(b: &bool) -> bool {
    *b == false
}
pub(crate) const fn seven() -> u8 {
    7
}
pub(crate) const fn fifty() -> u16 {
    50
}
pub(crate) const fn r#true() -> bool {
    true
}

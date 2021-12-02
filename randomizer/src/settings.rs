use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use crate::{regions, Location};

/// Logic and behavior settings.
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
    pub modifications: Modifications,
    pub logic: Logic,
    pub items: Items,
    pub behavior: Behavior,
    #[serde(skip_serializing_if = "Exclude::is_empty")]
    exclude: Exclude,
}

impl Settings {
    pub fn is_excluded(&self, location: &Location) -> bool {
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

/// Modifications to the core game mechanics.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Modifications {
    pub y_button_enabled: bool,
}

/// Settings to change the randomizer's logic checks.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Logic {
    /// Disables lamp requirement for dark areas.
    pub dont_require_lamp_for_darkness: bool,
    /// Requires seller's golden bee for Bee Badge check.
    pub require_golden_bee_for_sale: bool,
    /// Allows keys to be placed in such a way that the player can softlock.
    pub unsafe_key_placement: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Items {
    pub captains_sword: Skippable,
    pub borrowed_sword: Progression,
    pub lamp: Progression,
    pub first_bracelet: Skippable,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Behavior {
    pub open: bool,
    pub barrier: Barrier,
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
    Settings {
        modifications: Modifications {
            y_button_enabled: true,
        },
        items: Items {
            captains_sword: Skippable::Skip,
            borrowed_sword: Progression::Shuffled,
            lamp: Progression::Shuffled,
            first_bracelet: Skippable::Skip,
        },
        behavior: Behavior {
            open: true,
            barrier: Barrier::Start,
        },
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;
    use crate::Result;

    #[test]
    fn it_deserializes_standard_config() -> Result<()> {
        let file = include_str!("../../presets/Standard.toml");
        let uncommented = file
            .split_inclusive('\n')
            .map(|line| line.strip_prefix('#').unwrap_or(line))
            .collect::<String>();
        toml::from_str::<Settings>(&uncommented).expect("Could not deserialize Standard.toml");
        Ok(())
    }
}

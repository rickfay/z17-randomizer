use crate::settings::keysy::Keysy;
use crate::settings::logic::LogicMode;
use crate::settings::pedestal::PedestalSetting;
use crate::settings::portal_shuffle::PortalShuffle;
use crate::settings::portals::Portals;
use crate::settings::ravios_shop::RaviosShop;
use crate::settings::trials_door::TrialsDoor;
use crate::settings::weather_vanes::WeatherVanes;
use log::info;
use logic::LogicMode::*;
use serde::{Deserialize, Serialize};
use std::collections::btree_set::BTreeSet;
use std::hash::Hash;

pub mod keysy;
pub mod logic;
pub mod pedestal;
pub mod portal_shuffle;
pub mod portals;
pub mod ravios_shop;
pub mod trials_door;
pub mod weather_vanes;

/// Logic and behavior settings.
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(default)]
pub struct Settings {
    #[serde(skip_serializing_if = "is_false")]
    pub dev_mode: bool,

    /// The number of Portraits needed to trigger the Hilda cutscene to open Lorule Castle
    #[serde(default = "seven")]
    pub lc_requirement: u8,

    /// The number of Portraits needed to fight Yuga Ganon
    #[serde(default = "seven", skip_serializing)]
    pub yuganon_requirement: u8,

    /// Configure which Pendants are required to reach the Master Sword Pedestal
    #[serde(default)]
    pub ped_requirement: PedestalSetting,

    /// Logic to use for item placement (Normal, Hard, Glitched, Adv. Glitched, Hell, No Logic)
    #[serde(default)]
    pub logic_mode: LogicMode,

    /// Dark Room Lamp Requirement. If enabled, the player may have to cross dark rooms without Lamp
    #[serde(default)]
    pub dark_rooms_lampless: bool,

    /// Randomizes the Pendants and Portraits between Hyrule and Lorule dungeons
    #[serde(default = "r#true")]
    pub dungeon_prize_shuffle: bool,

    /// Maiamai Madness
    #[serde(default)]
    pub maiamai_madness: bool,

    /// Shuffles Nice Items into the general item pool as progressive upgrades (temporary: removes Maiamai cave)
    #[serde(default)]
    pub nice_mode: bool,

    /// Shuffle Super Lamp and Super Net
    #[serde(default)]
    pub super_mode: bool,

    /// Portals Open/Closed Setting
    pub portals: Portals,

    /// Shuffles the Portal destinations amongst each other
    #[serde(default)]
    pub portal_shuffle: PortalShuffle,

    /// Weather Vanes behavior and activation setting.
    #[serde(default)]
    pub weather_vanes: WeatherVanes,

    /// Ravio's Shop
    #[serde(default, skip_deserializing)]
    pub ravios_shop: RaviosShop,

    /// Guarantees Bow of Light will be placed in Lorule Castle
    #[serde(default)]
    pub bow_of_light_in_castle: bool,

    /// Removes Enemies from dungeons that are themselves Progression (e.g.: Bawbs, the bomb enemy).
    /// Logic will be adjusted to require the player's items instead.
    #[serde(default)]
    pub no_progression_enemies: bool,

    /// Keysy
    #[serde(default)]
    pub keysy: Keysy,

    /// Makes the Bow of Light the third upgrade for the Bow
    #[serde(default)]
    pub progressive_bow_of_light: bool,

    /// Swordless Mode
    /// Not available if [`sword_in_shop`] option is enabled.
    #[serde(default)]
    pub swordless_mode: bool,

    /// Start with the ability to Merge into walls, without Ravio's Bracelet.
    #[serde(default)]
    pub start_with_merge: bool,

    /// Start with a usable X button
    #[serde(default)]
    pub start_with_pouch: bool,

    /// Places the Bell in Ravio's Shop
    #[serde(default)]
    pub bell_in_shop: bool,

    /// Places a Sword in Ravio's Shop. Disables the ability to play in Swordless Mode.
    #[serde(default)]
    pub sword_in_shop: bool,

    /// Places the Pegasus Boots in Ravio's Shop
    #[serde(default)]
    pub boots_in_shop: bool,

    /// Guarantees a Weapon is placed in Ravio's Shop.
    /// Not available if [`boots_in_shop`] or [`sword_in_shop`] are enabled as they already are weapons.
    #[serde(default)]
    pub assured_weapon: bool,

    /// Alters treasure chest sizes depending on their contents: Large for Progression items, Small for everything else.
    pub chest_size_matches_contents: bool,

    /// Excludes Cucco Ranch, both Rupee Rushes, Treacherous Tower, Octoball Derby, and Hyrule Hotfoot (both races)
    #[serde(default)]
    pub minigames_excluded: bool,

    /// Skips the Big Bomb Flower by removing the 5 Big Rocks in Lorule Field (Does not affect Lorule Castle Bomb Trial)
    #[serde(default)]
    pub skip_big_bomb_flower: bool,

    /// Trials Door setting
    #[serde(default)]
    pub trials_door: TrialsDoor,

    /// Number of floors in Treacherous Tower
    #[serde(default = "five")]
    pub treacherous_tower_floors: usize,

    /// Experimental: Change Hyrule to the nighttime color scheme (until visiting Lorule)
    pub night_mode: bool,

    /// Set of user-provided locations to be excluded from having progression.
    pub user_exclusions: BTreeSet<String>,
}

impl Settings {
    pub fn log_settings(&self) {
        info!(
            "Logic Mode:                     {}",
            match self.logic_mode {
                Normal => "Normal",
                Hard => "Hard",
                Glitched => "Glitched",
                AdvGlitched => "Adv. Glitched",
                Hell => "Hell - Did you really mean to choose this?",
                NoLogic => "No Logic",
            }
        );
        info!(
            "Dungeon Prizes:                 {}",
            if self.dungeon_prize_shuffle { "Randomized" } else { "Not Randomized" }
        );
        info!("Lorule Castle Requirement:      {} Portraits", self.lc_requirement);
        info!("Yuga Ganon Requirement:         {} Portraits", self.yuganon_requirement);
        info!("Pedestal Requirement:           {}", self.ped_requirement);

        info!("Nice Mode:                      {}", if self.nice_mode { "ON" } else { "OFF" });
        info!("Super Items:                    {}", if self.super_mode { "Shuffled" } else { "Not Shuffled" });
        info!("Progression-Granting Enemies:   {}", if self.no_progression_enemies { "Removed" } else { "Vanilla" });

        info!("Maiamai:                        {}", if self.maiamai_madness { "Madness" } else { "Not Randomized" });

        info!("Start with Merge:               {}", if self.start_with_merge { "Yes" } else { "No" });
        info!("Start with Pouch:               {}", if self.start_with_pouch { "Yes" } else { "No" });
        let shop_items = vec![
            (&self.bell_in_shop, "Bell"),
            (&self.sword_in_shop, "Sword"),
            (&self.boots_in_shop, "Pegasus Boots"),
            (&self.assured_weapon, "Weapon"),
        ]
        .iter()
        .flat_map(|(setting, str)| if **setting { Some(*str) } else { None })
        .collect::<Vec<_>>()
        .join(", ");
        if !shop_items.is_empty() {
            info!("Starting Shop Items:            {}", shop_items);
        }
        info!("Minigames:                      {}", if self.minigames_excluded { "Excluded" } else { "Included" });
        info!("Trials Door:                    {}", self.trials_door);
        info!("Bow of Light:                   {}", if self.bow_of_light_in_castle { "Tournament" } else { "Normal" });
        info!("Weather Vanes:                  {}", self.weather_vanes);
        info!(
            "Dark Room Crossing:             {}",
            if self.dark_rooms_lampless { "Lamp Not Required" } else { "Lamp Required" }
        );
        info!(
            "Swords:                         {}",
            if self.swordless_mode { "Swordless Mode - NO SWORDS" } else { "Normal" }
        );
        info!(
            "Chest Size:                     {}",
            if self.chest_size_matches_contents { "Matches Contents" } else { "Normal" }
        );
        info!("Portal Shuffle:                 {}", self.portal_shuffle)
    }
}

const fn is_false(b: &bool) -> bool {
    !(*b)
}

const fn five() -> usize {
    5
}

const fn seven() -> u8 {
    7
}

const fn r#true() -> bool {
    true
}

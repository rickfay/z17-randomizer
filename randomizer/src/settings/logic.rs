use {
    crate::settings::{
        entrance_shuffle_setting::EntranceShuffleSetting, logic_mode::LogicMode,
        pedestal_setting::PedestalSetting,
    },
    serde::{Deserialize, Serialize},
};

/// Settings to change the randomizer's logic checks.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Logic {
    /// Logic to use for item placement (Normal, Hard, Glitched, Adv. Glitched, Hell, No Logic)
    #[serde(default)]
    pub mode: LogicMode,

    /// Randomizes the Pendants and Portraits between Hyrule and Lorule dungeons
    #[serde(default = "crate::settings::r#true")]
    pub randomize_dungeon_prizes: bool,

    /// The number of Portraits needed to trigger the Hilda cutscene to open Lorule Castle
    #[serde(default = "crate::settings::seven")]
    pub lc_requirement: u8,

    /// The number of Portraits needed to fight Yuga Ganon
    #[serde(default = "crate::settings::seven", skip_serializing)]
    pub yuganon_requirement: u8,

    /// Configure which Pendants are required to reach the Master Sword Pedestal
    #[serde(default)]
    pub ped_requirement: PedestalSetting,

    /// Shuffles Nice Items into the general item pool as progressive upgrades (temporary: removes Maiamai cave)
    #[serde(default)]
    pub nice_mode: bool,

    /// todo
    /// Makes Sage related checks and events be tied to rescuing the respective Sage
    #[serde(default, skip_serializing)]
    pub saved_sage_logic: bool,

    /// todo
    ///
    #[serde(default, skip_serializing)]
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

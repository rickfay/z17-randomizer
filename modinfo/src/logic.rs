use {
    crate::{
        entrance_shuffle_setting::EntranceShuffleSetting,
        hyrule_castle_setting::HyruleCastleSetting, logic_mode::LogicMode,
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
    pub logic_mode: LogicMode,

    /// Randomizes the Pendants and Portraits between Hyrule and Lorule dungeons
    #[serde(default = "crate::r#true")]
    pub randomize_dungeon_prizes: bool,

    /// Vanilla Charm
    /// Enabling this forces one of the two Pendant of Courage Upgrades to be in Zelda's Throne Room.
    /// Otherwise, a random Sage Portrait or Pendant will be placed in Zelda's Throne Room.
    /// Note: Has no effect unless `randomize_dungeon_prizes` is enabled.
    #[serde(default)]
    pub vanilla_charm: bool,

    /// The number of Portraits needed to trigger the Hilda cutscene to open Lorule Castle
    #[serde(default = "crate::seven")]
    pub lc_requirement: u8,

    /// The number of Portraits needed to fight Yuga Ganon
    #[serde(default = "crate::seven", skip_serializing)]
    pub yuganon_requirement: u8,

    /// Configure which Pendants are required to reach the Master Sword Pedestal
    #[serde(default)]
    pub ped_requirement: PedestalSetting,

    /// Configure how Hyrule Castle is handled by the randomizer
    #[serde(default)]
    pub hyrule_castle_setting: HyruleCastleSetting,

    /// Shuffles Nice Items into the general item pool as progressive upgrades (temporary: removes Maiamai cave)
    #[serde(default)]
    pub nice_mode: bool,

    /// Removes Enemies from dungeons that are themselves Progression (e.g.: Bawbs, the bomb enemy).
    /// Logic will be adjusted to require the player's items instead.
    #[serde(default)]
    pub no_progression_enemies: bool,

    /// Skips the Big Bomb Flower by removing the 5 Big Rocks in Lorule Field (Does not affect Lorule Castle Bomb Trial)
    #[serde(default)]
    pub skip_big_bomb_flower: bool,

    /// Makes Sage related checks and events be tied to rescuing the respective Sage
    #[serde(default)]
    pub reverse_sage_events: bool,

    /// todo
    ///
    #[serde(default, skip_serializing)]
    pub entrance_rando: EntranceShuffleSetting,

    /// Start with the ability to Merge into walls, without Ravio's Bracelet.
    #[serde(default)]
    pub start_with_merge: bool,

    /// Places the Bell in Ravio's Shop
    #[serde(default)]
    pub bell_in_shop: bool,

    /// Places the Pouch in Ravio's Shop
    #[serde(default)]
    pub pouch_in_shop: bool,

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

    /// Excludes Cucco Ranch, both Rupee Rushes, Treacherous Tower, Octoball Derby, and Hyrule Hotfoot (both races)
    #[serde(default)]
    pub minigames_excluded: bool,

    /// Swordless Mode
    /// Not available if [`sword_in_shop`] option is enabled.
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

    /// Dark Room Lamp Requirement. If enabled, the player may have to cross dark rooms without Lamp
    #[serde(default)]
    pub dark_rooms_lampless: bool,

    /// Maiamai Madness
    #[serde(default)]
    pub maiamai_madness: bool,

    /// Pre-activates Weather Vanes, allowing the Bell to travel anywhere from game start
    #[serde(default)]
    pub weather_vanes_activated: bool,

    /// Price of Hints from Hint Ghosts
    #[serde(default = "crate::thirty")]
    pub hint_ghost_price: u16,
}

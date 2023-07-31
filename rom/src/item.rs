use {
    crate::{actors::Actor, int_map, scene::Vec3, Error, Result, Rom},
    once_cell::sync::Lazy,
    regex::Regex,
    serde::{Deserialize, Serialize},
};

int_map! {
/**
 * An enum for in-game items.
 *
 * Item indexes match the array indexes found in `World/Byaml/GetItem.byaml`.
 */
Item(u16) {
    /**
     * An `Empty` item, or no item. Shows Link holding up nothing with the text: "It's Empty"
     *
     * Note: This is actually called `None` in the game files, but we've renamed it to avoid
     * confusion with Rust's [`Option::None`].
     */
    Empty = 0x00,
    /// A generic Small Key. Adds 1 to the key count of the current scene.
    KeySmall = 0x01,
    /// A generic Boss Key. Marks the Boss Key as obtained for the current scene.
    KeyBoss = 0x02,
    /// A generic Compass. Marks the Compass as obtained for the current scene.
    Compass = 0x03,
    /// Heart Container
    HeartContainer = 0x04,
    /// Red Rupee
    RupeeR = 0x05,
    /// Green Rupee
    RupeeG = 0x06,
    /// Blue Rupee
    RupeeB = 0x07,
    /// Heart Piece
    HeartPiece = 0x08,
    /// Ice Rod
    ItemIceRod = 0x09,
    /// Sand Rod
    ItemSandRod = 0x0A,
    /// Tornado Rod
    ItemTornadeRod = 0x0B,
    /// Bombs
    ItemBomb = 0x0C,
    /// Fire Rod
    ItemFireRod = 0x0D,
    /// Hookshot
    ItemHookShot = 0x0E,
    /// Boomerang
    ItemBoomerang = 0x0F,
    /// Hammer
    ItemHammer = 0x10,
    /// Bow
    ItemBow = 0x11,
    /// Shield (the one bought from Item Shops)
    ItemShield = 0x12,
    /**
     * Empty Bottles
     *
     * This [`Item`] represents all 5 Bottles and is a kind of faux-Progressive Item: The first
     * one obtained unlocks Bottle #1, the second unlocks Bottle #2, etc.
     *
     * You can determine which bottles are which in your inventory by emptying all of them and then
     * catching or buying any bottle item--the bottle it goes into is Bottle #1, regardless of how
     * items are arranged in the inventory. Repeat to identify them all if you're desperate to waste
     * 10 minutes of your Thursday evening.
     */
    ItemBottle = 0x13,
    /// Red Potion
    ItemPotShopRed = 0x14,
    /// Blue Potion
    ItemPotShopBlue = 0x15,
    /// Smooth Gem
    ItemStoneBeauty = 0x16,
    /// Pendant of Power
    PendantPower = 0x17,
    /// Pendant of Wisdom
    PendantWisdom = 0x18,
    /// Pendant of Courage
    PendantCourage = 0x19,
    /// Lamp
    ItemKandelaar = 0x1A,
    /// Forgotten Sword
    ItemSwordLv1 = 0x1B,
    /// Master Sword
    ItemSwordLv2 = 0x1C,
    /// Master Sword Lv2
    ItemSwordLv3 = 0x1D,
    /// Master Sword Lv3
    ItemSwordLv4 = 0x1E,
    /// Zora's Flippers
    ItemMizukaki = 0x1F,
    /// Rented Ice Rod
    ItemRentalIceRod = 0x20,
    /// Rented Sand Rod
    ItemRentalSandRod = 0x21,
    /// Rented Tornado Rod
    ItemRentalTornadeRod = 0x22,
    /// Rented Bombs
    ItemRentalBomb = 0x23,
    /// Rented Fire Rod
    ItemRentalFireRod = 0x24,
    /// Rented Hookshot
    ItemRentalHookShot = 0x25,
    /// Rented Boomerang
    ItemRentalBoomerang = 0x26,
    /// Rented Hammer
    ItemRentalHammer = 0x27,
    /// Rented Bow
    ItemRentalBow = 0x28,
    /**
     * Rented Shield
     *
     * The game files suggest that Shields were originally going to be rented from Ravio, but
     * this idea thankfully didn't make it to the final game.
     */
    ItemRentalShield = 0x29,
    /**
     * Ravio's Bracelet (unpowered)
     *
     * This is the first Bracelet Link receives from Ravio.
     *
     * It does not let you merge and it smells funny.
     */
    RingRental = 0x2A,
    /**
     * Ravio's Bracelet
     *
     * The upgraded version of the Bracelet that lets Link merge into walls.
     *
     * It's powered by plot points.
     */
    RingHekiga = 0x2B,
    /// Bell
    ItemBell = 0x2C,
    /// Gold Rupee
    RupeeGold = 0x2D,
    /// Silver Rupee
    RupeeSilver = 0x2E,
    /// Power Glove
    PowerGlove = 0x2F,
    /// Net
    ItemInsectNet = 0x30,
    /// Titan's Mitt
    PowerfulGlove = 0x31,
    /// Maiamai
    Kinsta = 0x32,
    /// Bee Badge
    BadgeBee = 0x33,
    /// Golden Bee (functionally identical to [`Item::GoldenBeeForSale`])
    GoldenBee = 0x34,
    /// Hint Glasses
    HintGlasses = 0x35,
    /// Scoot Fruit
    EscapeFruit = 0x36,
    /// Foul Fruit
    StopFruit = 0x37,
    /// Bee
    Bee = 0x38,
    /// Fairy
    Fairy = 0x39,
    /// Monster Tail
    LiverBlue = 0x3A,
    /// Monster Guts
    LiverPurple = 0x3B,
    /// Monster Horn
    LiverYellow = 0x3C,
    /// Captain's Sword
    PackageSword = 0x3D,
    /// Charm
    ZeldaAmulet = 0x3E,
    /// Blue Mail (progressive even in vanilla)
    ClothesBlue = 0x3F,
    /// Red Mail (progressive even in vanilla)
    ClothesRed = 0x40,
    /// Hylian Shield
    HyruleShield = 0x41,
    /// Master Ore (Dark Palace)
    OreYellow = 0x42,
    /// Master Ore (Skull Woods)
    OreGreen = 0x43,
    /// Master Ore (Thieves' Hideout)
    OreBlue = 0x44,
    /// Stamina Scroll
    GanbariPowerUp = 0x45,
    /// Pouch
    Pouch = 0x46,
    /// Pegasus Boots
    DashBoots = 0x47,
    /// Master Ore (Graveyard)
    OreRed = 0x48,
    /// Letter in a Bottle
    MessageBottle = 0x49,
    /// Premium Milk
    MilkMatured = 0x4A,
    /// Purple Potion
    ItemPotShopPurple = 0x4B,
    /// Yellow Potion
    ItemPotShopYellow = 0x4C,
    /// Nice Ice Rod
    ItemIceRodLv2 = 0x4D,
    /// Nice Sand Rod
    ItemSandRodLv2 = 0x4E,
    /// Nice Tornado Rod
    ItemTornadeRodLv2 = 0x4F,
    /// Nice Bombs
    ItemBombLv2 = 0x50,
    /// Nice Fire Rod
    ItemFireRodLv2 = 0x51,
    /// Nice Hookshot
    ItemHookShotLv2 = 0x52,
    /// Nice Boomerang
    ItemBoomerangLv2 = 0x53,
    /// Nice Hammer
    ItemHammerLv2 = 0x54,
    /// Nice Bow
    ItemBowLv2 = 0x55,
    /// Great Spin
    SpecialMove = 0x56,
    /// Milk (regular)
    Milk = 0x57,
    /// Super Lamp
    ItemKandelaarLv2 = 0x58,
    /// Super Net
    ItemInsectNetLv2 = 0x59,
    /// Energy Potion (for one-time event the first time you pick it up)
    GanbariTubo = 0x5A,
    /// Purple Rupee
    RupeePurple = 0x5B,
    /// Bow of Light
    ItemBowLight = 0x5C,
    /// This is not the Triforce of Courage obtained during the game, but a dummy item that's never used.
    TriforceCourage = 0x5D,
    /// Heart from Stylish Woman (Street Merchant & Big Cucco hearts are not GetItems)
    Heart = 0x5E,
    /// Osfala's Sand Rod, functionally the same as Rental Sand Rod but in vanilla you lose it immediately
    ItemRentalSandRodFirst = 0x5F,
    /// Golden Bee (functionally identical to [`Item::GoldenBee`])
    GoldenBeeForSale = 0x60,

    ////////////////////////////////////////////////////////////////////////////////////////////////
    // IDs below are fake, to-be-added as new GetItems                                            //
    ////////////////////////////////////////////////////////////////////////////////////////////////

    /// Sage Gulley (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageGulley = 0x61,
    /// Sage Oren (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageOren = 0x62,
    /// Sage Seres (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageSeres = 0x63,
    /// Sage Osfala (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageOsfala = 0x64,
    /// Sage Impa (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageImpa = 0x65,
    /// Sage Irene (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageIrene = 0x66,
    /// Sage Rosso (FAKE ITEM, acts as stand-in until we can add new GetItems)
    SageRosso = 0x67,
}}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetItem(
    pub String,
    pub String,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub String,
    pub String,
    pub String,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
);

impl GetItem {
    pub fn actor(&self, game: &Rom) -> Option<Actor> {
        if self.1.is_empty() {
            None
        } else {
            Some(game.get_item_actor(self.actor_name().unwrap()).unwrap())
        }
    }

    pub fn actor_name(&self) -> Result<&str> {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Actor/([A-Za-z]+)\.bch$").unwrap());
        REGEX
            .captures(&self.1)
            .and_then(|captures| captures.get(1))
            .map(|match_| match_.as_str())
            .ok_or_else(|| Error::new(format!("Invalid actor name: '{}'", &self.1)))
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.2
    }

    pub fn get_rotate(&self) -> Vec3 {
        Vec3 { x: self.9, y: self.10, z: self.11 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod actor_name {
        use super::*;

        #[test]
        fn it_works() -> Result<()> {
            let get_item = GetItem(
                String::new(),
                "Actor/Test.bch".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                String::new(),
                String::new(),
                String::new(),
                0,
                0,
                0,
                0,
                0,
            );
            assert_eq!(get_item.actor_name()?, "Test");
            Ok(())
        }
    }
}

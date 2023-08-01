use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{AsRefStr, Display, EnumIter, EnumString};

/**
 * An enum for in-game items.
 *
 * Item indexes match the array indexes found in `World/Byaml/GetItem.byaml`.
 */
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Deserialize_repr,
    Serialize_repr,
    AsRefStr,
    Display,
    EnumIter,
    EnumString,
)]
#[repr(u16)]
pub enum Item {
    /**
     * An `Empty` item, or no item. Shows Link holding up nothing with the text: "It's Empty"
     *
     * Note: This is actually called `None` in the game files, but we've renamed it to avoid
     * confusion with Rust's [`Option::None`].
     */
    Empty = 0x00,
    /// A generic Small Key. Adds 1 to the key count of the current scene.
    #[strum(serialize = "Small Key")]
    KeySmall = 0x01,
    /// A generic Boss Key. Marks the Boss Key as obtained for the current scene.
    #[strum(serialize = "Boss Key")]
    KeyBoss = 0x02,
    /// A generic Compass. Marks the Compass as obtained for the current scene.
    Compass = 0x03,
    /// Heart Container
    #[strum(serialize = "Heart Container")]
    HeartContainer = 0x04,
    /// Red Rupee
    #[strum(serialize = "Red Rupee")]
    RupeeR = 0x05,
    /// Green Rupee
    #[strum(serialize = "Green Rupee")]
    RupeeG = 0x06,
    /// Blue Rupee
    #[strum(serialize = "Blue Rupee")]
    RupeeB = 0x07,
    /// Heart Piece
    #[strum(serialize = "Heart Piece")]
    HeartPiece = 0x08,
    /// Ice Rod
    #[strum(serialize = "Ice Rod")]
    ItemIceRod = 0x09,
    /// Sand Rod
    #[strum(serialize = "Sand Rod")]
    ItemSandRod = 0x0A,
    /// Tornado Rod
    #[strum(serialize = "Tornado Rod")]
    ItemTornadeRod = 0x0B,
    /// Bombs
    #[strum(serialize = "Bombs")]
    ItemBomb = 0x0C,
    /// Fire Rod
    #[strum(serialize = "Fire Rod")]
    ItemFireRod = 0x0D,
    /// Hookshot
    #[strum(serialize = "Hookshot")]
    ItemHookShot = 0x0E,
    /// Boomerang
    #[strum(serialize = "Boomerang")]
    ItemBoomerang = 0x0F,
    /// Hammer
    #[strum(serialize = "Hammer")]
    ItemHammer = 0x10,
    /// Bow
    #[strum(serialize = "Bow")]
    ItemBow = 0x11,
    /// Shield (the one bought from Item Shops)
    #[strum(serialize = "Shield")]
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
    #[strum(serialize = "Bottle")]
    ItemBottle = 0x13,
    /// Red Potion
    #[strum(serialize = "Red Potion")]
    ItemPotShopRed = 0x14,
    /// Blue Potion
    #[strum(serialize = "Blue Potion")]
    ItemPotShopBlue = 0x15,
    /// Smooth Gem
    #[strum(serialize = "Smooth Gem")]
    ItemStoneBeauty = 0x16,
    /// Pendant of Power
    #[strum(serialize = "Pendant of Power")]
    PendantPower = 0x17,
    /// Pendant of Wisdom
    #[strum(serialize = "Pendant of Wisdom")]
    PendantWisdom = 0x18,
    /// Pendant of Courage
    #[strum(serialize = "Pendant of Courage")]
    PendantCourage = 0x19,
    /// Lamp
    #[strum(serialize = "Lamp")]
    ItemKandelaar = 0x1A,
    /// Forgotten Sword
    #[strum(serialize = "Forgotten Sword")]
    ItemSwordLv1 = 0x1B,
    /// Master Sword
    #[strum(serialize = "Master Sword")]
    ItemSwordLv2 = 0x1C,
    /// Master Sword Lv2
    #[strum(serialize = "Master Sword Lv2")]
    ItemSwordLv3 = 0x1D,
    /// Master Sword Lv3
    #[strum(serialize = "Master Sword Lv3")]
    ItemSwordLv4 = 0x1E,
    /// Zora's Flippers
    #[strum(serialize = "Zora's Flippers")]
    ItemMizukaki = 0x1F,
    /// Rented Ice Rod
    #[strum(serialize = "Rented Ice Rod")]
    ItemRentalIceRod = 0x20,
    /// Rented Sand Rod
    #[strum(serialize = "Rented Sand Rod")]
    ItemRentalSandRod = 0x21,
    /// Rented Tornado Rod
    #[strum(serialize = "Rented Tornado Rod")]
    ItemRentalTornadeRod = 0x22,
    /// Rented Bombs
    #[strum(serialize = "Rented Bombs")]
    ItemRentalBomb = 0x23,
    /// Rented Fire Rod
    #[strum(serialize = "Rented Fire Rod")]
    ItemRentalFireRod = 0x24,
    /// Rented Hookshot
    #[strum(serialize = "Rented Hookshot")]
    ItemRentalHookShot = 0x25,
    /// Rented Boomerang
    #[strum(serialize = "Rented Boomerang")]
    ItemRentalBoomerang = 0x26,
    /// Rented Hammer
    #[strum(serialize = "Rented Hammer")]
    ItemRentalHammer = 0x27,
    /// Rented Bow
    #[strum(serialize = "Rented Bow")]
    ItemRentalBow = 0x28,
    /**
     * Rented Shield
     *
     * The game files suggest that Shields were originally going to be rented from Ravio, but
     * this idea thankfully didn't make it to the final game.
     */
    #[strum(serialize = "Rented Shield")]
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
    #[strum(serialize = "Ravio's Bracelet")]
    RingHekiga = 0x2B,
    /// Bell
    #[strum(serialize = "Bell")]
    ItemBell = 0x2C,
    /// Gold Rupee
    #[strum(serialize = "Gold Rupee")]
    RupeeGold = 0x2D,
    /// Silver Rupee
    #[strum(serialize = "Silver Rupee")]
    RupeeSilver = 0x2E,
    /// Power Glove
    #[strum(serialize = "Power Glove")]
    PowerGlove = 0x2F,
    /// Net
    #[strum(serialize = "Net")]
    ItemInsectNet = 0x30,
    /// Titan's Mitt
    #[strum(serialize = "Titan's Mitt")]
    PowerfulGlove = 0x31,
    /// Maiamai
    #[strum(serialize = "Maiamai")]
    Kinsta = 0x32,
    /// Bee Badge
    #[strum(serialize = "Bee Badge")]
    BadgeBee = 0x33,
    /// Golden Bee (functionally identical to [`Item::GoldenBeeForSale`])
    #[strum(serialize = "Golden Bee")]
    GoldenBee = 0x34,
    /// Hint Glasses
    #[strum(serialize = "Hint Glasses")]
    HintGlasses = 0x35,
    /// Scoot Fruit
    #[strum(serialize = "Scoot Fruit")]
    EscapeFruit = 0x36,
    /// Foul Fruit
    #[strum(serialize = "Foul Fruit")]
    StopFruit = 0x37,
    /// Bee
    #[strum(serialize = "Bee")]
    Bee = 0x38,
    /// Fairy
    #[strum(serialize = "Fairy")]
    Fairy = 0x39,
    /// Monster Tail
    #[strum(serialize = "Monster Tail")]
    LiverBlue = 0x3A,
    /// Monster Guts
    #[strum(serialize = "Monster Guts")]
    LiverPurple = 0x3B,
    /// Monster Horn
    #[strum(serialize = "Monster Horn")]
    LiverYellow = 0x3C,
    /// Captain's Sword
    #[strum(serialize = "Captain's Sword")]
    PackageSword = 0x3D,
    /// Charm
    #[strum(serialize = "Charm")]
    ZeldaAmulet = 0x3E,
    /// Blue Mail (progressive even in vanilla)
    #[strum(serialize = "Blue Mail")]
    ClothesBlue = 0x3F,
    /// Red Mail (progressive even in vanilla)
    #[strum(serialize = "Red Mail")]
    ClothesRed = 0x40,
    /// Hylian Shield
    #[strum(serialize = "Hylian Shield")]
    HyruleShield = 0x41,
    /// Master Ore (Dark Palace)
    #[strum(serialize = "Master Ore")]
    OreYellow = 0x42,
    /// Master Ore (Skull Woods)
    OreGreen = 0x43,
    /// Master Ore (Thieves' Hideout)
    OreBlue = 0x44,
    /// Stamina Scroll
    #[strum(serialize = "Stamina Scroll")]
    GanbariPowerUp = 0x45,
    /// Pouch
    Pouch = 0x46,
    /// Pegasus Boots
    #[strum(serialize = "Pegasus Boots")]
    DashBoots = 0x47,
    /// Master Ore (Graveyard)
    OreRed = 0x48,
    /// Letter in a Bottle
    #[strum(serialize = "Letter in a Bottle")]
    MessageBottle = 0x49,
    /// Premium Milk
    #[strum(serialize = "Premium Milk")]
    MilkMatured = 0x4A,
    /// Purple Potion
    #[strum(serialize = "Purple Potion")]
    ItemPotShopPurple = 0x4B,
    /// Yellow Potion
    #[strum(serialize = "Yellow Potion")]
    ItemPotShopYellow = 0x4C,
    /// Nice Ice Rod
    #[strum(serialize = "Nice Ice Rod")]
    ItemIceRodLv2 = 0x4D,
    /// Nice Sand Rod
    #[strum(serialize = "Nice Sand Rod")]
    ItemSandRodLv2 = 0x4E,
    /// Nice Tornado Rod
    #[strum(serialize = "Nice Tornado Rod")]
    ItemTornadeRodLv2 = 0x4F,
    /// Nice Bombs
    #[strum(serialize = "Nice Bombs")]
    ItemBombLv2 = 0x50,
    /// Nice Fire Rod
    #[strum(serialize = "Nice Fire Rod")]
    ItemFireRodLv2 = 0x51,
    /// Nice Hookshot
    #[strum(serialize = "Nice Hookshot")]
    ItemHookShotLv2 = 0x52,
    /// Nice Boomerang
    #[strum(serialize = "Nice Boomerang")]
    ItemBoomerangLv2 = 0x53,
    /// Nice Hammer
    #[strum(serialize = "Nice Hammer")]
    ItemHammerLv2 = 0x54,
    /// Nice Bow
    #[strum(serialize = "Nice Bow")]
    ItemBowLv2 = 0x55,
    /// Great Spin
    #[strum(serialize = "Great Spin")]
    SpecialMove = 0x56,
    /// Milk (regular)
    Milk = 0x57,
    /// Super Lamp
    #[strum(serialize = "Super Lamp")]
    ItemKandelaarLv2 = 0x58,
    /// Super Net
    #[strum(serialize = "Super Net")]
    ItemInsectNetLv2 = 0x59,
    /// Energy Potion (for one-time event the first time you pick it up)
    #[strum(serialize = "Energy Potion")]
    GanbariTubo = 0x5A,
    /// Purple Rupee
    #[strum(serialize = "Purple Rupee")]
    RupeePurple = 0x5B,
    /// Bow of Light
    #[strum(serialize = "Bow of Light")]
    ItemBowLight = 0x5C,
    /// This is not the Triforce of Courage obtained during the game, but a dummy item that's never used.
    TriforceCourage = 0x5D,
    /// Heart from Stylish Woman (Street Merchant & Big Cucco hearts are not GetItems)
    #[strum(serialize = "Heart")]
    Heart = 0x5E,
    /// Osfala's Sand Rod, functionally the same as Rental Sand Rod but in vanilla you lose it immediately
    ItemRentalSandRodFirst = 0x5F,
    /// Golden Bee (functionally identical to [`Item::GoldenBee`])
    GoldenBeeForSale = 0x60,
}

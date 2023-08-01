use strum::{AsRefStr, IntoStaticStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, AsRefStr, IntoStaticStr)]
#[repr(u16)]
pub enum Item {
    Empty = 0x00,
    #[strum(serialize = "Small Key")]
    KeySmall = 0x01,
    #[strum(serialize = "Boss Key")]
    KeyBoss = 0x02,
    Compass = 0x03,
    #[strum(serialize = "Heart Container")]
    HeartContainer = 0x04,
    #[strum(serialize = "Red Rupee")]
    RupeeR = 0x05,
    #[strum(serialize = "Green Rupee")]
    RupeeG = 0x06,
    #[strum(serialize = "Blue Rupee")]
    RupeeB = 0x07,
    #[strum(serialize = "Heart Piece")]
    HeartPiece = 0x08,
    #[strum(serialize = "Ice Rod")]
    ItemIceRod = 0x09,
    #[strum(serialize = "Sand Rod")]
    ItemSandRod = 0x0A,
    #[strum(serialize = "Tornado Rod")]
    ItemTornadeRod = 0x0B,
    #[strum(serialize = "Bombs")]
    ItemBomb = 0x0C,
    #[strum(serialize = "Fire Rod")]
    ItemFireRod = 0x0D,
    #[strum(serialize = "Hookshot")]
    ItemHookShot = 0x0E,
    #[strum(serialize = "Boomerang")]
    ItemBoomerang = 0x0F,
    #[strum(serialize = "Hammer")]
    ItemHammer = 0x10,
    #[strum(serialize = "Bow")]
    ItemBow = 0x11,
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
    #[strum(serialize = "Red Potion")]
    ItemPotShopRed = 0x14,
    #[strum(serialize = "Blue Potion")]
    ItemPotShopBlue = 0x15,
    #[strum(serialize = "Smooth Gem")]
    ItemStoneBeauty = 0x16,
    #[strum(serialize = "Pendant of Power")]
    PendantPower = 0x17,
    #[strum(serialize = "Pendant of Wisdom")]
    PendantWisdom = 0x18,
    #[strum(serialize = "Pendant of Courage")]
    PendantCourage = 0x19,
    #[strum(serialize = "Lamp")]
    ItemKandelaar = 0x1A,
    #[strum(serialize = "Sword")]
    ItemSword = 0x1C,
    #[strum(serialize = "Zora's Flippers")]
    ItemMizukaki = 0x1F,
    #[strum(serialize = "Ravio's Bracelet")]
    RingHekiga = 0x2B,
    #[strum(serialize = "Bell")]
    ItemBell = 0x2C,
    #[strum(serialize = "Gold Rupee")]
    RupeeGold = 0x2D,
    #[strum(serialize = "Silver Rupee")]
    RupeeSilver = 0x2E,
    #[strum(serialize = "Power Glove")]
    PowerGlove = 0x2F,
    #[strum(serialize = "Net")]
    ItemInsectNet = 0x30,
    #[strum(serialize = "Maiamai")]
    Kinsta = 0x32,
    #[strum(serialize = "Bee Badge")]
    BadgeBee = 0x33,
    #[strum(serialize = "Golden Bee")]
    GoldenBee = 0x34,
    #[strum(serialize = "Hint Glasses")]
    HintGlasses = 0x35,
    #[strum(serialize = "Scoot Fruit")]
    EscapeFruit = 0x36,
    #[strum(serialize = "Foul Fruit")]
    StopFruit = 0x37,
    #[strum(serialize = "Bee")]
    Bee = 0x38,
    #[strum(serialize = "Fairy")]
    Fairy = 0x39,
    #[strum(serialize = "Monster Tail")]
    LiverBlue = 0x3A,
    #[strum(serialize = "Monster Guts")]
    LiverPurple = 0x3B,
    #[strum(serialize = "Monster Horn")]
    LiverYellow = 0x3C,
    #[strum(serialize = "Charm")]
    ZeldaAmulet = 0x3E,
    #[strum(serialize = "Progrssive Mail")]
    Clothes = 0x3F,
    #[strum(serialize = "Hylian Shield")]
    HyruleShield = 0x41,
    #[strum(serialize = "Master Ore")]
    OreYellow = 0x42,
    #[strum(serialize = "Master Ore")]
    OreGreen = 0x43,
    #[strum(serialize = "Master Ore")]
    OreBlue = 0x44,
    #[strum(serialize = "Stamina Scroll")]
    GanbariPowerUp = 0x45,
    Pouch = 0x46,
    #[strum(serialize = "Pegasus Boots")]
    DashBoots = 0x47,
    #[strum(serialize = "Master Ore")]
    OreRed = 0x48,
    #[strum(serialize = "Letter in a Bottle")]
    MessageBottle = 0x49,
    #[strum(serialize = "Premium Milk")]
    MilkMatured = 0x4A,
    #[strum(serialize = "Purple Potion")]
    ItemPotShopPurple = 0x4B,
    #[strum(serialize = "Yellow Potion")]
    ItemPotShopYellow = 0x4C,
    #[strum(serialize = "Great Spin")]
    SpecialMove = 0x56,
    Milk = 0x57,
    #[strum(serialize = "Energy Potion")]
    GanbariTubo = 0x5A,
    #[strum(serialize = "Purple Rupee")]
    RupeePurple = 0x5B,
    #[strum(serialize = "Bow of Light")]
    ItemBowLight = 0x5C,
    TriforceCourage = 0x5D,
    Heart = 0x5E,
    GoldenBeeForSale = 0x60,

    // IDs below are fake, to-be-added as new GetItems
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
}

impl Item {
    pub fn big_chest(&self) -> bool {
        matches!(
            self,
            Self::KeySmall
                | Self::KeyBoss
                | Self::ItemIceRod
                | Self::ItemSandRod
                | Self::ItemTornadeRod
                | Self::ItemBomb
                | Self::ItemFireRod
                | Self::ItemHookShot
                | Self::ItemBoomerang
                | Self::ItemHammer
                | Self::ItemBow
                | Self::ItemShield
                | Self::HyruleShield
                | Self::ItemBottle
                | Self::ItemStoneBeauty
                | Self::PendantPower
                | Self::PendantWisdom
                | Self::PendantCourage
                | Self::ZeldaAmulet
                | Self::ItemKandelaar
                | Self::ItemSword
                | Self::ItemMizukaki
                | Self::RingHekiga
                | Self::ItemBell
                | Self::PowerGlove
                | Self::ItemInsectNet
                | Self::BadgeBee
                | Self::GoldenBee
                | Self::HintGlasses
                | Self::EscapeFruit
                | Self::StopFruit
                | Self::Clothes
                | Self::OreYellow
                | Self::OreGreen
                | Self::OreBlue
                | Self::OreRed
                | Self::GanbariPowerUp
                | Self::Pouch
                | Self::DashBoots
                | Self::MessageBottle
                | Self::MilkMatured
                | Self::SpecialMove
                | Self::ItemBowLight
                | Self::SageGulley
                | Self::SageOren
                | Self::SageSeres
                | Self::SageOsfala
                | Self::SageImpa
                | Self::SageIrene
                | Self::SageRosso
        )
    }

    pub fn is_sage(&self) -> bool {
        matches!(
            self,
            Self::SageGulley
                | Self::SageOren
                | Self::SageSeres
                | Self::SageOsfala
                | Self::SageImpa
                | Self::SageIrene
                | Self::SageRosso
        )
    }
}

impl From<game::Item> for Item {
    fn from(item: game::Item) -> Self {
        match item {
            game::Item::Empty => Self::Empty,
            game::Item::KeySmall => Self::KeySmall,
            game::Item::KeyBoss => Self::KeyBoss,
            game::Item::Compass => Self::Compass,
            game::Item::HeartContainer => Self::HeartContainer,
            game::Item::RupeeR => Self::RupeeR,
            game::Item::RupeeG => Self::RupeeG,
            game::Item::RupeeB => Self::RupeeB,
            game::Item::HeartPiece => Self::HeartPiece,
            game::Item::ItemIceRod => Self::ItemIceRod,
            game::Item::ItemSandRod => Self::ItemSandRod,
            game::Item::ItemTornadeRod => Self::ItemTornadeRod,
            game::Item::ItemBomb => Self::ItemBomb,
            game::Item::ItemFireRod => Self::ItemFireRod,
            game::Item::ItemHookShot => Self::ItemHookShot,
            game::Item::ItemBoomerang => Self::ItemBoomerang,
            game::Item::ItemHammer => Self::ItemHammer,
            game::Item::ItemBow => Self::ItemBow,
            game::Item::ItemShield => Self::ItemShield,
            game::Item::ItemBottle => Self::ItemBottle,
            game::Item::ItemPotShopRed => Self::ItemPotShopRed,
            game::Item::ItemPotShopBlue => Self::ItemPotShopBlue,
            game::Item::ItemStoneBeauty => Self::ItemStoneBeauty,
            game::Item::PendantPower => Self::PendantPower,
            game::Item::PendantWisdom => Self::PendantWisdom,
            game::Item::PendantCourage => Self::PendantCourage,
            game::Item::ItemKandelaar => Self::ItemKandelaar,
            game::Item::ItemSwordLv1 => Self::ItemSword,
            game::Item::ItemSwordLv2 => Self::ItemSword,
            game::Item::ItemSwordLv3 => Self::ItemSword,
            game::Item::ItemSwordLv4 => Self::ItemSword,
            game::Item::ItemMizukaki => Self::ItemMizukaki,
            game::Item::ItemRentalIceRod => Self::ItemIceRod,
            game::Item::ItemRentalSandRod => Self::ItemSandRod,
            game::Item::ItemRentalTornadeRod => Self::ItemTornadeRod,
            game::Item::ItemRentalBomb => Self::ItemBomb,
            game::Item::ItemRentalFireRod => Self::ItemFireRod,
            game::Item::ItemRentalHookShot => Self::ItemHookShot,
            game::Item::ItemRentalBoomerang => Self::ItemBoomerang,
            game::Item::ItemRentalHammer => Self::ItemHammer,
            game::Item::ItemRentalBow => Self::ItemBow,
            game::Item::ItemRentalShield => Self::ItemShield,
            game::Item::RingRental => Self::RingHekiga,
            game::Item::RingHekiga => Self::RingHekiga,
            game::Item::ItemBell => Self::ItemBell,
            game::Item::RupeeGold => Self::RupeeGold,
            game::Item::RupeeSilver => Self::RupeeSilver,
            game::Item::PowerGlove => Self::PowerGlove,
            game::Item::ItemInsectNet => Self::ItemInsectNet,
            game::Item::PowerfulGlove => Self::PowerGlove,
            game::Item::Kinsta => Self::Kinsta,
            game::Item::BadgeBee => Self::BadgeBee,
            game::Item::GoldenBee => Self::GoldenBee,
            game::Item::HintGlasses => Self::HintGlasses,
            game::Item::EscapeFruit => Self::EscapeFruit,
            game::Item::StopFruit => Self::StopFruit,
            game::Item::Bee => Self::Bee,
            game::Item::Fairy => Self::Fairy,
            game::Item::LiverBlue => Self::LiverBlue,
            game::Item::LiverPurple => Self::LiverPurple,
            game::Item::LiverYellow => Self::LiverYellow,
            game::Item::PackageSword => Self::ItemSword,
            game::Item::ZeldaAmulet => Self::ZeldaAmulet,
            game::Item::ClothesBlue => Self::Clothes,
            game::Item::ClothesRed => Self::Clothes,
            game::Item::HyruleShield => Self::HyruleShield,
            game::Item::OreYellow => Self::OreYellow,
            game::Item::OreGreen => Self::OreGreen,
            game::Item::OreBlue => Self::OreBlue,
            game::Item::GanbariPowerUp => Self::GanbariPowerUp,
            game::Item::Pouch => Self::Pouch,
            game::Item::DashBoots => Self::DashBoots,
            game::Item::OreRed => Self::OreRed,
            game::Item::MessageBottle => Self::MessageBottle,
            game::Item::MilkMatured => Self::MilkMatured,
            game::Item::ItemPotShopPurple => Self::ItemPotShopPurple,
            game::Item::ItemPotShopYellow => Self::ItemPotShopYellow,
            game::Item::ItemIceRodLv2 => Self::ItemIceRod,
            game::Item::ItemSandRodLv2 => Self::ItemSandRod,
            game::Item::ItemTornadeRodLv2 => Self::ItemTornadeRod,
            game::Item::ItemBombLv2 => Self::ItemBomb,
            game::Item::ItemFireRodLv2 => Self::ItemFireRod,
            game::Item::ItemHookShotLv2 => Self::ItemHookShot,
            game::Item::ItemBoomerangLv2 => Self::ItemBoomerang,
            game::Item::ItemHammerLv2 => Self::ItemHammer,
            game::Item::ItemBowLv2 => Self::ItemBow,
            game::Item::SpecialMove => Self::SpecialMove,
            game::Item::Milk => Self::Milk,
            game::Item::ItemKandelaarLv2 => Self::ItemKandelaar,
            game::Item::ItemInsectNetLv2 => Self::ItemInsectNet,
            game::Item::GanbariTubo => Self::GanbariTubo,
            game::Item::RupeePurple => Self::RupeePurple,
            game::Item::ItemBowLight => Self::ItemBowLight,
            game::Item::TriforceCourage => Self::TriforceCourage,
            game::Item::Heart => Self::Heart,
            game::Item::ItemRentalSandRodFirst => Self::ItemSandRod,
            game::Item::GoldenBeeForSale => Self::GoldenBeeForSale,
        }
    }
}

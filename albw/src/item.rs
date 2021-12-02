use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::{actors::Actor, int_map, Error, Game, Result};

int_map! {
/// An in-game item.
Item(u16) {
    KeySmall = 0x01,
    KeyBoss = 0x02,
    Compass = 0x03,
    HeartContainer = 0x04,
    RupeeR = 0x05,
    RupeeG = 0x06,
    RupeeB = 0x07,
    HeartPiece = 0x08,
    ItemIceRod = 0x09,
    ItemSandRod = 0x0A,
    ItemTornadeRod = 0x0B,
    ItemBomb = 0x0C,
    ItemFireRod = 0x0D,
    ItemHookShot = 0x0E,
    ItemBoomerang = 0x0F,
    ItemHammer = 0x10,
    ItemBow = 0x11,
    ItemShield = 0x12,
    ItemBottle = 0x13,
    ItemPotShopRed = 0x14,
    ItemPotShopBlue = 0x15,
    ItemStoneBeauty = 0x16, // Smooth Gem
    PendantPower = 0x17,
    PendantWisdom = 0x18,
    PendantCourage = 0x19,
    ItemKandelaar = 0x1A, // Lamp
    ItemSwordLv1 = 0x1B,
    ItemSwordLv2 = 0x1C,
    ItemSwordLv3 = 0x1D,
    ItemSwordLv4 = 0x1E,
    ItemMizukaki = 0x1F, // Zora's Flippers
    ItemRentalIceRod = 0x20,
    ItemRentalSandRod = 0x21,
    ItemRentalTornadeRod = 0x22,
    ItemRentalBomb = 0x23,
    ItemRentalFireRod = 0x24,
    ItemRentalHookShot = 0x25,
    ItemRentalBoomerang = 0x26,
    ItemRentalHammer = 0x27,
    ItemRentalBow = 0x28,
    ItemRentalShield = 0x29,
    RingRental = 0x2A, // Ravio's Bracelet
    RingHekiga = 0x2B,
    ItemBell = 0x2C,
    RupeeGold = 0x2D,
    RupeeSilver = 0x2E,
    PowerGlove = 0x2F,
    ItemInsectNet = 0x30,
    PowerfulGlove = 0x31,
    Kinsta = 0x32, // Maiamai
    BadgeBee = 0x33,
    GoldenBee = 0x34,
    HintGlasses = 0x35,
    EscapeFruit = 0x36, // Scoot Fruit
    StopFruit = 0x37, // Foul Fruit
    Bee = 0x38,
    Fairy = 0x39,
    LiverBlue = 0x3A, // Monster Tail
    LiverPurple = 0x3B, // Monster Guts
    LiverYellow = 0x3C, // Monster Horn
    PackageSword = 0x3D, // Captain's Sword
    ZeldaAmulet = 0x3E,
    ClothesBlue = 0x3F,
    ClothesRed = 0x40,
    HyruleShield = 0x41,
    OreYellow = 0x42,
    OreGreen = 0x43,
    OreBlue = 0x44,
    GanbariPowerUp = 0x45, // Stamina Scroll
    Pouch = 0x46,
    DashBoots = 0x47,
    OreRed = 0x48,
    MessageBottle = 0x49,
    MilkMatured = 0x4A, // Premium Milk
    ItemPotShopPurple = 0x4B,
    ItemPotShopYellow = 0x4C,
    ItemIceRodLv2 = 0x4D,
    ItemSandRodLv2 = 0x4E,
    ItemTornadeRodLv2 = 0x4F,
    ItemBombLv2 = 0x50,
    ItemFireRodLv2 = 0x51,
    ItemHookShotLv2 = 0x52,
    ItemBoomerangLv2 = 0x53,
    ItemHammerLv2 = 0x54,
    ItemBowLv2 = 0x55,
    SpecialMove = 0x56, // Great Spin
    Milk = 0x57,
    ItemKandelaarLv2 = 0x58, // Super Lamp
    ItemInsectNetLv2 = 0x59,
    GanbariTubo = 0x5A, // Stamina Potion
    RupeePurple = 0x5B,
    ItemBowLight = 0x5C,
    TriforceCourage = 0x5D,
    Heart = 0x5E,
    ItemRentalSandRodFirst = 0x5F, // Osfala's Sand Rod
    GoldenBeeForSale = 0x60,
}}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetItem(
    String,
    String,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    f32,
    String,
    String,
    String,
    i32,
    i32,
    i32,
    i32,
    i32,
);

impl GetItem {
    pub fn actor(&self, game: &Game) -> Result<Actor> {
        game.get_item_actor(self.actor_name()?)
    }

    pub fn actor_name(&self) -> Result<&str> {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Actor/([A-Za-z]+)\.bch$").unwrap());
        REGEX
            .captures(&self.1)
            .and_then(|captures| captures.get(1))
            .map(|match_| match_.as_str())
            .ok_or_else(|| Error::new(format!("Invalid actor name: '{}'", &self.1)))
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

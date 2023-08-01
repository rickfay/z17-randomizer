use std::collections::BTreeMap;

use game::{
    world::{Area, Group as GroupId, LocationKey},
    Item::{self, *},
};
use log::debug;
use serde::{ser::SerializeMap, Serialize, Serializer};
use strum::AsRefStr;

use hints::Hints;
pub use settings::Settings;

pub mod filler_item;
pub mod hints;
pub mod settings;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
}

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

#[derive(Debug, Serialize)]
pub struct Mod {
    pub name: String,
    pub hash: Option<String>,
    pub settings: Settings,
    pub layout: Layout,
    pub hints: Hints,
}

/// A world layout for the patcher.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Layout {
    #[serde(rename = "Hyrule", serialize_with = "serialize_world")]
    hyrule: Group,
    #[serde(rename = "Lorule", serialize_with = "serialize_world")]
    lorule: Group,
    #[serde(rename = "Dungeons", serialize_with = "serialize_world")]
    dungeons: Group,
}

impl Layout {
    fn group(&self, id: GroupId) -> &Group {
        match id {
            GroupId::Hyrule => &self.hyrule,
            GroupId::Lorule => &self.lorule,
            GroupId::Dungeons => &self.dungeons,
        }
    }

    fn group_mut(&mut self, id: GroupId) -> &mut Group {
        match id {
            GroupId::Hyrule => &mut self.hyrule,
            GroupId::Lorule => &mut self.lorule,
            GroupId::Dungeons => &mut self.dungeons,
        }
    }

    fn get_area_mut(&mut self, area: Area) -> &mut BTreeMap<&'static str, Item> {
        self.group_mut(area.group()).entry(area.name()).or_insert_with(Default::default)
    }

    pub fn get(&self, key: &LocationKey) -> Option<Item> {
        let LocationKey { area, name } = key;
        self.group(area.group()).get(area.name()).and_then(|region| region.get(name).copied())
    }

    #[allow(unused)]
    fn find(&self, item: Item) -> Vec<&'static str> {
        todo!()
    }

    /// This just highlights why we need to redo [`Layout`]
    pub fn find_single(&self, find_item: Item) -> Option<(&'static str, &'static str)> {
        for (region_name, region) in &self.hyrule {
            for (loc_name, item) in region {
                if find_item.eq(item) {
                    return Some((region_name, loc_name));
                }
            }
        }

        for (region_name, region) in &self.lorule {
            for (loc_name, item) in region {
                if find_item.eq(item) {
                    return Some((region_name, loc_name));
                }
            }
        }

        for (region_name, region) in &self.dungeons {
            for (loc_name, item) in region {
                if find_item.eq(item) {
                    return Some((region_name, loc_name));
                }
            }
        }

        None
    }

    pub fn set(&mut self, location: LocationKey, item: Item) {
        let LocationKey { area: node, name } = location;
        self.get_area_mut(node).insert(name, item.normalize());
        debug!(
            "Placed {} in {}/{}",
            item.normalize().as_ref(),
            location.area.name(),
            location.name
        );
    }
}

pub type Group = BTreeMap<&'static str, BTreeMap<&'static str, Item>>;

fn serialize_world<S>(region: &Group, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    struct Wrap<'a>(&'a BTreeMap<&'static str, Item>);

    impl<'a> Serialize for Wrap<'a> {
        fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = ser.serialize_map(Some(self.0.len()))?;
            for (k, v) in self.0 {
                map.serialize_entry(k, item_to_str(v))?;
            }
            map.end()
        }
    }

    let mut map = ser.serialize_map(Some(region.len()))?;
    for (k, v) in region {
        map.serialize_entry(k, &Wrap(v))?;
    }
    map.end()
}

pub fn item_to_str(item: &Item) -> &'static str {
    match item {
        KeySmall => "Small Key",
        KeyBoss => "Big Key",
        Compass => "Compass",
        HeartContainer => "Heart Container",
        RupeeR => "Red Rupee",
        RupeeG => "Green Rupee",
        RupeeB => "Blue Rupee",
        HeartPiece => "Piece of Heart",
        ItemIceRod => "Ice Rod",
        ItemIceRodLv2 => "Nice Ice Rod",
        ItemSandRod => "Sand Rod",
        ItemSandRodLv2 => "Nice Sand Rod",
        ItemTornadeRod => "Tornado Rod",
        ItemTornadeRodLv2 => "Nice Tornado Rod",
        ItemBomb => "Bombs",
        ItemBombLv2 => "Nice Bombs",
        ItemFireRod => "Fire Rod",
        ItemFireRodLv2 => "Nice Fire Rod",
        ItemHookShot => "Hookshot",
        ItemHookShotLv2 => "Nice Hookshot",
        ItemBoomerang => "Boomerang",
        ItemBoomerangLv2 => "Nice Boomerang",
        ItemHammer => "Hammer",
        ItemHammerLv2 => "Nice Hammer",
        ItemBow => "Bow",
        ItemBowLv2 => "Nice Bow",
        ItemShield => "Shield",
        ItemBottle => "Empty Bottle",
        ItemStoneBeauty => "Smooth Gem",
        ItemKandelaar => "Lamp",
        ItemKandelaarLv2 => "Super Lamp",
        ItemSwordLv1 => "Sword Upgrade",
        ItemSwordLv2 => "Sword Upgrade",
        ItemSwordLv3 => "Master Sword Lv2",
        ItemSwordLv4 => "Master Sword Lv3",
        ItemMizukaki => "Zora's Flippers",
        RingRental => "Bracelet Upgrade",
        RingHekiga => "Ravio's Bracelet",
        ItemBell => "Bell",
        RupeeGold => "Gold Rupee",
        RupeeSilver => "Silver Rupee",
        PowerGlove => "Strength Upgrade",
        ItemInsectNet => "Net",
        ItemInsectNetLv2 => "Super Net",
        Kinsta => "Lost Maiamai",
        BadgeBee => "Bee Badge",
        HintGlasses => "Hint Glasses",
        LiverBlue => "Monster Tail",
        LiverPurple => "Monster Guts",
        LiverYellow => "Monster Horn",
        ClothesBlue | ClothesRed => "Armor Upgrade",
        HyruleShield => "Hylian Shield",
        OreYellow => "Master Ore",
        OreGreen => "Master Ore",
        OreBlue => "Master Ore",
        GanbariPowerUp => "Stamina Scroll",
        Pouch => "Pouch",
        DashBoots => "Pegasus Boots",
        OreRed => "Master Ore",
        MessageBottle => "Letter in a Bottle",
        MilkMatured => "Premium Milk",
        SpecialMove => "Great Spin",
        GanbariTubo => "Energy Potion",
        RupeePurple => "Purple Rupee",
        ItemBowLight => "Bow of Light",
        Heart => "Heart",

        Empty => "Nothing",

        PendantPower => "Pendant of Power",
        PendantWisdom => "Pendant of Wisdom",
        ZeldaAmulet | PendantCourage => "Pendant of Courage Upgrade",

        SageGulley => "Sage Gulley",
        SageOren => "Sage Oren",
        SageSeres => "Sage Seres",
        SageOsfala => "Sage Osfala",
        SageImpa => "Sage Impa",
        SageIrene => "Sage Irene",
        SageRosso => "Sage Rosso",

        TriforceCourage => "Triforce of Courage",

        ItemPotShopRed => "Red Potion",
        ItemPotShopBlue => "Blue Potion",
        ItemPotShopPurple => "Purple Potion",
        ItemPotShopYellow => "Yellow Potion",

        EscapeFruit => "Scoot Fruit",
        StopFruit => "Foul Fruit",
        Bee => "Bee",
        GoldenBeeForSale => "Golden Bee",
        Fairy => "Fairy",
        Milk => "Milk",

        ItemRentalIceRod => "Rented Ice Rod",
        ItemRentalSandRod => "Rented Sand Rod",
        ItemRentalTornadeRod => "Rented Tornado Rod",
        ItemRentalBomb => "Rented Bomb Rod",
        ItemRentalFireRod => "Rented Fire Rod",
        ItemRentalHookShot => "Rented Hookshot",
        ItemRentalBoomerang => "Rented Boomerang",
        ItemRentalHammer => "Rented Hammer",
        ItemRentalBow => "Rented Bow",
        ItemRentalShield => "Rented Shield",
        ItemRentalSandRodFirst => "Rented Sand Rod (Osfala)",
        PowerfulGlove => "Titan's Mitt",
        GoldenBee => "Golden Bee",
        PackageSword => "Captain's Sword",
    }
}

pub trait ItemExt {
    fn normalize(self) -> Self;
    fn goes_in_csmc_large_chest(&self) -> bool;
    fn msbf_key(self) -> Result<Option<&'static str>>;

    // fn is_dungeon(&self) -> bool;
    // fn is_sword(&self) -> bool;
    // fn is_super(&self) -> bool;
    // fn is_ore(&self) -> bool;
}

impl ItemExt for Item {
    fn normalize(self) -> Self {
        match self {
            PackageSword | ItemSwordLv1 | ItemSwordLv3 | ItemSwordLv4 => ItemSwordLv2,
            ItemRentalIceRod => ItemIceRod,
            ItemRentalSandRod => ItemSandRod,
            ItemRentalTornadeRod => ItemTornadeRod,
            ItemRentalBomb => ItemBomb,
            ItemRentalFireRod => ItemFireRod,
            ItemRentalHookShot => ItemHookShot,
            ItemRentalBoomerang => ItemBoomerang,
            ItemRentalHammer => ItemHammer,
            ItemRentalBow => ItemBow,
            PowerfulGlove => PowerGlove,
            ClothesRed => ClothesBlue,
            // RingRental => RingHekiga,
            ItemKandelaarLv2 => ItemKandelaar,
            ItemInsectNetLv2 => ItemInsectNet,
            item => item,
        }
    }

    fn goes_in_csmc_large_chest(&self) -> bool {
        matches!(
            self,
            // Empty |
            KeySmall | KeyBoss |
            // Compass |
            // HeartContainer | HeartPiece |
            // RupeeR | RupeeG | RupeeB | RupeeGold | RupeeSilver | RupeePurple |
            ItemIceRod | ItemRentalIceRod | ItemIceRodLv2 |
            ItemSandRod | ItemRentalSandRod | ItemSandRodLv2 | ItemRentalSandRodFirst |
            ItemTornadeRod | ItemRentalTornadeRod | ItemTornadeRodLv2 |
            ItemBomb | ItemRentalBomb | ItemBombLv2 |
            ItemFireRod | ItemRentalFireRod | ItemFireRodLv2 |
            ItemHookShot | ItemRentalHookShot | ItemHookShotLv2 |
            ItemBoomerang | ItemRentalBoomerang | ItemBoomerangLv2 |
            ItemHammer | ItemRentalHammer | ItemHammerLv2 |
            ItemBow | ItemRentalBow | ItemBowLv2 |
            ItemShield | ItemRentalShield | HyruleShield |
            ItemBottle |
            // ItemPotShopRed | ItemPotShopBlue | ItemPotShopPurple | ItemPotShopYellow | Milk |
            ItemStoneBeauty |
            PendantPower | PendantWisdom | PendantCourage |
            ZeldaAmulet |
            ItemKandelaar | ItemKandelaarLv2 |
            ItemSwordLv1 | ItemSwordLv2 | ItemSwordLv3 | ItemSwordLv4 | PackageSword |
            ItemMizukaki |
            RingRental | RingHekiga |
            ItemBell |
            PowerGlove | PowerfulGlove |
            ItemInsectNet | ItemInsectNetLv2 |
            // Kinsta |
            BadgeBee |
            GoldenBee |
            // Bee | Fairy | GoldenBeeForSale |
            HintGlasses |
            EscapeFruit |
            StopFruit |
            // LiverBlue | LiverPurple | LiverYellow |
            ClothesBlue | ClothesRed |
            OreYellow | OreGreen | OreBlue | OreRed |
            GanbariPowerUp |
            // GanbariTubo |
            Pouch |
            DashBoots |
            MessageBottle | MilkMatured |
            SpecialMove |
            ItemBowLight |
            // TriforceCourage |
            // Heart |
            SageGulley | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso
        )
    }

    fn msbf_key(self) -> Result<Option<&'static str>> {
        match self {
            SageGulley => Ok(Some(MsbfKey::Dark.as_ref())),
            SageOren => Ok(Some(MsbfKey::Water.as_ref())),
            SageSeres => Ok(Some(MsbfKey::Dokuro.as_ref())),
            SageOsfala => Ok(Some(MsbfKey::Hagure.as_ref())),
            SageIrene => Ok(Some(MsbfKey::Sand.as_ref())),
            SageRosso => Ok(Some(MsbfKey::Ice.as_ref())),
            SageImpa => Ok(None), // Impa special
            PendantPower | PendantWisdom | PendantCourage | ZeldaAmulet => Ok(None),
            _ => Err(Error::new("")),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, AsRefStr)]
pub enum MsbfKey {
    Castle,
    CatchInsect,
    Cave,
    CaveDark10,
    cl_Church_UG,
    CrossBattle,
    CrossBoard,
    CrossForceTalk,
    CrossOldMan,
    Dark,
    Dokuro,
    DoorHouse,
    E3_flow,
    East,
    Ending,
    FieldDark_00_GoldenBeeShop,
    FieldDark_05_GameTower,
    FieldDark_0F_Namazu,
    FieldDark_13_Sinpu,
    FieldDark_14_Danpei,
    FieldDark_16_HagureHouse,
    FieldDark_16_MagicShop,
    FieldDark_17_NpcHinox,
    FieldDark_18_BakudanTouzoku,
    FieldDark_18_BoxManDark,
    FieldDark_18_ItemShop,
    FieldDark_1A_FortuneGirlUra,
    FieldDark_1B_Bakudanya,
    FieldDark_1B_Hilda,
    FieldDark_1E_Sennyukun,
    FieldDark_28_Minigame,
    FieldDark_29_BakudanShop,
    FieldDark_29_HappyFairy,
    FieldDark_2A_GameMaster,
    FieldDark_2C_RaviosDiary,
    FieldDark_33_Daibakudankabe,
    FieldDark_33_Touzoku,
    FieldDark_35_ItemShop,
    FieldDark_35_Kame,
    FieldDark_3A_CrazyMan,
    FieldDark_Tennokoe,
    FieldLight_00_JyohoShop,
    FieldLight_00_Mayoinomori,
    FieldLight_02_KikoriMan,
    FieldLight_03_Kanban,
    FieldLight_05_Climber,
    FieldLight_0A_Kanban,
    FieldLight_0F_Kanban,
    FieldLight_0F_Zora,
    FieldLight_11_FortuneGirl,
    FieldLight_11_Maple,
    FieldLight_12_Maple,
    FieldLight_12_SignBoard,
    FieldLight_13_Danpei,
    FieldLight_13_Medium,
    FieldLight_13_SignBoard,
    FieldLight_13_Sinpu,
    FieldLight_13_Sister,
    FieldLight_14_Danpei,
    FieldLight_14_Maple,
    FieldLight_16_Ending,
    FieldLight_16_MagicShop,
    FieldLight_16_Obaba,
    FieldLight_16_SignBoard,
    FieldLight_17_Kanban,
    FieldLight_18_Bard,
    FieldLight_18_BoxMan,
    FieldLight_18_ClosedHouse,
    FieldLight_18_InsectNet,
    FieldLight_18_ItemShop,
    FieldLight_18_Kakarikoboy,
    FieldLight_18_KakarikoGirl,
    FieldLight_18_MaidSahasulala,
    FieldLight_18_MiddleLady,
    FieldLight_18_MiddleMan,
    FieldLight_18_MilkbarMaster,
    FieldLight_18_MilkbarSoldier,
    FieldLight_18_Rotenshonin,
    FieldLight_18_SahasPupil,
    FieldLight_18_SignBoard,
    FieldLight_18_Soldier,
    FieldLight_18_StandItem,
    FieldLight_18_Touzoku,
    FieldLight_1A_Maple,
    FieldLight_1A_SignBoard,
    FieldLight_1B_BlackSmithKid,
    FieldLight_1B_Commander,
    FieldLight_1B_Hekiga,
    FieldLight_1B_Impa,
    FieldLight_1B_Rakcha,
    FieldLight_1B_Sahasrahla,
    FieldLight_1B_Soldier,
    FieldLight_1B_Zelda,
    FieldLight_1E_Sahasrahla,
    FieldLight_22_BlackSmith,
    FieldLight_22_BlackSmithKid,
    FieldLight_22_BlackSmithWife,
    FieldLight_22_Dwarf,
    FieldLight_22_Maple,
    FieldLight_28_Minigame,
    FieldLight_29_Kokko,
    FieldLight_2A_BlacksmithKid,
    FieldLight_2A_BlacksmithWife,
    FieldLight_2B_AppleTree,
    FieldLight_2B_BlackSmithKid,
    FieldLight_2B_Maple,
    FieldLight_2C_BlackSmithKid,
    FieldLight_2C_GanbariTutorial,
    FieldLight_2C_Rental,
    FieldLight_2C_RentalItem,
    FieldLight_2C_SahasPupil,
    FieldLight_2C_Sahasrahla,
    FieldLight_2C_SignBoard,
    FieldLight_2C_Soldier,
    FieldLight_2D_Maple,
    FieldLight_2D_UnderBridgeStranger,
    FieldLight_2E_Maple,
    FieldLight_33_Douguya,
    FieldLight_35_Douguya,
    FieldLight_35_ItemShop,
    FieldLight_35_Kinsta,
    FieldLight_35_Marutakun,
    FieldLight_35_Zora,
    FieldLight_37_MessageBottle,
    FieldLight_BlacksmithWife,
    FieldLight_HyruleRace,
    FieldLight_Tennokoe,
    FieldLight_WarpEvent,
    FiledDark_22_BlackSmithUra,
    FiledDark_22_BlackSmithWifeUra,
    GameOver,
    Ganon,
    GirigiriGameTest,
    Hagure,
    Hera,
    HintGhost,
    Ice,
    IndoorDark1_ZoraQueen,
    IndoorDark2_Demo080,
    Kame,
    MessageBoard,
    MiniDungeon_FieldDark_2B,
    MiniDungeon_FieldLight_07,
    MiniDungeon_FieldLight_15,
    MiniDungeon_FieldLight_1E,
    MiniDungeon_FieldLight_32,
    MiniDungeon_FieldLight_33,
    NpcClimberTest,
    NpcHinox,
    NpcShadowLink,
    NpcStand,
    npcTest00,
    NpcTestIwata,
    NpcTownEtc,
    Sand,
    Telephone,
    test,
    ToRentalShopBoard,
    Water,
    Wind,
    yamazaki,
    yamazaki2,
}

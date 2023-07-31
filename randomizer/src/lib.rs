use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    fs::File,
    hash::{Hash, Hasher},
    io::{self, Write},
    ops::Deref,
};

use log::{debug, error, info};
use model::filler_item::{convert, FillerItem};
use patch::Patcher;
use path_absolutize::*;
use rand::{rngs::StdRng, SeedableRng};
use regions::Area;
use rom::{
    Item::{self, *},
    Rom,
};
use serde::{ser::SerializeMap, Serialize, Serializer};
use settings::Settings;

use crate::{
    constants::VERSION,
    hints::{formatting::*, Hints},
    metrics::Metrics,
    patch::msbf::MsbfKey,
    system::UserConfig,
};

pub mod constants;
mod entrance_rando;
mod filler;
mod filler_util;
mod hints;
mod item_pools;
mod legacy;
mod metrics;
pub mod model;
mod patch;
pub mod regions;
pub mod system;
#[rustfmt::skip]
mod world;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Rom(#[from] rom::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LocationKey {
    area: Area,
    name: &'static str,
}

impl LocationKey {
    pub const fn new(area: Area, name: &'static str) -> Self {
        Self { area, name }
    }

    pub fn world(&self) -> regions::Group {
        self.area.world()
    }

    pub fn region(&self) -> &'static str {
        self.area.name()
    }

    pub fn region_colorized(&self) -> String {
        self.area.name_colorized()
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// A world layout for the patcher.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Layout {
    #[serde(rename = "Hyrule", serialize_with = "serialize_world")]
    hyrule: World,
    #[serde(rename = "Lorule", serialize_with = "serialize_world")]
    lorule: World,
    #[serde(rename = "Dungeons", serialize_with = "serialize_world")]
    dungeons: World,
}

impl Layout {
    fn world(&self, id: regions::Group) -> &World {
        match id {
            regions::Group::Hyrule => &self.hyrule,
            regions::Group::Lorule => &self.lorule,
            regions::Group::Dungeons => &self.dungeons,
        }
    }

    fn world_mut(&mut self, id: regions::Group) -> &mut World {
        match id {
            regions::Group::Hyrule => &mut self.hyrule,
            regions::Group::Lorule => &mut self.lorule,
            regions::Group::Dungeons => &mut self.dungeons,
        }
    }

    fn get_area_mut(&mut self, area: Area) -> &mut BTreeMap<&'static str, Item> {
        self.world_mut(area.world()).entry(area.name()).or_insert_with(Default::default)
    }

    fn get(&self, key: &LocationKey) -> Option<Item> {
        let LocationKey { area, name } = key;
        self.world(area.world()).get(area.name()).and_then(|region| region.get(name).copied())
    }

    #[allow(unused)]
    fn find(&self, item: Item) -> Vec<&'static str> {
        todo!()
    }

    /// This just highlights why we need to redo [`Layout`]
    #[allow(unused)]
    fn find_single(&self, find_item: Item) -> Option<(&'static str, &'static str)> {
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
            item.normalize().as_str(),
            location.area.name(),
            location.name
        );
    }
}

pub type World = BTreeMap<&'static str, BTreeMap<&'static str, Item>>;

fn serialize_world<S>(region: &World, ser: S) -> Result<S::Ok, S::Error>
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

fn item_to_str(item: &Item) -> &'static str {
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

trait ItemExt {
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
            SageGulley => Ok(Some(MsbfKey::Dark)),
            SageOren => Ok(Some(MsbfKey::Water)),
            SageSeres => Ok(Some(MsbfKey::Dokuro)),
            SageOsfala => Ok(Some(MsbfKey::Hagure)),
            SageIrene => Ok(Some(MsbfKey::Sand)),
            SageRosso => Ok(Some(MsbfKey::Ice)),
            SageImpa => Ok(None), // Impa special
            PendantPower | PendantWisdom | PendantCourage | ZeldaAmulet => Ok(None),
            _ => Err(Error::new("")),
        }
    }
}

/// Align JSON Key-Values for readability
/// Can't find a decent library for this, so we're doing it manually
fn align_json_values(json: &mut String) -> Result<()> {
    const KEY_ALIGNMENT: usize = 56;
    let mut index_colon = 0;
    while index_colon < json.len() {
        let index_colon_opt = json[index_colon..].find(':');
        if index_colon_opt.is_none() {
            break;
        }
        index_colon += index_colon_opt.unwrap();
        if ['{', '['].contains(&json[index_colon..].chars().nth(2).unwrap()) {
            index_colon += 1;
            continue;
        }

        let index_prev_new_line = json[..index_colon].rfind('\n').ok_or_else(|| {
            Error::new(format!("Couldn't fine new line character before index: {}", index_colon))
        })?;
        let line_length_up_to_value = index_colon - index_prev_new_line;

        if KEY_ALIGNMENT < line_length_up_to_value {
            error!("Failed to write Spoiler Log");
            error!(
                "JSON Key Alignment value smaller than line length up to that point: {} < {}",
                KEY_ALIGNMENT, line_length_up_to_value
            );
            return Err(Error::new(format!(
                "Problem line: {}",
                &json[index_prev_new_line..index_colon]
            )));
        }

        let spaces_to_add = KEY_ALIGNMENT - line_length_up_to_value;

        json.insert_str(
            index_colon + 1,
            (0..spaces_to_add).map(|_| " ").collect::<String>().as_str(),
        );
        index_colon += 1;
    }
    Ok(())
}

#[derive(Serialize)]
pub struct SeedInfo<'s> {
    pub seed: u32,
    pub version: &'static str,
    pub hash: SeedHash,
    pub settings: &'s Settings,
    pub layout: Layout,
    pub metrics: Metrics,
    pub hints: Hints,
}

/// Main entry point to generate one ALBWR Seed.
pub fn generate_seed(
    seed: u32, settings: &Settings, user_config: &UserConfig, no_patch: bool, no_spoiler: bool,
) -> Result<()> {
    validate_settings(settings)?;

    let rng = &mut StdRng::seed_from_u64(seed as u64);
    let hash = SeedHash::new(seed, settings);

    info!("Hash:                           {}\n", hash.text_hash);
    settings.log_settings();

    let seed_info = &calculate_seed_info(seed, settings, hash, rng)?;
    patch_seed(seed_info, user_config, no_patch, no_spoiler)?;

    Ok(())
}

/// A hash used in-game to quickly verify that two players are playing the same seed.
///
/// The hash is calculated as `u64`, truncated to `u16` (5 digits), then converted to a Symbolic form that can be
/// displayed in-game as well as in the spoiler log.
pub struct SeedHash {
    item_hash: String,
    text_hash: String,
}

impl SeedHash {
    pub fn new(seed: u32, settings: &Settings) -> Self {
        // Calculate underlying Hash
        let mut hasher = DefaultHasher::new();
        (seed, settings, VERSION).hash(&mut hasher);
        let mut hash = hasher.finish() % 100_000;

        // Convert to Item Hash
        let hash_item_lut: Vec<(&String, &str)> = vec![
            (A_BUTTON.deref(), "(A)"),
            (B_BUTTON.deref(), "(B)"),
            (X_BUTTON.deref(), "(X)"),
            (Y_BUTTON.deref(), "(Y)"),
            (L_BUTTON.deref(), "(L)"),
            (R_BUTTON.deref(), "(R)"),
            (RAVIO.deref(), "(Ravio)"),
            (BOW.deref(), "(Bow)"),
            (BOMBS.deref(), "(Bombs)"),
            (FIRE_ROD.deref(), "(Fire Rod)"),
        ];

        const HASH_LEN: usize = 5;
        let mut digit = Vec::with_capacity(HASH_LEN);
        for _ in 0..HASH_LEN {
            digit.push(hash_item_lut.get((hash % 10) as usize).unwrap());
            hash /= 10;
        }

        let item_hash =
            format!("{} {} {} {} {}", digit[4].0, digit[3].0, digit[2].0, digit[1].0, digit[0].0);
        let text_hash =
            format!("{} {} {} {} {}", digit[4].1, digit[3].1, digit[2].1, digit[1].1, digit[0].1);

        Self { item_hash, text_hash }
    }
}

impl Serialize for SeedHash {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.text_hash.as_str())
    }
}

/// Validates the Settings to make sure the user hasn't made incompatible selections
fn validate_settings(settings: &Settings) -> Result<()> {
    // LC Requirement
    if !(0..=7).contains(&settings.logic.lc_requirement) {
        return Err(Error::new(format!(
            "Invalid Lorule Castle Requirement: \"{}\" was not between 0-7, inclusive.",
            settings.logic.lc_requirement
        )));
    }

    // Yuganon Requirement
    // if !(0..=7).contains(&settings.logic.yuganon_requirement) {
    //     fail!("Invalid Yuga Ganon Requirement: \"{}\" was not between 0-7, inclusive.", settings.logic.yuganon_requirement);
    // }

    if settings.logic.yuganon_requirement != settings.logic.lc_requirement {
        return Err(Error::new(format!(
            "Yuga Ganon Requirement: \"{}\" is different than Lorule Castle Requirement: \"{}\"\n\
        Different values for these settings are not yet supported!",
            settings.logic.yuganon_requirement, settings.logic.lc_requirement
        )));
    }

    // Swords
    if settings.logic.sword_in_shop && settings.logic.swordless_mode {
        return Err(Error::new(
            "The sword_in_shop and swordless_mode settings cannot both be enabled.",
        ));
    }

    // Assured Weapons
    if settings.logic.assured_weapon
        && (settings.logic.sword_in_shop || settings.logic.boots_in_shop)
    {
        return Err(Error::new(
            "The assured_weapon setting cannot be enabled when either sword_in_shop or boots_in_shop is also enabled."
        ));
    }

    Ok(())
}

pub type CheckMap = BTreeMap<String, Option<FillerItem>>;

fn calculate_seed_info<'s>(
    seed: u32, settings: &'s Settings, hash: SeedHash, rng: &mut StdRng,
) -> Result<SeedInfo<'s>> {
    println!();
    info!("Calculating Seed Info...");

    // Build World Graph
    let world_graph = &mut world::build_world_graph();
    let check_map = &mut filler::prefill_check_map(world_graph)?;
    let (mut progression, mut junk) = item_pools::get_item_pools(settings, rng);

    // Filler Algorithm
    let filled: Vec<(LocationKey, Item)> = filler::fill_all_locations_reachable(
        world_graph, check_map, &mut progression, &mut junk, settings, rng,
    )?;

    // Build legacy Layout object
    let mut layout = Layout::default();
    for (location_info, item) in filled {
        layout.set(location_info, item);
    }

    let metrics = metrics::calculate_metrics(world_graph, check_map, settings)?;
    let hints = hints::generate_hints(world_graph, check_map, settings, rng)?;

    Ok(SeedInfo { seed, version: VERSION, hash, settings, layout, metrics, hints })
}

pub fn patch_seed(
    seed_info: &SeedInfo, user_config: &UserConfig, no_patch: bool, no_spoiler: bool,
) -> Result<()> {
    println!();

    if !no_patch {
        info!("Starting Patch Process...");

        let game = Rom::load(user_config.rom()).map_err(|err| Error::new(err.to_string()))?;
        let mut patcher = Patcher::new(game)?;

        info!("ROM Loaded.\n");

        patcher.patch_locations(&seed_info.layout, seed_info.settings)?;
        let patches = patcher.prepare(seed_info)?;
        patches.dump(user_config.output())?;
    }
    if !no_spoiler {
        let path = user_config.output().join(format!("{:0>10}_spoiler.json", seed_info.seed));
        info!("Writing Spoiler Log to:         {}", &path.absolutize()?.display());

        //let spoiler = Spoiler::from(seed_info);

        let mut serialized = serde_json::to_string_pretty(&seed_info).unwrap();
        align_json_values(&mut serialized)?;

        write!(File::create(path)?, "{}", serialized).expect("Could not write the spoiler log.");
    }
    Ok(())
}

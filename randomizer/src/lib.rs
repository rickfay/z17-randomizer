use {
    crate::{
        filler::fill_stuff,
        model::metrics::Metrics,
        patch::msbf::MsbfKey,
        settings::settings::Settings,
        system::{Paths, System},
    },
    albw::{
        Game,
        Item::{self, *},
    },
    linked_hash_map::LinkedHashMap,
    log::{debug, error, info},
    model::filler_item::{convert, FillerItem},
    patch::Patcher,
    path_absolutize::*,
    regions::Subregion,
    serde::{ser::SerializeMap, Serialize, Serializer},
    std::{
        collections::BTreeMap,
        error::Error as StdError,
        fs::File,
        io::{self, stdin, stdout, Read, Write},
    },
};

pub mod cli;
pub mod constants;
mod entrance_rando;
mod filler;
mod filler_util;
mod item_pools;
mod legacy;
pub mod model;
mod patch;
pub mod regions;
pub mod settings;
mod system;
#[rustfmt::skip]
mod world;

/**
 * Shuts down the program in a controlled fashion:
 * - Displays an error message (optional)
 * - Pauses execution of the CLI
 * - Terminates with exit code 1.
 */
#[macro_export]
macro_rules! fail {
    (target: $target:expr, $($arg:tt)+) => ({
        log::error!(target: $target, $($arg)+);
        crate::pause();
        std::process::exit(1);
    });
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        crate::pause();
        std::process::exit(1);
    });
    () => ({
        crate::pause();
        std::process::exit(1);
    });
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...\n").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    fn game<S>(err: S) -> Self
    where
        S: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { kind: ErrorKind::Game, inner: err.into() }
    }

    fn io<S>(err: S) -> Self
    where
        S: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { kind: ErrorKind::Io, inner: err.into() }
    }

    /// Gets the type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Converts this error into its inner value.
    pub fn into_inner(self) -> Box<dyn StdError + Send + Sync + 'static> {
        self.inner
    }
}

impl From<albw::Error> for Error {
    fn from(err: albw::Error) -> Self {
        let kind = match err.kind() {
            albw::ErrorKind::Io => ErrorKind::Io,
            albw::ErrorKind::Rom => ErrorKind::Game,
        };
        Self { kind, inner: err.into_inner() }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self { kind: ErrorKind::Io, inner: err.into() }
    }
}

impl From<system::Error> for Error {
    fn from(err: system::Error) -> Self {
        Self { kind: ErrorKind::Sys, inner: err.into() }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Sys,
    Game,
    Io,
}

pub type Seed = u32;

#[derive(Debug)]
pub struct Hash(u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LocationInfo {
    subregion: &'static Subregion,
    name: &'static str,
}

impl LocationInfo {
    pub const fn new(subregion: &'static Subregion, name: &'static str) -> Self {
        Self { subregion, name }
    }

    pub fn world(&self) -> regions::World {
        self.subregion.world()
    }

    pub fn region(&self) -> &'static str {
        self.subregion.name()
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
    fn world(&self, id: regions::World) -> &World {
        match id {
            regions::World::Hyrule => &self.hyrule,
            regions::World::Lorule => &self.lorule,
            regions::World::Dungeons => &self.dungeons,
        }
    }

    fn world_mut(&mut self, id: regions::World) -> &mut World {
        match id {
            regions::World::Hyrule => &mut self.hyrule,
            regions::World::Lorule => &mut self.lorule,
            regions::World::Dungeons => &mut self.dungeons,
        }
    }

    fn get_node_mut(&mut self, node: &'static Subregion) -> &mut BTreeMap<&'static str, Item> {
        self.world_mut(node.world()).entry(node.name()).or_insert_with(Default::default)
    }

    fn get(&self, location: &LocationInfo) -> Option<Item> {
        let LocationInfo { subregion: node, name } = location;
        self.world(node.world()).get(node.name()).and_then(|region| region.get(name).copied())
    }

    pub fn set(&mut self, location: LocationInfo, item: Item) {
        let LocationInfo { subregion: node, name } = location;
        self.get_node_mut(node).insert(name, item.normalize());
        debug!(
            "Placed {} in {}/{}",
            item.normalize().as_str(),
            location.subregion.name(),
            location.name
        );
    }
}

pub type World = LinkedHashMap<&'static str, BTreeMap<&'static str, Item>>;

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
        ItemBottle => "Bottle",
        ItemStoneBeauty => "Smooth Gem",
        ItemKandelaar => "Lamp",
        ItemKandelaarLv2 => "Super Lamp",
        ItemSwordLv1 => "Progressive Sword",
        ItemSwordLv2 => "Progressive Sword",
        ItemSwordLv3 => "Master Sword Lv2",
        ItemSwordLv4 => "Master Sword Lv3",
        ItemMizukaki => "Flippers",
        RingRental => "Progressive Bracelet",
        RingHekiga => "Ravio's Bracelet",
        ItemBell => "Bell",
        RupeeGold => "Gold Rupee",
        RupeeSilver => "Silver Rupee",
        PowerGlove => "Progressive Glove",
        ItemInsectNet => "Net",
        ItemInsectNetLv2 => "Super Net",
        Kinsta => "Lost Maiamai",
        BadgeBee => "Bee Badge",
        HintGlasses => "Hint Glasses",
        LiverBlue => "Monster Tail",
        LiverPurple => "Monster Guts",
        LiverYellow => "Monster Horn",
        ClothesBlue | ClothesRed => "Progressive Mail",
        HyruleShield => "Hylian Shield",
        OreYellow => "Master Ore",
        OreGreen => "Master Ore",
        OreBlue => "Master Ore",
        GanbariPowerUp => "Stamina Scroll",
        Pouch => "Pouch",
        DashBoots => "Pegasus Boots",
        OreRed => "Master Ore",
        MessageBottle => "Message in a Bottle",
        MilkMatured => "Premium Milk",
        SpecialMove => "Great Spin",
        GanbariTubo => "Stamina Scroll",
        RupeePurple => "Purple Rupee",
        ItemBowLight => "Bow of Light",
        Heart => "Heart",

        Empty => "Empty",

        PendantPower => "Pendant of Power",
        PendantWisdom => "Pendant of Wisdom",
        ZeldaAmulet | PendantCourage => "Progressive Pendant of Courage",

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
    fn msbf_key(self) -> Option<&'static str>;

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

    fn msbf_key(self) -> Option<&'static str> {
        match self {
            SageGulley => Some(MsbfKey::Dark),
            SageOren => Some(MsbfKey::Water),
            SageSeres => Some(MsbfKey::Dokuro),
            SageOsfala => Some(MsbfKey::Hagure),
            SageIrene => Some(MsbfKey::Sand),
            SageRosso => Some(MsbfKey::Ice),
            SageImpa => None, // Impa special
            PendantPower | PendantWisdom | PendantCourage | ZeldaAmulet => None,
            _ => fail!(),
        }
    }
}

/// A log of seed info and item placements
#[derive(Debug, Serialize)]
pub struct Spoiler<'settings> {
    version: &'settings str,
    seed: Seed,
    settings: &'settings Settings,
    layout: Layout,
    metrics: Metrics,
}

impl<'settings> Spoiler<'settings> {
    pub fn new(
        version: &'settings str, seed: Seed, settings: &'settings Settings, layout: Layout,
        metrics: Metrics,
    ) -> Self {
        Self { version, seed, settings, layout, metrics }
    }

    pub fn patch(self, paths: Paths, patch: bool, spoiler: bool, hints: bool) -> Result<()> {
        if patch {
            info!("Starting Patch Process...");

            let game = Game::load(paths.rom())?;
            let mut patcher = Patcher::new(game)?;

            info!("ROM Loaded.\n");

            regions::patch(&mut patcher, &self.layout, self.settings)?;
            let patches = patcher.prepare(&self.layout, self.settings)?;
            patches.dump(paths.output())?;
        }
        if spoiler {
            let path = paths.output().join(format!("{:0>10}_spoiler.json", self.seed));
            info!("Writing Spoiler Log to:         {}", &path.absolutize()?.display());

            let mut serialized = serde_json::to_string_pretty(&self).unwrap();
            align_json_values(&mut serialized);

            write!(File::create(path)?, "{}", serialized)
                .expect("Could not write the spoiler log.");
        }
        if hints {
            let path = paths.output().join(format!("{:0>10}_hints.json", self.seed));
            info!("Writing Hints to:               {}", &path.absolutize()?.display());

            let mut serialized = serde_json::to_string_pretty(&self.metrics.get_hints()).unwrap();
            align_json_values(&mut serialized);

            write!(File::create(path)?, "{}", serialized).expect("Could not write the hints file.");
        }
        Ok(())
    }
}

/// Align JSON Key-Values for readability
/// Fuck it I can't find a decent library for this so we're doing it manually
fn align_json_values(json: &mut String) {
    const KEY_ALIGNMENT: usize = 56;
    let mut index_colon = 0;
    while index_colon < json.len() {
        let index_colon_opt = json[index_colon..].find(":");
        if index_colon_opt.is_none() {
            break;
        }
        index_colon += index_colon_opt.unwrap();
        if ['{', '['].contains(&json[index_colon..].chars().nth(2).unwrap()) {
            index_colon += 1;
            continue;
        }

        let index_prev_new_line = json[..index_colon].rfind("\n").unwrap_or_else(|| {
            fail!("Couldn't fine new line character before index: {}", index_colon);
        });
        let line_length_up_to_value = index_colon - index_prev_new_line;

        if KEY_ALIGNMENT < line_length_up_to_value {
            error!("Failed to write Spoiler Log");
            error!(
                "JSON Key Alignment value smaller than line length up to that point: {} < {}",
                KEY_ALIGNMENT, line_length_up_to_value
            );
            fail!("Problem line: {}", &json[index_prev_new_line..index_colon]);
        }

        let spaces_to_add = KEY_ALIGNMENT - line_length_up_to_value;

        json.insert_str(
            index_colon + 1,
            (0..spaces_to_add).map(|_| " ").collect::<String>().as_str(),
        );
        index_colon += 1;
    }
}

/// Gets the system object for the platform.
pub fn system() -> system::Result<System<Settings>> {
    System::new()
}

pub fn filler_new<'a>(version: &'a str, settings: &'a Settings, seed: Seed) -> Spoiler<'a> {
    // New Filler
    let filled: (Vec<(LocationInfo, Item)>, Metrics) = fill_stuff(settings, seed);

    // Build legacy Layout object
    let mut layout = Layout::default();
    for (location_info, item) in filled.0 {
        layout.set(location_info, item);
    }

    Spoiler { seed, version, settings, layout, metrics: filled.1 }
}

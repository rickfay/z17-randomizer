use std::{array, collections::{BTreeMap, HashMap, HashSet}, error::Error as StdError, fs, fs::File, hash::Hash as _, io};
use std::io::{stdin, stdout, Write};
use std::path::Path;

use crc::{crc32, Hasher32};
use linked_hash_map::LinkedHashMap;
use log::{debug, info};
use rand::prelude::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

use albw::{course, Game, Item};
use albw::Item::*;

mod fill;
mod graph;
mod patch;
mod queue_custom;
mod regions;
pub mod settings;
mod state;
mod check;
mod filler_item;
mod loading_zone;
mod loading_zone_pair;
mod location;
mod location_node;
mod path;
mod progress;
mod world;
mod filler;

use patch::Patcher;
use queue_custom::Queue;
use regions::Subregion;
pub use settings::Settings;
use state::State;
use sys::{Paths, System};
use crate::filler::fill_stuff;
use crate::filler_item::{convert, FillerItem};
use crate::settings::plando_settings;

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
        Self {
            kind: ErrorKind::Game,
            inner: err.into(),
        }
    }

    fn io<S>(err: S) -> Self
        where
            S: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self {
            kind: ErrorKind::Io,
            inner: err.into(),
        }
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
        Self {
            kind,
            inner: err.into_inner(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io,
            inner: err.into(),
        }
    }
}

impl From<sys::Error> for Error {
    fn from(err: sys::Error) -> Self {
        Self {
            kind: ErrorKind::Sys,
            inner: err.into(),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Sys,
    Game,
    Io,
}

pub type Seed = u32;

/// A randomized patch generator.
#[derive(Debug)]
pub struct Generator<'settings> {
    settings: &'settings Settings,
    seed: Seed,
}

impl<'settings> Generator<'settings> {
    /// Generate a new randomizer with the specified configuration.
    pub fn new(settings: &'settings Settings, seed: Seed) -> Self {
        Self { settings, seed }
    }

    /// Generate a unique hash.
    pub fn hash(&self) -> Hash {
        let mut hasher = crc32::Digest::new(crc32::IEEE);
        self.settings.hash(&mut hasher);
        self.seed.hash(&mut hasher);
        Hash(hasher.sum32())
    }

    /// Randomize world and generate files according to settings.
    pub fn randomize(&self) -> Spoiler {
        info!("Seed:                           {}", self.seed);
        info!("Hash:                           {}", self.hash().0);
        info!("Logic:                          {}", if self.settings.logic.glitched_logic {"Glitched"} else {"Normal"});
        info!("Swords:                         {}", if self.settings.logic.swordless_mode {"Swordless Mode - No Swords"} else {"Normal"});
        info!("Super Items:                    {}", if self.settings.logic.super_items {"Included"} else {"Not Included"});
        info!("Trials:                         {}", if self.settings.logic.skip_trials {"Skipped"} else {"Normal"});

        let rng = StdRng::seed_from_u64(self.seed as u64);
        let (randomized, layout) = Randomized::new(rng, exclude(&self.settings), &self.settings);
        let layout = fill::fill(
            &self.settings,
            randomized.locations,
            layout,
            randomized.world,
            randomized.dungeons,
        );
        Spoiler {
            seed: self.seed,
            settings: self.settings,
            layout,
        }
    }
}

#[derive(Debug)]
pub struct Hash(u32);

pub(crate) type Condition = for<'state> fn(&'state State) -> bool;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LocationInfo {
    subregion: &'static Subregion,
    name: &'static str,
}

impl LocationInfo {
    pub(crate) const fn new(subregion: &'static Subregion, name: &'static str) -> Self {
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
        self.world_mut(node.world())
            .entry(node.name())
            .or_insert_with(Default::default)
    }

    fn get(&self, location: &LocationInfo) -> Option<Item> {
        let LocationInfo {
            subregion: node,
            name,
        } = location;
        self.world(node.world())
            .get(node.name())
            .and_then(|region| region.get(name).copied())
    }

    fn set(&mut self, location: LocationInfo, item: Item) {
        let LocationInfo {
            subregion: node,
            name,
        } = location;
        self.get_node_mut(node).insert(name, item.normalize());
        debug!(
            "Placed {} in {}/{}",
            item.normalize().as_str(),
            location.subregion.name(),
            location.name
        );
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Quest {
    Sanctuary,
    Pendant(Pendant),
    Portrait(Portrait),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pendant {
    Courage,
    Wisdom,
    Power,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Portrait {
    Gulley,
    Oren,
    Seres,
    Osfala,
    Rosso,
    Irene,
    Impa,
}

pub(crate) type World = LinkedHashMap<&'static str, BTreeMap<&'static str, Item>>;

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
        ItemMizukaki => "Flippers",
        RingHekiga => "Bracelet",
        ItemBell => "Bell",
        RupeeGold => "Gold Rupee",
        RupeeSilver => "Silver Rupee",
        PowerGlove => "Progressive Glove",
        ItemInsectNet => "Net",
        ItemInsectNetLv2 => "Super Net",
        Kinsta => "Maiamai",
        BadgeBee => "Bee Badge",
        HintGlasses => "Hint Glasses",
        LiverBlue => "Monster Tail",
        LiverPurple => "Monster Guts",
        LiverYellow => "Monster Horn",
        ClothesBlue => "Progressive Mail",
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

        HyruleSanctuaryKey => "Hyrule Sanctuary Small Key",
        LoruleSanctuaryKey => "Lorule Sanctuary Small Key",

        EasternKeySmall => "Eastern Palace Small Key",
        EasternKeyBig => "Eastern Palace Big Key",
        EasternCompass => "Eastern Palace Compass",

        GalesKeySmall => "House of Gales Small Key",
        GalesKeyBig => "House of Gales Big Key",
        GalesCompass => "House of Gales Compass",

        HeraKeySmall => "Tower of Hera Small Key",
        HeraKeyBig => "Tower of Hera Big Key",
        HeraCompass => "Tower of Hera Compass",

        DarkKeySmall => "Dark Palace Small Key",
        DarkKeyBig => "Dark Palace Big Key",
        DarkCompass => "Dark Palace Compass",

        SwampKeySmall => "Swamp Palace Small Key",
        SwampKeyBig => "Swamp Palace Big Key",
        SwampCompass => "Swamp Palace Compass",

        SkullKeySmall => "Skull Woods Small Key",
        SkullKeyBig => "Skull Woods Big Key",
        SkullCompass => "Skull Woods Compass",

        ThievesKeySmall => "Thieves' Hideout Small Key",
        ThievesKeyBig => "Thieves' Hideout Big Key",
        ThievesCompass => "Thieves' Hideout Compass",

        IceKeySmall => "Ice Ruins Small Key",
        IceKeyBig => "Ice Ruins Big Key",
        IceCompass => "Ice Ruins Compass",

        DesertKeySmall => "Desert Palace Small Key",
        DesertKeyBig => "Desert Palace Big Key",
        DesertCompass => "Desert Palace Compass",

        TurtleKeySmall => "Turtle Rock Small Key",
        TurtleKeyBig => "Turtle Rock Big Key",
        TurtleCompass => "Turtle Rock Compass",

        LoruleCastleKeySmall => "Lorule Castle Small Key",
        LoruleCastleCompass => "Lorule Castle Compass",

        _ => unreachable!("{}", item.as_str()),
    }
}

#[derive(Debug, Default)]
pub(crate) struct Pool {
    progression: Queue<Item>,
    rest: Queue<Item>,
}

impl Pool {
    fn insert(&mut self, weight: u32, item: Item) {
        if item.is_progression() {
            &mut self.progression
        } else {
            &mut self.rest
        }
            .push(weight, item)
    }

    fn insert_unique(&mut self, weight: u32, item: Item) {
        if item.is_progression() && !self.progression.contains(&item) {
            &mut self.progression
        } else {
            &mut self.rest
        }
            .push(weight, item)
    }
}

trait ItemExt {
    fn is_dungeon(&self) -> bool;
    fn is_progression(&self) -> bool;
    fn is_sword(&self) -> bool;
    fn is_super(&self) -> bool;
    fn is_ore(&self) -> bool;
    fn normalize(self) -> Self;
}

impl ItemExt for Item {
    fn is_dungeon(&self) -> bool {
        matches!(self, Item::KeySmall | Item::KeyBoss | Item::Compass)
    }

    fn is_progression(&self) -> bool {
        matches!(
            self,
            Item::KeySmall
                | Item::KeyBoss
                | Item::ItemStoneBeauty
                | Item::ItemKandelaar
                | Item::ItemSwordLv1
                | Item::ItemSwordLv2
                | Item::ItemMizukaki
                | Item::ItemRentalIceRod
                | Item::ItemRentalSandRod
                | Item::ItemRentalTornadeRod
                | Item::ItemRentalBomb
                | Item::ItemRentalFireRod
                | Item::ItemRentalHookShot
                | Item::ItemRentalBoomerang
                | Item::ItemRentalHammer
                | Item::ItemRentalBow
                | Item::ItemBottle
                | Item::RingHekiga
                | Item::PowerGlove
                | Item::ItemInsectNet
                | Item::PowerfulGlove
                | Item::OreYellow
                | Item::OreGreen
                | Item::OreBlue
                | Item::DashBoots
                | Item::OreRed
                | Item::MessageBottle
                | Item::MilkMatured
                | Item::GanbariPowerUp
                | Item::PackageSword
        )
    }

    fn is_sword(&self) -> bool {
        matches!(
            self,
            Item::ItemSwordLv1 |
            Item::ItemSwordLv2 |
            Item::ItemSwordLv3 |
            Item::ItemSwordLv4
        )
    }

    fn is_super(&self) -> bool {
        matches! {
            self,
            Item::ItemKandelaarLv2 |
            Item::ItemInsectNetLv2
        }
    }

    fn is_ore(&self) -> bool {
        matches!(
            self,
            Item::OreYellow | Item::OreGreen | Item::OreBlue | Item::OreRed
        )
    }

    fn normalize(self) -> Self {
        match self {
            PackageSword | ItemSwordLv1 | ItemSwordLv3 | ItemSwordLv4 => {
                ItemSwordLv2
            }
            ItemRentalIceRod => ItemIceRod,
            ItemRentalSandRod => ItemSandRod,
            ItemRentalTornadeRod => ItemTornadeRod,
            ItemRentalBomb => ItemBomb,
            //Item::ItemRentalBomb => Item::ItemBombLv2,
            ItemRentalFireRod => ItemFireRod,
            ItemRentalHookShot => ItemHookShot,
            ItemRentalBoomerang => ItemBoomerang,
            ItemRentalHammer => ItemHammer,
            ItemRentalBow => ItemBow,
            PowerfulGlove => PowerGlove,
            ClothesRed => ClothesBlue,
            RingRental => RingHekiga,
            ItemKandelaarLv2 => ItemKandelaar,
            ItemInsectNetLv2 => ItemInsectNet,

            HyruleSanctuaryKey => KeySmall,
            LoruleSanctuaryKey => KeySmall,

            EasternCompass => Compass,
            EasternKeySmall => KeySmall,
            EasternKeyBig => KeyBoss,

            GalesCompass => Compass,
            GalesKeySmall => KeySmall,
            GalesKeyBig => KeyBoss,

            HeraCompass => Compass,
            HeraKeySmall => KeySmall,
            HeraKeyBig => KeyBoss,

            DarkCompass => Compass,
            DarkKeySmall => KeySmall,
            DarkKeyBig => KeyBoss,

            SwampCompass => Compass,
            SwampKeySmall => KeySmall,
            SwampKeyBig => KeyBoss,

            SkullCompass => Compass,
            SkullKeySmall => KeySmall,
            SkullKeyBig => KeyBoss,

            ThievesCompass => Compass,
            ThievesKeySmall => KeySmall,
            ThievesKeyBig => KeyBoss,

            IceCompass => Compass,
            IceKeySmall => KeySmall,
            IceKeyBig => KeyBoss,

            DesertCompass => Compass,
            DesertKeySmall => KeySmall,
            DesertKeyBig => KeyBoss,

            TurtleCompass => Compass,
            TurtleKeySmall => KeySmall,
            TurtleKeyBig => KeyBoss,

            LoruleCastleCompass => Compass,
            LoruleCastleKeySmall => KeySmall,

            item => item,
        }
    }
}

#[derive(Debug)]
struct Randomized {
    world: Pool,
    dungeons: HashMap<course::Id, Pool>,
    locations: HashMap<LocationInfo, u32>,
}

impl Randomized {
    fn new<R>(mut rng: R, exclude: HashSet<LocationInfo>, settings: &&Settings) -> (Self, Layout)
        where
            R: Rng,
    {
        let mut world = Pool::default();
        let mut dungeons = HashMap::<_, Pool>::new();
        let mut locations = HashMap::new();
        let mut layout = Layout::default();
        for (location, item) in regions::items() {
            if exclude.contains(&location) {
                // let skipped =
                //     (item == Item::PackageSword && settings.items.captains_sword.is_skipped())
                //         || (item == Item::RingRental && settings.items.first_bracelet.is_skipped()
                //     );
                // if !skipped {

                layout.set(location, item);

                //}
            } else {
                if item.is_dungeon() {
                    dungeons
                        .entry(location.subregion.course())
                        .or_default()
                        .insert(rng.next_u32(), item);
                } else {
                    let i = if settings.logic.swordless_mode && item.is_sword() {
                        RupeeG
                    } else if !settings.logic.super_items && item.is_super() {
                        RupeePurple
                    } else {
                        item
                    };

                    world.insert_unique(rng.next_u32(), i);
                }
                locations.insert(location, rng.next_u32());
            }
        }
        (
            Self {
                world,
                dungeons,
                locations,
            },
            layout,
        )
    }
}

/// A log of seed info and item placements
#[derive(Debug, Serialize)]
pub struct Spoiler<'settings> {
    seed: Seed,
    settings: &'settings Settings,
    layout: Layout,
}

impl<'settings> Spoiler<'settings> {
    pub fn patch(self, paths: Paths, patch: bool, spoiler: bool) -> Result<()> {
        let game = Game::load(paths.rom())?;
        let mut patcher = Patcher::new(game)?;
        regions::patch(&mut patcher, &self.layout, self.settings)?;
        let patches = patcher.prepare(self.settings)?;
        if patch {
            patches.dump(paths.output())?;
        }
        if spoiler {
            let path = paths.output().join(format!("spoiler {}.yaml", self.seed));
            info!("Writing spoiler to:             {}", path.display());
            serde_yaml::to_writer(File::create(path)?, &self)
                .expect("Could not write the spoiler log.");
        }
        Ok(())
    }
}

fn exclude(settings: &Settings) -> HashSet<LocationInfo> {
    let mut exclude = HashSet::new();

    // if !settings.items.captains_sword.is_shuffled() {
    //     exclude.insert(Location::new(
    //         regions::hyrule::field::main::SUBREGION,
    //         "Delivery",
    //     ));
    // }

    // if !settings.items.borrowed_sword.is_shuffled() {
    //     exclude.insert(Location::new(
    //         regions::hyrule::field::main::SUBREGION,
    //         "Dampe",
    //     ));
    // }

    // if !settings.items.lamp.is_shuffled() {
    //     exclude.insert(Location::new(
    //         regions::hyrule::sanctuary::lobby::SUBREGION,
    //         "Entrance",
    //     ));
    // }

    // Lock Message in a Bottle to its original spot
    exclude.insert(LocationInfo::new(
        regions::hyrule::lake::hylia::SUBREGION,
        "Shore",
    ));

    // Lock Smooth Gem to its original spot
    exclude.insert(LocationInfo::new(
        regions::hyrule::kakariko::shady_guy::SUBREGION,
        "Merchant (Right)",
    ));

    if settings.logic.swordless_mode {
        exclude.insert(LocationInfo::new(
            regions::hyrule::field::castle::SUBREGION,
            "Castle (Indoors)",
        ));

        exclude.insert(LocationInfo::new(
            regions::hyrule::field::castle::SUBREGION,
            "Castle Balcony",
        ));
    }

    if settings.logic.boots_in_shop {
        exclude.insert(LocationInfo::new(
            regions::hyrule::field::rentals::SUBREGION,
            "Ravio (2)",
        ));
    }

    if settings.logic.pouch_in_shop {
        exclude.insert(LocationInfo::new(
            regions::hyrule::field::rentals::SUBREGION,
            "Ravio (3)",
        ));
    }

    if settings.logic.bell_in_shop {
        exclude.insert(LocationInfo::new(
            regions::hyrule::field::rentals::SUBREGION,
            "Ravio (4)",
        ));
    }

    if settings.logic.start_with_bracelet {
        exclude.insert(LocationInfo::new(
            regions::hyrule::field::rentals::SUBREGION,
            "Ravio (5)",
        ));
    }

    if settings.logic.minigames_excluded {
        exclude.insert(LocationInfo::new(
            regions::hyrule::kakariko::post_sanc::SUBREGION,
            "Cucco Ranch",
        ));

        exclude.insert(LocationInfo::new(
            regions::hyrule::field::rupee_rush::SUBREGION,
            "Rupee Rush",
        ));

        exclude.insert(LocationInfo::new(
            regions::lorule::field::main::SUBREGION,
            "Rupee Rush",
        ));

        exclude.insert(LocationInfo::new(
            regions::lorule::death::tower::SUBREGION,
            "Treacherous Tower (Intermediate)",
        ));

        exclude.insert(LocationInfo::new(
            regions::lorule::field::main::SUBREGION,
            "Octoball Derby",
        ));

        exclude.insert(LocationInfo::new(
            regions::hyrule::lake::hotfoot::SUBREGION,
            "Hyrule Hotfoot",
        ));
    }

    exclude
}

/// Gets the system object for the platform.
pub fn system() -> sys::Result<System<Settings>> {
    System::new(array::IntoIter::new([]))
}

#[cfg(test)]
pub fn test_game() -> albw::Result<Game> {
    Game::load("../test.3ds")
}

fn prompt_until<F>(prompt: &str, until: F, error: &str) -> sys::Result<String>
    where
        F: Fn(&str) -> bool,
{
    loop {
        print!("{}: ", prompt);
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        input = input.trim().to_string();
        if until(&input) {
            break Ok(input);
        } else {
            eprintln!("{}", error);
        }
    }
}

fn create_paths() -> sys::Result<Paths> {
    let rom = prompt_until(
        "Path to ROM",
        |rom| Game::load(&rom).is_ok(),
        "The provided path does not point to a valid ROM.",
    )?;
    let output = prompt_until(
        "Output directory",
        |output| Path::new(output).exists() || fs::create_dir_all(&output).is_ok(),
        "The provided path could not be created.",
    )?;

    Ok(Paths::new(rom.into(), output.into()))
}

pub fn plando() -> Result<(), Error> {
    info!("Start the Plando!");

    let system = system()?;
    let settings = plando_settings();
    let mut layout = Layout::default();

    //////////////////////
    // --- Dungeons --- //
    //////////////////////

    // Eastern Palace
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "(1F) Outside (East)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "(1F) Near Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "(1F) Defeat Popos"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "(1F) Hidden Door"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "(1F) Switch Puzzle"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Ball Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Defeat Popos"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Switch Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Big Chest"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::eastern::boss::SUBREGION, "(3F) After Cutscene"), RingRental);
    layout.set(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "Yuga"), ItemBell);
    layout.set(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "(3F) Outside (North)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "(1F) Outside (West)"), ItemInsectNetLv2);

    // House of Gales
    layout.set(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "(1F) Torches"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "(1F) Switch Room"), ItemBomb);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "(1F) Fire Bubbles"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1west::SUBREGION, "(1F) Blue Bari Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1west::SUBREGION, "(1F) Blue Bari Room (Bottom Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor2::SUBREGION, "(2F) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::house::floor2::SUBREGION, "(2F) Narrow Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor2outer::SUBREGION, "(2F) Fire Ring"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::floor3::SUBREGION, "(3F) Rat Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::floor3::SUBREGION, "(3F) Fire Bubbles"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::boss::SUBREGION, "Margomill"), RupeeGold);

    // Tower of Hera
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "(1F) Outside"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor2::SUBREGION, "(1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor2::SUBREGION, "(3F) Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "(5F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "(6F) Left Mole"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "(6F) Right Mole"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "(7F) Outside (Ledge)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "(8F) Fairy Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "(11F) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::tower::boss::SUBREGION, "Moldorm"), RingHekiga);

    // Lorule Sanctuary
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "Lower Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "Ledge"), KeySmall);

    // Dark Palace
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "(1F) Near Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "(1F) Narrow Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(1F) Switch Puzzle"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(1F) Hidden Room (Upper)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(1F) Hidden Room (Lower)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Fall From 1F"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Maze"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Helmasaur Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Helmasaur Room (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "(2F) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "(2F) Alcove"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "(1F) Fall From 2F"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::boss_key::SUBREGION, "(B1) Big Chest (Switches)"), OreGreen);
    layout.set(LocationInfo::new(regions::dungeons::dark::boss::SUBREGION, "Gemesaur King"), RupeeGold);

    // Swamp Palace
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Raft Room (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Raft Room (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Gyorm"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Waterfall Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Raft Room (Pillar)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(B1) Big Chest (Secret)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(1F) Water Puzzle"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(1F) East Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(1F) West Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "(1F) Big Chest (Fire)"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::dungeons::swamp::boss::SUBREGION, "Arrghus"), KeyBoss);

    // Skull Woods
    layout.set(LocationInfo::new(regions::dungeons::skull::palace::SUBREGION, "(B1) Gibdo Room (Lower)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::palace::SUBREGION, "(B1) South Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::skull::outdoors::SUBREGION, "(B1) Gibdo Room (Hole)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::outdoors::SUBREGION, "(B1) Grate Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::basement2::SUBREGION, "(B2) Moving Platform Room"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::skull::end::SUBREGION, "(B1) Big Chest (Upper)"), ItemKandelaarLv2);
    layout.set(LocationInfo::new(regions::dungeons::skull::end::SUBREGION, "(B1) Big Chest (Eyes)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::skull::boss::SUBREGION, "Knucklemaster"), OreRed);

    // Thieves' Hideout
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "(B1) Jail Cell"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "(B1) Grate Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "(B2) Grate Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "(B2) Switch Puzzle Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "(B2) Jail Cell"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "(B2) Eyegores"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "(B1) Behind Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "(B1) Big Chest (Entrance)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "(B3) Underwater"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "(B3) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::thieves::boss::SUBREGION, "Stalblind"), OreYellow);

    // Ice Ruins
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "(1F) Hidden Chest"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "(B3) Grate Chest (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "(B3) Grate Chest (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "(B4) Ice Pillar"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "(B5) Big Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement1::SUBREGION, "(B1) East Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement1::SUBREGION, "(B1) Narrow Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B1) Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B3) Big Chest (Puzzle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Southwest Chest (Fall)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Narrow Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B2) Far North"), DashBoots);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Southeast Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::boss::SUBREGION, "Dharkstare"), KeyBoss);

    // Desert Palace
    layout.set(LocationInfo::new(regions::dungeons::desert::floor1::SUBREGION, "(1F) Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "(1F) Sand Room (South)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "(1F) Sand Switch Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "(1F) Sand Room (North)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "(1F) Big Chest (Behind Wall)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "(1F) Behind Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Under Rock (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Beamos Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Under Rock (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Under Rock (Ball Room)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Big Chest (Puzzle)"), PowerfulGlove);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2west::SUBREGION, "(2F) Leever Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor3::SUBREGION, "(3F) Silver Rupee"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor3::SUBREGION, "(3F) Armos Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::boss::SUBREGION, "Zaganaga"), RupeeGold);

    // Turtle Rock
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Grate Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Portal Room (Northwest)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Northeast Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Southeast Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Defeat Flamolas"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Northeast Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Grate Chest (Small)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Big Chest (Center)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Big Chest (Top)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::boss::SUBREGION, "Grinexx"), KeyBoss);

    // Lorule Castle
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "(1F) Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "(1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "(2F) Near Torches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "(2F) Hidden Path"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "(2F) Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::floor4::SUBREGION, "(4F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::floor4::SUBREGION, "(4F) Hidden Path"), ItemBowLight);
    layout.set(LocationInfo::new(regions::dungeons::castle::bomb_trial::SUBREGION, "(3F) Bomb Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::bomb_trial::SUBREGION, "(3F) Bomb Trial (Behind Rock)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::ball_trial::SUBREGION, "(3F) Ball Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::ball_trial::SUBREGION, "(3F) Ball Trial (Puzzle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lamp_trial::SUBREGION, "(4F) Lamp Trial"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "(4F) Hookshot Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "(4F) Hookshot Trial (Eyes)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::boss::SUBREGION, "Zelda"), ItemBow);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Delivery"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Dampe"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Rosso Cave"), ItemInsectNet);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Sanctuary Pegs"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Treasure Room"), ItemBoomerangLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Behind Blacksmith"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith Cave"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Castle Rocks"), RupeeGold);
    //layout.set(Location::new(regions::hyrule::field::post_sanc::SUBREGION, "Thanks"), Item::BadgeBee);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Rosso"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Clean Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Irene"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Woods"), RupeeGold);


    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (1)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (2)"), ItemSwordLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (3)"), ItemSwordLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (4)"), DashBoots);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (5)"), RingHekiga);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (6)"), ClothesBlue);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (7)"), ItemTornadeRodLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (8)"), ItemBombLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (9)"), RupeeGold);


    layout.set(LocationInfo::new(regions::hyrule::field::rupee_rush::SUBREGION, "Rupee Rush"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::castle::SUBREGION, "Castle (Indoors)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::castle::SUBREGION, "Castle Balcony"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::sanctuary_cave::SUBREGION, "Sanctuary Cave"), RupeeGold);

    // Lost Woods
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Pedestal"), ItemBottle);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Alcove"), ItemHookShot);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Chest"), ItemIceRod);

    // Death Mountain
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "First Cave"), PowerGlove);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Blocked Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Fairy Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Ledge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Pegs)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Top)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Hidden Area"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Ore Mine Column"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Bouldering Guy"), HyruleShield);
    layout.set(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Treasure Room"), ItemHookShotLv2);
    layout.set(LocationInfo::new(regions::hyrule::death::far_island::SUBREGION, "Distant Pillar"), RupeeGold);

    // Sanctuary
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::lobby::SUBREGION, "Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "Lower Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "Ledge"), RupeeGold);

    // Kakariko
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Chest)"), ClothesBlue);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Upper)"), ClothesBlue);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Jail"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Merchant (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy"), HintGlasses);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy (Golden Bee)"), ItemFireRod);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Fortune Teller"), Pouch);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Milk Bar Owner"), MilkMatured);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Cucco Ranch"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Shady Guy"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Merchant (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::closed::SUBREGION, "Stylish Woman"), RupeeGold);

    // Zora's Domain
    layout.set(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Behind Waterfall"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Zora Queen"), RupeeGold);

    // Eastern Ruins
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Treasure Room"), ItemHammerLv2);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Armos Chest"), ItemTornadeRod);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Hookshot Chest"), ItemSandRod);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Merge Chest"), ItemBoomerang);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Pegs (South)"), RupeeGold);

    // Southern Ruins
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Runaway Item Seller"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Behind Pillars"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Treasure Room"), ItemHammer);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Ledge"), RupeeGold);

    // Lake Hylia
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Torch Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Ledge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Bird Lover"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Secret Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Shore"), MessageBottle);
    layout.set(LocationInfo::new(regions::hyrule::lake::hotfoot::SUBREGION, "Hyrule Hotfoot"), RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Treasure Room"), GanbariPowerUp);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Vacant House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Rupee Rush"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Great Rupee Fairy"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Big Bomb Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Octoball Derby"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Blacksmith"), ItemKandelaar);
    layout.set(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Middle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::thief_girl::SUBREGION, "Thief Girl"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::ledge::SUBREGION, "Hookshot Ledge"), RupeeGold);

    // Skull Woods (overworld)
    layout.set(LocationInfo::new(regions::lorule::skull::woods::SUBREGION, "Alcove"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::woods::SUBREGION, "Balcony"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::chest::SUBREGION, "Chest"), RupeeGold);

    // Lorule Death Mountain
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Ledge (East)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Behind Ice Gimos"), ItemFireRodLv2);
    layout.set(LocationInfo::new(regions::lorule::death::west::SUBREGION, "Ledge (West)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::west::SUBREGION, "Ice Gimos (West)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::tower::SUBREGION, "Treacherous Tower (Intermediate)"), RupeeGold);

    // Lorule Graveyard
    layout.set(LocationInfo::new(regions::lorule::graveyard::cave::SUBREGION, "Big Chest"), OreBlue);
    layout.set(LocationInfo::new(regions::lorule::graveyard::field::SUBREGION, "Field Chest"), RupeeGold);

    // Dark Ruins
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Lake Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Maze Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Maze Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (1)"), RupeeG);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (2)"), RupeeB);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (3)"), RupeeR);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (4)"), RupeePurple);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (5)"), RupeeSilver);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (6)"), SpecialMove);

    // Misery Mire
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Treasure Room"), ItemSandRodLv2);

    // Lake Lolia
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::balcony::SUBREGION, "Balcony"), ItemMizukaki);

    let spoiler = Spoiler {
        seed: 0,
        settings: &settings,
        layout,
    };

    spoiler.patch(
        system.get_or_create_paths(create_paths)?,
        true,
        true,
    )
}

pub fn filler_new(settings: &Settings, seed: u64) -> Result<()> {

    // TODO integrate settings properly

    // New Filler
    let filled: Vec<(LocationInfo, Item)> = fill_stuff(settings, seed);

    // Build legacy Layout object
    let mut layout = Layout::default();
    for (location_info, item) in filled {
        layout.set(location_info, item);
    }

    // Patch and build spoiler log
    let system = system()?;
    let settings = plando_settings();
    let spoiler = Spoiler { seed: seed as Seed, settings: &settings, layout };
    let result = spoiler.patch(system.get_or_create_paths(create_paths)?, true, true);

    result
}
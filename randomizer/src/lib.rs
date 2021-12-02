use std::{
    array,
    collections::{BTreeMap, HashMap, HashSet},
    error::Error as StdError,
    fs::File,
    hash::Hash as _,
    io,
};

use crc::{crc32, Hasher32};
use linked_hash_map::LinkedHashMap;
use log::info;
use rand::prelude::*;
use serde::{ser::SerializeMap, Serialize, Serializer};

use albw::{course, Game, Item};

mod fill;
mod graph;
mod patch;
mod queue;
mod regions;
pub mod settings;
mod state;

use patch::Patcher;
use queue::Queue;
use regions::Subregion;
pub use settings::Settings;
use state::State;
use sys::{Paths, System};

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

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
        info!("Using seed {}", self.seed);
        info!("Hash: {}", self.hash().0);
        let rng = StdRng::seed_from_u64(self.seed as u64);
        let (randomized, layout) = Randomized::new(rng, &self.settings, exclude(&self.settings));
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Location {
    subregion: &'static Subregion,
    name: &'static str,
}

impl Location {
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

    fn get(&self, location: &Location) -> Option<Item> {
        let Location {
            subregion: node,
            name,
        } = location;
        self.world(node.world())
            .get(node.name())
            .and_then(|region| region.get(name).copied())
    }

    fn set(&mut self, location: Location, item: Item) {
        let Location {
            subregion: node,
            name,
        } = location;
        self.get_node_mut(node).insert(name, item.normalize());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Quest {
    Sanctuary,
    Pendant(Pendant),
    Lorule,
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
        Item::KeySmall => "Small Key",
        Item::KeyBoss => "Big Key",
        Item::Compass => "Compass",
        Item::HeartContainer => "Heart Container",
        Item::RupeeR => "Red Rupee",
        Item::RupeeG => "Green Rupee",
        Item::RupeeB => "Blue Rupee",
        Item::HeartPiece => "Piece of Heart",
        Item::ItemIceRod => "Ice Rod",
        Item::ItemSandRod => "Sand Rod",
        Item::ItemTornadeRod => "Tornado Rod",
        Item::ItemBomb => "Bombs",
        Item::ItemFireRod => "Fire Rod",
        Item::ItemHookShot => "Hookshot",
        Item::ItemBoomerang => "Boomerang",
        Item::ItemHammer => "Hammer",
        Item::ItemBow => "Bow",
        Item::ItemShield => "Shield",
        Item::ItemBottle => "Bottle",
        Item::ItemStoneBeauty => "Smooth Gem",
        Item::ItemKandelaar => "Lamp",
        Item::ItemSwordLv1 => "Progressive Sword",
        Item::ItemSwordLv2 => "Progressive Sword",
        Item::ItemMizukaki => "Flippers",
        Item::RingHekiga => "Bracelet",
        Item::ItemBell => "Bell",
        Item::RupeeGold => "Gold Rupee",
        Item::RupeeSilver => "Silver Rupee",
        Item::PowerGlove => "Progressive Glove",
        Item::ItemInsectNet => "Bug Net",
        Item::Kinsta => "Maiamai",
        Item::BadgeBee => "Bee Badge",
        Item::HintGlasses => "Hint Glasses",
        Item::LiverBlue => "Monster Tail",
        Item::LiverPurple => "Monster Guts",
        Item::LiverYellow => "Monster Horn",
        Item::ClothesBlue => "Progressive Mail",
        Item::HyruleShield => "Hylian Shield",
        Item::OreYellow => "Master Ore",
        Item::OreGreen => "Master Ore",
        Item::OreBlue => "Master Ore",
        Item::GanbariPowerUp => "Stamina Scroll",
        Item::Pouch => "Pouch",
        Item::DashBoots => "Pegasus Boots",
        Item::OreRed => "Master Ore",
        Item::MessageBottle => "Message in a Bottle",
        Item::MilkMatured => "Premium Milk",
        Item::SpecialMove => "Great Spin",
        Item::GanbariTubo => "Stamina Scroll",
        Item::RupeePurple => "Purple Rupee",
        Item::ItemBowLight => "Bow of Light",
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

    fn is_ore(&self) -> bool {
        matches!(
            self,
            Item::OreYellow | Item::OreGreen | Item::OreBlue | Item::OreRed
        )
    }

    fn normalize(self) -> Self {
        match self {
            Item::PackageSword | Item::ItemSwordLv1 | Item::ItemSwordLv3 | Item::ItemSwordLv4 => {
                Item::ItemSwordLv2
            }
            Item::ItemRentalIceRod => Item::ItemIceRod,
            Item::ItemRentalSandRod => Item::ItemSandRod,
            Item::ItemRentalTornadeRod => Item::ItemTornadeRod,
            Item::ItemRentalBomb => Item::ItemBomb,
            Item::ItemRentalFireRod => Item::ItemFireRod,
            Item::ItemRentalHookShot => Item::ItemHookShot,
            Item::ItemRentalBoomerang => Item::ItemBoomerang,
            Item::ItemRentalHammer => Item::ItemHammer,
            Item::ItemRentalBow => Item::ItemBow,
            Item::PowerfulGlove => Item::PowerGlove,
            Item::ClothesRed => Item::ClothesBlue,
            Item::RingRental => Item::RingHekiga,
            item => item,
        }
    }
}

#[derive(Debug)]
struct Randomized {
    world: Pool,
    dungeons: HashMap<course::Id, Pool>,
    locations: HashMap<Location, u32>,
}

impl Randomized {
    fn new<R>(mut rng: R, settings: &Settings, exclude: HashSet<Location>) -> (Self, Layout)
    where
        R: Rng,
    {
        let mut world = Pool::default();
        let mut dungeons = HashMap::<_, Pool>::new();
        let mut locations = HashMap::new();
        let mut layout = Layout::default();
        for (location, item) in regions::items() {
            if exclude.contains(&location) {
                let skipped = (item == Item::PackageSword
                    && settings.items.captains_sword.is_skipped())
                    || (item == Item::RingRental && settings.items.first_bracelet.is_skipped());
                if !skipped {
                    layout.set(location, item);
                }
            } else {
                if item.is_dungeon() {
                    dungeons
                        .entry(location.subregion.course())
                        .or_default()
                        .insert(rng.next_u32(), item);
                } else {
                    world.insert_unique(rng.next_u32(), item);
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
        let mut patcher = Patcher::new(game, self.seed)?;
        regions::patch(&mut patcher, &self.layout, self.settings)?;
        let patches = patcher.prepare(self.settings)?;
        if patch {
            patches.dump(paths.output())?;
        }
        if spoiler {
            let path = paths.output().join(format!("spoiler {}.yaml", self.seed));
            info!("Writing spoiler to {}", path.display());
            serde_yaml::to_writer(File::create(path)?, &self)
                .expect("Could not write the spoiler log.");
        }
        Ok(())
    }
}

fn exclude(settings: &Settings) -> HashSet<Location> {
    let mut exclude = HashSet::new();
    if !settings.items.captains_sword.is_shuffled() {
        exclude.insert(Location::new(
            regions::hyrule::field::main::SUBREGION,
            "Delivery",
        ));
    }
    if !settings.items.borrowed_sword.is_shuffled() {
        exclude.insert(Location::new(
            regions::hyrule::field::main::SUBREGION,
            "Dampe",
        ));
    }
    if !settings.items.lamp.is_shuffled() {
        exclude.insert(Location::new(
            regions::hyrule::sanctuary::lobby::SUBREGION,
            "Entrance",
        ));
    }
    if !settings.items.first_bracelet.is_shuffled() {
        exclude.insert(Location::new(
            regions::hyrule::field::post_sanc::SUBREGION,
            "Thanks",
        ));
    }
    exclude.insert(Location::new(
        regions::hyrule::lake::hylia::SUBREGION,
        "Shore",
    ));
    exclude
}

/// Gets the system object for the platform.
pub fn system() -> sys::Result<System<Settings>> {
    System::new(array::IntoIter::new([("Open", settings::open_default())]))
}

#[cfg(test)]
pub fn test_game() -> albw::Result<Game> {
    Game::load("../test.3ds")
}

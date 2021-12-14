use std::{array, collections::{BTreeMap, HashMap, HashSet}, error::Error as StdError, fs, fs::File, hash::Hash as _, io};
use std::io::{stdin, stdout, Write};
use std::path::Path;

use crc::{crc32, Hasher32};
use linked_hash_map::LinkedHashMap;
use log::{info, warn};
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
use crate::settings::plando_settings;

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
        info!("Using Logic: {}", if self.settings.logic.glitched_logic {"Glitched"} else {"Normal"});
        info!("Using Seed:  {}", self.seed);
        info!("Hash:        {}", self.hash().0);
        if (self.settings.behavior.portals_open) {
            warn!("Portals are Open! Seed will not be completable!!!");
        }

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
        info!(
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
        Item::ItemIceRodLv2 => "Nice Ice Rod",
        Item::ItemSandRod => "Sand Rod",
        Item::ItemSandRodLv2 => "Nice Sand Rod",
        Item::ItemTornadeRod => "Tornado Rod",
        Item::ItemTornadeRodLv2 => "Nice Tornado Rod",
        Item::ItemBomb => "Bombs",
        Item::ItemBombLv2 => "Nice Bombs",
        Item::ItemFireRod => "Fire Rod",
        Item::ItemFireRodLv2 => "Nice Fire Rod",
        Item::ItemHookShot => "Hookshot",
        Item::ItemHookShotLv2 => "Nice Hookshot",
        Item::ItemBoomerang => "Boomerang",
        Item::ItemBoomerangLv2 => "Nice Boomerang",
        Item::ItemHammer => "Hammer",
        Item::ItemHammerLv2 => "Nice Hammer",
        Item::ItemBow => "Bow",
        Item::ItemBowLv2 => "Nice Bow",
        Item::ItemShield => "Shield",
        Item::ItemBottle => "Bottle",
        Item::ItemStoneBeauty => "Smooth Gem",
        Item::ItemKandelaar => "Lamp",
        Item::ItemKandelaarLv2 => "Super Lamp",
        Item::ItemSwordLv1 => "Progressive Sword",
        Item::ItemSwordLv2 => "Progressive Sword",
        Item::ItemMizukaki => "Flippers",
        Item::RingHekiga => "Bracelet",
        Item::ItemBell => "Bell",
        Item::RupeeGold => "Gold Rupee",
        Item::RupeeSilver => "Silver Rupee",
        Item::PowerGlove => "Progressive Glove",
        Item::ItemInsectNet => "Net",
        Item::ItemInsectNetLv2 => "Super Net",
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
            //Item::ItemRentalBomb => Item::ItemBomb, // I don't want to do Maiamai logic, so lets just give them Nice Bombs!
            Item::ItemRentalBomb => Item::ItemBombLv2, // I don't want to do Maiamai logic, so lets just give them Nice Bombs!
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
        let mut patcher = Patcher::new(game)?;
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

fn prompt_until<F>(prompt: &str, until: F, error: &str) -> ::sys::Result<String>
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

fn create_paths() -> ::sys::Result<Paths> {

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
    layout.set(Location::new(regions::dungeons::eastern::palace::SUBREGION, "(1F) Outside (East)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::palace::SUBREGION, "(1F) Near Entrance"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::floor1::SUBREGION, "(1F) Defeat Popos"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::floor1::SUBREGION, "(1F) Hidden Door"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::eastern::floor1::SUBREGION, "(1F) Switch Puzzle"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Ball Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Defeat Popos"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::floor2::SUBREGION, "(2F) Switch Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::eastern::boss_key::SUBREGION, "(2F) Big Chest"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::eastern::boss::SUBREGION, "(3F) After Cutscene"), Item::RingRental);
    layout.set(Location::new(regions::dungeons::eastern::post_boss::SUBREGION, "Yuga"), Item::ItemBell);
    layout.set(Location::new(regions::dungeons::eastern::post_boss::SUBREGION, "(3F) Outside (North)"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::eastern::post_boss::SUBREGION, "(1F) Outside (West)"), Item::ItemInsectNetLv2);

    // House of Gales
    layout.set(Location::new(regions::dungeons::house::floor1::SUBREGION, "(1F) Torches"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::house::floor1::SUBREGION, "(1F) Switch Room"), Item::ItemBomb);
    layout.set(Location::new(regions::dungeons::house::floor1::SUBREGION, "(1F) Fire Bubbles"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::house::floor1west::SUBREGION, "(1F) Blue Bari Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::house::floor1west::SUBREGION, "(1F) Blue Bari Room (Bottom Left)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::house::floor2::SUBREGION, "(2F) Big Chest"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::house::floor2::SUBREGION, "(2F) Narrow Ledge"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::house::floor2outer::SUBREGION, "(2F) Fire Ring"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::house::floor3::SUBREGION, "(3F) Rat Room"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::house::floor3::SUBREGION, "(3F) Fire Bubbles"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::house::boss::SUBREGION, "Margomill"), Item::RupeeGold);

    // Tower of Hera
    layout.set(Location::new(regions::dungeons::tower::hera::SUBREGION, "(1F) Outside"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::tower::floor2::SUBREGION, "(1F) Center"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::tower::floor2::SUBREGION, "(3F) Platform"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::tower::floor4::SUBREGION, "(5F) Red/Blue Switches"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::tower::floor4::SUBREGION, "(6F) Left Mole"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::tower::floor4::SUBREGION, "(6F) Right Mole"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::tower::floor7::SUBREGION, "(7F) Outside (Ledge)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::tower::floor7::SUBREGION, "(8F) Fairy Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::tower::floor7::SUBREGION, "(11F) Big Chest"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::tower::boss::SUBREGION, "Moldorm"), Item::RingHekiga);

    // Lorule Sanctuary
    layout.set(Location::new(regions::dungeons::graveyard::main::SUBREGION, "Entrance"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::graveyard::main::SUBREGION, "Lower Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::graveyard::main::SUBREGION, "Upper Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::graveyard::main::SUBREGION, "Ledge"), Item::KeySmall);

    // Dark Palace
    layout.set(Location::new(regions::dungeons::dark::palace::SUBREGION, "(1F) Near Entrance"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::palace::SUBREGION, "(1F) Narrow Ledge"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(1F) Switch Puzzle"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(1F) Hidden Room (Upper)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(1F) Hidden Room (Lower)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Fall From 1F"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Maze"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Helmasaur Room"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::dark::floor1::SUBREGION, "(B1) Helmasaur Room (Fall)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::floor2::SUBREGION, "(2F) Big Chest (Hidden)"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::dark::floor2::SUBREGION, "(2F) Alcove"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::floor2::SUBREGION, "(1F) Fall From 2F"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::dark::boss_key::SUBREGION, "(B1) Big Chest (Switches)"), Item::OreGreen);
    layout.set(Location::new(regions::dungeons::dark::boss::SUBREGION, "Gemesaur King"), Item::RupeeGold);

    // Swamp Palace
    layout.set(Location::new(regions::dungeons::swamp::floor1::SUBREGION, "(B1) Center"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::swamp::floor1::SUBREGION, "(B1) Raft Room (Left)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::swamp::floor1::SUBREGION, "(B1) Raft Room (Right)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::swamp::floor1::SUBREGION, "(B1) Gyorm"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::swamp::floor1::SUBREGION, "(B1) Waterfall Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::swamp::miniboss::SUBREGION, "(B1) Raft Room (Pillar)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::swamp::deep::SUBREGION, "(B1) Big Chest (Secret)"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::swamp::deep::SUBREGION, "(1F) Water Puzzle"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::swamp::deep::SUBREGION, "(1F) East Room"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::swamp::deep::SUBREGION, "(1F) West Room"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::swamp::end::SUBREGION, "(1F) Big Chest (Fire)"), Item::ItemSwordLv1);
    layout.set(Location::new(regions::dungeons::swamp::boss::SUBREGION, "Arrghus"), Item::KeyBoss);

    // Skull Woods
    layout.set(Location::new(regions::dungeons::skull::palace::SUBREGION, "(B1) Gibdo Room (Lower)"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::skull::palace::SUBREGION, "(B1) South Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::skull::outdoors::SUBREGION, "(B1) Gibdo Room (Hole)"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::skull::outdoors::SUBREGION, "(B1) Grate Room"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::skull::basement2::SUBREGION, "(B2) Moving Platform Room"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::skull::end::SUBREGION, "(B1) Big Chest (Upper)"), Item::ItemKandelaarLv2);
    layout.set(Location::new(regions::dungeons::skull::end::SUBREGION, "(B1) Big Chest (Eyes)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::skull::boss::SUBREGION, "Knucklemaster"), Item::OreRed);

    // Thieves' Hideout
    layout.set(Location::new(regions::dungeons::thieves::hideout::SUBREGION, "(B1) Jail Cell"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::hideout::SUBREGION, "(B1) Grate Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::hideout::SUBREGION, "(B2) Grate Chest (Fall)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::basement2::SUBREGION, "(B2) Switch Puzzle Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::basement2::SUBREGION, "(B2) Jail Cell"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::basement2::SUBREGION, "(B2) Eyegores"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::escape::SUBREGION, "(B1) Behind Wall"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::escape::SUBREGION, "(B1) Big Chest (Entrance)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::thieves::escape::SUBREGION, "(B3) Underwater"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::thieves::escape::SUBREGION, "(B3) Big Chest (Hidden)"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::thieves::boss::SUBREGION, "Stalblind"), Item::OreYellow);

    // Ice Ruins
    layout.set(Location::new(regions::dungeons::ice::ruins::SUBREGION, "(1F) Hidden Chest"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::ice::ruins::SUBREGION, "(B3) Grate Chest (Left)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::ruins::SUBREGION, "(B3) Grate Chest (Right)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::ruins::SUBREGION, "(B4) Ice Pillar"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::ruins::SUBREGION, "(B5) Big Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::basement1::SUBREGION, "(B1) East Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::basement1::SUBREGION, "(B1) Narrow Ledge"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B1) Upper Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B3) Big Chest (Puzzle)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Switches"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Southwest Chest (Fall)"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Narrow Platform"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B2) Far North"), Item::DashBoots);
    layout.set(Location::new(regions::dungeons::ice::basement2::SUBREGION, "(B4) Southeast Chest (Fall)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::ice::boss::SUBREGION, "Dharkstare"), Item::KeyBoss);

    // Desert Palace
    layout.set(Location::new(regions::dungeons::desert::floor1::SUBREGION, "(1F) Entrance"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor1::SUBREGION, "(1F) Sand Room (South)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor1::SUBREGION, "(1F) Sand Switch Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor1::SUBREGION, "(1F) Sand Room (North)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::center::SUBREGION, "(1F) Big Chest (Behind Wall)"), Item::KeyBoss);
    layout.set(Location::new(regions::dungeons::desert::center::SUBREGION, "(1F) Behind Rocks"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Under Rock (Left)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Beamos Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Under Rock (Right)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Under Rock (Ball Room)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Big Chest (Puzzle)"), Item::PowerfulGlove);
    layout.set(Location::new(regions::dungeons::desert::floor2::SUBREGION, "(2F) Red/Blue Switches"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor2west::SUBREGION, "(2F) Leever Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor3::SUBREGION, "(3F) Silver Rupee"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::floor3::SUBREGION, "(3F) Armos Room"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::desert::boss::SUBREGION, "Zaganaga"), Item::RupeeGold);

    // Turtle Rock
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Center"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Grate Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Portal Room (Northwest)"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Northeast Ledge"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Southeast Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(1F) Defeat Flamolas"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Northeast Room"), Item::KeySmall);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Grate Chest (Small)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Big Chest (Center)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Platform"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::rock::SUBREGION, "(B1) Big Chest (Top)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::turtle::boss::SUBREGION, "Grinexx"), Item::KeyBoss);

    // Lorule Castle
    layout.set(Location::new(regions::dungeons::castle::lorule::SUBREGION, "(1F) Ledge"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::lorule::SUBREGION, "(1F) Center"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::lorule::SUBREGION, "(2F) Near Torches"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::lorule::SUBREGION, "(2F) Hidden Path"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::lorule::SUBREGION, "(2F) Ledge"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::floor4::SUBREGION, "(4F) Center"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::floor4::SUBREGION, "(4F) Hidden Path"), Item::ItemBowLight);
    layout.set(Location::new(regions::dungeons::castle::bomb_trial::SUBREGION, "(3F) Bomb Trial (Chest)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::bomb_trial::SUBREGION, "(3F) Bomb Trial (Behind Rock)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::ball_trial::SUBREGION, "(3F) Ball Trial (Chest)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::ball_trial::SUBREGION, "(3F) Ball Trial (Puzzle)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::lamp_trial::SUBREGION, "(4F) Lamp Trial"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "(4F) Hookshot Trial (Chest)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "(4F) Hookshot Trial (Eyes)"), Item::RupeeGold);
    layout.set(Location::new(regions::dungeons::castle::boss::SUBREGION, "Zelda"), Item::ItemBow);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Delivery"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Dampe"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Rosso Cave"), Item::ItemInsectNet);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Sanctuary Pegs"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Treasure Room"), Item::ItemBoomerangLv2);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Behind Blacksmith"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Blacksmith Cave"), Item::ItemSwordLv1);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Blacksmith"), Item::ItemSwordLv1);
    layout.set(Location::new(regions::hyrule::field::main::SUBREGION, "Castle Rocks"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::post_sanc::SUBREGION, "Thanks"), Item::BadgeBee);
    layout.set(Location::new(regions::hyrule::field::post_eastern::SUBREGION, "Rosso"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::post_eastern::SUBREGION, "Clean Rocks"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::post_eastern::SUBREGION, "Irene"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::post_eastern::SUBREGION, "Woods"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (1)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (2)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (3)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (4)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::post_sanc::SUBREGION, "Ravio (5)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (6)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (7)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (8)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (9)"), Item::HeartContainer);
    layout.set(Location::new(regions::hyrule::field::rupee_rush::SUBREGION, "Rupee Rush"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::castle::SUBREGION, "Castle (Indoors)"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::castle::SUBREGION, "Castle Balcony"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::field::sanctuary_cave::SUBREGION, "Sanctuary Cave"), Item::RupeeGold);

    // Lost Woods
    layout.set(Location::new(regions::hyrule::lost::woods::SUBREGION, "Pedestal"), Item::ItemBottle);
    layout.set(Location::new(regions::hyrule::lost::woods::SUBREGION, "Alcove"), Item::ItemHookShot);
    layout.set(Location::new(regions::hyrule::lost::woods::SUBREGION, "Chest"), Item::ItemIceRod);

    // Death Mountain
    layout.set(Location::new(regions::hyrule::death::mountain::SUBREGION, "First Cave"), Item::PowerGlove);
    layout.set(Location::new(regions::hyrule::death::mountain::SUBREGION, "Blocked Cave"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::mountain::SUBREGION, "Fairy Cave"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::upper::SUBREGION, "Ledge Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Pegs)"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Top)"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::upper::SUBREGION, "Hidden Area"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::east::SUBREGION, "Ore Mine Column"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::death::east::SUBREGION, "Bouldering Guy"), Item::HyruleShield);
    layout.set(Location::new(regions::hyrule::death::east::SUBREGION, "Treasure Room"), Item::ItemHookShotLv2);
    layout.set(Location::new(regions::hyrule::death::far_island::SUBREGION, "Distant Pillar"), Item::RupeeGold);

    // Sanctuary
    layout.set(Location::new(regions::hyrule::sanctuary::lobby::SUBREGION, "Entrance"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::sanctuary::inside::SUBREGION, "Lower Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::sanctuary::inside::SUBREGION, "Upper Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::sanctuary::inside::SUBREGION, "Ledge"), Item::RupeeGold);

    // Kakariko
    layout.set(Location::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Chest)"), Item::ClothesBlue);
    layout.set(Location::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Upper)"), Item::ClothesBlue);
    layout.set(Location::new(regions::hyrule::kakariko::village::SUBREGION, "Jail"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Merchant (Left)"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy"), Item::HintGlasses);
    layout.set(Location::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy (Golden Bee)"), Item::ItemFireRod);
    layout.set(Location::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Fortune Teller"), Item::Pouch);
    layout.set(Location::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Milk Bar Owner"), Item::MilkMatured);
    layout.set(Location::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Cucco Ranch"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Shady Guy"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Merchant (Right)"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::kakariko::closed::SUBREGION, "Stylish Woman"), Item::RupeeGold);

    // Zora's Domain
    layout.set(Location::new(regions::hyrule::zoras::domain::SUBREGION, "Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::zoras::domain::SUBREGION, "Behind Waterfall"), Item::ItemSwordLv1);
    layout.set(Location::new(regions::hyrule::zoras::domain::SUBREGION, "Zora Queen"), Item::RupeeGold);

    // Eastern Ruins
    layout.set(Location::new(regions::hyrule::eastern::hill::SUBREGION, "Treasure Room"), Item::ItemHammerLv2);
    layout.set(Location::new(regions::hyrule::eastern::hill::SUBREGION, "Armos Chest"), Item::ItemTornadeRod);
    layout.set(Location::new(regions::hyrule::eastern::hill::SUBREGION, "Hookshot Chest"), Item::ItemSandRod);
    layout.set(Location::new(regions::hyrule::eastern::hill::SUBREGION, "Merge Chest"), Item::ItemBoomerang);
    layout.set(Location::new(regions::hyrule::eastern::hill::SUBREGION, "Cave"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::eastern::hill::SUBREGION, "Pegs (South)"), Item::RupeeGold);

    // Southern Ruins
    layout.set(Location::new(regions::hyrule::southern::ruins::SUBREGION, "Runaway Item Seller"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::southern::ruins::SUBREGION, "Behind Pillars"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::southern::ruins::SUBREGION, "Treasure Room"), Item::ItemHammer);
    layout.set(Location::new(regions::hyrule::southern::ruins::SUBREGION, "Ledge"), Item::RupeeGold);

    // Lake Hylia
    layout.set(Location::new(regions::hyrule::lake::hylia::SUBREGION, "Torch Cave"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::lake::hylia::SUBREGION, "Ledge Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::lake::hylia::SUBREGION, "Bird Lover"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::lake::hylia::SUBREGION, "Secret Cave"), Item::RupeeGold);
    layout.set(Location::new(regions::hyrule::lake::hylia::SUBREGION, "Shore"), Item::MessageBottle);
    layout.set(Location::new(regions::hyrule::lake::hotfoot::SUBREGION, "Hyrule Hotfoot"), Item::RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Treasure Room"), Item::GanbariPowerUp);
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Vacant House"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Rupee Rush"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Great Rupee Fairy"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Big Bomb Cave"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Octoball Derby"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::main::SUBREGION, "Blacksmith"), Item::ItemKandelaar);
    layout.set(Location::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Left)"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Middle)"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Right)"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::thief_girl::SUBREGION, "Thief Girl"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::field::ledge::SUBREGION, "Hookshot Ledge"), Item::RupeeGold);

    // Skull Woods (overworld)
    layout.set(Location::new(regions::lorule::skull::woods::SUBREGION, "Alcove"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::skull::woods::SUBREGION, "Balcony"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::skull::chest::SUBREGION, "Chest"), Item::RupeeGold);

    // Lorule Death Mountain
    layout.set(Location::new(regions::lorule::death::mountain::SUBREGION, "Ledge (East)"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::death::mountain::SUBREGION, "Behind Ice Gimos"), Item::ItemFireRodLv2);
    layout.set(Location::new(regions::lorule::death::west::SUBREGION, "Ledge (West)"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::death::west::SUBREGION, "Ice Gimos (West)"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::death::tower::SUBREGION, "Treacherous Tower (Intermediate)"), Item::RupeeGold);

    // Lorule Graveyard
    layout.set(Location::new(regions::lorule::graveyard::cave::SUBREGION, "Big Chest"), Item::OreBlue);
    layout.set(Location::new(regions::lorule::graveyard::field::SUBREGION, "Field Chest"), Item::RupeeGold);

    // Dark Ruins
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Lake Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Maze Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Maze Ledge"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (1)"), Item::RupeeG);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (2)"), Item::RupeeB);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (3)"), Item::RupeeR);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (4)"), Item::RupeePurple);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (5)"), Item::RupeeSilver);
    layout.set(Location::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (6)"), Item::SpecialMove);

    // Misery Mire
    layout.set(Location::new(regions::lorule::misery::mire::SUBREGION, "Ledge"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::misery::mire::SUBREGION, "Treasure Room"), Item::ItemSandRodLv2);

    // Lake Lolia
    layout.set(Location::new(regions::lorule::lake::lorule::SUBREGION, "Chest"), Item::RupeeGold);
    layout.set(Location::new(regions::lorule::lake::balcony::SUBREGION, "Balcony"), Item::ItemMizukaki);

    let spoiler = Spoiler {
        seed: 0,
        settings: &settings,
        layout
    };

    spoiler.patch(
        system.get_or_create_paths(create_paths)?,
        true,
        true,
    )
}


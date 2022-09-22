use std::{collections::BTreeMap, error::Error as StdError, fs, fs::File, io};
use std::io::{stdin, stdout, Write};
use std::path::Path;

use linked_hash_map::LinkedHashMap;
use log::{debug, info};
use serde::{ser::SerializeMap, Serialize, Serializer};

use albw::{Game, Item};
use albw::Item::*;
use patch::Patcher;
use regions::Subregion;
pub use settings::Settings;
use state::State;
use sys::{Paths, System};

use crate::filler::fill_stuff;
use crate::filler_item::{convert, FillerItem};
use crate::settings::plando_settings;

mod graph;
mod patch;
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
mod logic;
pub mod logic_mode;

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
        RingHekiga => "Progressive Bracelet",
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
            let path = paths.output().join(format!("spoiler {}.json", self.seed));
            info!("Writing spoiler to:             {}", path.display());

            serde_json::to_writer_pretty(File::create(path)?, &self)
                .expect("Could not write the spoiler log.");
        }
        Ok(())
    }
}

/// Gets the system object for the platform.
pub fn system() -> sys::Result<System<Settings>> {
    System::new()
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
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Outside (East)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Near Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "[EP] (1F) Defeat Popos"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "[EP] (1F) Hidden Door"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "[EP] (1F) Switch Puzzle"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Ball Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Defeat Popos"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Switch Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Big Chest"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::eastern::boss::SUBREGION, "[EP] (3F) After Cutscene"), RingRental);
    layout.set(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "[EP] Yuga"), ItemBell);
    layout.set(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "[EP] (3F) Outside (North)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "[EP] (1F) Outside (West)"), ItemInsectNetLv2);

    // House of Gales
    layout.set(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "[HoG] (1F) Torches"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "[HoG] (1F) Switch Room"), ItemBomb);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "[HoG] (1F) Fire Bubbles"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1west::SUBREGION, "[HoG] (1F) Blue Bari Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor1west::SUBREGION, "[HoG] (1F) Blue Bari Room (Bottom Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor2::SUBREGION, "[HoG] (2F) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::house::floor2::SUBREGION, "[HoG] (2F) Narrow Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::floor2outer::SUBREGION, "[HoG] (2F) Fire Ring"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::floor3::SUBREGION, "[HoG] (3F) Rat Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::floor3::SUBREGION, "[HoG] (3F) Fire Bubbles"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::boss::SUBREGION, "[HoG] Margomill"), RupeeGold);

    // Tower of Hera
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[ToH] (1F) Outside"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor2::SUBREGION, "[ToH] (1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor2::SUBREGION, "[ToH] (3F) Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "[ToH] (5F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "[ToH] (6F) Left Mole"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "[ToH] (6F) Right Mole"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "[ToH] (7F) Outside (Ledge)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "[ToH] (8F) Fairy Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "[ToH] (11F) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::tower::boss::SUBREGION, "[ToH] Moldorm"), RingHekiga);

    // Lorule Sanctuary
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Entrance Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Lower Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Ledge"), KeySmall);

    // Dark Palace
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PoD] (1F) Near Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PoD] (1F) Narrow Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (1F) Switch Puzzle"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (1F) Hidden Room (Upper)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (1F) Hidden Room (Lower)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Fall From 1F"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Maze"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Helmasaur Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Helmasaur Room (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (2F) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (2F) South Hidden Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (2F) Alcove"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (1F) Fall From 2F"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::boss_key::SUBREGION, "[PoD] (B1) Big Chest (Switches)"), OreGreen);
    layout.set(LocationInfo::new(regions::dungeons::dark::boss::SUBREGION, "[PoD] Gemesaur King"), RupeeGold);

    // Swamp Palace
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Raft Room (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Raft Room (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Gyorm"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Waterfall Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Raft Room (Pillar)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Big Chest (Secret)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) Water Puzzle"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) East Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) West Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) Big Chest (Fire)"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::dungeons::swamp::boss::SUBREGION, "[SP] Arrghus"), KeyBoss);

    // Skull Woods
    layout.set(LocationInfo::new(regions::dungeons::skull::palace::SUBREGION, "[SW] (B1) Gibdo Room (Lower)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::palace::SUBREGION, "[SW] (B1) South Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::skull::outdoors::SUBREGION, "[SW] (B1) Gibdo Room (Hole)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::outdoors::SUBREGION, "[SW] (B1) Grate Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::basement2::SUBREGION, "[SW] (B2) Moving Platform Room"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::skull::end::SUBREGION, "[SW] (B1) Big Chest (Upper)"), ItemKandelaarLv2);
    layout.set(LocationInfo::new(regions::dungeons::skull::end::SUBREGION, "[SW] (B1) Big Chest (Eyes)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::skull::boss::SUBREGION, "[SW] Knucklemaster"), OreRed);

    // Thieves' Hideout
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[TH] (B1) Jail Cell"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[TH] (B1) Grate Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[TH] (B2) Grate Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "[TH] (B2) Switch Puzzle Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "[TH] (B2) Jail Cell"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "[TH] (B2) Eyegores"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B1) Behind Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B1) Big Chest (Entrance)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B3) Underwater"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B3) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::thieves::boss::SUBREGION, "Stalblind"), OreYellow);

    // Ice Ruins
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (1F) Hidden Chest"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Grate Chest (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Grate Chest (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Ice Pillar"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B5) Big Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement1::SUBREGION, "[IR] (B1) East Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement1::SUBREGION, "[IR] (B1) Narrow Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B1) Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B3) Big Chest (Puzzle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Southwest Chest (Fall)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Narrow Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B2) Far North"), DashBoots);
    layout.set(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Southeast Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::boss::SUBREGION, "[IR] Dharkstare"), KeyBoss);

    // Desert Palace
    layout.set(LocationInfo::new(regions::dungeons::desert::floor1::SUBREGION, "[DP] (1F) Entrance"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Sand Room (South)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Sand Switch Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Sand Room (North)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Big Chest (Behind Wall)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Behind Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Under Rock (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Beamos Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Under Rock (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Under Rock (Ball Room)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Big Chest (Puzzle)"), PowerfulGlove);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor2west::SUBREGION, "[DP] (2F) Leever Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor3::SUBREGION, "[DP] (3F) Behind Falling Sand"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::floor3::SUBREGION, "[DP] (3F) Armos Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::boss::SUBREGION, "Zaganaga"), RupeeGold);

    // Turtle Rock
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Grate Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Portal Room (Northwest)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Northeast Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Southeast Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Defeat Flamolas"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Northeast Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Grate Chest (Small)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Big Chest (Center)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Big Chest (Top)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Under Center"), RupeeSilver);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Under Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::boss::SUBREGION, "[TR] Grinexx"), KeyBoss);

    // Lorule Castle
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (1F) Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (2F) Near Torches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (2F) Hidden Path"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (2F) Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::floor4::SUBREGION, "[LC] (4F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::floor4::SUBREGION, "[LC] (4F) Hidden Path"), ItemBowLight);
    layout.set(LocationInfo::new(regions::dungeons::castle::bomb_trial::SUBREGION, "[LC] (3F) Bomb Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::bomb_trial::SUBREGION, "[LC] (3F) Bomb Trial (Behind Rock)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::ball_trial::SUBREGION, "[LC] (3F) Ball Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::ball_trial::SUBREGION, "[LC] (3F) Ball Trial (Puzzle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::lamp_trial::SUBREGION, "[LC] (4F) Lamp Trial"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "[LC] (4F) Hookshot Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "[LC] (4F) Hookshot Trial (Eyes)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::castle::boss::SUBREGION, "[LC] Zelda"), ItemBow);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Delivery"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Dampe"), RingRental);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Rosso Cave"), ItemInsectNet);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Sanctuary Pegs"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Sanctuary Treasure Dungeon"), ItemBoomerangLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Behind Blacksmith"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith Cave"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Castle Rocks"), RupeeGold);
    //layout.set(Location::new(regions::hyrule::field::post_sanc::SUBREGION, "Thanks"), Item::BadgeBee);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Rosso"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Clean Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Irene"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Haunted Grove Tree Stump"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Cucco Dungeon"), RupeeSilver);


    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (1)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (2)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (3)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (4)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (5)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (6)"), RupeeGold); // Sand Rod Slot
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (7)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (8)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (9)"), RupeeGold);


    layout.set(LocationInfo::new(regions::hyrule::field::rupee_rush::SUBREGION, "Rupee Rush (Hyrule)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::castle::SUBREGION, "Castle (Indoors)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::castle::SUBREGION, "Castle Balcony"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::sanctuary_cave::SUBREGION, "Sanctuary Cave"), RupeeGold);

    // Lost Woods
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Master Sword Pedestal"), ItemBottle);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Alcove"), ItemHookShot);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Lost Woods Big Rock Chest"), ItemIceRod);

    // Death Mountain
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "First Cave"), PowerGlove);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Blocked Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Fairy Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Death Mountain West Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Pegs)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Top)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Spectacle Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Fire Cave Pillar"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Bouldering Guy"), HyruleShield);
    layout.set(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Hookshot Treasure Dungeon"), ItemHookShotLv2);
    layout.set(LocationInfo::new(regions::hyrule::death::far_island::SUBREGION, "Floating Island"), RupeeGold);

    // Sanctuary
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::lobby::SUBREGION, "[HS] Entrance"), KeySmall);
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "[HS] Lower Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "[HS] Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "[HS] Ledge"), RupeeGold);

    // Kakariko
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Upper)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Jail"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Merchant (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Merchant (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy"), HintGlasses);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy (Golden Bee)"), ItemFireRod);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Fortune Teller"), RingRental);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Milk Bar Owner"), MilkMatured);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Cucco Ranch"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Shady Guy"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::closed::SUBREGION, "Stylish Woman"), RupeeGold);

    // Zora's Domain
    layout.set(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Zora's Domain Ledge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Behind Waterfall"), Kinsta);
    layout.set(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Zora Queen"), RupeeGold);

    // Eastern Ruins
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Merge Treasure Dungeon"), ItemHammerLv2);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Armos Chest"), ItemTornadeRod);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Hookshot Chest"), ItemSandRod);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Merge Chest"), ItemBoomerang);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Pegs (South)"), RupeeGold);

    // Southern Ruins
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Runaway Item Seller"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Behind Pillars"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Treasure Room"), ItemHammer);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Southern Ruins Ledge"), RupeeGold);

    // Lake Hylia
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Torch Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lake Hylia Ledge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Bird Lover"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Secret Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Shore"), MessageBottle);
    layout.set(LocationInfo::new(regions::hyrule::lake::hotfoot::SUBREGION, "Hyrule Hotfoot"), RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Boots Treasure Dungeon"), GanbariPowerUp);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Vacant House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Rupee Rush (Lorule)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Great Rupee Fairy"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Big Bomb Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Octoball Derby"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Blacksmith (Lorule)"), ItemKandelaar);
    layout.set(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Middle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::thief_girl::SUBREGION, "Thief Girl Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::ledge::SUBREGION, "Hookshot Ledge"), RupeeGold);

    // Skull Woods (overworld)
    layout.set(LocationInfo::new(regions::lorule::skull::woods::SUBREGION, "Canyon House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::woods::SUBREGION, "Cucco Shack"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::chest::SUBREGION, "Skull Woods Outdoor Chest"), RupeeGold);

    // Lorule Death Mountain
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Ledge (East)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Behind Ice Gimos (East)"), ItemFireRodLv2);
    layout.set(LocationInfo::new(regions::lorule::death::west::SUBREGION, "Ledge (West)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::west::SUBREGION, "Defeat Ice Gimos (West)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::tower::SUBREGION, "Treacherous Tower (Intermediate)"), RupeeGold);

    // Lorule Graveyard
    layout.set(LocationInfo::new(regions::lorule::graveyard::cave::SUBREGION, "Philosopher's Cave Big Chest"), OreBlue);
    layout.set(LocationInfo::new(regions::lorule::graveyard::field::SUBREGION, "Peninsula Chest"), RupeeGold);

    // Dark Ruins
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Ruins Lakeview Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Ledge"), HeartPiece);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (1)"), RupeeG);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (2)"), RupeeB);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (3)"), RupeeR);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (4)"), RupeePurple);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (5)"), RupeeSilver);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (6)"), SpecialMove);

    // Misery Mire
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Misery Mire Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Sand Rod Treasure Dungeon"), ItemSandRodLv2);

    // Lake Lolia
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lake NW Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::balcony::SUBREGION, "Turtle Rock Left Balcony"), ItemMizukaki);


    ////////////////////////////
    // --- Hyrule Maiamai --- //
    ////////////////////////////

    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Rosso Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Lost Woods Path Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Lost Woods Bush"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Lost Woods Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Fortune-Teller Tent"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Moldorm Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Small Pond"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Lost Woods Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Death Mountain West Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Death Mountain West Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Death Mountain East Ledge Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Rosso's Ore Mine Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Outside Hookshot Dungeon"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Sanctuary Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Zora's Domain Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Zora's Domain South Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Under Wooden Bridge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Hyrule Graveyard Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Waterfall Ledge Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Inside Witch's House"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Kakariko Bush"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Woman's Roof Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Kakariko Sand"), ItemBowLight);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Hyrule Rupee Rush Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Cucco Ranch Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Cucco Dungeon Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Behind Blacksmith Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Hyrule Castle West Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Hyrule Castle Wind Tile"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Blacksmith Wind Tile"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Haunted Grove Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Link's House Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Behind Link's House"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Eastern Ruins Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Eastern Ruins Yellow Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Eastern Ruins Green Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Eastern Ruins Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Atop Eastern Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Southern Bridge River"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Buried in the Desert"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Buried near Desert Palace"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Southern Ruins Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Southern Ruins Pillars"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Outside Flippers Dungeon"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Southern Ruins Bomb Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Outside Maiamai Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Island Wind Tile"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Hyrule Hotfoot Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Lake Hylia Shallow Ring"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::maiamai::maiamai::SUBREGION, "[Mai] Lake Hylia SE Wall"), RupeeGold);

    ////////////////////////////
    // --- Lorule Maiamai --- //
    ////////////////////////////

    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Grass"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Shack Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Bush"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Entrance Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Skull Woods Dry Pond"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Canyon House Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Death Mountain West Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Death Mountain West Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Death Mountain East Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Death Mountain East Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Outside Ice Ruins"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Death Mountain East Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Graveyard Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Graveyard Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Graveyard Peninsula Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Ku's Domain Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Dark Ruins Waterfall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Outside Hinox Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Ku's Domain Grass"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Dark Maze Entrance Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Dark Maze Center Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Atop Dark Ruins Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Dark Ruins West Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Dark Ruins East Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Dark Ruins South Area Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Thieves' Town Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Fortune-Teller Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Castle Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Castle Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Blacksmith Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Rupee Rush Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Big Bomb Flower Field Grass"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Octoball Derby Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Vacant House Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Behind Vacant House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Haunted Grove Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Southern Ruins Pillars"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Southern Ruins Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Southern Ruins Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Thieves' Town Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Misery Mire Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Misery Mire Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Misery Mire Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Lake West Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Lake Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Lake Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Lake Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::maiamai::maiamai::SUBREGION, "[Mai] Lorule Lake SE Wall"), RupeeGold);

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

pub fn filler_new(settings: &Settings, seed: Seed) -> Spoiler {

    // New Filler
    let filled: Vec<(LocationInfo, Item)> = fill_stuff(settings, seed);

    // Build legacy Layout object
    let mut layout = Layout::default();
    for (location_info, item) in filled {
        layout.set(location_info, item);
    }

    Spoiler {
        seed,
        settings,
        layout,
    }
}
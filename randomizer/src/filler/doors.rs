use log::info;
use rand::prelude::StdRng;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use modinfo::Settings;
use modinfo::settings::DoorShuffle;
use serde::{Serialize, Serializer};

use crate::{DashMap, DoorMap, filler};
use rom::scene::SpawnPoint;

use crate::filler::filler_item::Randomizable;
use crate::filler::item_pools;
use crate::filler::location::Location;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Door {
    EasternPalaceEntrance,
    EasternPalaceExit,
    HouseOfGalesEntrance,
    HouseOfGalesExit,
    TowerOfHeraEntrance,
    TowerOfHeraExit,

    InsideHyruleCastleEntrance,
    InsideHyruleCastleExit,

    DarkPalaceEntrance,
    DarkPalaceExit,
    SwampPalaceEntrance,
    SwampPalaceExit,
    SkullWoodsEntrance,
    SkullWoodsExit,
    ThievesHideoutEntrance,
    ThievesHideoutExit,
    TurtleRockEntrance,
    TurtleRockExit,
    DesertPalaceEntrance,
    DesertPalaceExit,
    IceRuinsEntrance,
    IceRuinsExit,

    LoruleCastleEntrance,
    LoruleCastleExit,
}

impl Door {
    ///
    pub fn get_vanilla_destination(&self) -> Self {
        use self::Door::*;
        match self {
            EasternPalaceEntrance => EasternPalaceExit,
            EasternPalaceExit => EasternPalaceEntrance,
            HouseOfGalesEntrance => HouseOfGalesExit,
            HouseOfGalesExit => HouseOfGalesEntrance,
            TowerOfHeraEntrance => TowerOfHeraExit,
            TowerOfHeraExit => TowerOfHeraEntrance,
            InsideHyruleCastleEntrance => InsideHyruleCastleExit,
            InsideHyruleCastleExit => InsideHyruleCastleEntrance,
            DarkPalaceEntrance => DarkPalaceExit,
            DarkPalaceExit => DarkPalaceEntrance,
            SwampPalaceEntrance => SwampPalaceExit,
            SwampPalaceExit => SwampPalaceEntrance,
            SkullWoodsEntrance => SkullWoodsExit,
            SkullWoodsExit => SkullWoodsEntrance,
            ThievesHideoutEntrance => ThievesHideoutExit,
            ThievesHideoutExit => ThievesHideoutEntrance,
            TurtleRockEntrance => TurtleRockExit,
            TurtleRockExit => TurtleRockEntrance,
            DesertPalaceEntrance => DesertPalaceExit,
            DesertPalaceExit => DesertPalaceEntrance,
            IceRuinsEntrance => IceRuinsExit,
            IceRuinsExit => IceRuinsEntrance,
            LoruleCastleEntrance => LoruleCastleExit,
            LoruleCastleExit => LoruleCastleEntrance,
        }
    }

    ///
    pub fn get_location(&self) -> Location {
        use crate::filler::location::Location::*;
        match self {
            Door::EasternPalaceEntrance => EasternRuinsUpper,
            Door::EasternPalaceExit => EasternPalaceFoyer,

            Door::HouseOfGalesEntrance => HouseOfGalesIsland,
            Door::HouseOfGalesExit => HouseOfGalesFoyer,

            Door::TowerOfHeraEntrance => DeathMountainWestTop,
            Door::TowerOfHeraExit => TowerOfHeraFoyer,

            Door::InsideHyruleCastleEntrance => HyruleCastleRoof,
            Door::InsideHyruleCastleExit => HyruleCastleDungeon,

            Door::DarkPalaceEntrance => DarkPalaceWeatherVane,
            Door::DarkPalaceExit => DarkPalaceFoyer,

            Door::SwampPalaceEntrance => SwampPalaceAntechamber,
            Door::SwampPalaceExit => SwampPalaceFoyer,

            Door::SkullWoodsEntrance => SkullWoodsOverworld,
            Door::SkullWoodsExit => SkullWoodsFoyer,

            Door::ThievesHideoutEntrance => LoruleCastleArea,
            Door::ThievesHideoutExit => ThievesHideoutB1,

            Door::TurtleRockEntrance => TurtleRockFrontDoor,
            Door::TurtleRockExit => TurtleRockFoyer,

            Door::DesertPalaceEntrance => DesertPalaceWeatherVane,
            Door::DesertPalaceExit => DesertPalaceFoyer,

            Door::IceRuinsEntrance => LoruleDeathEastTop,
            Door::IceRuinsExit => IceRuinsFoyer,

            Door::LoruleCastleEntrance => LoruleCastleArea,
            Door::LoruleCastleExit => LoruleCastle1F,
        }
    }

    pub fn get_spawn_point(&self) -> SpawnPoint {
        use game::Course::*;
        let (course, scene, spawn) = match self {
            Door::EasternPalaceEntrance => (FieldLight, 20, 0),
            Door::EasternPalaceExit => (DungeonEast, 1, 0),
            Door::HouseOfGalesEntrance => (FieldLight, 35, 0),
            Door::HouseOfGalesExit => (DungeonWind, 1, 0),
            Door::TowerOfHeraEntrance => (FieldLight, 3, 3),
            Door::TowerOfHeraExit => (DungeonHera, 1, 0),
            Door::InsideHyruleCastleEntrance => (FieldLight, 18, 0),
            Door::InsideHyruleCastleExit => (DungeonCastle, 1, 0),
            Door::DarkPalaceEntrance => (FieldDark, 20, 5),
            Door::DarkPalaceExit => (DungeonDark, 2, 0),
            Door::SwampPalaceEntrance => (CaveDark, 1, 0),
            Door::SwampPalaceExit => (DungeonWater, 1, 0),
            Door::SkullWoodsEntrance => (FieldDark, 1, 10),
            Door::SkullWoodsExit => (DungeonDokuro, 1, 0),
            Door::ThievesHideoutEntrance => (FieldDark, 16, 5),
            Door::ThievesHideoutExit => (DungeonHagure, 1, 0),
            Door::TurtleRockEntrance => (FieldDark, 35, 6),
            Door::TurtleRockExit => (DungeonKame, 1, 0),
            Door::DesertPalaceEntrance => (FieldLight, 31, 2),
            Door::DesertPalaceExit => (DungeonSand, 1, 0),
            Door::IceRuinsEntrance => (FieldDark, 5, 0),
            Door::IceRuinsExit => (DungeonIce, 1, 0),
            Door::LoruleCastleEntrance => (FieldDark, 18, 0),
            Door::LoruleCastleExit => (DungeonGanon, 1, 0),
        };

        SpawnPoint { course, scene, spawn }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Door::EasternPalaceEntrance => "Eastern Palace Entrance",
            Door::EasternPalaceExit => "Eastern Palace Exit",
            Door::HouseOfGalesEntrance => "House of Gales Entrance",
            Door::HouseOfGalesExit => "House of Gales Exit",
            Door::TowerOfHeraEntrance => "Tower of Hera Entrance",
            Door::TowerOfHeraExit => "Tower of Hera Exit",
            Door::InsideHyruleCastleEntrance => "Inside Hyrule Castle Entrance",
            Door::InsideHyruleCastleExit => "Inside Hyrule Castle Exit",
            Door::DarkPalaceEntrance => "Dark Palace Entrance",
            Door::DarkPalaceExit => "Dark Palace Exit",
            Door::SwampPalaceEntrance => "Swamp Palace Entrance",
            Door::SwampPalaceExit => "Swamp Palace Exit",
            Door::SkullWoodsEntrance => "Skull Woods Entrance",
            Door::SkullWoodsExit => "Skull Woods Exit",
            Door::ThievesHideoutEntrance => "Thieves' Hideout Entrance",
            Door::ThievesHideoutExit => "Thieves' Hideout Exit",
            Door::TurtleRockEntrance => "Turtle Rock Entrance",
            Door::TurtleRockExit => "Turtle Rock Exit",
            Door::DesertPalaceEntrance => "Desert Palace Entrance",
            Door::DesertPalaceExit => "Desert Palace Exit",
            Door::IceRuinsEntrance => "Ice Ruins Entrance",
            Door::IceRuinsExit => "Ice Ruins Exit",
            Door::LoruleCastleEntrance => "Lorule Castle Entrance",
            Door::LoruleCastleExit => "Lorule Castle Exit",
        }
    }

    pub fn get_world(&self) -> game::World {
        match self {
            Door::EasternPalaceEntrance
            | Door::EasternPalaceExit
            | Door::HouseOfGalesEntrance
            | Door::HouseOfGalesExit
            | Door::TowerOfHeraEntrance
            | Door::TowerOfHeraExit
            | Door::InsideHyruleCastleEntrance
            | Door::InsideHyruleCastleExit => game::World::Hyrule,
            // --- //
            Door::DarkPalaceEntrance
            | Door::DarkPalaceExit
            | Door::SwampPalaceEntrance
            | Door::SwampPalaceExit
            | Door::SkullWoodsEntrance
            | Door::SkullWoodsExit
            | Door::ThievesHideoutEntrance
            | Door::ThievesHideoutExit
            | Door::TurtleRockEntrance
            | Door::TurtleRockExit
            | Door::DesertPalaceEntrance
            | Door::DesertPalaceExit
            | Door::IceRuinsEntrance
            | Door::IceRuinsExit
            | Door::LoruleCastleEntrance
            | Door::LoruleCastleExit => game::World::Lorule,
        }
    }
}

impl Display for Door {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Ord for Door {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd<Door> for Door {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Randomizable> for Door {
    fn from(filler_item: Randomizable) -> Self {
        match filler_item {
            Randomizable::Door(door) => door,
            _ => unreachable!("Not a Door: {:?}", filler_item),
        }
    }
}

impl Serialize for Door {
    fn serialize<S>(&self, serializer: S) -> crate::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

pub(crate) fn build_door_map(settings: &Settings, rng: &mut StdRng) -> crate::Result<DoorMap> {
    info!("Building Door Map...");
    let mut door_map: DashMap<_, _> = Default::default();

    let door_entrances = item_pools::get_door_entrances();
    let door_exits = item_pools::get_door_exits();

    match settings.door_shuffle {
        DoorShuffle::Off => {
            for entrance in door_entrances {
                let exit = entrance.get_vanilla_destination();

                door_map.insert(entrance, exit);
                door_map.insert(exit, entrance);
            }
        },
        DoorShuffle::DungeonEntrances => {
            let door_exits = filler::util::shuffle(rng, door_exits);

            for i in 0..door_entrances.len() {
                let entrance = *door_entrances.get(i).unwrap();
                let exit = *door_exits.get(i).unwrap();

                door_map.insert(entrance, exit);
                door_map.insert(exit, entrance);
            }
        },
    }

    Ok(door_map.iter().map(|(&a, &b)| (a, b)).collect())
}

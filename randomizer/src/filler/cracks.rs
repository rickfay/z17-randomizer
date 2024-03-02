use crate::filler::filler_item::Randomizable;
use crate::filler::item_pools;
use crate::filler::location::Location;
use crate::Result;
use crate::{filler, CrackMap, DashMap};
use game::Course::{CaveDark, FieldDark, FieldLight, IndoorDark, IndoorLight};
use log::info;
use modinfo::settings::cracksanity::Cracksanity;
use modinfo::Settings;
use rand::rngs::StdRng;
use rand::Rng;
use rom::flag::Flag;
use rom::scene::SpawnPoint;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

/// Crack item
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Crack {
    // --- Hyrule --- //
    StylishWoman,
    YourHouse,
    ParadoxRightHyrule,
    ParadoxLeftHyrule,
    WaterfallHyrule,
    EasternRuinsPillar,
    EasternRuinsSE,
    LostWoodsPillar,
    SahasrahlasHouse,
    RossosHouse,
    MiseryMireEntrance,
    DesertPillarRight,
    DesertPillarLeft,
    DesertMiddle,
    DesertSW,
    DesertPalace,
    DesertNorth,
    DeathWestHyrule,
    FloatingIslandHyrule,
    RiverHyrule,
    LakeHylia,
    HyruleHotfoot,
    Sanctuary,
    GraveyardLedgeHyrule,
    RossosOreMineHyrule,
    SwampPillarHyrule,
    ZorasDomain,
    HyruleCastle,

    // --- Lorule --- //
    ThievesTown,
    VacantHouse,
    ParadoxRightLorule,
    ParadoxLeftLorule,
    WaterfallLorule,
    DarkRuinsPillar,
    DarkRuinsSE,
    SkullWoodsPillar,
    NShapedHouse,
    DestroyedHouse,
    MiseryMireExit,
    MirePillarRight,
    MirePillarLeft,
    MireMiddle,
    MireSW,
    Zaganaga,
    MireNorth,
    DeathWestLorule,
    FloatingIslandLorule,
    RiverLorule,
    LoruleLake,
    LoruleHotfoot,
    Philosopher,
    GraveyardLedgeLorule,
    RossosOreMineLorule,
    SwampPillarLorule,
    KusDomain,
    LoruleCastle,
}

impl Crack {
    pub(crate) fn get_type(&self) -> i32 {
        use Crack::*;
        match self {
            YourHouse | WaterfallHyrule | EasternRuinsSE | MiseryMireEntrance | DesertMiddle | DesertSW
            | DesertPalace | DesertNorth | DeathWestHyrule | RossosOreMineHyrule | RiverHyrule | LakeHylia
            | HyruleHotfoot | GraveyardLedgeHyrule => 1,

            VacantHouse | WaterfallLorule | DarkRuinsSE | MiseryMireExit | MireMiddle | MireSW | Zaganaga
            | MireNorth | DeathWestLorule | RossosOreMineLorule | RiverLorule | LoruleLake | LoruleHotfoot
            | GraveyardLedgeLorule => 2,

            ZorasDomain => 3,

            KusDomain => 4,

            StylishWoman | LostWoodsPillar | RossosHouse | FloatingIslandHyrule => 5,

            ThievesTown | SkullWoodsPillar | DestroyedHouse | FloatingIslandLorule => 6,

            EasternRuinsPillar | SahasrahlasHouse | SwampPillarHyrule => 7,

            ParadoxRightLorule | ParadoxLeftLorule | DarkRuinsPillar | NShapedHouse | SwampPillarLorule => 8,

            HyruleCastle | DesertPillarRight | DesertPillarLeft | Sanctuary => 9,

            LoruleCastle | MirePillarRight | MirePillarLeft | Philosopher => 10,

            ParadoxRightHyrule | ParadoxLeftHyrule => 11,
        }
    }

    pub(crate) fn get_reverse_type(&self) -> i32 {
        use Crack::*;
        match self {
            VacantHouse | WaterfallLorule | DarkRuinsSE | MiseryMireExit | MireMiddle | MireSW | Zaganaga
            | MireNorth | DeathWestLorule | RossosOreMineLorule | RiverLorule | LoruleLake | LoruleHotfoot
            | GraveyardLedgeLorule => 9,

            YourHouse | WaterfallHyrule | EasternRuinsSE | MiseryMireEntrance | DesertMiddle | DesertSW
            | DesertPalace | DesertNorth | DeathWestHyrule | RossosOreMineHyrule | RiverHyrule | LakeHylia
            | HyruleHotfoot | GraveyardLedgeHyrule => 2,

            KusDomain => 3,

            ZorasDomain => 4,

            ThievesTown | SkullWoodsPillar | DestroyedHouse | FloatingIslandLorule => 5,

            StylishWoman | LostWoodsPillar | RossosHouse | FloatingIslandHyrule => 6,

            DarkRuinsPillar | NShapedHouse | SwampPillarLorule => 7,

            EasternRuinsPillar | SahasrahlasHouse | SwampPillarHyrule | ParadoxRightHyrule | ParadoxLeftHyrule => 8,

            LoruleCastle | MirePillarRight | MirePillarLeft | Philosopher => 9,

            HyruleCastle | DesertPillarRight | DesertPillarLeft | Sanctuary => 10,

            ParadoxRightLorule | ParadoxLeftLorule => 11,
        }
    }

    pub(crate) fn get_mirror_crack(&self) -> Crack {
        use Crack::*;
        match self {
            StylishWoman => ThievesTown,
            YourHouse => VacantHouse,
            ParadoxRightHyrule => ParadoxRightLorule,
            ParadoxLeftHyrule => ParadoxLeftLorule,
            WaterfallHyrule => WaterfallLorule,
            EasternRuinsPillar => DarkRuinsPillar,
            EasternRuinsSE => DarkRuinsSE,
            LostWoodsPillar => SkullWoodsPillar,
            SahasrahlasHouse => NShapedHouse,
            RossosHouse => DestroyedHouse,
            MiseryMireEntrance => MiseryMireExit,
            DesertPillarRight => MirePillarRight,
            DesertPillarLeft => MirePillarLeft,
            DesertMiddle => MireMiddle,
            DesertSW => MireSW,
            DesertPalace => Zaganaga,
            DesertNorth => MireNorth,
            DeathWestHyrule => DeathWestLorule,
            FloatingIslandHyrule => FloatingIslandLorule,
            RiverHyrule => RiverLorule,
            LakeHylia => LoruleLake,
            HyruleHotfoot => LoruleHotfoot,
            Sanctuary => Philosopher,
            GraveyardLedgeHyrule => GraveyardLedgeLorule,
            RossosOreMineHyrule => RossosOreMineLorule,
            SwampPillarHyrule => SwampPillarLorule,
            ZorasDomain => KusDomain,
            HyruleCastle => LoruleCastle,
            ThievesTown => StylishWoman,
            VacantHouse => YourHouse,
            ParadoxRightLorule => ParadoxRightHyrule,
            ParadoxLeftLorule => ParadoxLeftHyrule,
            WaterfallLorule => WaterfallHyrule,
            DarkRuinsPillar => EasternRuinsPillar,
            DarkRuinsSE => EasternRuinsSE,
            SkullWoodsPillar => LostWoodsPillar,
            NShapedHouse => SahasrahlasHouse,
            DestroyedHouse => RossosHouse,
            MiseryMireExit => MiseryMireEntrance,
            MirePillarRight => DesertPillarRight,
            MirePillarLeft => DesertPillarLeft,
            MireMiddle => DesertMiddle,
            MireSW => DesertSW,
            Zaganaga => DesertPalace,
            MireNorth => DesertNorth,
            DeathWestLorule => DeathWestHyrule,
            FloatingIslandLorule => FloatingIslandHyrule,
            RiverLorule => RiverHyrule,
            LoruleLake => LakeHylia,
            LoruleHotfoot => HyruleHotfoot,
            Philosopher => Sanctuary,
            GraveyardLedgeLorule => GraveyardLedgeHyrule,
            RossosOreMineLorule => RossosOreMineHyrule,
            SwampPillarLorule => SwampPillarHyrule,
            KusDomain => ZorasDomain,
            LoruleCastle => HyruleCastle,
        }
    }

    pub(crate) fn get_left_right_locations(&self) -> (Location, Location) {
        use crate::filler::location::Location::*;
        use Crack::*;
        match self {
            StylishWoman => (StylishWomanHouse, StylishWomanHouse),
            YourHouse => (HyruleField, HyruleField),
            ParadoxRightHyrule => (HyruleField, HyruleField),
            ParadoxLeftHyrule => (CuccoDungeonLedge, CuccoDungeonLedge),
            WaterfallHyrule => (ZoraRiver, WaterfallLedge),
            EasternRuinsPillar => (HyruleField, HyruleField),
            EasternRuinsSE => (EasternRuinsBlockedCrack, EasternRuinsBlockedCrack),
            LostWoodsPillar => (HyruleField, HyruleField),
            SahasrahlasHouse => (HyruleField, HyruleField),
            Crack::RossosHouse => (Location::RossosHouse, Location::RossosHouse),
            MiseryMireEntrance => (HyruleField, HyruleField),
            DesertPillarRight => (Desert, Desert),
            DesertPillarLeft => (Desert, Desert),
            DesertMiddle => (Desert, DesertCenterLedge),
            DesertSW => (DesertSouthWestLedge, DesertSouthWestLedge),
            DesertPalace => (DesertZaganagaLedge, DesertZaganagaLedge),
            DesertNorth => (Desert, Desert),
            DeathWestHyrule => (DeathMountainBase, DeathMountainBase),
            Crack::FloatingIslandHyrule => (Location::FloatingIslandHyrule, Location::FloatingIslandHyrule),
            RiverHyrule => (BridgeShallowWater, Location::LakeHylia),
            Crack::LakeHylia => (HyruleField, HyruleField),
            HyruleHotfoot => (HyruleField, HyruleField),
            Crack::Sanctuary => (SanctuaryChurch, SanctuaryChurch),
            Crack::GraveyardLedgeHyrule => (Location::GraveyardLedgeHyrule, Location::GraveyardLedgeHyrule),
            RossosOreMineHyrule => (RossosOreMine, RossosOreMine),
            SwampPillarHyrule => (HyruleField, HyruleField),
            ZorasDomain => (ZoraDomainArea, ZoraDomainArea),
            HyruleCastle => (ZeldasStudy, ZeldasStudy),
            // --- //
            ThievesTown => (LoruleCastleArea, LoruleCastleArea),
            VacantHouse => (LoruleCastleArea, LoruleCastleArea),
            ParadoxRightLorule => (HauntedGroveLedge, HauntedGroveLedge),
            ParadoxLeftLorule => (LoruleCastleArea, LoruleCastleArea),
            WaterfallLorule => (DarkRuinsRiver, DarkRuinsShallowWater),
            DarkRuinsPillar => (DarkRuins, DarkRuins),
            DarkRuinsSE => (DarkRuinsBlockedCrack, DarkRuinsBlockedCrack),
            SkullWoodsPillar => (SkullWoodsOverworld, SkullWoodsOverworld),
            NShapedHouse => (SkullWoodsOverworld, SkullWoodsOverworld),
            DestroyedHouse => (SkullWoodsOverworld, SkullWoodsOverworld),
            MiseryMireExit => (MiseryMire, MiseryMire),
            MirePillarRight => (MiseryMire, MiseryMire),
            MirePillarLeft => (MiseryMireLeftPillarMerged, MiseryMireLeftPillarMerged),
            MireMiddle => (MiseryMireBridge, MiseryMireBridge),
            MireSW => (MiseryMireBridge, MiseryMireBridge),
            Zaganaga => (ZaganagasArena, ZaganagasArena),
            MireNorth => (MiseryMireLedge, MiseryMireLedge),
            DeathWestLorule => (LoruleDeathWest, LoruleDeathWest),
            Crack::FloatingIslandLorule => (Location::FloatingIslandLorule, Location::FloatingIslandLorule),
            RiverLorule => (LoruleRiverCrackShallows, LoruleRiverCrackShallows),
            LoruleLake => (LoruleLakeNorthWest, LoruleLakeNorthWest),
            LoruleHotfoot => (LoruleLakeEast, LoruleLakeEast),
            Philosopher => (LoruleSanctuaryCaveLower, LoruleSanctuaryCaveLower),
            Crack::GraveyardLedgeLorule => (LoruleGraveyard, LoruleGraveyard),
            Crack::RossosOreMineLorule => (Location::RossosOreMineLorule, Location::RossosOreMineLorule),
            SwampPillarLorule => (LoruleCastleArea, LoruleCastleArea),
            Crack::KusDomain => (KusDomainSouth, KusDomainSouth),
            LoruleCastle => (HildasStudy, HildasStudy),
        }
    }

    /// Get destination spawn point coordinates for this crack
    pub fn get_spawn_point(self) -> SpawnPoint {
        match self {
            Self::HyruleCastle => SpawnPoint::new(IndoorLight, 7, 3),
            Self::StylishWoman => SpawnPoint::new(IndoorLight, 14, 1),
            Self::YourHouse => SpawnPoint::new(FieldLight, 27, 20),
            Self::ParadoxRightHyrule => SpawnPoint::new(FieldLight, 32, 21),
            Self::ParadoxLeftHyrule => SpawnPoint::new(FieldLight, 32, 20),
            Self::WaterfallHyrule => SpawnPoint::new(FieldLight, 13, 4),
            Self::EasternRuinsPillar => SpawnPoint::new(FieldLight, 22, 4),
            Self::EasternRuinsSE => SpawnPoint::new(FieldLight, 30, 20),
            Self::LostWoodsPillar => SpawnPoint::new(FieldLight, 1, 25),
            Self::SahasrahlasHouse => SpawnPoint::new(FieldLight, 16, 13),
            Self::RossosHouse => SpawnPoint::new(IndoorLight, 10, 1),
            Self::MiseryMireEntrance => SpawnPoint::new(FieldLight, 37, 3),
            Self::DesertPillarRight => SpawnPoint::new(FieldLight, 31, 12),
            Self::DesertPillarLeft => SpawnPoint::new(FieldLight, 31, 11),
            Self::DesertMiddle => SpawnPoint::new(FieldLight, 31, 13),
            Self::DesertSW => SpawnPoint::new(FieldLight, 31, 15),
            Self::DesertPalace => SpawnPoint::new(FieldLight, 31, 7),
            Self::DesertNorth => SpawnPoint::new(FieldLight, 31, 10),
            Self::DeathWestHyrule => SpawnPoint::new(FieldLight, 3, 20),
            Self::FloatingIslandHyrule => SpawnPoint::new(FieldLight, 4, 8),
            Self::RiverHyrule => SpawnPoint::new(FieldLight, 29, 5),
            Self::LakeHylia => SpawnPoint::new(FieldLight, 35, 20),
            Self::HyruleHotfoot => SpawnPoint::new(FieldLight, 36, 2),
            Self::Sanctuary => SpawnPoint::new(IndoorLight, 11, 3),
            Self::GraveyardLedgeHyrule => SpawnPoint::new(FieldLight, 12, 20),
            Self::RossosOreMineHyrule => SpawnPoint::new(FieldLight, 4, 7),
            Self::SwampPillarHyrule => SpawnPoint::new(FieldLight, 33, 20),
            Self::ZorasDomain => SpawnPoint::new(FieldLight, 15, 20),
            Self::LoruleCastle => SpawnPoint::new(IndoorDark, 5, 1),
            Self::ThievesTown => SpawnPoint::new(FieldDark, 16, 10),
            Self::VacantHouse => SpawnPoint::new(FieldDark, 27, 20),
            Self::ParadoxRightLorule => SpawnPoint::new(FieldDark, 32, 21),
            Self::ParadoxLeftLorule => SpawnPoint::new(FieldDark, 32, 20),
            Self::WaterfallLorule => SpawnPoint::new(FieldDark, 13, 4),
            Self::DarkRuinsPillar => SpawnPoint::new(FieldDark, 22, 4),
            Self::DarkRuinsSE => SpawnPoint::new(FieldDark, 30, 20),
            Self::SkullWoodsPillar => SpawnPoint::new(FieldDark, 1, 25),
            Self::NShapedHouse => SpawnPoint::new(FieldDark, 16, 13),
            Self::DestroyedHouse => SpawnPoint::new(FieldDark, 2, 3),
            Self::MiseryMireExit => SpawnPoint::new(FieldDark, 37, 3),
            Self::MirePillarRight => SpawnPoint::new(FieldDark, 31, 12),
            Self::MirePillarLeft => SpawnPoint::new(FieldDark, 31, 11),
            Self::MireMiddle => SpawnPoint::new(FieldDark, 31, 13),
            Self::MireSW => SpawnPoint::new(FieldDark, 31, 15),
            Self::Zaganaga => SpawnPoint::new(FieldDark, 31, 2),
            Self::MireNorth => SpawnPoint::new(FieldDark, 31, 10),
            Self::DeathWestLorule => SpawnPoint::new(FieldDark, 3, 3),
            Self::FloatingIslandLorule => SpawnPoint::new(FieldDark, 4, 7),
            Self::RiverLorule => SpawnPoint::new(FieldDark, 29, 5),
            Self::LoruleLake => SpawnPoint::new(FieldDark, 35, 20),
            Self::LoruleHotfoot => SpawnPoint::new(FieldDark, 36, 2),
            Self::Philosopher => SpawnPoint::new(CaveDark, 5, 3),
            Self::GraveyardLedgeLorule => SpawnPoint::new(FieldDark, 12, 20),
            Self::RossosOreMineLorule => SpawnPoint::new(FieldDark, 4, 3),
            Self::SwampPillarLorule => SpawnPoint::new(FieldDark, 33, 20),
            Self::KusDomain => SpawnPoint::new(FieldDark, 15, 0),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::YourHouse => "Your House Crack",
            Self::StylishWoman => "Stylish Woman's House Crack",
            Self::ParadoxRightHyrule => "Hyrule Right Paradox Crack",
            Self::ParadoxLeftHyrule => "Hyrule Left Paradox Crack",
            Self::WaterfallHyrule => "Hyrule Waterfall Crack",
            Self::EasternRuinsPillar => "Eastern Ruins Pillar Crack",
            Self::EasternRuinsSE => "Eastern Ruins SE Crack",
            Self::LostWoodsPillar => "Lost Woods Pillar Crack",
            Self::SahasrahlasHouse => "Sahasrahla's House Crack",
            Self::RossosHouse => "Rosso's House Crack",
            Self::MiseryMireEntrance => "Misery Mire Entrance Crack",
            Self::DesertMiddle => "Desert Middle Crack",
            Self::DesertSW => "Desert SW Crack",
            Self::DesertNorth => "Desert North Crack",
            Self::DesertPillarLeft => "Desert Left Pillar Crack",
            Self::DesertPillarRight => "Desert Right Pillar Crack",
            Self::DesertPalace => "Desert Palace Crack",
            Self::DeathWestHyrule => "Hyrule Death West Crack",
            Self::FloatingIslandHyrule => "Hyrule Floating Island Crack",
            Self::RiverHyrule => "Hyrule River Crack",
            Self::LakeHylia => "Lake Hylia Crack",
            Self::HyruleHotfoot => "Hyrule Hotfoot Crack",
            Self::Sanctuary => "Sanctuary Crack",
            Self::GraveyardLedgeHyrule => "Hyrule Graveyard Ledge Crack",
            Self::RossosOreMineHyrule => "Hyrule Rosso's Ore Mine Crack",
            Self::SwampPillarHyrule => "Hyrule Swamp Pillar Crack",
            Self::ZorasDomain => "Zora's Domain Crack",
            Self::HyruleCastle => "[HC] Crack",
            // --- //
            Self::ThievesTown => "Thieves' Town Crack",
            Self::VacantHouse => "Vacant House Crack",
            Self::ParadoxRightLorule => "Lorule Right Paradox Crack",
            Self::ParadoxLeftLorule => "Lorule Left Paradox Crack",
            Self::WaterfallLorule => "Lorule Waterfall Crack",
            Self::DarkRuinsPillar => "Dark Ruins Pillar Crack",
            Self::DarkRuinsSE => "Dark Ruins SE Crack",
            Self::SkullWoodsPillar => "Skull Woods Pillar Crack",
            Self::NShapedHouse => "n-Shaped House Crack",
            Self::DestroyedHouse => "Destroyed House Crack",
            Self::MiseryMireExit => "Misery Mire Exit Crack",
            Self::MirePillarRight => "Mire Right Pillar Crack",
            Self::MirePillarLeft => "Mire Left Pillar Crack",
            Self::MireMiddle => "Mire Middle Crack",
            Self::MireSW => "Mire SW Crack",
            Self::Zaganaga => "Zaganaga Crack",
            Self::MireNorth => "Mire North Crack",
            Self::DeathWestLorule => "Lorule Death West Crack",
            Self::FloatingIslandLorule => "Lorule Floating Island Crack",
            Self::RiverLorule => "Lorule River Crack",
            Self::LoruleLake => "Lorule Lake Crack",
            Self::LoruleHotfoot => "Lorule Hotfoot Crack",
            Self::Philosopher => "Philosopher's Cave Crack",
            Self::GraveyardLedgeLorule => "Lorule Graveyard Ledge Crack",
            Self::RossosOreMineLorule => "Lorule Rosso's Ore Mine Crack",
            Self::SwampPillarLorule => "Lorule Swamp Pillar Crack",
            Self::KusDomain => "Ku's Domain Crack",
            Self::LoruleCastle => "[LC] Crack",
        }
    }

    pub fn get_world(self) -> game::World {
        match self {
            Self::StylishWoman
            | Self::YourHouse
            | Self::ParadoxRightHyrule
            | Self::ParadoxLeftHyrule
            | Self::WaterfallHyrule
            | Self::EasternRuinsPillar
            | Self::EasternRuinsSE
            | Self::LostWoodsPillar
            | Self::SahasrahlasHouse
            | Self::RossosHouse
            | Self::MiseryMireEntrance
            | Self::DesertMiddle
            | Self::DesertSW
            | Self::DesertNorth
            | Self::DesertPillarLeft
            | Self::DesertPillarRight
            | Self::DesertPalace
            | Self::DeathWestHyrule
            | Self::FloatingIslandHyrule
            | Self::RiverHyrule
            | Self::LakeHylia
            | Self::HyruleHotfoot
            | Self::Sanctuary
            | Self::GraveyardLedgeHyrule
            | Self::RossosOreMineHyrule
            | Self::SwampPillarHyrule
            | Self::ZorasDomain
            | Self::HyruleCastle => game::World::Hyrule,

            Self::ThievesTown
            | Self::VacantHouse
            | Self::ParadoxRightLorule
            | Self::ParadoxLeftLorule
            | Self::WaterfallLorule
            | Self::DarkRuinsPillar
            | Self::DarkRuinsSE
            | Self::SkullWoodsPillar
            | Self::NShapedHouse
            | Self::DestroyedHouse
            | Self::MiseryMireExit
            | Self::MirePillarRight
            | Self::MirePillarLeft
            | Self::MireMiddle
            | Self::MireSW
            | Self::Zaganaga
            | Self::MireNorth
            | Self::DeathWestLorule
            | Self::FloatingIslandLorule
            | Self::RiverLorule
            | Self::LoruleLake
            | Self::LoruleHotfoot
            | Self::Philosopher
            | Self::GraveyardLedgeLorule
            | Self::RossosOreMineLorule
            | Self::SwampPillarLorule
            | Self::KusDomain
            | Self::LoruleCastle => game::World::Lorule,
        }
    }

    pub fn get_flag(self) -> Flag {
        match self {
            Self::StylishWoman => Flag::CRACK_STYLISH_WOMAN,
            Self::YourHouse => Flag::CRACK_YOUR_HOUSE,
            Self::ParadoxRightHyrule => Flag::CRACK_PARADOX_LOWER_HYRULE,
            Self::ParadoxLeftHyrule => Flag::CRACK_PARADOX_UPPER_HYRULE,
            Self::WaterfallHyrule => Flag::CRACK_WATERFALL_HYRULE,
            Self::EasternRuinsPillar => Flag::CRACK_EASTERN_RUINS_PILLAR,
            Self::EasternRuinsSE => Flag::CRACK_EASTERN_RUINS_SE,
            Self::LostWoodsPillar => Flag::CRACK_LOST_WOODS,
            Self::SahasrahlasHouse => Flag::CRACK_SAHASRAHLAS_HOUSE,
            Self::RossosHouse => Flag::CRACK_ROSSOS_HOUSE,
            Self::MiseryMireEntrance => Flag::CRACK_MISERY_MIRE_EXIT,
            Self::DesertPillarRight => Flag::CRACK_DESERT_RILLAR_RIGHT,
            Self::DesertPillarLeft => Flag::CRACK_DESERT_PILLAR_LEFT,
            Self::DesertMiddle => Flag::CRACK_DESERT_MIDDLE,
            Self::DesertSW => Flag::CRACK_DESERT_SW,
            Self::DesertPalace => Flag::CRACK_TO_ZAGANAGA,
            Self::DesertNorth => Flag::CRACK_DESERT_NORTH,
            Self::DeathWestHyrule => Flag::CRACK_DM_WEST_HYRULE,
            Self::FloatingIslandHyrule => Flag::CRACK_FLOATING_ISLAND_HYRULE,
            Self::RiverHyrule => Flag::CRACK_RIVER_HYRULE,
            Self::LakeHylia => Flag::CRACK_LAKE_HYLIA,
            Self::HyruleHotfoot => Flag::CRACK_HYRULE_HOTFOOT,
            Self::Sanctuary => Flag::CRACK_SANCTUARY,
            Self::GraveyardLedgeHyrule => Flag::CRACK_GRAVEYARD_LEDGE_HYRULE,
            Self::RossosOreMineHyrule => Flag::CRACK_ROSSOS_ORE_MINE_HYRULE,
            Self::SwampPillarHyrule => Flag::CRACK_SWAMP_PILLAR_HYRULE,
            Self::ZorasDomain => Flag::CRACK_ZORAS_DOMAIN,
            Self::HyruleCastle => Flag::ZERO_ZERO,
            Self::ThievesTown => Flag::CRACK_THIEVES_TOWN,
            Self::VacantHouse => Flag::CRACK_VACANT_HOUSE,
            Self::ParadoxRightLorule => Flag::CRACK_PARADOX_UPPER_LORULE,
            Self::ParadoxLeftLorule => Flag::CRACK_PARADOX_LOWER_LORULE,
            Self::WaterfallLorule => Flag::CRACK_WATERFALL_LORULE,
            Self::DarkRuinsPillar => Flag::CRACK_DARK_RUINS_PILLAR,
            Self::DarkRuinsSE => Flag::CRACK_DARK_MAZE_SE,
            Self::SkullWoodsPillar => Flag::CRACK_SKULL_WOODS_PILLAR,
            Self::NShapedHouse => Flag::CRACK_N_SHAPED_HOUSE,
            Self::DestroyedHouse => Flag::CRACK_DESTROYED_HOUSE,
            Self::MiseryMireExit => Flag::CRACK_MISERY_MIRE_EXIT,
            Self::MirePillarRight => Flag::CRACK_MIRE_PILLAR_RIGHT,
            Self::MirePillarLeft => Flag::CRACK_MIRE_PILLAR_LEFT,
            Self::MireMiddle => Flag::CRACK_MIRE_MIDDLE,
            Self::MireSW => Flag::CRACK_MIRE_SW,
            Self::Zaganaga => Flag::CRACK_ZAGANAGA_EXIT,
            Self::MireNorth => Flag::CRACK_MIRE_NORTH,
            Self::DeathWestLorule => Flag::CRACK_DM_WEST_LORULE,
            Self::FloatingIslandLorule => Flag::CRACK_FLOATING_ISLAND_LORULE,
            Self::RiverLorule => Flag::CRACK_RIVER_LORULE,
            Self::LoruleLake => Flag::CRACK_LORULE_LAKE_WEST,
            Self::LoruleHotfoot => Flag::CRACK_LORULE_COLDFOOT,
            Self::Philosopher => Flag::CRACK_PHILOSOPHERS_CAVE,
            Self::GraveyardLedgeLorule => Flag::CRACK_GRAVEYARD_LEDGE_LORULE,
            Self::RossosOreMineLorule => Flag::CRACK_ROSSOS_ORE_MINE_LORULE,
            Self::SwampPillarLorule => Flag::CRACK_SWAMP_PILLAR_LORULE,
            Self::KusDomain => Flag::CRACK_KUS_DOMAIN,
            Self::LoruleCastle => Flag::ZERO_ZERO,
        }
    }
}

impl Display for Crack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Ord for Crack {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd<Crack> for Crack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Randomizable> for Crack {
    fn from(filler_item: Randomizable) -> Self {
        match filler_item {
            Randomizable::Crack(crack) => crack,
            _ => unreachable!("Not a Crack: {:?}", filler_item),
        }
    }
}

impl Serialize for Crack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Builds out the CrackMap for use traversing the WorldGraph.
/// Shuffles the crack destinations if CrackShuffle is enabled.
///
/// The 6 pairs of "down-facing" Cracks are only shuffled between themselves, for technical reasons
pub fn build_crack_map(settings: &Settings, rng: &mut StdRng) -> Result<CrackMap> {
    info!("Building Crack Map...");
    let mut crack_map: DashMap<_, _> = Default::default();

    let mut hyrule_up_cracks = item_pools::get_hyrule_up_cracks();
    let mut hyrule_down_cracks = item_pools::get_hyrule_down_cracks();
    let mut lorule_up_cracks = item_pools::get_lorule_up_cracks();
    let lorule_down_cracks = item_pools::get_lorule_down_cracks();

    // Keep Desert Palace + Zaganaga Cracks vanilla (TODO new feature)
    crack_map.insert(Crack::DesertPalace, Crack::Zaganaga);
    crack_map.insert(Crack::Zaganaga, Crack::DesertPalace);
    hyrule_up_cracks.retain(|&p| p != Crack::DesertPalace);
    lorule_up_cracks.retain(|&p| p != Crack::Zaganaga);

    match settings.cracksanity {
        Cracksanity::Off => {
            let mut hyrule_cracks = hyrule_up_cracks;
            hyrule_cracks.extend(hyrule_down_cracks);

            for hyrule_crack in hyrule_cracks {
                let lorule_crack = hyrule_crack.get_mirror_crack();

                crack_map.insert(hyrule_crack, lorule_crack);
                crack_map.insert(lorule_crack, hyrule_crack);
            }
        },
        Cracksanity::CrossWorldPairs => {
            let mut hyrule_cracks = hyrule_up_cracks;
            hyrule_cracks.extend(hyrule_down_cracks);

            let mut lorule_cracks = filler::util::shuffle(rng, lorule_up_cracks);
            lorule_cracks.extend(filler::util::shuffle(rng, lorule_down_cracks));

            create_map(&mut crack_map, &hyrule_cracks, &lorule_cracks);
        },
        Cracksanity::AnyWorldPairs => {
            // Force Hyrule Castle crack to always be paired with a Lorule (Up) crack
            let hc_match = lorule_up_cracks.remove(rng.gen_range(0..lorule_up_cracks.len()));
            hyrule_up_cracks.retain(|&p| p != Crack::HyruleCastle);
            lorule_up_cracks.retain(|&p| p != hc_match);
            crack_map.insert(Crack::HyruleCastle, hc_match);
            crack_map.insert(hc_match, Crack::HyruleCastle);

            //
            let mut up_cracks = Vec::new();
            up_cracks.extend(hyrule_up_cracks);
            up_cracks.extend(lorule_up_cracks);

            let mut down_cracks = Vec::new();
            down_cracks.extend(hyrule_down_cracks);
            down_cracks.extend(lorule_down_cracks);

            crack_map.extend(filler::util::pair_randomly(rng, up_cracks)?);
            crack_map.extend(filler::util::pair_randomly(rng, down_cracks)?);
        },
        Cracksanity::MirroredCrossWorldPairs => {
            // UP
            let mut lorule_up_cracks = filler::util::shuffle(rng, lorule_up_cracks);

            while !hyrule_up_cracks.is_empty() {
                let hyrule_up_crack = hyrule_up_cracks.pop().unwrap();
                let lorule_up_crack = lorule_up_cracks.pop().unwrap();

                crack_map.insert(hyrule_up_crack, lorule_up_crack);
                crack_map.insert(lorule_up_crack, hyrule_up_crack);

                // Add mirror crack pairing, if crack pair isn't vanilla
                if hyrule_up_crack.get_mirror_crack() != lorule_up_crack {
                    let hyrule_mirror = hyrule_up_crack.get_mirror_crack();
                    let lorule_mirror = lorule_up_crack.get_mirror_crack();

                    hyrule_up_cracks.retain(|&p| p != lorule_mirror);
                    lorule_up_cracks.retain(|&p| p != hyrule_mirror);

                    crack_map.insert(hyrule_mirror, lorule_mirror);
                    crack_map.insert(lorule_mirror, hyrule_mirror);
                }
            }

            // DOWN
            let mut lorule_down_cracks = filler::util::shuffle(rng, lorule_down_cracks);

            while !hyrule_down_cracks.is_empty() {
                let hyrule_down_crack = hyrule_down_cracks.pop().unwrap();
                let lorule_down_crack = lorule_down_cracks.pop().unwrap();

                crack_map.insert(hyrule_down_crack, lorule_down_crack);
                crack_map.insert(lorule_down_crack, hyrule_down_crack);

                // Add mirror crack pairing, if crack pair isn't vanilla
                if hyrule_down_crack.get_mirror_crack() != lorule_down_crack {
                    let hyrule_mirror = hyrule_down_crack.get_mirror_crack();
                    let lorule_mirror = lorule_down_crack.get_mirror_crack();

                    hyrule_down_cracks.retain(|&p| p != lorule_mirror);
                    lorule_down_cracks.retain(|&p| p != hyrule_mirror);

                    crack_map.insert(hyrule_mirror, lorule_mirror);
                    crack_map.insert(lorule_mirror, hyrule_mirror);
                }
            }
        },
        Cracksanity::MirroredAnyWorldPairs => {
            // Force Hyrule Castle crack to always be paired with a Lorule (Up) crack
            let hc_match = lorule_up_cracks.remove(rng.gen_range(0..lorule_up_cracks.len()));
            hyrule_up_cracks.retain(|&p| p != Crack::HyruleCastle);
            lorule_up_cracks.retain(|&p| p != hc_match);
            crack_map.insert(Crack::HyruleCastle, hc_match);
            crack_map.insert(hc_match, Crack::HyruleCastle);

            // Mirror HC match
            let hc_match_mirror = hc_match.get_mirror_crack();
            lorule_up_cracks.retain(|&p| p != Crack::LoruleCastle);
            hyrule_up_cracks.retain(|&p| p != hc_match_mirror);
            crack_map.insert(Crack::LoruleCastle, hc_match_mirror);
            crack_map.insert(hc_match_mirror, Crack::LoruleCastle);

            // UP
            let mut up_cracks = Vec::new();
            up_cracks.extend(hyrule_up_cracks);
            up_cracks.extend(lorule_up_cracks);

            let mut up_cracks = filler::util::shuffle(rng, up_cracks);

            while !up_cracks.is_empty() {
                let first = up_cracks.pop().unwrap();
                let second = up_cracks.pop().unwrap();

                crack_map.insert(first, second);
                crack_map.insert(second, first);

                // Add mirror crack pairing, if crack pair isn't vanilla
                if first.get_mirror_crack() != second {
                    let first_mirror = first.get_mirror_crack();
                    let second_mirror = second.get_mirror_crack();

                    up_cracks.retain(|&p| p != first_mirror);
                    up_cracks.retain(|&p| p != second_mirror);

                    crack_map.insert(first_mirror, second_mirror);
                    crack_map.insert(second_mirror, first_mirror);
                }
            }

            // DOWN
            let mut down_cracks = Vec::new();
            down_cracks.extend(hyrule_down_cracks);
            down_cracks.extend(lorule_down_cracks);

            let mut down_cracks = filler::util::shuffle(rng, down_cracks);

            while !down_cracks.is_empty() {
                let first = down_cracks.pop().unwrap();
                let second = down_cracks.pop().unwrap();

                crack_map.insert(first, second);
                crack_map.insert(second, first);

                // Add mirror crack pairing, if crack pair isn't vanilla
                if first.get_mirror_crack() != second {
                    let first_mirror = first.get_mirror_crack();
                    let second_mirror = second.get_mirror_crack();

                    down_cracks.retain(|&p| p != first_mirror);
                    down_cracks.retain(|&p| p != second_mirror);

                    crack_map.insert(first_mirror, second_mirror);
                    crack_map.insert(second_mirror, first_mirror);
                }
            }
        },
    }

    // info!("{:?}", crack_map);
    //
    // let mut keys = crate::DashSet::default();
    // let mut values = crate::DashSet::default();
    // for (k, v) in &crack_map {
    //     keys.insert(k);
    //     values.insert(v);
    // }
    //
    // assert_eq!(56, keys.len());
    // assert_eq!(56, values.len());
    //
    // info!("PASS");

    Ok(crack_map.iter().map(|(&a, &b)| (a, b)).collect())
}

fn create_map<T>(crack_map: &mut DashMap<T, T>, vec1: &Vec<T>, vec2: &Vec<T>)
where
    T: Copy + Eq + Hash + PartialEq,
{
    for i in 0..vec1.len() {
        crack_map.insert(vec1[i], vec2[i]);
        crack_map.insert(vec2[i], vec1[i]);
    }
}

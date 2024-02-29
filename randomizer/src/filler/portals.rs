use crate::filler::filler_item::Randomizable;
use crate::filler::item_pools;
use crate::filler::location::Location;
use crate::Result;
use crate::{filler, DashMap, PortalMap};
use game::Course::{CaveDark, FieldDark, FieldLight, IndoorDark, IndoorLight};
use log::info;
use modinfo::settings::portal_shuffle::PortalShuffle;
use modinfo::Settings;
use rand::rngs::StdRng;
use rand::Rng;
use rom::flag::Flag;
use rom::scene::SpawnPoint;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

/// Weather Vane Item
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Portal {
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

impl Portal {
    pub(crate) fn get_type(&self) -> i32 {
        use Portal::*;
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
        use Portal::*;
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

    pub(crate) fn get_mirror_portal(&self) -> Portal {
        use Portal::*;
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
        use Portal::*;
        match self {
            StylishWoman => (StylishWomanHouse, StylishWomanHouse),
            YourHouse => (HyruleField, HyruleField),
            ParadoxRightHyrule => (HyruleField, HyruleField),
            ParadoxLeftHyrule => (CuccoDungeonLedge, CuccoDungeonLedge),
            WaterfallHyrule => (ZoraRiver, WaterfallLedge),
            EasternRuinsPillar => (HyruleField, HyruleField),
            EasternRuinsSE => (EasternRuinsBlockedPortal, EasternRuinsBlockedPortal),
            LostWoodsPillar => (HyruleField, HyruleField),
            SahasrahlasHouse => (HyruleField, HyruleField),
            Portal::RossosHouse => (Location::RossosHouse, Location::RossosHouse),
            MiseryMireEntrance => (HyruleField, HyruleField),
            DesertPillarRight => (Desert, Desert),
            DesertPillarLeft => (Desert, Desert),
            DesertMiddle => (Desert, DesertCenterLedge),
            DesertSW => (DesertSouthWestLedge, DesertSouthWestLedge),
            DesertPalace => (DesertZaganagaLedge, DesertZaganagaLedge),
            DesertNorth => (Desert, Desert),
            DeathWestHyrule => (DeathMountainBase, DeathMountainBase),
            Portal::FloatingIslandHyrule => (Location::FloatingIslandHyrule, Location::FloatingIslandHyrule),
            RiverHyrule => (BridgeShallowWater, Location::LakeHylia),
            Portal::LakeHylia => (HyruleField, HyruleField),
            HyruleHotfoot => (HyruleField, HyruleField),
            Portal::Sanctuary => (SanctuaryChurch, SanctuaryChurch),
            Portal::GraveyardLedgeHyrule => (Location::GraveyardLedgeHyrule, Location::GraveyardLedgeHyrule),
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
            DarkRuinsSE => (DarkRuinsBlockedPortal, DarkRuinsBlockedPortal),
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
            Portal::FloatingIslandLorule => (Location::FloatingIslandLorule, Location::FloatingIslandLorule),
            RiverLorule => (LoruleRiverPortalShallows, LoruleRiverPortalShallows),
            LoruleLake => (LoruleLakeNorthWest, LoruleLakeNorthWest),
            LoruleHotfoot => (LoruleLakeEast, LoruleLakeEast),
            Philosopher => (LoruleSanctuaryCaveLower, LoruleSanctuaryCaveLower),
            Portal::GraveyardLedgeLorule => (LoruleGraveyard, LoruleGraveyard),
            Portal::RossosOreMineLorule => (Location::RossosOreMineLorule, Location::RossosOreMineLorule),
            SwampPillarLorule => (LoruleCastleArea, LoruleCastleArea),
            Portal::KusDomain => (KusDomainSouth, KusDomainSouth),
            LoruleCastle => (HildasStudy, HildasStudy),
        }
    }

    /// Get destination spawn point coordinates for this Portal
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
            Self::YourHouse => "Your House Portal",
            Self::StylishWoman => "Stylish Woman Portal",
            Self::ParadoxRightHyrule => "Hyrule Right Paradox Portal",
            Self::ParadoxLeftHyrule => "Hyrule Left Paradox Portal",
            Self::WaterfallHyrule => "Hyrule Waterfall Portal",
            Self::EasternRuinsPillar => "Eastern Ruins Pillar Portal",
            Self::EasternRuinsSE => "Eastern Ruins SE Portal",
            Self::LostWoodsPillar => "Lost Woods Pillar Portal",
            Self::SahasrahlasHouse => "Sahasrahla's House Portal",
            Self::RossosHouse => "Rosso's House Portal",
            Self::MiseryMireEntrance => "Misery Mire Entrance Portal",
            Self::DesertMiddle => "Desert Middle Portal",
            Self::DesertSW => "Desert SW Portal",
            Self::DesertNorth => "Desert North Portal",
            Self::DesertPillarLeft => "Desert Left Pillar Portal",
            Self::DesertPillarRight => "Desert Right Pillar Portal",
            Self::DesertPalace => "Desert Palace Portal",
            Self::DeathWestHyrule => "Hyrule Death West Portal",
            Self::FloatingIslandHyrule => "Hyrule Floating Island Portal",
            Self::RiverHyrule => "Hyrule River Portal",
            Self::LakeHylia => "Lake Hylia Portal",
            Self::HyruleHotfoot => "Hyrule Hotfoot Portal",
            Self::Sanctuary => "Sanctuary Portal",
            Self::GraveyardLedgeHyrule => "Hyrule Graveyard Ledge Portal",
            Self::RossosOreMineHyrule => "Hyrule Rosso's Ore Mine Portal",
            Self::SwampPillarHyrule => "Hyrule Swamp Pillar Portal",
            Self::ZorasDomain => "Zora's Domain Portal",
            Self::HyruleCastle => "[HC] Portal",
            Self::ThievesTown => "Thieves' Town Portal",
            Self::VacantHouse => "Vacant House Portal",
            Self::ParadoxRightLorule => "Lorule Right Paradox Portal",
            Self::ParadoxLeftLorule => "Lorule Left Paradox Portal",
            Self::WaterfallLorule => "Lorule Waterfall Portal",
            Self::DarkRuinsPillar => "Dark Ruins Pillar Portal",
            Self::DarkRuinsSE => "Dark Ruins SE Portal",
            Self::SkullWoodsPillar => "Skull Woods Pillar Portal",
            Self::NShapedHouse => "n-Shaped House Portal",
            Self::DestroyedHouse => "Destroyed House Portal",
            Self::MiseryMireExit => "Misery Mire Exit Portal",
            Self::MirePillarRight => "Mire Right Pillar Portal",
            Self::MirePillarLeft => "Mire Left Pillar Portal",
            Self::MireMiddle => "Mire Middle Portal",
            Self::MireSW => "Mire SW Portal",
            Self::Zaganaga => "Zaganaga Portal",
            Self::MireNorth => "Mire North Portal",
            Self::DeathWestLorule => "Lorule Death West Portal",
            Self::FloatingIslandLorule => "Lorule Floating Island Portal",
            Self::RiverLorule => "Lorule River Portal",
            Self::LoruleLake => "Lorule Lake Portal",
            Self::LoruleHotfoot => "Lorule Hotfoot Portal",
            Self::Philosopher => "Philosopher's Cave Portal",
            Self::GraveyardLedgeLorule => "Lorule Graveyard Ledge Portal",
            Self::RossosOreMineLorule => "Lorule Rosso's Ore Mine Portal",
            Self::SwampPillarLorule => "Lorule Swamp Pillar Portal",
            Self::KusDomain => "Ku's Domain Portal",
            Self::LoruleCastle => "[LC] Portal",
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
            Self::StylishWoman => Flag::PORTAL_STYLISH_WOMAN,
            Self::YourHouse => Flag::PORTAL_YOUR_HOUSE,
            Self::ParadoxRightHyrule => Flag::PORTAL_PARADOX_LOWER_HYRULE,
            Self::ParadoxLeftHyrule => Flag::PORTAL_PARADOX_UPPER_HYRULE,
            Self::WaterfallHyrule => Flag::PORTAL_WATERFALL_HYRULE,
            Self::EasternRuinsPillar => Flag::PORTAL_EASTERN_RUINS_PILLAR,
            Self::EasternRuinsSE => Flag::PORTAL_EASTERN_RUINS_SE,
            Self::LostWoodsPillar => Flag::PORTAL_LOST_WOODS,
            Self::SahasrahlasHouse => Flag::PORTAL_SAHASRAHLAS_HOUSE,
            Self::RossosHouse => Flag::PORTAL_ROSSOS_HOUSE,
            Self::MiseryMireEntrance => Flag::PORTAL_MISERY_MIRE_EXIT,
            Self::DesertPillarRight => Flag::PORTAL_DESERT_RILLAR_RIGHT,
            Self::DesertPillarLeft => Flag::PORTAL_DESERT_PILLAR_LEFT,
            Self::DesertMiddle => Flag::PORTAL_DESERT_MIDDLE,
            Self::DesertSW => Flag::PORTAL_DESERT_SW,
            Self::DesertPalace => Flag::PORTAL_TO_ZAGANAGA,
            Self::DesertNorth => Flag::PORTAL_DESERT_NORTH,
            Self::DeathWestHyrule => Flag::PORTAL_DM_WEST_HYRULE,
            Self::FloatingIslandHyrule => Flag::PORTAL_FLOATING_ISLAND_HYRULE,
            Self::RiverHyrule => Flag::PORTAL_RIVER_HYRULE,
            Self::LakeHylia => Flag::PORTAL_LAKE_HYLIA,
            Self::HyruleHotfoot => Flag::PORTAL_HYRULE_HOTFOOT,
            Self::Sanctuary => Flag::PORTAL_SANCTUARY,
            Self::GraveyardLedgeHyrule => Flag::PORTAL_GRAVEYARD_LEDGE_HYRULE,
            Self::RossosOreMineHyrule => Flag::PORTAL_ROSSOS_ORE_MINE_HYRULE,
            Self::SwampPillarHyrule => Flag::PORTAL_SWAMP_PILLAR_HYRULE,
            Self::ZorasDomain => Flag::PORTAL_ZORAS_DOMAIN,
            Self::HyruleCastle => Flag::ZERO_ZERO,
            Self::ThievesTown => Flag::PORTAL_THIEVES_TOWN,
            Self::VacantHouse => Flag::PORTAL_VACANT_HOUSE,
            Self::ParadoxRightLorule => Flag::PORTAL_PARADOX_UPPER_LORULE,
            Self::ParadoxLeftLorule => Flag::PORTAL_PARADOX_LOWER_LORULE,
            Self::WaterfallLorule => Flag::PORTAL_WATERFALL_LORULE,
            Self::DarkRuinsPillar => Flag::PORTAL_DARK_RUINS_PILLAR,
            Self::DarkRuinsSE => Flag::PORTAL_DARK_MAZE_SE,
            Self::SkullWoodsPillar => Flag::PORTAL_SKULL_WOODS_PILLAR,
            Self::NShapedHouse => Flag::PORTAL_N_SHAPED_HOUSE,
            Self::DestroyedHouse => Flag::PORTAL_DESTROYED_HOUSE,
            Self::MiseryMireExit => Flag::PORTAL_MISERY_MIRE_EXIT,
            Self::MirePillarRight => Flag::PORTAL_MIRE_PILLAR_RIGHT,
            Self::MirePillarLeft => Flag::PORTAL_MIRE_PILLAR_LEFT,
            Self::MireMiddle => Flag::PORTAL_MIRE_MIDDLE,
            Self::MireSW => Flag::PORTAL_MIRE_SW,
            Self::Zaganaga => Flag::PORTAL_ZAGANAGA_EXIT,
            Self::MireNorth => Flag::PORTAL_MIRE_NORTH,
            Self::DeathWestLorule => Flag::PORTAL_DM_WEST_LORULE,
            Self::FloatingIslandLorule => Flag::PORTAL_FLOATING_ISLAND_LORULE,
            Self::RiverLorule => Flag::PORTAL_RIVER_LORULE,
            Self::LoruleLake => Flag::PORTAL_LORULE_LAKE_WEST,
            Self::LoruleHotfoot => Flag::PORTAL_LORULE_COLDFOOT,
            Self::Philosopher => Flag::PORTAL_PHILOSOPHERS_CAVE,
            Self::GraveyardLedgeLorule => Flag::PORTAL_GRAVEYARD_LEDGE_LORULE,
            Self::RossosOreMineLorule => Flag::PORTAL_ROSSOS_ORE_MINE_LORULE,
            Self::SwampPillarLorule => Flag::PORTAL_SWAMP_PILLAR_LORULE,
            Self::KusDomain => Flag::PORTAL_KUS_DOMAIN,
            Self::LoruleCastle => Flag::ZERO_ZERO,
        }
    }
}

impl Display for Portal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Ord for Portal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd<Portal> for Portal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Randomizable> for Portal {
    fn from(filler_item: Randomizable) -> Self {
        match filler_item {
            Randomizable::Portal(portal) => portal,
            _ => unreachable!("Not a Portal: {:?}", filler_item),
        }
    }
}

impl Serialize for Portal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Builds out the Portal map for use traversing the WorldGraph.
/// Shuffles the Portal destinations if PortalShuffle is enabled.
///
/// The 6 pairs of "down-facing" Portals are only shuffled between themselves, for technical reasons
pub fn build_portal_map(settings: &Settings, rng: &mut StdRng) -> Result<PortalMap> {
    info!("Building Portal Map...");
    let mut portal_map: DashMap<_, _> = Default::default();

    let mut hyrule_up_portals = item_pools::get_hyrule_up_portals();
    let mut hyrule_down_portals = item_pools::get_hyrule_down_portals();
    let mut lorule_up_portals = item_pools::get_lorule_up_portals();
    let lorule_down_portals = item_pools::get_lorule_down_portals();

    // Keep Desert Palace + Zaganaga Portals vanilla (TODO new feature)
    portal_map.insert(Portal::DesertPalace, Portal::Zaganaga);
    portal_map.insert(Portal::Zaganaga, Portal::DesertPalace);
    hyrule_up_portals.retain(|&p| p != Portal::DesertPalace);
    lorule_up_portals.retain(|&p| p != Portal::Zaganaga);

    match settings.portal_shuffle {
        PortalShuffle::Off => {
            let mut hyrule_portals = hyrule_up_portals;
            hyrule_portals.extend(hyrule_down_portals);

            for hyrule_portal in hyrule_portals {
                let lorule_portal = hyrule_portal.get_mirror_portal();

                portal_map.insert(hyrule_portal, lorule_portal);
                portal_map.insert(lorule_portal, hyrule_portal);
            }
        },
        PortalShuffle::CrossWorldPairs => {
            let mut hyrule_portals = hyrule_up_portals;
            hyrule_portals.extend(hyrule_down_portals);

            let mut lorule_portals = filler::util::shuffle(rng, lorule_up_portals);
            lorule_portals.extend(filler::util::shuffle(rng, lorule_down_portals));

            create_map(&mut portal_map, &hyrule_portals, &lorule_portals);
        },
        PortalShuffle::AnyWorldPairs => {
            // Force Hyrule Castle portal to always be paired with a Lorule (Up) portal
            let hc_match = lorule_up_portals.remove(rng.gen_range(0..lorule_up_portals.len()));
            hyrule_up_portals.retain(|&p| p != Portal::HyruleCastle);
            lorule_up_portals.retain(|&p| p != hc_match);
            portal_map.insert(Portal::HyruleCastle, hc_match);
            portal_map.insert(hc_match, Portal::HyruleCastle);

            //
            let mut up_portals = Vec::new();
            up_portals.extend(hyrule_up_portals);
            up_portals.extend(lorule_up_portals);

            let mut down_portals = Vec::new();
            down_portals.extend(hyrule_down_portals);
            down_portals.extend(lorule_down_portals);

            portal_map.extend(filler::util::pair_randomly(rng, up_portals)?);
            portal_map.extend(filler::util::pair_randomly(rng, down_portals)?);
        },
        PortalShuffle::MirroredCrossWorldPairs => {
            // UP
            let mut lorule_up_portals = filler::util::shuffle(rng, lorule_up_portals);

            while !hyrule_up_portals.is_empty() {
                let hyrule_up_portal = hyrule_up_portals.pop().unwrap();
                let lorule_up_portal = lorule_up_portals.pop().unwrap();

                portal_map.insert(hyrule_up_portal, lorule_up_portal);
                portal_map.insert(lorule_up_portal, hyrule_up_portal);

                // Add mirror portal pairing, if portal pair isn't vanilla
                if hyrule_up_portal.get_mirror_portal() != lorule_up_portal {
                    let hyrule_mirror = hyrule_up_portal.get_mirror_portal();
                    let lorule_mirror = lorule_up_portal.get_mirror_portal();

                    hyrule_up_portals.retain(|&p| p != lorule_mirror);
                    lorule_up_portals.retain(|&p| p != hyrule_mirror);

                    portal_map.insert(hyrule_mirror, lorule_mirror);
                    portal_map.insert(lorule_mirror, hyrule_mirror);
                }
            }

            // DOWN
            let mut lorule_down_portals = filler::util::shuffle(rng, lorule_down_portals);

            while !hyrule_down_portals.is_empty() {
                let hyrule_down_portal = hyrule_down_portals.pop().unwrap();
                let lorule_down_portal = lorule_down_portals.pop().unwrap();

                portal_map.insert(hyrule_down_portal, lorule_down_portal);
                portal_map.insert(lorule_down_portal, hyrule_down_portal);

                // Add mirror portal pairing, if portal pair isn't vanilla
                if hyrule_down_portal.get_mirror_portal() != lorule_down_portal {
                    let hyrule_mirror = hyrule_down_portal.get_mirror_portal();
                    let lorule_mirror = lorule_down_portal.get_mirror_portal();

                    hyrule_down_portals.retain(|&p| p != lorule_mirror);
                    lorule_down_portals.retain(|&p| p != hyrule_mirror);

                    portal_map.insert(hyrule_mirror, lorule_mirror);
                    portal_map.insert(lorule_mirror, hyrule_mirror);
                }
            }
        },
        PortalShuffle::MirroredAnyWorldPairs => {
            // Force Hyrule Castle portal to always be paired with a Lorule (Up) portal
            let hc_match = lorule_up_portals.remove(rng.gen_range(0..lorule_up_portals.len()));
            hyrule_up_portals.retain(|&p| p != Portal::HyruleCastle);
            lorule_up_portals.retain(|&p| p != hc_match);
            portal_map.insert(Portal::HyruleCastle, hc_match);
            portal_map.insert(hc_match, Portal::HyruleCastle);

            // Mirror HC match
            let hc_match_mirror = hc_match.get_mirror_portal();
            lorule_up_portals.retain(|&p| p != Portal::LoruleCastle);
            hyrule_up_portals.retain(|&p| p != hc_match_mirror);
            portal_map.insert(Portal::LoruleCastle, hc_match_mirror);
            portal_map.insert(hc_match_mirror, Portal::LoruleCastle);

            // UP
            let mut up_portals = Vec::new();
            up_portals.extend(hyrule_up_portals);
            up_portals.extend(lorule_up_portals);

            let mut up_portals = filler::util::shuffle(rng, up_portals);

            while !up_portals.is_empty() {
                let first = up_portals.pop().unwrap();
                let second = up_portals.pop().unwrap();

                portal_map.insert(first, second);
                portal_map.insert(second, first);

                // Add mirror portal pairing, if portal pair isn't vanilla
                if first.get_mirror_portal() != second {
                    let first_mirror = first.get_mirror_portal();
                    let second_mirror = second.get_mirror_portal();

                    up_portals.retain(|&p| p != first_mirror);
                    up_portals.retain(|&p| p != second_mirror);

                    portal_map.insert(first_mirror, second_mirror);
                    portal_map.insert(second_mirror, first_mirror);
                }
            }

            // DOWN
            let mut down_portals = Vec::new();
            down_portals.extend(hyrule_down_portals);
            down_portals.extend(lorule_down_portals);

            let mut down_portals = filler::util::shuffle(rng, down_portals);

            while !down_portals.is_empty() {
                let first = down_portals.pop().unwrap();
                let second = down_portals.pop().unwrap();

                portal_map.insert(first, second);
                portal_map.insert(second, first);

                // Add mirror portal pairing, if portal pair isn't vanilla
                if first.get_mirror_portal() != second {
                    let first_mirror = first.get_mirror_portal();
                    let second_mirror = second.get_mirror_portal();

                    down_portals.retain(|&p| p != first_mirror);
                    down_portals.retain(|&p| p != second_mirror);

                    portal_map.insert(first_mirror, second_mirror);
                    portal_map.insert(second_mirror, first_mirror);
                }
            }
        },
    }

    // info!("{:?}", portal_map);
    //
    // let mut keys = crate::DashSet::default();
    // let mut values = crate::DashSet::default();
    // for (k, v) in &portal_map {
    //     keys.insert(k);
    //     values.insert(v);
    // }
    //
    // assert_eq!(56, keys.len());
    // assert_eq!(56, values.len());
    //
    // info!("PASS");

    Ok(portal_map.iter().map(|(&a, &b)| (a, b)).collect())
}

fn create_map<T>(portal_map: &mut DashMap<T, T>, vec1: &Vec<T>, vec2: &Vec<T>)
where
    T: Copy + Eq + Hash + PartialEq,
{
    for i in 0..vec1.len() {
        portal_map.insert(vec1[i], vec2[i]);
        portal_map.insert(vec2[i], vec1[i]);
    }
}

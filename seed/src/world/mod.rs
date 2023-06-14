use {
    crate::{
        hints::hint_color::HintColor,
        settings::logic_mode::LogicMode,
        world::{
            check::{
                Check,
                CheckType::{self, *},
            },
            location::Location,
            logic::Logic,
            path::Path,
            region::Region,
        },
    },
    jack::rom::fs::{
        US_English::*,
        World::{self, Byaml::*},
    },
    std::collections::{BTreeSet, HashMap, HashSet},
};

pub mod check;
pub mod location;
pub mod logic;
pub mod path;
pub mod region;
pub mod regions;

macro_rules! world {
    (
        $(
            $region:ident {
                name: $region_name:literal,
                color: $color:ident,
                locations: [
                    $(
                        $location:ident
                        {
                            $(
                                checks: [
                                    $(
                                        $check:ident {
                                            name: $check_name:literal,
                                            check_type: $check_type:expr,
                                            $(logic: [
                                                $($check_logic_mode:ident: $check_logic:expr,)*
                                            ],)?
                                        },
                                    )*
                                ],
                            )?
                            $(
                                paths: [
                                    $(
                                        $path:ident $({
                                            $(logic: [
                                                $($path_logic_mode:ident: $path_logic:expr,)*
                                            ],)?
                                        })?,
                                    )*
                                ],
                            )?
                        },
                    )+
                ],
            },
        )+
    ) => {
        use crate::world::RegionId::*;
        use crate::world::LocationId::*;



        /// Regions
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum RegionId {
            $($region,)+
        }

        /// Locations
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum LocationId {
            $($($location,)+)+
        }

        /// Checks
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum CheckId {
            $($($($($check,)*)?)+)+
        }

        pub(crate) fn region_map() -> HashMap<RegionId, HashSet<Region>> {
            HashMap::from([
                $((
                    $region, HashSet::from([
                        Region::new($region, $region_name, HintColor::$color, BTreeSet::from([
                            $($location,)+
                        ]))
                ])),)+
            ])
        }

        pub(crate) fn check_map() -> HashMap<CheckId, Check> {
            HashMap::from([
                $($(
                    $($((
                        CheckId::$check, Check::new(CheckId::$check, $check_name.to_owned(), $check_type, Logic::new()
                           $($(.add(LogicMode::$check_logic_mode, $check_logic ))*.to_owned())?
                    )),)*)?
                )+)+
            ])
        }

        // Map of Locations to Checks
        pub(crate) fn location_map() -> HashMap<LocationId, Location> {
            HashMap::from([
                $($((
                    $location,Location::new($location,
                        Vec::from([$($(CheckId::$check,)*)?]),
                        Vec::from([
                            $($(
                                Path::new($path, Logic::new()
                                $($($(.add(LogicMode::$path_logic_mode, $path_logic ))*.to_owned())?)?
                            ),)*)?
                        ])
                )),)+)+
            ])
        }
    }
}



world!(
    // Hyrule ----------------------------------------------------------------------------------------------------------
    LostWoodsRegion {
        name: "Lost Woods Region",
        color: Name,
        locations: [
            LostWoodsLocation {
                checks: [
                    FortuneTeller {
                        name: "Fortune-Teller",
                        check_type: msbf(INDOOR_LIGHT, "FieldLight_11_FortuneGirl", 4),
                    },
                    HyruleHotfoot1 {
                        name: "Hyrule Hotfoot (First Race)",
                        check_type: msbf(FIELD_LIGHT, "FieldLight_HyruleRace", 0x21),
                        logic: [], // todo
                    },
                    HyruleHotfoot2 {
                        name: "Hyrule Hotfoot (Second Race)",
                        check_type: msbf(FIELD_LIGHT, "FieldLight_HyruleRace", 0x14),
                        logic: [], // todo
                    },
                    LostWoodsAlcove {
                        name: "Lost Woods Alcove",
                        check_type: heart(FIELD_LIGHT_STAGE_1, 46),
                        logic: [], // todo
                    },
                    LostWoodsBigRockChest {
                        name: "Lost Woods Big Rock Chest",
                        check_type: chest(FIELD_LIGHT_STAGE_1, 133),
                        logic: [], // todo
                    },
                    MasterSwordPedestal {
                        name: "Master Sword Pedestal",
                        check_type: chest(FIELD_LIGHT_STAGE_34, 71),
                        logic: [], // todo
                    },
                    RossoCave {
                        name: "Rosso Cave",
                        check_type: chest(INDOOR_LIGHT_STAGE_6, 6),
                        logic: [], // todo
                    },
                ],
                paths: [
                    RossoHouse,
                ],
            },
            RossoHouse {
                checks: [
                    Rosso {
                        name: "Rosso",
                        check_type: chest(INDOOR_LIGHT_STAGE_10, 7),
                        logic: [
                            Normal: |p| if p.is_rse() { p.has_sage_rosso() } else { p.has_pendant_of_courage() },
                        ],
                    },
                    RossoRocks {
                        name: "Rosso Rocks",
                        check_type: chest(INDOOR_LIGHT_STAGE_10, 25),
                        logic: [
                            Normal: |p| {
                                p.has_power_glove() &&
                                if p.is_rse() {
                                    p.has_sage_rosso()
                                } else {
                                    p.has_pendant_of_courage()
                                }
                            },
                        ],
                    },
                ],
                paths: [
                    LostWoodsLocation,
                    SkullWoodsOverworld {
                        logic: [
                            Normal: |p| p.can_merge(),
                        ],
                    },
                ],
            },
        ],
    },
    DeathMountainRegion {
        name: "Death Mountain",
        color: Attention,
        locations: [
            DeathMountainEast1F {

            },
        ],
    },
    ZorasRiverRegion {
        name: "Zora's River",
        color: Name,
        locations: [
            ZorasRiver {

            },
        ],
    },
    KakarikoRegion {
        name: "Kakariko Village",
        color: Name,
        locations: [
            KakarikoVillage {
                checks: [
                    DodgeTheCuccos {
                        name: "Dodge the Cuccos",
                        check_type: msbf(FIELD_LIGHT, "FieldLight_29_Kokko", 0x67),
                    },
                ],
            },
        ],
    },
    CentralHyruleRegion {
        name: "Central Hyrule",
        color: Green,
        locations: [
            RaviosShop {
                checks: [
                    Ravio1 {
                        name: "Ravio (1)",
                        check_type: Ravio(0),
                    },
                    Ravio2 {
                        name: "Ravio (2)",
                        check_type: Ravio(5),
                    },
                    Ravio3 {
                        name: "Ravio (3)",
                        check_type: Ravio(2),
                    },
                    Ravio4 {
                        name: "Ravio (4)",
                        check_type: Ravio(3),
                    },
                    Ravio5 {
                        name: "Ravio (5)",
                        check_type: Ravio(8),
                    },
                    Ravio6 {
                        name: "Ravio (6)",
                        check_type: Ravio(1),
                        logic: [
                            Normal: |p| p.has_sage_osfala(),
                        ],
                    },
                    Ravio7 {
                        name: "Ravio (7)",
                        check_type: Ravio(7),
                    },
                    Ravio8 {
                        name: "Ravio (8)",
                        check_type: Ravio(6),
                    },
                    Ravio9 {
                        name: "Ravio (9)",
                        check_type: Ravio(4),
                    },
                ],
                paths: [
                    HyruleFieldCentral,
                ],
            },
            HyruleFieldCentral {
                checks: [],
                paths: [
                    RaviosShop,
                ],
            },
            HyruleFieldGraveyard {
                checks: [
                    Dampe {
                        name: "Dampe",
                        check_type: msbf(FIELD_LIGHT, "FieldLight_13_Sister", 29),
                    },
                ],
                paths: [],
            },
        ],
    },
    EasternRuinsRegion {
        name: "Eastern Ruins",
        color: Name,
        locations: [
            EasternRuins {
                checks: [
                    PegCircle {
                        name: "Peg Circle",
                        check_type: heart(World::Byaml::FIELD_LIGHT_STAGE_30, 41),
                        logic: [
                            Normal: |p| p.has_hammer(),
                            Glitched: |p| { p.has_boomerang() || p.has_hookshot() },
                            AdvGlitched: |p| p.has_tornado_rod(),
                            Hell: |p| p.has_sand_rod(),
                        ],
                    },
                ],
                paths: [],
            },
        ],
    },
    DesertRegion {
        name: "Desert of Mystery",
        color: Name,
        locations: [
            Desert {

            },
        ],
    },
    SouthernRuinsRegion {
        name: "SouthernRuins",
        color: Name,
        locations: [
            SouthernRuins {

            },
        ],
    },
    LakeHyliaRegion {
        name: "Lake Hylia",
        color: Name,
        locations: [
            LakeHyliaNW {
                checks: [],
                paths: [
                    MaiamaiCave {
                        logic: [
                            Normal: |p| !p.nice_mode(),
                        ],
                    },
                ],
            },
            LakeHyliaSE {

            },
            MaiamaiCave {
                checks: [
                    Maiamai10 {
                        name: " 10 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(10),
                        ],
                    },
                    Maiamai20 {
                        name: " 20 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(20),
                        ],
                    },
                    Maiamai30 {
                        name: " 30 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(30),
                        ],
                    },
                    Maiamai40 {
                        name: " 40 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(40),
                        ],
                    },
                    Maiamai50 {
                        name: " 50 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(50),
                        ],
                    },
                    Maiamai60 {
                        name: " 60 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(60),
                        ],
                    },
                    Maiamai70 {
                        name: " 70 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(70),
                        ],
                    },
                    Maiamai80 {
                        name: " 80 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(80),
                        ],
                    },
                    Maiamai90 {
                        name: " 90 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(90),
                        ],
                    },
                    Maiamai100 {
                        name: "100 Maiamai",
                        check_type: None, // todo
                        logic: [
                            Normal: |p| p.has_maiamai(100),
                        ],
                    },
                ],
                paths: [
                    LakeHyliaNW,
                ],
            },
        ],
    },
    IreneTheRegion {
        name: "Irene the Witch",
        color: Name,
        locations: [
            IreneLocation {
                checks: [
                    Irene {
                        name: "Irene",
                        check_type: Multi(vec![
                            msbf(FIELD_LIGHT, "FieldLight_11_Maple", 6),
                            msbf(FIELD_LIGHT, "FieldLight_12_Maple", 8),
                            msbf(FIELD_LIGHT, "FieldLight_12_Maple", 38),
                            msbf(FIELD_LIGHT, "FieldLight_2D_Maple", 7),
                        ]),
                        logic: [
                            Normal: |p| { if p.is_rse() { p.has_sage_irene() } else { p.has_pendant_of_courage() } },
                        ],
                    },
                ],
                paths: [
                    LostWoodsLocation,
                    EasternRuins,
                ],
            },
        ],
    },
    // Lorule ----------------------------------------------------------------------------------------------------------
    SkullWoodsRegion {
        name: "Skull Woods Region",
        color: Name,
        locations: [
            SkullWoodsOverworld {

            },
        ],
    },
    LoruleDeathMountainRegion {
        name: "Death Mountain",
        color: Name,
        locations: [
            LoruleDeathMountainWest {

            },
        ],
    },
    DarkRuinsRegion {
        name: "Dark Ruins",
        color: Name,
        locations: [
            DarkRuins {

            },
        ],
    },
    CentralLoruleRegion {
        name: "Central Lorule",
        color: Name,
        locations: [
            CentralLorule {

            },
        ],
    },
    MiseryMireRegion {
        name: "Misery Mire",
        color: Name,
        locations: [
            MiseryMire {

            },
        ],
    },
    LoruleLakeRegion {
        name: "Lorule Lake",
        color: Name,
        locations: [
            LoruleLake {

            },
        ],
    },
    // Graveyard (shared between Hyrule/Lorule) ------------------------------------------------------------------------
    GraveyardRegion {
        name: "Graveyard",
        color: Purple,
        locations: [
            HyruleGraveyard {

            },
            LoruleGraveyard {

            },
        ],
    },
    // Dungeons --------------------------------------------------------------------------------------------------------
    EasternPalace {
        name: "Eastern Palace",
        color: Green,
        locations: [
            EasternPalaceFoyer {

            },
        ],
    },
    HouseOfGales {
        name: "House of Gales",
        color: Blue,
        locations: [
            HouseOfGalesFoyer {

            },
        ],
    },
    TowerOfHera {
        name: "Tower of Hera",
        color: Red,
        locations: [
            TowerOfHeraFoyer {

            },
        ],
    },
    InsideHyruleCastle {
        name: "Inside Hyrule Castle",
        color: Name,
        locations: [
            HyruleCastleDungeon {
                checks: [],
                paths: [
                    LostWoodsLocation,
                    SkullWoodsOverworld {
                        logic: [
                            Normal: |p| p.can_merge(),
                        ],
                    },
                ],
            },
        ],
    },
    DarkPalace {
        name: "Dark Palace",
        color: Green,
        locations: [
            DarkPalaceFoyer {

            },
        ],
    },
    SwampPalace {
        name: "Swamp Palace",
        color: Beige,
        locations: [
            SwampPalaceFoyer {

            },
        ],
    },
    SkullWoods {
        name: "Skull Woods",
        color: Blue,
        locations: [
            SkullWoodsFoyer {

            },
        ],
    },
    ThievesHideout {
        name: "Thieves' Hideout",
        color: Beige,
        locations: [
            ThievesHideoutFoyer {

            },
        ],
    },
    TurtleRock {
        name: "Turtle Rock",
        color: Red,
        locations: [
            TurtleRockFoyer {

            },
        ],
    },
    DesertPalace {
        name: "Desert Palace",
        color: Blue,
        locations: [
            DesertPalaceFoyer {

            },
        ],
    },
    IceRuins {
        name: "Ice Ruins",
        color: Red,
        locations: [
            IceRuinsFoyer {

            },
        ],
    },
    LoruleCastle {
        name: "Lorule Castle",
        color: Purple,
        locations: [
            LoruleCastleFoyer {

            },
        ],
    },
);

fn chest(file: &str, unq: u16) -> CheckType {
    Chest { file: file.to_owned(), unq }
}

fn heart(file: &str, unq: u16) -> CheckType {
    Heart { file: file.to_owned(), unq }
}

fn msbf(archive: &str, file: &str, index: u16) -> CheckType {
    Msbf { archive: archive.to_owned(), file: format!("World/Flow/{}.msbf", file), index }
}

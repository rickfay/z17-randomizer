use {
    crate::filler::location::{LocationId, LocationId::*},
    jack::byaml::course::{CourseId, CourseId::*},
    std::collections::HashMap,
    LoadingZoneId::*,
};

/**

Desired Groups:
- Dungeon Entrances
- Cave Entrances
- Portals

 */
#[allow(unused)]
pub struct LoadingZones {}

#[allow(unused)]
pub struct LoadingZone {
    location: LocationId,

    unq: u16,

    scene: CourseId,
    scene_id: u16,
    spawn: u16,

    flag: u16,
}

// pub struct SpawnPoint {
//     scene: Id,
//     scene_id: u16,
//     spawn_id: u16,
// }
//
// pub struct Entrance {
//     loading_zone: Option<LoadingZone>,
//     spawn_point: Option<SpawnPoint>,
// }

macro_rules! loading_zones {
    (
        $($func:ident {
            $(
                $name:ident: entry($entry_loc:ident, $entry_scene:ident, $entry_scene_id:literal, $entry_unq:literal, $entry_spawn:literal) exit($exit_loc:ident, $exit_scene:ident, $exit_scene_id:literal, $exit_unq:literal, $exit_spawn:literal),
            )+
        })+
    ) => {
        #[derive(Eq, PartialEq, Hash)]
        #[allow(non_camel_case_types)]
        #[allow(unused)]
        pub enum LoadingZoneId {
            $($($name,)+)+
        }

        #[allow(unused)]
        impl LoadingZones {
            $(pub fn $func() -> (HashMap<LoadingZoneId, LoadingZone>, HashMap<LoadingZoneId, LoadingZone>) {
                let mut entrances = HashMap::new();
                let mut exits = HashMap::new();

                $(
                    entrances.insert($name, LoadingZone { location: $entry_loc, scene: $entry_scene, scene_id: $entry_scene_id, unq: $entry_unq, spawn: $entry_spawn, flag: 0 });
                    exits.insert($name, LoadingZone { location: $exit_loc, scene: $exit_scene, scene_id: $exit_scene_id, unq: $exit_unq, spawn: $exit_spawn, flag: 0 });
                )+

                (entrances, exits)
            })+
        }
    };
}
/*macro_rules! portals {
    (
        $(#[$attr:meta])*
        $type:ident {
            $($name:ident:
                hyrule($hyrule_loc:ident, $hyrule_scene:ident, $hyrule_scene_id:literal, $hyrule_unq:literal, $hyrule_spawn:literal, $hyrule_flag:literal)
                lorule($lorule_loc:ident, $lorule_scene:ident, $lorule_scene_id:literal, $lorule_unq:literal, $lorule_spawn:literal, $lorule_flag:literal),
            )+
        }
    ) => {
    $(#[$attr])*
    pub struct $type {
        hyrule: LoadingZone,
        lorule: LoadingZone,
    }

    $(#[$attr])*
        impl $type {
            $(pub const $name: Self = Self {
                hyrule: LoadingZone { location: $hyrule_loc, scene: $hyrule_scene, scene_id: $hyrule_scene_id, unq: $hyrule_unq, spawn: $hyrule_spawn, flag: $hyrule_flag },
                lorule: LoadingZone { location: $lorule_loc, scene: $lorule_scene, scene_id: $lorule_scene_id, unq: $lorule_unq, spawn: $lorule_spawn, flag: $lorule_flag }
            };)+
        }
    };
}*/

loading_zones! {
    hyrule_entrances {
        AMIDA_CAVE_LOWER: entry(DeathSecondFloor, FieldLight, 3, 94, 8) exit(AmidaCaveLower, CaveLight, 2, 50, 0),
        AMIDA_CAVE_MIDDLE_LEFT: entry(DeathThirdFloor, FieldLight, 3, 190, 21) exit(AmidaCaveUpper, CaveLight, 2, 66, 21),
        AMIDA_CAVE_MIDDLE_RIGHT: entry(DeathThirdFloor, FieldLight, 3, 193, 22) exit(AmidaCaveLower, CaveLight, 2, 99, 22),
        AMIDA_CAVE_TOP: entry(DeathTopLeftLedge, FieldLight, 3, 195, 23) exit(AmidaCaveUpper, CaveLight, 2, 70, 23),
        BEE_GUY_HOUSE: entry(HyruleField, FieldLight, 16, 208, 11) exit(BeeGuyHouse, IndoorLight, 17, 5, 0),
        BLACKSMITH_HOUSE: entry(HyruleField, FieldLight, 21, 79, 4) exit(BlacksmithHouse, IndoorLight, 19, 2, 0),
        BLACKSMITH_CAVE: entry(HyruleField, FieldLight, 21, 150, 6) exit(BlacksmithCave, CaveLight, 16, 4, 0),
        CUCCO_DUNGEON: entry(CuccoDungeonLedge, FieldLight, 32, 62, 3) exit(CuccoDungeon, AttractionLight, 3, 12, 0),
        CUCCO_HOUSE_FRONT: entry(HyruleField, FieldLight, 16, 403, 18) exit(CuccoHouse, IndoorLight, 9, 4, 0),
        CUCCO_HOUSE_REAR: entry(CuccoHouseRear, FieldLight, 16, 404, 19) exit(CuccoHouse, IndoorLight, 9, 5, 1),
        DESERT_FAIRY_CAVE: entry(DesertFairyLedge, FieldLight, 31, 63, 17) exit(DesertFairyCave, CaveLight, 8, 5, 0),
        DESERT_PALACE_MAIN: entry(DesertPalaceWeatherVane, FieldLight, 31, 10, 2) exit(DesertPalaceFoyer, DungeonSand, 1, 38, 0),
        // DESERT_PALACE_1F_SIDE: entry(DesertPalaceMidwayLedge, FieldLight, 31, 10, 2) exit(oh goddddd whyyyy, DungeonSand, 1, 38, 0),
        // DESERT_PALACE_2F: entry(DesertPalaceMidwayLedge, FieldLight, 31, 19, 5) exit(ahhhhhhhh, DungeonSand, 2, 11, 0),
        // DESERT_PALACE_3F: entry(DesertZaganagaLedge, FieldLight, 31, 21, 6) exit(DesertPalaceExit3F, 3, 20, 9),
        DM_BIG_FAIRY_CAVE: entry(RossosOreMine, FieldLight, 4, 56, 6) exit(RossosOreMineFairyCave, CaveLight, 24, 5, 0),
        DM_BOMB_CAVE: entry(DeathMountainBase, FieldLight, 3, 184, 12) exit(DeathBombCave, CaveLight, 3, 51, 2),
        DM_FAIRY_CAVE: entry(DeathMountainBase, FieldLight, 3, 188, 14) exit(DeathFairyCave, CaveLight, 3, 53, 4),
        DM_WV_CAVE: entry(DeathMountainBase, FieldLight, 3, 183, 13) exit(DeathWeatherVaneCaveLeft, CaveLight, 3, 50, 3),
        DONKEY_CAVE_LEDGE: entry(DeathWestLedge, FieldLight, 3, 83, 6) exit(DonkeyCaveUpper, CaveLight, 1, 62, 1),
        DONKEY_CAVE_LOWER: entry(DeathMountainBase, FieldLight, 3, 82, 5) exit(DonkeyCaveLower, CaveLight, 1, 64, 0),
        DONKEY_CAVE_TOP: entry(DeathSecondFloor, FieldLight, 3, 84, 7) exit(DonkeyCaveUpper, CaveLight, 1, 63, 2),
        EASTERN_BIG_FAIRY_CAVE: entry(HyruleField, FieldLight, 29, 49, 4) exit(EasternBigFairyCave, CaveLight, 12, 5, 0),
        EASTERN_PALACE: entry(EasternRuinsUpper, FieldLight, 20, 36, 0) exit(EasternPalaceFoyer, DungeonEast, 1, 31, 0),
        EAST_RUINS_BOMB_CAVE_LOWER: entry(HyruleField, FieldLight, 20, 161, 7) exit(EastRuinsBombCaveLower, CaveLight, 29, 3, 0),
        EAST_RUINS_BOMB_CAVE_UPPER: entry(EasternRuinsEastLedge, FieldLight, 20, 160, 6) exit(EastRuinsBombCaveUpper, CaveLight, 29, 4, 1),
        WITCH_CAVE_LOWER: entry(EasternRuinsUpper, FieldLight, 20, 162, 8) exit(WitchCave, CaveLight, 30, 3, 0),
        WITCH_CAVE_UPPER: entry(HyruleField, FieldLight, 14, 75, 4) exit(WitchCave, CaveLight, 30, 4, 1),
        EASTERN_FAIRY_CAVE: entry(HyruleField, FieldLight, 30, 71, 0) exit(EasternFairyCave, CaveLight, 10, 5, 0),
        TOWER_OF_HERA : entry( TowerOfHeraEntrancePegs, FieldLight, 3, 14, 3 )  exit( TowerOfHeraFoyer, DungeonHera, 1, 758, 0 ),





        // fixme set correct locations
/*
        FIRE_CAVE_BOTTOM : entry( dest, FieldLight, 4, 54, 5 ) exit( dest, CaveLight, 25, 108, 6 ),
        FIRE_CAVE_MIDDLE_LEFT : entry( dest, FieldLight, 4, 52, 3 ) exit( dest, CaveLight, 25, 106, 4 ),
        FIRE_CAVE_MIDDLE_RIGHT : entry( dest, FieldLight, 4, 53, 4 )  exit( dest, CaveLight, 25, 107, 5 ),
        FIRE_CAVE_TOP : entry( dest, FieldLight, 4, 51, 2 )  exit( dest, CaveLight, 25, 105, 0 ),
        FLIPPERS_DUNGEON : entry( dest, FieldLight, 33, 330, 0 )  exit( dest, AttractionLight, 2, 9, 0 ),
        FORTUNE_TELLER : entry( dest, FieldLight, 9, 68, 4 )  exit( dest, IndoorLight, 18, 5, 0 ),
        FORTUNE_TELLER_FAIRY_CAVE : entry( dest, FieldLight, 9, 92, 5 )  exit( dest, CaveLight, 21, 3, 0 ),
        FORTUNES_CHOICE : entry( dest, FieldLight, 16, 283, 14 )  exit( dest, IndoorLight, 20, 5, 0 ),
        HOOKSHOT_DUNGEON : entry( dest, FieldLight, 5, 48, 0 )  exit( dest, AttractionLight, 4, 3, 0 ),
        HOUSE_OF_GALES : entry( dest, FieldLight, 35, 43, 0 )  exit( dest, DungeonWind, 1, 305, 0 ),
        HYRULE_CASTLE_FRONT_DOOR : entry( dest, FieldLight, 18, 160, 10 )  exit( dest, IndoorLight, 12, 17, 0 ),
        HYRULE_CASTLE_LOWER_LEFT : entry( dest, FieldLight, 18, 374, 16 )  exit( dest, IndoorLight, 12, 50, 16 ),
        HYRULE_CASTLE_LOWER_RIGHT : entry( dest, FieldLight, 18, 375, 15 )  exit( dest, IndoorLight, 12, 51, 15 ),
        HYRULE_CASTLE_UPPER_LEFT : entry( dest, FieldLight, 18, 159, 12 )  exit( dest, IndoorLight, 12, 19, 7 ),
        HYRULE_CASTLE_UPPER_RIGHT : entry( dest, FieldLight, 18, 154, 11 )  exit( dest, IndoorLight, 12, 18, 5 ),
        ICE_ROD_CAVE_LEFT : entry( dest, FieldLight, 36, 34, 16 )  exit( dest, CaveLight, 9, 10, 1 ),
        ICE_ROD_CAVE_RIGHT : entry( dest, FieldLight, 36, 24, 15 )  exit( dest, CaveLight, 9, 5, 0 ),
        INSIDE_HYRULE_CASTLE : entry( dest, FieldLight, 18, 155, 0 )  exit( dest, DungeonCastle, 1, 15, 0 ),
        KAKARIKO_ITEM_SHOP : entry( dest, FieldLight, 16, 269, 9 )  exit( dest, IndoorLight, 8, 5, 0 ),
        KAKARIKO_JAIL : entry( dest, FieldLight, 16, 396, 17 )  exit( dest, IndoorLight, 3, 4, 0 ),
        KAKARIKO_WELL : entry( dest, FieldLight, 16, 278, 15 )  exit( dest, CaveLight, 4, 7, 0 ),
        //KAKARIKO_WELL : entry( dest, FieldLight, 16, 277, - )  exit( dest, CaveLight, 4, -, 1 ), // todo add kak well spawn point and exit
        LAKE_DARK_CAVE : entry( dest, FieldLight, 35, 135, 7 )  exit( dest, CaveLight, 11, 7, 0 ),
        LAKESIDE_ITEM_SHOP : entry( dest, FieldLight, 35, 86, 6 )  exit( dest, IndoorLight, 6, 5, 0 ),
        MAIAMAI_CAVE : entry( dest, FieldLight, 35, 140, 8 )  exit( dest, CaveLight, 15, 4, 0 ),
        MERGE_DUNGEON : entry( dest, FieldLight, 20, 164, 2 )  exit( dest, AttractionLight, 1, 3, 0 ),
        MILK_BAR : entry( dest, FieldLight, 16, 271, 12 )  exit( dest, IndoorLight, 15, 5, 0 ),
        MOLDORM_CAVE_BOTTOM : entry( dest, FieldLight, 6, 10, 3 )  exit( dest, CaveLight, 19, 4, 0 ),
        MOLDORM_CAVE_LEDGE : entry( dest, FieldLight, 6, 138, 4 )  exit( dest, CaveLight, 19, 13, 2 ),
        MOLDORM_CAVE_TOP : entry( dest, FieldLight, 3, 12, 0 )  exit( dest, CaveLight, 19, 5, 1 ),
        RAVIOS_SHOP : entry( dest, FieldLight, 27, 51, 5 )  exit( dest, IndoorLight, 1, 24, 1 ),
        ROSSO_CAVE : entry( dest, FieldLight, 2, 95, 3 )  exit( dest, CaveLight, 6, 7, 0 ),
        //ROSSOS_HOUSE : entry( dest, FieldLight, 2, 88, 0 ) todo
        ROSSOS_HOUSE_DUPLICATE : entry( dest, FieldLight, 2, 136, 0 )  exit( dest, IndoorLight, 10, 5, 0 ),
        RUMOR_GUY_CAVE : entry( dest, FieldLight, 1, 233, 5 )  exit( dest, CaveLight, 17, 3, 0 ),
        RUNAWAY_ITEM_SELLER_CAVE : entry( dest, FieldLight, 33, 314, 7 )  exit( dest, CaveLight, 27, 3, 0 ),
        SAHASRAHLAS_HOUSE_LEFT : entry( dest, FieldLight, 16, 262, 8 )  exit( dest, IndoorLight, 16, 11, 1 ),
        SAHASRAHLAS_HOUSE_RIGHT : entry( dest, FieldLight, 16, 258, 7 )  exit( dest, IndoorLight, 16, 4, 0 ),
        SANCTUARY : entry( dest, FieldLight, 11, 103, 0 )  exit( dest, IndoorLight, 11, 7, 1 ),
        SANCTUARY_GRAVESTONE : entry( dest, FieldLight, 12, 75, 3 )  exit( dest, CaveLight, 18, 106, 0 ),
        SANCTUARY_PORTAL_CAVE : entry( dest, FieldLight, 12, 125, 4 )  exit( dest, CaveLight, 5, 4, 0 ),
        SOUTHERN_RUINS_BIG_FAIRY_CAVE : entry( dest, FieldLight, 37, 44, 4 )  exit( dest, CaveLight, 20, 5, 0 ),
        SOUTHERN_RUINS_BOMB_CAVE : entry( dest, FieldLight, 33, 319, 8 )  exit( dest, CaveLight, 28, 10, 0 ),
        SOUTHERN_RUINS_FAIRY_CAVE : entry( dest, FieldLight, 33, 303, 6 )  exit( dest, CaveLight, 26, 5, 0 ),
        SOUTHERN_RUINS_PILLAR_CAVE : entry( dest, FieldLight, 33, 316, 9 )  exit( dest, CaveLight, 28, 11, 1 ),
        SPECTACLE_ROCK_LEFT : entry( dest, FieldLight, 3, 305, 26 )  exit( dest, CaveLight, 3, 87, 6 ),
        SPECTACLE_ROCK_RIGHT : entry( dest, FieldLight, 3, 307, 25 )  exit( dest, CaveLight, 3, 85, 5 ),
        //STYLISH_WOMANS_HOUSE : entry( dest, FieldLight, 16, 206, 10 ) todo
        STYLISH_WOMANS_HOUSE_DUPLICATE : entry( dest, FieldLight, 16, 272, 10 )  exit( dest, IndoorLight, 14, 5, 0 ),
        TORNADO_ROD_DUNGEON : entry( dest, FieldLight, 13, 37, 0 )  exit( dest, AttractionLight, 5, 3, 0 ),

        WITCHS_HUT : entry( dest, FieldLight, 14, 48, 3 )  exit( dest, IndoorLight, 2, 8, 0 ),
        WOMANS_HOUSE : entry( dest, FieldLight, 16, 284, 16 )  exit( dest, IndoorLight, 21, 5, 0 ),
        ZORAS_DOMAIN : entry( dest, FieldLight, 7, 13, 0 )  exit( dest, CaveLight, 7, 112, 0 ),
 */
    }

    lorule_entrances {
        BIG_BOMB_FLOWER_SHOP: entry(BigBombFlowerField, FieldDark, 24, 45, 0) exit(BigBombFlowerShop, IndoorDark, 3, 32, 1),
    }

    portals {
        PORTAL_DM_WEST: entry(DeathMountainBase, FieldLight, 3, 361, 20) exit(LoruleDeathWest, FieldDark, 3, 45, 3),
    }


}

// portals! {
//     #[allow(unused)]
//     Portal {
//         DEATH_MOUNTAIN_WEST: hyrule(DeathMountainBase, FieldLight, 3, 361, 20, 818) lorule(LoruleDeathWest, FieldDark, 3, 45, 3, 848),
//     }
// }

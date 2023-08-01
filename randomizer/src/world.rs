use std::collections::HashMap;

use game::world::{self as game_world};
use log::info;
use modd::filler_item::FillerItem::{self, *};

use crate::model::{
    check::Check,
    location::{Location, Location::*},
    location_node::LocationNode,
    logic::Logic,
    progress::Progress,
};

pub type WorldGraph = HashMap<Location, LocationNode>;

#[derive(Copy, Clone)]
pub struct Path {
    destination: Location,
    logic: Logic,
}

impl Path {
    pub fn new(destination: Location, logic: Logic) -> Self {
        Self { destination, logic }
    }

    pub fn get_destination(self) -> Location {
        self.destination
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        self.logic.can_access(progress)
    }
}

// TODO Rewrite logic using combinators

/// Build the World Graph
pub fn build_world_graph() -> WorldGraph {
    info!("Building World Graph...");

    let mut world = WorldGraph::new();

    world.extend(hyrule());
    world.extend(lorule());

    world.extend(eastern_palace());
    world.extend(house_of_gales());
    world.extend(tower_of_hera());

    world.extend(inside_hyrule_castle());

    world.extend(dark_palace());
    world.extend(swamp_palace());
    world.extend(skull_woods());
    world.extend(thieves_hideout());
    world.extend(ice_ruins());
    world.extend(desert_palace());
    world.extend(turtle_rock());

    world.extend(lorule_castle());

    world
}

/// Hyrule
fn hyrule() -> HashMap<Location, LocationNode> {
    HashMap::from([
        // Starting Node
        (
            RavioShop,
            location(
                "Ravio's Shop",
                vec![
                    check_free(game_world::hyrule::field::main::get("Ravio (1)").unwrap()),
                    check_free(game_world::hyrule::field::main::get("Ravio (2)").unwrap()),
                    check_free(game_world::hyrule::field::main::get("Ravio (3)").unwrap()),
                    check_free(game_world::hyrule::field::main::get("Ravio (4)").unwrap()),
                    check_free(game_world::hyrule::field::main::get("Ravio (5)").unwrap()),
                    check(
                        game_world::hyrule::field::main::get("Ravio (6)").unwrap(),
                        Some(|p| p.has_sage_osfala()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(game_world::hyrule::field::main::get("Ravio (7)").unwrap()),
                    check_free(game_world::hyrule::field::main::get("Ravio (8)").unwrap()),
                    check_free(game_world::hyrule::field::main::get("Ravio (9)").unwrap()),
                ],
                vec![
                    path_free(HyruleField),
                    path_free(ChamberOfSages), // not technically true but gives us what we need
                ],
            ),
        ),
        (
            ChamberOfSages,
            location(
                "Chamber of Sages",
                vec![check(
                    game_world::lorule::chamber::sages::get("Osfala").unwrap(),
                    Some(|p| p.has_sage_osfala()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![],
            ),
        ),
        (
            HyruleBellTravel,
            location(
                "Hyrule Bell Travel",
                vec![],
                vec![
                    path_free(HyruleField),
                    path_free(DesertPalaceWeatherVane),
                    path_free(EasternRuinsUpper),
                    path_free(HouseOfGalesIsland),
                    path_free(DeathMountainBase),
                    path_free(DeathMountainWestTop),
                ],
            ),
        ),
        (
            HyruleField,
            location(
                "Hyrule Field",
                vec![
                    check_free(game_world::dungeons::graveyards::hyrule::get("Dampe").unwrap()),
                    check(
                        game_world::hyrule::irene::witch::get("Irene").unwrap(),
                        Some(|p| {
                            if p.is_rse() {
                                p.has_sage_irene()
                            } else {
                                p.has_pendant_of_courage()
                            }
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::hyrule::get("Sanctuary Pegs").unwrap(),
                        Some(|p| p.has_hammer()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("Behind Blacksmith").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boosting
                    ),
                    check(
                        game_world::hyrule::field::main::get("Hyrule Castle Rocks").unwrap(),
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("Wildlife Clearing Stump").unwrap(),
                        Some(|p| p.has_pendant_of_courage()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::southern::ruins::get("Southern Ruins Ledge").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    // Lake Hylia
                    check(
                        game_world::hyrule::lake::hylia::get("Lake Hylia Ledge Chest").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lake::hylia::get("Southeastern Shore").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boosting
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("Hyrule Hotfoot (First Race)")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("Hyrule Hotfoot (Second Race)")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        Some(|p| p.can_merge() && p.has_bell()),
                        None,
                        None,
                        Some(|_| true), // Can just walk it
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("Bird Lover").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None, // Fake Flippers does not work
                        None,
                    ),
                    // Kakariko Village
                    check_free(
                        game_world::hyrule::kakariko::village::get("Street Merchant (Left)")
                            .unwrap(),
                    ),
                    check(
                        game_world::hyrule::kakariko::village::get("Street Merchant (Right)")
                            .unwrap(),
                        Some(|p| p.has_shady_guy_trigger()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::kakariko::village::get("Shady Guy").unwrap(),
                        Some(|p| p.has_shady_guy_trigger() && (p.can_merge() || p.has_boots())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::hyrule::kakariko::village::get("Dodge the Cuccos").unwrap(),
                    ),
                    check_free(
                        game_world::hyrule::kakariko::village::get("Rupee Rush (Hyrule)").unwrap(),
                    ),
                    check_free(
                        game_world::hyrule::kakariko::village::get("[Mai] Kakariko Bush").unwrap(),
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("[Mai] Lost Woods Path Rock").unwrap(),
                        Some(|p| p.has_titans_mitt() || (p.has_power_glove() && p.has_hammer())),
                        None,
                        Some(|p| {
                            p.has_power_glove()
                                && (p.has_hookshot() || (p.has_boomerang() && p.can_escape()))
                        }),
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("[Mai] Fortune-Teller Tent").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::kakariko::village::get("[Mai] Woman's Roof Rock")
                            .unwrap(),
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest(
                        "Woman Roof Maiamai",
                        WomanRoofMaiamai,
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    // Eastern Ruins
                    check(
                        game_world::hyrule::eastern::ruins::get("Eastern Ruins Peg Circle")
                            .unwrap(),
                        Some(|p| p.has_hammer()),
                        None,
                        Some(|p| p.has_boomerang() || p.has_hookshot()),
                        Some(|p| p.has_tornado_rod()),
                        Some(|p| p.has_sand_rod()),
                    ),
                    // Maiamai
                    check(
                        game_world::hyrule::lost::woods::get("[Mai] Rosso Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("[Mai] Small Pond").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::hyrule::get("[Mai] Sanctuary Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Tree Behind Blacksmith")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("[Mai] Lost Woods Tree").unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Hyrule Castle Tree").unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Hyrule Castle Tornado Tile")
                            .unwrap(),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::zora::river::get("[Mai] Under Wooden Bridge").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()), // bee boost fake flippers
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Yellow Tree")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Green Tree")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Big Rock")
                            .unwrap(),
                        Some(|p| p.can_merge() && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Blacksmith Tornado Tile")
                            .unwrap(),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("[Mai] Atop Eastern Rocks")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::kakariko::village::get("[Mai] Hyrule Rupee Rush Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::kakariko::village::get("[Mai] Cucco Ranch Tree")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Wildlife Clearing Tree")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Tree West of Link's House")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::field::main::get("[Mai] Behind Link's House").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("[Mai] Southern Bridge River")
                            .unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()), // bee boost fake flippers
                    ),
                    check(
                        game_world::hyrule::southern::ruins::get("[Mai] Southern Ruins Pillars")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::southern::ruins::get("[Mai] Outside Flippers Dungeon")
                            .unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lake::hylia::get("[Mai] Outside Maiamai Cave").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lake::hylia::get("[Mai] Lake Hylia SE Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lake::hylia::get("[Mai] Hyrule Hotfoot Big Rock")
                            .unwrap(),
                        Some(|p| p.can_merge() && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::desert::mystery::get("[Mai] Southern Ruins Big Rock")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lake::hylia::get("[Mai] Lake Hylia Shallow Ring")
                            .unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostLostWoodsMaze1),
                    ghost(HintGhostLostWoodsMaze2),
                    ghost(HintGhostLostWoodsMaze3),
                    ghost(HintGhostLostWoods),
                    ghost(HintGhostMoldormCave),
                    ghost(HintGhostFortuneTellerHyrule),
                    ghost(HintGhostSanctuary),
                    ghost(HintGhostGraveyardHyrule),
                    ghost(HintGhostWell),
                    ghost(HintGhostShadyGuy),
                    ghost(HintGhostStylishWoman),
                    ghost(HintGhostBlacksmithCave),
                    ghost(HintGhostEasternRuinsEntrance),
                    ghost(HintGhostRupeeRushHyrule),
                    ghost(HintGhostCuccos),
                    ghost(HintGhostSouthBridge),
                    ghost(HintGhostSouthernRuins),
                    ghost(HintGhostHyruleHotfoot),
                    ghost(HintGhostLetter),
                    ghost(HintGhostStreetPassTree),
                    ghost(HintGhostBlacksmithBehind),
                    ghost(HintGhostGraveyardLedge),
                    ghost(HintGhostHyruleCastleRocks),
                    ghost(HintGhostWitchsHouse),
                ],
                vec![
                    fast_travel_hyrule(),
                    path_free(RavioShop),
                    path(
                        EasternRuinsUpper,
                        Some(|p| p.can_hit_far_switch() || p.has_ice_rod() || p.can_merge()),
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                    ),
                    path(
                        EasternRuinsEastLedge,
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(WitchCave, Some(|p| p.has_bombs()), None, None, None, None),
                    path(
                        ZoraDomainArea,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        Some(|_| true), // Bee Boost
                    ),
                    path(
                        WaterfallCaveShallowWater,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(BlacksmithHouse),
                    path(
                        BlacksmithCave,
                        Some(|p| p.has_titans_mitt()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boost
                    ),
                    path_free(LostWoods),
                    path(
                        HyruleCastleCourtyard,
                        Some(|p| p.has_master_sword() || p.swordless_mode()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(FortuneTeller),
                    path_free(KakarikoJailCell),
                    path(
                        WellUpper,
                        Some(|p| p.has_power_glove()),
                        Some(|_| true), // Cucco jump
                        None,
                        None,
                        None,
                    ),
                    path_free(WellLower),
                    path_free(MilkBar),
                    path_free(BeeGuyHouse),
                    path_free(KakarikoItemShop),
                    path_free(LakesideItemShop),
                    path(ItemSellerCave, Some(|p| p.has_bombs()), None, None, None, None),
                    path(
                        FlippersDungeon,
                        Some(|p| p.has_titans_mitt()),
                        None,
                        Some(|p| p.has_sword() && p.has_ice_rod()),
                        Some(|p| p.has_ice_rod()),
                        None,
                    ),
                    path(SouthernRuinsBombCave, Some(|p| p.has_bombs()), None, None, None, None),
                    path_free(LakeDarkCave),
                    path(IceRodCave, Some(|p| p.has_bombs()), None, None, None, None),
                    path(
                        Sanctuary,
                        Some(|p| {
                            p.has_sword()
                                || p.has_bombs()
                                || p.has_fire_rod()
                                || p.has_ice_rod()
                                || p.has_lamp()
                                || p.has_boots()
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        MoldormCave,
                        Some(|p| p.has_power_glove()),
                        None,
                        Some(|_| true), // Crow boost
                        None,
                        None,
                    ),
                    path(
                        RossoHouse,
                        Some(|p| {
                            if p.is_rse() {
                                p.has_sage_rosso()
                            } else {
                                p.has_pendant_of_courage()
                            }
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        RossoCave,
                        Some(|p| p.has_hammer()),
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())),
                        Some(|p| p.not_nice_mode() && (p.can_use_shield() && p.has_tornado_rod())),
                        None,
                    ),
                    path(TornadoRodDungeon, Some(|p| p.has_bombs()), None, None, None, None),
                    path(
                        HouseOfGalesIsland,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| {
                            (p.has_hookshot() && p.has_ice_rod())
                                || (p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()))
                        }),
                        Some(|p| p.has_boots()), // Bee Boost
                    ),
                    path(HauntedGroveLedge, Some(|p| p.can_merge()), None, None, None, None),
                    path(LoruleLakeNorthWest, Some(|p| p.can_merge()), None, None, None, None),
                    path(LoruleLakeEast, Some(|p| p.can_merge()), None, None, None, None),
                    path(MiseryMire, Some(|p| p.can_merge()), None, None, None, None),
                    path(SkullWoodsOverworld, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(WitchHouse),
                    path(
                        SanctuaryChurch,
                        Some(|p| p.has_opened_sanctuary_doors()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(CuccoDungeonLedge, Some(|p| p.can_merge()), None, None, None, None),
                    path(
                        WaterfallLedge,
                        Some(|p| p.can_merge() && p.has_flippers()),
                        None,
                        None,
                        Some(|p| {
                            p.can_merge()
                                && p.has_boots()
                                && (p.has_fire_rod() || p.has_nice_bombs())
                        }),
                        Some(|p| p.can_merge() && p.has_boots()),
                    ),
                    path_free(CuccoHouse),
                    path_free(WomanHouse),
                    path(
                        StylishWomanHouse,
                        Some(|p| p.has_opened_stylish_womans_house()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(MaiamaiCave),
                ],
            ),
        ),
        (
            MaiamaiCave,
            location(
                "Mother Maiamai Cave",
                vec![
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 10 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 20 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 30 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 40 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 50 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 60 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 70 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 80 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get(" 90 Maiamai").unwrap()),
                    check_unreachable(game_world::hyrule::lake::hylia::get("100 Maiamai").unwrap()),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            WomanHouse,
            location(
                "Woman's House",
                vec![check(
                    game_world::hyrule::kakariko::village::get("Woman").unwrap(),
                    Some(|p| p.has_woman_roof_maiamai()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            CuccoHouse,
            location(
                "Cucco House",
                vec![],
                vec![path_free(HyruleField), path_free(CuccoHouseRear)],
            ),
        ),
        (
            CuccoHouseRear,
            location(
                "Cucco House Rear",
                vec![check(
                    game_world::hyrule::kakariko::village::get("[Mai] Kakariko Sand").unwrap(),
                    Some(|p| p.has_sand_rod()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![fast_travel_hyrule(), path_free(CuccoHouseRear)],
            ),
        ),
        (
            WaterfallLedge,
            location(
                "Waterfall Ledge",
                vec![check(
                    game_world::hyrule::zora::river::get("[Mai] Waterfall Ledge Wall").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    //portal(DarkRuins), // need to make left/right system for portals, just ignore this for now
                    path(
                        HyruleField,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_hookshot()),
                        None,
                    ),
                ],
            ),
        ),
        (
            CuccoDungeonLedge,
            location(
                "Cucco Dungeon Ledge",
                vec![check(
                    game_world::hyrule::field::main::get("[Mai] Cucco Dungeon Big Rock").unwrap(),
                    Some(|p| p.has_titans_mitt()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleField),
                    path_free(CuccoDungeon),
                    portal_std(LoruleCastleField),
                ],
            ),
        ),
        (
            CuccoDungeon,
            location(
                "Cucco Treasure Dungeon",
                vec![check_free(
                    game_world::hyrule::field::main::get("Cucco Treasure Dungeon").unwrap(),
                )],
                vec![path_free(CuccoDungeonLedge)],
            ),
        ),
        (
            WitchHouse,
            location(
                "Witch's House",
                vec![
                    check_quest_free("Access Potion Shop", AccessPotionShop),
                    check(
                        game_world::hyrule::zora::river::get("[Mai] Inside Witch's House").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            EasternRuinsUpper,
            location(
                "Eastern Ruins Upper",
                vec![
                    check_free(
                        game_world::hyrule::eastern::ruins::get("Eastern Ruins Armos Chest")
                            .unwrap(),
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("Eastern Ruins Hookshot Chest")
                            .unwrap(),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::eastern::ruins::get("Eastern Ruins Merge Chest")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_tornado_rod() || p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                    ghost(HintGhostEasternRuinsPegs),
                ],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleField),
                    path(
                        EasternRuinsEastLedge,
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_tornado_rod()), // Armos boost
                        None,
                        None,
                    ),
                    path_free(EasternPalaceFoyer),
                    path_free(MergeDungeon),
                    path(WitchCave, Some(|p| p.has_bombs()), None, None, None, None),
                ],
            ),
        ),
        (
            EasternRuinsEastLedge,
            location(
                "Eastern Ruins East Ledge",
                vec![ghost(HintGhostEasternRuinsCave)],
                vec![
                    fast_travel_hyrule(),
                    path(EastRuinsBombCaveUpper, Some(|p| p.has_bombs()), None, None, None, None),
                    path(EasternRuinsUpper, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(HyruleField),
                ],
            ),
        ),
        (
            WitchCave,
            location(
                "Witch Cave",
                vec![],
                vec![path_free(EasternRuinsUpper), path_free(HyruleField)],
            ),
        ),
        (
            ZoraDomain,
            location(
                "Zora's Domain",
                vec![check(
                    game_world::hyrule::zora::river::get("Queen Oren").unwrap(),
                    Some(|p| p.has_smooth_gem() && (!p.is_rse() || p.has_sage_oren())),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(ZoraDomainArea)],
            ),
        ),
        (
            ZoraDomainArea,
            location(
                "Zora's Domain Area",
                vec![
                    check_quest(
                        "Shady Guy Trigger",
                        ShadyGuyTrigger,
                        Some(|p| !p.is_rse() || p.has_sage_oren()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::zora::river::get("Zora's Domain Ledge").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::zora::river::get("[Mai] Zora's Domain Water").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::zora::river::get("[Mai] Zora's Domain South Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostZorasDomain),
                    ghost(HintGhostWaterfallCave),
                ],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleField),
                    path_free(ZoraDomain),
                    path(KusDomainSouth, Some(|p| p.can_merge()), None, None, None, None),
                    path(
                        WaterfallCaveShallowWater,
                        Some(|p| p.has_flippers()),
                        None,
                        Some(|_| true), // Crow Boost
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            WaterfallCaveShallowWater,
            location(
                "Waterfall Cave Shallow Water",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(WaterfallCave),
                    path(HyruleField, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            WaterfallCave,
            location(
                "Waterfall Cave",
                vec![check_free(game_world::hyrule::zora::river::get("Waterfall Cave").unwrap())],
                vec![path_free(WaterfallCaveShallowWater)],
            ),
        ),
        (
            MergeDungeon,
            location(
                "Eastern Ruins Treasure Dungeon",
                vec![check(
                    game_world::hyrule::eastern::ruins::get("Eastern Ruins Treasure Dungeon")
                        .unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(EasternRuinsUpper)],
            ),
        ),
        (
            EastRuinsBombCaveUpper,
            location(
                "Eastern Ruins Bomb Cave Upper",
                vec![check(
                    game_world::hyrule::eastern::ruins::get("Eastern Ruins Cave").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path(
                        EastRuinsBombCaveLower,
                        Some(|p| p.can_merge()),
                        Some(|_| true), // It's not obvious but you can just walk
                        None,
                        None,
                        None,
                    ),
                    path_free(EasternRuinsUpper),
                ],
            ),
        ),
        (
            EastRuinsBombCaveLower,
            location("Eastern Ruins Bomb Cave Lower", vec![], vec![path_free(HyruleField)]),
        ),
        (
            HouseOfGalesIsland,
            location(
                "House of Gales Island",
                vec![
                    check(
                        game_world::hyrule::lake::hylia::get("[Mai] Island Tornado Tile").unwrap(),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostHouseOfGalesIsland),
                ],
                vec![
                    fast_travel_hyrule(),
                    path(
                        HyruleField,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None, // I guess you could water walk, but I'm not evil enough to include that
                    ),
                    path(HouseOfGalesFoyer, Some(|p| p.has_tornado_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            RossoHouse,
            location(
                "Rosso's House",
                vec![
                    check(
                        game_world::hyrule::lost::woods::get("Rosso").unwrap(),
                        Some(|p| {
                            if p.is_rse() {
                                p.has_sage_rosso()
                            } else {
                                p.has_pendant_of_courage()
                            }
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("Rosso Rocks").unwrap(),
                        Some(|p| {
                            p.has_power_glove()
                                && if p.is_rse() {
                                    p.has_sage_rosso()
                                } else {
                                    p.has_pendant_of_courage()
                                }
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(HyruleField),
                    path(SkullWoodsOverworld, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            RossoCave,
            location(
                "Rosso Cave",
                vec![check_free(game_world::hyrule::lost::woods::get("Rosso Cave").unwrap())],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            TornadoRodDungeon,
            location(
                "Zora's River Treasure Dungeon",
                vec![check(
                    game_world::hyrule::zora::river::get("Zora's River Treasure Dungeon").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            GraveyardLedgeHyrule,
            location(
                "Graveyard Ledge",
                vec![check(
                    game_world::dungeons::graveyards::hyrule::get("[Mai] Hyrule Graveyard Wall")
                        .unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleField),
                    path_free(GraveyardLedgeCave),
                    portal_std(GraveyardLedgeLorule),
                ],
            ),
        ),
        (
            GraveyardLedgeCave,
            location(
                "Graveyard Ledge Cave",
                vec![check_free(
                    game_world::dungeons::graveyards::hyrule::get("Graveyard Ledge Cave").unwrap(),
                )],
                vec![path_free(GraveyardLedgeHyrule)],
            ),
        ),
        (
            BlacksmithHouse,
            location(
                "Blacksmith's House (Hyrule)",
                vec![
                    check_free(game_world::hyrule::field::main::get("Blacksmith Table").unwrap()),
                    check(
                        game_world::hyrule::field::main::get("Blacksmith").unwrap(),
                        Some(|p| p.has_master_ore(2)),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest_free("Access Hyrule Blacksmith", AccessHyruleBlacksmith),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            BlacksmithCave,
            location(
                "Blacksmith Cave",
                vec![check_free(game_world::hyrule::field::main::get("Blacksmith Cave").unwrap())],
                vec![path_free(HyruleField)],
            ),
        ),
        // Hyrule Castle
        (
            HyruleCastleCourtyard,
            location(
                "Hyrule Castle Courtyard",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleCastleLeftRoom),
                    path_free(HyruleCastleRightRoom),
                    path(
                        HyruleCastleInterior,
                        Some(|p| !p.is_rse() || p.has_sage_impa()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        HyruleField,
                        Some(|p| p.has_master_sword() || p.swordless_mode()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HyruleCastleInterior,
            location(
                "Hyrule Castle Interior",
                vec![
                    check_free(
                        game_world::dungeons::hyrule::castle::get("Hyrule Castle Prize").unwrap(),
                    ),
                    check_quest_free("Zelda's Throne", ZeldasThrone),
                ],
                vec![path_free(HyruleCastleCourtyard), path_free(HyruleCastleRoof)],
            ),
        ),
        (
            HyruleCastleRightRoom,
            location("Hyrule Castle Right Room", vec![], vec![path_free(HyruleCastleCourtyard)]),
        ),
        (
            HyruleCastleLeftRoom,
            location(
                "Hyrule Castle Left Room",
                vec![check_free(
                    game_world::dungeons::hyrule::castle::get("Hyrule Castle West Wing").unwrap(),
                )],
                vec![path_free(HyruleCastleCourtyard)],
            ),
        ),
        (
            HyruleCastleRoof,
            location(
                "Hyrule Castle Roof",
                vec![check_free(
                    game_world::dungeons::hyrule::castle::get("Hyrule Castle Battlement").unwrap(),
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleField),
                    path_free(HyruleCastleCourtyard),
                    path_free(HyruleCastleInterior),
                    path(
                        HyruleCastleDungeon,
                        Some(|p| p.hc_is_open() && p.has_pendant_of_courage()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            LostWoods,
            location(
                "Lost Woods",
                vec![
                    check(
                        game_world::hyrule::lost::woods::get("Lost Woods Alcove").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| {
                            p.can_escape()
                                && (p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot()))
                        }),
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // Use Crow to escape
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("Lost Woods Big Rock Chest").unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // Use Crow to escape
                    ),
                    check_free(
                        game_world::hyrule::lost::woods::get("[Mai] Lost Woods Bush").unwrap(),
                    ),
                    check(
                        game_world::hyrule::lost::woods::get("[Mai] Lost Woods Rock").unwrap(),
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    fast_travel_hyrule(),
                    path_free(HyruleField),
                    path(
                        MasterSwordArea,
                        Some(|p| p.has_required_pendants()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            MasterSwordArea,
            location(
                "Master Sword Area",
                vec![check_free(
                    game_world::hyrule::lost::woods::get("Master Sword Pedestal").unwrap(),
                )],
                vec![fast_travel_hyrule(), path_free(LostWoods)],
            ),
        ),
        (
            FortuneTeller,
            location(
                "Fortune-Teller (Hyrule)",
                vec![check_free(game_world::hyrule::lost::woods::get("Fortune-Teller").unwrap())],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            KakarikoJailCell,
            location(
                "Kakariko Jail Cell",
                vec![check(
                    game_world::hyrule::kakariko::village::get("Kakariko Jail").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            WellUpper,
            location(
                "Kakariko Well Upper",
                vec![check_free(
                    game_world::hyrule::kakariko::village::get("Kakariko Well (Top)").unwrap(),
                )],
                vec![path_free(WellLower)],
            ),
        ),
        (
            WellLower,
            location(
                "Kakariko Well Lower",
                vec![check_free(
                    game_world::hyrule::kakariko::village::get("Kakariko Well (Bottom)").unwrap(),
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            StylishWomanHouse,
            location(
                "Stylish Woman's House",
                vec![
                    check_free(
                        game_world::hyrule::kakariko::village::get("Stylish Woman").unwrap(),
                    ),
                    check_quest_free("Open Stylish Woman's House", StylishWomansHouseOpen),
                ],
                vec![portal_std(LoruleCastleField), path_free(HyruleField)],
            ),
        ),
        (
            MilkBar,
            location(
                "Milk Bar",
                vec![check_quest_free("Access Milk Bar", AccessMilkBar)],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            BeeGuyHouse,
            location(
                "Bee Guy's House",
                vec![
                    check(
                        game_world::hyrule::kakariko::village::get("Bee Guy (1)").unwrap(),
                        Some(|p| p.has_bottle()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::hyrule::kakariko::village::get("Bee Guy (2)").unwrap(),
                        Some(|p| p.has_bottle() && p.has_gold_bee()),
                        None,
                        None,
                        None,
                        Some(|p| p.has_bottle() && p.has_net()),
                    ),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            KakarikoItemShop,
            location(
                "Kakariko Item Shop",
                vec![
                    check_free(
                        game_world::hyrule::kakariko::village::get("Kakariko Item Shop (1)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::hyrule::kakariko::village::get("Kakariko Item Shop (2)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::hyrule::kakariko::village::get("Kakariko Item Shop (3)")
                            .unwrap(),
                    ),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            LakesideItemShop,
            location(
                "Lakeside Item Shop",
                vec![
                    check_free(
                        game_world::hyrule::lake::hylia::get("Lakeside Item Shop (1)").unwrap(),
                    ),
                    check_free(
                        game_world::hyrule::lake::hylia::get("Lakeside Item Shop (2)").unwrap(),
                    ),
                    check_free(
                        game_world::hyrule::lake::hylia::get("Lakeside Item Shop (3)").unwrap(),
                    ),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            ItemSellerCave,
            location(
                "Runaway Item-Seller Cave",
                vec![check(
                    game_world::hyrule::southern::ruins::get("Runaway Item Seller").unwrap(),
                    Some(|p| p.has_scoot_fruit()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            FlippersDungeon,
            location(
                "Southern Ruins Treasure Dungeon",
                vec![check(
                    game_world::hyrule::southern::ruins::get("Southern Ruins Treasure Dungeon")
                        .unwrap(),
                    Some(|p| p.has_boomerang() && p.has_hookshot() && p.has_flippers()),
                    Some(|p| {
                        p.has_hookshot()
                            && p.has_flippers()
                            && (p.has_master_sword() || p.has_bombs())
                    }),
                    Some(|p| p.has_nice_bombs() || p.has_nice_ice_rod() || p.can_great_spin()),
                    None,
                    None,
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            SouthernRuinsBombCave,
            location(
                "Southern Ruins Bomb Cave",
                vec![check(
                    game_world::hyrule::southern::ruins::get("[Mai] Southern Ruins Bomb Cave")
                        .unwrap(),
                    Some(|p| p.has_flippers()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(HyruleField), path_free(SouthernRuinsPillars)],
            ),
        ),
        (
            SouthernRuinsPillars,
            location(
                "Southern Ruins Pillars",
                vec![check_free(
                    game_world::hyrule::southern::ruins::get("Southern Ruins Pillar Cave").unwrap(),
                )],
                vec![fast_travel_hyrule(), path_free(SouthernRuinsBombCave)],
            ),
        ),
        (
            LakeDarkCave,
            location(
                "Lake Hylia Dark Cave",
                vec![check(
                    game_world::hyrule::lake::hylia::get("Lake Hylia Dark Cave").unwrap(),
                    Some(|p| p.has_fire_source()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            IceRodCave,
            location(
                "Ice Rod Cave",
                vec![check_free(game_world::hyrule::lake::hylia::get("Ice Rod Cave").unwrap())],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            SanctuaryChurch,
            location(
                "Sanctuary Church",
                vec![],
                vec![
                    portal_std(LoruleSanctuaryCaveLower),
                    path(
                        HyruleField,
                        Some(|p| p.has_opened_sanctuary_doors()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            Sanctuary,
            location(
                "Sanctuary",
                vec![
                    check_free(
                        game_world::dungeons::graveyards::hyrule::get("[HS] Entrance").unwrap(),
                    ),
                    check(
                        game_world::dungeons::graveyards::hyrule::get("[HS] Lower Chest").unwrap(),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::hyrule::get("[HS] Upper Chest").unwrap(),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::hyrule::get("[HS] Ledge").unwrap(),
                        Some(|p| {
                            p.can_merge() && (p.has_lamp() || (p.has_fire_rod() && p.lampless()))
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest(
                        "Open Sanctuary Doors",
                        OpenSanctuaryDoors,
                        Some(|p| {
                            (p.has_lamp() || (p.has_fire_rod() && p.lampless()))
                                && p.can_attack()
                                && p.has_sanctuary_key()
                        }),
                        Some(|p| p.has_lamp() && p.has_sanctuary_key()),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(HyruleField),
                    path(
                        SanctuaryChurch,
                        Some(|p| {
                            (p.has_lamp() || (p.has_fire_rod() && p.lampless()))
                                && p.can_attack()
                                && p.has_sanctuary_key()
                        }),
                        Some(|p| p.has_lamp() && p.has_sanctuary_key()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            MoldormCave,
            location(
                "Moldorm Cave",
                vec![],
                vec![
                    path_free(HyruleField),
                    path(MoldormCaveTop, Some(|p| p.has_titans_mitt()), None, None, None, None),
                    path_free(DeathMountainBase),
                ],
            ),
        ),
        (
            MoldormCaveTop,
            location(
                "Moldorm Cave Top",
                vec![],
                vec![
                    path_free(MoldormLedge),
                    path(MoldormCave, Some(|p| p.has_titans_mitt()), None, None, None, None),
                ],
            ),
        ),
        (
            MoldormLedge,
            location(
                "Moldorm Ledge",
                vec![check(
                    game_world::hyrule::lost::woods::get("[Mai] Moldorm Ledge").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![fast_travel_hyrule(), path_free(MoldormCaveTop), path_free(HyruleField)],
            ),
        ),
        (
            DeathMountainBase,
            location(
                "Death Mountain Base",
                vec![check(
                    game_world::hyrule::death::mountain::get("[Mai] Death Mountain Base Rock")
                        .unwrap(),
                    Some(|p| p.has_power_glove()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(MoldormCave),
                    path(
                        DeathBombCave,
                        Some(|p| p.can_merge() && p.has_bombs()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(DeathWeatherVaneCaveLeft),
                    path(DeathFairyCave, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(DonkeyCaveLower),
                    portal_std(LoruleDeathWest),
                ],
            ),
        ),
        (
            DeathBombCave,
            location(
                "Death Mountain Blocked Cave",
                vec![check_free(
                    game_world::hyrule::death::mountain::get("Death Mountain Blocked Cave")
                        .unwrap(),
                )],
                vec![path_free(DeathMountainBase)],
            ),
        ),
        (
            DeathWeatherVaneCaveLeft,
            location(
                "Death Mountain Cave Left of Weather Vane",
                vec![check_free(
                    game_world::hyrule::death::mountain::get("Death Mountain Open Cave").unwrap(),
                )],
                vec![path_free(DeathMountainBase)],
            ),
        ),
        (
            DeathFairyCave,
            location(
                "Death Mountain Fairy Cave",
                vec![check(
                    game_world::hyrule::death::mountain::get("Death Mountain Fairy Cave").unwrap(),
                    Some(|p| p.has_hammer() || p.has_bombs()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(DeathMountainBase)],
            ),
        ),
        (
            DonkeyCaveLower,
            location(
                "Donkey Cave Lower",
                vec![],
                vec![
                    path_free(DeathMountainBase),
                    path(
                        DonkeyCaveUpper,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.can_get_potion() || p.has_mail()),
                        None,
                    ),
                ],
            ),
        ),
        (
            DonkeyCaveUpper,
            location(
                "Donkey Cave Upper",
                vec![check(
                    game_world::hyrule::death::mountain::get("Donkey Cave Pegs").unwrap(),
                    Some(|p| p.has_hammer()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path(
                        DonkeyCaveLower,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.can_get_potion() || p.has_mail()),
                        None,
                    ),
                    path_free(DeathWestLedge),
                    path_free(DeathSecondFloor),
                ],
            ),
        ),
        (
            DeathWestLedge,
            location(
                "Death Mountain West Ledge",
                vec![
                    check_free(
                        game_world::hyrule::death::mountain::get("Death Mountain West Ledge")
                            .unwrap(),
                    ),
                    check(
                        game_world::hyrule::death::mountain::get("[Mai] Death Mountain West Ledge")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![fast_travel_hyrule(), path_free(DonkeyCaveUpper), path_free(DeathSecondFloor)],
            ),
        ),
        (
            DeathSecondFloor,
            location(
                "Death Mountain Second Floor",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(DonkeyCaveUpper),
                    path_free(AmidaCaveLower),
                    path_free(DeathMountainBase),
                    path(
                        DeathFairyCave,
                        None,
                        None,
                        Some(|p| {
                            p.has_fire_rod()
                                || p.has_nice_bombs()
                                || p.has_boomerang()
                                || p.has_hookshot()
                        }),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                ],
            ),
        ),
        (
            AmidaCaveLower,
            location(
                "Amida Cave Lower",
                vec![],
                vec![
                    path_free(DeathSecondFloor),
                    path_free(DeathThirdFloor),
                    path(AmidaCaveUpper, None, None, Some(|p| p.has_boots()), None, None),
                ],
            ),
        ),
        (
            DeathThirdFloor,
            location(
                "Death Mountain Third Floor",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(AmidaCaveLower),
                    path_free(AmidaCaveUpper),
                    path_free(DeathSecondFloor),
                    path(
                        DeathWestLedge,
                        None,
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            AmidaCaveUpper,
            location(
                "Amida Cave Upper",
                vec![check_free(
                    game_world::hyrule::death::mountain::get("Death Mountain West Highest Cave")
                        .unwrap(),
                )],
                vec![
                    path_free(AmidaCaveLower),
                    path_free(DeathThirdFloor),
                    path_free(DeathTopLeftLedge),
                ],
            ),
        ),
        (
            DeathTopLeftLedge,
            location(
                "Death Mountain West Top Left Ledge",
                vec![ghost(HintGhostSpectacleRock)],
                vec![
                    fast_travel_hyrule(),
                    path_free(AmidaCaveUpper),
                    path_free(DeathThirdFloor),
                    path(
                        SpectacleRock,
                        Some(|p| p.can_merge()),
                        Some(|_| true), // noobs don't realize you can just jump here
                        None,
                        None,
                        None,
                    ),
                    path(DeathMountainWestTop, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            SpectacleRock,
            location(
                "Spectacle Rock",
                vec![check_free(
                    game_world::hyrule::death::mountain::get("Spectacle Rock").unwrap(),
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(DeathThirdFloor),
                    path_free(SpectacleRockCaveLeft),
                ],
            ),
        ),
        (
            SpectacleRockCaveLeft,
            location(
                "Spectacle Rock Cave Left",
                vec![],
                vec![path_free(SpectacleRock), path_free(SpectacleRockCaveRight)],
            ),
        ),
        (
            SpectacleRockCaveRight,
            location("Spectacle Rock Cave Right", vec![], vec![path_free(DeathMountainWestTop)]),
        ),
        (
            DeathMountainWestTop,
            location(
                "Death Mountain West Top",
                vec![ghost(HintGhostTowerOfHeraOutside)],
                vec![
                    fast_travel_hyrule(),
                    path_free(SpectacleRockCaveRight),
                    path(TowerOfHeraFoyer, Some(|p| p.has_hammer()), None, None, None, None),
                    path(DeathTopLeftLedge, Some(|p| p.can_merge()), None, None, None, None),
                    path(
                        SpectacleRock,
                        Some(|p| p.can_merge()),
                        Some(|_| true), // noobs don't realize you can just jump here
                        None,
                        None,
                        None,
                    ),
                    path_free(DeathThirdFloor),
                    path(DeathMountainEastTop, Some(|p| p.has_hookshot()), None, None, None, None),
                ],
            ),
        ),
        (
            DeathMountainEastTop,
            location(
                "Death Mountain East Top",
                vec![
                    check(
                        game_world::hyrule::death::mountain::get("[Mai] Outside Hookshot Dungeon")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostFloatingIsland),
                    ghost(HintGhostFireCave),
                ],
                vec![
                    fast_travel_hyrule(),
                    path(DeathMountainWestTop, Some(|p| p.has_hookshot()), None, None, None, None),
                    path_free(FireCaveTop),
                    path_free(HookshotDungeon),
                    path(
                        BoulderingLedgeRight,
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_boots()),
                        None,
                        None,
                    ),
                    path(
                        RossosOreMine,
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_boots()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HookshotDungeon,
            location(
                "Death Mountain Treasure Dungeon",
                vec![check(
                    game_world::hyrule::death::mountain::get("Death Mountain Treasure Dungeon")
                        .unwrap(),
                    Some(|p| p.can_merge() && p.has_hookshot()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(DeathMountainEastTop)],
            ),
        ),
        (
            FireCaveTop,
            location(
                "Fire Cave Top",
                vec![],
                vec![path_free(DeathMountainEastTop), path_free(FireCaveCenter)],
            ),
        ),
        (
            FireCaveCenter,
            location(
                "Fire Cave Center",
                vec![check(
                    game_world::hyrule::death::mountain::get("Fire Cave Pillar").unwrap(),
                    Some(|p| p.can_merge() && p.has_hammer()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path(FireCaveMiddle, Some(|p| p.can_merge()), None, None, None, None),
                    path(FireCaveBottom, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            FireCaveMiddle,
            location(
                "Fire Cave Middle",
                vec![],
                vec![
                    path(FireCaveCenter, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(BoulderingLedgeLeft),
                    path_free(BoulderingLedgeRight),
                ],
            ),
        ),
        (
            FireCaveBottom,
            location(
                "Fire Cave Bottom",
                vec![],
                vec![path_free(RossosOreMine), path_free(FireCaveTop)],
            ),
        ),
        (
            BoulderingLedgeLeft,
            location(
                "Bouldering Guy Left Ledge",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(FireCaveMiddle),
                    path(BoulderingLedgeRight, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(BoulderingLedgeBottom),
                    path(RossosOreMine, None, None, Some(|p| p.has_nice_bombs()), None, None),
                ],
            ),
        ),
        (
            BoulderingLedgeBottom,
            location(
                "Bouldering Guy Bottom Ledge",
                vec![check(
                    game_world::hyrule::death::mountain::get("[Mai] Death Mountain East Ledge")
                        .unwrap(),
                    Some(|p| p.has_power_glove()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![fast_travel_hyrule(), path_free(FireCaveMiddle)],
            ),
        ),
        (
            BoulderingLedgeRight,
            location(
                "Bouldering Guy Right Ledge",
                vec![
                    check(
                        game_world::hyrule::death::mountain::get("Bouldering Guy").unwrap(),
                        Some(|p| {
                            p.has_premium_milk()
                                || (p.has_letter_in_a_bottle() && p.can_access_milk_bar())
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest(
                        "Bouldering Guy's Trash",
                        Bottle05,
                        Some(|p| {
                            p.has_premium_milk()
                                || (p.has_letter_in_a_bottle() && p.can_access_milk_bar())
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    fast_travel_hyrule(),
                    path_free(BoulderingLedgeBottom),
                    path(BoulderingLedgeLeft, Some(|p| p.can_merge()), None, None, None, None),
                    path(RossosOreMine, None, None, Some(|p| p.has_nice_bombs()), None, None),
                ],
            ),
        ),
        (
            RossosOreMine,
            location(
                "Rosso's Ore Mine",
                vec![check(
                    game_world::hyrule::death::mountain::get("[Mai] Rosso's Ore Mine Rock")
                        .unwrap(),
                    Some(|p| p.has_power_glove()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(FireCaveBottom),
                    portal_std(RossosOreMineLorule),
                ],
            ),
        ),
        (
            FloatingIslandHyrule,
            location(
                "Hyrule Floating Island",
                vec![check_free(
                    game_world::hyrule::death::mountain::get("Floating Island").unwrap(),
                )],
                vec![fast_travel_hyrule(), portal_std(FloatingIslandLorule)],
            ),
        ),
    ])
}

/// Lorule
fn lorule() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            LoruleBellTravel,
            location(
                "Lorule Bell Travel",
                vec![],
                vec![
                    path_free(LoruleCastleField),
                    path_free(SkullWoodsOverworld),
                    path_free(MiseryMire),
                    path_free(SwampPalaceOutside),
                    path_free(LoruleDeathWest),
                    path_free(LoruleGraveyard),
                    path_free(RossosOreMineLorule),
                    path_free(TurtleRockWeatherVane),
                    path_free(LoruleDeathEastTop),
                    path_free(DarkRuins),
                ],
            ),
        ),
        (
            LoruleCastleField,
            location(
                "Lorule Castle Field",
                vec![
                    check_free(
                        game_world::lorule::field::main::get("Rupee Rush (Lorule)").unwrap(),
                    ),
                    check_free(game_world::lorule::field::main::get("Octoball Derby").unwrap()),
                    check_quest_free("Access Hilda Barrier", AccessLoruleCastleField),
                    check_free(game_world::lorule::field::main::get("Fortune's Choice").unwrap()),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule Castle Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule Castle Tree").unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Thieves' Town Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Near Lorule Fortune-Teller")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule Blacksmith Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule Rupee Rush Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Octoball Derby Skull").unwrap(),
                        Some(|p| p.can_destroy_skull()),
                        Some(|_| true), // throw bush at skull
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Vacant House Big Rock")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Behind Vacant House").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule S Ruins Pillars")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule S Ruins Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule S Ruins Water").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Thieves' Town Tree").unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostFortuneTellerLorule),
                    ghost(HintGhostRupeeRushLorule),
                    ghost(HintGhostGreatRupeeFairy),
                    ghost(HintGhostOctoballDerby),
                    ghost(HintGhostVacantHouse),
                    ghost(HintGhostSwampPalaceOutsideLeft),
                    ghost(HintGhostSwampPalaceOutsideRight),
                ],
                vec![
                    fast_travel_lorule(),
                    path(
                        GreatRupeeFairyCave,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(LoruleBlacksmith),
                    path_free(BootsDungeon),
                    path_free(VacantHouseBottom),
                    path(
                        VacantHouseTop,
                        Some(|p| p.has_bombs()),
                        Some(|p| p.has_bomb_flower()),
                        None,
                        None,
                        None,
                    ),
                    path_free(ThiefGirlCave),
                    path(
                        SwampCave,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        Some(|p| p.has_stamina_scroll() && p.has_tornado_rod()),
                        Some(|_| true), // Bee Boosting
                    ),
                    path(BigBombCave, Some(|p| p.has_bomb_flower()), None, None, None, None),
                    path(
                        SwampPalaceOutside,
                        Some(|p| p.has_hookshot()), // cannot consider flippers as water may be drained
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(ThievesHideoutB1),
                    path(LoruleCastle1F, Some(|p| p.has_lc_requirement()), None, None, None, None),
                    portal_std(StylishWomanHouse),
                    path_free(BigBombFlowerShop),
                    path(
                        BigBombFlowerField,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                    ),
                    portal_std(CuccoDungeonLedge),
                    path_free(ThievesTownItemShop),
                    path_free(VeteranThiefsHouse),
                    path_free(FortunesChoiceLorule),
                ],
            ),
        ),
        (
            VeteranThiefsHouse,
            location(
                "Veteran Thief's House",
                vec![ghost(HintGhostVeteranThief)],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            FortunesChoiceLorule,
            location(
                "Fortune's Choice (Lorule)",
                vec![ghost(HintGhostFortunesChoice)],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            ThievesTownItemShop,
            location(
                "Thieves' Town Item Shop",
                vec![
                    check_free(
                        game_world::lorule::field::main::get("Thieves' Town Item Shop (1)")
                            .unwrap(),
                    ),
                    check_unreachable(
                        game_world::lorule::field::main::get("Thieves' Town Item Shop (2)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::lorule::field::main::get("Thieves' Town Item Shop (3)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::lorule::field::main::get("Thieves' Town Item Shop (4)")
                            .unwrap(),
                    ),
                ],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            BigBombFlowerShop,
            location(
                "Big Bomb Flower Shop",
                vec![],
                vec![path_free(LoruleCastleField), path_free(BigBombFlowerField)],
            ),
        ),
        (
            BigBombFlowerField,
            location(
                "Big Bomb Flower Field",
                vec![
                    check_quest_free("Obtain Big Bomb Flower", BigBombFlower),
                    check_free(
                        game_world::lorule::field::main::get("[Mai] Big Bomb Flower Grass")
                            .unwrap(),
                    ),
                ],
                vec![
                    fast_travel_lorule(),
                    path_free(BigBombFlowerShop),
                    path(LoruleCastleField, Some(|p| p.has_bomb_flower()), None, None, None, None),
                ],
            ),
        ),
        (
            LoruleGraveyard,
            location(
                "Lorule Graveyard",
                vec![
                    check_free(
                        game_world::dungeons::graveyards::lorule::get("Graveyard Peninsula")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::graveyards::lorule::get(
                            "[Mai] Lorule Graveyard Big Rock",
                        )
                        .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::lorule::get(
                            "[Mai] Lorule Graveyard Wall",
                        )
                        .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::lorule::get(
                            "[Mai] Lorule Graveyard Tree",
                        )
                        .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostLoruleGraveyard),
                ],
                vec![
                    fast_travel_lorule(),
                    path_free(LoruleSanctuaryCaveLower),
                    path(LoruleSanctuary, Some(|p| p.has_titans_mitt()), None, None, None, None),
                    path(
                        DarkRuins,
                        None,
                        None,
                        Some(|p| (p.has_fire_rod() || p.has_nice_bombs()) && p.has_flippers()),
                        Some(|p| {
                            (p.has_fire_rod() || p.has_nice_bombs())
                                && (p.has_flippers() || p.has_hookshot())
                        }), // Hookshot trick
                        Some(|p| p.has_flippers() || p.has_hookshot()), // Bee Boost
                    ),
                    path(GraveyardLedgeLorule, Some(|p| p.has_bombs()), None, None, None, None),
                ],
            ),
        ),
        (
            GraveyardLedgeLorule,
            location(
                "Graveyard Ledge Lorule",
                vec![],
                vec![
                    fast_travel_lorule(),
                    portal_std(GraveyardLedgeHyrule),
                    path_free(LoruleGraveyard),
                ],
            ),
        ),
        (
            LoruleSanctuary,
            location(
                "Lorule Sanctuary",
                vec![
                    check(
                        game_world::dungeons::graveyards::lorule::get("[LS] Entrance Chest")
                            .unwrap(),
                        Some(|p| p.has_lamp() || p.lampless()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::lorule::get("[LS] Lower Chest").unwrap(),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::lorule::get("[LS] Upper Chest").unwrap(),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::graveyards::lorule::get("[LS] Ledge").unwrap(),
                        Some(|p| {
                            p.can_merge() && (p.has_lamp() || (p.has_fire_rod() && p.lampless()))
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(LoruleGraveyard),
                    path(
                        LoruleSanctuaryCaveUpper,
                        Some(|p| {
                            (p.has_lamp() || (p.has_fire_rod() && p.lampless()))
                                && p.can_attack()
                                && p.has_lorule_sanctuary_key()
                        }),
                        Some(|p| p.has_lamp() && p.has_lorule_sanctuary_key()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            LoruleSanctuaryCaveLower,
            location(
                "Philosopher's Cave Lower",
                vec![],
                vec![portal_std(SanctuaryChurch), path_free(LoruleGraveyard)],
            ),
        ),
        (
            LoruleSanctuaryCaveUpper,
            location(
                "Philosopher's Cave Upper",
                vec![check(
                    game_world::dungeons::graveyards::lorule::get("Philosopher's Cave").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(LoruleSanctuary), path_free(LoruleSanctuaryCaveLower)],
            ),
        ),
        (
            GreatRupeeFairyCave,
            location(
                "Great Rupee Fairy Cave",
                vec![check(
                    game_world::lorule::field::main::get("Great Rupee Fairy").unwrap(),
                    Some(|p| p.has_rupees(4000)), // Actual requirement is 3000 but higher threshold helps prevent rupee grinds
                    None,
                    None,
                    None,
                    Some(|_| true), // suffer lol
                )],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            LoruleBlacksmith,
            location(
                "Lorule Blacksmith",
                vec![check(
                    game_world::lorule::field::main::get("Blacksmith (Lorule)").unwrap(),
                    Some(|p| {
                        p.has_master_ore(4)
                            && p.can_access_hyrule_blacksmith()
                            && p.can_access_lorule_castle_field()
                    }),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            BootsDungeon,
            location(
                "Lorule Field Treasure Dungeon",
                vec![check(
                    game_world::lorule::field::main::get("Lorule Field Treasure Dungeon").unwrap(),
                    Some(|p| p.has_boots()),
                    Some(|p| p.has_master_sword() || p.has_bombs() || p.has_boomerang()), // we're not set up for Nice Ice Rod or Nice Bow yet...
                    None,
                    None,
                    None,
                )],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            VacantHouseBottom,
            location("Vacant House (Bottom)", vec![], vec![path_free(LoruleCastleField)]),
        ),
        (
            VacantHouseTop,
            location(
                "Vacant House (Top)",
                vec![check_free(game_world::lorule::field::main::get("Vacant House").unwrap())],
                vec![path(
                    LoruleCastleField,
                    Some(|p| p.has_bombs()),
                    Some(|p| p.has_bomb_flower()),
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            ThiefGirlCave,
            location(
                "Thief Girl",
                vec![check(
                    game_world::lorule::field::main::get("Thief Girl").unwrap(),
                    Some(|p| p.has_sage_osfala()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            SwampCave,
            location(
                "Swamp Cave",
                vec![
                    check_free(game_world::lorule::field::main::get("Swamp Cave (Left)").unwrap()),
                    check_free(
                        game_world::lorule::field::main::get("Swamp Cave (Middle)").unwrap(),
                    ),
                    check_free(game_world::lorule::field::main::get("Swamp Cave (Right)").unwrap()),
                ],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            BigBombCave,
            location(
                "Haunted Grove Big Bomb Cave",
                vec![check_free(
                    game_world::lorule::field::main::get("Big Bomb Flower Cave").unwrap(),
                )],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            HauntedGroveLedge,
            location(
                "Haunted Grove Upper Ledge",
                vec![
                    check(
                        game_world::lorule::field::main::get("Lorule Field Hookshot Chest")
                            .unwrap(),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::field::main::get("[Mai] Lorule Haunted Grove Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![fast_travel_lorule(), path_free(LoruleCastleField), portal_std(HyruleField)],
            ),
        ),
        // Desert / Misery Mire
        (
            Desert,
            location(
                "Desert",
                vec![
                    check(
                        game_world::hyrule::desert::mystery::get("[Mai] Buried in the Desert")
                            .unwrap(),
                        Some(|p| p.has_sand_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostDesertEast),
                ],
                vec![
                    fast_travel_hyrule(),
                    portal_std(MiseryMire),
                    path(
                        MiseryMireLedge, // todo portal-ify
                        Some(|p| p.can_merge() && p.has_bombs() && p.has_sand_rod()),
                        Some(|p| p.can_merge() && p.has_bombs() && p.has_stamina_scroll()),
                        Some(|p| {
                            p.can_merge()
                                && (p.has_nice_bombs() || (p.has_fire_rod() && p.has_bombs()))
                        }),
                        Some(|p| p.can_merge() && p.has_bombs()), // Vulture Boost
                        None,
                    ),
                    path(DesertCenterLedge, Some(|p| p.has_sand_rod()), None, None, None, None),
                    path(
                        DesertSouthWestLedge,
                        None,
                        Some(|p| p.has_stamina_scroll()),
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        Some(|_| true), // vulture boost
                        None,
                    ),
                    path(
                        DesertPalaceWeatherVane,
                        None,
                        None,
                        Some(|_| true), // vulture clip
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DesertCenterLedge,
            location(
                "Desert Center Ledge",
                vec![ghost(HintGhostDesertCenter)],
                vec![path_free(Desert), portal_std(MiseryMireBridge)],
            ),
        ),
        (
            DesertSouthWestLedge,
            location(
                "Desert South West Ledge",
                vec![ghost(HintGhostDesertSouthWest)],
                vec![
                    fast_travel_hyrule(),
                    path_free(Desert),
                    portal_std(MiseryMireBridge),
                    path(
                        DesertPalaceWeatherVane,
                        Some(|p| p.has_sand_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DesertPalaceWeatherVane,
            location(
                "Desert Palace Weather Vane",
                vec![check(
                    game_world::hyrule::desert::mystery::get("[Mai] Buried near Desert Palace")
                        .unwrap(),
                    Some(|p| p.has_sand_rod()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    path_free(Desert),
                    path(DesertPalaceFoyer, Some(|p| p.has_sand_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            MiseryMire,
            location(
                "Misery Mire",
                vec![
                    check(
                        game_world::lorule::misery::mire::get("[Mai] Misery Mire Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::misery::mire::get("[Mai] Misery Mire Water").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::misery::mire::get("[Mai] Misery Mire Big Rock")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostMiseryMireLedge),
                    ghost(HintGhostMiseryMireBridge),
                ],
                vec![
                    fast_travel_lorule(),
                    path_free(SandRodDungeon),
                    portal_std(Desert),
                    path(
                        MiseryMireOoB,
                        None,
                        None,
                        None,
                        Some(|p| p.has_nice_bombs()), // double lemon boost
                        Some(|p| p.has_bombs()),      // awful version
                    ),
                    path(
                        MiseryMireBridge,
                        None,
                        None,
                        None,
                        Some(|p| p.has_ice_rod() && p.has_tornado_rod()),
                        None,
                    ),
                    path(
                        MiseryMireLedge,
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_nice_bombs() || p.has_fire_rod())),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            MiseryMireBridge,
            location(
                "Misery Mire Bridge",
                vec![],
                vec![
                    fast_travel_lorule(),
                    path_free(MiseryMire),
                    portal_std(DesertCenterLedge),
                    portal_std(DesertSouthWestLedge),
                    path(
                        MiseryMireOoB,
                        None,
                        None,
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        Some(|p| (p.has_hookshot() || p.has_boomerang()) && p.has_tornado_rod()), // portal clip
                    ),
                ],
            ),
        ),
        (
            MiseryMireOoB,
            location(
                "Misery Mire Out of Bounds",
                vec![],
                vec![
                    fast_travel_lorule(),
                    path_free(MiseryMire),
                    path_free(MiseryMireBridge),
                    portal_std(DesertZaganagaLedge),
                    path_free(ZaganagasArena),
                    path(MiseryMireRewardBasket, None, None, None, Some(|p| p.has_boots()), None),
                ],
            ),
        ),
        (
            SandRodDungeon,
            location(
                "Misery Mire Treasure Dungeon",
                vec![check(
                    game_world::lorule::misery::mire::get("Misery Mire Treasure Dungeon").unwrap(),
                    Some(|p| p.has_sand_rod() && p.has_tornado_rod()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(MiseryMire)],
            ),
        ),
        (
            MiseryMireLedge,
            location(
                "Misery Mire Ledge",
                vec![check_free(
                    game_world::lorule::misery::mire::get("Misery Mire Ledge").unwrap(),
                )],
                vec![fast_travel_lorule(), path_free(MiseryMire)],
            ),
        ),
        // Lorule Lake Area
        (
            LoruleLakeEast,
            location(
                "Lorule Lake East",
                vec![
                    check(
                        game_world::lorule::lake::lorule::get("[Mai] Lorule Lake SE Wall").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::lake::lorule::get("[Mai] Lorule Lake Skull").unwrap(),
                        Some(|p| p.can_merge() && p.can_destroy_skull()),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    fast_travel_lorule(),
                    portal_std(HyruleField),
                    path(
                        LoruleLakeWater,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        None,
                    ),
                    path(
                        DarkRuins,
                        None,
                        None,
                        Some(|p| p.has_nice_bombs() && p.has_stamina_scroll()),
                        None,
                        Some(|p| p.has_stamina_scroll()), // bee boost
                    ),
                ],
            ),
        ),
        (
            LoruleLakeNorthWest,
            location(
                "Lorule Lake North West",
                vec![
                    check_quest(
                        "Turtle (wall)",
                        TurtleWall,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(game_world::lorule::lake::lorule::get("Lorule Lake Chest").unwrap()),
                    check(
                        game_world::lorule::lake::lorule::get("[Mai] Lorule Lake West Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostTurtleWall),
                ],
                vec![
                    fast_travel_lorule(),
                    portal_std(HyruleField),
                    path_free(LoruleLakesideItemShop),
                    path(LoruleLakeSouthWest, Some(|p| p.can_merge()), None, None, None, None),
                    path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            LoruleLakeSouthWest,
            location(
                "Lorule Lake South West",
                vec![
                    check_quest_free("Turtle (flipped)", TurtleFlipped),
                    check(
                        game_world::lorule::lake::lorule::get("[Mai] Lorule Lake Big Rock")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    fast_travel_lorule(),
                    path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            LoruleLakesideItemShop,
            location(
                "Lorule Lakeside Item Shop",
                vec![
                    check_free(
                        game_world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (1)")
                            .unwrap(),
                    ),
                    check_unreachable(
                        game_world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (2)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (3)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (4)")
                            .unwrap(),
                    ),
                ],
                vec![path_free(LoruleLakeNorthWest)],
            ),
        ),
        // This location assumes the player is already swimming, real or fake
        (
            LoruleLakeWater,
            location(
                "Lorule Lake Water",
                vec![
                    check_quest(
                        "Turtle (under attack)",
                        TurtleAttacked,
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::lorule::lake::lorule::get("[Mai] Lorule Lake Water").unwrap(),
                    ),
                    ghost(HintGhostTurtleBullied),
                ],
                vec![
                    fast_travel_lorule(),
                    path_free(LoruleLakeNorthWest),
                    path_free(LoruleLakeSouthWest),
                    path_free(LoruleLakeEast),
                    path(
                        TurtleRockWeatherVane,
                        Some(|p| p.can_rescue_turtles()),
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
                    path(
                        TurtleRockFrontDoor,
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            TurtleRockWeatherVane,
            location(
                "Turtle Rock Weather Vane",
                vec![ghost(HintGhostTurtleRockOutside)],
                vec![
                    fast_travel_lorule(),
                    path(
                        TurtleRockFrontDoor,
                        Some(|p| p.has_ice_rod() && p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockFrontDoor,
            location(
                "Turtle Rock Front Door",
                vec![],
                vec![
                    fast_travel_lorule(),
                    path_free(TurtleRockFoyer),
                    path(
                        TurtleRockWeatherVane,
                        Some(|p| p.has_ice_rod() && p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        // Dark Ruins Area
        (
            DarkRuins,
            location(
                "Dark Ruins",
                vec![
                    check_free(
                        game_world::lorule::dark::ruins::get("Dark Ruins Lakeview Chest").unwrap(),
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Dark Ruins Waterfall").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()),
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Dark Maze Entrance Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Atop Dark Ruins Rocks")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Dark Ruins West Tree").unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Dark Ruins East Tree").unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Dark Ruins South Area Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostDarkRuinsNorth),
                ],
                vec![
                    fast_travel_lorule(),
                    portal_std(HyruleField),
                    path_free(DarkMazeEntrance),
                    path(KusDomainSouth, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(DarkRuinsShallowWater),
                    path(
                        LoruleLakeWater,
                        None,
                        None,
                        Some(|p| p.has_flippers() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())), // fake flipper
                        Some(|p| p.has_boots()), // Bee boost
                    ),
                    path(
                        LoruleLakeEast,
                        None,
                        None,
                        Some(|p| {
                            p.has_stamina_scroll() && (p.has_fire_rod() || p.has_nice_bombs())
                        }), // long merge
                        None,
                        Some(|p| p.has_stamina_scroll()), // Bee Boost
                    ),
                ],
            ),
        ),
        (
            DarkMazeEntrance,
            location(
                "Dark Maze Entrance",
                vec![check(
                    game_world::lorule::dark::ruins::get("Dark Maze Chest").unwrap(),
                    Some(|p| p.can_merge() || p.has_sage_gulley()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path_free(DarkRuins),
                    path(
                        DarkMazeHalfway,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        Some(|_| true), // scuffed sneak
                    ),
                    path(
                        DarkPalaceWeatherVane,
                        Some(|p| p.has_sage_gulley()),
                        None,
                        None, // No situation where Dark Maze Skip is required, items required can break skulls and merge is required anyway
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkMazeHalfway,
            location(
                "Dark Maze Halfway",
                vec![
                    check_free(game_world::lorule::dark::ruins::get("Dark Maze Ledge").unwrap()),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Dark Maze Center Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostDarkMaze),
                ],
                vec![
                    path(
                        DarkMazeEntrance,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        Some(|_| true),
                    ),
                    path(
                        DarkPalaceWeatherVane,
                        Some(|p| p.can_destroy_skull() && (p.can_merge() || p.has_sage_gulley())),
                        None,
                        None, // Dark Maze Skip implies skulls can be broken, no logical benefit
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceWeatherVane,
            location(
                "Dark Ruins Weather Vane",
                vec![ghost(HintGhostDarkPalaceOutside)],
                vec![
                    path(
                        DarkMazeEntrance,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        DarkMazeHalfway,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(DarkPalaceFoyer, Some(|p| p.has_bombs()), None, None, None, None),
                ],
            ),
        ),
        (
            DarkRuinsShallowWater,
            location(
                "Dark Ruins Shallow Water",
                vec![],
                vec![
                    fast_travel_lorule(),
                    // todo figure out waterfall portal
                    path(
                        HinoxCaveWater,
                        Some(|p| p.can_merge() && p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(HinoxCaveShallowWater, Some(|p| p.can_merge()), None, None, None, None),
                    path(DarkRuins, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            KusDomainSouth,
            location(
                "Ku's Domain South",
                vec![check(
                    game_world::lorule::dark::ruins::get("[Mai] Ku's Domain Grass").unwrap(),
                    Some(|p| p.can_merge() && p.can_cut_grass()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_lorule(),
                    portal_std(ZoraDomainArea),
                    path(
                        HinoxCaveWater,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots()), // Crow boost fake flippers
                        None,
                    ),
                    path(
                        HinoxCaveShallowWater,
                        Some(|p| p.has_flippers()),
                        None,
                        Some(|_| true), // Crow boost
                        None,
                        None,
                    ),
                    path(
                        DarkRuins,
                        Some(|p| p.can_merge()),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                    ),
                    path(KusDomain, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            KusDomain,
            location(
                "Ku's Domain",
                vec![
                    check(
                        game_world::lorule::dark::ruins::get("Ku's Domain Fight").unwrap(),
                        Some(|p| {
                            p.has_bow()
                                || p.has_bombs()
                                || p.can_great_spin()
                                || p.has_nice_ice_rod()
                                || p.has_nice_hookshot()
                        }),
                        Some(|p| p.has_master_sword() || (p.has_sword() && p.has_power_glove())),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::dark::ruins::get("[Mai] Ku's Domain Water").unwrap(),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![fast_travel_lorule(), path_free(KusDomainSouth)],
            ),
        ),
        (
            HinoxCaveWater,
            location(
                "Hinox Cave Water",
                vec![
                    // This location assumes the player is already swimming, real or fake
                    check_free(
                        game_world::lorule::dark::ruins::get("[Mai] Outside Hinox Cave").unwrap(),
                    ),
                ],
                vec![path_free(HinoxCaveShallowWater)],
            ),
        ),
        (
            HinoxCaveShallowWater,
            location(
                "Hinox Cave Shallow Water",
                vec![],
                vec![
                    fast_travel_lorule(),
                    path_free(HinoxCave),
                    path(HinoxCaveWater, Some(|p| p.has_flippers()), None, None, None, None),
                    path(DarkRuinsShallowWater, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            HinoxCave,
            location(
                "Hinox Cave",
                vec![
                    check_free(game_world::lorule::dark::ruins::get("Hinox (1)").unwrap()),
                    check_free(game_world::lorule::dark::ruins::get("Hinox (2)").unwrap()),
                    check_free(game_world::lorule::dark::ruins::get("Hinox (3)").unwrap()),
                    check_free(game_world::lorule::dark::ruins::get("Hinox (4)").unwrap()),
                    check_free(game_world::lorule::dark::ruins::get("Hinox (5)").unwrap()),
                    check_free(game_world::lorule::dark::ruins::get("Hinox (6)").unwrap()),
                ],
                vec![path_free(HinoxCaveShallowWater)],
            ),
        ),
        // Skull Woods Area
        (
            SkullWoodsOverworld,
            location(
                "Skull Woods (Overworld)",
                vec![
                    check(
                        game_world::lorule::skull::overworld::get("Canyon House").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // portal clip through house
                        None,
                    ),
                    check_free(
                        game_world::lorule::skull::overworld::get("Destroyed House").unwrap(),
                    ),
                    check(
                        game_world::lorule::skull::overworld::get("[Mai] Skull Woods Grass")
                            .unwrap(),
                        Some(|p| p.can_cut_grass()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::skull::overworld::get("[Mai] Skull Woods Skull")
                            .unwrap(),
                        Some(|p| p.can_destroy_skull()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::skull::overworld::get("[Mai] Skull Woods Shack Tree")
                            .unwrap(),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::lorule::skull::overworld::get("[Mai] Skull Woods Bush")
                            .unwrap(),
                    ),
                    check(
                        game_world::lorule::skull::overworld::get("[Mai] Skull Woods Big Rock")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::skull::overworld::get(
                            "[Mai] Skull Woods Entrance Wall",
                        )
                        .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::skull::overworld::get("[Mai] Skull Woods Dry Pond")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::skull::overworld::get("[Mai] Canyon House Wall")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostSkullWoodsCuccos),
                    ghost(HintGhostSkullWoodsSouth),
                ],
                vec![
                    fast_travel_lorule(),
                    portal_std(RossoHouse),
                    path_free(MysteriousManCave),
                    portal_std(HyruleField),
                    path_free(SkullWoodsFoyer),
                ],
            ),
        ),
        (
            MysteriousManCave,
            location(
                "Mysterious Man Cave",
                vec![check(
                    game_world::lorule::skull::overworld::get("Mysterious Man").unwrap(),
                    Some(|p| p.has_bottle()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![path_free(SkullWoodsOverworld)],
            ),
        ),
        // Lorule Death Mountain
        (
            LoruleDeathWest,
            location(
                "Lorule Death Mountain West",
                vec![
                    check(
                        game_world::lorule::death::mountain::get("Ice Gimos Fight").unwrap(),
                        Some(|p| p.can_defeat_margomill()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::death::mountain::get("Lorule Mountain W Ledge")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs()),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                    check(
                        game_world::lorule::death::mountain::get("Treacherous Tower Intermediate")
                            .unwrap(),
                        Some(|p| {
                            (p.has_sword() || (p.swordless_mode() && p.can_attack()))
                                && (p.has_bombs() || p.has_hammer() || p.has_tornado_rod())
                        }),
                        Some(|p| {
                            p.has_bombs()
                                || p.has_hammer()
                                || (p.has_tornado_rod() && p.has_lamp_or_net())
                        }),
                        None,
                        None,
                        None,
                    ),
                    check_unreachable(
                        game_world::lorule::death::mountain::get("Treacherous Tower Advanced (1)")
                            .unwrap(),
                    ),
                    check_unreachable(
                        game_world::lorule::death::mountain::get("Treacherous Tower Advanced (2)")
                            .unwrap(),
                    ),
                    check(
                        game_world::lorule::death::mountain::get("[Mai] Lorule Mountain W Skull")
                            .unwrap(),
                        Some(|p| p.can_destroy_skull()),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::death::mountain::get(
                            "[Mai] Lorule Mountain W Big Rock",
                        )
                        .unwrap(),
                        Some(|p| p.has_titans_mitt() && p.has_hammer()),
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_nice_bombs()), // Not enough room for Fire Rod
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_bombs()),
                    ),
                    check(
                        game_world::lorule::death::mountain::get(
                            "[Mai] Lorule Mountain E Big Rock",
                        )
                        .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostTreacherousTower),
                ],
                vec![
                    fast_travel_lorule(),
                    portal_std(DeathMountainBase),
                    path(
                        RossosOreMineLorule,
                        None,
                        None,
                        Some(|p| {
                            p.has_hookshot()
                                && (p.has_fire_rod() || p.has_nice_bombs() || p.has_tornado_rod())
                        }),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            RossosOreMineLorule,
            location(
                "Rosso's Ore Mine Lorule",
                vec![check(
                    game_world::lorule::death::mountain::get("[Mai] Lorule Mountain E Wall")
                        .unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_lorule(),
                    portal_std(RossosOreMine),
                    path(LoruleDeathWest, Some(|p| p.has_hookshot()), None, None, None, None),
                    path_free(IceCaveEast),
                ],
            ),
        ),
        (
            IceCaveEast,
            location(
                "Ice Cave East",
                vec![],
                vec![
                    path_free(RossosOreMineLorule),
                    path(IceCaveCenter, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            IceCaveCenter,
            location(
                "Ice Cave Center",
                vec![],
                vec![
                    path(IceCaveEast, Some(|p| p.can_merge()), None, None, None, None),
                    path(
                        IceCaveSouth,
                        Some(|p| p.can_merge()),
                        Some(|p| p.has_tornado_rod()), // jump over merge block
                        None,
                        None,
                        Some(|_| true), // big yeets from the statue
                    ),
                    path(IceCaveWest, Some(|p| p.has_tornado_rod()), None, None, None, None),
                    path_free(LoruleDeathEastTop),
                ],
            ),
        ),
        (
            IceCaveSouth,
            location(
                "Ice Cave South",
                vec![],
                vec![
                    path_free(LoruleDeathEastLedgeLower),
                    path(IceCaveCenter, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            IceCaveWest,
            location(
                "Ice Cave West",
                vec![],
                vec![
                    path_free(IceCaveCenter),
                    path(IceCaveNorthWest, Some(|p| p.has_tornado_rod()), None, None, None, None),
                    path(IceCaveSouthWest, Some(|p| p.has_tornado_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            IceCaveNorthWest,
            location(
                "Ice Cave North West",
                vec![],
                vec![
                    path_free(FloatingIslandLorule),
                    path(
                        IceCaveWest,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        Some(|p| p.has_boots()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            FloatingIslandLorule,
            location(
                "Floating Island Lorule",
                vec![],
                vec![
                    fast_travel_lorule(),
                    path_free(IceCaveNorthWest),
                    portal_std(FloatingIslandHyrule),
                ],
            ),
        ),
        (
            IceCaveSouthWest,
            location(
                "Ice Cave South West",
                vec![],
                vec![path_free(IceCaveWest), path_free(LoruleDeathEastLedgeUpper)],
            ),
        ),
        (
            LoruleDeathEastLedgeUpper,
            location(
                "Lorule Death Mountain East Upper Ledge",
                vec![check(
                    game_world::lorule::death::mountain::get("Lorule Mountain E Ledge").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_lorule(),
                    path_free(IceCaveWest),
                    path_free(LoruleDeathEastLedgeLower),
                ],
            ),
        ),
        (
            LoruleDeathEastLedgeLower,
            location(
                "Lorule Death Mountain East Lower Ledge",
                vec![check(
                    game_world::lorule::death::mountain::get("[Mai] Lorule Mountain E Skull")
                        .unwrap(),
                    Some(|p| p.can_destroy_skull()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![fast_travel_lorule(), path_free(IceCaveSouth)],
            ),
        ),
        (
            LoruleDeathEastTop,
            location(
                "Lorule Death Mountain East Top",
                vec![
                    check(
                        game_world::lorule::death::mountain::get("Behind Ice Gimos").unwrap(),
                        Some(|p| p.has_fire_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::lorule::death::mountain::get("[Mai] Outside Ice Ruins")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhostIceRuinsOutside),
                ],
                vec![
                    fast_travel_lorule(),
                    path_free(IceCaveCenter),
                    path(IceRuinsFoyer, Some(|p| p.has_fire_rod()), None, None, None, None),
                ],
            ),
        ),
    ])
}

/// Eastern Palace
fn eastern_palace() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            EasternPalaceFoyer,
            location(
                "Eastern Palace",
                vec![check(
                    game_world::dungeons::eastern::palace::get("[EP] (1F) Merge Chest").unwrap(),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path_free(EasternRuinsUpper),
                    path(
                        EasternPalace1F,
                        Some(|p| p.can_hit_far_switch() || p.can_merge() || p.has_nice_ice_rod()),
                        Some(|p| p.has_master_sword()),
                        None,
                        None,
                        None, // not including Nice Ice Rod for now
                    ),
                ],
            ),
        ),
        (
            EasternPalace1F,
            location(
                "Eastern Palace 1F",
                vec![
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (1F) Left Door Chest")
                            .unwrap(),
                        Some(|p| p.can_hit_far_switch() || p.has_nice_ice_rod()),
                        Some(|_| true), // throw pot
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (1F) Popo Room").unwrap(),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (1F) Secret Room")
                            .unwrap(),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (1F) Switch Room")
                            .unwrap(),
                        Some(|p| p.can_hit_far_switch()),
                        Some(|p| p.has_ice_rod() || p.has_master_sword()), // Ice Rod + Pot
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path(
                        EasternPalaceFoyer,
                        Some(|p| p.can_hit_switch() || p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        EasternPalaceMiniboss,
                        Some(|p| p.has_eastern_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            EasternPalaceMiniboss,
            location(
                "Eastern Palace Miniboss",
                vec![],
                vec![
                    path(
                        EasternPalace1F,
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    path(
                        EasternPalace2F,
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            EasternPalace2F,
            location(
                "Eastern Palace 2F",
                vec![
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (2F) Defeat Popos")
                            .unwrap(),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::eastern::palace::get("[EP] (2F) Ball Room").unwrap(),
                    ),
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (2F) Switch Room")
                            .unwrap(),
                        Some(|p| p.can_hit_far_switch() || p.has_ice_rod()),
                        Some(|_| true), // pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::eastern::palace::get("[EP] (2F) Big Chest").unwrap(),
                        Some(|p| p.has_eastern_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
                vec![
                    path_free(EasternPalaceMiniboss),
                    path(
                        EasternPalaceBoss,
                        Some(|p| {
                            p.has_eastern_big_key()
                                && p.has_eastern_keys(2)
                                && p.can_attack()
                                && p.can_hit_far_switch()
                        }),
                        Some(|p| {
                            p.has_eastern_big_key()
                                && (p.has_bombs()
                                    || p.has_ice_rod()
                                    || (p.has_eastern_keys(2)
                                        && p.has_lamp_or_net()
                                        && p.can_hit_far_switch()))
                        }),
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
            ),
        ),
        (
            EasternPalaceBoss,
            location(
                "Eastern Palace 3F",
                vec![],
                vec![path(
                    EasternPalacePostYuga,
                    Some(|p| p.has_bow()),
                    Some(|p| {
                        p.has_bombs()
                            || ((p.has_boomerang() || p.has_hookshot())
                                && (p.can_attack() || p.has_lamp_or_net()))
                            || p.has_nice_ice_rod()
                    }),
                    None,
                    None,
                    Some(|p| p.has_master_sword() || p.has_ice_rod()), // gross
                )],
            ),
        ),
        (
            EasternPalacePostYuga,
            location(
                "Eastern Palace Post Yuga",
                vec![
                    check_free(
                        game_world::dungeons::eastern::palace::get("[EP] Yuga (1)").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::eastern::palace::get("[EP] Yuga (2)").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::eastern::palace::get("Eastern Palace Prize").unwrap(),
                    ),
                    check_quest_free("Eastern Palace Complete", Yuga),
                ],
                vec![
                    path_free(EasternPalace2F),
                    path(EasternPalaceEscape, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            EasternPalaceEscape,
            location(
                "Eastern Palace Escape",
                vec![
                    check_free(
                        game_world::dungeons::eastern::palace::get("[EP] (3F) Escape Chest")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::eastern::palace::get("[EP] (1F) Escape Chest")
                            .unwrap(),
                    ),
                ],
                vec![
                    // do not include path back to 3F
                    path_free(EasternPalace1F),
                ],
            ),
        ),
    ])
}

/// House of Gales
fn house_of_gales() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            HouseOfGalesFoyer,
            location(
                "House of Gales Entrance",
                vec![],
                vec![
                    path_free(HouseOfGalesIsland),
                    path(HouseOfGalesEast1F, Some(|p| p.has_tornado_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            HouseOfGalesEast1F,
            location(
                "House of Gales East 1F",
                vec![
                    check(
                        game_world::dungeons::house::gales::get("[HG] (1F) Torches").unwrap(),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::house::gales::get("[HG] (1F) Switch Room").unwrap(),
                        Some(|p| p.can_merge()),
                        Some(|_| true), // might need to deathwarp to escape
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::house::gales::get("[HG] (1F) Fire Bubbles").unwrap(),
                        Some(|p| p.can_merge() && p.can_attack_fireproof()),
                        Some(|p| p.can_merge() && p.has_net()),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(HouseOfGalesFoyer),
                    path(
                        HouseOfGalesWest1F,
                        Some(|p| p.has_gales_keys(1) && p.can_merge()),
                        Some(|p| p.has_gales_keys(1)), // TRod jump onto blocks
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HouseOfGalesWest1F,
            location(
                "House of Gales West 1F",
                vec![
                    check_free(
                        game_world::dungeons::house::gales::get("[HG] (1F) West Room").unwrap(),
                    ),
                    check(
                        game_world::dungeons::house::gales::get("[HG] (1F) West Room Secret")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(HouseOfGalesEast1F),
                    path(
                        HouseOfGales2F,
                        Some(|p| p.can_hit_hog_1f_switch()), // oddly specific switch hitting requirements
                        Some(|p| p.has_master_sword()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HouseOfGales2F,
            location(
                "House of Gales 2F",
                vec![
                    check(
                        game_world::dungeons::house::gales::get("[HG] (2F) Narrow Ledge").unwrap(),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        Some(|_| true), // can just grab it with TRod
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::house::gales::get("[HG] (2F) Big Chest").unwrap(),
                    ),
                    check(
                        game_world::dungeons::house::gales::get("[HG] (2F) Fire Ring").unwrap(),
                        Some(|p| p.can_merge() && p.has_gales_keys(3)), // should really be 2, but defending against bad key use
                        None,
                        Some(|p| p.can_merge() && p.has_boots()),
                        None,
                        Some(|p| p.can_merge()), // awful Armos Boost
                    ),
                ],
                vec![
                    path_free(HouseOfGalesWest1F),
                    path(
                        HouseOfGales3F,
                        Some(|p| {
                            p.has_gales_keys(3)
                                && p.can_attack_fireproof()
                                && p.can_hit_switch()
                                && p.can_merge()
                        }),
                        Some(|p| {
                            p.has_gales_keys(3)
                                && p.has_net()
                                && p.can_hit_switch()
                                && p.can_merge()
                        }),
                        Some(|p| p.can_merge()), // Skip Skip Skip
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HouseOfGales3F,
            location(
                "House of Gales 3F",
                vec![
                    check(
                        game_world::dungeons::house::gales::get("[HG] (3F) Fire Bubbles").unwrap(),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::house::gales::get("[HG] (3F) Rat Room").unwrap(),
                        Some(|p| p.has_fire_source() || p.has_gales_keys(4)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(HouseOfGales2F),
                    path(
                        HouseOfGalesBoss,
                        Some(|p| p.has_gales_keys(4) && p.has_gales_big_key()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HouseOfGalesBoss,
            location(
                "House of Gales Boss",
                vec![],
                vec![path(
                    HouseOfGalesPostBoss,
                    Some(|p| p.can_defeat_margomill()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            HouseOfGalesPostBoss,
            location(
                "Margomill Defeated",
                vec![
                    check_free(game_world::dungeons::house::gales::get("[HG] Margomill").unwrap()),
                    check_free(
                        game_world::dungeons::house::gales::get("House of Gales Prize").unwrap(),
                    ),
                    check_quest_free("Margomill Defeated", Margomill),
                ],
                vec![],
            ),
        ),
    ])
}

/// Tower of Hera
fn tower_of_hera() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            TowerOfHeraFoyer,
            location(
                "Tower of Hera Entrance",
                vec![],
                vec![
                    path_free(DeathMountainWestTop),
                    path(TowerOfHeraBottom, Some(|p| p.has_hammer()), None, None, None, None),
                ],
            ),
        ),
        (
            TowerOfHeraBottom,
            location(
                "Tower of Hera Bottom",
                vec![
                    check(
                        game_world::dungeons::tower::hera::get("[TH] (1F) Outside").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::tower::hera::get("[TH] (1F) Center").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs()),
                        None,
                    ),
                    check(
                        game_world::dungeons::tower::hera::get("[TH] (3F) Platform").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs()),
                        None,
                    ),
                ],
                vec![
                    path(TowerOfHeraFoyer, Some(|p| p.has_hammer()), None, None, None, None),
                    path(
                        TowerOfHeraMiddle,
                        Some(|p| p.has_hera_keys(1) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs() && p.has_tornado_rod()),
                        None,
                    ),
                ],
            ),
        ),
        (
            TowerOfHeraMiddle,
            location(
                "Tower of Hera Middle",
                vec![
                    check_free(
                        game_world::dungeons::tower::hera::get("[TH] (5F) Red/Blue Switches")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::tower::hera::get("[TH] (6F) Right Mole").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::tower::hera::get("[TH] (6F) Left Mole").unwrap(),
                    ),
                ],
                vec![
                    path_free(TowerOfHeraBottom),
                    path(
                        TowerOfHeraTop,
                        Some(|p| p.has_hera_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_bombs() && p.has_tornado_rod()),
                        None,
                    ),
                ],
            ),
        ),
        (
            TowerOfHeraTop,
            location(
                "Tower of Hera Top",
                vec![
                    check_free(
                        game_world::dungeons::tower::hera::get("[TH] (7F) Outside (Ledge)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::tower::hera::get("[TH] (8F) Fairy Room").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::tower::hera::get("[TH] (11F) Big Chest").unwrap(),
                    ),
                ],
                vec![
                    path_free(TowerOfHeraMiddle),
                    path(TowerOfHeraBoss, Some(|p| p.has_hera_big_key()), None, None, None, None),
                ],
            ),
        ),
        (
            TowerOfHeraBoss,
            location(
                "Tower of Hera Boss",
                vec![],
                vec![path(
                    TowerOfHeraPostBoss,
                    Some(|p| p.can_defeat_moldorm()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            TowerOfHeraPostBoss,
            location(
                "Tower of Hera Post Boss",
                vec![
                    check_free(game_world::dungeons::tower::hera::get("[TH] Moldorm").unwrap()),
                    check_free(
                        game_world::dungeons::tower::hera::get("Tower of Hera Prize").unwrap(),
                    ),
                    check_quest_free("Moldorm", Moldorm),
                ],
                vec![],
            ),
        ),
    ])
}

/// Inside Hyrule Castle
fn inside_hyrule_castle() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            HyruleCastleDungeon,
            location(
                "Inside Hyrule Castle",
                vec![],
                vec![
                    path_free(HyruleCastleRoof),
                    path(
                        HyruleCastleDungeonBoss,
                        Some(|p| (p.can_merge() && p.can_attack()) || p.has_ice_rod()), // add Nice TRod, when nice items figured out
                        Some(|p| p.has_bow() || p.has_nice_bombs()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HyruleCastleDungeonBoss,
            location(
                "Hyrule Castle Dungeon Boss",
                vec![],
                vec![
                    path(
                        HyruleCastleDungeon,
                        Some(|p| p.can_defeat_yuga2()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(ZeldasStudy, Some(|p| p.can_defeat_yuga2()), None, None, None, None),
                ],
            ),
        ),
        (
            ZeldasStudy,
            location(
                "Zelda's Study",
                vec![],
                vec![
                    //path_free(HyruleCastleDungeonBoss), // Don't allow reverse Hyrule Castle
                    portal(
                        HildasStudy,
                        Some(|p| p.can_merge() && p.can_destroy_curtain()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
    ])
}

/// Dark Palace
fn dark_palace() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            DarkPalaceFoyer,
            location(
                "Dark Palace",
                vec![],
                vec![
                    path_free(DarkRuins),
                    path(
                        DarkPalaceSecondRoom,
                        Some(|p| p.has_bombs() && (p.has_lamp() || p.lampless())),
                        None, // not considering Fire Rod + Nice Ice Rod combo yet
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceSecondRoom,
            location(
                "Dark Palace Second Room",
                vec![
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (1F) Right Pit").unwrap(),
                    ),
                    check(
                        game_world::dungeons::dark::palace::get("[PD] (1F) Left Pit").unwrap(),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(DarkPalaceFoyer),
                    path(DarkPalaceMain, Some(|p| p.has_dark_keys(1)), None, None, None, None),
                ],
            ),
        ),
        (
            DarkPalaceMain,
            location(
                "Dark Palace",
                vec![
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (1F) Switch Puzzle").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (1F) Hidden Room (Upper)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (1F) Hidden Room (Lower)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (B1) Fall From 1F").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (B1) Helmasaur Room")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (B1) Helmasaur Room (Fall)")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::dark::palace::get("[PD] (B1) Maze").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(DarkPalaceSecondRoom),
                    path(
                        DarkPalaceLockedDoors,
                        Some(|p| p.has_dark_keys(4)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceLockedDoors,
            location(
                "Dark Palace Locked Doors",
                vec![
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (1F) Fall From 2F").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (2F) Big Chest (Hidden)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (2F) South Hidden Room")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::dark::palace::get("[PD] (2F) Alcove").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] (B1) Big Chest (Switches)")
                            .unwrap(),
                    ),
                ],
                vec![
                    path_free(DarkPalaceMain),
                    path(
                        DarkPalaceBoss,
                        Some(|p| p.has_dark_big_key() && p.can_merge()),
                        Some(|p| p.has_dark_big_key() && p.has_ice_rod()),
                        Some(|p| p.has_dark_big_key() && p.has_nice_bombs()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceBoss,
            location(
                "Dark Palace Boss",
                vec![],
                vec![path(
                    DarkPalaceAfterBoss,
                    Some(|p| p.can_defeat_gemesaur()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            DarkPalaceAfterBoss,
            location(
                "Dark Palace After Boss",
                vec![
                    check_free(
                        game_world::dungeons::dark::palace::get("[PD] Gemesaur King").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::dark::palace::get("Dark Palace Prize").unwrap(),
                    ),
                    check_quest_free("Gemesaur King", GemesaurKing),
                ],
                vec![],
            ),
        ),
    ])
}

/// Swamp Palace
fn swamp_palace() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            SwampPalaceOutside,
            location(
                "Swamp Palace Outside",
                vec![],
                vec![
                    path(
                        LoruleCastleField,
                        Some(|p| p.has_hookshot() || p.has_flippers() || p.has_bomb_flower()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path_free(SwampPalaceAntechamber),
                ],
            ),
        ),
        (
            SwampPalaceAntechamber,
            location(
                "Swamp Palace Antechamber",
                vec![],
                vec![
                    path_free(SwampPalaceOutside),
                    path(
                        SwampPalaceFoyer,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        None,
                        Some(|p| {
                            p.not_nice_mode()
                                && p.can_merge()
                                && p.has_ice_rod()
                                && p.has_flippers()
                                && (p.has_sword()
                                    || p.has_tornado_rod()
                                    || p.has_net()
                                    || p.has_bombs())
                        }),
                        None,
                    ),
                ],
            ),
        ),
        (
            SwampPalaceFoyer,
            location(
                "Swamp Palace Foyer",
                vec![],
                vec![
                    path_free(SwampPalaceAntechamber),
                    path(
                        SwampPalaceMain,
                        Some(|p| p.has_flippers() && p.has_hookshot()),
                        None,
                        None, // what a cruel game
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SwampPalaceMain,
            location(
                "Swamp Palace",
                vec![
                    check_free(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Center").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Waterfall Room")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Raft Room (Pillar)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Raft Room (Right)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Raft Room (Left)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Gyorm").unwrap(),
                    ),
                    check(
                        game_world::dungeons::swamp::palace::get("[SP] (B1) Big Chest (Secret)")
                            .unwrap(),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.has_bow()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        Some(|p| p.has_swamp_keys(2) && p.has_boots()),
                        Some(|p| p.has_swamp_keys(2) && p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::swamp::palace::get("[SP] (1F) West Room").unwrap(),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        Some(|p| p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::swamp::palace::get("[SP] (1F) East Room").unwrap(),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        Some(|p| p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::swamp::palace::get("[SP] (1F) Water Puzzle").unwrap(),
                        Some(|p| {
                            p.has_swamp_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        Some(|p| p.not_nice_mode() && p.can_merge() && p.has_ice_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::swamp::palace::get("[SP] (1F) Big Chest (Fire)")
                            .unwrap(),
                        Some(|p| {
                            p.can_merge()
                                && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                                && (p.has_swamp_keys(4)
                                    || (p.has_swamp_keys(2)
                                        && (p.has_tornado_rod() || p.has_ice_rod())))
                        }),
                        Some(|p| {
                            p.can_merge()
                                && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                                && p.has_swamp_keys(2)
                        }),
                        Some(|p| p.has_boots()),
                        Some(|p| p.not_nice_mode() && p.has_ice_rod()),
                        None,
                    ),
                ],
                vec![path(
                    SwampPalacePostBoss,
                    Some(|p| {
                        p.can_merge()
                            && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                            && p.has_swamp_keys(4)
                            && p.has_swamp_big_key()
                            && p.can_defeat_arrgus()
                    }),
                    None,
                    None,
                    Some(|p| {
                        p.not_nice_mode()
                            && p.has_ice_rod()
                            && (p.has_swamp_big_key() || p.has_tornado_rod())
                    }),
                    None,
                )],
            ),
        ),
        (
            SwampPalacePostBoss,
            location(
                "Swamp Palace Post Boss",
                vec![
                    check_free(game_world::dungeons::swamp::palace::get("[SP] Arrghus").unwrap()),
                    check_free(
                        game_world::dungeons::swamp::palace::get("Swamp Palace Prize").unwrap(),
                    ),
                    check_quest_free("Arrghus", Arrghus),
                ],
                vec![],
            ),
        ),
    ])
}

/// Skull Woods
fn skull_woods() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            SkullWoodsFoyer,
            location(
                "Skull Woods Foyer",
                vec![],
                vec![
                    path_free(SkullWoodsOverworld),
                    path(
                        SkullWoodsMain,
                        Some(|p| p.has_lamp() || p.lampless()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsMain,
            location(
                "Skull Woods",
                vec![
                    check_free(
                        game_world::dungeons::skull::woods::get("[SW] (B1) South Chest").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::skull::woods::get("[SW] (B1) Gibdo Room (Lower)")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::skull::woods::get("[SW] (B1) Gibdo Room (Hole)")
                            .unwrap(),
                        Some(|p| p.has_skull_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::skull::woods::get("[SW] (B1) Grate Room").unwrap(),
                        Some(|p| {
                            p.has_skull_keys(1)
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(SkullWoodsFoyer),
                    path(
                        SkullWoodsB2,
                        Some(|p| {
                            p.has_skull_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsB2,
            location(
                "Skull Woods B2",
                vec![],
                vec![
                    path(
                        SkullWoodsMain,
                        Some(|p| p.can_merge() && p.can_attack()),
                        Some(|p| p.can_merge() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    path(
                        SkullWoodsElevatorHallway,
                        Some(|p| p.can_merge() && p.can_attack()),
                        Some(|p| p.can_merge() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsElevatorHallway,
            location(
                "Skull Woods Elevator Hallway",
                vec![check_free(
                    game_world::dungeons::skull::woods::get("[SW] (B2) Moving Platform Room")
                        .unwrap(),
                )],
                vec![
                    path_free(SkullWoodsB2),
                    path(
                        SkullWoodsBossHallway,
                        Some(|p| p.has_skull_keys(3)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsBossHallway,
            location(
                "Skull Woods Boss Hallway",
                vec![],
                vec![
                    path_free(SkullWoodsElevatorHallway),
                    path(
                        SkullWoodsEastB1NorthFoyer,
                        Some(|p| p.has_fire_source() && p.can_attack()),
                        Some(|p| p.has_lamp()),
                        None,
                        None,
                        None,
                    ),
                    path(
                        SkullWoodsBossRoom,
                        Some(|p| p.has_skull_big_key()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsBossRoom,
            location(
                "Skull Woods Boss Room",
                vec![check(
                    game_world::dungeons::skull::woods::get("[SW] Knucklemaster").unwrap(),
                    Some(|p| p.can_defeat_knucklemaster()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path(
                        SkullWoodsBossHallway,
                        Some(|p| p.can_defeat_knucklemaster()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        SkullWoodsSeresGrove,
                        Some(|p| p.can_defeat_knucklemaster()),
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsSeresGrove,
            location(
                "Skull Woods Seres Grove",
                vec![
                    check_free(
                        game_world::dungeons::skull::woods::get("Skull Woods Prize").unwrap(),
                    ),
                    check_quest_free("Knucklemaster", Knucklemaster),
                ],
                vec![path_free(SkullWoodsBossRoom)],
            ),
        ),
        (
            SkullWoodsEastB1NorthFoyer,
            location(
                "Skull Woods East B1 North Foyer",
                vec![],
                vec![
                    path_free(SkullWoodsBossHallway),
                    path(SkullWoodsEastB1North, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            SkullWoodsEastB1North,
            location(
                "Skull Woods East B1 North",
                vec![
                    check(
                        game_world::dungeons::skull::woods::get("[SW] (B1) Big Chest (Eyes)")
                            .unwrap(),
                        Some(|p| p.has_skull_eyes()),
                        None,
                        None, // Eyeball dupe cannot be considered as it cannot be retried if missed
                        None,
                        None,
                    ),
                    check_quest_free("Skull Eye Right", SkullEyeRight),
                ],
                vec![
                    path(
                        SkullWoodsEastB1NorthFoyer,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        SkullWoodsEastB1South,
                        Some(|p| p.has_skull_eye_right()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsEastB1South,
            location(
                "Skull Woods East B1 South",
                vec![],
                vec![
                    path(
                        SkullWoodsEastB1North,
                        Some(|p| p.can_merge() && p.has_skull_eye_right()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        SkullWoodsEastB1SouthFoyer,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsEastB1SouthFoyer,
            location(
                "Skull Woods East B1 South Foyer",
                vec![],
                vec![
                    path(SkullWoodsEastB1South, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(SkullWoodsOutdoor3),
                ],
            ),
        ),
        (
            SkullWoodsEastB1SouthLedges,
            location(
                "Skull Woods East B1 South Ledges",
                vec![
                    check(
                        game_world::dungeons::skull::woods::get("[SW] (B1) Big Chest (Upper)")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest(
                        "Skull Eye Left",
                        SkullEyeLeft,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![path_free(SkullWoodsEastB1South)],
            ),
        ),
        (
            SkullWoodsOutdoor3,
            location(
                "Skull Woods Outdoor Area 3",
                vec![
                    check_free(
                        game_world::dungeons::skull::woods::get("Skull Woods Outdoor Chest")
                            .unwrap(),
                    ), // Do not use [SW] prefix
                ],
                vec![
                    fast_travel_lorule(),
                    path_free(SkullWoodsEastB1SouthFoyer),
                    path_free(SkullWoodsEastB1SouthLedges),
                ],
            ),
        ),
    ])
}

/// Thieves' Hideout
fn thieves_hideout() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            ThievesHideoutB1,
            location(
                "Thieves' Hideout",
                vec![
                    /* B1 */
                    check_free(
                        game_world::dungeons::thieves::hideout::get("[T'H] (B1) Grate Chest")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get("[T'H] (B1) Jail Cell")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_boots()), // jailbreak
                        None,
                        Some(|p| {
                            p.hell_thieves_statue_clip()
                                && p.has_tornado_rod()
                                && p.can_escape_dungeon()
                        }),
                    ),
                    check_quest(
                        "Thieves' Hideout B1 Door Open",
                        ThievesB1DoorOpen,
                        Some(|p| p.can_merge() && p.can_hit_switch()),
                        None,
                        Some(|p| p.has_boots() && (p.has_boomerang() || p.has_ice_rod())),
                        None,
                        Some(|p| p.has_boots() && p.has_bombs()),
                    ),
                    /* B2 */
                    check(
                        game_world::dungeons::thieves::hideout::get(
                            "[T'H] (B2) Grate Chest (Fall)",
                        )
                        .unwrap(),
                        Some(|p| p.thieves_b1_door_open()),
                        None,
                        None,
                        Some(|p| p.adv_thieves_statue_clip()),
                        Some(|p| p.hell_thieves_statue_clip()),
                    ),
                    check_quest(
                        "Thieves' Hideout B2 Door Open",
                        ThievesB2DoorOpen,
                        Some(|p| {
                            p.thieves_b1_door_open()
                                && p.can_merge()
                                && (p.progression_enemies() || p.has_bombs())
                        }),
                        None,
                        None,
                        Some(|p| {
                            (p.can_merge() || p.can_escape_dungeon()) && p.adv_thieves_statue_clip()
                        }),
                        Some(|p| p.has_bombs()),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get("[T'H] (B2) Jail Cell")
                            .unwrap(),
                        Some(|p| p.thieves_b1b2_doors_open() && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.can_hit_switch()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()), // reach from B3 Out of Bounds
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get(
                            "[T'H] (B2) Switch Puzzle Room",
                        )
                        .unwrap(),
                        Some(|p| p.thieves_b1b2_doors_open()),
                        None,
                        None,
                        Some(|p| p.adv_thieves_statue_clip()),
                        Some(|p| p.hell_thieves_statue_clip()),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get("[T'H] (B2) Eyegores").unwrap(),
                        Some(|p| {
                            p.thieves_b1b2_doors_open()
                                && p.can_merge()
                                && (p.progression_enemies() || p.has_bombs())
                                && p.can_hit_shielded_switch()
                                && (p.has_sword() || p.has_bow()) // Fight is too hard for "any attacking item" to be in Normal Logic. Limit to Sword or Bow (which deals triple damage)
                        }),
                        Some(|p| {
                            p.thieves_b1b2_doors_open()
                                && p.can_merge()
                                && (p.progression_enemies() || p.has_bombs())
                                && p.can_hit_shielded_switch()
                                && (p.can_attack() || p.has_lamp_or_net())
                        }),
                        None,
                        Some(|p| {
                            p.adv_thieves_statue_clip() && (p.has_boots() || p.has_tornado_rod())
                        }),
                        None,
                    ),
                    /* Escape */
                    check_quest(
                        "Thieves' Hideout B3 Water Drained",
                        ThievesB3WaterDrained,
                        Some(|p| {
                            p.thieves_b1b2_doors_open()
                                && p.has_thieves_key()
                                && p.can_merge()
                                && p.has_flippers()
                                && p.can_attack()
                        }),
                        Some(|p| {
                            p.thieves_b1b2_doors_open()
                                && p.has_thieves_key()
                                && p.can_merge()
                                && p.has_flippers()
                                && p.has_lamp_or_net()
                        }),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get("[T'H] (B3) Underwater")
                            .unwrap(),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get(
                            "[T'H] (B3) Big Chest (Hidden)",
                        )
                        .unwrap(),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get("[T'H] (B1) Behind Wall")
                            .unwrap(),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        None, // I'm just not including this
                        Some(|p| {
                            p.hell_thieves_statue_clip()
                                && p.has_tornado_rod()
                                && p.can_escape_dungeon()
                        }),
                    ),
                    check(
                        game_world::dungeons::thieves::hideout::get(
                            "[T'H] (B1) Big Chest (Entrance)",
                        )
                        .unwrap(),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        None, // I'm just not including this
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                ],
                vec![
                    path_free(LoruleCastleField),
                    path(
                        ThievesBoss,
                        Some(|p| {
                            p.has_thieves_big_key()
                                && p.has_thieves_key()
                                && p.thieves_escape_equipment()
                                && p.can_merge()
                                && p.can_attack()
                        }),
                        Some(|p| {
                            p.has_thieves_big_key()
                                && p.has_thieves_key()
                                && p.thieves_escape_equipment()
                                && p.can_merge()
                                && p.has_lamp_or_net()
                        }),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            ThievesBoss,
            location(
                "Thieves' Hideout Boss",
                vec![],
                vec![path(
                    ThievesPostBoss,
                    Some(|p| p.can_merge() || p.can_attack()),
                    Some(|p| p.can_merge() || p.has_lamp_or_net()),
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            ThievesPostBoss,
            location(
                "Thieves' Hideout Post Boss",
                vec![
                    check_free(game_world::dungeons::thieves::hideout::get("Stalblind").unwrap()),
                    check_free(
                        game_world::dungeons::thieves::hideout::get("Thieves' Hideout Prize")
                            .unwrap(),
                    ),
                    check_quest_free("Stalblind Defeated", Stalblind),
                ],
                vec![],
            ),
        ),
    ])
}

/// Ice Ruins
fn ice_ruins() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            IceRuinsFoyer,
            location(
                "Ice Ruins Entrance",
                vec![],
                vec![
                    path_free(LoruleDeathEastTop),
                    path(IceRuins, Some(|p| p.has_fire_rod()), None, None, None, None),
                ],
            ),
        ),
        // Require Fire Rod
        (
            IceRuins,
            location(
                "Ice Ruins",
                vec![
                    check_free(
                        game_world::dungeons::ice::ruins::get("[IR] (1F) Hidden Chest").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::ice::ruins::get("[IR] (B4) Ice Pillar").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::ice::ruins::get("[IR] (B3) Grate Chest (Left)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::ice::ruins::get("[IR] (B3) Grate Chest (Right)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::ice::ruins::get("[IR] (B5) Big Chest").unwrap(),
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B1) Narrow Ledge").unwrap(),
                        Some(|p| p.can_merge() && p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B1) East Chest").unwrap(),
                        Some(|p| p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B1) Upper Chest").unwrap(),
                        Some(|p| p.has_ice_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B2) Long Merge Chest")
                            .unwrap(),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.has_stamina_scroll()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B3) Big Chest (Puzzle)")
                            .unwrap(),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.can_hit_switch()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B4) Switches").unwrap(),
                        Some(|p| {
                            p.has_ice_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies()
                                    || p.has_bombs()
                                    || p.has_nice_ice_rod())
                        }),
                        None,
                        None,
                        Some(|p| {
                            p.has_boots()
                                && (p.progression_enemies()
                                    || p.has_bombs()
                                    || p.has_nice_ice_rod())
                        }),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B4) Southwest Chest (Fall)")
                            .unwrap(),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B4) Narrow Platform").unwrap(),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        game_world::dungeons::ice::ruins::get("[IR] (B4) Southeast Chest (Fall)")
                            .unwrap(),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                ],
                vec![
                    path(IceRuinsFoyer, Some(|p| p.has_fire_rod()), None, None, None, None),
                    path(
                        IceRuinsBoss,
                        Some(|p| p.has_ice_keys(3) && p.has_ice_big_key() && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                ],
            ),
        ),
        (
            IceRuinsBoss,
            location(
                "Ice Ruins Boss",
                vec![],
                vec![path(
                    IceRuinsPostBoss,
                    Some(|p| p.can_defeat_dharkstare()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            IceRuinsPostBoss,
            location(
                "Ice Ruins Post Boss",
                vec![
                    check_free(game_world::dungeons::ice::ruins::get("[IR] Dharkstare").unwrap()),
                    check_free(game_world::dungeons::ice::ruins::get("Ice Ruins Prize").unwrap()),
                    check_quest_free("Dharkstare", Dharkstare),
                ],
                vec![],
            ),
        ),
    ])
}

/// Desert Palace
fn desert_palace() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            DesertPalaceFoyer,
            location(
                "Desert Palace Entrance",
                vec![check(
                    game_world::dungeons::desert::palace::get("[DP] (1F) Entrance").unwrap(),
                    Some(|p| p.has_sand_rod() && p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    path_free(DesertPalaceWeatherVane),
                    path(
                        DesertPalace1F,
                        Some(|p| p.has_sand_rod() && p.can_merge() && p.can_attack()),
                        Some(|p| p.has_sand_rod() && p.can_merge() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DesertPalace1F,
            location(
                "Desert Palace 1F",
                vec![
                    check_free(
                        game_world::dungeons::desert::palace::get("[DP] (1F) Sand Switch Room")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::desert::palace::get("[DP] (1F) Sand Room (North)")
                            .unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::desert::palace::get("[DP] (1F) Sand Room (South)")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::desert::palace::get("[DP] (1F) Behind Rocks")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::desert::palace::get(
                            "[DP] (1F) Big Chest (Behind Wall)",
                        )
                        .unwrap(),
                        Some(|p| p.has_desert_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path(
                        DesertPalaceFoyer,
                        Some(|p| p.has_sand_rod() && p.can_attack()),
                        Some(|p| p.has_sand_rod() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    path(
                        DesertPalaceMidwayLedge,
                        Some(|p| p.has_desert_keys(2) && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DesertPalaceMidwayLedge,
            location(
                "Desert Palace Midway Ledge",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(DesertPalaceWeatherVane),
                    path_free(DesertPalace1F),
                    path_free(DesertPalace2F),
                ],
            ),
        ),
        (
            DesertPalace2F,
            location(
                "Desert Palace 2F",
                vec![
                    check(
                        game_world::dungeons::desert::palace::get("[DP] (2F) Under Rock (Left)")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::desert::palace::get("[DP] (2F) Under Rock (Right)")
                            .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::desert::palace::get(
                            "[DP] (2F) Under Rock (Ball Room)",
                        )
                        .unwrap(),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::desert::palace::get("[DP] (2F) Beamos Room").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::desert::palace::get("[DP] (2F) Red/Blue Switches")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::desert::palace::get("[DP] (2F) Big Chest (Puzzle)")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::desert::palace::get("[DP] (2F) Leever Room").unwrap(),
                        Some(|p| p.has_desert_keys(3)),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
                vec![
                    path_free(DesertPalaceMidwayLedge),
                    path(
                        DesertPalace1F,
                        Some(|p| p.can_attack()),      // midway
                        Some(|p| p.has_lamp_or_net()), // midway
                        None,
                        None,
                        None,
                    ),
                    path(
                        DesertPalace3F,
                        Some(|p| p.has_desert_keys(4) && p.can_merge() && p.has_sand_rod()),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_boots()),
                        None,
                    ),
                ],
            ),
        ),
        (
            DesertPalace3F,
            location(
                "Desert Palace 3F",
                vec![
                    check_free(
                        game_world::dungeons::desert::palace::get("[DP] (3F) Behind Falling Sand")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::desert::palace::get("[DP] (3F) Armos Room").unwrap(),
                        Some(|p| p.can_attack()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(DesertPalace2F),
                    path(
                        DesertPalaceExit3F,
                        Some(|p| {
                            p.has_desert_keys(5)
                                && p.has_desert_big_key()
                                && (p.progression_enemies() || p.has_bombs())
                        }),
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_desert_big_key()),
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
            ),
        ),
        (
            DesertPalaceExit3F,
            location(
                "Desert Palace Exit 3F",
                vec![],
                vec![
                    path(DesertPalace3F, Some(|p| p.has_sand_rod()), None, None, None, None),
                    path_free(DesertZaganagaLedge),
                ],
            ),
        ),
        (
            DesertZaganagaLedge,
            location(
                "Desert Zaganaga Ledge",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    path_free(DesertPalaceExit3F),
                    portal_std(ZaganagasArena),
                ],
            ),
        ),
        (
            ZaganagasArena,
            location(
                "Zaganaga's Arena",
                vec![],
                vec![
                    fast_travel_lorule(),
                    portal_std(DesertZaganagaLedge),
                    path(
                        MiseryMireRewardBasket,
                        Some(|p| p.can_defeat_zaganaga()),
                        None,
                        None,
                        None,
                        Some(|p| p.has_bow() || p.has_master_sword()),
                    ),
                ],
            ),
        ),
        (
            MiseryMireRewardBasket,
            location(
                "Misery Mire Reward Basket",
                vec![
                    check_free(game_world::dungeons::desert::palace::get("Zaganaga").unwrap()), // Do not use [DP] prefix
                    check_free(
                        game_world::dungeons::desert::palace::get("Desert Palace Prize").unwrap(),
                    ),
                    check_quest_free("Zaganaga Defeated", Zaganaga),
                ],
                vec![fast_travel_lorule()],
            ),
        ),
    ])
}

/// Turtle Rock
fn turtle_rock() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            TurtleRockFoyer,
            location(
                "Turtle Rock Foyer",
                vec![],
                vec![
                    path_free(TurtleRockFrontDoor),
                    path(TurtleRockMain, Some(|p| p.has_ice_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockMain,
            location(
                "Turtle Rock Main",
                vec![
                    check_free(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Center").unwrap(),
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Northeast Ledge")
                            .unwrap(),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Southeast Chest")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs() && p.has_tornado_rod()), // bombrod into warp tile
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Defeat Flamolas")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Portal Room NW")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Grate Chest").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::turtle::rock::get("[TR] (B1) Northeast Room")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (B1) Grate Chest (Small)")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None, // I swear there was a bombrod you could do here, idk, leaving it off for now
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (B1) Big Chest (Top)")
                            .unwrap(),
                        Some(|p| {
                            p.has_turtle_keys(1) && p.can_merge() && p.can_hit_shielded_switch()
                        }),
                        Some(|p| (p.has_turtle_keys(1) && p.can_merge())), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (B1) Big Chest (Center)")
                            .unwrap(),
                        Some(|p| p.can_merge() && p.can_hit_shielded_switch()),
                        Some(|p| p.can_merge()), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::turtle::rock::get("[TR] (B1) Platform").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::turtle::rock::get("[TR] (1F) Under Center").unwrap(),
                    ),
                    check_free(
                        game_world::dungeons::turtle::rock::get("[TR] (B1) Under Center").unwrap(),
                    ),
                ],
                vec![
                    path(TurtleRockFoyer, Some(|p| p.has_ice_rod()), None, None, None, None),
                    path(
                        TurtleRockLeftBalconyPath,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        TurtleRockRightBalconyPath,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    path(
                        TurtleRockBoss,
                        Some(|p| p.has_turtle_keys(3) && p.can_merge() && p.has_turtle_big_key()),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_nice_bombs()),
                        None,
                    ),
                ],
            ),
        ),
        (
            TurtleRockLeftBalconyPath,
            location(
                "Turtle Rock Left Balcony Path",
                vec![],
                vec![
                    path(TurtleRockMain, Some(|p| p.has_ice_rod()), None, None, None, None),
                    path(TurtleRockLeftBalcony, Some(|p| p.has_ice_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockLeftBalcony,
            location(
                "Turtle Rock Left Balcony",
                vec![
                    check_free(
                        game_world::dungeons::turtle::rock::get("Turtle Rock Left Balcony")
                            .unwrap(),
                    ), // Do not use [TR] prefix
                ],
                vec![fast_travel_lorule(), path_free(TurtleRockLeftBalconyPath)],
            ),
        ),
        (
            TurtleRockRightBalconyPath,
            location(
                "Turtle Rock Right Balcony Path",
                vec![],
                vec![
                    path(TurtleRockMain, Some(|p| p.has_ice_rod()), None, None, None, None),
                    path(TurtleRockRightBalcony, Some(|p| p.has_ice_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockRightBalcony,
            location(
                "Turtle Rock Right Balcony",
                vec![],
                vec![fast_travel_lorule(), path_free(TurtleRockRightBalconyPath)],
            ),
        ),
        (
            TurtleRockBoss,
            location(
                "Turtle Rock Boss",
                vec![],
                vec![path(
                    TurtleRockPostBoss,
                    Some(|p| p.can_defeat_grinexx()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            TurtleRockPostBoss,
            location(
                "Turtle Rock Post Boss",
                vec![
                    check_free(game_world::dungeons::turtle::rock::get("[TR] Grinexx").unwrap()),
                    check_free(
                        game_world::dungeons::turtle::rock::get("Turtle Rock Prize").unwrap(),
                    ),
                    check_quest_free("Grinexx", Grinexx),
                ],
                vec![],
            ),
        ),
    ])
}

/// Lorule Castle
fn lorule_castle() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            LoruleCastle1F,
            location(
                "Lorule Castle 1F",
                vec![],
                vec![
                    path_free(LoruleCastleField),
                    path(LoruleCastleEastLedge1F, Some(|p| p.can_merge()), None, None, None, None),
                    path(
                        LoruleCastle2F3F,
                        Some(|p| p.can_attack()),
                        Some(|_| true), // throw skulls
                        None,
                        None,
                        None,
                    ),
                    path(LoruleCastleCenter1F, None, None, Some(|p| p.has_boots()), None, None),
                ],
            ),
        ),
        (
            LoruleCastleEastLedge1F,
            location(
                "Lorule Castle East Ledge 1F",
                vec![check_free(
                    game_world::dungeons::lorule::castle::get("[LC] (1F) Ledge").unwrap(),
                )],
                vec![path(LoruleCastle1F, Some(|p| p.can_merge()), None, None, None, None)],
            ),
        ),
        (
            LoruleCastleCenter1F,
            location(
                "Lorule Castle 1F Center",
                vec![check_free(
                    game_world::dungeons::lorule::castle::get("[LC] (1F) Center").unwrap(),
                )],
                vec![
                    path_free(LoruleCastle1F),
                    path(
                        LoruleCastleEastLedge1F,
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            LoruleCastle2F3F,
            location(
                "Lorule Castle 2F 3F",
                vec![
                    check_free(
                        game_world::dungeons::lorule::castle::get("[LC] (2F) Near Torches")
                            .unwrap(),
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (2F) Hidden Path").unwrap(),
                        Some(|p| p.can_extinguish_torches()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (2F) Ledge").unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_boots()),
                        Some(|p| p.has_lorule_keys(3)), // drop from 4F -> 3F -> 2F
                        None,
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get(
                            "[LC] (3F) Bomb Trial Center Chest",
                        )
                        .unwrap(),
                        Some(|p| p.has_bombs()),
                        None,
                        Some(|p| p.has_ice_rod()),
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get(
                            "[LC] (3F) Big Bomb Flower Chest",
                        )
                        .unwrap(),
                        Some(|p| p.has_bombs() && p.can_merge()),
                        Some(|p| p.has_bombs() && p.has_bow()),
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::lorule::castle::get(
                            "[LC] (3F) Merge Trial Free Chest",
                        )
                        .unwrap(),
                    ),
                    check_quest(
                        "Bomb Trial",
                        LcBombTrial,
                        Some(|p| p.has_lorule_keys(5) && p.can_hit_switch() && p.can_attack()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (3F) Spike Ball Chest")
                            .unwrap(),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_sword()),
                        None,
                    ),
                    check_quest(
                        "Ball Trial",
                        LcBallTrial,
                        Some(|p| p.has_lorule_keys(5) && (p.can_attack() || p.has_hookshot())),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    path_free(LoruleCastle1F),
                    path_free(LoruleCastleCenter1F),
                    path(
                        LoruleCastle4F5F,
                        Some(|p| p.has_lorule_keys(3)),
                        None,
                        None,
                        Some(|p| {
                            p.has_nice_bombs()
                                && p.has_tornado_rod()
                                && (p.has_bow() || p.can_merge())
                        }), // secret path
                        None,
                    ),
                    path(
                        HildasStudy,
                        Some(|p| p.has_completed_trials()),
                        None,
                        None,
                        Some(|p| {
                            p.has_sword() && p.has_nice_bombs() && (p.has_bow() || p.can_merge())
                        }),
                        None,
                    ),
                ],
            ),
        ),
        // require 3 small keys
        (
            LoruleCastle4F5F,
            location(
                "Lorule Castle 4F 5F",
                vec![
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (4F) Lamp Trial Chest")
                            .unwrap(),
                        Some(|p| p.has_fire_source()),
                        Some(|_| true), // you don't need it...
                        None,
                        None,
                        None,
                    ),
                    check_quest(
                        "Lamp Trial",
                        LcLampTrial,
                        Some(|p| p.has_lorule_keys(5) && p.has_fire_source() && p.can_attack()),
                        Some(|p| p.has_lorule_keys(5) && p.can_attack()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (4F) Eyeball Chest")
                            .unwrap(),
                        Some(|p| p.has_hookshot() && (p.has_ice_rod() || p.can_merge())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (4F) Lava Switch Chest")
                            .unwrap(),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest(
                        "Hookshot Trial",
                        LcHookTrial,
                        Some(|p| p.has_lorule_keys(5) && p.has_hookshot() && p.can_attack()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(
                        game_world::dungeons::lorule::castle::get("[LC] (4F) Center").unwrap(),
                    ),
                    check(
                        game_world::dungeons::lorule::castle::get("[LC] (4F) Hidden Path").unwrap(),
                        Some(|p| p.can_extinguish_torches()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![path_free(LoruleCastle2F3F)],
            ),
        ),
        (
            HildasStudy,
            location(
                "Hilda's Study",
                vec![],
                vec![path_free(LoruleCastle2F3F), portal_std(ZeldasStudy), path_free(ThroneRoom)],
            ),
        ),
        (
            ThroneRoom,
            location(
                "Throne Room",
                vec![check(
                    game_world::dungeons::lorule::castle::get("Zelda").unwrap(),
                    Some(|p| {
                        p.has_yuganon_requirement()
                            && (p.has_sword() || (p.swordless_mode() && p.has_net()))
                    }),
                    Some(|p| p.has_yuganon_requirement() && p.has_net()),
                    None,
                    None,
                    None,
                )],
                vec![path(
                    SacredRealm,
                    Some(|p| {
                        p.has_yuganon_requirement()
                            && (p.has_sword() || (p.swordless_mode() && p.has_net()))
                            && p.can_merge()
                            && p.has_bow_of_light()
                    }),
                    Some(|p| {
                        p.has_yuganon_requirement()
                            && p.has_net()
                            && p.can_merge()
                            && p.has_bow_of_light()
                    }),
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            SacredRealm,
            location("Sacred Realm", vec![check_quest_free("Sacred Realm", Triforce)], vec![]),
        ),
    ])
}

fn location(name: &'static str, checks: Vec<Check>, paths: Vec<Path>) -> LocationNode {
    LocationNode::new(name, checks, paths)
}

fn check(
    location: game_world::LocationNode, normal: Option<fn(&Progress) -> bool>,
    hard: Option<fn(&Progress) -> bool>, glitched: Option<fn(&Progress) -> bool>,
    adv_glitched: Option<fn(&Progress) -> bool>, hell: Option<fn(&Progress) -> bool>,
) -> Check {
    Check::new(
        location.name,
        Logic::new(normal, hard, glitched, adv_glitched, hell),
        None,
        Some(location),
    )
}

fn check_free(location: game_world::LocationNode) -> Check {
    Check::new(location.name, Logic::free(), None, Some(location))
}

fn check_unreachable(location: game_world::LocationNode) -> Check {
    Check::new(
        location.name,
        Logic { normal: None, hard: None, glitched: None, adv_glitched: None, hell: None },
        None,
        Some(location),
    )
}

fn check_quest(
    name: &'static str, quest: FillerItem, normal: Option<fn(&Progress) -> bool>,
    hard: Option<fn(&Progress) -> bool>, glitched: Option<fn(&Progress) -> bool>,
    adv_glitched: Option<fn(&Progress) -> bool>, hell: Option<fn(&Progress) -> bool>,
) -> Check {
    Check::new(name, Logic::new(normal, hard, glitched, adv_glitched, hell), Some(quest), None)
}

fn check_quest_free(name: &'static str, quest: FillerItem) -> Check {
    Check::new(name, Logic::free(), Some(quest), None)
}

fn ghost(ghost: FillerItem) -> Check {
    Check::new(ghost.as_str(), Logic::free(), Some(ghost), None)
}

fn path_free(default: Location) -> Path {
    Path::new(default, Logic::free())
}

// add logic to choose random entrances here
fn path(
    default: Location, normal: Option<fn(&Progress) -> bool>, hard: Option<fn(&Progress) -> bool>,
    glitched: Option<fn(&Progress) -> bool>, adv_glitched: Option<fn(&Progress) -> bool>,
    hell: Option<fn(&Progress) -> bool>,
) -> Path {
    Path::new(default, Logic::new(normal, hard, glitched, adv_glitched, hell))
}

fn portal_std(default: Location) -> Path {
    portal(default, Some(|p| p.can_merge()), None, None, None, None)
}

// TODO read destination from portal map
fn portal(
    default: Location, normal: Option<fn(&Progress) -> bool>, hard: Option<fn(&Progress) -> bool>,
    glitched: Option<fn(&Progress) -> bool>, adv_glitched: Option<fn(&Progress) -> bool>,
    hell: Option<fn(&Progress) -> bool>,
) -> Path {
    Path::new(default, Logic::new(normal, hard, glitched, adv_glitched, hell))
}

fn fast_travel_hyrule() -> Path {
    Path::new(
        HyruleBellTravel,
        Logic::new(Some(|p| p.are_vanes_activated() && p.has_bell()), None, None, None, None),
    )
}

fn fast_travel_lorule() -> Path {
    Path::new(
        LoruleBellTravel,
        Logic::new(Some(|p| p.are_vanes_activated() && p.has_bell()), None, None, None, None),
    )
}

use std::collections::HashMap;

use game::world as game_world;
use log::info;

use crate::{
    legacy::path::Path,
    model::{
        check::Check,
        location::{Location, Location::*},
        location_node::LocationNode,
        logic::Logic,
        progress::Progress,
    },
    FillerItem::{self, *},
    LocationKey,
};

pub type WorldGraph = HashMap<Location, LocationNode>;

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
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (1)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (3)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (4)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (5)",
                    )),
                    check(
                        LocationKey::new(game_world::hyrule::field::main::AREA, "Ravio (6)"),
                        Some(|p| p.has_sage_osfala()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (7)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (8)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Ravio (9)",
                    )),
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
                    LocationKey::new(game_world::lorule::chamber::sages::AREA, "Osfala"),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::graveyards::hyrule::AREA,
                        "Dampe",
                    )),
                    check(
                        LocationKey::new(game_world::hyrule::irene::witch::AREA, "Irene"),
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
                        LocationKey::new(
                            game_world::dungeons::graveyards::hyrule::AREA,
                            "Sanctuary Pegs",
                        ),
                        Some(|p| p.has_hammer()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "Behind Blacksmith",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boosting
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "Hyrule Castle Rocks",
                        ),
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "Wildlife Clearing Stump",
                        ),
                        Some(|p| p.has_pendant_of_courage()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::southern::ruins::AREA,
                            "Southern Ruins Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    // Lake Hylia
                    check(
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "Lake Hylia Ledge Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "Southeastern Shore",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boosting
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "Hyrule Hotfoot (First Race)",
                        ),
                        Some(|p| p.has_boots()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "Hyrule Hotfoot (Second Race)",
                        ),
                        Some(|p| p.has_boots()),
                        Some(|p| p.can_merge() && p.has_bell()),
                        None,
                        None,
                        Some(|_| true), // Can just walk it
                    ),
                    check(
                        LocationKey::new(game_world::hyrule::eastern::ruins::AREA, "Bird Lover"),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None, // Fake Flippers does not work
                        None,
                    ),
                    // Kakariko Village
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Street Merchant (Left)",
                    )),
                    check(
                        LocationKey::new(
                            game_world::hyrule::kakariko::village::AREA,
                            "Street Merchant (Right)",
                        ),
                        Some(|p| p.has_shady_guy_trigger()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(game_world::hyrule::kakariko::village::AREA, "Shady Guy"),
                        Some(|p| p.has_shady_guy_trigger() && (p.can_merge() || p.has_boots())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Dodge the Cuccos",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Rupee Rush (Hyrule)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "[Mai] Kakariko Bush",
                    )),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "[Mai] Lost Woods Path Rock",
                        ),
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
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "[Mai] Fortune-Teller Tent",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::kakariko::village::AREA,
                            "[Mai] Woman's Roof Rock",
                        ),
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
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "Eastern Ruins Peg Circle",
                        ),
                        Some(|p| p.has_hammer()),
                        None,
                        Some(|p| p.has_boomerang() || p.has_hookshot()),
                        Some(|p| p.has_tornado_rod()),
                        Some(|p| p.has_sand_rod()),
                    ),
                    // Maiamai
                    check(
                        LocationKey::new(game_world::hyrule::lost::woods::AREA, "[Mai] Rosso Wall"),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(game_world::hyrule::lost::woods::AREA, "[Mai] Small Pond"),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::hyrule::AREA,
                            "[Mai] Sanctuary Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Tree Behind Blacksmith",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "[Mai] Lost Woods Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Hyrule Castle Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Hyrule Castle Tornado Tile",
                        ),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::zora::river::AREA,
                            "[Mai] Under Wooden Bridge",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()), // bee boost fake flippers
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "[Mai] Eastern Ruins Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "[Mai] Eastern Ruins Yellow Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "[Mai] Eastern Ruins Green Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "[Mai] Eastern Ruins Big Rock",
                        ),
                        Some(|p| p.can_merge() && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Blacksmith Tornado Tile",
                        ),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "[Mai] Atop Eastern Rocks",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::kakariko::village::AREA,
                            "[Mai] Hyrule Rupee Rush Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::kakariko::village::AREA,
                            "[Mai] Cucco Ranch Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Wildlife Clearing Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Tree West of Link's House",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::field::main::AREA,
                            "[Mai] Behind Link's House",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "[Mai] Southern Bridge River",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()), // bee boost fake flippers
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::southern::ruins::AREA,
                            "[Mai] Southern Ruins Pillars",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::southern::ruins::AREA,
                            "[Mai] Outside Flippers Dungeon",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "[Mai] Outside Maiamai Cave",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "[Mai] Lake Hylia SE Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "[Mai] Hyrule Hotfoot Big Rock",
                        ),
                        Some(|p| p.can_merge() && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::desert::mystery::AREA,
                            "[Mai] Southern Ruins Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "[Mai] Lake Hylia Shallow Ring",
                        ),
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
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 10 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 20 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 30 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 40 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 50 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 60 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 70 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 80 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        " 90 Maiamai",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        "100 Maiamai",
                    )),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            WomanHouse,
            location(
                "Woman's House",
                vec![check(
                    LocationKey::new(game_world::hyrule::kakariko::village::AREA, "Woman"),
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
                    LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "[Mai] Kakariko Sand",
                    ),
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
                    LocationKey::new(
                        game_world::hyrule::zora::river::AREA,
                        "[Mai] Waterfall Ledge Wall",
                    ),
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
                    LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "[Mai] Cucco Dungeon Big Rock",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::field::main::AREA,
                    "Cucco Treasure Dungeon",
                ))],
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
                        LocationKey::new(
                            game_world::hyrule::zora::river::AREA,
                            "[Mai] Inside Witch's House",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::hyrule::eastern::ruins::AREA,
                        "Eastern Ruins Armos Chest",
                    )),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "Eastern Ruins Hookshot Chest",
                        ),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::eastern::ruins::AREA,
                            "Eastern Ruins Merge Chest",
                        ),
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
                    LocationKey::new(game_world::hyrule::zora::river::AREA, "Queen Oren"),
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
                        LocationKey::new(
                            game_world::hyrule::zora::river::AREA,
                            "Zora's Domain Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::zora::river::AREA,
                            "[Mai] Zora's Domain Water",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::zora::river::AREA,
                            "[Mai] Zora's Domain South Wall",
                        ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::zora::river::AREA,
                    "Waterfall Cave",
                ))],
                vec![path_free(WaterfallCaveShallowWater)],
            ),
        ),
        (
            MergeDungeon,
            location(
                "Eastern Ruins Treasure Dungeon",
                vec![check(
                    LocationKey::new(
                        game_world::hyrule::eastern::ruins::AREA,
                        "Eastern Ruins Treasure Dungeon",
                    ),
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
                    LocationKey::new(
                        game_world::hyrule::eastern::ruins::AREA,
                        "Eastern Ruins Cave",
                    ),
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
                        LocationKey::new(
                            game_world::hyrule::lake::hylia::AREA,
                            "[Mai] Island Tornado Tile",
                        ),
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
                        LocationKey::new(game_world::hyrule::lost::woods::AREA, "Rosso"),
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
                        LocationKey::new(game_world::hyrule::lost::woods::AREA, "Rosso Rocks"),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::lost::woods::AREA,
                    "Rosso Cave",
                ))],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            TornadoRodDungeon,
            location(
                "Zora's River Treasure Dungeon",
                vec![check(
                    LocationKey::new(
                        game_world::hyrule::zora::river::AREA,
                        "Zora's River Treasure Dungeon",
                    ),
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
                    LocationKey::new(
                        game_world::dungeons::graveyards::hyrule::AREA,
                        "[Mai] Hyrule Graveyard Wall",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::dungeons::graveyards::hyrule::AREA,
                    "Graveyard Ledge Cave",
                ))],
                vec![path_free(GraveyardLedgeHyrule)],
            ),
        ),
        (
            BlacksmithHouse,
            location(
                "Blacksmith's House (Hyrule)",
                vec![
                    check_free(LocationKey::new(
                        game_world::hyrule::field::main::AREA,
                        "Blacksmith Table",
                    )),
                    check(
                        LocationKey::new(game_world::hyrule::field::main::AREA, "Blacksmith"),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::field::main::AREA,
                    "Blacksmith Cave",
                ))],
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
                    check_free(LocationKey::new(
                        game_world::dungeons::hyrule::castle::AREA,
                        "Hyrule Castle Prize",
                    )),
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
                vec![check_free(LocationKey::new(
                    game_world::dungeons::hyrule::castle::AREA,
                    "Hyrule Castle West Wing",
                ))],
                vec![path_free(HyruleCastleCourtyard)],
            ),
        ),
        (
            HyruleCastleRoof,
            location(
                "Hyrule Castle Roof",
                vec![check_free(LocationKey::new(
                    game_world::dungeons::hyrule::castle::AREA,
                    "Hyrule Castle Battlement",
                ))],
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
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "Lost Woods Alcove",
                        ),
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
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "Lost Woods Big Rock Chest",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // Use Crow to escape
                    ),
                    check_free(LocationKey::new(
                        game_world::hyrule::lost::woods::AREA,
                        "[Mai] Lost Woods Bush",
                    )),
                    check(
                        LocationKey::new(
                            game_world::hyrule::lost::woods::AREA,
                            "[Mai] Lost Woods Rock",
                        ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::lost::woods::AREA,
                    "Master Sword Pedestal",
                ))],
                vec![fast_travel_hyrule(), path_free(LostWoods)],
            ),
        ),
        (
            FortuneTeller,
            location(
                "Fortune-Teller (Hyrule)",
                vec![check_free(LocationKey::new(
                    game_world::hyrule::lost::woods::AREA,
                    "Fortune-Teller",
                ))],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            KakarikoJailCell,
            location(
                "Kakariko Jail Cell",
                vec![check(
                    LocationKey::new(game_world::hyrule::kakariko::village::AREA, "Kakariko Jail"),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::kakariko::village::AREA,
                    "Kakariko Well (Top)",
                ))],
                vec![path_free(WellLower)],
            ),
        ),
        (
            WellLower,
            location(
                "Kakariko Well Lower",
                vec![check_free(LocationKey::new(
                    game_world::hyrule::kakariko::village::AREA,
                    "Kakariko Well (Bottom)",
                ))],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            StylishWomanHouse,
            location(
                "Stylish Woman's House",
                vec![
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Stylish Woman",
                    )),
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
                        LocationKey::new(
                            game_world::hyrule::kakariko::village::AREA,
                            "Bee Guy (1)",
                        ),
                        Some(|p| p.has_bottle()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::hyrule::kakariko::village::AREA,
                            "Bee Guy (2)",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Kakariko Item Shop (1)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Kakariko Item Shop (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::kakariko::village::AREA,
                        "Kakariko Item Shop (3)",
                    )),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            LakesideItemShop,
            location(
                "Lakeside Item Shop",
                vec![
                    check_free(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        "Lakeside Item Shop (1)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        "Lakeside Item Shop (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::hyrule::lake::hylia::AREA,
                        "Lakeside Item Shop (3)",
                    )),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            ItemSellerCave,
            location(
                "Runaway Item-Seller Cave",
                vec![check(
                    LocationKey::new(
                        game_world::hyrule::southern::ruins::AREA,
                        "Runaway Item Seller",
                    ),
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
                    LocationKey::new(
                        game_world::hyrule::southern::ruins::AREA,
                        "Southern Ruins Treasure Dungeon",
                    ),
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
                    LocationKey::new(
                        game_world::hyrule::southern::ruins::AREA,
                        "[Mai] Southern Ruins Bomb Cave",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::southern::ruins::AREA,
                    "Southern Ruins Pillar Cave",
                ))],
                vec![fast_travel_hyrule(), path_free(SouthernRuinsBombCave)],
            ),
        ),
        (
            LakeDarkCave,
            location(
                "Lake Hylia Dark Cave",
                vec![check(
                    LocationKey::new(game_world::hyrule::lake::hylia::AREA, "Lake Hylia Dark Cave"),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::lake::hylia::AREA,
                    "Ice Rod Cave",
                ))],
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
                    check_free(LocationKey::new(
                        game_world::dungeons::graveyards::hyrule::AREA,
                        "[HS] Entrance",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::hyrule::AREA,
                            "[HS] Lower Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::hyrule::AREA,
                            "[HS] Upper Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::hyrule::AREA,
                            "[HS] Ledge",
                        ),
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
                    LocationKey::new(game_world::hyrule::lost::woods::AREA, "[Mai] Moldorm Ledge"),
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
                    LocationKey::new(
                        game_world::hyrule::death::mountain::AREA,
                        "[Mai] Death Mountain Base Rock",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::death::mountain::AREA,
                    "Death Mountain Blocked Cave",
                ))],
                vec![path_free(DeathMountainBase)],
            ),
        ),
        (
            DeathWeatherVaneCaveLeft,
            location(
                "Death Mountain Cave Left of Weather Vane",
                vec![check_free(LocationKey::new(
                    game_world::hyrule::death::mountain::AREA,
                    "Death Mountain Open Cave",
                ))],
                vec![path_free(DeathMountainBase)],
            ),
        ),
        (
            DeathFairyCave,
            location(
                "Death Mountain Fairy Cave",
                vec![check(
                    LocationKey::new(
                        game_world::hyrule::death::mountain::AREA,
                        "Death Mountain Fairy Cave",
                    ),
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
                    LocationKey::new(game_world::hyrule::death::mountain::AREA, "Donkey Cave Pegs"),
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
                    check_free(LocationKey::new(
                        game_world::hyrule::death::mountain::AREA,
                        "Death Mountain West Ledge",
                    )),
                    check(
                        LocationKey::new(
                            game_world::hyrule::death::mountain::AREA,
                            "[Mai] Death Mountain West Ledge",
                        ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::death::mountain::AREA,
                    "Death Mountain West Highest Cave",
                ))],
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::death::mountain::AREA,
                    "Spectacle Rock",
                ))],
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
                        LocationKey::new(
                            game_world::hyrule::death::mountain::AREA,
                            "[Mai] Outside Hookshot Dungeon",
                        ),
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
                    LocationKey::new(
                        game_world::hyrule::death::mountain::AREA,
                        "Death Mountain Treasure Dungeon",
                    ),
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
                    LocationKey::new(game_world::hyrule::death::mountain::AREA, "Fire Cave Pillar"),
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
                    LocationKey::new(
                        game_world::hyrule::death::mountain::AREA,
                        "[Mai] Death Mountain East Ledge",
                    ),
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
                        LocationKey::new(
                            game_world::hyrule::death::mountain::AREA,
                            "Bouldering Guy",
                        ),
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
                    LocationKey::new(
                        game_world::hyrule::death::mountain::AREA,
                        "[Mai] Rosso's Ore Mine Rock",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::hyrule::death::mountain::AREA,
                    "Floating Island",
                ))],
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
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Rupee Rush (Lorule)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Octoball Derby",
                    )),
                    check_quest_free("Access Hilda Barrier", AccessLoruleCastleField),
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Fortune's Choice",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule Castle Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule Castle Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Thieves' Town Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Near Lorule Fortune-Teller",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule Blacksmith Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule Rupee Rush Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Octoball Derby Skull",
                        ),
                        Some(|p| p.can_destroy_skull()),
                        Some(|_| true), // throw bush at skull
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Vacant House Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Behind Vacant House",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule S Ruins Pillars",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule S Ruins Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule S Ruins Water",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Thieves' Town Tree",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Thieves' Town Item Shop (1)",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Thieves' Town Item Shop (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Thieves' Town Item Shop (3)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Thieves' Town Item Shop (4)",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "[Mai] Big Bomb Flower Grass",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::graveyards::lorule::AREA,
                        "Graveyard Peninsula",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[Mai] Lorule Graveyard Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[Mai] Lorule Graveyard Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[Mai] Lorule Graveyard Tree",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[LS] Entrance Chest",
                        ),
                        Some(|p| p.has_lamp() || p.lampless()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[LS] Lower Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[LS] Upper Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::graveyards::lorule::AREA,
                            "[LS] Ledge",
                        ),
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
                    LocationKey::new(
                        game_world::dungeons::graveyards::lorule::AREA,
                        "Philosopher's Cave",
                    ),
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
                    LocationKey::new(game_world::lorule::field::main::AREA, "Great Rupee Fairy"),
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
                    LocationKey::new(game_world::lorule::field::main::AREA, "Blacksmith (Lorule)"),
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
                    LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Lorule Field Treasure Dungeon",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::lorule::field::main::AREA,
                    "Vacant House",
                ))],
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
                    LocationKey::new(game_world::lorule::field::main::AREA, "Thief Girl"),
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
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Swamp Cave (Left)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Swamp Cave (Middle)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::field::main::AREA,
                        "Swamp Cave (Right)",
                    )),
                ],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            BigBombCave,
            location(
                "Haunted Grove Big Bomb Cave",
                vec![check_free(LocationKey::new(
                    game_world::lorule::field::main::AREA,
                    "Big Bomb Flower Cave",
                ))],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            HauntedGroveLedge,
            location(
                "Haunted Grove Upper Ledge",
                vec![
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "Lorule Field Hookshot Chest",
                        ),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::field::main::AREA,
                            "[Mai] Lorule Haunted Grove Wall",
                        ),
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
                        LocationKey::new(
                            game_world::hyrule::desert::mystery::AREA,
                            "[Mai] Buried in the Desert",
                        ),
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
                    LocationKey::new(
                        game_world::hyrule::desert::mystery::AREA,
                        "[Mai] Buried near Desert Palace",
                    ),
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
                        LocationKey::new(
                            game_world::lorule::misery::mire::AREA,
                            "[Mai] Misery Mire Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::misery::mire::AREA,
                            "[Mai] Misery Mire Water",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::misery::mire::AREA,
                            "[Mai] Misery Mire Big Rock",
                        ),
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
                    LocationKey::new(
                        game_world::lorule::misery::mire::AREA,
                        "Misery Mire Treasure Dungeon",
                    ),
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
                vec![check_free(LocationKey::new(
                    game_world::lorule::misery::mire::AREA,
                    "Misery Mire Ledge",
                ))],
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
                        LocationKey::new(
                            game_world::lorule::lake::lorule::AREA,
                            "[Mai] Lorule Lake SE Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::lake::lorule::AREA,
                            "[Mai] Lorule Lake Skull",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::lorule::lake::lorule::AREA,
                        "Lorule Lake Chest",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::lake::lorule::AREA,
                            "[Mai] Lorule Lake West Wall",
                        ),
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
                        LocationKey::new(
                            game_world::lorule::lake::lorule::AREA,
                            "[Mai] Lorule Lake Big Rock",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::lorule::lake::lorule::AREA,
                        "Lorule Lakeside Item Shop (1)",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::lorule::lake::lorule::AREA,
                        "Lorule Lakeside Item Shop (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::lake::lorule::AREA,
                        "Lorule Lakeside Item Shop (3)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::lake::lorule::AREA,
                        "Lorule Lakeside Item Shop (4)",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::lorule::lake::lorule::AREA,
                        "[Mai] Lorule Lake Water",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Dark Ruins Lakeview Chest",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Dark Ruins Waterfall",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()),
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Dark Maze Entrance Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Atop Dark Ruins Rocks",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Dark Ruins West Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Dark Ruins East Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Dark Ruins South Area Wall",
                        ),
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
                    LocationKey::new(game_world::lorule::dark::ruins::AREA, "Dark Maze Chest"),
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
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Dark Maze Ledge",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Dark Maze Center Wall",
                        ),
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
                    LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "[Mai] Ku's Domain Grass",
                    ),
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
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "Ku's Domain Fight",
                        ),
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
                        LocationKey::new(
                            game_world::lorule::dark::ruins::AREA,
                            "[Mai] Ku's Domain Water",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "[Mai] Outside Hinox Cave",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Hinox (1)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Hinox (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Hinox (3)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Hinox (4)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Hinox (5)",
                    )),
                    check_free(LocationKey::new(
                        game_world::lorule::dark::ruins::AREA,
                        "Hinox (6)",
                    )),
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
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "Canyon House",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // portal clip through house
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::lorule::skull::overworld::AREA,
                        "Destroyed House",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Skull Woods Grass",
                        ),
                        Some(|p| p.can_cut_grass()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Skull Woods Skull",
                        ),
                        Some(|p| p.can_destroy_skull()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Skull Woods Shack Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::lorule::skull::overworld::AREA,
                        "[Mai] Skull Woods Bush",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Skull Woods Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Skull Woods Entrance Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Skull Woods Dry Pond",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::skull::overworld::AREA,
                            "[Mai] Canyon House Wall",
                        ),
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
                    LocationKey::new(game_world::lorule::skull::overworld::AREA, "Mysterious Man"),
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
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "Ice Gimos Fight",
                        ),
                        Some(|p| p.can_defeat_margomill()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "Lorule Mountain W Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs()),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "Treacherous Tower Intermediate",
                        ),
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
                    check_unreachable(LocationKey::new(
                        game_world::lorule::death::mountain::AREA,
                        "Treacherous Tower Advanced (1)",
                    )),
                    check_unreachable(LocationKey::new(
                        game_world::lorule::death::mountain::AREA,
                        "Treacherous Tower Advanced (2)",
                    )),
                    check(
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "[Mai] Lorule Mountain W Skull",
                        ),
                        Some(|p| p.can_destroy_skull()),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "[Mai] Lorule Mountain W Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt() && p.has_hammer()),
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_nice_bombs()), // Not enough room for Fire Rod
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_bombs()),
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "[Mai] Lorule Mountain E Big Rock",
                        ),
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
                    LocationKey::new(
                        game_world::lorule::death::mountain::AREA,
                        "[Mai] Lorule Mountain E Wall",
                    ),
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
                    LocationKey::new(
                        game_world::lorule::death::mountain::AREA,
                        "Lorule Mountain E Ledge",
                    ),
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
                    LocationKey::new(
                        game_world::lorule::death::mountain::AREA,
                        "[Mai] Lorule Mountain E Skull",
                    ),
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
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "Behind Ice Gimos",
                        ),
                        Some(|p| p.has_fire_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::lorule::death::mountain::AREA,
                            "[Mai] Outside Ice Ruins",
                        ),
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
                    LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "[EP] (1F) Merge Chest",
                    ),
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
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (1F) Left Door Chest",
                        ),
                        Some(|p| p.can_hit_far_switch() || p.has_nice_ice_rod()),
                        Some(|_| true), // throw pot
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (1F) Popo Room",
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (1F) Secret Room",
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (1F) Switch Room",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (2F) Defeat Popos",
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "[EP] (2F) Ball Room",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (2F) Switch Room",
                        ),
                        Some(|p| p.can_hit_far_switch() || p.has_ice_rod()),
                        Some(|_| true), // pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::eastern::palace::AREA,
                            "[EP] (2F) Big Chest",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "[EP] Yuga (1)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "[EP] Yuga (2)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "Eastern Palace Prize",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "[EP] (3F) Escape Chest",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::eastern::palace::AREA,
                        "[EP] (1F) Escape Chest",
                    )),
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
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (1F) Torches",
                        ),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (1F) Switch Room",
                        ),
                        Some(|p| p.can_merge()),
                        Some(|_| true), // might need to deathwarp to escape
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (1F) Fire Bubbles",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::house::gales::AREA,
                        "[HG] (1F) West Room",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (1F) West Room Secret",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (2F) Narrow Ledge",
                        ),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        Some(|_| true), // can just grab it with TRod
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::house::gales::AREA,
                        "[HG] (2F) Big Chest",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (2F) Fire Ring",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (3F) Fire Bubbles",
                        ),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::house::gales::AREA,
                            "[HG] (3F) Rat Room",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::house::gales::AREA,
                        "[HG] Margomill",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::house::gales::AREA,
                        "House of Gales Prize",
                    )),
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
                        LocationKey::new(
                            game_world::dungeons::tower::hera::AREA,
                            "[TH] (1F) Outside",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::tower::hera::AREA,
                            "[TH] (1F) Center",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::tower::hera::AREA,
                            "[TH] (3F) Platform",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] (5F) Red/Blue Switches",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] (6F) Right Mole",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] (6F) Left Mole",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] (7F) Outside (Ledge)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] (8F) Fairy Room",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] (11F) Big Chest",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "[TH] Moldorm",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::tower::hera::AREA,
                        "Tower of Hera Prize",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (1F) Right Pit",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::dark::palace::AREA,
                            "[PD] (1F) Left Pit",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (1F) Switch Puzzle",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (1F) Hidden Room (Upper)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (1F) Hidden Room (Lower)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (B1) Fall From 1F",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (B1) Helmasaur Room",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (B1) Helmasaur Room (Fall)",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::dark::palace::AREA,
                            "[PD] (B1) Maze",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (1F) Fall From 2F",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (2F) Big Chest (Hidden)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (2F) South Hidden Room",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::dark::palace::AREA,
                            "[PD] (2F) Alcove",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] (B1) Big Chest (Switches)",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "[PD] Gemesaur King",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::dark::palace::AREA,
                        "Dark Palace Prize",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] (B1) Center",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] (B1) Waterfall Room",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] (B1) Raft Room (Pillar)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] (B1) Raft Room (Right)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] (B1) Raft Room (Left)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] (B1) Gyorm",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::swamp::palace::AREA,
                            "[SP] (B1) Big Chest (Secret)",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::swamp::palace::AREA,
                            "[SP] (1F) West Room",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::swamp::palace::AREA,
                            "[SP] (1F) East Room",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::swamp::palace::AREA,
                            "[SP] (1F) Water Puzzle",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::swamp::palace::AREA,
                            "[SP] (1F) Big Chest (Fire)",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "[SP] Arrghus",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::swamp::palace::AREA,
                        "Swamp Palace Prize",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::skull::woods::AREA,
                        "[SW] (B1) South Chest",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::skull::woods::AREA,
                        "[SW] (B1) Gibdo Room (Lower)",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::skull::woods::AREA,
                            "[SW] (B1) Gibdo Room (Hole)",
                        ),
                        Some(|p| p.has_skull_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::skull::woods::AREA,
                            "[SW] (B1) Grate Room",
                        ),
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
                vec![check_free(LocationKey::new(
                    game_world::dungeons::skull::woods::AREA,
                    "[SW] (B2) Moving Platform Room",
                ))],
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
                    LocationKey::new(
                        game_world::dungeons::skull::woods::AREA,
                        "[SW] Knucklemaster",
                    ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::skull::woods::AREA,
                        "Skull Woods Prize",
                    )),
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
                        LocationKey::new(
                            game_world::dungeons::skull::woods::AREA,
                            "[SW] (B1) Big Chest (Eyes)",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::skull::woods::AREA,
                            "[SW] (B1) Big Chest (Upper)",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::skull::woods::AREA,
                        "Skull Woods Outdoor Chest",
                    )), // Do not use [SW] prefix
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
                    check_free(LocationKey::new(
                        game_world::dungeons::thieves::hideout::AREA,
                        "[T'H] (B1) Grate Chest",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B1) Jail Cell",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B2) Grate Chest (Fall)",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B2) Jail Cell",
                        ),
                        Some(|p| p.thieves_b1b2_doors_open() && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.can_hit_switch()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()), // reach from B3 Out of Bounds
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B2) Switch Puzzle Room",
                        ),
                        Some(|p| p.thieves_b1b2_doors_open()),
                        None,
                        None,
                        Some(|p| p.adv_thieves_statue_clip()),
                        Some(|p| p.hell_thieves_statue_clip()),
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B2) Eyegores",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B3) Underwater",
                        ),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B3) Big Chest (Hidden)",
                        ),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B1) Behind Wall",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::thieves::hideout::AREA,
                            "[T'H] (B1) Big Chest (Entrance)",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::thieves::hideout::AREA,
                        "Stalblind",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::thieves::hideout::AREA,
                        "Thieves' Hideout Prize",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "[IR] (1F) Hidden Chest",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "[IR] (B4) Ice Pillar",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "[IR] (B3) Grate Chest (Left)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "[IR] (B3) Grate Chest (Right)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "[IR] (B5) Big Chest",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B1) Narrow Ledge",
                        ),
                        Some(|p| p.can_merge() && p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B1) East Chest",
                        ),
                        Some(|p| p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B1) Upper Chest",
                        ),
                        Some(|p| p.has_ice_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B2) Long Merge Chest",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.has_stamina_scroll()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B3) Big Chest (Puzzle)",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.can_hit_switch()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B4) Switches",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B4) Southwest Chest (Fall)",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B4) Narrow Platform",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::ice::ruins::AREA,
                            "[IR] (B4) Southeast Chest (Fall)",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "[IR] Dharkstare",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::ice::ruins::AREA,
                        "Ice Ruins Prize",
                    )),
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
                    LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (1F) Entrance",
                    ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (1F) Sand Switch Room",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (1F) Sand Room (North)",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (1F) Sand Room (South)",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (1F) Behind Rocks",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (1F) Big Chest (Behind Wall)",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (2F) Under Rock (Left)",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (2F) Under Rock (Right)",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (2F) Under Rock (Ball Room)",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (2F) Beamos Room",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (2F) Red/Blue Switches",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (2F) Big Chest (Puzzle)",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (2F) Leever Room",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "[DP] (3F) Behind Falling Sand",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::desert::palace::AREA,
                            "[DP] (3F) Armos Room",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "Zaganaga",
                    )), // Do not use [DP] prefix
                    check_free(LocationKey::new(
                        game_world::dungeons::desert::palace::AREA,
                        "Desert Palace Prize",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "[TR] (1F) Center",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (1F) Northeast Ledge",
                        ),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (1F) Southeast Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs() && p.has_tornado_rod()), // bombrod into warp tile
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (1F) Defeat Flamolas",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (1F) Portal Room NW",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (1F) Grate Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "[TR] (B1) Northeast Room",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (B1) Grate Chest (Small)",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None, // I swear there was a bombrod you could do here, idk, leaving it off for now
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (B1) Big Chest (Top)",
                        ),
                        Some(|p| {
                            p.has_turtle_keys(1) && p.can_merge() && p.can_hit_shielded_switch()
                        }),
                        Some(|p| (p.has_turtle_keys(1) && p.can_merge())), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (B1) Big Chest (Center)",
                        ),
                        Some(|p| p.can_merge() && p.can_hit_shielded_switch()),
                        Some(|p| p.can_merge()), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::turtle::rock::AREA,
                            "[TR] (B1) Platform",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "[TR] (1F) Under Center",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "[TR] (B1) Under Center",
                    )),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "Turtle Rock Left Balcony",
                    )), // Do not use [TR] prefix
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
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "[TR] Grinexx",
                    )),
                    check_free(LocationKey::new(
                        game_world::dungeons::turtle::rock::AREA,
                        "Turtle Rock Prize",
                    )),
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
                vec![check_free(LocationKey::new(
                    game_world::dungeons::lorule::castle::AREA,
                    "[LC] (1F) Ledge",
                ))],
                vec![path(LoruleCastle1F, Some(|p| p.can_merge()), None, None, None, None)],
            ),
        ),
        (
            LoruleCastleCenter1F,
            location(
                "Lorule Castle 1F Center",
                vec![check_free(LocationKey::new(
                    game_world::dungeons::lorule::castle::AREA,
                    "[LC] (1F) Center",
                ))],
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
                    check_free(LocationKey::new(
                        game_world::dungeons::lorule::castle::AREA,
                        "[LC] (2F) Near Torches",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (2F) Hidden Path",
                        ),
                        Some(|p| p.can_extinguish_torches()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (2F) Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_boots()),
                        Some(|p| p.has_lorule_keys(3)), // drop from 4F -> 3F -> 2F
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (3F) Bomb Trial Center Chest",
                        ),
                        Some(|p| p.has_bombs()),
                        None,
                        Some(|p| p.has_ice_rod()),
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (3F) Big Bomb Flower Chest",
                        ),
                        Some(|p| p.has_bombs() && p.can_merge()),
                        Some(|p| p.has_bombs() && p.has_bow()),
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationKey::new(
                        game_world::dungeons::lorule::castle::AREA,
                        "[LC] (3F) Merge Trial Free Chest",
                    )),
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
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (3F) Spike Ball Chest",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (4F) Lamp Trial Chest",
                        ),
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
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (4F) Eyeball Chest",
                        ),
                        Some(|p| p.has_hookshot() && (p.has_ice_rod() || p.can_merge())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (4F) Lava Switch Chest",
                        ),
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
                    check_free(LocationKey::new(
                        game_world::dungeons::lorule::castle::AREA,
                        "[LC] (4F) Center",
                    )),
                    check(
                        LocationKey::new(
                            game_world::dungeons::lorule::castle::AREA,
                            "[LC] (4F) Hidden Path",
                        ),
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
                    LocationKey::new(game_world::dungeons::lorule::castle::AREA, "Zelda"),
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
    location_info: LocationKey, normal: Option<fn(&Progress) -> bool>,
    hard: Option<fn(&Progress) -> bool>, glitched: Option<fn(&Progress) -> bool>,
    adv_glitched: Option<fn(&Progress) -> bool>, hell: Option<fn(&Progress) -> bool>,
) -> Check {
    Check::new(
        location_info.name,
        Logic::new(normal, hard, glitched, adv_glitched, hell),
        None,
        Some(location_info),
    )
}

fn check_free(location_info: LocationKey) -> Check {
    Check::new(location_info.name, Logic::free(), None, Some(location_info))
}

fn check_unreachable(location_info: LocationKey) -> Check {
    Check::new(
        location_info.name,
        Logic { normal: None, hard: None, glitched: None, adv_glitched: None, hell: None },
        None,
        Some(location_info),
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

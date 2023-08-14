use std::collections::HashMap;

use game::HintGhost;
use log::info;

use crate::{
    hints::hint_ghost_name,
    legacy::path::Path,
    model::{
        check::Check,
        filler_item,
        location::{Location, Location::*},
        location_node::LocationNode,
        logic::Logic,
        progress::Progress,
    },
    regions, FillerItem, LocationInfo,
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
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (1)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (3)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (4)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (5)",
                    )),
                    check(
                        LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (6)"),
                        Some(|p| p.has_sage_osfala()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (7)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Ravio (8)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
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
                    LocationInfo::new(regions::lorule::chamber::sages::SUBREGION, "Osfala"),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::graveyards::hyrule::SUBREGION,
                        "Dampe",
                    )),
                    check(
                        LocationInfo::new(regions::hyrule::irene::witch::SUBREGION, "Irene"),
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
                        LocationInfo::new(
                            regions::dungeons::graveyards::hyrule::SUBREGION,
                            "Sanctuary Pegs",
                        ),
                        Some(|p| p.has_hammer()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "Behind Blacksmith",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boosting
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "Hyrule Castle Rocks",
                        ),
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "Wildlife Clearing Stump",
                        ),
                        Some(|p| p.has_pendant_of_courage()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::southern::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "Lake Hylia Ledge Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "Southeastern Shore",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|_| true), // Bee Boosting
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "Hyrule Hotfoot (First Race)",
                        ),
                        Some(|p| p.has_boots()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "Hyrule Hotfoot (Second Race)",
                        ),
                        Some(|p| p.has_boots()),
                        Some(|p| p.can_merge() && p.has_bell()),
                        None,
                        None,
                        Some(|_| true), // Can just walk it
                    ),
                    check(
                        LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Bird Lover"),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None, // Fake Flippers does not work
                        None,
                    ),
                    // Kakariko Village
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Street Merchant (Left)",
                    )),
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
                            "Street Merchant (Right)",
                        ),
                        Some(|p| p.has_shady_guy_trigger()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
                            "Shady Guy",
                        ),
                        Some(|p| p.has_shady_guy_trigger() && (p.can_merge() || p.has_boots())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Dodge the Cuccos",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Rupee Rush (Hyrule)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "[Mai] Kakariko Bush",
                    )),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "[Mai] Fortune-Teller Tent",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
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
                        filler_item::Goal::WomanRoofMaiamai,
                        Some(|p| p.has_power_glove()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    // Eastern Ruins
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "[Mai] Rosso Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "[Mai] Small Pond",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::hyrule::SUBREGION,
                            "[Mai] Sanctuary Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Tree Behind Blacksmith",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "[Mai] Lost Woods Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Hyrule Castle Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Hyrule Castle Tornado Tile",
                        ),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::zora::river::SUBREGION,
                            "[Mai] Under Wooden Bridge",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()), // bee boost fake flippers
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "[Mai] Eastern Ruins Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "[Mai] Eastern Ruins Yellow Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "[Mai] Eastern Ruins Green Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "[Mai] Eastern Ruins Big Rock",
                        ),
                        Some(|p| p.can_merge() && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Blacksmith Tornado Tile",
                        ),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "[Mai] Atop Eastern Rocks",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
                            "[Mai] Hyrule Rupee Rush Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
                            "[Mai] Cucco Ranch Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Wildlife Clearing Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Tree West of Link's House",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::field::main::SUBREGION,
                            "[Mai] Behind Link's House",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "[Mai] Southern Bridge River",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()), // bee boost fake flippers
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::southern::ruins::SUBREGION,
                            "[Mai] Southern Ruins Pillars",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::southern::ruins::SUBREGION,
                            "[Mai] Outside Flippers Dungeon",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "[Mai] Outside Maiamai Cave",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "[Mai] Lake Hylia SE Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "[Mai] Hyrule Hotfoot Big Rock",
                        ),
                        Some(|p| p.can_merge() && p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::desert::mystery::SUBREGION,
                            "[Mai] Southern Ruins Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "[Mai] Lake Hylia Shallow Ring",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::LostWoodsMaze1),
                    ghost(HintGhost::LostWoodsMaze2),
                    ghost(HintGhost::LostWoodsMaze3),
                    ghost(HintGhost::LostWoods),
                    ghost(HintGhost::MoldormCave),
                    ghost(HintGhost::FortuneTellerHyrule),
                    ghost(HintGhost::Sanctuary),
                    ghost(HintGhost::GraveyardHyrule),
                    ghost(HintGhost::Well),
                    ghost(HintGhost::ShadyGuy),
                    ghost(HintGhost::StylishWoman),
                    ghost(HintGhost::BlacksmithCave),
                    ghost(HintGhost::EasternRuinsEntrance),
                    ghost(HintGhost::RupeeRushHyrule),
                    ghost(HintGhost::Cuccos),
                    ghost(HintGhost::SouthBridge),
                    ghost(HintGhost::SouthernRuins),
                    ghost(HintGhost::HyruleHotfoot),
                    ghost(HintGhost::Letter),
                    ghost(HintGhost::StreetPassTree),
                    ghost(HintGhost::BlacksmithBehind),
                    ghost(HintGhost::GraveyardLedge),
                    ghost(HintGhost::HyruleCastleRocks),
                    ghost(HintGhost::WitchsHouse),
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
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 10 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 20 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 30 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 40 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 50 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 60 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 70 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 80 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        " 90 Maiamai",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
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
                    LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Woman"),
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
                    LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::zora::river::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::field::main::SUBREGION,
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
                    check_quest_free("Access Potion Shop", filler_item::Goal::AccessPotionShop),
                    check(
                        LocationInfo::new(
                            regions::hyrule::zora::river::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::hyrule::eastern::ruins::SUBREGION,
                        "Eastern Ruins Armos Chest",
                    )),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "Eastern Ruins Hookshot Chest",
                        ),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::eastern::ruins::SUBREGION,
                            "Eastern Ruins Merge Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_tornado_rod() || p.has_fire_rod() || p.has_nice_bombs()),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                    ghost(HintGhost::EasternRuinsPegs),
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
                vec![ghost(HintGhost::EasternRuinsCave)],
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
                    LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "Queen Oren"),
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
                        filler_item::Goal::ShadyGuyTrigger,
                        Some(|p| !p.is_rse() || p.has_sage_oren()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::zora::river::SUBREGION,
                            "Zora's Domain Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::zora::river::SUBREGION,
                            "[Mai] Zora's Domain Water",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::zora::river::SUBREGION,
                            "[Mai] Zora's Domain South Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::ZorasDomain),
                    ghost(HintGhost::WaterfallCave),
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::zora::river::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::eastern::ruins::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::eastern::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::lake::hylia::SUBREGION,
                            "[Mai] Island Tornado Tile",
                        ),
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::HouseOfGalesIsland),
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
                        LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Rosso"),
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
                        LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Rosso Rocks"),
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::lost::woods::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::zora::river::SUBREGION,
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
                    LocationInfo::new(
                        regions::dungeons::graveyards::hyrule::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::dungeons::graveyards::hyrule::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::hyrule::field::main::SUBREGION,
                        "Blacksmith Table",
                    )),
                    check(
                        LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith"),
                        Some(|p| p.has_master_ore(2)),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_quest_free(
                        "Access Hyrule Blacksmith",
                        filler_item::Goal::AccessHyruleBlacksmith,
                    ),
                ],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            BlacksmithCave,
            location(
                "Blacksmith Cave",
                vec![check_free(LocationInfo::new(
                    regions::hyrule::field::main::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::hyrule::castle::SUBREGION,
                        "Hyrule Castle Prize",
                    )),
                    check_quest_free("Zelda's Throne", filler_item::Goal::ZeldasThrone),
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
                vec![check_free(LocationInfo::new(
                    regions::dungeons::hyrule::castle::SUBREGION,
                    "Hyrule Castle West Wing",
                ))],
                vec![path_free(HyruleCastleCourtyard)],
            ),
        ),
        (
            HyruleCastleRoof,
            location(
                "Hyrule Castle Roof",
                vec![check_free(LocationInfo::new(
                    regions::dungeons::hyrule::castle::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
                            "Lost Woods Big Rock Chest",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // Use Crow to escape
                    ),
                    check_free(LocationInfo::new(
                        regions::hyrule::lost::woods::SUBREGION,
                        "[Mai] Lost Woods Bush",
                    )),
                    check(
                        LocationInfo::new(
                            regions::hyrule::lost::woods::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::lost::woods::SUBREGION,
                    "Master Sword Pedestal",
                ))],
                vec![fast_travel_hyrule(), path_free(LostWoods)],
            ),
        ),
        (
            FortuneTeller,
            location(
                "Fortune-Teller (Hyrule)",
                vec![check_free(LocationInfo::new(
                    regions::hyrule::lost::woods::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Kakariko Jail",
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
            WellUpper,
            location(
                "Kakariko Well Upper",
                vec![check_free(LocationInfo::new(
                    regions::hyrule::kakariko::village::SUBREGION,
                    "Kakariko Well (Top)",
                ))],
                vec![path_free(WellLower)],
            ),
        ),
        (
            WellLower,
            location(
                "Kakariko Well Lower",
                vec![check_free(LocationInfo::new(
                    regions::hyrule::kakariko::village::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Stylish Woman",
                    )),
                    check_quest_free(
                        "Open Stylish Woman's House",
                        filler_item::Goal::StylishWomansHouseOpen,
                    ),
                ],
                vec![portal_std(LoruleCastleField), path_free(HyruleField)],
            ),
        ),
        (
            MilkBar,
            location(
                "Milk Bar",
                vec![check_quest_free("Access Milk Bar", filler_item::Goal::AccessMilkBar)],
                vec![path_free(HyruleField)],
            ),
        ),
        (
            BeeGuyHouse,
            location(
                "Bee Guy's House",
                vec![
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
                            "Bee Guy (1)",
                        ),
                        Some(|p| p.has_bottle()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::hyrule::kakariko::village::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Kakariko Item Shop (1)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
                        "Kakariko Item Shop (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::kakariko::village::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        "Lakeside Item Shop (1)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        "Lakeside Item Shop (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::southern::ruins::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::southern::ruins::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::southern::ruins::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::southern::ruins::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::lake::hylia::SUBREGION,
                        "Lake Hylia Dark Cave",
                    ),
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::lake::hylia::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::graveyards::hyrule::SUBREGION,
                        "[HS] Entrance",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::hyrule::SUBREGION,
                            "[HS] Lower Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::hyrule::SUBREGION,
                            "[HS] Upper Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::hyrule::SUBREGION,
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
                        filler_item::Goal::OpenSanctuaryDoors,
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
                    LocationInfo::new(
                        regions::hyrule::lost::woods::SUBREGION,
                        "[Mai] Moldorm Ledge",
                    ),
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::death::mountain::SUBREGION,
                    "Death Mountain Blocked Cave",
                ))],
                vec![path_free(DeathMountainBase)],
            ),
        ),
        (
            DeathWeatherVaneCaveLeft,
            location(
                "Death Mountain Cave Left of Weather Vane",
                vec![check_free(LocationInfo::new(
                    regions::hyrule::death::mountain::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
                        "Donkey Cave Pegs",
                    ),
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
                    check_free(LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
                        "Death Mountain West Ledge",
                    )),
                    check(
                        LocationInfo::new(
                            regions::hyrule::death::mountain::SUBREGION,
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
                    path(
                        DeathBombCave,
                        None,
                        None,
                        Some(|p| p.has_bombs() && (p.has_boomerang() || p.has_hookshot())),
                        None,
                        None,
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::death::mountain::SUBREGION,
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
                vec![ghost(HintGhost::SpectacleRock)],
                vec![
                    fast_travel_hyrule(),
                    path_free(AmidaCaveUpper),
                    path_free(DeathThirdFloor),
                    path_free(SpectacleRock),
                    path(DeathMountainWestTop, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            SpectacleRock,
            location(
                "Spectacle Rock",
                vec![check_free(LocationInfo::new(
                    regions::hyrule::death::mountain::SUBREGION,
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
                vec![ghost(HintGhost::TowerOfHeraOutside)],
                vec![
                    fast_travel_hyrule(),
                    path_free(SpectacleRockCaveRight),
                    path(TowerOfHeraFoyer, Some(|p| p.has_hammer()), None, None, None, None),
                    path(DeathTopLeftLedge, Some(|p| p.can_merge()), None, None, None, None),
                    path_free(SpectacleRock),
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
                        LocationInfo::new(
                            regions::hyrule::death::mountain::SUBREGION,
                            "[Mai] Outside Hookshot Dungeon",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::FloatingIsland),
                    ghost(HintGhost::FireCave),
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
                        "Fire Cave Pillar",
                    ),
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
                    path_free(BoulderingLedgeBottom),
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::death::mountain::SUBREGION,
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
                        filler_item::Item::Bottle05,
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
                    LocationInfo::new(
                        regions::hyrule::death::mountain::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::hyrule::death::mountain::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Rupee Rush (Lorule)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Octoball Derby",
                    )),
                    check_quest_free(
                        "Access Hilda Barrier",
                        filler_item::Goal::AccessLoruleCastleField,
                    ),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Fortune's Choice",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule Castle Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule Castle Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Thieves' Town Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Near Lorule Fortune-Teller",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule Blacksmith Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule Rupee Rush Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Octoball Derby Skull",
                        ),
                        Some(|p| p.can_destroy_skull()),
                        Some(|_| true), // throw bush at skull
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Vacant House Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Behind Vacant House",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule S Ruins Pillars",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule S Ruins Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Lorule S Ruins Water",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "[Mai] Thieves' Town Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::FortuneTellerLorule),
                    ghost(HintGhost::RupeeRushLorule),
                    ghost(HintGhost::GreatRupeeFairy),
                    ghost(HintGhost::OctoballDerby),
                    ghost(HintGhost::VacantHouse),
                    ghost(HintGhost::SwampPalaceOutsideLeft),
                    ghost(HintGhost::SwampPalaceOutsideRight),
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
                vec![ghost(HintGhost::VeteranThief)],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            FortunesChoiceLorule,
            location(
                "Fortune's Choice (Lorule)",
                vec![ghost(HintGhost::FortunesChoice)],
                vec![path_free(LoruleCastleField)],
            ),
        ),
        (
            ThievesTownItemShop,
            location(
                "Thieves' Town Item Shop",
                vec![
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Thieves' Town Item Shop (1)",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Thieves' Town Item Shop (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Thieves' Town Item Shop (3)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
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
                    check_quest_free("Obtain Big Bomb Flower", filler_item::Goal::BigBombFlower),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::graveyards::lorule::SUBREGION,
                        "Graveyard Peninsula",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
                            "[Mai] Lorule Graveyard Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
                            "[Mai] Lorule Graveyard Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
                            "[Mai] Lorule Graveyard Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::LoruleGraveyard),
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
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
                            "[LS] Entrance Chest",
                        ),
                        Some(|p| p.has_lamp() || p.lampless()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
                            "[LS] Lower Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
                            "[LS] Upper Chest",
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::graveyards::lorule::SUBREGION,
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
                    LocationInfo::new(
                        regions::dungeons::graveyards::lorule::SUBREGION,
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
                    LocationInfo::new(regions::lorule::field::main::SUBREGION, "Great Rupee Fairy"),
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
                    LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Blacksmith (Lorule)",
                    ),
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
                    LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::lorule::field::main::SUBREGION,
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
                    LocationInfo::new(regions::lorule::field::main::SUBREGION, "Thief Girl"),
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
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Swamp Cave (Left)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
                        "Swamp Cave (Middle)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::field::main::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::lorule::field::main::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
                            "Lorule Field Hookshot Chest",
                        ),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::field::main::SUBREGION,
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
                        LocationInfo::new(
                            regions::hyrule::desert::mystery::SUBREGION,
                            "[Mai] Buried in the Desert",
                        ),
                        Some(|p| p.has_sand_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::DesertEast),
                ],
                vec![
                    fast_travel_hyrule(),
                    portal_std(MiseryMire),
                    path(
                        MiseryMireLedge, // todo portal-ify
                        Some(|p| {
                            p.can_merge()
                                && p.has_bombs()
                                && (p.has_sand_rod() || p.has_stamina_scroll())
                        }),
                        None,
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
                vec![ghost(HintGhost::DesertCenter)],
                vec![path_free(Desert), portal_std(MiseryMireBridge)],
            ),
        ),
        (
            DesertSouthWestLedge,
            location(
                "Desert South West Ledge",
                vec![ghost(HintGhost::DesertSouthWest)],
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
                    LocationInfo::new(
                        regions::hyrule::desert::mystery::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::misery::mire::SUBREGION,
                            "[Mai] Misery Mire Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::misery::mire::SUBREGION,
                            "[Mai] Misery Mire Water",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::misery::mire::SUBREGION,
                            "[Mai] Misery Mire Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::MiseryMireLedge),
                    ghost(HintGhost::MiseryMireBridge),
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
                    LocationInfo::new(
                        regions::lorule::misery::mire::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::lorule::misery::mire::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::lake::lorule::SUBREGION,
                            "[Mai] Lorule Lake SE Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::lake::lorule::SUBREGION,
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
                        filler_item::Goal::TurtleWall,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::lorule::lake::lorule::SUBREGION,
                        "Lorule Lake Chest",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::lake::lorule::SUBREGION,
                            "[Mai] Lorule Lake West Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::TurtleWall),
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
                    check_quest_free("Turtle (flipped)", filler_item::Goal::TurtleFlipped),
                    check(
                        LocationInfo::new(
                            regions::lorule::lake::lorule::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::lorule::lake::lorule::SUBREGION,
                        "Lorule Lakeside Item Shop (1)",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::lorule::lake::lorule::SUBREGION,
                        "Lorule Lakeside Item Shop (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::lake::lorule::SUBREGION,
                        "Lorule Lakeside Item Shop (3)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::lake::lorule::SUBREGION,
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
                        filler_item::Goal::TurtleAttacked,
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::lorule::lake::lorule::SUBREGION,
                        "[Mai] Lorule Lake Water",
                    )),
                    ghost(HintGhost::TurtleBullied),
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
                vec![ghost(HintGhost::TurtleRockOutside)],
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
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Dark Ruins Lakeview Chest",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Dark Ruins Waterfall",
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()),
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Dark Maze Entrance Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Atop Dark Ruins Rocks",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Dark Ruins West Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Dark Ruins East Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Dark Ruins South Area Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::DarkRuinsNorth),
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
                    LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Chest"),
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
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Dark Maze Ledge",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
                            "[Mai] Dark Maze Center Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::DarkMaze),
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
                vec![ghost(HintGhost::DarkPalaceOutside)],
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
                    LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::dark::ruins::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Hinox (1)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Hinox (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Hinox (3)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Hinox (4)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
                        "Hinox (5)",
                    )),
                    check_free(LocationInfo::new(
                        regions::lorule::dark::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "Canyon House",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // portal clip through house
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::lorule::skull::overworld::SUBREGION,
                        "Destroyed House",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Skull Woods Grass",
                        ),
                        Some(|p| p.can_cut_grass()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Skull Woods Skull",
                        ),
                        Some(|p| p.can_destroy_skull()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Skull Woods Shack Tree",
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::lorule::skull::overworld::SUBREGION,
                        "[Mai] Skull Woods Bush",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Skull Woods Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Skull Woods Entrance Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Skull Woods Dry Pond",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::skull::overworld::SUBREGION,
                            "[Mai] Canyon House Wall",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::SkullWoodsCuccos),
                    ghost(HintGhost::SkullWoodsSouth),
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
                    LocationInfo::new(
                        regions::lorule::skull::overworld::SUBREGION,
                        "Mysterious Man",
                    ),
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
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "Ice Gimos Fight",
                        ),
                        Some(|p| p.can_defeat_margomill()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "Lorule Mountain W Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs()),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
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
                    check_unreachable(LocationInfo::new(
                        regions::lorule::death::mountain::SUBREGION,
                        "Treacherous Tower Advanced (1)",
                    )),
                    check_unreachable(LocationInfo::new(
                        regions::lorule::death::mountain::SUBREGION,
                        "Treacherous Tower Advanced (2)",
                    )),
                    check(
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "[Mai] Lorule Mountain W Skull",
                        ),
                        Some(|p| p.can_destroy_skull()),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "[Mai] Lorule Mountain W Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt() && p.has_hammer()),
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_nice_bombs()), // Not enough room for Fire Rod
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_bombs()),
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "[Mai] Lorule Mountain E Big Rock",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::TreacherousTower),
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
                    LocationInfo::new(
                        regions::lorule::death::mountain::SUBREGION,
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
                    LocationInfo::new(
                        regions::lorule::death::mountain::SUBREGION,
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
                    path(RossosOreMineLorule, None, None, Some(|p| p.has_nice_bombs()), None, None),
                ],
            ),
        ),
        (
            LoruleDeathEastLedgeLower,
            location(
                "Lorule Death Mountain East Lower Ledge",
                vec![check(
                    LocationInfo::new(
                        regions::lorule::death::mountain::SUBREGION,
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
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "Behind Ice Gimos",
                        ),
                        Some(|p| p.has_fire_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::lorule::death::mountain::SUBREGION,
                            "[Mai] Outside Ice Ruins",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    ghost(HintGhost::IceRuinsOutside),
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
                    LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
                            "[EP] (1F) Left Door Chest",
                        ),
                        Some(|p| p.can_hit_far_switch() || p.has_nice_ice_rod()),
                        Some(|_| true), // throw pot
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
                            "[EP] (1F) Popo Room",
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
                            "[EP] (1F) Secret Room",
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
                            "[EP] (2F) Defeat Popos",
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
                        "[EP] (2F) Ball Room",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
                            "[EP] (2F) Switch Room",
                        ),
                        Some(|p| p.can_hit_far_switch() || p.has_ice_rod()),
                        Some(|_| true), // pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::eastern::palace::SUBREGION,
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
                                && ((p.has_eastern_keys(2) && p.can_hit_far_switch())
                                    || p.has_ice_rod())
                                && p.can_attack()
                        }),
                        Some(|p| {
                            p.has_eastern_big_key()
                                && (p.has_bombs() || (p.has_eastern_keys(2) && p.has_lamp_or_net()))
                        }),
                        Some(|p| p.has_master_sword() || p.can_great_spin()),
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
                            || p.has_master_sword()
                            || ((p.has_boomerang() || p.has_hookshot())
                                && (p.can_attack() || p.has_lamp_or_net()))
                            || p.has_nice_ice_rod()
                    }),
                    None,
                    None,
                    Some(|p| p.has_ice_rod()), // gross
                )],
            ),
        ),
        (
            EasternPalacePostYuga,
            location(
                "Eastern Palace Post Yuga",
                vec![
                    check_free(LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
                        "[EP] Yuga (1)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
                        "[EP] Yuga (2)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
                        "Eastern Palace Prize",
                    )),
                    check_quest_free("Eastern Palace Complete", filler_item::Goal::Yuga),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
                        "[EP] (3F) Escape Chest",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::eastern::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
                            "[HG] (1F) Torches",
                        ),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
                            "[HG] (1F) Switch Room",
                        ),
                        Some(|p| p.can_merge()),
                        Some(|_| true), // might need to deathwarp to escape
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::house::gales::SUBREGION,
                        "[HG] (1F) West Room",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
                            "[HG] (2F) Narrow Ledge",
                        ),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        Some(|_| true), // can just grab it with TRod
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::house::gales::SUBREGION,
                        "[HG] (2F) Big Chest",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
                            "[HG] (3F) Fire Bubbles",
                        ),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::house::gales::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::house::gales::SUBREGION,
                        "[HG] Margomill",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::house::gales::SUBREGION,
                        "House of Gales Prize",
                    )),
                    check_quest_free("Margomill Defeated", filler_item::Goal::Margomill),
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
                        LocationInfo::new(
                            regions::dungeons::tower::hera::SUBREGION,
                            "[TH] (1F) Outside",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::tower::hera::SUBREGION,
                            "[TH] (1F) Center",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::tower::hera::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
                        "[TH] (5F) Red/Blue Switches",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
                        "[TH] (6F) Right Mole",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
                        "[TH] (7F) Outside (Ledge)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
                        "[TH] (8F) Fairy Room",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
                        "[TH] Moldorm",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::tower::hera::SUBREGION,
                        "Tower of Hera Prize",
                    )),
                    check_quest_free("Moldorm", filler_item::Goal::Moldorm),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (1F) Right Pit",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::dark::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (1F) Switch Puzzle",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (1F) Hidden Room (Upper)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (1F) Hidden Room (Lower)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (B1) Fall From 1F",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (B1) Helmasaur Room",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (B1) Helmasaur Room (Fall)",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::dark::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (1F) Fall From 2F",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (2F) Big Chest (Hidden)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] (2F) South Hidden Room",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::dark::palace::SUBREGION,
                            "[PD] (2F) Alcove",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "[PD] Gemesaur King",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::dark::palace::SUBREGION,
                        "Dark Palace Prize",
                    )),
                    check_quest_free("Gemesaur King", filler_item::Goal::GemesaurKing),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] (B1) Center",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] (B1) Waterfall Room",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] (B1) Raft Room (Pillar)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] (B1) Raft Room (Right)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] (B1) Raft Room (Left)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] (B1) Gyorm",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::swamp::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::swamp::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::swamp::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::swamp::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::swamp::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "[SP] Arrghus",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::swamp::palace::SUBREGION,
                        "Swamp Palace Prize",
                    )),
                    check_quest_free("Arrghus", filler_item::Goal::Arrghus),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::skull::woods::SUBREGION,
                        "[SW] (B1) South Chest",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::skull::woods::SUBREGION,
                        "[SW] (B1) Gibdo Room (Lower)",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::skull::woods::SUBREGION,
                            "[SW] (B1) Gibdo Room (Hole)",
                        ),
                        Some(|p| p.has_skull_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::skull::woods::SUBREGION,
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
                vec![check_free(LocationInfo::new(
                    regions::dungeons::skull::woods::SUBREGION,
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
                    LocationInfo::new(
                        regions::dungeons::skull::woods::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::skull::woods::SUBREGION,
                        "Skull Woods Prize",
                    )),
                    check_quest_free("Knucklemaster", filler_item::Goal::Knucklemaster),
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
                        LocationInfo::new(
                            regions::dungeons::skull::woods::SUBREGION,
                            "[SW] (B1) Big Chest (Eyes)",
                        ),
                        Some(|p| p.has_skull_eyes()),
                        None,
                        None, // Eyeball dupe cannot be considered as it cannot be retried if missed
                        None,
                        None,
                    ),
                    check_quest_free("Skull Eye Right", filler_item::Goal::SkullEyeRight),
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
                        LocationInfo::new(
                            regions::dungeons::skull::woods::SUBREGION,
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
                        filler_item::Goal::SkullEyeLeft,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::skull::woods::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::thieves::hideout::SUBREGION,
                        "[T'H] (B1) Grate Chest",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
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
                        filler_item::Goal::ThievesB1DoorOpen,
                        Some(|p| p.can_merge() && p.can_hit_switch()),
                        None,
                        Some(|p| p.has_boots() && (p.has_boomerang() || p.has_ice_rod())),
                        None,
                        Some(|p| p.has_boots() && p.has_bombs()),
                    ),
                    /* B2 */
                    check(
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
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
                        filler_item::Goal::ThievesB2DoorOpen,
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
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
                            "[T'H] (B2) Jail Cell",
                        ),
                        Some(|p| p.thieves_b1b2_doors_open() && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.can_hit_switch()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()), // reach from B3 Out of Bounds
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
                            "[T'H] (B2) Switch Puzzle Room",
                        ),
                        Some(|p| p.thieves_b1b2_doors_open()),
                        None,
                        None,
                        Some(|p| p.adv_thieves_statue_clip()),
                        Some(|p| p.hell_thieves_statue_clip()),
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
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
                        filler_item::Goal::ThievesB3WaterDrained,
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
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
                            "[T'H] (B3) Underwater",
                        ),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
                            "[T'H] (B3) Big Chest (Hidden)",
                        ),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::thieves::hideout::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::thieves::hideout::SUBREGION,
                        "Stalblind",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::thieves::hideout::SUBREGION,
                        "Thieves' Hideout Prize",
                    )),
                    check_quest_free("Stalblind Defeated", filler_item::Goal::Stalblind),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "[IR] (1F) Hidden Chest",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "[IR] (B4) Ice Pillar",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "[IR] (B3) Grate Chest (Left)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "[IR] (B3) Grate Chest (Right)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "[IR] (B5) Big Chest",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B1) Narrow Ledge",
                        ),
                        Some(|p| p.can_merge() && p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B1) East Chest",
                        ),
                        Some(|p| p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B1) Upper Chest",
                        ),
                        Some(|p| p.has_ice_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B2) Long Merge Chest",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.has_stamina_scroll()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B3) Big Chest (Puzzle)",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.can_hit_switch()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B4) Southwest Chest (Fall)",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
                            "[IR] (B4) Narrow Platform",
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::ice::ruins::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "[IR] Dharkstare",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::ice::ruins::SUBREGION,
                        "Ice Ruins Prize",
                    )),
                    check_quest_free("Dharkstare", filler_item::Goal::Dharkstare),
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
                    LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "[DP] (1F) Sand Switch Room",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "[DP] (1F) Sand Room (North)",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "[DP] (1F) Sand Room (South)",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
                            "[DP] (1F) Behind Rocks",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
                            "[DP] (2F) Under Rock (Left)",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
                            "[DP] (2F) Under Rock (Right)",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
                            "[DP] (2F) Under Rock (Ball Room)",
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "[DP] (2F) Beamos Room",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "[DP] (2F) Red/Blue Switches",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
                            "[DP] (2F) Big Chest (Puzzle)",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "[DP] (3F) Behind Falling Sand",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::desert::palace::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "Zaganaga",
                    )), // Do not use [DP] prefix
                    check_free(LocationInfo::new(
                        regions::dungeons::desert::palace::SUBREGION,
                        "Desert Palace Prize",
                    )),
                    check_quest_free("Zaganaga Defeated", filler_item::Goal::Zaganaga),
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
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
                        "[TR] (1F) Center",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (1F) Northeast Ledge",
                        ),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (1F) Southeast Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs() && p.has_tornado_rod()), // bombrod into warp tile
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (1F) Defeat Flamolas",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (1F) Portal Room NW",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (1F) Grate Chest",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
                        "[TR] (B1) Northeast Room",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (B1) Grate Chest (Small)",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None, // I swear there was a bombrod you could do here, idk, leaving it off for now
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
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
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (B1) Big Chest (Center)",
                        ),
                        Some(|p| p.can_merge() && p.can_hit_shielded_switch()),
                        Some(|p| p.can_merge()), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::turtle::rock::SUBREGION,
                            "[TR] (B1) Platform",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
                        "[TR] (1F) Under Center",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
                        "[TR] Grinexx",
                    )),
                    check_free(LocationInfo::new(
                        regions::dungeons::turtle::rock::SUBREGION,
                        "Turtle Rock Prize",
                    )),
                    check_quest_free("Grinexx", filler_item::Goal::Grinexx),
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
                vec![check_free(LocationInfo::new(
                    regions::dungeons::lorule::castle::SUBREGION,
                    "[LC] (1F) Ledge",
                ))],
                vec![path(LoruleCastle1F, Some(|p| p.can_merge()), None, None, None, None)],
            ),
        ),
        (
            LoruleCastleCenter1F,
            location(
                "Lorule Castle 1F Center",
                vec![check_free(LocationInfo::new(
                    regions::dungeons::lorule::castle::SUBREGION,
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
                    check_free(LocationInfo::new(
                        regions::dungeons::lorule::castle::SUBREGION,
                        "[LC] (2F) Near Torches",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
                            "[LC] (2F) Hidden Path",
                        ),
                        Some(|p| p.can_extinguish_torches()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
                            "[LC] (2F) Ledge",
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_boots()),
                        Some(|p| p.has_lorule_keys(3)), // drop from 4F -> 3F -> 2F
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
                            "[LC] (3F) Bomb Trial Center Chest",
                        ),
                        Some(|p| p.has_bombs()),
                        None,
                        Some(|p| p.has_ice_rod()),
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
                            "[LC] (3F) Big Bomb Flower Chest",
                        ),
                        Some(|p| p.has_bombs() && p.can_merge()),
                        Some(|p| p.has_bombs() && p.has_bow()),
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::lorule::castle::SUBREGION,
                        "[LC] (3F) Merge Trial Free Chest",
                    )),
                    check_quest(
                        "Bomb Trial",
                        filler_item::Goal::LcBombTrial,
                        Some(|p| p.has_lorule_keys(5) && p.can_hit_switch() && p.can_attack()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
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
                        filler_item::Goal::LcBallTrial,
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
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
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
                        filler_item::Goal::LcLampTrial,
                        Some(|p| p.has_lorule_keys(5) && p.has_fire_source() && p.can_attack()),
                        Some(|p| p.has_lorule_keys(5) && p.can_attack()),
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
                            "[LC] (4F) Eyeball Chest",
                        ),
                        Some(|p| p.has_hookshot() && (p.has_ice_rod() || p.can_merge())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
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
                        filler_item::Goal::LcHookTrial,
                        Some(|p| p.has_lorule_keys(5) && p.has_hookshot() && p.can_attack()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check_free(LocationInfo::new(
                        regions::dungeons::lorule::castle::SUBREGION,
                        "[LC] (4F) Center",
                    )),
                    check(
                        LocationInfo::new(
                            regions::dungeons::lorule::castle::SUBREGION,
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
                    LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "Zelda"),
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
            location(
                "Sacred Realm",
                vec![check_quest_free("Sacred Realm", filler_item::Goal::Triforce)],
                vec![],
            ),
        ),
    ])
}

fn location(name: &'static str, checks: Vec<Check>, paths: Vec<Path>) -> LocationNode {
    LocationNode::new(name, checks, paths)
}

fn check(
    location_info: LocationInfo, normal: Option<fn(&Progress) -> bool>,
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

fn check_free(location_info: LocationInfo) -> Check {
    Check::new(location_info.name, Logic::free(), None, Some(location_info))
}

fn check_unreachable(location_info: LocationInfo) -> Check {
    Check::new(
        location_info.name,
        Logic { normal: None, hard: None, glitched: None, adv_glitched: None, hell: None },
        None,
        Some(location_info),
    )
}

fn check_quest(
    name: &'static str, quest: impl Into<FillerItem>, normal: Option<fn(&Progress) -> bool>,
    hard: Option<fn(&Progress) -> bool>, glitched: Option<fn(&Progress) -> bool>,
    adv_glitched: Option<fn(&Progress) -> bool>, hell: Option<fn(&Progress) -> bool>,
) -> Check {
    Check::new(
        name,
        Logic::new(normal, hard, glitched, adv_glitched, hell),
        Some(quest.into()),
        None,
    )
}

fn check_quest_free(name: &'static str, quest: filler_item::Goal) -> Check {
    Check::new(name, Logic::free(), Some(quest.into()), None)
}

fn ghost(ghost: HintGhost) -> Check {
    Check::new(hint_ghost_name(&ghost), Logic::free(), Some(FillerItem::HintGhost(ghost)), None)
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

use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::{self, Goal};
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{
    check, edge, fast_travel_hyrule, ghost, goal, location, out_of_logic, portal_std,
};
use crate::LocationInfo;
use game::HintGhost;
use std::collections::HashMap;

/// Hyrule
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        // Starting Node
        (
            RavioShop,
            location(
                "Ravio's Shop",
                vec![
                    check!("Ravio (1)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (2)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (3)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (4)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (5)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (6)", regions::hyrule::field::main::SUBREGION, |p| p
                        .has_sage_osfala()),
                    check!("Ravio (7)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (8)", regions::hyrule::field::main::SUBREGION),
                    check!("Ravio (9)", regions::hyrule::field::main::SUBREGION),
                ],
                vec![
                    edge!(HyruleField),
                    edge!(ChamberOfSages), // not technically true but gives us what we need
                ],
            ),
        ),
        (
            ChamberOfSages,
            location(
                "Chamber of Sages",
                vec![check!("Osfala", regions::lorule::chamber::sages::SUBREGION, |p| p
                    .has_sage_osfala())],
                vec![],
            ),
        ),
        (
            HyruleBellTravel,
            location(
                "Hyrule Bell Travel",
                vec![],
                vec![
                    edge!(HyruleField),
                    edge!(DesertPalaceWeatherVane),
                    edge!(EasternRuinsUpper),
                    edge!(HouseOfGalesIsland),
                    edge!(DeathMountainBase),
                    edge!(DeathMountainWestTop),
                ],
            ),
        ),
        (
            HyruleField,
            location(
                "Hyrule Field",
                vec![
                    check!("Dampe", regions::dungeons::graveyards::hyrule::SUBREGION),
                    check!("Irene", regions::hyrule::irene::witch::SUBREGION, |p| {
                        if p.is_rse() {
                            p.has_sage_irene()
                        } else {
                            p.has_pendant_of_courage()
                        }
                    }),
                    check!(
                        "Sanctuary Pegs",
                        regions::dungeons::graveyards::hyrule::SUBREGION,
                        |p| p.has_hammer()
                    ),
                    check!(
                        "Behind Blacksmith",
                        regions::hyrule::field::main::SUBREGION => {
                            normal: |p| p.can_merge(),
                            glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                            hell: |_| true, // Bee Boosting
                        }
                    ),
                    check!("Hyrule Castle Rocks", regions::hyrule::field::main::SUBREGION, |p| p
                        .has_power_glove()),
                    check!(
                        "Wildlife Clearing Stump",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_pendant_of_courage()
                    ),
                    check!(
                        "Southern Ruins Ledge",
                        regions::hyrule::southern::ruins::SUBREGION,
                        |p| p.can_merge()
                    ),
                    // Lake Hylia
                    check!(
                        "Lake Hylia Ledge Chest",
                        regions::hyrule::lake::hylia::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "Southeastern Shore",
                        regions::hyrule::lake::hylia::SUBREGION => {
                            normal: |p| p.has_flippers(),
                            glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                            hell: |_| true, // Bee Boosting
                        }
                    ),
                    check!(
                        "Hyrule Hotfoot (First Race)",
                        regions::hyrule::lost::woods::SUBREGION => {
                            normal: |p| p.has_boots(),
                            hard: |_| true,
                        }
                    ),
                    check!(
                        "Hyrule Hotfoot (Second Race)",
                        regions::hyrule::lost::woods::SUBREGION => {
                            normal: | p | p.has_boots(),
                            hard: |p| p.can_merge() && p.has_bell(),
                            hell: |_| true, // Can just walk it
                        }
                    ),
                    check!("Bird Lover", regions::hyrule::eastern::ruins::SUBREGION, |p| p
                        .has_flippers()),
                    // Kakariko Village
                    check!("Street Merchant (Left)", regions::hyrule::kakariko::village::SUBREGION),
                    check!(
                        "Street Merchant (Right)",
                        regions::hyrule::kakariko::village::SUBREGION,
                        |p| p.has_shady_guy_trigger()
                    ),
                    check!("Shady Guy", regions::hyrule::kakariko::village::SUBREGION, |p| p
                        .has_shady_guy_trigger()
                        && (p.can_merge() || p.has_boots())),
                    check!("Dodge the Cuccos", regions::hyrule::kakariko::village::SUBREGION),
                    check!("Rupee Rush (Hyrule)", regions::hyrule::kakariko::village::SUBREGION),
                    check!("[Mai] Kakariko Bush", regions::hyrule::kakariko::village::SUBREGION),
                    check!(
                        "[Mai] Lost Woods Path Rock",
                        regions::hyrule::lost::woods::SUBREGION => {
                            normal: |p| p.has_titans_mitt() || (p.has_power_glove() && p.has_hammer()),
                            glitched: |p| {
                                p.has_power_glove() && (p.has_hookshot() || (p.has_boomerang() && p.can_escape()))
                            },
                        }
                    ),
                    check!(
                        "[Mai] Fortune-Teller Tent",
                        regions::hyrule::lost::woods::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Woman's Roof Rock",
                        regions::hyrule::kakariko::village::SUBREGION,
                        |p| p.has_power_glove()
                    ),
                    goal!("Woman Roof Maiamai", Goal::WomanRoofMaiamai, |p| p.has_power_glove()),
                    // Eastern Ruins
                    check!(
                        "Eastern Ruins Peg Circle",
                        regions::hyrule::eastern::ruins::SUBREGION => {
                            normal: |p| p.has_hammer(),
                            glitched: |p| p.has_boomerang() || p.has_hookshot(),
                            adv_glitched: |p| p.has_tornado_rod(),
                            hell: |p| p.has_sand_rod(),
                        }
                    ),
                    // Maiamai
                    check!("[Mai] Rosso Wall", regions::hyrule::lost::woods::SUBREGION, |p| p
                        .can_merge()),
                    check!("[Mai] Small Pond", regions::hyrule::lost::woods::SUBREGION, |p| p
                        .has_flippers()),
                    check!(
                        "[Mai] Sanctuary Wall",
                        regions::dungeons::graveyards::hyrule::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Tree Behind Blacksmith",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!("[Mai] Lost Woods Tree", regions::hyrule::lost::woods::SUBREGION, |p| p
                        .has_boots()),
                    check!(
                        "[Mai] Hyrule Castle Tree",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Hyrule Castle Tornado Tile",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_tornado_rod()
                    ),
                    check!(
                        "[Mai] Under Wooden Bridge",
                        regions::hyrule::zora::river::SUBREGION=> {
                            normal: |p| p.has_flippers(),
                            adv_glitched: |p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()),
                            hell: |p| p.has_boots(), // bee boost fake flippers
                        }
                    ),
                    check!(
                        "[Mai] Eastern Ruins Wall",
                        regions::hyrule::eastern::ruins::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Eastern Ruins Yellow Tree",
                        regions::hyrule::eastern::ruins::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Eastern Ruins Green Tree",
                        regions::hyrule::eastern::ruins::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Eastern Ruins Big Rock",
                        regions::hyrule::eastern::ruins::SUBREGION,
                        |p| p.can_merge() && p.has_titans_mitt()
                    ),
                    check!(
                        "[Mai] Blacksmith Tornado Tile",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_tornado_rod()
                    ),
                    check!(
                        "[Mai] Atop Eastern Rocks",
                        regions::hyrule::eastern::ruins::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Hyrule Rupee Rush Wall",
                        regions::hyrule::kakariko::village::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Cucco Ranch Tree",
                        regions::hyrule::kakariko::village::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Wildlife Clearing Tree",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Tree West of Link's House",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Behind Link's House",
                        regions::hyrule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Southern Bridge River",
                        regions::hyrule::eastern::ruins::SUBREGION => {
                            normal: |p| p.has_flippers(),
                            adv_glitched: |p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()),
                            hell: |p| p.has_boots(), // bee boost fake flippers
                        }
                    ),
                    check!(
                        "[Mai] Southern Ruins Pillars",
                        regions::hyrule::southern::ruins::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Outside Flippers Dungeon",
                        regions::hyrule::southern::ruins::SUBREGION,
                        |p| p.has_flippers()
                    ),
                    check!(
                        "[Mai] Outside Maiamai Cave",
                        regions::hyrule::lake::hylia::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Lake Hylia SE Wall",
                        regions::hyrule::lake::hylia::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Hyrule Hotfoot Big Rock",
                        regions::hyrule::lake::hylia::SUBREGION,
                        |p| p.can_merge() && p.has_titans_mitt()
                    ),
                    check!(
                        "[Mai] Southern Ruins Big Rock",
                        regions::hyrule::desert::mystery::SUBREGION,
                        |p| p.has_titans_mitt()
                    ),
                    check!(
                        "[Mai] Lake Hylia Shallow Ring",
                        regions::hyrule::lake::hylia::SUBREGION,
                        |p| p.has_flippers()
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
                    edge!(RavioShop),
                    edge!(EasternRuinsUpper => {
                        normal: |p| p.can_hit_far_switch() || p.has_ice_rod() || p.can_merge(),
                        hard: |p| p.has_power_glove(),
                    }),
                    edge!(EasternRuinsEastLedge, |p| p.has_power_glove()),
                    edge!(WitchCave, |p| p.has_bombs()),
                    edge!(ZoraDomainArea => {
                        normal: |p| p.can_merge(),
                        hell: |_| true, // Bee Boost
                    }),
                    edge!(WaterfallCaveShallowWater, |p| p.has_flippers()),
                    edge!(BlacksmithHouse),
                    edge!(BlacksmithCave => {
                        normal: |p| p.has_titans_mitt(),
                        glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                        hell: |_| true, // Bee Boost
                    }),
                    edge!(LostWoods),
                    edge!(HyruleCastleCourtyard, |p| p.has_master_sword() || p.swordless_mode()),
                    edge!(FortuneTeller),
                    edge!(KakarikoJailCell),
                    edge!(WellUpper => {
                        normal: |p| p.has_power_glove(),
                        hard: |_| true, // Cucco jump
                    }),
                    edge!(WellLower),
                    edge!(MilkBar),
                    edge!(BeeGuyHouse),
                    edge!(KakarikoItemShop),
                    edge!(LakesideItemShop),
                    edge!(ItemSellerCave, |p| p.has_bombs()),
                    edge!(FlippersDungeon => {
                        normal: |p| p.has_titans_mitt(),
                        glitched: |p| p.has_sword() && p.has_ice_rod(),
                        adv_glitched: |p| p.has_ice_rod(),
                    }),
                    edge!(SouthernRuinsBombCave, |p| p.has_bombs()),
                    edge!(LakeDarkCave),
                    edge!(IceRodCave, |p| p.has_bombs()),
                    edge!(Sanctuary, |p| p.has_sword()
                        || p.has_bombs()
                        || p.has_fire_rod()
                        || p.has_ice_rod()
                        || p.has_lamp()
                        || p.has_boots()),
                    edge!(MoldormCave => {
                        normal: |p| p.has_power_glove(),
                        glitched: |_| true, // Crow boost
                    }),
                    edge!(RossoHouse, |p| {
                        if p.is_rse() {
                            p.has_sage_rosso()
                        } else {
                            p.has_pendant_of_courage()
                        }
                    }),
                    edge!(RossoCave => {
                        normal: |p| p.has_hammer(),
                        glitched: |p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot()),
                        adv_glitched: |p| p.not_nice_mode() && (p.can_use_shield() && p.has_tornado_rod()),
                    }),
                    edge!(TornadoRodDungeon, |p| p.has_bombs()),
                    edge!(HouseOfGalesIsland => {
                        normal: |p| p.has_flippers(),
                        adv_glitched: |p| {
                            (p.has_hookshot() && p.has_ice_rod())
                                || (p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()))
                        },
                        hell: |p| p.has_boots(), // Bee Boost
                    }),
                    edge!(HauntedGroveLedge, |p| p.can_merge()),
                    edge!(LoruleLakeNorthWest, |p| p.can_merge()),
                    edge!(LoruleLakeEast, |p| p.can_merge()),
                    edge!(MiseryMire, |p| p.can_merge()),
                    edge!(SkullWoodsOverworld, |p| p.can_merge()),
                    edge!(WitchHouse),
                    edge!(SanctuaryChurch, |p| p.has_opened_sanctuary_doors()),
                    edge!(CuccoDungeonLedge, |p| p.can_merge()),
                    edge!(WaterfallLedge => {
                        normal: |p| p.has_flippers(),
                        adv_glitched: |p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()), // todo hookshot?
                        hell: |p| p.has_boots(),
                    }),
                    edge!(CuccoHouse),
                    edge!(WomanHouse),
                    edge!(StylishWomanHouse, |p| p.has_opened_stylish_womans_house()),
                    edge!(MaiamaiCave),
                ],
            ),
        ),
        (
            MaiamaiCave,
            location(
                "Mother Maiamai Cave",
                vec![
                    out_of_logic(" 10 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 20 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 30 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 40 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 50 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 60 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 70 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 80 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic(" 90 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                    out_of_logic("100 Maiamai", regions::hyrule::lake::hylia::SUBREGION),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            WomanHouse,
            location(
                "Woman's House",
                vec![check!("Woman", regions::hyrule::kakariko::village::SUBREGION, |p| p
                    .has_woman_roof_maiamai())],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            CuccoHouse,
            location("Cucco House", vec![], vec![edge!(HyruleField), edge!(CuccoHouseRear)]),
        ),
        (
            CuccoHouseRear,
            location(
                "Cucco House Rear",
                vec![check!(
                    "[Mai] Kakariko Sand",
                    regions::hyrule::kakariko::village::SUBREGION,
                    |p| p.has_sand_rod()
                )],
                vec![fast_travel_hyrule(), edge!(CuccoHouseRear)],
            ),
        ),
        (
            WaterfallLedge,
            location(
                "Waterfall Ledge",
                vec![check!(
                    "[Mai] Waterfall Ledge Wall",
                    regions::hyrule::zora::river::SUBREGION,
                    |p| p.can_merge()
                )],
                vec![
                    fast_travel_hyrule(),
                    //portal(DarkRuins), // need to make left/right system for portals, just ignore this for now
                    edge!(
                            HyruleField => {
                            normal: |p| p.has_flippers(),
                            adv_glitched: |p| p.has_hookshot(),
                    }),
                ],
            ),
        ),
        (
            CuccoDungeonLedge,
            location(
                "Cucco Dungeon Ledge",
                vec![check!(
                    "[Mai] Cucco Dungeon Big Rock",
                    regions::hyrule::field::main::SUBREGION,
                    |p| p.has_titans_mitt()
                )],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField),
                    edge!(CuccoDungeon),
                    portal_std(LoruleCastleField),
                ],
            ),
        ),
        (
            CuccoDungeon,
            location(
                "Cucco Treasure Dungeon",
                vec![check!("Cucco Treasure Dungeon", regions::hyrule::field::main::SUBREGION)],
                vec![edge!(CuccoDungeonLedge)],
            ),
        ),
        (
            WitchHouse,
            location(
                "Witch's House",
                vec![
                    goal!("Access Potion Shop", Goal::AccessPotionShop),
                    check!(
                        "[Mai] Inside Witch's House",
                        regions::hyrule::zora::river::SUBREGION,
                        |p| p.can_merge()
                    ),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            EasternRuinsUpper,
            location(
                "Eastern Ruins Upper",
                vec![
                    check!("Eastern Ruins Armos Chest", regions::hyrule::eastern::ruins::SUBREGION),
                    check!(
                        "Eastern Ruins Hookshot Chest",
                        regions::hyrule::eastern::ruins::SUBREGION,
                        |p| p.has_hookshot()
                    ),
                    check!(
                        "Eastern Ruins Merge Chest",
                        regions::hyrule::eastern::ruins::SUBREGION => {
                        normal: |p| p.can_merge(),
                        glitched: |p| p.has_tornado_rod() || p.has_fire_rod() || p.has_nice_bombs(),
                        hell: |p| p.has_bombs(),
                    }),
                    ghost(HintGhost::EasternRuinsPegs),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField),
                    edge!(EasternRuinsEastLedge => {
                        normal: |p| p.can_merge(),
                        glitched: |p| p.has_tornado_rod(), // Armos boost
                    }),
                    edge!(EasternPalaceFoyer),
                    edge!(MergeDungeon),
                    edge!(WitchCave, |p| p.has_bombs()),
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
                    edge!(EastRuinsBombCaveUpper, |p| p.has_bombs()),
                    edge!(EasternRuinsUpper, |p| p.can_merge()),
                    edge!(HyruleField),
                ],
            ),
        ),
        (
            WitchCave,
            location("Witch Cave", vec![], vec![edge!(EasternRuinsUpper), edge!(HyruleField)]),
        ),
        (
            ZoraDomain,
            location(
                "Zora's Domain",
                vec![check!("Queen Oren", regions::hyrule::zora::river::SUBREGION, |p| p
                    .has_smooth_gem()
                    && (!p.is_rse() || p.has_sage_oren()))],
                vec![edge!(ZoraDomainArea)],
            ),
        ),
        (
            ZoraDomainArea,
            location(
                "Zora's Domain Area",
                vec![
                    goal!("Shady Guy Trigger", Goal::ShadyGuyTrigger, |p| !p.is_rse()
                        || p.has_sage_oren()),
                    check!("Zora's Domain Ledge", regions::hyrule::zora::river::SUBREGION, |p| p
                        .can_merge()),
                    check!(
                        "[Mai] Zora's Domain Water",
                        regions::hyrule::zora::river::SUBREGION,
                        |p| p.has_flippers()
                    ),
                    check!(
                        "[Mai] Zora's Domain South Wall",
                        regions::hyrule::zora::river::SUBREGION,
                        |p| p.can_merge()
                    ),
                    ghost(HintGhost::ZorasDomain),
                    ghost(HintGhost::WaterfallCave),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField),
                    edge!(ZoraDomain),
                    edge!(KusDomainSouth, |p| p.can_merge()),
                    edge!(WaterfallCaveShallowWater => {
                        normal: |p| p.has_flippers(),
                        glitched: |_| true, // Crow Boost
                    }),
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
                    edge!(WaterfallCave),
                    edge!(HyruleField, |p| p.has_flippers()),
                ],
            ),
        ),
        (
            WaterfallCave,
            location(
                "Waterfall Cave",
                vec![check!("Waterfall Cave", regions::hyrule::zora::river::SUBREGION)],
                vec![edge!(WaterfallCaveShallowWater)],
            ),
        ),
        (
            MergeDungeon,
            location(
                "Eastern Ruins Treasure Dungeon",
                vec![check!(
                    "Eastern Ruins Treasure Dungeon",
                    regions::hyrule::eastern::ruins::SUBREGION,
                    |p| p.can_merge()
                )],
                vec![edge!(EasternRuinsUpper)],
            ),
        ),
        (
            EastRuinsBombCaveUpper,
            location(
                "Eastern Ruins Bomb Cave Upper",
                vec![check!(
                    "Eastern Ruins Cave",
                    regions::hyrule::eastern::ruins::SUBREGION,
                    |p| p.can_merge()
                )],
                vec![
                    edge!(EastRuinsBombCaveLower => {
                        normal: |p| p.can_merge(),
                        hard: |_| true, // It's not obvious but you can just walk
                    }),
                    edge!(EasternRuinsUpper),
                ],
            ),
        ),
        (
            EastRuinsBombCaveLower,
            location("Eastern Ruins Bomb Cave Lower", vec![], vec![edge!(HyruleField)]),
        ),
        (
            HouseOfGalesIsland,
            location(
                "House of Gales Island",
                vec![
                    check!(
                        "[Mai] Island Tornado Tile",
                        regions::hyrule::lake::hylia::SUBREGION,
                        |p| p.has_tornado_rod()
                    ),
                    ghost(HintGhost::HouseOfGalesIsland),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField, |p| p.has_flippers()),
                    edge!(HouseOfGalesFoyer, |p| p.has_tornado_rod()),
                ],
            ),
        ),
        (
            RossoHouse,
            location(
                "Rosso's House",
                vec![
                    check!("Rosso", regions::hyrule::lost::woods::SUBREGION, |p| {
                        if p.is_rse() {
                            p.has_sage_rosso()
                        } else {
                            p.has_pendant_of_courage()
                        }
                    }),
                    check!("Rosso Rocks", regions::hyrule::lost::woods::SUBREGION, |p| {
                        p.has_power_glove()
                            && if p.is_rse() {
                                p.has_sage_rosso()
                            } else {
                                p.has_pendant_of_courage()
                            }
                    }),
                ],
                vec![edge!(HyruleField), edge!(SkullWoodsOverworld, |p| p.can_merge())],
            ),
        ),
        (
            RossoCave,
            location(
                "Rosso Cave",
                vec![check!("Rosso Cave", regions::hyrule::lost::woods::SUBREGION)],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            TornadoRodDungeon,
            location(
                "Zora's River Treasure Dungeon",
                vec![check!(
                    "Zora's River Treasure Dungeon",
                    regions::hyrule::zora::river::SUBREGION,
                    |p| p.can_merge()
                )],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            GraveyardLedgeHyrule,
            location(
                "Graveyard Ledge",
                vec![check!(
                    "[Mai] Hyrule Graveyard Wall",
                    regions::dungeons::graveyards::hyrule::SUBREGION,
                    |p| p.can_merge()
                )],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField),
                    edge!(GraveyardLedgeCave),
                    portal_std(GraveyardLedgeLorule),
                ],
            ),
        ),
        (
            GraveyardLedgeCave,
            location(
                "Graveyard Ledge Cave",
                vec![check!(
                    "Graveyard Ledge Cave",
                    regions::dungeons::graveyards::hyrule::SUBREGION
                )],
                vec![edge!(GraveyardLedgeHyrule)],
            ),
        ),
        (
            BlacksmithHouse,
            location(
                "Blacksmith's House (Hyrule)",
                vec![
                    check!("Blacksmith Table", regions::hyrule::field::main::SUBREGION),
                    check!("Blacksmith", regions::hyrule::field::main::SUBREGION, |p| p
                        .has_master_ore(2)),
                    goal!("Access Hyrule Blacksmith", Goal::AccessHyruleBlacksmith),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            BlacksmithCave,
            location(
                "Blacksmith Cave",
                vec![check!("Blacksmith Cave", regions::hyrule::field::main::SUBREGION)],
                vec![edge!(HyruleField)],
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
                    edge!(HyruleCastleLeftRoom),
                    edge!(HyruleCastleRightRoom),
                    edge!(HyruleCastleInterior, |p| !p.is_rse() || p.has_sage_impa()),
                    edge!(HyruleField, |p| p.has_master_sword() || p.swordless_mode()),
                ],
            ),
        ),
        (
            HyruleCastleInterior,
            location(
                "Hyrule Castle Interior",
                vec![
                    check!("Hyrule Castle Prize", regions::dungeons::hyrule::castle::SUBREGION),
                    goal!("Zelda's Throne", Goal::ZeldasThrone),
                ],
                vec![edge!(HyruleCastleCourtyard), edge!(HyruleCastleRoof)],
            ),
        ),
        (
            HyruleCastleRightRoom,
            location("Hyrule Castle Right Room", vec![], vec![edge!(HyruleCastleCourtyard)]),
        ),
        (
            HyruleCastleLeftRoom,
            location(
                "Hyrule Castle Left Room",
                vec![check!(
                    "Hyrule Castle West Wing",
                    regions::dungeons::hyrule::castle::SUBREGION
                )],
                vec![edge!(HyruleCastleCourtyard)],
            ),
        ),
        (
            HyruleCastleRoof,
            location(
                "Hyrule Castle Roof",
                vec![check!(
                    "Hyrule Castle Battlement",
                    regions::dungeons::hyrule::castle::SUBREGION
                )],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField),
                    edge!(HyruleCastleCourtyard),
                    edge!(HyruleCastleInterior),
                    edge!(HyruleCastleDungeon, |p| p.hc_is_open() && p.has_pendant_of_courage()),
                ],
            ),
        ),
        (
            LostWoods,
            location(
                "Lost Woods",
                vec![
                    check!("Lost Woods Alcove", regions::hyrule::lost::woods::SUBREGION => {
                        normal: |p| p.can_merge(),
                        glitched: |p| p.can_escape() && (p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())),
                        hell: |p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot()), // Use Crow to escape
                    }),
                    check!("Lost Woods Big Rock Chest", regions::hyrule::lost::woods::SUBREGION => {
                        normal: |p| p.has_titans_mitt(),
                        hell: |p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot()), // Use Crow to escape
                    }),
                    check!("[Mai] Lost Woods Bush", regions::hyrule::lost::woods::SUBREGION),
                    check!("[Mai] Lost Woods Rock", regions::hyrule::lost::woods::SUBREGION, |p| p
                        .has_power_glove()),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(HyruleField),
                    edge!(MasterSwordArea, |p| p.has_required_pendants()),
                ],
            ),
        ),
        (
            MasterSwordArea,
            location(
                "Master Sword Area",
                vec![check!("Master Sword Pedestal", regions::hyrule::lost::woods::SUBREGION)],
                vec![fast_travel_hyrule(), edge!(LostWoods)],
            ),
        ),
        (
            FortuneTeller,
            location(
                "Fortune-Teller (Hyrule)",
                vec![check!("Fortune-Teller", regions::hyrule::lost::woods::SUBREGION)],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            KakarikoJailCell,
            location(
                "Kakariko Jail Cell",
                vec![check!("Kakariko Jail", regions::hyrule::kakariko::village::SUBREGION, |p| p
                    .can_merge())],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            WellUpper,
            location(
                "Kakariko Well Upper",
                vec![check!("Kakariko Well (Top)", regions::hyrule::kakariko::village::SUBREGION)],
                vec![edge!(WellLower)],
            ),
        ),
        (
            WellLower,
            location(
                "Kakariko Well Lower",
                vec![check!(
                    "Kakariko Well (Bottom)",
                    regions::hyrule::kakariko::village::SUBREGION
                )],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            StylishWomanHouse,
            location(
                "Stylish Woman's House",
                vec![
                    check!("Stylish Woman", regions::hyrule::kakariko::village::SUBREGION),
                    goal!("Open Stylish Woman's House", Goal::StylishWomansHouseOpen),
                ],
                vec![portal_std(LoruleCastleField), edge!(HyruleField)],
            ),
        ),
        (
            MilkBar,
            location(
                "Milk Bar",
                vec![goal!("Access Milk Bar", Goal::AccessMilkBar)],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            BeeGuyHouse,
            location(
                "Bee Guy's House",
                vec![
                    check!("Bee Guy (1)", regions::hyrule::kakariko::village::SUBREGION, |p| p
                        .has_bottle()),
                    check!("Bee Guy (2)", regions::hyrule::kakariko::village::SUBREGION => {
                        normal: |p| p.has_bottle() && p.has_gold_bee(),
                        hell: |p| p.has_bottle() && p.has_net(),
                    }),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            KakarikoItemShop,
            location(
                "Kakariko Item Shop",
                vec![
                    check!("Kakariko Item Shop (1)", regions::hyrule::kakariko::village::SUBREGION),
                    check!("Kakariko Item Shop (2)", regions::hyrule::kakariko::village::SUBREGION),
                    check!("Kakariko Item Shop (3)", regions::hyrule::kakariko::village::SUBREGION),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            LakesideItemShop,
            location(
                "Lakeside Item Shop",
                vec![
                    check!("Lakeside Item Shop (1)", regions::hyrule::lake::hylia::SUBREGION),
                    check!("Lakeside Item Shop (2)", regions::hyrule::lake::hylia::SUBREGION),
                    check!("Lakeside Item Shop (3)", regions::hyrule::lake::hylia::SUBREGION),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            ItemSellerCave,
            location(
                "Runaway Item-Seller Cave",
                vec![check!(
                    "Runaway Item Seller",
                    regions::hyrule::southern::ruins::SUBREGION,
                    |p| p.has_scoot_fruit()
                )],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            FlippersDungeon,
            location(
                "Southern Ruins Treasure Dungeon",
                vec![
                    check!("Southern Ruins Treasure Dungeon", regions::hyrule::southern::ruins::SUBREGION => {
                        normal: |p| p.has_boomerang() && p.has_hookshot() && p.has_flippers(),
                        hard: |p| p.has_hookshot() && p.has_flippers() && (p.has_master_sword() || p.has_bombs()),
                        glitched: |p| p.has_nice_bombs() || p.has_nice_ice_rod() || p.can_great_spin(),
                    }),
                ],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            SouthernRuinsBombCave,
            location(
                "Southern Ruins Bomb Cave",
                vec![check!(
                    "[Mai] Southern Ruins Bomb Cave",
                    regions::hyrule::southern::ruins::SUBREGION,
                    |p| p.has_flippers()
                )],
                vec![edge!(HyruleField), edge!(SouthernRuinsPillars)],
            ),
        ),
        (
            SouthernRuinsPillars,
            location(
                "Southern Ruins Pillars",
                vec![check!(
                    "Southern Ruins Pillar Cave",
                    regions::hyrule::southern::ruins::SUBREGION
                )],
                vec![fast_travel_hyrule(), edge!(SouthernRuinsBombCave)],
            ),
        ),
        (
            LakeDarkCave,
            location(
                "Lake Hylia Dark Cave",
                vec![check!(
                    "Lake Hylia Dark Cave",
                    regions::hyrule::lake::hylia::SUBREGION,
                    |p| p.has_fire_source()
                )],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            IceRodCave,
            location(
                "Ice Rod Cave",
                vec![check!("Ice Rod Cave", regions::hyrule::lake::hylia::SUBREGION)],
                vec![edge!(HyruleField)],
            ),
        ),
        (
            SanctuaryChurch,
            location(
                "Sanctuary Church",
                vec![],
                vec![
                    portal_std(LoruleSanctuaryCaveLower),
                    edge!(HyruleField, |p| p.has_opened_sanctuary_doors()),
                ],
            ),
        ),
        (
            Sanctuary,
            location(
                "Sanctuary",
                vec![
                    check!("[HS] Entrance", regions::dungeons::graveyards::hyrule::SUBREGION),
                    check!(
                        "[HS] Lower Chest",
                        regions::dungeons::graveyards::hyrule::SUBREGION,
                        |p| p.has_lamp() || (p.has_fire_rod() && p.lampless())
                    ),
                    check!(
                        "[HS] Upper Chest",
                        regions::dungeons::graveyards::hyrule::SUBREGION,
                        |p| p.has_lamp() || (p.has_fire_rod() && p.lampless())
                    ),
                    check!("[HS] Ledge", regions::dungeons::graveyards::hyrule::SUBREGION, |p| {
                        p.can_merge() && (p.has_lamp() || (p.has_fire_rod() && p.lampless()))
                    }),
                    goal!("Open Sanctuary Doors", Goal::OpenSanctuaryDoors => {
                        normal: |p| (p.has_lamp() || (p.has_fire_rod() && p.lampless())) && p.can_attack() && p.has_sanctuary_key(),
                        hard: |p| p.has_lamp() && p.has_sanctuary_key(),
                    }),
                ],
                vec![
                    edge!(HyruleField),
                    edge!(SanctuaryChurch => {
                        normal: |p| (p.has_lamp() || (p.has_fire_rod() && p.lampless())) && p.can_attack() && p.has_sanctuary_key(),
                        hard: |p| p.has_lamp() && p.has_sanctuary_key(),
                    }),
                ],
            ),
        ),
        (
            MoldormCave,
            location(
                "Moldorm Cave",
                vec![],
                vec![
                    edge!(HyruleField),
                    edge!(MoldormCaveTop, |p| p.has_titans_mitt()),
                    edge!(DeathMountainBase),
                ],
            ),
        ),
        (
            MoldormCaveTop,
            location(
                "Moldorm Cave Top",
                vec![],
                vec![edge!(MoldormLedge), edge!(MoldormCave, |p| p.has_titans_mitt())],
            ),
        ),
        (
            MoldormLedge,
            location(
                "Moldorm Ledge",
                vec![check!("[Mai] Moldorm Ledge", regions::hyrule::lost::woods::SUBREGION, |p| p
                    .can_merge())],
                vec![fast_travel_hyrule(), edge!(MoldormCaveTop), edge!(HyruleField)],
            ),
        ),
        (
            DeathMountainBase,
            location(
                "Death Mountain Base",
                vec![check!(
                    "[Mai] Death Mountain Base Rock",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.has_power_glove()
                )],
                vec![
                    fast_travel_hyrule(),
                    edge!(MoldormCave),
                    edge!(DeathBombCave, |p| p.can_merge() && p.has_bombs()),
                    edge!(DeathWeatherVaneCaveLeft),
                    edge!(DeathFairyCave, |p| p.can_merge()),
                    edge!(DonkeyCaveLower),
                    portal_std(LoruleDeathWest),
                ],
            ),
        ),
        (
            DeathBombCave,
            location(
                "Death Mountain Blocked Cave",
                vec![check!(
                    "Death Mountain Blocked Cave",
                    regions::hyrule::death::mountain::SUBREGION
                )],
                vec![edge!(DeathMountainBase)],
            ),
        ),
        (
            DeathWeatherVaneCaveLeft,
            location(
                "Death Mountain Cave Left of Weather Vane",
                vec![check!(
                    "Death Mountain Open Cave",
                    regions::hyrule::death::mountain::SUBREGION
                )],
                vec![edge!(DeathMountainBase)],
            ),
        ),
        (
            DeathFairyCave,
            location(
                "Death Mountain Fairy Cave",
                vec![check!(
                    "Death Mountain Fairy Cave",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.has_hammer() || p.has_bombs()
                )],
                vec![edge!(DeathMountainBase)],
            ),
        ),
        (
            DonkeyCaveLower,
            location(
                "Donkey Cave Lower",
                vec![],
                vec![
                    edge!(DeathMountainBase),
                    edge!(DonkeyCaveUpper => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.can_get_potion() || p.has_mail(),
                    }),
                ],
            ),
        ),
        (
            DonkeyCaveUpper,
            location(
                "Donkey Cave Upper",
                vec![check!(
                    "Donkey Cave Pegs",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.has_hammer()
                )],
                vec![
                    edge!(DonkeyCaveLower => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.can_get_potion() || p.has_mail(),
                    }),
                    edge!(DeathWestLedge),
                    edge!(DeathSecondFloor),
                ],
            ),
        ),
        (
            DeathWestLedge,
            location(
                "Death Mountain West Ledge",
                vec![
                    check!(
                        "Death Mountain West Ledge",
                        regions::hyrule::death::mountain::SUBREGION
                    ),
                    check!(
                        "[Mai] Death Mountain West Ledge",
                        regions::hyrule::death::mountain::SUBREGION,
                        |p| p.can_merge()
                    ),
                ],
                vec![fast_travel_hyrule(), edge!(DonkeyCaveUpper), edge!(DeathSecondFloor)],
            ),
        ),
        (
            DeathSecondFloor,
            location(
                "Death Mountain Second Floor",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    edge!(DonkeyCaveUpper),
                    edge!(AmidaCaveLower),
                    edge!(DeathMountainBase),
                    edge!(DeathFairyCave => {
                        glitched: |p| p.has_fire_rod() || p.has_nice_bombs() || p.has_boomerang() || p.has_hookshot(),
                        hell: |p| p.has_bombs(),
                    }),
                    edge!(DeathBombCave => {
                        glitched: |p| p.has_bombs() && (p.has_boomerang() || p.has_hookshot()),
                    }),
                ],
            ),
        ),
        (
            AmidaCaveLower,
            location(
                "Amida Cave Lower",
                vec![],
                vec![
                    edge!(DeathSecondFloor),
                    edge!(DeathThirdFloor),
                    edge!(AmidaCaveUpper => {
                        glitched: |p| p.has_boots(),
                    }),
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
                    edge!(AmidaCaveLower),
                    edge!(AmidaCaveUpper),
                    edge!(DeathSecondFloor),
                    edge!(DeathWestLedge => {
                        glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                    }),
                ],
            ),
        ),
        (
            AmidaCaveUpper,
            location(
                "Amida Cave Upper",
                vec![check!(
                    "Death Mountain West Highest Cave",
                    regions::hyrule::death::mountain::SUBREGION
                )],
                vec![edge!(AmidaCaveLower), edge!(DeathThirdFloor), edge!(DeathTopLeftLedge)],
            ),
        ),
        (
            DeathTopLeftLedge,
            location(
                "Death Mountain West Top Left Ledge",
                vec![ghost(HintGhost::SpectacleRock)],
                vec![
                    fast_travel_hyrule(),
                    edge!(AmidaCaveUpper),
                    edge!(DeathThirdFloor),
                    edge!(SpectacleRock),
                    edge!(DeathMountainWestTop, |p| p.can_merge()),
                ],
            ),
        ),
        (
            SpectacleRock,
            location(
                "Spectacle Rock",
                vec![check!("Spectacle Rock", regions::hyrule::death::mountain::SUBREGION)],
                vec![fast_travel_hyrule(), edge!(DeathThirdFloor), edge!(SpectacleRockCaveLeft)],
            ),
        ),
        (
            SpectacleRockCaveLeft,
            location(
                "Spectacle Rock Cave Left",
                vec![],
                vec![edge!(SpectacleRock), edge!(SpectacleRockCaveRight)],
            ),
        ),
        (
            SpectacleRockCaveRight,
            location("Spectacle Rock Cave Right", vec![], vec![edge!(DeathMountainWestTop)]),
        ),
        (
            DeathMountainWestTop,
            location(
                "Death Mountain West Top",
                vec![ghost(HintGhost::TowerOfHeraOutside)],
                vec![
                    fast_travel_hyrule(),
                    edge!(SpectacleRockCaveRight),
                    edge!(TowerOfHeraFoyer, |p| p.has_hammer()),
                    edge!(DeathTopLeftLedge, |p| p.can_merge()),
                    edge!(SpectacleRock),
                    edge!(DeathThirdFloor),
                    edge!(DeathMountainEastTop, |p| p.has_hookshot()),
                ],
            ),
        ),
        (
            DeathMountainEastTop,
            location(
                "Death Mountain East Top",
                vec![
                    check!(
                        "[Mai] Outside Hookshot Dungeon",
                        regions::hyrule::death::mountain::SUBREGION,
                        |p| p.can_merge()
                    ),
                    ghost(HintGhost::FloatingIsland),
                    ghost(HintGhost::FireCave),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(DeathMountainWestTop, |p| p.has_hookshot()),
                    edge!(FireCaveTop),
                    edge!(HookshotDungeon),
                    edge!(BoulderingLedgeRight => {
                        glitched: |p| p.has_tornado_rod() && p.has_boots(),
                    }),
                    edge!(RossosOreMine => {
                        glitched: |p| p.has_tornado_rod() && p.has_boots(),
                    }),
                ],
            ),
        ),
        (
            HookshotDungeon,
            location(
                "Death Mountain Treasure Dungeon",
                vec![check!(
                    "Death Mountain Treasure Dungeon",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.can_merge() && p.has_hookshot()
                )],
                vec![edge!(DeathMountainEastTop)],
            ),
        ),
        (
            FireCaveTop,
            location(
                "Fire Cave Top",
                vec![],
                vec![edge!(DeathMountainEastTop), edge!(FireCaveCenter)],
            ),
        ),
        (
            FireCaveCenter,
            location(
                "Fire Cave Center",
                vec![check!(
                    "Fire Cave Pillar",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.can_merge() && p.has_hammer()
                )],
                vec![
                    edge!(FireCaveMiddle, |p| p.can_merge()),
                    edge!(FireCaveBottom, |p| p.can_merge()),
                ],
            ),
        ),
        (
            FireCaveMiddle,
            location(
                "Fire Cave Middle",
                vec![],
                vec![
                    edge!(FireCaveCenter, |p| p.can_merge()),
                    edge!(BoulderingLedgeLeft),
                    edge!(BoulderingLedgeBottom),
                ],
            ),
        ),
        (
            FireCaveBottom,
            location("Fire Cave Bottom", vec![], vec![edge!(RossosOreMine), edge!(FireCaveTop)]),
        ),
        (
            BoulderingLedgeLeft,
            location(
                "Bouldering Guy Left Ledge",
                vec![],
                vec![
                    fast_travel_hyrule(),
                    edge!(FireCaveMiddle),
                    edge!(BoulderingLedgeRight, |p| p.can_merge()),
                    edge!(BoulderingLedgeBottom),
                    edge!(RossosOreMine => {
                        glitched: |p| p.has_nice_bombs(),
                    }),
                ],
            ),
        ),
        (
            BoulderingLedgeBottom,
            location(
                "Bouldering Guy Bottom Ledge",
                vec![check!(
                    "[Mai] Death Mountain East Ledge",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.has_power_glove()
                )],
                vec![fast_travel_hyrule(), edge!(FireCaveMiddle)],
            ),
        ),
        (
            BoulderingLedgeRight,
            location(
                "Bouldering Guy Right Ledge",
                vec![
                    check!("Bouldering Guy", regions::hyrule::death::mountain::SUBREGION, |p| {
                        p.has_premium_milk()
                            || (p.has_letter_in_a_bottle() && p.can_access_milk_bar())
                    }),
                    goal!("Bouldering Guy's Trash", filler_item::Item::Bottle05, |p| {
                        p.has_premium_milk()
                            || (p.has_letter_in_a_bottle() && p.can_access_milk_bar())
                    }),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(BoulderingLedgeBottom),
                    edge!(BoulderingLedgeLeft, |p| p.can_merge()),
                    edge!(RossosOreMine => {
                        glitched: |p| p.has_nice_bombs(),
                    }),
                ],
            ),
        ),
        (
            RossosOreMine,
            location(
                "Rosso's Ore Mine",
                vec![check!(
                    "[Mai] Rosso's Ore Mine Rock",
                    regions::hyrule::death::mountain::SUBREGION,
                    |p| p.has_power_glove()
                )],
                vec![fast_travel_hyrule(), edge!(FireCaveBottom), portal_std(RossosOreMineLorule)],
            ),
        ),
        (
            FloatingIslandHyrule,
            location(
                "Hyrule Floating Island",
                vec![check!("Floating Island", regions::hyrule::death::mountain::SUBREGION)],
                vec![fast_travel_hyrule(), portal_std(FloatingIslandLorule)],
            ),
        ),
    ])
}
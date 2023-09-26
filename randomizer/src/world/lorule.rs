use std::collections::HashMap;

use game::HintGhost;

use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{
    check, edge, fast_travel_hyrule, fast_travel_lorule, ghost, goal, location, old_check,
    old_path, out_of_logic, portal_std,
};
use crate::LocationInfo;

/// Lorule
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            LoruleBellTravel,
            location(
                "Lorule Bell Travel",
                vec![],
                vec![
                    edge!(LoruleCastleField),
                    edge!(SkullWoodsOverworld),
                    edge!(MiseryMire),
                    edge!(SwampPalaceOutside),
                    edge!(LoruleDeathWest),
                    edge!(LoruleGraveyard),
                    edge!(RossosOreMineLorule),
                    edge!(TurtleRockWeatherVane),
                    edge!(LoruleDeathEastTop),
                    edge!(DarkRuins),
                ],
            ),
        ),
        (
            LoruleCastleField,
            location(
                "Lorule Castle Field",
                vec![
                    check!("Rupee Rush (Lorule)", regions::lorule::field::main::SUBREGION),
                    check!("Octoball Derby", regions::lorule::field::main::SUBREGION),
                    goal!("Access Hilda Barrier", Goal::AccessLoruleCastleField),
                    check!("Fortune's Choice", regions::lorule::field::main::SUBREGION),
                    check!(
                        "[Mai] Lorule Castle Wall",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Lorule Castle Tree",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Thieves' Town Wall",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Near Lorule Fortune-Teller",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.has_titans_mitt()
                    ),
                    check!(
                        "[Mai] Lorule Blacksmith Wall",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Lorule Rupee Rush Wall",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!("[Mai] Octoball Derby Skull", regions::lorule::field::main::SUBREGION => {
                        normal: |p| p.can_destroy_skull(),
                        hard: |_| true, // throw bush at skull
                    }),
                    check!(
                        "[Mai] Vacant House Big Rock",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.has_titans_mitt()
                    ),
                    check!(
                        "[Mai] Behind Vacant House",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Lorule S Ruins Pillars",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.has_boots()
                    ),
                    check!(
                        "[Mai] Lorule S Ruins Wall",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.can_merge()
                    ),
                    check!(
                        "[Mai] Lorule S Ruins Water",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.has_flippers()
                    ),
                    check!(
                        "[Mai] Thieves' Town Tree",
                        regions::lorule::field::main::SUBREGION,
                        |p| p.has_boots()
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
                    edge!(GreatRupeeFairyCave, |p| p.has_bomb_flower()),
                    edge!(LoruleBlacksmith),
                    edge!(BootsDungeon),
                    edge!(VacantHouseBottom),
                    edge!(VacantHouseTop => {
                        normal: |p| p.has_bombs(),
                        hard: |p| p.has_bomb_flower(),
                    }),
                    edge!(ThiefGirlCave),
                    edge!(SwampCave => {
                        normal: |p| p.has_bomb_flower(),
                        glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                        adv_glitched: |p| p.has_stamina_scroll() && p.has_tornado_rod(),
                        hell: |_| true, // Bee Boosting
                    }),
                    edge!(BigBombCave, |p| p.has_bomb_flower()),
                    old_path(
                        SwampPalaceOutside,
                        Some(|p| p.has_hookshot()), // cannot consider flippers as water may be drained
                        None,
                        None,
                        None,
                        None,
                    ),
                    edge!(ThievesHideoutB1),
                    old_path(
                        LoruleCastle1F,
                        Some(|p| p.has_lc_requirement()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    portal_std(StylishWomanHouse),
                    edge!(BigBombFlowerShop),
                    old_path(
                        BigBombFlowerField,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                    ),
                    portal_std(CuccoDungeonLedge),
                    edge!(ThievesTownItemShop),
                    edge!(VeteranThiefsHouse),
                    edge!(FortunesChoiceLorule),
                ],
            ),
        ),
        (
            VeteranThiefsHouse,
            location(
                "Veteran Thief's House",
                vec![ghost(HintGhost::VeteranThief)],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            FortunesChoiceLorule,
            location(
                "Fortune's Choice (Lorule)",
                vec![ghost(HintGhost::FortunesChoice)],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            ThievesTownItemShop,
            location(
                "Thieves' Town Item Shop",
                vec![
                    check!("Thieves' Town Item Shop (1)", regions::lorule::field::main::SUBREGION),
                    out_of_logic(
                        "Thieves' Town Item Shop (2)",
                        regions::lorule::field::main::SUBREGION,
                    ),
                    check!("Thieves' Town Item Shop (3)", regions::lorule::field::main::SUBREGION),
                    check!("Thieves' Town Item Shop (4)", regions::lorule::field::main::SUBREGION),
                ],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            BigBombFlowerShop,
            location(
                "Big Bomb Flower Shop",
                vec![],
                vec![edge!(LoruleCastleField), edge!(BigBombFlowerField)],
            ),
        ),
        (
            BigBombFlowerField,
            location(
                "Big Bomb Flower Field",
                vec![
                    goal!("Obtain Big Bomb Flower", Goal::BigBombFlower),
                    check!("[Mai] Big Bomb Flower Grass", regions::lorule::field::main::SUBREGION),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(BigBombFlowerShop),
                    old_path(
                        LoruleCastleField,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            LoruleGraveyard,
            location(
                "Lorule Graveyard",
                vec![
                    check!("Graveyard Peninsula", regions::dungeons::graveyards::lorule::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Graveyard Big Rock",
                            regions::dungeons::graveyards::lorule::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Graveyard Wall",
                            regions::dungeons::graveyards::lorule::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Graveyard Tree",
                            regions::dungeons::graveyards::lorule::SUBREGION,
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
                    edge!(LoruleSanctuaryCaveLower),
                    old_path(
                        LoruleSanctuary,
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
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
                    old_path(GraveyardLedgeLorule, Some(|p| p.has_bombs()), None, None, None, None),
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
                    edge!(LoruleGraveyard),
                ],
            ),
        ),
        (
            LoruleSanctuary,
            location(
                "Lorule Sanctuary",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[LS] Entrance Chest",
                            regions::dungeons::graveyards::lorule::SUBREGION,
                        ),
                        Some(|p| p.has_lamp() || p.lampless()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LS] Lower Chest",
                            regions::dungeons::graveyards::lorule::SUBREGION,
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LS] Upper Chest",
                            regions::dungeons::graveyards::lorule::SUBREGION,
                        ),
                        Some(|p| p.has_lamp() || (p.has_fire_rod() && p.lampless())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LS] Ledge",
                            regions::dungeons::graveyards::lorule::SUBREGION,
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
                    edge!(LoruleGraveyard),
                    old_path(
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
                vec![portal_std(SanctuaryChurch), edge!(LoruleGraveyard)],
            ),
        ),
        (
            LoruleSanctuaryCaveUpper,
            location(
                "Philosopher's Cave Upper",
                vec![old_check(
                    LocationInfo::new(
                        "Philosopher's Cave",
                        regions::dungeons::graveyards::lorule::SUBREGION,
                    ),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![edge!(LoruleSanctuary), edge!(LoruleSanctuaryCaveLower)],
            ),
        ),
        (
            GreatRupeeFairyCave,
            location(
                "Great Rupee Fairy Cave",
                vec![old_check(
                    LocationInfo::new("Great Rupee Fairy", regions::lorule::field::main::SUBREGION),
                    Some(|p| p.has_rupees(4000)), // Actual requirement is 3000 but higher threshold helps prevent rupee grinds
                    None,
                    None,
                    None,
                    Some(|_| true), // suffer lol
                )],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            LoruleBlacksmith,
            location(
                "Lorule Blacksmith",
                vec![old_check(
                    LocationInfo::new(
                        "Blacksmith (Lorule)",
                        regions::lorule::field::main::SUBREGION,
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
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            BootsDungeon,
            location(
                "Lorule Field Treasure Dungeon",
                vec![old_check(
                    LocationInfo::new(
                        "Lorule Field Treasure Dungeon",
                        regions::lorule::field::main::SUBREGION,
                    ),
                    Some(|p| p.has_boots()),
                    Some(|p| p.has_master_sword() || p.has_bombs() || p.has_boomerang()), // we're not set up for Nice Ice Rod or Nice Bow yet...
                    None,
                    None,
                    None,
                )],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            VacantHouseBottom,
            location("Vacant House (Bottom)", vec![], vec![edge!(LoruleCastleField)]),
        ),
        (
            VacantHouseTop,
            location(
                "Vacant House (Top)",
                vec![check!("Vacant House", regions::lorule::field::main::SUBREGION)],
                vec![old_path(
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
                vec![old_check(
                    LocationInfo::new("Thief Girl", regions::lorule::field::main::SUBREGION),
                    Some(|p| p.has_sage_osfala()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            SwampCave,
            location(
                "Swamp Cave",
                vec![
                    check!("Swamp Cave (Left)", regions::lorule::field::main::SUBREGION),
                    check!("Swamp Cave (Middle)", regions::lorule::field::main::SUBREGION),
                    check!("Swamp Cave (Right)", regions::lorule::field::main::SUBREGION),
                ],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            BigBombCave,
            location(
                "Haunted Grove Big Bomb Cave",
                vec![check!("Big Bomb Flower Cave", regions::lorule::field::main::SUBREGION)],
                vec![edge!(LoruleCastleField)],
            ),
        ),
        (
            HauntedGroveLedge,
            location(
                "Haunted Grove Upper Ledge",
                vec![
                    old_check(
                        LocationInfo::new(
                            "Lorule Field Hookshot Chest",
                            regions::lorule::field::main::SUBREGION,
                        ),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Haunted Grove Wall",
                            regions::lorule::field::main::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![fast_travel_lorule(), edge!(LoruleCastleField), portal_std(HyruleField)],
            ),
        ),
        // Desert / Misery Mire
        (
            Desert,
            location(
                "Desert",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[Mai] Buried in the Desert",
                            regions::hyrule::desert::mystery::SUBREGION,
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
                    old_path(
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
                    old_path(DesertCenterLedge, Some(|p| p.has_sand_rod()), None, None, None, None),
                    old_path(
                        DesertSouthWestLedge,
                        None,
                        Some(|p| p.has_stamina_scroll()),
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        Some(|_| true), // vulture boost
                        None,
                    ),
                    old_path(
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
                vec![edge!(Desert), portal_std(MiseryMireBridge)],
            ),
        ),
        (
            DesertSouthWestLedge,
            location(
                "Desert South West Ledge",
                vec![ghost(HintGhost::DesertSouthWest)],
                vec![
                    fast_travel_hyrule(),
                    edge!(Desert),
                    portal_std(MiseryMireBridge),
                    old_path(
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
                vec![old_check(
                    LocationInfo::new(
                        "[Mai] Buried near Desert Palace",
                        regions::hyrule::desert::mystery::SUBREGION,
                    ),
                    Some(|p| p.has_sand_rod()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_hyrule(),
                    edge!(Desert),
                    old_path(DesertPalaceFoyer, Some(|p| p.has_sand_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            MiseryMire,
            location(
                "Misery Mire",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[Mai] Misery Mire Wall",
                            regions::lorule::misery::mire::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Misery Mire Water",
                            regions::lorule::misery::mire::SUBREGION,
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Misery Mire Big Rock",
                            regions::lorule::misery::mire::SUBREGION,
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
                    edge!(SandRodDungeon),
                    portal_std(Desert),
                    old_path(
                        MiseryMireOoB,
                        None,
                        None,
                        None,
                        Some(|p| p.has_nice_bombs()), // double lemon boost
                        Some(|p| p.has_bombs()),      // awful version
                    ),
                    old_path(
                        MiseryMireBridge,
                        None,
                        None,
                        None,
                        Some(|p| p.has_ice_rod() && p.has_tornado_rod()),
                        None,
                    ),
                    old_path(
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
                    edge!(MiseryMire),
                    portal_std(DesertCenterLedge),
                    portal_std(DesertSouthWestLedge),
                    old_path(
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
                    edge!(MiseryMire),
                    edge!(MiseryMireBridge),
                    portal_std(DesertZaganagaLedge),
                    edge!(ZaganagasArena),
                    old_path(
                        MiseryMireRewardBasket,
                        None,
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                ],
            ),
        ),
        (
            SandRodDungeon,
            location(
                "Misery Mire Treasure Dungeon",
                vec![old_check(
                    LocationInfo::new(
                        "Misery Mire Treasure Dungeon",
                        regions::lorule::misery::mire::SUBREGION,
                    ),
                    Some(|p| p.has_sand_rod() && p.has_tornado_rod()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![edge!(MiseryMire)],
            ),
        ),
        (
            MiseryMireLedge,
            location(
                "Misery Mire Ledge",
                vec![check!("Misery Mire Ledge", regions::lorule::misery::mire::SUBREGION)],
                vec![fast_travel_lorule(), edge!(MiseryMire)],
            ),
        ),
        // Lorule Lake Area
        (
            LoruleLakeEast,
            location(
                "Lorule Lake East",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Lake SE Wall",
                            regions::lorule::lake::lorule::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Lake Skull",
                            regions::lorule::lake::lorule::SUBREGION,
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
                    old_path(
                        LoruleLakeWater,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        None,
                    ),
                    old_path(
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
                    goal!("Turtle (wall)", Goal::TurtleWall, |p| p.can_merge()),
                    check!("Lorule Lake Chest", regions::lorule::lake::lorule::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Lake West Wall",
                            regions::lorule::lake::lorule::SUBREGION,
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
                    edge!(LoruleLakesideItemShop),
                    old_path(LoruleLakeSouthWest, Some(|p| p.can_merge()), None, None, None, None),
                    old_path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            LoruleLakeSouthWest,
            location(
                "Lorule Lake South West",
                vec![
                    goal!("Turtle (flipped)", Goal::TurtleFlipped),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Lake Big Rock",
                            regions::lorule::lake::lorule::SUBREGION,
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
                    old_path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            LoruleLakesideItemShop,
            location(
                "Lorule Lakeside Item Shop",
                vec![
                    check!(
                        "Lorule Lakeside Item Shop (1)",
                        regions::lorule::lake::lorule::SUBREGION
                    ),
                    out_of_logic(
                        "Lorule Lakeside Item Shop (2)",
                        regions::lorule::lake::lorule::SUBREGION,
                    ),
                    check!(
                        "Lorule Lakeside Item Shop (3)",
                        regions::lorule::lake::lorule::SUBREGION
                    ),
                    check!(
                        "Lorule Lakeside Item Shop (4)",
                        regions::lorule::lake::lorule::SUBREGION
                    ),
                ],
                vec![edge!(LoruleLakeNorthWest)],
            ),
        ),
        // This location assumes the player is already swimming, real or fake
        (
            LoruleLakeWater,
            location(
                "Lorule Lake Water",
                vec![
                    goal!("Turtle (under attack)",Goal::TurtleAttacked => {
                        normal: |p| p.can_attack(),
                        hard: |p| p.has_lamp_or_net(),
                    }),
                    check!("[Mai] Lorule Lake Water", regions::lorule::lake::lorule::SUBREGION),
                    ghost(HintGhost::TurtleBullied),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(LoruleLakeNorthWest),
                    edge!(LoruleLakeSouthWest),
                    edge!(LoruleLakeEast),
                    old_path(
                        TurtleRockWeatherVane,
                        Some(|p| p.can_rescue_turtles()),
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
                    old_path(
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
                    old_path(
                        TurtleRockFrontDoor,
                        Some(|p| p.has_ice_rod() && p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
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
                    edge!(TurtleRockFoyer),
                    old_path(
                        TurtleRockWeatherVane,
                        Some(|p| p.has_ice_rod() && p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(LoruleLakeWater, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        // Dark Ruins Area
        (
            DarkRuins,
            location(
                "Dark Ruins",
                vec![
                    check!("Dark Ruins Lakeview Chest", regions::lorule::dark::ruins::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Dark Ruins Waterfall",
                            regions::lorule::dark::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots()),
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Dark Maze Entrance Wall",
                            regions::lorule::dark::ruins::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Atop Dark Ruins Rocks",
                            regions::lorule::dark::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Dark Ruins West Tree",
                            regions::lorule::dark::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Dark Ruins East Tree",
                            regions::lorule::dark::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Dark Ruins South Area Wall",
                            regions::lorule::dark::ruins::SUBREGION,
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
                    edge!(DarkMazeEntrance),
                    old_path(KusDomainSouth, Some(|p| p.can_merge()), None, None, None, None),
                    edge!(DarkRuinsShallowWater),
                    old_path(
                        LoruleLakeWater,
                        None,
                        None,
                        Some(|p| p.has_flippers() && (p.has_fire_rod() || p.has_nice_bombs())),
                        Some(|p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs())), // fake flipper
                        Some(|p| p.has_boots()), // Bee boost
                    ),
                    old_path(
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
                vec![old_check(
                    LocationInfo::new("Dark Maze Chest", regions::lorule::dark::ruins::SUBREGION),
                    Some(|p| p.can_merge() || p.has_sage_gulley()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    edge!(DarkRuins),
                    old_path(
                        DarkMazeHalfway,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        Some(|_| true), // scuffed sneak
                    ),
                    old_path(
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
                    check!("Dark Maze Ledge", regions::lorule::dark::ruins::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Dark Maze Center Wall",
                            regions::lorule::dark::ruins::SUBREGION,
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
                    old_path(
                        DarkMazeEntrance,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        Some(|_| true),
                    ),
                    old_path(
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
                    old_path(
                        DarkMazeEntrance,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        DarkMazeHalfway,
                        Some(|p| p.can_merge() || p.has_sage_gulley()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(DarkPalaceFoyer, Some(|p| p.has_bombs()), None, None, None, None),
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
                    old_path(
                        HinoxCaveWater,
                        Some(|p| p.can_merge() && p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        HinoxCaveShallowWater,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(DarkRuins, Some(|p| p.has_flippers()), None, None, None, None),
                ],
            ),
        ),
        (
            KusDomainSouth,
            location(
                "Ku's Domain South",
                vec![old_check(
                    LocationInfo::new(
                        "[Mai] Ku's Domain Grass",
                        regions::lorule::dark::ruins::SUBREGION,
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
                    old_path(
                        HinoxCaveWater,
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        Some(|p| p.has_boots()), // Crow boost fake flippers
                        None,
                    ),
                    old_path(
                        HinoxCaveShallowWater,
                        Some(|p| p.has_flippers()),
                        None,
                        Some(|_| true), // Crow boost
                        None,
                        None,
                    ),
                    old_path(
                        DarkRuins,
                        Some(|p| p.can_merge()),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                    ),
                    old_path(KusDomain, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            KusDomain,
            location(
                "Ku's Domain",
                vec![
                    old_check(
                        LocationInfo::new(
                            "Ku's Domain Fight",
                            regions::lorule::dark::ruins::SUBREGION,
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
                    old_check(
                        LocationInfo::new(
                            "[Mai] Ku's Domain Water",
                            regions::lorule::dark::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_flippers()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![fast_travel_lorule(), edge!(KusDomainSouth)],
            ),
        ),
        (
            HinoxCaveWater,
            location(
                "Hinox Cave Water",
                vec![
                    // This location assumes the player is already swimming, real or fake
                    check!("[Mai] Outside Hinox Cave", regions::lorule::dark::ruins::SUBREGION),
                ],
                vec![edge!(HinoxCaveShallowWater)],
            ),
        ),
        (
            HinoxCaveShallowWater,
            location(
                "Hinox Cave Shallow Water",
                vec![],
                vec![
                    fast_travel_lorule(),
                    edge!(HinoxCave),
                    old_path(HinoxCaveWater, Some(|p| p.has_flippers()), None, None, None, None),
                    old_path(
                        DarkRuinsShallowWater,
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
            HinoxCave,
            location(
                "Hinox Cave",
                vec![
                    check!("Hinox (1)", regions::lorule::dark::ruins::SUBREGION),
                    check!("Hinox (2)", regions::lorule::dark::ruins::SUBREGION),
                    check!("Hinox (3)", regions::lorule::dark::ruins::SUBREGION),
                    check!("Hinox (4)", regions::lorule::dark::ruins::SUBREGION),
                    check!("Hinox (5)", regions::lorule::dark::ruins::SUBREGION),
                    check!("Hinox (6)", regions::lorule::dark::ruins::SUBREGION),
                ],
                vec![edge!(HinoxCaveShallowWater)],
            ),
        ),
        // Skull Woods Area
        (
            SkullWoodsOverworld,
            location(
                "Skull Woods (Overworld)",
                vec![
                    old_check(
                        LocationInfo::new(
                            "Canyon House",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot())), // portal clip through house
                        None,
                    ),
                    check!("Destroyed House", regions::lorule::skull::overworld::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Skull Woods Grass",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.can_cut_grass()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Skull Woods Skull",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.can_destroy_skull()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Skull Woods Shack Tree",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.has_boots()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check!("[Mai] Skull Woods Bush", regions::lorule::skull::overworld::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Skull Woods Big Rock",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Skull Woods Entrance Wall",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Skull Woods Dry Pond",
                            regions::lorule::skull::overworld::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Canyon House Wall",
                            regions::lorule::skull::overworld::SUBREGION,
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
                    edge!(MysteriousManCave),
                    portal_std(HyruleField),
                    edge!(SkullWoodsFoyer),
                ],
            ),
        ),
        (
            MysteriousManCave,
            location(
                "Mysterious Man Cave",
                vec![old_check(
                    LocationInfo::new(
                        "Mysterious Man",
                        regions::lorule::skull::overworld::SUBREGION,
                    ),
                    Some(|p| p.has_bottle()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![edge!(SkullWoodsOverworld)],
            ),
        ),
        // Lorule Death Mountain
        (
            LoruleDeathWest,
            location(
                "Lorule Death Mountain West",
                vec![
                    old_check(
                        LocationInfo::new(
                            "Ice Gimos Fight",
                            regions::lorule::death::mountain::SUBREGION,
                        ),
                        Some(|p| p.can_defeat_margomill()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "Lorule Mountain W Ledge",
                            regions::lorule::death::mountain::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs()),
                        None,
                        Some(|p| p.has_bombs()),
                    ),
                    old_check(
                        LocationInfo::new(
                            "Treacherous Tower Intermediate",
                            regions::lorule::death::mountain::SUBREGION,
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
                    out_of_logic(
                        "Treacherous Tower Advanced (1)",
                        regions::lorule::death::mountain::SUBREGION,
                    ),
                    out_of_logic(
                        "Treacherous Tower Advanced (2)",
                        regions::lorule::death::mountain::SUBREGION,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Mountain W Skull",
                            regions::lorule::death::mountain::SUBREGION,
                        ),
                        Some(|p| p.can_destroy_skull()),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Mountain W Big Rock",
                            regions::lorule::death::mountain::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt() && p.has_hammer()),
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_nice_bombs()), // Not enough room for Fire Rod
                        None,
                        Some(|p| p.has_titans_mitt() && p.has_bombs()),
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Lorule Mountain E Big Rock",
                            regions::lorule::death::mountain::SUBREGION,
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
                    old_path(
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
                vec![old_check(
                    LocationInfo::new(
                        "[Mai] Lorule Mountain E Wall",
                        regions::lorule::death::mountain::SUBREGION,
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
                    old_path(LoruleDeathWest, Some(|p| p.has_hookshot()), None, None, None, None),
                    edge!(IceCaveEast),
                ],
            ),
        ),
        (
            IceCaveEast,
            location(
                "Ice Cave East",
                vec![],
                vec![
                    edge!(RossosOreMineLorule),
                    old_path(IceCaveCenter, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            IceCaveCenter,
            location(
                "Ice Cave Center",
                vec![],
                vec![
                    old_path(IceCaveEast, Some(|p| p.can_merge()), None, None, None, None),
                    old_path(
                        IceCaveSouth,
                        Some(|p| p.can_merge()),
                        Some(|p| p.has_tornado_rod()), // jump over merge block
                        None,
                        None,
                        Some(|_| true), // big yeets from the statue
                    ),
                    old_path(IceCaveWest, Some(|p| p.has_tornado_rod()), None, None, None, None),
                    edge!(LoruleDeathEastTop),
                ],
            ),
        ),
        (
            IceCaveSouth,
            location(
                "Ice Cave South",
                vec![],
                vec![
                    edge!(LoruleDeathEastLedgeLower),
                    old_path(IceCaveCenter, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            IceCaveWest,
            location(
                "Ice Cave West",
                vec![],
                vec![
                    edge!(IceCaveCenter),
                    old_path(
                        IceCaveNorthWest,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        IceCaveSouthWest,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            IceCaveNorthWest,
            location(
                "Ice Cave North West",
                vec![],
                vec![
                    edge!(FloatingIslandLorule),
                    old_path(
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
                    edge!(IceCaveNorthWest),
                    portal_std(FloatingIslandHyrule),
                ],
            ),
        ),
        (
            IceCaveSouthWest,
            location(
                "Ice Cave South West",
                vec![],
                vec![edge!(IceCaveWest), edge!(LoruleDeathEastLedgeUpper)],
            ),
        ),
        (
            LoruleDeathEastLedgeUpper,
            location(
                "Lorule Death Mountain East Upper Ledge",
                vec![old_check(
                    LocationInfo::new(
                        "Lorule Mountain E Ledge",
                        regions::lorule::death::mountain::SUBREGION,
                    ),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    fast_travel_lorule(),
                    edge!(IceCaveWest),
                    edge!(LoruleDeathEastLedgeLower),
                    old_path(
                        RossosOreMineLorule,
                        None,
                        None,
                        Some(|p| p.has_nice_bombs()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            LoruleDeathEastLedgeLower,
            location(
                "Lorule Death Mountain East Lower Ledge",
                vec![old_check(
                    LocationInfo::new(
                        "[Mai] Lorule Mountain E Skull",
                        regions::lorule::death::mountain::SUBREGION,
                    ),
                    Some(|p| p.can_destroy_skull()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![fast_travel_lorule(), edge!(IceCaveSouth)],
            ),
        ),
        (
            LoruleDeathEastTop,
            location(
                "Lorule Death Mountain East Top",
                vec![
                    old_check(
                        LocationInfo::new(
                            "Behind Ice Gimos",
                            regions::lorule::death::mountain::SUBREGION,
                        ),
                        Some(|p| p.has_fire_rod()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[Mai] Outside Ice Ruins",
                            regions::lorule::death::mountain::SUBREGION,
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
                    edge!(IceCaveCenter),
                    old_path(IceRuinsFoyer, Some(|p| p.has_fire_rod()), None, None, None, None),
                ],
            ),
        ),
    ])
}

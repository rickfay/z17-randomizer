use crate::filler::check::Check;
use crate::filler::cracks::Crack;
use crate::filler::cracks::Crack::*;
use crate::filler::filler_item::Goal;
use crate::filler::filler_item::Vane::*;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::world::{
    check, crack_left, crack_right, edge, fast_travel_hyrule, fast_travel_lorule, ghost, goal, location, old_path,
    out_of_logic,
};
use crate::LocationInfo;
use crate::{regions, CrackMap};
use game::ghosts::HintGhost;
use std::collections::HashMap;

/// Lorule
pub(crate) fn graph(crack_map: &CrackMap) -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            LoruleBellTravel,
            location(
                "Lorule Bell Travel",
                vec![],
                vec![
                    edge!(LoruleCastleArea, |p| p.has_weather_vane(VacantHouseWV)
                        || p.has_weather_vane(BlacksmithWV)
                        || p.has_weather_vane(ThievesTownWV)
                        || p.has_weather_vane(LoruleCastleWV)),
                    edge!(SkullWoodsOverworld, |p| p.has_weather_vane(SkullWoodsWV)),
                    edge!(MiseryMire, |p| p.has_weather_vane(MiseryMireWV)),
                    edge!(SwampPalaceOutside, |p| p.has_weather_vane(SwampPalaceWV)),
                    edge!(LoruleDeathWest, |p| p.has_weather_vane(TreacherousTowerWV)),
                    edge!(LoruleGraveyard, |p| p.has_weather_vane(GraveyardWV)),
                    edge!(RossosOreMineLorule, |p| p.has_weather_vane(DeathMountainLoruleWV)),
                    edge!(TurtleRockWeatherVane, |p| p.has_weather_vane(TurtleRockWV)),
                    edge!(LoruleDeathEastTop, |p| p.has_weather_vane(IceRuinsWV)),
                    edge!(DarkPalaceWeatherVane, |p| p.has_weather_vane(DarkPalaceWV)),
                ],
            ),
        ),
        (
            LoruleCastleArea,
            location(
                "Lorule Castle Area",
                vec![
                    // check!("Vacant House Crack", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("Vacant House Weather Vane", regions::lorule::field::main::SUBREGION),
                    check!("Blacksmith Weather Vane", regions::lorule::field::main::SUBREGION),
                    check!("Lorule Castle Weather Vane", regions::lorule::field::main::SUBREGION),
                    check!("Thieves' Town Weather Vane", regions::lorule::field::main::SUBREGION),
                    check!("Rupee Rush (Lorule)", regions::lorule::field::main::SUBREGION),
                    check!("Octoball Derby", regions::lorule::field::main::SUBREGION),
                    goal!("Access Hilda Barrier", Goal::AccessLoruleCastleField),
                    check!("Fortune's Choice", regions::lorule::field::main::SUBREGION),
                    check!("[Mai] Lorule Castle Wall", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Lorule Castle Tree", regions::lorule::field::main::SUBREGION, |p| p.has_boots()),
                    check!("[Mai] Thieves' Town Wall", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Lorule Fortune-Teller Rock", regions::lorule::field::main::SUBREGION, |p| p
                        .has_titans_mitt()),
                    check!("[Mai] Lorule Blacksmith Wall", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Lorule Rupee Rush Wall", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Octoball Derby Skull", regions::lorule::field::main::SUBREGION => {
                        normal: |p| p.can_destroy_skull(),
                        hard: |_| true, // throw bush at skull
                    }),
                    check!("[Mai] Vacant House Rock", regions::lorule::field::main::SUBREGION, |p| p.has_titans_mitt()),
                    check!("[Mai] Behind Vacant House", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Lorule S Ruins Pillars", regions::lorule::field::main::SUBREGION, |p| p.has_boots()),
                    check!("[Mai] Lorule S Ruins Wall", regions::lorule::field::main::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Lorule S Ruins Water", regions::lorule::field::main::SUBREGION, |p| p.has_flippers()),
                    check!("[Mai] Thieves' Town Tree", regions::lorule::field::main::SUBREGION, |p| p.has_boots()),
                    ghost(HintGhost::FortuneTellerLorule),
                    ghost(HintGhost::RupeeRushLorule),
                    ghost(HintGhost::GreatRupeeFairy),
                    ghost(HintGhost::OctoballDerby),
                    ghost(HintGhost::VacantHouse),
                    ghost(HintGhost::SwampPalaceOutsideLeft),
                    ghost(HintGhost::SwampPalaceOutsideRight),
                ],
                vec![
                    crack_left(VacantHouse, crack_map, false),
                    crack_right(VacantHouse, crack_map, false),
                    crack_left(ThievesTown, crack_map, false),
                    crack_right(ThievesTown, crack_map, false),
                    crack_left(ParadoxLeftLorule, crack_map, false),
                    crack_right(ParadoxLeftLorule, crack_map, false),
                    crack_left(SwampPillarLorule, crack_map, false),
                    crack_right(SwampPillarLorule, crack_map, false),
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
                    edge!(ThievesHideoutB1, |p| p.hearts(6.0)),
                    edge!(LoruleCastle1F, |p| p.has_lc_requirement() && p.hearts(13.0)),
                    edge!(BigBombFlowerShop),
                    old_path(
                        BigBombFlowerField,
                        Some(|p| p.has_bomb_flower()),
                        None,
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                    ),
                    edge!(ThievesTownItemShop),
                    edge!(VeteranThiefsHouse),
                    edge!(FortunesChoiceLorule),
                ],
            ),
        ),
        (
            VeteranThiefsHouse,
            location("Veteran Thief's House", vec![ghost(HintGhost::VeteranThief)], vec![edge!(LoruleCastleArea)]),
        ),
        (
            FortunesChoiceLorule,
            location(
                "Fortune's Choice (Lorule)",
                vec![ghost(HintGhost::FortunesChoice)],
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (
            ThievesTownItemShop,
            location(
                "Thieves' Town Item Shop",
                vec![
                    check!("Thieves' Town Item Shop (1)", regions::lorule::field::main::SUBREGION),
                    out_of_logic("Thieves' Town Item Shop (2)", regions::lorule::field::main::SUBREGION),
                    check!("Thieves' Town Item Shop (3)", regions::lorule::field::main::SUBREGION),
                    check!("Thieves' Town Item Shop (4)", regions::lorule::field::main::SUBREGION),
                ],
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (
            BigBombFlowerShop,
            location("Big Bomb Flower Shop", vec![], vec![edge!(LoruleCastleArea), edge!(BigBombFlowerField)]),
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
                    old_path(LoruleCastleArea, Some(|p| p.has_bomb_flower()), None, None, None, None),
                ],
            ),
        ),
        (
            LoruleGraveyard,
            location(
                "Lorule Graveyard",
                vec![
                    check!("Graveyard Weather Vane", regions::lorule::graveyard::lorule::SUBREGION),
                    check!("Graveyard Peninsula", regions::lorule::graveyard::lorule::SUBREGION),
                    check!("[Mai] Lorule Graveyard Big Rock", regions::lorule::graveyard::lorule::SUBREGION, |p| p
                        .has_titans_mitt()),
                    check!("[Mai] Lorule Graveyard Wall", regions::lorule::graveyard::lorule::SUBREGION, |p| p
                        .can_merge()),
                    check!("[Mai] Lorule Graveyard Tree", regions::lorule::graveyard::lorule::SUBREGION, |p| p
                        .has_boots()),
                    ghost(HintGhost::LoruleGraveyard),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(LoruleSanctuaryCaveLower),
                    old_path(LoruleSanctuary, Some(|p| p.has_titans_mitt()), None, None, None, None),
                    old_path(
                        DarkRuins,
                        None,
                        None,
                        Some(|p| (p.has_fire_rod() || p.has_nice_bombs()) && p.has_flippers()),
                        Some(|p| (p.has_fire_rod() || p.has_nice_bombs()) && (p.has_flippers() || p.has_hookshot())), // Hookshot trick
                        Some(|p| p.has_flippers() || p.has_hookshot()), // Bee Boost
                    ),
                    old_path(Location::GraveyardLedgeLorule, Some(|p| p.has_bombs()), None, None, None, None),
                ],
            ),
        ),
        (
            Location::GraveyardLedgeLorule,
            location(
                "Graveyard Ledge Lorule",
                vec![],
                vec![
                    fast_travel_lorule(),
                    crack_left(Crack::GraveyardLedgeLorule, crack_map, false),
                    crack_right(Crack::GraveyardLedgeLorule, crack_map, false),
                    edge!(LoruleGraveyard),
                ],
            ),
        ),
        (
            LoruleSanctuary,
            location(
                "Lorule Sanctuary",
                vec![
                    check!("[LS] Entrance Chest", regions::lorule::graveyard::lorule::SUBREGION, |p| p.has_lamp()
                        || p.lampless()),
                    check!("[LS] Lower Chest", regions::lorule::graveyard::lorule::SUBREGION, |p| p.has_lamp()
                        || (p.has_fire_rod() && p.lampless())),
                    check!("[LS] Upper Chest", regions::lorule::graveyard::lorule::SUBREGION, |p| p.has_lamp()
                        || (p.has_fire_rod() && p.lampless())),
                    check!("[LS] Ledge", regions::lorule::graveyard::lorule::SUBREGION, |p| p.can_merge()
                        && (p.has_lamp() || (p.has_fire_rod() && p.lampless()))),
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
                vec![
                    crack_left(Philosopher, crack_map, false),
                    crack_right(Philosopher, crack_map, false),
                    edge!(LoruleGraveyard),
                ],
            ),
        ),
        (
            LoruleSanctuaryCaveUpper,
            location(
                "Philosopher's Cave Upper",
                vec![check!("Philosopher's Cave", regions::lorule::graveyard::lorule::SUBREGION, |p| p.can_merge())],
                vec![edge!(LoruleSanctuary), edge!(LoruleSanctuaryCaveLower)],
            ),
        ),
        (
            GreatRupeeFairyCave,
            location(
                "Great Rupee Fairy Cave",
                vec![check!("Great Rupee Fairy", regions::lorule::field::main::SUBREGION => {
                    normal: |p| p.has_rupees(4000), // Actual requirement is 3000 but higher threshold helps prevent rupee grinds
                    hell: |_| true, // suffer lol
                })],
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (
            LoruleBlacksmith,
            location(
                "Lorule Blacksmith",
                vec![check!("Blacksmith (Lorule)", regions::lorule::field::main::SUBREGION, |p| {
                    p.has_master_ore(4) && p.can_access_hyrule_blacksmith() && p.can_access_lorule_castle_field()
                })],
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (
            BootsDungeon,
            location(
                "Pegasus Boots Pyramid",
                vec![check!("Pegasus Boots Pyramid", regions::lorule::field::main::SUBREGION => {
                    normal: |p| p.has_boots() && p.can_hit_switch_bootless(),
                    hard: |p| p.has_master_sword() || p.has_bombs(),
                    hell: |p| p.has_boomerang() || p.has_nice_bow() || p.has_nice_ice_rod(),
                })],
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (VacantHouseBottom, location("Vacant House (Bottom)", vec![], vec![edge!(LoruleCastleArea)])),
        (
            VacantHouseTop,
            location(
                "Vacant House (Top)",
                vec![check!("Vacant House", regions::lorule::field::main::SUBREGION)],
                vec![edge!(LoruleCastleArea => {
                    normal: |p| p.has_bombs(),
                    hard: |p| p.has_bomb_flower(),
                })],
            ),
        ),
        (
            ThiefGirlCave,
            location(
                "Thief Girl",
                vec![check!("Thief Girl", regions::lorule::field::main::SUBREGION, |p| p.has_saved_thief_girl())],
                vec![edge!(LoruleCastleArea)],
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
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (
            BigBombCave,
            location(
                "Haunted Grove Big Bomb Cave",
                vec![check!("Big Bomb Flower Cave", regions::lorule::field::main::SUBREGION)],
                vec![edge!(LoruleCastleArea)],
            ),
        ),
        (
            HauntedGroveLedge,
            location(
                "Haunted Grove Upper Ledge",
                vec![
                    check!("Lorule Field Hookshot Chest", regions::lorule::field::main::SUBREGION, |p| p
                        .has_hookshot()),
                    check!("[Mai] Lorule Haunted Grove Wall", regions::lorule::field::main::SUBREGION, |p| p
                        .can_merge()),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(LoruleCastleArea),
                    crack_left(ParadoxRightLorule, crack_map, false),
                    crack_right(ParadoxRightLorule, crack_map, false),
                ],
            ),
        ),
        // Desert / Misery Mire
        (
            Desert,
            location(
                "Desert",
                vec![
                    check!("[Mai] Buried in the Desert", regions::hyrule::desert::mystery::SUBREGION, |p| p
                        .has_sand_rod()),
                    ghost(HintGhost::DesertEast),
                ],
                vec![
                    fast_travel_hyrule(),
                    crack_left(DesertPillarRight, crack_map, false),
                    crack_right(DesertPillarRight, crack_map, false),
                    crack_left(DesertPillarLeft, crack_map, false),
                    crack_right(DesertPillarLeft, crack_map, false),
                    edge!(DesertNorthLedge => {
                        normal: |p| p.can_merge() && (p.has_sand_rod() || p.has_stamina_scroll()),
                        glitched: |p| p.has_nice_bombs() || p.has_fire_rod(),
                        hell: |_| true, // Vulture Boost
                    }),
                    edge!(DesertCenterLedge, |p| p.has_sand_rod()),
                    edge!(DesertSouthWestLedge => {
                        normal: |p| p.can_merge() && (p.has_stamina_scroll() || p.has_nice_sand_rod()), // YUP
                        glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                        adv_glitched: |_| true, // vulture boost
                    }),
                    edge!(DesertPalaceWeatherVane => {
                        glitched: |_| true, // vulture clip
                    }),
                ],
            ),
        ),
        (
            DesertNorthLedge,
            location(
                "Desert North Ledge",
                None,
                vec![
                    edge!(Desert),
                    edge!(DesertUseBlockedCrackRight, |p| p.has_bombs()),
                    edge!(DesertUseBlockedCrackLeft, |p| p.has_bombs() && p.has_sand_rod()),
                ],
            ),
        ),
        (
            DesertUseBlockedCrackRight,
            location(
                "Desert Use Blocked Crack Right",
                None,
                vec![
                    // crack is blocked, no return paths
                    crack_right(DesertNorth, crack_map, false),
                ],
            ),
        ),
        (
            DesertUseBlockedCrackLeft,
            location(
                "Desert Use Blocked Crack Left",
                None,
                vec![
                    // crack is blocked, no return paths
                    crack_left(DesertNorth, crack_map, false),
                ],
            ),
        ),
        (
            DesertCenterLedge,
            location(
                "Desert Center Ledge",
                vec![ghost(HintGhost::DesertCenter)],
                vec![
                    edge!(Desert),
                    // crack_left unpossible
                    crack_right(DesertMiddle, crack_map, false),
                ],
            ),
        ),
        (
            DesertSouthWestLedge,
            location(
                "Desert South West Ledge",
                vec![ghost(HintGhost::DesertSouthWest)],
                vec![
                    fast_travel_hyrule(),
                    crack_left(DesertSW, crack_map, false),
                    crack_right(DesertSW, crack_map, false),
                    edge!(Desert),
                    edge!(DesertPalaceWeatherVane, |p| p.has_sand_rod()),
                ],
            ),
        ),
        (
            DesertPalaceWeatherVane,
            location(
                "Desert Palace Weather Vane",
                vec![
                    check!("Desert Palace Weather Vane", regions::hyrule::desert::mystery::SUBREGION),
                    check!("[Mai] Buried near Desert Palace", regions::hyrule::desert::mystery::SUBREGION, |p| p
                        .has_sand_rod()),
                ],
                vec![
                    fast_travel_hyrule(),
                    edge!(Desert),
                    edge!(DesertPalaceFoyer, |p| p.has_sand_rod() && p.hearts(9.0)),
                ],
            ),
        ),
        (
            MiseryMire,
            location(
                "Misery Mire",
                vec![
                    check!("Misery Mire Weather Vane", regions::lorule::misery::mire::SUBREGION),
                    check!("[Mai] Misery Mire Wall", regions::lorule::misery::mire::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Misery Mire Water", regions::lorule::misery::mire::SUBREGION, |p| p.has_flippers()),
                    check!("[Mai] Misery Mire Rock", regions::lorule::misery::mire::SUBREGION, |p| p.has_titans_mitt()),
                    ghost(HintGhost::MiseryMireLedge),
                    ghost(HintGhost::MiseryMireBridge),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(SandRodDungeon),
                    // no way to enter left pillar crack
                    // no way to enter mire north crack
                    crack_left(MiseryMireExit, crack_map, false),
                    crack_right(MiseryMireExit, crack_map, false),
                    crack_left(MirePillarRight, crack_map, false),
                    crack_right(MirePillarRight, crack_map, false),
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
            // This is the useless crack surrounded by water
            // Psst... it can be used to reverse the side of the crack you entered
            MiseryMireLeftPillarMerged,
            location(
                "Misery Mire Left Pillar Merged",
                None,
                vec![
                    edge!(MiseryMire, |p| p.has_flippers()),
                    crack_left(MirePillarLeft, crack_map, false),
                    crack_right(MirePillarLeft, crack_map, false),
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
                    crack_left(MireMiddle, crack_map, false),
                    crack_right(MireMiddle, crack_map, false),
                    crack_left(MireSW, crack_map, false),
                    crack_right(MireSW, crack_map, false),
                    old_path(
                        MiseryMireOoB,
                        None,
                        None,
                        None,
                        Some(|p| p.has_fire_rod() || p.has_nice_bombs()),
                        Some(|p| (p.has_hookshot() || p.has_boomerang()) && p.has_tornado_rod()), // crack clip
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
                    crack_left(Zaganaga, crack_map, false),
                    crack_right(Zaganaga, crack_map, false),
                    edge!(ZaganagasArena),
                    old_path(MiseryMireRewardBasket, None, None, None, Some(|p| p.has_boots()), None),
                ],
            ),
        ),
        (
            SandRodDungeon,
            location(
                "Sand Mini-Dungeon",
                vec![check!("Sand Mini-Dungeon", regions::lorule::misery::mire::SUBREGION => {
                    normal: |p| p.has_sand_rod() && p.has_tornado_rod(),
                    glitched: |p| p.has_sand_rod(),
                })],
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
                    check!("[Mai] Lorule Lake SE Wall", regions::lorule::lake::lorule::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Lorule Lake Skull", regions::lorule::lake::lorule::SUBREGION => {
                        normal: |p| p.can_merge() && p.can_destroy_skull(),
                        hard: |p| p.can_merge(),
                    }),
                ],
                vec![
                    fast_travel_lorule(),
                    crack_left(LoruleHotfoot, crack_map, false),
                    crack_right(LoruleHotfoot, crack_map, false),
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
                    check!("[Mai] Lorule Lake West Wall", regions::lorule::lake::lorule::SUBREGION, |p| p.can_merge()),
                    ghost(HintGhost::TurtleWall),
                ],
                vec![
                    fast_travel_lorule(),
                    crack_left(LoruleLake, crack_map, false),
                    crack_right(LoruleLake, crack_map, false),
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
                    check!("[Mai] Lorule Lake Rock", regions::lorule::lake::lorule::SUBREGION, |p| p.has_titans_mitt()),
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
                    check!("Lorule Lakeside Item Shop (1)", regions::lorule::lake::lorule::SUBREGION),
                    out_of_logic("Lorule Lakeside Item Shop (2)", regions::lorule::lake::lorule::SUBREGION),
                    check!("Lorule Lakeside Item Shop (3)", regions::lorule::lake::lorule::SUBREGION),
                    check!("Lorule Lakeside Item Shop (4)", regions::lorule::lake::lorule::SUBREGION),
                ],
                vec![edge!(LoruleLakeNorthWest)],
            ),
        ),
        (
            LoruleRiverCrackShallows,
            location(
                "Lorule River Crack Shallows",
                None,
                vec![
                    fast_travel_lorule(),
                    crack_left(RiverLorule, crack_map, false),
                    crack_right(RiverLorule, crack_map, false),
                    edge!(LoruleLakeWater, |p| p.has_flippers()),
                ],
            ),
        ),
        // This location assumes the player is already swimming, real or fake
        (
            LoruleLakeWater,
            location(
                "Lorule Lake Water",
                vec![
                    goal!("Turtle (bullied)", Goal::TurtleAttacked, |p| p.can_attack()),
                    check!("[Mai] Lorule Lake Water", regions::lorule::lake::lorule::SUBREGION),
                    ghost(HintGhost::TurtleBullied),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(LoruleLakeNorthWest),
                    edge!(LoruleLakeSouthWest),
                    edge!(LoruleLakeEast),
                    edge!(LoruleRiverCrackShallows),
                    edge!(TurtleRockWeatherVane => {
                        normal: |p| p.can_rescue_turtles(),
                        glitched: |p| p.has_tornado_rod(),
                    }),
                    edge!(TurtleRockFrontDoor => {
                        glitched: |p| p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            TurtleRockWeatherVane,
            location(
                "Turtle Rock Weather Vane",
                vec![
                    check!("Turtle Rock Weather Vane", regions::lorule::lake::lorule::SUBREGION),
                    ghost(HintGhost::TurtleRockOutside),
                ],
                vec![
                    fast_travel_lorule(),
                    old_path(TurtleRockFrontDoor, Some(|p| p.has_ice_rod() && p.can_merge()), None, None, None, None),
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
                    edge!(TurtleRockFoyer, |p| p.hearts(9.0)),
                    old_path(TurtleRockWeatherVane, Some(|p| p.has_ice_rod() && p.can_merge()), None, None, None, None),
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
                    check!("Dark/Turtle Chest", regions::lorule::lake::lorule::SUBREGION),
                    check!("[Mai] Dark Ruins Waterfall", regions::lorule::dark::ruins::SUBREGION => {
                        normal: |p| p.has_flippers(),
                        adv_glitched: |p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()),
                        hell: |p| p.has_boots(),
                    }),
                    check!("[Mai] Dark Maze Entrance Wall", regions::lorule::dark::ruins::SUBREGION, |p| p.can_merge()),
                    check!("[Mai] Dark Ruins Bonk Rocks", regions::lorule::dark::ruins::SUBREGION, |p| p.has_boots()),
                    check!("[Mai] Dark Ruins West Tree", regions::lorule::dark::ruins::SUBREGION, |p| p.has_boots()),
                    check!("[Mai] Dark Ruins East Tree", regions::lorule::dark::ruins::SUBREGION, |p| p.has_boots()),
                    check!("[Mai] Dark Ruins South Wall", regions::lorule::dark::ruins::SUBREGION, |p| p.can_merge()),
                    ghost(HintGhost::DarkRuinsNorth),
                ],
                vec![
                    fast_travel_lorule(),
                    crack_left(DarkRuinsPillar, crack_map, false),
                    crack_right(DarkRuinsPillar, crack_map, false),
                    edge!(DarkRuinsBlockedCrack, |p| p.has_bombs()),
                    edge!(DarkMazeEntrance),
                    edge!(KusDomainSouth, |p| p.can_merge()),
                    edge!(DarkRuinsShallowWater),
                    edge!(LoruleRiverCrackShallows => {
                        glitched: |p| p.has_fire_rod() || p.has_nice_bombs(),
                        hell: |_| true, // Bee Boost
                    }),
                    edge!(LoruleLakeWater => {
                        glitched: |p| p.has_flippers() && (p.has_fire_rod() || p.has_nice_bombs()),
                        adv_glitched: |p| p.has_boots() && (p.has_fire_rod() || p.has_nice_bombs()), // fake flipper
                        hell: |p| p.has_boots(), // Bee boost
                    }),
                    edge!(LoruleLakeEast => {
                        glitched: |p| p.has_stamina_scroll() && (p.has_fire_rod() || p.has_nice_bombs()), // long merge
                        hell: |p| p.has_stamina_scroll(), // Bee Boost
                    }),
                ],
            ),
        ),
        (
            DarkRuinsBlockedCrack,
            location(
                "Dark Ruins Blocked Crack",
                None,
                vec![
                    edge!(DarkRuins),
                    crack_left(DarkRuinsSE, crack_map, false),
                    crack_right(DarkRuinsSE, crack_map, false),
                ],
            ),
        ),
        (
            DarkMazeEntrance,
            location(
                "Dark Maze Entrance",
                vec![check!("Dark Maze Chest", regions::lorule::dark::ruins::SUBREGION, |p| p.can_merge()
                    || p.has_sage_gulley())],
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
                    check!("[Mai] Dark Maze Center Wall", regions::lorule::dark::ruins::SUBREGION, |p| p.can_merge()),
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
                vec![
                    check!("Dark Palace Weather Vane", regions::lorule::dark::ruins::SUBREGION),
                    ghost(HintGhost::DarkPalaceOutside),
                ],
                vec![
                    old_path(DarkMazeEntrance, Some(|p| p.can_merge() || p.has_sage_gulley()), None, None, None, None),
                    old_path(DarkMazeHalfway, Some(|p| p.can_merge() || p.has_sage_gulley()), None, None, None, None),
                    edge!(DarkPalaceFoyer, |p| p.has_bombs() && p.hearts(6.0)),
                ],
            ),
        ),
        (
            DarkRuinsRiver,
            location(
                "Dark Ruins River",
                None,
                vec![edge!(DarkRuins, |p| p.has_flippers()), edge!(DarkRuinsShallowWater, |p| p.has_flippers())],
            ),
        ),
        (
            DarkRuinsShallowWater,
            location(
                "Dark Ruins Shallow Water",
                None,
                vec![
                    fast_travel_lorule(),
                    // crack_left unpossible
                    crack_right(WaterfallLorule, crack_map, false),
                    edge!(HinoxCaveWater, |p| p.can_merge() && p.has_flippers()),
                    edge!(HinoxCaveShallowWater, |p| p.can_merge()),
                    edge!(DarkRuins, |p| p.has_flippers()),
                ],
            ),
        ),
        (
            KusDomainSouth,
            location(
                "Ku's Domain South",
                vec![check!("[Mai] Ku's Domain Grass", regions::lorule::dark::ruins::SUBREGION, |p| p.can_merge()
                    && p.can_cut_grass())],
                vec![
                    fast_travel_lorule(),
                    crack_left(Crack::KusDomain, crack_map, false),
                    crack_right(Crack::KusDomain, crack_map, false),
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
                    old_path(DarkRuins, Some(|p| p.can_merge()), Some(|p| p.has_hookshot()), None, None, None),
                    old_path(Location::KusDomain, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            Location::KusDomain,
            location(
                "Ku's Domain",
                vec![
                    check!("Ku's Domain Fight", regions::lorule::dark::ruins::SUBREGION => {
                        normal: |p| {
                            p.has_bow()
                                || p.has_bombs()
                                || p.can_great_spin()
                                || p.has_nice_ice_rod()
                                || p.has_nice_hookshot()
                        },
                        hard: |p| p.has_master_sword() || (p.has_sword() && p.has_power_glove()),
                    }),
                    check!("[Mai] Ku's Domain Water", regions::lorule::dark::ruins::SUBREGION, |p| p.has_flippers()),
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
                    old_path(DarkRuinsShallowWater, Some(|p| p.can_merge()), None, None, None, None),
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
                    check!("Skull Woods Weather Vane", regions::lorule::skull::overworld::SUBREGION),
                    check!("n-Shaped House", regions::lorule::skull::overworld::SUBREGION => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.has_boomerang() || (p.not_nice_mode() && p.has_hookshot()), // crack clip through house
                    }),
                    check!("Destroyed House", regions::lorule::skull::overworld::SUBREGION),
                    check!("[Mai] Skull Woods Grass", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .can_cut_grass()),
                    check!("[Mai] Skull Woods Skull", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .can_destroy_skull()),
                    check!("[Mai] Destroyed House Tree", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .has_boots()),
                    check!("[Mai] Skull Woods Bush", regions::lorule::skull::overworld::SUBREGION),
                    check!("[Mai] Skull Woods Rock", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .has_titans_mitt()),
                    check!("[Mai] Skull Woods Entrance Wall", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .can_merge()),
                    check!("[Mai] Skull Woods Dry Pond", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .can_merge()),
                    check!("[Mai] n-Shaped House Wall", regions::lorule::skull::overworld::SUBREGION, |p| p
                        .can_merge()),
                    ghost(HintGhost::SkullWoodsCuccos),
                    ghost(HintGhost::SkullWoodsSouth),
                ],
                vec![
                    fast_travel_lorule(),
                    crack_left(DestroyedHouse, crack_map, false),
                    crack_right(DestroyedHouse, crack_map, false),
                    crack_left(NShapedHouse, crack_map, false),
                    crack_right(NShapedHouse, crack_map, false),
                    crack_left(SkullWoodsPillar, crack_map, false),
                    crack_right(SkullWoodsPillar, crack_map, false),
                    edge!(MysteriousManCave),
                    edge!(SkullWoodsFoyer, |p| p.hearts(6.0)),
                ],
            ),
        ),
        (
            MysteriousManCave,
            location(
                "Mysterious Man Cave",
                vec![check!("Mysterious Man", regions::lorule::skull::overworld::SUBREGION, |p| p.has_bottle())],
                vec![edge!(SkullWoodsOverworld)],
            ),
        ),
        // Lorule Death Mountain
        (
            LoruleDeathWest,
            location(
                "Lorule Death Mountain West",
                vec![
                    check!("Treacherous Tower Weather Vane", regions::lorule::death::mountain::SUBREGION),
                    check!("Ice Gimos Fight", regions::lorule::death::mountain::SUBREGION, |p| p
                        .can_defeat_margomill()),
                    check!("Lorule Mountain W Ledge", regions::lorule::death::mountain::SUBREGION => {
                        normal: |p| p.can_merge(),
                        glitched: |p| p.has_nice_bombs(),
                        hell: |p| p.has_bombs(),
                    }),
                    check!("Treacherous Tower",regions::lorule::death::mountain::SUBREGION => {
                        normal: |p| (p.has_sword() || (p.swordless_mode() && p.can_attack())) && (p.has_bombs() || p.has_hammer() || p.has_tornado_rod()),
                        hard: |p| p.has_bombs() || p.has_hammer() || (p.has_tornado_rod() && p.can_attack()),
                    }),
                    check!("[Mai] Lorule Mountain W Skull", regions::lorule::death::mountain::SUBREGION => {
                        normal: |p| p.can_destroy_skull(),
                        hard: |p| p.can_merge(),
                    }),
                    check!("[Mai] Lorule Mountain W Big Rock", regions::lorule::death::mountain::SUBREGION => {
                        normal: |p| p.has_titans_mitt() && p.has_hammer(),
                        glitched: |p| p.has_titans_mitt() && p.has_nice_bombs(), // Not enough room for Fire Rod
                        hell: |p| p.has_titans_mitt() && p.has_bombs(),
                    }),
                    check!("[Mai] Lorule Mountain E Big Rock", regions::lorule::death::mountain::SUBREGION, |p| p
                        .has_titans_mitt()),
                    ghost(HintGhost::TreacherousTower),
                ],
                vec![
                    fast_travel_lorule(),
                    crack_left(DeathWestLorule, crack_map, false),
                    crack_right(DeathWestLorule, crack_map, false),
                    old_path(
                        Location::RossosOreMineLorule,
                        None,
                        None,
                        Some(|p| p.has_hookshot() && (p.has_fire_rod() || p.has_nice_bombs() || p.has_tornado_rod())),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            Location::RossosOreMineLorule,
            location(
                "Rosso's Ore Mine Lorule",
                vec![
                    check!("Death Mountain (Lorule) Weather Vane", regions::lorule::death::mountain::SUBREGION),
                    check!("[Mai] Lorule Mountain E Wall", regions::lorule::death::mountain::SUBREGION, |p| p
                        .can_merge()),
                ],
                vec![
                    fast_travel_lorule(),
                    crack_left(Crack::RossosOreMineLorule, crack_map, false),
                    crack_right(Crack::RossosOreMineLorule, crack_map, false),
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
                    old_path(IceCaveNorthWest, Some(|p| p.has_tornado_rod()), None, None, None, None),
                    old_path(IceCaveSouthWest, Some(|p| p.has_tornado_rod()), None, None, None, None),
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
                    old_path(IceCaveWest, Some(|p| p.has_tornado_rod()), None, Some(|p| p.has_boots()), None, None),
                ],
            ),
        ),
        (
            Location::FloatingIslandLorule,
            location(
                "Floating Island Lorule",
                vec![],
                vec![
                    fast_travel_lorule(),
                    edge!(IceCaveNorthWest),
                    crack_left(Crack::FloatingIslandLorule, crack_map, false),
                    crack_right(Crack::FloatingIslandLorule, crack_map, false),
                ],
            ),
        ),
        (
            IceCaveSouthWest,
            location("Ice Cave South West", vec![], vec![edge!(IceCaveWest), edge!(LoruleDeathEastLedgeUpper)]),
        ),
        (
            LoruleDeathEastLedgeUpper,
            location(
                "Lorule Death Mountain East Upper Ledge",
                vec![check!("Ice Cave Ledge", regions::lorule::death::mountain::SUBREGION, |p| p.can_merge())],
                vec![
                    fast_travel_lorule(),
                    edge!(IceCaveWest),
                    edge!(LoruleDeathEastLedgeLower),
                    old_path(Location::RossosOreMineLorule, None, None, Some(|p| p.has_nice_bombs()), None, None),
                ],
            ),
        ),
        (
            LoruleDeathEastLedgeLower,
            location(
                "Lorule Death Mountain East Lower Ledge",
                vec![check!("[Mai] Ice Cave Ledge", regions::lorule::death::mountain::SUBREGION, |p| p
                    .can_destroy_skull())],
                vec![fast_travel_lorule(), edge!(IceCaveSouth)],
            ),
        ),
        (
            LoruleDeathEastTop,
            location(
                "Lorule Death Mountain East Top",
                vec![
                    check!("Ice Ruins Weather Vane", regions::lorule::death::mountain::SUBREGION),
                    check!("Behind Ice Gimos", regions::lorule::death::mountain::SUBREGION, |p| p.has_fire_rod()),
                    check!("[Mai] Outside Ice Ruins", regions::lorule::death::mountain::SUBREGION, |p| p.can_merge()),
                    ghost(HintGhost::IceRuinsOutside),
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(IceCaveCenter),
                    edge!(IceRuinsFoyer, |p| p.has_fire_rod() && p.hearts(9.0)),
                ],
            ),
        ),
    ])
}

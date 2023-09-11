use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{
    check, edge, fast_travel_hyrule, fast_travel_lorule, goal, location, old_check, old_path,
    portal_std,
};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            DesertPalaceFoyer,
            location(
                "Desert Palace Entrance",
                vec![old_check(
                    LocationInfo::new(
                        "[DP] (1F) Entrance",
                        regions::dungeons::desert::palace::SUBREGION,
                    ),
                    Some(|p| p.has_sand_rod() && p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    edge!(DesertPalaceWeatherVane),
                    old_path(
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
                    check!(
                        "[DP] (1F) Sand Switch Room",
                        regions::dungeons::desert::palace::SUBREGION
                    ),
                    check!(
                        "[DP] (1F) Sand Room (North)",
                        regions::dungeons::desert::palace::SUBREGION
                    ),
                    check!(
                        "[DP] (1F) Sand Room (South)",
                        regions::dungeons::desert::palace::SUBREGION
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (1F) Behind Rocks",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (1F) Big Chest (Behind Wall)",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.has_desert_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    old_path(
                        DesertPalaceFoyer,
                        Some(|p| p.has_sand_rod() && p.can_attack()),
                        Some(|p| p.has_sand_rod() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    old_path(
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
                    edge!(DesertPalaceWeatherVane),
                    edge!(DesertPalace1F),
                    edge!(DesertPalace2F),
                ],
            ),
        ),
        (
            DesertPalace2F,
            location(
                "Desert Palace 2F",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[DP] (2F) Under Rock (Left)",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (2F) Under Rock (Right)",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (2F) Under Rock (Ball Room)",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.has_titans_mitt()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check!("[DP] (2F) Beamos Room", regions::dungeons::desert::palace::SUBREGION),
                    check!(
                        "[DP] (2F) Red/Blue Switches",
                        regions::dungeons::desert::palace::SUBREGION
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (2F) Big Chest (Puzzle)",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (2F) Leever Room",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.has_desert_keys(3)),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
                vec![
                    edge!(DesertPalaceMidwayLedge),
                    old_path(
                        DesertPalace1F,
                        Some(|p| p.can_attack()),      // midway
                        Some(|p| p.has_lamp_or_net()), // midway
                        None,
                        None,
                        None,
                    ),
                    old_path(
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
                    check!(
                        "[DP] (3F) Behind Falling Sand",
                        regions::dungeons::desert::palace::SUBREGION
                    ),
                    old_check(
                        LocationInfo::new(
                            "[DP] (3F) Armos Room",
                            regions::dungeons::desert::palace::SUBREGION,
                        ),
                        Some(|p| p.can_attack()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(DesertPalace2F),
                    old_path(
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
                    old_path(DesertPalace3F, Some(|p| p.has_sand_rod()), None, None, None, None),
                    edge!(DesertZaganagaLedge),
                ],
            ),
        ),
        (
            DesertZaganagaLedge,
            location(
                "Desert Zaganaga Ledge",
                vec![],
                vec![fast_travel_hyrule(), edge!(DesertPalaceExit3F), portal_std(ZaganagasArena)],
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
                    old_path(
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
                    check!("Zaganaga", regions::dungeons::desert::palace::SUBREGION), // Do not use [DP] prefix
                    check!("Desert Palace Prize", regions::dungeons::desert::palace::SUBREGION),
                    goal!("Zaganaga Defeated", Goal::Zaganaga),
                ],
                vec![fast_travel_lorule()],
            ),
        ),
    ])
}

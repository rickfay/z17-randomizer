use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::world::{check, crack_left, crack_right, edge, fast_travel_hyrule, fast_travel_lorule, goal, location};
use crate::LocationInfo;
use crate::{regions, CrackMap};

use crate::filler::cracks::Crack::{DesertPalace, Zaganaga};
use std::collections::HashMap;

/// Desert Palace World Graph
pub(crate) fn graph(crack_map: &CrackMap) -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            DesertPalaceFoyer,
            location(
                "Desert Palace Entrance",
                vec![check!("[DP] (1F) Entrance", regions::dungeons::desert::palace::SUBREGION => {
                    normal: |p| p.has_sand_rod() && p.can_merge(),
                    hell: |p| p.has_sand_rod() && p.has_tornado_rod(),
                })],
                vec![
                    edge!(DesertPalaceWeatherVane),
                    edge!(DesertPalace1F => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.can_attack(),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod() && p.can_attack(),
                    }),
                ],
            ),
        ),
        (
            DesertPalace1F,
            location(
                "Desert Palace 1F",
                vec![
                    check!("[DP] (1F) Sand Switch Room", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge(),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod(),
                    }),
                    check!(
                        "[DP] (1F) Sand Room (North)",
                        regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge(),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod(),
                    }
                    ),
                    check!(
                        "[DP] (1F) Sand Room (South)",
                        regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge(),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod(),
                    }
                    ),
                    check!(
                        "[DP] (1F) Behind Rocks",
                        regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.has_titans_mitt(),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod(),
                    }
                    ),
                    check!(
                        "[DP] (1F) Big Chest (Behind Wall)",
                        regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.has_desert_keys(1),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod() && p.has_desert_keys(1),
                    }),
                ],
                vec![
                    edge!(DesertPalaceFoyer, |p| p.has_sand_rod() && p.can_attack()),
                    edge!(DesertPalaceMidwayLedge, |p| p.has_desert_keys(2) && p.has_titans_mitt()),
                ],
            ),
        ),
        (
            DesertPalaceMidwayLedge,
            location(
                "Desert Palace Midway Ledge",
                None,
                vec![
                    fast_travel_hyrule(),
                    edge!(DesertPalaceWeatherVane),
                    edge!(DesertPalace1F, |p| p.hearts(9.0)),
                    edge!(DesertPalace2FMiniboss, |p| p.hearts(9.0)),
                ],
            ),
        ),
        (
            DesertPalace2FMiniboss,
            location(
                "Desert Palace 2F Miniboss",
                None,
                vec![
                    edge!(
                        DesertPalaceMidwayLedge => {
                            glitched: |_| true,
                        }
                    ),
                    edge!(DesertPalace1F, |p| p.can_attack()),
                    edge!(DesertPalace2F => {
                        normal: |p| p.can_attack() && p.has_sand_rod() && p.can_merge(),
                        glitched: |p| p.can_attack() && p.has_sand_rod() && p.has_boots(),
                    }),
                ],
            ),
        ),
        (
            DesertPalace2F,
            location(
                "Desert Palace 2F",
                vec![
                    check!("[DP] (2F) Under Rock (Left)", regions::dungeons::desert::palace::SUBREGION, |p| p
                        .has_titans_mitt()),
                    check!("[DP] (2F) Under Rock (Right)", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.has_titans_mitt(),
                        adv_glitched: |p| p.has_sand_rod() && p.has_tornado_rod() && p.has_titans_mitt(),
                    }),
                    check!("[DP] (2F) Under Rock (Ball Room)", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.has_titans_mitt(),
                        adv_glitched: |p| p.has_sand_rod() && p.has_tornado_rod() && p.has_titans_mitt(),
                    }),
                    check!("[DP] (2F) Beamos Room", regions::dungeons::desert::palace::SUBREGION, |p| p.has_sand_rod()),
                    check!("[DP] (2F) Red/Blue Switches", regions::dungeons::desert::palace::SUBREGION, |p| p
                        .has_sand_rod()),
                    check!(
                        "[DP] (2F) Big Chest (Puzzle)",
                        regions::dungeons::desert::palace::SUBREGION => {
                            normal: |p| p.has_sand_rod() && p.can_merge(),
                            adv_glitched: |p| p.has_tornado_rod(),
                        }
                    ),
                    check!("[DP] (2F) Leever Room", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_desert_keys(3),
                        adv_glitched: |p| p.has_tornado_rod(),
                    }),
                ],
                vec![
                    edge!(DesertPalace2FMiniboss),
                    edge!(DesertPalace3F => {
                        normal: |p| p.has_desert_keys(4) && p.can_merge() && p.has_sand_rod(),
                        adv_glitched: |p| p.has_tornado_rod() && p.has_boots(),
                    }),
                ],
            ),
        ),
        (
            DesertPalace3F,
            location(
                "Desert Palace 3F",
                vec![
                    check!("[DP] (3F) Behind Falling Sand", regions::dungeons::desert::palace::SUBREGION, |p| p
                        .has_sand_rod()),
                    check!("[DP] (3F) Armos Room", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_attack(),
                        hard: |p| p.has_sand_rod(),
                    }),
                ],
                vec![
                    edge!(DesertPalace2F),
                    edge!(DesertPalaceExit3F => {
                        normal: |p| {
                            p.has_desert_keys(5)
                                && p.has_desert_big_key()
                                && (p.progression_enemies() || p.has_bombs())
                        },
                        adv_glitched: |p| p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            DesertPalaceExit3F,
            location(
                "Desert Palace Exit 3F",
                None,
                vec![edge!(DesertPalace3F, |p| p.has_sand_rod()), edge!(DesertZaganagaLedge)],
            ),
        ),
        (
            DesertZaganagaLedge,
            location(
                "Desert Zaganaga Ledge",
                None,
                vec![
                    fast_travel_hyrule(),
                    edge!(DesertPalaceExit3F, |p| p.hearts(9.0)),
                    crack_left(DesertPalace, crack_map, false),
                    crack_right(DesertPalace, crack_map, false),
                ],
            ),
        ),
        (
            ZaganagasArena,
            location(
                "Zaganaga's Arena",
                None,
                vec![
                    fast_travel_lorule(),
                    crack_left(Zaganaga, crack_map, false),
                    crack_right(Zaganaga, crack_map, false),
                    edge!(MiseryMireRewardBasket => {
                        normal: |p| p.has_sand_rod() && p.hearts(9.0) && (p.has_master_sword() || (p.swordless_mode() && p.can_attack())),
                        hard: |p| p.has_sand_rod() && p.can_attack(),
                        hell: |p| p.has_bow() || p.has_master_sword(),
                    }),
                ],
            ),
        ),
        (
            MiseryMireRewardBasket,
            location(
                "Misery Mire Reward Basket",
                vec![
                    check!("[DP] Zaganaga", regions::dungeons::desert::palace::SUBREGION), // Do not use [DP] prefix
                    check!("[DP] Prize", regions::dungeons::desert::palace::SUBREGION),
                    goal!("Zaganaga Defeated", Goal::Zaganaga),
                ],
                vec![fast_travel_lorule()],
            ),
        ),
    ])
}

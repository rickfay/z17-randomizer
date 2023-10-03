use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{
    check, edge, fast_travel_hyrule, fast_travel_lorule, goal, location, portal_std,
};
use crate::LocationInfo;

use std::collections::HashMap;

/// Desert Palace World Graph
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            DesertPalaceFoyer,
            location(
                "Desert Palace Entrance",
                vec![
                    check!("[DP] (1F) Entrance", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge(),
                        hell: |p| p.has_sand_rod() && p.has_tornado_rod(),
                    }),
                ],
                vec![
                    edge!(DesertPalaceWeatherVane),
                    edge!(DesertPalace1F => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.can_attack(),
                        hard: |p| p.has_sand_rod() && p.can_merge() && p.has_lamp_or_net(),
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
                    edge!(DesertPalaceFoyer => {
                        normal: |p| p.has_sand_rod() && p.can_attack(),
                        hard: |p| p.has_sand_rod() && p.has_lamp_or_net(),
                    }),
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
                    edge!(DesertPalace1F),
                    edge!(DesertPalace2FMiniboss),
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
                    edge!(DesertPalace1F => {
                        normal: |p| p.can_attack(), // midway
                        hard: |p| p.has_lamp_or_net(), // midway
                    }),
                    edge!(DesertPalace2F => {
                        normal: |p| p.can_attack() && p.has_sand_rod() && p.can_merge(),
                        hard: |p| p.has_lamp_or_net() && p.has_sand_rod() && p.can_merge(),
                        glitched: |p| (p.can_attack() || p.has_lamp_or_net()) && p.has_sand_rod() && p.has_boots(),
                    }),
                ],
            ),
        ),
        (
            DesertPalace2F,
            location(
                "Desert Palace 2F",
                vec![
                    check!(
                        "[DP] (2F) Under Rock (Left)",
                        regions::dungeons::desert::palace::SUBREGION,
                        |p| p.has_titans_mitt()
                    ),
                    check!("[DP] (2F) Under Rock (Right)", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.has_titans_mitt(),
                        adv_glitched: |p| p.has_sand_rod() && p.has_tornado_rod() && p.has_titans_mitt(),
                    }),
                    check!("[DP] (2F) Under Rock (Ball Room)", regions::dungeons::desert::palace::SUBREGION => {
                        normal: |p| p.has_sand_rod() && p.can_merge() && p.has_titans_mitt(),
                        adv_glitched: |p| p.has_sand_rod() && p.has_tornado_rod() && p.has_titans_mitt(),
                    }),
                    check!(
                        "[DP] (2F) Beamos Room",
                        regions::dungeons::desert::palace::SUBREGION,
                        |p| p.has_sand_rod()
                    ),
                    check!(
                        "[DP] (2F) Red/Blue Switches",
                        regions::dungeons::desert::palace::SUBREGION,
                        |p| p.has_sand_rod()
                    ),
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
                    check!(
                        "[DP] (3F) Behind Falling Sand",
                        regions::dungeons::desert::palace::SUBREGION,
                        |p| p.has_sand_rod()
                    ),
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
                vec![fast_travel_hyrule(), edge!(DesertPalaceExit3F), portal_std(ZaganagasArena)],
            ),
        ),
        (
            ZaganagasArena,
            location(
                "Zaganaga's Arena",
                None,
                vec![
                    fast_travel_lorule(),
                    portal_std(DesertZaganagaLedge),
                    edge!(MiseryMireRewardBasket => {
                        normal: |p| p.has_sand_rod() && p.has_master_sword(),
                        hell: |p| (p.has_sand_rod() && p.can_attack()) || p.has_bow() || p.has_master_sword(),
                    }),
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

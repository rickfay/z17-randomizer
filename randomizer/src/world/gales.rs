use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::regions;
use crate::world::{check, edge, goal, location};
use crate::LocationInfo;
use std::collections::HashMap;

/// House of Gales World Graph
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            HouseOfGalesFoyer,
            location(
                "House of Gales Entrance",
                None,
                vec![edge!(HouseOfGalesIsland), edge!(HouseOfGalesEast1F, |p| p.has_tornado_rod())],
            ),
        ),
        (
            HouseOfGalesEast1F,
            location(
                "House of Gales East 1F",
                vec![
                    check!("[HG] (1F) Torches", regions::dungeons::house::gales::SUBREGION, |p| p.has_fire_source()),
                    check!("[HG] (1F) Switch Room", regions::dungeons::house::gales::SUBREGION),
                    check!("[HG] (1F) Fire Bubbles", regions::dungeons::house::gales::SUBREGION => {
                        normal: |p| p.can_merge() && p.can_attack_fireproof(),
                        hard: |p| p.can_merge() && p.has_net(),
                    }),
                ],
                vec![
                    edge!(HouseOfGalesFoyer),
                    edge!(HouseOfGalesWest1F => {
                        normal: |p| p.has_gales_keys(1) && p.can_merge(),
                        hard: |p| p.has_gales_keys(1), // TRod jump onto blocks
                    }),
                ],
            ),
        ),
        (
            HouseOfGalesWest1F,
            location(
                "House of Gales West 1F",
                vec![
                    check!("[HG] (1F) West Room", regions::dungeons::house::gales::SUBREGION),
                    check!("[HG] (1F) West Room Secret", regions::dungeons::house::gales::SUBREGION, |p| p.can_merge()),
                ],
                vec![
                    edge!(HouseOfGalesEast1F),
                    edge!(HouseOfGales2F => {
                        normal: |p| p.can_hit_hog_1f_switch(), // oddly specific switch hitting requirements
                        hard: |p| p.has_master_sword(),
                    }),
                ],
            ),
        ),
        (
            HouseOfGales2F,
            location(
                "House of Gales 2F",
                vec![
                    check!("[HG] (2F) Narrow Ledge", regions::dungeons::house::gales::SUBREGION => {
                        normal: |p| p.can_merge() || p.has_boomerang() || p.has_hookshot(),
                        hard: |_| true, // can just grab it with TRod
                    }),
                    check!("[HG] (2F) Big Chest", regions::dungeons::house::gales::SUBREGION),
                    check!("[HG] (2F) Fire Ring", regions::dungeons::house::gales::SUBREGION => {
                        normal: |p| p.can_merge() && p.has_gales_keys(3), // should really be 2, but defending against bad key use
                        glitched: |p| p.can_merge() && p.has_boots(),
                        hell: |p| p.can_merge(), // awful Armos Boost
                    }),
                ],
                vec![
                    edge!(HouseOfGalesWest1F),
                    edge!(HouseOfGales3F => {
                        normal: |p| p.has_gales_keys(3) && p.can_attack_fireproof() && p.can_hit_switch() && p.can_merge(),
                        hard: |p| p.has_gales_keys(3) && p.has_net() && p.can_hit_switch() && p.can_merge(),
                        glitched: |p| p.can_merge(), // Skip Skip Skip
                    }),
                ],
            ),
        ),
        (
            HouseOfGales3F,
            location(
                "House of Gales 3F",
                vec![
                    check!("[HG] (3F) Fire Bubbles", regions::dungeons::house::gales::SUBREGION, |p| p
                        .has_fire_source()),
                    check!("[HG] (3F) Rat Room", regions::dungeons::house::gales::SUBREGION, |p| p.has_fire_source()
                        || p.has_gales_keys(4)),
                ],
                vec![edge!(HouseOfGales2F), edge!(HouseOfGalesBoss, |p| p.has_gales_keys(4) && p.has_gales_big_key())],
            ),
        ),
        (
            HouseOfGalesBoss,
            location("House of Gales Boss", None, vec![edge!(HouseOfGalesPostBoss, |p| p.can_defeat_margomill())]),
        ),
        (
            HouseOfGalesPostBoss,
            location(
                "Margomill Defeated",
                vec![
                    check!("[HG] Margomill", regions::dungeons::house::gales::SUBREGION),
                    check!("[HG] Prize", regions::dungeons::house::gales::SUBREGION),
                    goal!("Margomill Defeated", Goal::Margomill),
                ],
                None,
            ),
        ),
    ])
}

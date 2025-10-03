use crate::LocationInfo;
use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::world::{check, door, edge, goal, location};
use crate::{DoorMap, regions};

use std::collections::HashMap;

pub(crate) fn graph(door_map: &DoorMap) -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            TowerOfHeraFoyer,
            location(
                "Tower of Hera Entrance",
                vec![],
                vec![door!(TowerOfHeraExit, door_map), edge!(TowerOfHeraBottom, |p| p.has_hammer())],
            ),
        ),
        (
            TowerOfHeraBottom,
            location(
                "Tower of Hera Bottom",
                vec![
                    check!("[TH] (1F) Outside", regions::dungeons::tower::hera::SUBREGION => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.has_sword() && p.has_bombs() && p.has_tornado_rod(),
                    }),
                    check!("[TH] (1F) Center", regions::dungeons::tower::hera::SUBREGION => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.has_sword() && p.has_bombs(),
                    }),
                    check!("[TH] (3F) Platform", regions::dungeons::tower::hera::SUBREGION => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.has_sword() && p.has_bombs(),
                    }),
                ],
                vec![
                    edge!(TowerOfHeraFoyer, |p| p.has_hammer()),
                    edge!(TowerOfHeraMiddle => {
                        normal: |p| p.has_hera_keys(1) && p.can_merge(),
                        adv_glitched: |p| p.has_sword() && p.has_bombs() && p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            TowerOfHeraMiddle,
            location(
                "Tower of Hera Middle",
                vec![
                    check!("[TH] (5F) Red/Blue Switches", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (6F) Right Mole", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (6F) Left Mole", regions::dungeons::tower::hera::SUBREGION),
                ],
                vec![
                    edge!(TowerOfHeraBottom),
                    edge!(TowerOfHeraTop => {
                        normal: |p| p.has_hera_keys(2),
                        adv_glitched: |p| p.has_bombs() && p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            TowerOfHeraTop,
            location(
                "Tower of Hera Top",
                vec![
                    check!("[TH] (7F) Outside (Ledge)", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (8F) Fairy Room", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (11F) Big Chest", regions::dungeons::tower::hera::SUBREGION),
                ],
                vec![edge!(TowerOfHeraMiddle), edge!(TowerOfHeraBoss, |p| p.has_hera_big_key())],
            ),
        ),
        (
            TowerOfHeraBoss,
            location("Tower of Hera Boss", vec![], vec![edge!(TowerOfHeraPostBoss, |p| p.can_defeat_moldorm())]),
        ),
        (
            TowerOfHeraPostBoss,
            location(
                "Tower of Hera Post Boss",
                vec![
                    check!("[TH] Moldorm", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] Prize", regions::dungeons::tower::hera::SUBREGION),
                    goal!("Moldorm", Goal::Moldorm),
                ],
                vec![],
            ),
        ),
    ])
}

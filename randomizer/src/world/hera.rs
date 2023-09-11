use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, goal, location, old_check, old_path};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            TowerOfHeraFoyer,
            location(
                "Tower of Hera Entrance",
                vec![],
                vec![
                    edge!(DeathMountainWestTop),
                    old_path(TowerOfHeraBottom, Some(|p| p.has_hammer()), None, None, None, None),
                ],
            ),
        ),
        (
            TowerOfHeraBottom,
            location(
                "Tower of Hera Bottom",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[TH] (1F) Outside",
                            regions::dungeons::tower::hera::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs() && p.has_tornado_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[TH] (1F) Center",
                            regions::dungeons::tower::hera::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[TH] (3F) Platform",
                            regions::dungeons::tower::hera::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_sword() && p.has_bombs()),
                        None,
                    ),
                ],
                vec![
                    old_path(TowerOfHeraFoyer, Some(|p| p.has_hammer()), None, None, None, None),
                    old_path(
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
                    check!(
                        "[TH] (5F) Red/Blue Switches",
                        regions::dungeons::tower::hera::SUBREGION
                    ),
                    check!("[TH] (6F) Right Mole", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (6F) Left Mole", regions::dungeons::tower::hera::SUBREGION),
                ],
                vec![
                    edge!(TowerOfHeraBottom),
                    old_path(
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
                    check!("[TH] (7F) Outside (Ledge)", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (8F) Fairy Room", regions::dungeons::tower::hera::SUBREGION),
                    check!("[TH] (11F) Big Chest", regions::dungeons::tower::hera::SUBREGION),
                ],
                vec![
                    edge!(TowerOfHeraMiddle),
                    old_path(
                        TowerOfHeraBoss,
                        Some(|p| p.has_hera_big_key()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            TowerOfHeraBoss,
            location(
                "Tower of Hera Boss",
                vec![],
                vec![old_path(
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
                    check!("[TH] Moldorm", regions::dungeons::tower::hera::SUBREGION),
                    check!("Tower of Hera Prize", regions::dungeons::tower::hera::SUBREGION),
                    goal!("Moldorm", Goal::Moldorm),
                ],
                vec![],
            ),
        ),
    ])
}

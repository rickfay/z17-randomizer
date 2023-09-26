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
            HouseOfGalesFoyer,
            location(
                "House of Gales Entrance",
                vec![],
                vec![
                    edge!(HouseOfGalesIsland),
                    old_path(
                        HouseOfGalesEast1F,
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
            HouseOfGalesEast1F,
            location(
                "House of Gales East 1F",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[HG] (1F) Torches",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[HG] (1F) Switch Room",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        Some(|_| true), // might need to deathwarp to escape
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[HG] (1F) Fire Bubbles",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.can_merge() && p.can_attack_fireproof()),
                        Some(|p| p.can_merge() && p.has_net()),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(HouseOfGalesFoyer),
                    old_path(
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
                    check!("[HG] (1F) West Room", regions::dungeons::house::gales::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[HG] (1F) West Room Secret",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(HouseOfGalesEast1F),
                    old_path(
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
                    old_check(
                        LocationInfo::new(
                            "[HG] (2F) Narrow Ledge",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        Some(|_| true), // can just grab it with TRod
                        None,
                        None,
                        None,
                    ),
                    check!("[HG] (2F) Big Chest", regions::dungeons::house::gales::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[HG] (2F) Fire Ring",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.can_merge() && p.has_gales_keys(3)), // should really be 2, but defending against bad key use
                        None,
                        Some(|p| p.can_merge() && p.has_boots()),
                        None,
                        Some(|p| p.can_merge()), // awful Armos Boost
                    ),
                ],
                vec![
                    edge!(HouseOfGalesWest1F),
                    old_path(
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
                    old_check(
                        LocationInfo::new(
                            "[HG] (3F) Fire Bubbles",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.has_fire_source()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[HG] (3F) Rat Room",
                            regions::dungeons::house::gales::SUBREGION,
                        ),
                        Some(|p| p.has_fire_source() || p.has_gales_keys(4)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(HouseOfGales2F),
                    old_path(
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
                vec![old_path(
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
                    check!("[HG] Margomill", regions::dungeons::house::gales::SUBREGION),
                    check!("House of Gales Prize", regions::dungeons::house::gales::SUBREGION),
                    goal!("Margomill Defeated", Goal::Margomill),
                ],
                vec![],
            ),
        ),
    ])
}

use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::regions;
use crate::world::{check, edge, fast_travel_lorule, goal, location, old_check, old_path};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            TurtleRockFoyer,
            location(
                "Turtle Rock Foyer",
                vec![],
                vec![
                    edge!(TurtleRockFrontDoor),
                    old_path(TurtleRockMain, Some(|p| p.has_ice_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockMain,
            location(
                "Turtle Rock Main",
                vec![
                    check!("[TR] (1F) Center", regions::dungeons::turtle::rock::SUBREGION),
                    old_check(
                        LocationInfo::new("[TR] (1F) Northeast Ledge", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (1F) Southeast Chest", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_nice_bombs() && p.has_tornado_rod()), // bombrod into warp tile
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (1F) Defeat Flamolas", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (1F) Northwest Room", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (1F) Grate Chest", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check!("[TR] (B1) Northeast Room", regions::dungeons::turtle::rock::SUBREGION),
                    old_check(
                        LocationInfo::new("[TR] (B1) Grate Chest (Small)", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None, // I swear there was a bombrod you could do here, idk, leaving it off for now
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (B1) Big Chest (Top)", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.has_turtle_keys(3) && p.can_merge() && p.can_hit_shielded_switch()),
                        Some(|p| (p.has_turtle_keys(3) && p.can_merge())), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (B1) Big Chest (Center)", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge() && p.can_hit_shielded_switch()),
                        Some(|p| p.can_merge()), // hit switch with pots
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new("[TR] (B1) Platform", regions::dungeons::turtle::rock::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check!("[TR] (1F) Under Center", regions::dungeons::turtle::rock::SUBREGION),
                    check!("[TR] (B1) Under Center", regions::dungeons::turtle::rock::SUBREGION),
                ],
                vec![
                    old_path(TurtleRockFoyer, Some(|p| p.has_ice_rod()), None, None, None, None),
                    old_path(TurtleRockLeftBalconyPath, Some(|p| p.can_merge()), None, None, None, None),
                    old_path(TurtleRockRightBalconyPath, Some(|p| p.can_merge()), None, None, None, None),
                    old_path(
                        TurtleRockBoss,
                        Some(|p| p.has_turtle_keys(3) && p.can_merge() && p.has_turtle_big_key()),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_nice_bombs()),
                        None,
                    ),
                ],
            ),
        ),
        (
            TurtleRockLeftBalconyPath,
            location(
                "Turtle Rock Left Balcony Path",
                vec![],
                vec![
                    old_path(TurtleRockMain, Some(|p| p.has_ice_rod()), None, None, None, None),
                    old_path(TurtleRockLeftBalcony, Some(|p| p.has_ice_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockLeftBalcony,
            location(
                "[TR] Left Balcony",
                vec![
                    check!("[TR] Left Balcony", regions::dungeons::turtle::rock::SUBREGION), // Do not use [TR] prefix
                ],
                vec![fast_travel_lorule(), edge!(TurtleRockLeftBalconyPath, |p| p.hearts(9.0))],
            ),
        ),
        (
            TurtleRockRightBalconyPath,
            location(
                "Turtle Rock Right Balcony Path",
                vec![],
                vec![
                    old_path(TurtleRockMain, Some(|p| p.has_ice_rod()), None, None, None, None),
                    old_path(TurtleRockRightBalcony, Some(|p| p.has_ice_rod()), None, None, None, None),
                ],
            ),
        ),
        (
            TurtleRockRightBalcony,
            location(
                "Turtle Rock Right Balcony",
                vec![],
                vec![fast_travel_lorule(), edge!(TurtleRockRightBalconyPath, |p| p.hearts(9.0))],
            ),
        ),
        (
            TurtleRockBoss,
            location(
                "Turtle Rock Boss",
                vec![],
                vec![old_path(TurtleRockPostBoss, Some(|p| p.can_defeat_grinexx()), None, None, None, None)],
            ),
        ),
        (
            TurtleRockPostBoss,
            location(
                "Turtle Rock Post Boss",
                vec![
                    check!("[TR] Grinexx", regions::dungeons::turtle::rock::SUBREGION),
                    check!("[TR] Prize", regions::dungeons::turtle::rock::SUBREGION),
                    goal!("Grinexx", Goal::Grinexx),
                ],
                vec![],
            ),
        ),
    ])
}

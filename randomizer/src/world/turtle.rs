use crate::LocationInfo;
use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::world::{check, door, edge, fast_travel_lorule, goal, location};
use crate::{DoorMap, regions};
use std::collections::HashMap;

pub(crate) fn graph(door_map: &DoorMap) -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            TurtleRockFoyer,
            location("Turtle Rock Foyer", vec![], vec![
                door!(TurtleRockExit, door_map),
                edge!(TurtleRockMain, |p| p.has_ice_rod()),
            ]),
        ),
        (
            TurtleRockMain,
            location(
                "Turtle Rock Main",
                vec![
                    check!("[TR] (1F) Center", regions::dungeons::turtle::rock::SUBREGION),
                    check!("[TR] (1F) Northeast Ledge", regions::dungeons::turtle::rock::SUBREGION, |p| p.can_merge()
                        || p.has_boomerang()
                        || p.has_hookshot()),
                    check!("[TR] (1F) Southeast Chest", regions::dungeons::turtle::rock::SUBREGION => {
                        normal: |p| p.can_merge(),
                        glitched: |p| p.has_nice_bombs() && p.has_tornado_rod(), // bombrod into warp tile
                    }),
                    check!("[TR] (1F) Defeat Flamolas", regions::dungeons::turtle::rock::SUBREGION, |p| p.can_merge()),
                    check!("[TR] (1F) Northwest Room", regions::dungeons::turtle::rock::SUBREGION, |p| p.can_merge()),
                    check!("[TR] (1F) Grate Chest", regions::dungeons::turtle::rock::SUBREGION, |p| p.can_merge()),
                    check!("[TR] (B1) Northeast Room", regions::dungeons::turtle::rock::SUBREGION),
                    check!("[TR] (B1) Grate Chest (Small)", regions::dungeons::turtle::rock::SUBREGION, |p| p
                        .can_merge()),
                    check!("[TR] (B1) Big Chest (Top)", regions::dungeons::turtle::rock::SUBREGION => {
                        normal: |p| p.has_turtle_keys(1) && p.can_merge() && p.can_hit_shielded_switch(),
                        hard: |p| (p.has_turtle_keys(1) && p.can_merge()), // hit switch with pots
                    }),
                    check!("[TR] (B1) Big Chest (Center)", regions::dungeons::turtle::rock::SUBREGION => {
                        normal: |p| p.can_merge() && p.can_hit_shielded_switch(),
                        hard: |p| p.can_merge(), // hit switch with pots
                    }),
                    check!("[TR] (B1) Platform", regions::dungeons::turtle::rock::SUBREGION, |p| p.can_merge()),
                    check!("[TR] (1F) Under Center", regions::dungeons::turtle::rock::SUBREGION),
                    check!("[TR] (B1) Under Center", regions::dungeons::turtle::rock::SUBREGION),
                ],
                vec![
                    edge!(TurtleRockFoyer, |p| p.has_ice_rod()),
                    edge!(TurtleRockLeftBalconyPath, |p| p.can_merge()),
                    edge!(TurtleRockRightBalconyPath, |p| p.can_merge()),
                    edge!(TurtleRockBoss => {
                        normal: |p| p.has_turtle_keys(3) && p.can_merge() && p.has_turtle_big_key(),
                        adv_glitched: |p| p.has_tornado_rod() && p.has_nice_bombs(),
                    }),
                ],
            ),
        ),
        (
            TurtleRockLeftBalconyPath,
            location("Turtle Rock Left Balcony Path", vec![], vec![
                edge!(TurtleRockMain, |p| p.has_ice_rod()),
                edge!(TurtleRockLeftBalcony, |p| p.has_ice_rod()),
            ]),
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
            location("Turtle Rock Right Balcony Path", vec![], vec![
                edge!(TurtleRockMain, |p| p.has_ice_rod()),
                edge!(TurtleRockRightBalcony, |p| p.has_ice_rod()),
            ]),
        ),
        (
            TurtleRockRightBalcony,
            location("Turtle Rock Right Balcony", vec![], vec![
                fast_travel_lorule(),
                edge!(TurtleRockRightBalconyPath, |p| p.hearts(9.0)),
            ]),
        ),
        (
            TurtleRockBoss,
            location("Turtle Rock Boss", vec![], vec![edge!(TurtleRockPostBoss, |p| p.can_defeat_grinexx())]),
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

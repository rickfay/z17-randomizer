use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, fast_travel_lorule, goal, location};
use crate::LocationInfo;

use std::collections::HashMap;

/// Skull Woods World Graph
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            SkullWoodsFoyer,
            location(
                "Skull Woods Foyer",
                None,
                vec![
                    edge!(SkullWoodsOverworld),
                    edge!(SkullWoodsMain, |p| p.has_lamp() || p.lampless()),
                ],
            ),
        ),
        (
            SkullWoodsMain,
            location(
                "Skull Woods",
                vec![
                    check!("[SW] (B1) South Chest", regions::dungeons::skull::woods::SUBREGION),
                    check!(
                        "[SW] (B1) Gibdo Room (Lower)",
                        regions::dungeons::skull::woods::SUBREGION
                    ),
                    check!(
                        "[SW] (B1) Gibdo Room (Hole)",
                        regions::dungeons::skull::woods::SUBREGION,
                        |p| p.has_skull_keys(1)
                    ),
                    check!(
                        "[SW] (B1) Grate Room",
                        regions::dungeons::skull::woods::SUBREGION,
                        |p| p.has_skull_keys(1)
                            && (p.progression_enemies() || p.break_floor_tiles())
                    ),
                ],
                vec![
                    edge!(SkullWoodsFoyer),
                    edge!(SkullWoodsB2, |p| p.has_skull_keys(2)
                        && p.can_merge()
                        && (p.progression_enemies() || p.break_floor_tiles())),
                ],
            ),
        ),
        (
            SkullWoodsB2,
            location(
                "Skull Woods B2",
                None,
                vec![
                    edge!(SkullWoodsMain => {
                        normal: |p| p.can_merge() && p.can_attack(),
                        hard: |p| p.can_merge() && p.has_lamp_or_net(),
                    }),
                    edge!(SkullWoodsElevatorHallway => {
                        normal: |p| p.can_merge() && p.can_attack(),
                        hard: |p| p.can_merge() && p.has_lamp_or_net(),
                    }),
                ],
            ),
        ),
        (
            SkullWoodsElevatorHallway,
            location(
                "Skull Woods Elevator Hallway",
                vec![check!(
                    "[SW] (B2) Moving Platform Room",
                    regions::dungeons::skull::woods::SUBREGION
                )],
                vec![edge!(SkullWoodsB2), edge!(SkullWoodsBossHallway, |p| p.has_skull_keys(3))],
            ),
        ),
        (
            SkullWoodsBossHallway,
            location(
                "Skull Woods Boss Hallway",
                None,
                vec![
                    edge!(SkullWoodsElevatorHallway),
                    edge!(SkullWoodsEastB1NorthFoyer => {
                        normal: |p| p.has_fire_source() && p.can_attack(),
                        hard: |p| p.has_lamp(),
                    }),
                    edge!(SkullWoodsBossRoom, |p| p.has_skull_big_key()),
                ],
            ),
        ),
        (
            SkullWoodsBossRoom,
            location(
                "Skull Woods Boss Room",
                vec![check!("[SW] Knucklemaster", regions::dungeons::skull::woods::SUBREGION => {
                    normal: |p| p.has_master_sword(),
                    hell: |p| p.can_technically_defeat_knucklemaster(),
                })],
                vec![
                    edge!(SkullWoodsBossHallway => {
                        normal: |p| p.has_master_sword(),
                        hell: |p| p.can_technically_defeat_knucklemaster(),
                    }),
                    edge!(SkullWoodsSeresGrove => {
                        normal: |p| p.has_master_sword(),
                        glitched: |p| p.has_tornado_rod(),
                        hell: |p| p.can_technically_defeat_knucklemaster(),
                    }),
                ],
            ),
        ),
        (
            SkullWoodsSeresGrove,
            location(
                "Skull Woods Seres Grove",
                vec![
                    check!("Skull Woods Prize", regions::dungeons::skull::woods::SUBREGION),
                    goal!("Knucklemaster", Goal::Knucklemaster),
                ],
                vec![edge!(SkullWoodsBossRoom)],
            ),
        ),
        (
            SkullWoodsEastB1NorthFoyer,
            location(
                "Skull Woods East B1 North Foyer",
                None,
                vec![edge!(SkullWoodsBossHallway), edge!(SkullWoodsEastB1North, |p| p.can_merge())],
            ),
        ),
        (
            SkullWoodsEastB1North,
            location(
                "Skull Woods East B1 North",
                vec![
                    // Eyeball dupe cannot be considered as it cannot be retried if missed
                    check!(
                        "[SW] (B1) Big Chest (Eyes)",
                        regions::dungeons::skull::woods::SUBREGION,
                        |p| p.has_skull_eyes()
                    ),
                    goal!("Skull Eye Right", Goal::SkullEyeRight),
                ],
                vec![
                    edge!(SkullWoodsEastB1NorthFoyer, |p| p.can_merge()),
                    edge!(SkullWoodsEastB1South, |p| p.has_skull_eye_right()),
                ],
            ),
        ),
        (
            SkullWoodsEastB1South,
            location(
                "Skull Woods East B1 South",
                None,
                vec![
                    edge!(SkullWoodsEastB1North, |p| p.can_merge() && p.has_skull_eye_right()),
                    edge!(SkullWoodsEastB1SouthFoyer, |p| p.can_merge()),
                ],
            ),
        ),
        (
            SkullWoodsEastB1SouthFoyer,
            location(
                "Skull Woods East B1 South Foyer",
                None,
                vec![edge!(SkullWoodsEastB1South, |p| p.can_merge()), edge!(SkullWoodsOutdoor3)],
            ),
        ),
        (
            SkullWoodsEastB1SouthLedges,
            location(
                "Skull Woods East B1 South Ledges",
                vec![
                    check!(
                        "[SW] (B1) Big Chest (Upper)",
                        regions::dungeons::skull::woods::SUBREGION,
                        |p| p.can_merge()
                    ),
                    goal!("Skull Eye Left", Goal::SkullEyeLeft, |p| p.can_merge()),
                ],
                vec![edge!(SkullWoodsEastB1South)],
            ),
        ),
        (
            SkullWoodsOutdoor3,
            location(
                "Skull Woods Outdoor Area 3",
                vec![
                    check!("Skull Woods Outdoor Chest", regions::dungeons::skull::woods::SUBREGION), // Do not use [SW] prefix
                ],
                vec![
                    fast_travel_lorule(),
                    edge!(SkullWoodsEastB1SouthFoyer),
                    edge!(SkullWoodsEastB1SouthLedges),
                ],
            ),
        ),
    ])
}

use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, fast_travel_lorule, goal, location, old_check, old_path};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            SkullWoodsFoyer,
            location(
                "Skull Woods Foyer",
                vec![],
                vec![
                    edge!(SkullWoodsOverworld),
                    old_path(
                        SkullWoodsMain,
                        Some(|p| p.has_lamp() || p.lampless()),
                        None,
                        None,
                        None,
                        None,
                    ),
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
                    old_check(
                        LocationInfo::new(
                            "[SW] (B1) Gibdo Room (Hole)",
                            regions::dungeons::skull::woods::SUBREGION,
                        ),
                        Some(|p| p.has_skull_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[SW] (B1) Grate Room",
                            regions::dungeons::skull::woods::SUBREGION,
                        ),
                        Some(|p| {
                            p.has_skull_keys(1)
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(SkullWoodsFoyer),
                    old_path(
                        SkullWoodsB2,
                        Some(|p| {
                            p.has_skull_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies() || p.break_floor_tiles())
                        }),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsB2,
            location(
                "Skull Woods B2",
                vec![],
                vec![
                    old_path(
                        SkullWoodsMain,
                        Some(|p| p.can_merge() && p.can_attack()),
                        Some(|p| p.can_merge() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        SkullWoodsElevatorHallway,
                        Some(|p| p.can_merge() && p.can_attack()),
                        Some(|p| p.can_merge() && p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
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
                vec![
                    edge!(SkullWoodsB2),
                    old_path(
                        SkullWoodsBossHallway,
                        Some(|p| p.has_skull_keys(3)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsBossHallway,
            location(
                "Skull Woods Boss Hallway",
                vec![],
                vec![
                    edge!(SkullWoodsElevatorHallway),
                    old_path(
                        SkullWoodsEastB1NorthFoyer,
                        Some(|p| p.has_fire_source() && p.can_attack()),
                        Some(|p| p.has_lamp()),
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        SkullWoodsBossRoom,
                        Some(|p| p.has_skull_big_key()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsBossRoom,
            location(
                "Skull Woods Boss Room",
                vec![old_check(
                    LocationInfo::new(
                        "[SW] Knucklemaster",
                        regions::dungeons::skull::woods::SUBREGION,
                    ),
                    Some(|p| p.can_defeat_knucklemaster()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    old_path(
                        SkullWoodsBossHallway,
                        Some(|p| p.can_defeat_knucklemaster()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        SkullWoodsSeresGrove,
                        Some(|p| p.can_defeat_knucklemaster()),
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
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
                vec![],
                vec![
                    edge!(SkullWoodsBossHallway),
                    old_path(
                        SkullWoodsEastB1North,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsEastB1North,
            location(
                "Skull Woods East B1 North",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[SW] (B1) Big Chest (Eyes)",
                            regions::dungeons::skull::woods::SUBREGION,
                        ),
                        Some(|p| p.has_skull_eyes()),
                        None,
                        None, // Eyeball dupe cannot be considered as it cannot be retried if missed
                        None,
                        None,
                    ),
                    goal!("Skull Eye Right", Goal::SkullEyeRight),
                ],
                vec![
                    old_path(
                        SkullWoodsEastB1NorthFoyer,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        SkullWoodsEastB1South,
                        Some(|p| p.has_skull_eye_right()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsEastB1South,
            location(
                "Skull Woods East B1 South",
                vec![],
                vec![
                    old_path(
                        SkullWoodsEastB1North,
                        Some(|p| p.can_merge() && p.has_skull_eye_right()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        SkullWoodsEastB1SouthFoyer,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            SkullWoodsEastB1SouthFoyer,
            location(
                "Skull Woods East B1 South Foyer",
                vec![],
                vec![
                    old_path(
                        SkullWoodsEastB1South,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    edge!(SkullWoodsOutdoor3),
                ],
            ),
        ),
        (
            SkullWoodsEastB1SouthLedges,
            location(
                "Skull Woods East B1 South Ledges",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[SW] (B1) Big Chest (Upper)",
                            regions::dungeons::skull::woods::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
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

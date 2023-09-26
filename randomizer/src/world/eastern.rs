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
            EasternPalaceFoyer,
            location(
                "Eastern Palace",
                vec![old_check(
                    LocationInfo::new(
                        "[EP] (1F) Merge Chest",
                        regions::dungeons::eastern::palace::SUBREGION,
                    ),
                    Some(|p| p.can_merge()),
                    None,
                    None,
                    None,
                    None,
                )],
                vec![
                    edge!(EasternRuinsUpper),
                    edge!(EasternPalace1F => {
                        normal: |p| p.can_hit_far_switch() || p.can_merge() || p.has_nice_ice_rod(),
                        hard: |p| p.has_master_sword(),
                    }),
                ],
            ),
        ),
        (
            EasternPalace1F,
            location(
                "Eastern Palace 1F",
                vec![
                    check!("[EP] (1F) Left Door Chest", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_hit_far_switch() || p.has_nice_ice_rod(),
                        hard: |_| true, // throw pot
                    }),
                    old_check(
                        LocationInfo::new(
                            "[EP] (1F) Popo Room",
                            regions::dungeons::eastern::palace::SUBREGION,
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[EP] (1F) Secret Room",
                            regions::dungeons::eastern::palace::SUBREGION,
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[EP] (1F) Switch Room",
                            regions::dungeons::eastern::palace::SUBREGION,
                        ),
                        Some(|p| p.can_hit_far_switch()),
                        Some(|p| p.has_ice_rod() || p.has_master_sword()), // Ice Rod + Pot
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    old_path(
                        EasternPalaceFoyer,
                        Some(|p| p.can_hit_switch() || p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        EasternPalaceMiniboss,
                        Some(|p| p.has_eastern_keys(1)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            EasternPalaceMiniboss,
            location(
                "Eastern Palace Miniboss",
                vec![],
                vec![
                    old_path(
                        EasternPalace1F,
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        EasternPalace2F,
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            EasternPalace2F,
            location(
                "Eastern Palace 2F",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[EP] (2F) Defeat Popos",
                            regions::dungeons::eastern::palace::SUBREGION,
                        ),
                        Some(|p| p.can_attack()),
                        Some(|p| p.has_lamp_or_net()),
                        None,
                        None,
                        None,
                    ),
                    check!("[EP] (2F) Ball Room", regions::dungeons::eastern::palace::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[EP] (2F) Switch Room",
                            regions::dungeons::eastern::palace::SUBREGION,
                        ),
                        Some(|p| p.can_hit_far_switch() || p.has_ice_rod()),
                        Some(|_| true), // pots
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[EP] (2F) Big Chest",
                            regions::dungeons::eastern::palace::SUBREGION,
                        ),
                        Some(|p| p.has_eastern_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
                vec![
                    edge!(EasternPalaceMiniboss),
                    old_path(
                        EasternPalaceBoss,
                        Some(|p| {
                            p.has_eastern_big_key()
                                && ((p.has_eastern_keys(2) && p.can_hit_far_switch())
                                    || p.has_ice_rod())
                                && p.can_attack()
                        }),
                        Some(|p| {
                            p.has_eastern_big_key()
                                && (p.has_bombs() || (p.has_eastern_keys(2) && p.has_lamp_or_net()))
                        }),
                        Some(|p| p.has_master_sword() || p.can_great_spin()),
                        Some(|p| p.has_tornado_rod()),
                        None,
                    ),
                ],
            ),
        ),
        (
            EasternPalaceBoss,
            location(
                "Eastern Palace 3F",
                vec![],
                vec![edge!(EasternPalacePostYuga => {
                    normal: |p| p.has_bow(),
                    hard: |p| {
                        p.has_bombs()
                            || p.has_master_sword()
                            || ((p.has_boomerang() || p.has_hookshot())
                                && (p.can_attack() || p.has_lamp_or_net()))
                            || p.has_nice_ice_rod()
                    },
                    hell: |p| p.has_ice_rod(), // gross
                })],
            ),
        ),
        (
            EasternPalacePostYuga,
            location(
                "Eastern Palace Post Yuga",
                vec![
                    check!("[EP] Yuga (1)", regions::dungeons::eastern::palace::SUBREGION),
                    check!("[EP] Yuga (2)", regions::dungeons::eastern::palace::SUBREGION),
                    check!("Eastern Palace Prize", regions::dungeons::eastern::palace::SUBREGION),
                    goal!("Eastern Palace Complete", Goal::Yuga),
                ],
                vec![
                    edge!(EasternPalace2F),
                    old_path(EasternPalaceEscape, Some(|p| p.can_merge()), None, None, None, None),
                ],
            ),
        ),
        (
            EasternPalaceEscape,
            location(
                "Eastern Palace Escape",
                vec![
                    check!("[EP] (3F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION),
                    check!("[EP] (1F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION),
                ],
                vec![
                    // do not include path back to 3F
                    edge!(EasternPalace1F),
                ],
            ),
        ),
    ])
}

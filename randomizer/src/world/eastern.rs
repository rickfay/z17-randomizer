use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, goal, location};
use crate::LocationInfo;

use std::collections::HashMap;

/// Eastern Palace World Graph
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            EasternPalaceFoyer,
            location(
                "Eastern Palace",
                vec![check!(
                    "[EP] (1F) Merge Chest",
                    regions::dungeons::eastern::palace::SUBREGION,
                    |p| p.can_merge()
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
                    check!("[EP] (1F) Popo Room", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_attack(),
                        hard: |p| p.has_lamp_or_net(),
                    }),
                    check!("[EP] (1F) Secret Room", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_attack(),
                        hard: |p| p.has_lamp_or_net(),
                    }),
                    check!("[EP] (1F) Switch Room", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_hit_far_switch(),
                        hard: |p| p.has_ice_rod() || p.has_master_sword(), // Ice Rod + Pot
                    }),
                ],
                vec![
                    edge!(EasternPalaceFoyer, |p| p.can_hit_switch() || p.can_merge()),
                    edge!(EasternPalaceMiniboss, |p| p.has_eastern_keys(1)),
                ],
            ),
        ),
        (
            EasternPalaceMiniboss,
            location(
                "Eastern Palace Miniboss",
                None,
                vec![
                    edge!(EasternPalace1F => {
                        normal: |p| p.can_attack(),
                        hard: |p| p.has_lamp_or_net(),
                    }),
                    edge!(EasternPalace2F => {
                        normal: |p| p.can_attack(),
                        hard: |p| p.has_lamp_or_net(),
                    }),
                ],
            ),
        ),
        (
            EasternPalace2F,
            location(
                "Eastern Palace 2F",
                vec![
                    check!("[EP] (2F) Defeat Popos", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_attack(),
                        hard: |p| p.has_lamp_or_net(),
                    }),
                    check!("[EP] (2F) Ball Room", regions::dungeons::eastern::palace::SUBREGION),
                    check!("[EP] (2F) Switch Room", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_hit_far_switch() || p.has_ice_rod(),
                        hard: |_| true, // pots
                    }),
                    check!("[EP] (2F) Big Chest", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.has_eastern_keys(2),
                        adv_glitched: |p| p.has_tornado_rod(),
                    }),
                ],
                vec![
                    edge!(EasternPalaceMiniboss),
                    edge!(
                        EasternPalaceBoss => {
                        normal: |p| p.has_eastern_big_key()
                            && (
                                (p.has_eastern_keys(2) && p.can_hit_far_switch())
                                || p.has_ice_rod()
                                || p.has_bombs()
                            )
                            && p.can_attack(),
                        hard: |p| p.has_eastern_big_key() && p.has_eastern_keys(2) && p.has_lamp_or_net(),
                        glitched: |p| p.has_master_sword() || p.can_great_spin(),
                        adv_glitched: |p| p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            EasternPalaceBoss,
            location(
                "Eastern Palace 3F",
                None,
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
                vec![edge!(EasternPalace2F), edge!(EasternPalaceEscape, |p| p.can_merge())],
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

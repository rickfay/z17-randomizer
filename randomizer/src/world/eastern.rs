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

/// Eastern Palace World Graph
pub(crate) fn graph(door_map: &DoorMap) -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            EasternPalaceFoyer,
            location(
                "Eastern Palace",
                None,
                vec![
                    door!(EasternPalaceExit, door_map),
                    edge!(EasternRuinsUpper),
                    edge!(EasternPalaceFoyerCrack, |p| p.can_merge()),
                    edge!(EasternPalace1F => {
                        normal: |p| p.can_hit_far_switch() || p.can_merge() || p.has_nice_ice_rod(),
                        hard: |p| p.has_master_sword(),
                    }),
                ],
            ),
        ),
        (
            EasternPalaceFoyerCrack,
            location(
                "Eastern Palace Foyer Crack",
                vec![check!("[EP] (1F) Merge Chest", regions::dungeons::eastern::palace::SUBREGION, |p| p
                    .has_eastern_compass())],
                vec![edge!(EasternPalaceFoyer, |p| p.can_merge())],
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
                    check!("[EP] (1F) Popo Room", regions::dungeons::eastern::palace::SUBREGION, |p| p.can_attack()),
                    check!("[EP] (1F) Secret Room", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_attack(),
                        hell: |p| p.has_tornado_rod(),
                    }),
                    check!("[EP] (1F) Switch Room", regions::dungeons::eastern::palace::SUBREGION => {
                        normal: |p| p.can_hit_far_switch(),
                        hard: |p| p.has_ice_rod() || p.has_master_sword(), // Ice Rod + Pot
                        hell: |p| p.has_tornado_rod(),
                    }),
                ],
                vec![
                    edge!(EasternPalaceFoyer, |p| p.can_hit_switch() || p.can_merge()),
                    edge!(EasternPalace1FOutOfBounds => {
                        hell: |p| p.has_tornado_rod(),
                    }),
                    edge!(EasternPalaceMiniboss, |p| p.has_eastern_keys(1)),
                ],
            ),
        ),
        (
            EasternPalace1FOutOfBounds,
            location(
                "Eastern Palace 1F Out of Bounds",
                None,
                vec![
                    edge!(EasternPalaceFoyerCrack => {
                        hell: |_| true,
                    }),
                    edge!(EasternPalaceEscape1F => {
                        hell: |_| true,
                    }),
                    edge!(EasternPalaceFinalChest => {
                        hell: |_| true,
                    }),
                ],
            ),
        ),
        (
            EasternPalaceMiniboss,
            location(
                "Eastern Palace Miniboss",
                None,
                vec![edge!(EasternPalace1F, |p| p.can_attack()), edge!(EasternPalace2F, |p| p.can_attack())],
            ),
        ),
        (
            EasternPalace2F,
            location(
                "Eastern Palace 2F",
                vec![
                    check!("[EP] (2F) Defeat Popos", regions::dungeons::eastern::palace::SUBREGION, |p| p.can_attack()),
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
                        normal: |p| p.has_eastern_big_key() && ((p.has_eastern_keys(2) && p.can_hit_far_switch()) || p.has_ice_rod() || p.has_bombs()),
                        hard: |p| p.has_eastern_big_key() && (p.has_eastern_keys(2) || p.has_ice_rod() || p.has_bombs()),
                        glitched: |p| p.has_eastern_big_key() && p.has_master_sword() || p.can_great_spin(),
                        adv_glitched: |p| p.has_tornado_rod(),
                    }),
                    edge!(EasternPalaceEscape2F => {
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
                            || p.has_fire_rod()
                            || ((p.has_boomerang() || p.has_hookshot() || p.has_foul_fruit()) && p.can_attack())
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
                    check!("[EP] Prize", regions::dungeons::eastern::palace::SUBREGION),
                    goal!("Eastern Palace Complete", Goal::Yuga),
                ],
                vec![edge!(EasternPalace2F), edge!(EasternPalaceEscape3F, |p| p.can_merge())],
            ),
        ),
        (
            EasternPalaceEscape3F,
            location(
                "Eastern Palace Escape 3F",
                vec![check!("[EP] (3F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION)],
                vec![edge!(EasternPalaceEscape2F, |p| p.can_merge())],
            ),
        ),
        (
            EasternPalaceEscape2F,
            location(
                "Eastern Palace Escape 2F",
                None,
                vec![
                    // do not include path back to 3F
                    edge!(EasternPalaceEscape1F => {
                        normal: |p| p.can_merge(),
                        hard: |p| p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            EasternPalaceEscape1F,
            location(
                "Eastern Palace Escape 1F",
                None,
                vec![
                    edge!(EasternPalaceEscape2F => {
                        normal: |p| p.can_merge(),
                        hard: |p| p.has_tornado_rod(),
                    }),
                    edge!(EasternPalaceFinalChest, |p| p.can_merge()),
                    edge!(EasternPalaceFoyer, |p| p.can_merge()),
                ],
            ),
        ),
        (
            EasternPalaceFinalChest,
            location(
                "Eastern Palace Final Chest",
                vec![check!("[EP] (1F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION, |p| p.can_merge())],
                vec![edge!(EasternPalaceEscape1F, |p| p.can_merge())],
            ),
        ),
    ])
}

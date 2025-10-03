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
            SwampPalaceOutside,
            location(
                "Swamp Palace Outside",
                vec![check!("Swamp Palace Weather Vane", regions::lorule::field::main::SUBREGION)],
                vec![
                    edge!(LoruleCastleArea, |p| p.has_hookshot() || p.has_flippers() || p.has_bomb_flower()),
                    edge!(SwampPalaceAntechamber),
                ],
            ),
        ),
        (
            SwampPalaceAntechamber,
            location(
                "Swamp Palace Antechamber",
                None,
                vec![
                    edge!(SwampPalaceOutside),
                    door!(SwampPalaceEntrance, door_map => {
                        normal: |p| p.has_bomb_flower() && p.hearts(6.0),
                        adv_glitched: |p| {
                            p.not_nice_mode()
                                && p.can_merge()
                                && p.has_ice_rod()
                                && p.has_flippers()
                                && (p.has_sword() || p.has_tornado_rod() || p.has_net() || p.has_bombs())
                                && p.hearts(6.0)
                        },
                    }),
                ],
            ),
        ),
        (
            SwampPalaceFoyer,
            location(
                "Swamp Palace Foyer",
                None,
                vec![
                    door!(SwampPalaceExit, door_map),
                    edge!(SwampPalaceMain, |p| p.has_flippers() && p.has_hookshot()),
                ],
            ),
        ),
        (
            SwampPalaceMain,
            location(
                "Swamp Palace",
                vec![
                    check!("[SP] (B1) Center", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Waterfall Room", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Raft Room (Pillar)", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Raft Room (Right)", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Raft Room (Left)", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Gyorm", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] (B1) Big Chest (Secret)", regions::dungeons::swamp::palace::SUBREGION => {
                        normal: |p| p.has_swamp_keys(2) && p.can_merge() && (p.progression_enemies() || p.break_floor_tiles()),
                        hard: |p| p.has_swamp_keys(2) && p.has_bow() && (p.progression_enemies() || p.break_floor_tiles()),
                        glitched: |p| p.has_swamp_keys(2) && p.has_boots(),
                        adv_glitched: |p| p.has_swamp_keys(2) && p.not_nice_mode() && p.has_ice_rod(),
                    }),
                    check!("[SP] (1F) West Room", regions::dungeons::swamp::palace::SUBREGION => {
                        normal: |p| p.has_swamp_keys(2) && p.can_merge() && (p.progression_enemies() || p.break_floor_tiles()),
                        adv_glitched: |p| p.not_nice_mode() && p.has_ice_rod(),
                    }),
                    check!("[SP] (1F) East Room", regions::dungeons::swamp::palace::SUBREGION => {
                        normal: |p| p.has_swamp_keys(2) && p.can_merge() && (p.progression_enemies() || p.break_floor_tiles()),
                        adv_glitched: |p| p.not_nice_mode() && p.has_ice_rod(),
                    }),
                    check!("[SP] (1F) Water Puzzle", regions::dungeons::swamp::palace::SUBREGION => {
                        normal: |p| p.has_swamp_keys(2) && p.can_merge() && (p.progression_enemies() || p.break_floor_tiles()),
                        adv_glitched: |p| p.not_nice_mode() && p.can_merge() && p.has_ice_rod(),
                    }),
                    check!("[SP] (1F) Big Chest (Fire)", regions::dungeons::swamp::palace::SUBREGION => {
                        normal: |p| p.can_merge() && (p.progression_enemies() || p.has_bombs() || p.has_hammer()) && (p.has_swamp_keys(4) || (p.has_swamp_keys(2) && (p.has_tornado_rod() || p.has_ice_rod()))),
                        hard: |p| p.can_merge() && (p.progression_enemies() || p.has_bombs() || p.has_hammer()) && p.has_swamp_keys(2),
                        glitched: |p| p.has_boots(),
                        adv_glitched: |p| p.not_nice_mode() && p.has_ice_rod(),
                    }),
                ],
                vec![edge!(SwampPalacePostBoss => {
                    normal: |p| {
                        p.can_merge()
                            && (p.progression_enemies() || p.has_bombs() || p.has_hammer())
                            && p.has_swamp_keys(4)
                            && p.has_swamp_big_key()
                            && p.can_defeat_arrghus()
                    },
                    adv_glitched: |p| p.not_nice_mode() && p.has_ice_rod() && (p.has_swamp_big_key() || p.has_tornado_rod()),
                })],
            ),
        ),
        (
            SwampPalacePostBoss,
            location(
                "Swamp Palace Post Boss",
                vec![
                    check!("[SP] Arrghus", regions::dungeons::swamp::palace::SUBREGION),
                    check!("[SP] Prize", regions::dungeons::swamp::palace::SUBREGION),
                    goal!("Arrghus", Goal::Arrghus),
                ],
                None,
            ),
        ),
    ])
}

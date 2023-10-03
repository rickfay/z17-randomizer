use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, goal, location, portal_std};
use crate::LocationInfo;

use std::collections::HashMap;

/// Lorule Castle World Graph
pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            LoruleCastle1F,
            location(
                "Lorule Castle 1F",
                None,
                vec![
                    edge!(LoruleCastleField),
                    edge!(LoruleCastleEastLedge1F, |p| p.can_merge()),
                    edge!(LoruleCastle2F3F => {
                        normal: |p| p.can_attack(),
                        hard: |_| true, // throw skulls
                    }),
                    edge!(LoruleCastleCenter1F => {
                        glitched: |p| p.has_boots(),
                    }),
                ],
            ),
        ),
        (
            LoruleCastleEastLedge1F,
            location(
                "Lorule Castle East Ledge 1F",
                vec![check!("[LC] (1F) Ledge", regions::dungeons::lorule::castle::SUBREGION)],
                vec![edge!(LoruleCastle1F, |p| p.can_merge())],
            ),
        ),
        (
            LoruleCastleCenter1F,
            location(
                "Lorule Castle 1F Center",
                vec![check!("[LC] (1F) Center", regions::dungeons::lorule::castle::SUBREGION)],
                vec![
                    edge!(LoruleCastle1F),
                    edge!(LoruleCastleEastLedge1F => {
                        glitched: |p| p.has_tornado_rod(),
                    }),
                ],
            ),
        ),
        (
            LoruleCastle2F3F,
            location(
                "Lorule Castle 2F 3F",
                vec![
                    check!("[LC] (2F) Near Torches", regions::dungeons::lorule::castle::SUBREGION),
                    check!("[LC] (2F) Hidden Path", regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.can_extinguish_torches(),
                        hard: |_| true,
                    }),
                    check!("[LC] (2F) Ledge",regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.can_merge(),
                        glitched: |p| p.has_boots(),
                        adv_glitched: |p| p.has_lorule_keys(3), // drop from 4F -> 3F -> 2F
                    }),
                    check!("[LC] (3F) Bomb Trial Center Chest", regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.has_bombs(),
                        glitched: |p| p.has_ice_rod(),
                    }),
                    check!("[LC] (3F) Big Bomb Flower Chest", regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.has_bombs() && p.can_merge(),
                        hard: |p| p.has_bombs() && p.has_bow(),
                    }),
                    check!(
                        "[LC] (3F) Merge Trial Free Chest",
                        regions::dungeons::lorule::castle::SUBREGION
                    ),
                    goal!("Bomb Trial", Goal::LcBombTrial, |p| p.has_lorule_keys(5)
                        && p.can_hit_switch()
                        && p.can_attack()),
                    check!("[LC] (3F) Spike Ball Chest", regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.can_merge(),
                        adv_glitched: |p| p.has_tornado_rod() && p.has_sword(),
                    }),
                    goal!("Ball Trial", Goal::LcBallTrial, |p| p.has_lorule_keys(5)
                        && (p.can_attack() || p.has_hookshot())),
                ],
                vec![
                    edge!(LoruleCastle1F),
                    edge!(LoruleCastleCenter1F),
                    edge!(LoruleCastle4F5F => {
                        normal: |p| p.has_lorule_keys(3),
                        adv_glitched: |p| p.has_nice_bombs() && p.has_tornado_rod() && (p.has_bow() || p.can_merge()), // secret path
                    }),
                    edge!(HildasStudy => {
                        normal: |p| p.has_completed_trials(),
                        adv_glitched: |p| p.has_sword() && p.has_nice_bombs() && (p.has_bow() || p.can_merge()),
                    }),
                ],
            ),
        ),
        // require 3 small keys
        (
            LoruleCastle4F5F,
            location(
                "Lorule Castle 4F 5F",
                vec![
                    check!("[LC] (4F) Lamp Trial Chest", regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.has_fire_source(),
                        hard: |_| true, // you don't need it...
                    }),
                    goal!("Lamp Trial", Goal::LcLampTrial => {
                        normal: |p| p.has_lorule_keys(5) && p.has_fire_source() && p.can_attack(),
                        hard: |p| p.has_lorule_keys(5) && p.can_attack(),
                    }),
                    check!(
                        "[LC] (4F) Eyeball Chest",
                        regions::dungeons::lorule::castle::SUBREGION,
                        |p| p.has_hookshot() && (p.has_ice_rod() || p.can_merge())
                    ),
                    check!(
                        "[LC] (4F) Lava Switch Chest",
                        regions::dungeons::lorule::castle::SUBREGION,
                        |p| p.has_hookshot()
                    ),
                    goal!("Hookshot Trial", Goal::LcHookTrial, |p| p.has_lorule_keys(5)
                        && p.has_hookshot()
                        && p.can_attack()),
                    check!("[LC] (4F) Center", regions::dungeons::lorule::castle::SUBREGION),
                    check!("[LC] (4F) Hidden Path", regions::dungeons::lorule::castle::SUBREGION => {
                        normal: |p| p.can_extinguish_torches(),
                        hard: |_| true,
                    }),
                ],
                vec![edge!(LoruleCastle2F3F)],
            ),
        ),
        (
            HildasStudy,
            location(
                "Hilda's Study",
                None,
                vec![edge!(LoruleCastle2F3F), portal_std(ZeldasStudy), edge!(ThroneRoom)],
            ),
        ),
        (
            ThroneRoom,
            location(
                "Throne Room",
                vec![check!("Zelda", regions::dungeons::lorule::castle::SUBREGION => {
                    normal: |p| p.has_yuganon_requirement() && (p.has_master_sword() || (p.swordless_mode() && p.has_net())),
                    hell: |p| p.has_yuganon_requirement() && p.has_sword(),
                })],
                vec![edge!(SacredRealm => {
                    normal: |p| {
                        p.has_yuganon_requirement()
                            && (p.has_master_sword() || (p.swordless_mode() && p.has_net()))
                            && p.can_merge()
                            && p.has_bow_of_light()
                    },
                    hell: |p| {
                        p.has_yuganon_requirement()
                            && (p.has_sword() || (p.swordless_mode() && p.has_net()))
                            && p.can_merge()
                            && p.has_bow_of_light()
                    },
                })],
            ),
        ),
        (SacredRealm, location("Sacred Realm", vec![goal!("Sacred Realm", Goal::Triforce)], None)),
    ])
}

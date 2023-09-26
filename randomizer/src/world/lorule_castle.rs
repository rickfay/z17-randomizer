use crate::legacy::path::Path;
use crate::model::check::Check;
use crate::model::filler_item::Goal;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::regions;
use crate::world::{check, edge, goal, location, old_check, old_path, portal_std};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            LoruleCastle1F,
            location(
                "Lorule Castle 1F",
                vec![],
                vec![
                    edge!(LoruleCastleField),
                    old_path(
                        LoruleCastleEastLedge1F,
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(
                        LoruleCastle2F3F,
                        Some(|p| p.can_attack()),
                        Some(|_| true), // throw skulls
                        None,
                        None,
                        None,
                    ),
                    old_path(LoruleCastleCenter1F, None, None, Some(|p| p.has_boots()), None, None),
                ],
            ),
        ),
        (
            LoruleCastleEastLedge1F,
            location(
                "Lorule Castle East Ledge 1F",
                vec![check!("[LC] (1F) Ledge", regions::dungeons::lorule::castle::SUBREGION)],
                vec![old_path(LoruleCastle1F, Some(|p| p.can_merge()), None, None, None, None)],
            ),
        ),
        (
            LoruleCastleCenter1F,
            location(
                "Lorule Castle 1F Center",
                vec![check!("[LC] (1F) Center", regions::dungeons::lorule::castle::SUBREGION)],
                vec![
                    edge!(LoruleCastle1F),
                    old_path(
                        LoruleCastleEastLedge1F,
                        None,
                        None,
                        Some(|p| p.has_tornado_rod()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            LoruleCastle2F3F,
            location(
                "Lorule Castle 2F 3F",
                vec![
                    check!("[LC] (2F) Near Torches", regions::dungeons::lorule::castle::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[LC] (2F) Hidden Path",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.can_extinguish_torches()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LC] (2F) Ledge",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_boots()),
                        Some(|p| p.has_lorule_keys(3)), // drop from 4F -> 3F -> 2F
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LC] (3F) Bomb Trial Center Chest",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.has_bombs()),
                        None,
                        Some(|p| p.has_ice_rod()),
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LC] (3F) Big Bomb Flower Chest",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.has_bombs() && p.can_merge()),
                        Some(|p| p.has_bombs() && p.has_bow()),
                        None,
                        None,
                        None,
                    ),
                    check!(
                        "[LC] (3F) Merge Trial Free Chest",
                        regions::dungeons::lorule::castle::SUBREGION
                    ),
                    goal!("Bomb Trial", Goal::LcBombTrial, |p| p.has_lorule_keys(5)
                        && p.can_hit_switch()
                        && p.can_attack()),
                    old_check(
                        LocationInfo::new(
                            "[LC] (3F) Spike Ball Chest",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_tornado_rod() && p.has_sword()),
                        None,
                    ),
                    goal!("Ball Trial", Goal::LcBallTrial, |p| p.has_lorule_keys(5)
                        && (p.can_attack() || p.has_hookshot())),
                ],
                vec![
                    edge!(LoruleCastle1F),
                    edge!(LoruleCastleCenter1F),
                    old_path(
                        LoruleCastle4F5F,
                        Some(|p| p.has_lorule_keys(3)),
                        None,
                        None,
                        Some(|p| {
                            p.has_nice_bombs()
                                && p.has_tornado_rod()
                                && (p.has_bow() || p.can_merge())
                        }), // secret path
                        None,
                    ),
                    old_path(
                        HildasStudy,
                        Some(|p| p.has_completed_trials()),
                        None,
                        None,
                        Some(|p| {
                            p.has_sword() && p.has_nice_bombs() && (p.has_bow() || p.can_merge())
                        }),
                        None,
                    ),
                ],
            ),
        ),
        // require 3 small keys
        (
            LoruleCastle4F5F,
            location(
                "Lorule Castle 4F 5F",
                vec![
                    old_check(
                        LocationInfo::new(
                            "[LC] (4F) Lamp Trial Chest",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.has_fire_source()),
                        Some(|_| true), // you don't need it...
                        None,
                        None,
                        None,
                    ),
                    goal!("Lamp Trial", Goal::LcLampTrial => {
                        normal: |p| p.has_lorule_keys(5) && p.has_fire_source() && p.can_attack(),
                        hard: |p| p.has_lorule_keys(5) && p.can_attack(),
                    }),
                    old_check(
                        LocationInfo::new(
                            "[LC] (4F) Eyeball Chest",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.has_hookshot() && (p.has_ice_rod() || p.can_merge())),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[LC] (4F) Lava Switch Chest",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    goal!("Hookshot Trial", Goal::LcHookTrial, |p| p.has_lorule_keys(5)
                        && p.has_hookshot()
                        && p.can_attack()),
                    check!("[LC] (4F) Center", regions::dungeons::lorule::castle::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[LC] (4F) Hidden Path",
                            regions::dungeons::lorule::castle::SUBREGION,
                        ),
                        Some(|p| p.can_extinguish_torches()),
                        Some(|_| true),
                        None,
                        None,
                        None,
                    ),
                ],
                vec![edge!(LoruleCastle2F3F)],
            ),
        ),
        (
            HildasStudy,
            location(
                "Hilda's Study",
                vec![],
                vec![edge!(LoruleCastle2F3F), portal_std(ZeldasStudy), edge!(ThroneRoom)],
            ),
        ),
        (
            ThroneRoom,
            location(
                "Throne Room",
                vec![old_check(
                    LocationInfo::new("Zelda", regions::dungeons::lorule::castle::SUBREGION),
                    Some(|p| {
                        p.has_yuganon_requirement()
                            && (p.has_sword() || (p.swordless_mode() && p.has_net()))
                    }),
                    Some(|p| p.has_yuganon_requirement() && p.has_net()),
                    None,
                    None,
                    None,
                )],
                vec![old_path(
                    SacredRealm,
                    Some(|p| {
                        p.has_yuganon_requirement()
                            && (p.has_sword() || (p.swordless_mode() && p.has_net()))
                            && p.can_merge()
                            && p.has_bow_of_light()
                    }),
                    Some(|p| {
                        p.has_yuganon_requirement()
                            && p.has_net()
                            && p.can_merge()
                            && p.has_bow_of_light()
                    }),
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            SacredRealm,
            location("Sacred Realm", vec![goal!("Sacred Realm", Goal::Triforce)], vec![]),
        ),
    ])
}

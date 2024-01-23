use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::regions;
use crate::world::{check, edge, goal, location};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            IceRuinsFoyer,
            location(
                "Ice Ruins Entrance",
                vec![],
                vec![edge!(LoruleDeathEastTop), edge!(IceRuins, |p| p.has_fire_rod())],
            ),
        ),
        // Require Fire Rod
        (
            IceRuins,
            location(
                "Ice Ruins",
                vec![
                    check!("[IR] (1F) Hidden Chest", regions::dungeons::ice::ruins::SUBREGION, |p| p.has_ice_compass()),
                    check!("[IR] (B4) Ice Pillar", regions::dungeons::ice::ruins::SUBREGION),
                    check!("[IR] (B3) Grate Chest (Left)", regions::dungeons::ice::ruins::SUBREGION),
                    check!("[IR] (B3) Grate Chest (Right)", regions::dungeons::ice::ruins::SUBREGION),
                    check!("[IR] (B5) Big Chest", regions::dungeons::ice::ruins::SUBREGION),
                    check!("[IR] (B1) Narrow Ledge", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.can_merge() && p.has_ice_keys(1),
                        adv_glitched: |p| p.can_merge() && p.has_boots() && p.has_tornado_rod(),
                    }),
                    check!("[IR] (B1) East Chest", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(1),
                        adv_glitched: |p| p.has_boots() && p.has_tornado_rod(),
                    }),
                    check!("[IR] (B1) Upper Chest", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(2),
                        adv_glitched: |p| p.has_boots() && p.has_tornado_rod(),
                    }),
                    check!("[IR] (B2) Long Merge Chest", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(2) && p.can_merge() && p.can_hit_switch() && p.has_stamina_scroll(),
                        adv_glitched: |p| p.has_boots(),
                    }),
                    check!("[IR] (B3) Big Chest (Puzzle)", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(2) && p.can_merge() && p.can_hit_switch(),
                        adv_glitched: |p| p.has_boots(),
                    }),
                    check!("[IR] (B4) Switches", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| {
                            p.has_ice_keys(2)
                                && p.can_merge()
                                && (p.progression_enemies()
                                    || p.has_bombs()
                                    || p.can_great_spin()
                                    || p.has_nice_ice_rod())
                        },
                        adv_glitched: |p| {
                            p.has_boots()
                                && (p.progression_enemies()
                                    || p.has_bombs()
                                    || p.can_great_spin()
                                    || p.has_nice_ice_rod())
                        },
                    }),
                    check!("[IR] (B4) Southwest Chest (Fall)", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(2) && p.can_merge(),
                        adv_glitched: |p| p.has_boots(),
                    }),
                    check!("[IR] (B4) Narrow Platform", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(2) && p.can_merge(),
                        adv_glitched: |p| p.has_boots(),
                    }),
                    check!("[IR] (B4) Southeast Chest (Fall)", regions::dungeons::ice::ruins::SUBREGION => {
                        normal: |p| p.has_ice_keys(3) || (p.has_ice_keys(2) && p.can_hit_switch()) && p.can_merge(),
                        adv_glitched: |p| p.has_boots(),
                    }),
                ],
                vec![
                    edge!(IceRuinsFoyer, |p| p.has_fire_rod()),
                    edge!(IceRuinsBoss => {
                        normal: |p| p.has_ice_keys(3) && p.has_ice_big_key() && p.can_merge(),
                        adv_glitched: |p| p.has_boots(),
                    }),
                ],
            ),
        ),
        (
            IceRuinsBoss,
            location("Ice Ruins Boss", vec![], vec![edge!(IceRuinsPostBoss, |p| p.can_defeat_dharkstare())]),
        ),
        (
            IceRuinsPostBoss,
            location(
                "Ice Ruins Post Boss",
                vec![
                    check!("[IR] Dharkstare", regions::dungeons::ice::ruins::SUBREGION),
                    check!("[IR] Prize", regions::dungeons::ice::ruins::SUBREGION),
                    goal!("Dharkstare", Goal::Dharkstare),
                ],
                vec![],
            ),
        ),
    ])
}

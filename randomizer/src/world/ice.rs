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
            IceRuinsFoyer,
            location(
                "Ice Ruins Entrance",
                vec![],
                vec![
                    edge!(LoruleDeathEastTop),
                    old_path(IceRuins, Some(|p| p.has_fire_rod()), None, None, None, None),
                ],
            ),
        ),
        // Require Fire Rod
        (
            IceRuins,
            location(
                "Ice Ruins",
                vec![
                    check!("[IR] (1F) Hidden Chest", regions::dungeons::ice::ruins::SUBREGION),
                    check!("[IR] (B4) Ice Pillar", regions::dungeons::ice::ruins::SUBREGION),
                    check!(
                        "[IR] (B3) Grate Chest (Left)",
                        regions::dungeons::ice::ruins::SUBREGION
                    ),
                    check!(
                        "[IR] (B3) Grate Chest (Right)",
                        regions::dungeons::ice::ruins::SUBREGION
                    ),
                    check!("[IR] (B5) Big Chest", regions::dungeons::ice::ruins::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B1) Narrow Ledge",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.can_merge() && p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B1) East Chest",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(1)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B1) Upper Chest",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(2)),
                        None,
                        None,
                        Some(|p| p.has_boots() && p.has_tornado_rod()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B2) Long Merge Chest",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.has_stamina_scroll()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B3) Big Chest (Puzzle)",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge() && p.can_hit_switch()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
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
                    old_check(
                        LocationInfo::new(
                            "[IR] (B4) Southwest Chest (Fall)",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B4) Narrow Platform",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                    old_check(
                        LocationInfo::new(
                            "[IR] (B4) Southeast Chest (Fall)",
                            regions::dungeons::ice::ruins::SUBREGION,
                        ),
                        Some(|p| p.has_ice_keys(2) && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                ],
                vec![
                    old_path(IceRuinsFoyer, Some(|p| p.has_fire_rod()), None, None, None, None),
                    old_path(
                        IceRuinsBoss,
                        Some(|p| p.has_ice_keys(3) && p.has_ice_big_key() && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.has_boots()),
                        None,
                    ),
                ],
            ),
        ),
        (
            IceRuinsBoss,
            location(
                "Ice Ruins Boss",
                vec![],
                vec![old_path(
                    IceRuinsPostBoss,
                    Some(|p| p.can_defeat_dharkstare()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            IceRuinsPostBoss,
            location(
                "Ice Ruins Post Boss",
                vec![
                    check!("[IR] Dharkstare", regions::dungeons::ice::ruins::SUBREGION),
                    check!("Ice Ruins Prize", regions::dungeons::ice::ruins::SUBREGION),
                    goal!("Dharkstare", Goal::Dharkstare),
                ],
                vec![],
            ),
        ),
    ])
}

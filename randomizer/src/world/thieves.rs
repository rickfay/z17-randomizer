use crate::filler::check::Check;
use crate::filler::filler_item::Goal;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::regions;
use crate::world::{check, edge, goal, location, old_check, old_path};
use crate::LocationInfo;

use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            ThievesHideoutB1,
            location(
                "Thieves' Hideout",
                vec![
                    /* B1 */
                    check!("[TT] (B1) Grate Chest", regions::dungeons::thieves::hideout::SUBREGION),
                    old_check(
                        LocationInfo::new("[TT] (B1) Jail Cell", regions::dungeons::thieves::hideout::SUBREGION),
                        Some(|p| p.can_merge()),
                        None,
                        Some(|p| p.has_boots()), // jailbreak
                        None,
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod() && p.can_escape_dungeon()),
                    ),
                    goal!("Thieves' Hideout B1 Door Open", Goal::ThievesB1DoorOpen => {
                        normal: |p| p.can_merge() && p.can_hit_switch(),
                        glitched: |p| p.has_boots() && (p.has_boomerang() || p.has_ice_rod()),
                        hell: |p| p.has_boots() && p.has_bombs(),
                    }),
                    /* B2 */
                    old_check(
                        LocationInfo::new(
                            "[TT] (B2) Grate Chest (Fall)",
                            regions::dungeons::thieves::hideout::SUBREGION,
                        ),
                        Some(|p| p.thieves_b1_door_open()),
                        None,
                        None,
                        Some(|p| p.adv_thieves_statue_clip()),
                        Some(|p| p.hell_thieves_statue_clip()),
                    ),
                    goal!("Thieves' Hideout B2 Door Open", Goal::ThievesB2DoorOpen => {
                        normal: |p| p.thieves_b1_door_open() && p.can_merge() && (p.progression_enemies() || p.has_bombs()),
                        adv_glitched: |p| (p.can_merge() || p.can_escape_dungeon()) && p.adv_thieves_statue_clip(),
                        hell: |p| p.has_bombs(),
                    }),
                    old_check(
                        LocationInfo::new("[TT] (B2) Jail Cell", regions::dungeons::thieves::hideout::SUBREGION),
                        Some(|p| p.thieves_b1b2_doors_open() && p.can_merge()),
                        None,
                        None,
                        Some(|p| p.can_merge() && p.can_hit_switch()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()), // reach from B3 Out of Bounds
                    ),
                    old_check(
                        LocationInfo::new(
                            "[TT] (B2) Switch Puzzle Room",
                            regions::dungeons::thieves::hideout::SUBREGION,
                        ),
                        Some(|p| p.thieves_b1b2_doors_open()),
                        None,
                        None,
                        Some(|p| p.adv_thieves_statue_clip()),
                        Some(|p| p.hell_thieves_statue_clip()),
                    ),
                    old_check(
                        LocationInfo::new("[TT] (B2) Eyegores", regions::dungeons::thieves::hideout::SUBREGION),
                        Some(|p| {
                            p.thieves_b1b2_doors_open()
                                && p.can_merge()
                                && (p.progression_enemies() || p.has_bombs())
                                && p.can_hit_shielded_switch()
                                && (p.has_sword() || p.has_bow()) // Fight is too hard for "any attacking item" to be in Normal Logic. Limit to Sword or Bow (which deals triple damage)
                        }),
                        Some(|p| {
                            p.thieves_b1b2_doors_open()
                                && p.can_merge()
                                && (p.progression_enemies() || p.has_bombs())
                                && p.can_hit_shielded_switch()
                                && (p.can_attack() || p.has_lamp_or_net())
                        }),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && (p.has_boots() || p.has_tornado_rod())),
                        None,
                    ),
                    /* Escape */
                    goal!("Thieves' Hideout B3 Water Drained", Goal::ThievesB3WaterDrained => {
                        normal: |p| {
                            p.thieves_b1b2_doors_open()
                                && p.has_thieves_key()
                                && p.can_merge()
                                && p.has_flippers()
                                && p.can_attack()
                        },
                        hard: |p| {
                            p.thieves_b1b2_doors_open()
                                && p.has_thieves_key()
                                && p.can_merge()
                                && p.has_flippers()
                                && p.has_lamp_or_net()
                        },
                        adv_glitched: |p| p.adv_thieves_statue_clip() && p.has_tornado_rod(),
                        hell: |p| p.hell_thieves_statue_clip() && p.has_tornado_rod(),
                    }),
                    old_check(
                        LocationInfo::new("[TT] (B3) Underwater", regions::dungeons::thieves::hideout::SUBREGION),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    old_check(
                        LocationInfo::new(
                            "[TT] (B3) Big Chest (Hidden)",
                            regions::dungeons::thieves::hideout::SUBREGION,
                        ),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        Some(|p| p.adv_thieves_statue_clip() && p.has_tornado_rod()),
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                    old_check(
                        LocationInfo::new("[TT] (B1) Behind Wall", regions::dungeons::thieves::hideout::SUBREGION),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        None, // I'm just not including this
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod() && p.can_escape_dungeon()),
                    ),
                    old_check(
                        LocationInfo::new(
                            "[TT] (B1) Big Chest (Entrance)",
                            regions::dungeons::thieves::hideout::SUBREGION,
                        ),
                        Some(|p| p.thieves_escape_equipment() && p.can_attack()),
                        Some(|p| p.thieves_escape_equipment() && p.has_lamp_or_net()),
                        None,
                        None, // I'm just not including this
                        Some(|p| p.hell_thieves_statue_clip() && p.has_tornado_rod()),
                    ),
                ],
                vec![
                    edge!(LoruleCastleArea),
                    old_path(
                        ThievesBoss,
                        Some(|p| {
                            p.has_thieves_big_key()
                                && p.has_thieves_key()
                                && p.thieves_escape_equipment()
                                && p.can_merge()
                                && p.can_attack()
                        }),
                        Some(|p| {
                            p.has_thieves_big_key()
                                && p.has_thieves_key()
                                && p.thieves_escape_equipment()
                                && p.can_merge()
                                && p.has_lamp_or_net()
                        }),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            ThievesBoss,
            location(
                "Thieves' Hideout Boss",
                vec![],
                vec![old_path(
                    ThievesPostBoss,
                    Some(|p| p.can_merge() || p.can_attack()),
                    Some(|p| p.can_merge() || p.has_lamp_or_net()),
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            ThievesPostBoss,
            location(
                "Thieves' Hideout Post Boss",
                vec![
                    check!("[TT] Stalblind", regions::dungeons::thieves::hideout::SUBREGION),
                    check!("[TT] Prize", regions::dungeons::thieves::hideout::SUBREGION),
                    goal!("Stalblind Defeated", Goal::Stalblind),
                ],
                vec![],
            ),
        ),
    ])
}

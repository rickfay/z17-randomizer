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
            DarkPalaceFoyer,
            location(
                "Dark Palace",
                vec![],
                vec![
                    edge!(DarkRuins),
                    old_path(
                        DarkPalaceSecondRoom,
                        Some(|p| p.has_bombs() && (p.has_lamp() || p.lampless())),
                        None, // not considering Fire Rod + Nice Ice Rod combo yet
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceSecondRoom,
            location(
                "Dark Palace Second Room",
                vec![
                    check!("[PD] (1F) Right Pit", regions::dungeons::dark::palace::SUBREGION),
                    old_check(
                        LocationInfo::new(
                            "[PD] (1F) Left Pit",
                            regions::dungeons::dark::palace::SUBREGION,
                        ),
                        Some(|p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(DarkPalaceFoyer),
                    old_path(DarkPalaceMain, Some(|p| p.has_dark_keys(1)), None, None, None, None),
                ],
            ),
        ),
        (
            DarkPalaceMain,
            location(
                "Dark Palace",
                vec![
                    check!("[PD] (1F) Switch Puzzle", regions::dungeons::dark::palace::SUBREGION),
                    check!(
                        "[PD] (1F) Hidden Room (Upper)",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                    check!(
                        "[PD] (1F) Hidden Room (Lower)",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                    check!("[PD] (B1) Fall From 1F", regions::dungeons::dark::palace::SUBREGION),
                    check!("[PD] (B1) Helmasaur Room", regions::dungeons::dark::palace::SUBREGION),
                    check!(
                        "[PD] (B1) Helmasaur Room (Fall)",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                    old_check(
                        LocationInfo::new(
                            "[PD] (B1) Maze",
                            regions::dungeons::dark::palace::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
                vec![
                    edge!(DarkPalaceSecondRoom),
                    old_path(
                        DarkPalaceLockedDoors,
                        Some(|p| p.has_dark_keys(4)),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceLockedDoors,
            location(
                "Dark Palace Locked Doors",
                vec![
                    check!("[PD] (1F) Fall From 2F", regions::dungeons::dark::palace::SUBREGION),
                    check!(
                        "[PD] (2F) Big Chest (Hidden)",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                    check!(
                        "[PD] (2F) South Hidden Room",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                    old_check(
                        LocationInfo::new(
                            "[PD] (2F) Alcove",
                            regions::dungeons::dark::palace::SUBREGION,
                        ),
                        Some(|p| p.can_merge()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    check!(
                        "[PD] (B1) Big Chest (Switches)",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                ],
                vec![
                    edge!(DarkPalaceMain),
                    old_path(
                        DarkPalaceBoss,
                        Some(|p| p.has_dark_big_key() && p.can_merge()),
                        Some(|p| p.has_dark_big_key() && p.has_ice_rod()),
                        Some(|p| p.has_dark_big_key() && p.has_nice_bombs()),
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            DarkPalaceBoss,
            location(
                "Dark Palace Boss",
                vec![],
                vec![old_path(
                    DarkPalaceAfterBoss,
                    Some(|p| p.can_defeat_gemesaur()),
                    None,
                    None,
                    None,
                    None,
                )],
            ),
        ),
        (
            DarkPalaceAfterBoss,
            location(
                "Dark Palace After Boss",
                vec![
                    check!("[PD] Gemesaur King", regions::dungeons::dark::palace::SUBREGION),
                    check!("Dark Palace Prize", regions::dungeons::dark::palace::SUBREGION),
                    goal!("Gemesaur King", Goal::GemesaurKing),
                ],
                vec![],
            ),
        ),
    ])
}

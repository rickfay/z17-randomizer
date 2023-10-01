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

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            DarkPalaceFoyer,
            location(
                "Dark Palace",
                vec![check!(
                    "[PD] (1F) Right Pit",
                    regions::dungeons::dark::palace::SUBREGION,
                    |p| p.has_bombs()
                )],
                vec![
                    edge!(DarkRuins),
                    edge!(DarkPalaceSecondRoom, |p| (p.has_bombs()
                        || (p.has_nice_ice_rod() && p.has_fire_rod()))
                        && (p.has_lamp() || p.lampless())),
                ],
            ),
        ),
        (
            DarkPalaceSecondRoom,
            location(
                "Dark Palace Second Room",
                vec![check!(
                    "[PD] (1F) Left Pit",
                    regions::dungeons::dark::palace::SUBREGION,
                    |p| p.can_merge() || p.has_boomerang() || p.has_hookshot()
                )],
                vec![edge!(DarkPalaceFoyer), edge!(DarkPalaceMain, |p| p.has_dark_keys(1))],
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
                    check!("[PD] (B1) Maze", regions::dungeons::dark::palace::SUBREGION, |p| p
                        .can_merge()),
                ],
                vec![
                    edge!(DarkPalaceSecondRoom),
                    edge!(DarkPalaceLockedDoors, |p| p.has_dark_keys(4)),
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
                    check!("[PD] (2F) Alcove", regions::dungeons::dark::palace::SUBREGION, |p| p
                        .can_merge()),
                    check!(
                        "[PD] (B1) Big Chest (Switches)",
                        regions::dungeons::dark::palace::SUBREGION
                    ),
                ],
                vec![
                    edge!(DarkPalaceMain),
                    edge!(DarkPalaceBoss => {
                        normal: |p| p.has_dark_big_key() && p.can_merge(),
                        hard: |p| p.has_dark_big_key() && p.has_ice_rod(),
                        glitched: |p| p.has_dark_big_key() && p.has_nice_bombs(),
                    }),
                ],
            ),
        ),
        (
            DarkPalaceBoss,
            location(
                "Dark Palace Boss",
                None,
                vec![edge!(DarkPalaceAfterBoss, |p| p.can_defeat_gemesaur())],
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
                None,
            ),
        ),
    ])
}

use crate::legacy::path::Path;
use crate::model::location::Location::{self, *};
use crate::model::location_node::LocationNode;
use crate::model::logic::Logic;
use crate::world::{edge, location, old_path, portal};
use std::collections::HashMap;

pub(crate) fn graph() -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            HyruleCastleDungeon,
            location(
                "Inside Hyrule Castle",
                vec![],
                vec![
                    edge!(HyruleCastleRoof),
                    old_path(
                        HyruleCastleDungeonBoss,
                        Some(|p| (p.can_merge() && p.can_attack()) || p.has_ice_rod()), // add Nice TRod, when nice items figured out
                        Some(|p| p.has_bow() || p.has_nice_bombs()),
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
        (
            HyruleCastleDungeonBoss,
            location(
                "Hyrule Castle Dungeon Boss",
                vec![],
                vec![
                    old_path(
                        HyruleCastleDungeon,
                        Some(|p| p.can_defeat_yuga2()),
                        None,
                        None,
                        None,
                        None,
                    ),
                    old_path(ZeldasStudy, Some(|p| p.can_defeat_yuga2()), None, None, None, None),
                ],
            ),
        ),
        (
            ZeldasStudy,
            location(
                "Zelda's Study",
                vec![],
                vec![
                    //path!(HyruleCastleDungeonBoss), // Don't allow reverse Hyrule Castle
                    portal(
                        HildasStudy,
                        Some(|p| p.can_merge() && p.can_destroy_curtain()),
                        None,
                        None,
                        None,
                        None,
                    ),
                ],
            ),
        ),
    ])
}

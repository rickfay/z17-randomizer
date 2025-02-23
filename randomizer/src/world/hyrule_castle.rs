use crate::filler::cracks::Crack::HyruleCastle;
use crate::filler::location::Location::{self, *};
use crate::filler::location_node::LocationNode;
use crate::filler::logic::Logic;
use crate::filler::path::Path;
use crate::world::{crack_left, crack_right, edge, location};
use crate::CrackMap;
use std::collections::HashMap;

pub(crate) fn graph(crack_map: &CrackMap) -> HashMap<Location, LocationNode> {
    HashMap::from([
        (
            HyruleCastleDungeon,
            location(
                "Inside Hyrule Castle",
                None,
                vec![
                    edge!(HyruleCastleRoof),
                    edge!(HyruleCastleDungeonBoss => {
                        normal: |p| (p.can_merge() && p.can_attack()) || p.has_ice_rod(), // add Nice TRod, when nice items figured out
                        hard: |p| p.has_bow() || p.has_nice_bombs(),
                    }),
                ],
            ),
        ),
        (
            HyruleCastleDungeonBoss,
            location(
                "Hyrule Castle Dungeon Boss",
                None,
                vec![
                    edge!(HyruleCastleDungeon, |p| p.can_defeat_yuga2()),
                    edge!(ZeldasStudy, |p| p.can_defeat_yuga2()),
                ],
            ),
        ),
        (
            ZeldasStudy,
            location(
                "Zelda's Study",
                //vec![check!("[HC] Crack", regions::dungeons::hyrule::castle::SUBREGION, |p| p.can_merge())],
                None,
                vec![
                    //path!(HyruleCastleDungeonBoss), // Don't allow reverse Hyrule Castle
                    crack_left(HyruleCastle, crack_map, true),
                    crack_right(HyruleCastle, crack_map, true),
                ],
            ),
        ),
    ])
}

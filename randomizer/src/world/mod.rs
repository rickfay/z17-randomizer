use crate::regions::Subregion;
use crate::{
    filler::filler_item::Randomizable,
    filler::{check::Check, location::Location, location_node::LocationNode, logic::Logic, progress::Progress},
    hints::hint_ghost_name,
    CrackMap, DashMap, LocationInfo,
};
use game::ghosts::HintGhost;
use log::info;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

mod dark;
mod desert;
mod eastern;
mod gales;
mod hera;
mod hyrule;
mod hyrule_castle;
mod ice;
mod lorule;
mod lorule_castle;
mod skull;
mod swamp;
mod thieves;
mod turtle;

#[derive(Default, Debug, Serialize)]
pub struct WorldGraph {
    graph: DashMap<Location, LocationNode>,
}

impl WorldGraph {
    fn new() -> Self {
        Self { graph: Default::default() }
    }
}

impl Deref for WorldGraph {
    type Target = DashMap<Location, LocationNode>;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl DerefMut for WorldGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

/// Build the World Graph
/// FIXME shouldn't take crack_map as argument, map should be independent of that randomization
pub fn build_world_graph(crack_map: &CrackMap) -> WorldGraph {
    info!("Building World Graph...");
    let mut world = WorldGraph::new();

    world.extend(hyrule::graph(crack_map));
    world.extend(lorule::graph(crack_map));

    world.extend(eastern::graph());
    world.extend(gales::graph());
    world.extend(hera::graph());

    world.extend(hyrule_castle::graph(crack_map));

    world.extend(dark::graph());
    world.extend(swamp::graph());
    world.extend(skull::graph());
    world.extend(thieves::graph());
    world.extend(ice::graph());
    world.extend(desert::graph(crack_map));
    world.extend(turtle::graph());

    world.extend(lorule_castle::graph(crack_map));

    world
}

/// Check convenience macros
macro_rules! check {
    ($loc_name:expr, $loc_region:expr) => {
        Check::new($loc_name, Logic::free(), None, Some(LocationInfo::new($loc_name, $loc_region)))
    };
    ($loc_name:expr, $loc_region:expr, $normal:expr) => {
        Check::new($loc_name, *Logic::new()
            .normal($normal), None, Some(LocationInfo::new($loc_name, $loc_region)))
    };
    ($loc_name:expr, $loc_region:expr => {
        $(normal: $normal:expr,)?
        $(hard: $hard:expr,)?
        $(glitched: $glitched:expr,)?
        $(adv_glitched: $adv_glitched:expr,)?
        $(hell: $hell:expr,)?
    }) => (
        Check::new($loc_name, *Logic::new()
            $(.normal($normal))?
            $(.hard($hard))?
            $(.glitched($glitched))?
            $(.adv_glitched($adv_glitched))?
            $(.hell($hell))?, None, Some(LocationInfo::new($loc_name, $loc_region)))
    );
}

/// Weather Vane convenience macros
// macro_rules! vane {
//     ($loc_name:expr, $vane:expr) => {
//         Check::new($loc_name, Logic::free(), Some($vane.into()), None)
//     };
//     ($loc_name:expr, $vane:expr, $normal:expr) => {
//         Check::new($loc_name, *Logic::new()
//             .normal($normal), Some($vane.into()), None)
//     };
//     ($loc_name:expr, $vane:expr => {
//         $(normal: $normal:expr,)?
//         $(hard: $hard:expr,)?
//         $(glitched: $glitched:expr,)?
//         $(adv_glitched: $adv_glitched:expr,)?
//         $(hell: $hell:expr,)?
//     }) => (
//         Check::new($loc_name, *Logic::new()
//             $(.normal($normal))?
//             $(.hard($hard))?
//             $(.glitched($glitched))?
//             $(.adv_glitched($adv_glitched))?
//             $(.hell($hell))?, Some($vane.into()), None)
//     );
// }

/// Goal convenience macros
macro_rules! goal {
    ($loc_name:expr, $goal:expr) => {
        Check::new($loc_name, Logic::free(), Some($goal.into()), None)
    };
    ($loc_name:expr, $goal:expr, $normal:expr) => {
        Check::new($loc_name, *Logic::new()
            .normal($normal), Some($goal.into()), None)
    };
    ($loc_name:expr, $goal:expr => {
        $(normal: $normal:expr,)?
        $(hard: $hard:expr,)?
        $(glitched: $glitched:expr,)?
        $(adv_glitched: $adv_glitched:expr,)?
        $(hell: $hell:expr,)?
    }) => (
        Check::new($loc_name, *Logic::new()
            $(.normal($normal))?
            $(.hard($hard))?
            $(.glitched($glitched))?
            $(.adv_glitched($adv_glitched))?
            $(.hell($hell))?, Some($goal.into()), None)
    );
}

/// Path convenience macros
/// Rust won't let me call it a Path so I'm using the dumb math name >:(
macro_rules! edge {
    ($dest:ident) => (
        Path::new(Location::$dest, Logic::free())
    );
    ($dest:ident, $normal:expr) => (
        Path::new(Location::$dest, *Logic::new()
            .normal($normal))
    );
    ($dest:ident => {
        $(normal: $normal:expr,)?
        $(hard: $hard:expr,)?
        $(glitched: $glitched:expr,)?
        $(adv_glitched: $adv_glitched:expr,)?
        $(hell: $hell:expr,)?
    }) => (
        Path::new(Location::$dest,
            *Logic::new()
            $(.normal($normal))?
            $(.hard($hard))?
            $(.glitched($glitched))?
            $(.adv_glitched($adv_glitched))?
            $(.hell($hell))?
        )
    );
}

use crate::filler::cracks::Crack;
use crate::filler::path::Path;
pub(crate) use check;
pub(crate) use edge;
pub(crate) use goal;

fn location<C, P>(name: &'static str, checks: C, paths: P) -> LocationNode
where
    C: Into<Option<Vec<Check>>>,
    P: Into<Option<Vec<Path>>>,
{
    LocationNode::new(name, checks.into(), paths.into())
}

// TODO REMOVE
// #[deprecated]
fn old_check(
    location_info: LocationInfo, normal: Option<fn(&Progress) -> bool>, hard: Option<fn(&Progress) -> bool>,
    glitched: Option<fn(&Progress) -> bool>, adv_glitched: Option<fn(&Progress) -> bool>,
    hell: Option<fn(&Progress) -> bool>,
) -> Check {
    Check::new(location_info.name, Logic::config(normal, hard, glitched, adv_glitched, hell), None, Some(location_info))
}

// todo REMOVE
// #[deprecated]
fn old_path(
    default: Location, normal: Option<fn(&Progress) -> bool>, hard: Option<fn(&Progress) -> bool>,
    glitched: Option<fn(&Progress) -> bool>, adv_glitched: Option<fn(&Progress) -> bool>,
    hell: Option<fn(&Progress) -> bool>,
) -> Path {
    Path::new(default, Logic::config(normal, hard, glitched, adv_glitched, hell))
}

/// Used for checks that the Randomizer should be aware of existing, but are not considered part of any logic.
/// Most things that use this are typically not in logic *yet*
fn out_of_logic(name: &'static str, subregion: &'static Subregion) -> Check {
    Check::new(
        name,
        Logic { normal: None, hard: None, glitched: None, adv_glitched: None, hell: None },
        None,
        Some(LocationInfo::new(name, subregion)),
    )
}

fn crack_left(crack: Crack, crack_map: &CrackMap, is_hc: bool) -> Path {
    let dest_crack = crack_map.get(&crack).expect(&format!("CrackMap missing Crack: {:?}", crack));
    let (_left, right) = dest_crack.get_left_right_locations();

    let logic: fn(&Progress) -> bool = if is_hc { |p| p.can_merge() } else { |p| p.are_cracks_open() && p.can_merge() };

    Path::new(right, *Logic::new().normal(logic))
}

fn crack_right(crack: Crack, crack_map: &CrackMap, is_hc: bool) -> Path {
    let dest_crack = crack_map.get(&crack).expect("CrackMap should have all Cracks mapped");
    let (left, _right) = dest_crack.get_left_right_locations();

    let logic: fn(&Progress) -> bool = if is_hc { |p| p.can_merge() } else { |p| p.are_cracks_open() && p.can_merge() };

    Path::new(left, *Logic::new().normal(logic))
}

fn fast_travel_hyrule() -> Path {
    edge!(HyruleBellTravel, |p| p.has_bell() && p.are_hyrule_vanes_active())
}

fn fast_travel_lorule() -> Path {
    edge!(LoruleBellTravel, |p| p.has_bell() && p.are_lorule_vanes_active())
}

/// Hint Ghost checks
fn ghost(ghost: HintGhost) -> Check {
    Check::new(hint_ghost_name(&ghost), Logic::free(), Some(Randomizable::HintGhost(ghost)), None)
}

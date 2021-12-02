use crate::{regions::Subregion, state::State, Condition, Location, Settings};

pub trait Graph {
    fn settings(&self) -> &Settings;
    fn check(&self, predicate: fn(&State) -> bool) -> bool;
    fn add(&mut self, node: Node);
    fn add_edge(&mut self, edge: Condition, node: Node);
}

#[derive(Clone, Debug)]
pub enum Node {
    Location(Location),
    Path(&'static Subregion),
}

impl From<Location> for Node {
    fn from(location: Location) -> Self {
        Self::Location(location)
    }
}

impl From<&'static Subregion> for Node {
    fn from(subregion: &'static Subregion) -> Self {
        Self::Path(subregion)
    }
}

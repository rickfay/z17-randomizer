use crate::{regions::Area, LocationKey};

pub trait Graph {}

#[derive(Clone, Debug)]
pub enum Node {
    Location(LocationKey),
    Path(Area),
}

impl From<LocationKey> for Node {
    fn from(location: LocationKey) -> Self {
        Self::Location(location)
    }
}

impl From<Area> for Node {
    fn from(node: Area) -> Self {
        Self::Path(node)
    }
}

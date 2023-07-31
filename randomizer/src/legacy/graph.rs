use crate::{regions::Area, LocationInfo};

pub trait Graph {}

#[derive(Clone, Debug)]
pub enum Node {
    Location(LocationInfo),
    Path(Area),
}

impl From<LocationInfo> for Node {
    fn from(location: LocationInfo) -> Self {
        Self::Location(location)
    }
}

impl From<Area> for Node {
    fn from(node: Area) -> Self {
        Self::Path(node)
    }
}

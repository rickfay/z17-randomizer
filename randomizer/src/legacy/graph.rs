use crate::{regions::Subregion, LocationInfo};

pub trait Graph {}

#[derive(Clone, Debug)]
pub enum Node {
    Location(LocationInfo),
    Path(&'static Subregion),
}

impl From<LocationInfo> for Node {
    fn from(location: LocationInfo) -> Self {
        Self::Location(location)
    }
}

impl From<&'static Subregion> for Node {
    fn from(subregion: &'static Subregion) -> Self {
        Self::Path(subregion)
    }
}

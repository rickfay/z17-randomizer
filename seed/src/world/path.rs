use {
    crate::{
        filler::{logic::Logic, progress::Progress},
        world::{logic::Logic, LocationId},
    },
    seed::filler::{location::LocationId, logic::Logic, progress::Progress},
};

/// A Path connecting two [`Location`]s.
///
/// [`Location`]: crate::world::location::Location
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Path {
    destination: LocationId,
    logic: Logic,
}

impl Path {
    pub fn new(destination: LocationId, logic: Logic) -> Self {
        Self { destination, logic }
    }

    pub fn get_destination(self) -> LocationId {
        self.destination
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        self.logic.can_access(progress)
    }
}

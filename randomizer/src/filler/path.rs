use crate::filler::{location::Location, logic::Logic, progress::Progress};
use serde::Serialize;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct Path {
    destination: Location,
    logic: Logic,
}

impl Path {
    pub fn new(destination: Location, logic: Logic) -> Self {
        Self { destination, logic }
    }

    pub fn get_destination(self) -> Location {
        self.destination
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        self.logic.can_access(progress)
    }
}

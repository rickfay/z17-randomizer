use crate::location::Location;
use crate::progress::Progress;

#[derive(Copy, Clone)]
pub struct Path {
    destination: Location,
    logic: Option<fn(&Progress) -> bool>,
}

impl Path {
    pub fn new(destination: Location, logic: Option<fn(&Progress) -> bool>) -> Self {
        Self { destination, logic }
    }

    pub fn get_destination(self) -> Location {
        self.destination
    }

    pub fn can_travel(self, progress: &Progress) -> bool {
        match self.logic {
            None => true,
            Some(_) => (self.logic.unwrap())(progress)
        }
    }
}
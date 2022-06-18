use crate::{FillerItem, LocationInfo};
use crate::progress::Progress;

#[derive(Copy, Clone)]
pub struct Check {
    name: &'static str,
    logic: Option<fn(&Progress) -> bool>,
    quest: Option<FillerItem>,
    location_info: Option<LocationInfo>,
}

impl Check {
    //if location_info.is_some() { location_info.unwrap().name } else { "" }
    pub fn new(name: &'static str, logic: Option<fn(&Progress) -> bool>, quest: Option<FillerItem>, location_info: Option<LocationInfo>,) -> Self {
        Self { name, logic, quest, location_info }
    }

    pub fn get_name(self) -> &'static str {
        self.name
    }

    pub fn get_logic(self) -> Option<fn(&Progress) -> bool> {
        self.logic
    }

    pub fn get_quest(self) -> Option<FillerItem> {
        self.quest
    }

    pub fn get_location_info(self) -> Option<LocationInfo> {
        self.location_info
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        match self.logic {
            None => true,
            Some(_) => (self.logic.unwrap())(progress)
        }
    }
}
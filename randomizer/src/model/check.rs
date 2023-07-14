use {
    crate::{
        model::{logic::Logic, progress::Progress},
        FillerItem, LocationInfo,
    },
    std::hash::{Hash, Hasher},
};

/// A specific location containing a randomized item
#[derive(Copy, Clone, Debug)]
pub struct Check {
    name: &'static str,
    logic: Logic,
    quest: Option<FillerItem>,
    location_info: Option<LocationInfo>,
}

impl Check {
    pub fn new(
        name: &'static str, logic: Logic, quest: Option<FillerItem>,
        location_info: Option<LocationInfo>,
    ) -> Self {
        Self { name, logic, quest, location_info }
    }

    pub fn get_name(self) -> &'static str {
        self.name
    }

    pub fn get_quest(self) -> Option<FillerItem> {
        self.quest
    }

    pub fn get_location_info(self) -> Option<LocationInfo> {
        self.location_info
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        self.logic.can_access(progress)
    }
}

impl Eq for Check {}

impl PartialEq<Self> for Check {
    fn eq(&self, other: &Self) -> bool {
        self.get_name().eq(other.get_name())
    }
}

impl Hash for Check {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

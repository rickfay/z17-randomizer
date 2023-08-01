use game::world::LocationNode;

use {
    crate::{
        model::{logic::Logic, progress::Progress},
        FillerItem,
    },
    std::hash::{Hash, Hasher},
};

/// A specific location containing a randomized item
#[derive(Copy, Clone, Debug)]
pub struct Check {
    name: &'static str,
    logic: Logic,
    quest: Option<FillerItem>,
    location: Option<LocationNode>,
}

impl Check {
    pub fn new(
        name: &'static str, logic: Logic, quest: Option<FillerItem>, location: Option<LocationNode>,
    ) -> Self {
        Self { name, logic, quest, location }
    }

    pub fn get_name(self) -> &'static str {
        self.name
    }

    pub fn get_quest(self) -> Option<FillerItem> {
        self.quest
    }

    pub fn get_location(self) -> Option<LocationNode> {
        self.location
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

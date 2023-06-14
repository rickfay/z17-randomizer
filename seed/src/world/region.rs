use std::collections::BTreeSet;
use {
    crate::{
        hints::hint_color::HintColor,
        world::{LocationId, RegionId},
    },
};

/// Region
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Region {
    id: RegionId,
    name: String,
    color: HintColor,
    locations: BTreeSet<LocationId>,
}

impl Region {
    pub(crate) fn new(
        id: RegionId, name: &str, color: HintColor, locations: BTreeSet<LocationId>,
    ) -> Self {
        Self { id, name: name.to_owned(), color, locations }
    }

    pub fn get_id(&self) -> RegionId {
        self.id.to_owned()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(&self) -> HintColor {
        self.color.to_owned()
    }

    pub fn get_locations(&self) -> &BTreeSet<LocationId> {
        &self.locations
    }
}

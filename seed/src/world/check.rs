use crate::{
    filler::{filler_item::FillerItem, progress::Progress},
    world::{logic::Logic, CheckId},
};

/// A specific location containing a randomized item
#[derive(Clone, Debug)]
pub struct Check {
    id: CheckId,
    name: String,
    check_type: CheckType,
    logic: Logic,
    item: Option<FillerItem>,
}

impl Check {
    pub fn new(id: CheckId, name: String, check_type: CheckType, logic: Logic) -> Self {
        Self { id, name, check_type, logic, item: None }
    }

    pub fn get_item(&self) -> Option<FillerItem> {
        self.item
    }

    pub fn set_item(&mut self, item: FillerItem) {
        self.item = Some(item);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        self.logic.can_access(progress)
    }

    pub fn get_check_id(&self) -> CheckId {
        self.id.to_owned()
    }
}

/// The type of Patch to create
#[derive(Clone, Debug)]
pub enum CheckType {
    Chest { file: String, unq: u16 },
    BigChest { file: String, unq: u16 },
    Heart { file: String, unq: u16 },
    Key { file: String, unq: u16 },
    SilverRupee { file: String, unq: u16 },
    GoldRupee { file: String, unq: u16 },
    Maiamai { file: String, unq: u16 },
    Msbf { archive: String, file: String, index: u16 },
    Ravio(u8),
    StreetMerchant(u8),
    Multi(Vec<CheckType>),
    None,
}

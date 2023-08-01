use std::collections::HashMap;

use game::world::{self, LocationNode, NamedArea};
use log::debug;
use serde::{ser::SerializeMap, Serialize, Serializer};

use hints::Hints;
pub use item::Item;
pub use settings::Settings;

pub mod filler_item;
pub mod hints;
pub mod item;
pub mod settings;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
}

impl Error {
    fn new(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

#[derive(Debug, Serialize)]
pub struct Mod {
    pub name: String,
    pub hash: Option<String>,
    pub settings: Settings,
    pub items: Items,
    pub hints: Hints,
}

/// A world layout for the patcher.
#[derive(Clone, Debug, Default)]
pub struct Items(HashMap<LocationNode, Item>);

impl Items {
    fn map(&self) -> &HashMap<LocationNode, Item> {
        &self.0
    }

    fn map_mut(&mut self) -> &mut HashMap<LocationNode, Item> {
        &mut self.0
    }

    pub fn get(&self, location: &LocationNode) -> Option<Item> {
        self.map().get(location).copied()
    }

    /// This just highlights why we need to redo [`Layout`]
    pub fn find_single(&self, item: Item) -> Option<LocationNode> {
        self.map()
            .iter()
            .find_map(|(location, value)| if item == *value { Some(*location) } else { None })
    }

    pub fn set(&mut self, location: LocationNode, item: Item) {
        self.map_mut().insert(location, item);
        debug!("Placed {} in {}/{}", item.as_ref(), location.area.name(), location.name);
    }
}

impl Serialize for Items {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct Wrapper<'a> {
            area: &'a NamedArea,
            items: &'a Items,
        }

        impl<'a> Serialize for Wrapper<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                fn serialize_area<M>(
                    map: &mut M, area: &NamedArea, items: &Items,
                ) -> Result<(), M::Error>
                where
                    M: SerializeMap,
                {
                    for area in area.areas {
                        if let Some(name) = area.name {
                            map.serialize_entry(name, &Wrapper { area, items })?;
                        } else {
                            serialize_area(map, area, items)?;
                        }
                    }
                    for location in area.locations() {
                        if let Some(item) = items.get(&location) {
                            map.serialize_entry(location.name, item.as_ref())?;
                        }
                    }
                    Ok(())
                }

                let mut map = serializer.serialize_map(None)?;
                serialize_area(&mut map, self.area, self.items)?;
                map.end()
            }
        }

        Wrapper { area: &world::NAMED_AREA, items: self }.serialize(serializer)
    }
}

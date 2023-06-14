use {
    crate::{
        filler::{filler_item::FillerItem, location::LocationId},
        world::{check::Check, LocationId},
    },
    jack::lms::msbt::formatting,
    rand::prelude::{IteratorRandom, StdRng},
    serde::{ser::SerializeStruct, Serialize, Serializer},
};

pub mod hint_color;
mod text;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Hints {
    pub path_hints: Vec<PathHint>,
    pub always_hints: Vec<LocationHint>,
    pub sometimes_hints: Vec<LocationHint>,
    pub bow_of_light_hint: Option<BowOfLightHint>,
}

/// Basic functionality for all in-game hints.
pub trait Hint: Serialize {
    fn get_hint(&self) -> String;
    fn get_hint_spoiler(&self) -> String;
}

/// A [`Hint`] that exposes the item at a certain location
#[derive(Debug, Clone)]
pub struct LocationHint {
    /// The hinted item
    pub item: FillerItem,

    pub location: LocationId,

    /// The specific [`Check`] containing the hinted item.
    pub check: Check,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    pub logical_ghosts: Vec<FillerItem>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    pub ghosts: Vec<FillerItem>,
}

impl LocationHint {
    pub fn choose_ghost(
        &mut self, rng: &mut StdRng, taken_ghosts: &mut Vec<FillerItem>,
    ) -> Result<(), &'static str> {
        match self
            .logical_ghosts
            .iter()
            .filter(|ghost| !taken_ghosts.contains(&ghost))
            .choose_stable(rng)
        {
            None => Err("No Ghosts available to place this hint"),
            Some(ghost) => {
                self.ghosts.push(*ghost);
                taken_ghosts.push(*ghost);
                Ok(())
            }
        }
    }
}

impl Hint for LocationHint {
    fn get_hint(&self) -> String {
        // format!(
        //     "It says here that {}\nhas the {}.",
        //     formatting::name(&self.check.get_location_info().unwrap().name()),
        //     formatting::name(&self.item.as_str_colorized())
        // )
        todo!()
    }

    fn get_hint_spoiler(&self) -> String {
        // format!(
        //     "It says here that {} has the {}.",
        //     &self.check.get_location_info().unwrap().name(),
        //     &self.item.as_str()
        // )
        todo!()
    }
}

impl Serialize for LocationHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("LocationHint", 2)?;
        ser.serialize_field("hint", &self.get_hint_spoiler())?;
        ser.serialize_field("ghosts", &self.ghosts)?;
        ser.end()
    }
}

/// A [`Hint`] that tells where an item needed to reach a specific `gaol` is located.
#[derive(Debug, Clone)]
pub struct PathHint {
    /// The specific [`Check`] containing the hinted item.
    pub check: Check,

    /// The goal that this hint leads to.
    pub goal: FillerItem,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    pub logical_ghosts: Vec<FillerItem>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    pub ghosts: Vec<FillerItem>,
}

impl Hint for PathHint {
    fn get_hint(&self) -> String {
        // format!(
        //     "It says here that {}\nis on the path to {}.",
        //     &self.check.get_location_info().unwrap().region_colorized(),
        //     &self.goal.as_str_colorized()
        // )
        todo!()
    }

    fn get_hint_spoiler(&self) -> String {
        // format!(
        //     "It says here that {} is on the path to {}.",
        //     self.check.get_location_info().unwrap().region(),
        //     self.goal.as_str()
        // )
        todo!()
    }
}

impl Serialize for PathHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("PathHint", 2)?;
        ser.serialize_field("hint", &self.get_hint_spoiler())?;
        ser.serialize_field("ghosts", &self.ghosts)?;
        ser.end()
    }
}

/// A [`Hint`] specifically for the Bow of Light.
#[derive(Debug, Clone)]
pub struct BowOfLightHint {
    /// The specific [`Check`] containing the Bow of Light.
    pub check: Check,
}

impl Hint for BowOfLightHint {
    fn get_hint(&self) -> String {
        // format!(
        //     "Did you find the {}\nin {}?",
        //     formatting::name("Bow of Light"),
        //     &self.check.get_location_info().unwrap().region_colorized(),
        // )
        todo!()
    }

    fn get_hint_spoiler(&self) -> String {
        // format!(
        //     "Did you find the Bow of Light in {}?",
        //     &self.check.get_location_info().unwrap().region()
        // )
        todo!()
    }
}

impl Serialize for BowOfLightHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.get_hint_spoiler())
    }
}

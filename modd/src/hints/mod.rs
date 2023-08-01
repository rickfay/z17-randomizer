use std::fmt::Debug;

use serde::ser::{Serialize, SerializeSeq};

use crate::filler_item::FillerItem;
use formatting::*;

pub mod formatting;

#[derive(Debug, Default)]
pub struct Hints {
    pub path_hints: Vec<Box<dyn Hint>>,
    pub always_hints: Vec<Box<dyn Hint>>,
    pub sometimes_hints: Vec<Box<dyn Hint>>,
    pub bow_of_light_hint: Option<Box<dyn Hint>>,
}

impl Serialize for Hints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        for hint in self
            .path_hints
            .iter()
            .chain(self.always_hints.iter())
            .chain(self.sometimes_hints.iter())
            .chain(self.bow_of_light_hint.iter())
        {
            if let Some(spoiler) = hint.get_hint_spoiler() {
                seq.serialize_element(&spoiler)?;
            }
        }
        seq.end()
    }
}

/// Basic functionality for all in-game hints.
pub trait Hint: Debug {
    fn get_hint(&self) -> Option<String>;
    fn get_hint_spoiler(&self) -> Option<String>;
    fn ghosts(&self) -> &[FillerItem];
}

/// The color to use when displaying a particular piece of hinted text.
#[allow(dead_code)]
pub enum HintColor {
    Black,
    Gray,
    White,
    Beige,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Name,
    Attention,
    YugaTalking,
}

impl HintColor {
    pub fn format(&self, text: &str) -> String {
        match self {
            Self::Black => black(text),
            Self::Gray => gray(text),
            Self::White => white(text),
            Self::Beige => beige(text),
            Self::Red => red(text),
            Self::Green => green(text),
            Self::Blue => blue(text),
            Self::Yellow => yellow(text),
            Self::Purple => purple(text),
            Self::Name => name(text),
            Self::Attention => attention(text),
            Self::YugaTalking => yuga_talking(text),
        }
    }
}

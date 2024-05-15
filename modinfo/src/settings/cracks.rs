use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Cracks Open/Closed Settings
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Cracks {
    /// All Cracks except the Hyrule Castle Crack (and its pair) remain closed until the Quake Item is found.
    #[default]
    Closed,
    /// All Cracks are open from the start of the game, and the Quake Item is not in the item pool.
    Open,
}

impl TryFrom<u8> for Cracks {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Closed),
            1 => Ok(Self::Open),
            _ => Err(format!("Invalid Cracks setting: {}", value)),
        }
    }
}

impl TryFrom<String> for Cracks {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Closed" => Ok(Self::Closed),
            "Open" => Ok(Self::Open),
            _ => Err(format!("Invalid Cracks setting: {}", value)),
        }
    }
}

impl Display for Cracks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Closed => "Closed",
                Self::Open => "Open",
            }
        )
    }
}

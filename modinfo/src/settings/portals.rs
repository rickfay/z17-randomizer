use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Portals Open/Closed Settings
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Portals {
    /// All Portals except the Hyrule Castle Portal (and its pair) remain closed until the Quake Item is found.
    #[default]
    Closed,
    /// All Portals are open from the start of the game, and the Quake Item is not in the item pool.
    Open,
}

impl TryFrom<u8> for Portals {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Closed),
            1 => Ok(Self::Open),
            _ => Err("Invalid Portals index: {}".to_owned()),
        }
    }
}

impl Display for Portals {
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

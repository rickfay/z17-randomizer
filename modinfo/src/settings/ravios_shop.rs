use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Ravio's Shop setting
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum RaviosShop {
    /// Closed Shop. Game starts post-Sanctuary, with Link waking up to Ravio in his house. Players must activate one
    /// of three triggers to fully open the shop, but the Bow Slot item can be gotten early with the "Signs" event.
    Closed,
    /// Open Shop. Game starts with Ravio's Shop already open for business. The Shop and Sign triggers do nothing.
    #[default]
    Open,
}

impl TryFrom<u8> for RaviosShop {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Closed),
            1 => Ok(Self::Open),
            _ => Err("Invalid RaviosShop index: {}".to_owned()),
        }
    }
}

impl Display for RaviosShop {
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

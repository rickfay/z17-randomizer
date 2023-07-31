use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use HyruleCastle::*;

/// Choose how the randomizer handles the Dungeon portion of Hyrule Castle
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum HyruleCastle {
    /// Allows early access to Lorule Castle via the Portal + Trial's Door.
    EarlyLoruleCastle,
    /// Closes Hyrule Castle completely, denying all access and removing it from logic
    #[default]
    Closed,
}

impl TryFrom<u8> for HyruleCastle {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EarlyLoruleCastle),
            2 => Ok(Closed),
            _ => Err("Invalid Hyrule Castle Requirement: {}".to_owned()),
        }
    }
}

impl TryFrom<&str> for HyruleCastle {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "EarlyLoruleCastle" => Ok(EarlyLoruleCastle),
            "Closed" => Ok(Closed),
            _ => Err("Invalid Hyrule Castle Requirement: {}".to_owned()),
        }
    }
}

impl Display for HyruleCastle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EarlyLoruleCastle => "Early Lorule Castle",
                Closed => "Closed",
            }
        )
    }
}

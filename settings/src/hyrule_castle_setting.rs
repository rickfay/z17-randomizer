use {
    crate::hyrule_castle_setting::HyruleCastleSetting::*,
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter},
};

/// Choose how the randomizer handles the Dungeon portion of Hyrule Castle
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum HyruleCastleSetting {
    /// Allows early access to Lorule Castle via the Portal + Trial's Door.
    EarlyLoruleCastle,
    /// Closes Hyrule Castle completely, denying all access and removing it from logic
    Closed,
}

impl TryFrom<u8> for HyruleCastleSetting {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EarlyLoruleCastle),
            2 => Ok(Closed),
            _ => Err("Invalid Hyrule Castle Requirement: {}".to_owned()),
        }
    }
}

impl TryFrom<&str> for HyruleCastleSetting {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "EarlyLoruleCastle" => Ok(EarlyLoruleCastle),
            "Closed" => Ok(Closed),
            _ => Err("Invalid Hyrule Castle Requirement: {}".to_owned()),
        }
    }
}

impl Default for HyruleCastleSetting {
    fn default() -> Self {
        Closed
    }
}

impl Display for HyruleCastleSetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            EarlyLoruleCastle => "Early Lorule Castle",
            Closed => "Closed",
        })
    }
}

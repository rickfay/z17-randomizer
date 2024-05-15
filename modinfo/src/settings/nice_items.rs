use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Setting for handling Nice Items and Mother Maiamai Rewards
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum NiceItems {
    /// Nice Items are obtained as upgrades from Mother Maiamai as in the vanilla game.
    Vanilla,

    /// Two progressive copies of each Ravio item are freely shuffled, and Mother Maiamai's rewards are randomized.
    Shuffled,

    /// Remove and replaces the Nice Items with junk, and Mother Maiamai's rewards are randomized.
    #[default]
    Off,
}

impl TryFrom<u8> for NiceItems {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Vanilla),
            1 => Ok(Self::Shuffled),
            2 => Ok(Self::Off),

            _ => Err(format!("Invalid NiceItems setting: {}", value)),
        }
    }
}

impl TryFrom<String> for NiceItems {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Vanilla" => Ok(Self::Vanilla),
            "Shuffled" => Ok(Self::Shuffled),
            "Off" => Ok(Self::Off),

            _ => Err(format!("Invalid NiceItems setting: {}", value)),
        }
    }
}

impl Display for NiceItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vanilla => "Vanilla",
                Self::Shuffled => "Shuffled",
                Self::Off => "Off",
            }
        )
    }
}

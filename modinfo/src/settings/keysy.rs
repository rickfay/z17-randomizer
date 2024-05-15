use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Keysy removes locked keys and doors from dungeons if enabled.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Keysy {
    /// Key doors remain as they are in vanilla.
    #[default]
    Off,
    /// Small Keys and locked doors are removed from all dungeons.
    SmallKeysy,
    /// Big Keys and huge doors are removed from all dungeons.
    BigKeysy,
    /// All Keys and their doors are removed from all dungeons.
    AllKeysy,
}

impl TryFrom<u8> for Keysy {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::SmallKeysy),
            2 => Ok(Self::BigKeysy),
            3 => Ok(Self::AllKeysy),
            _ => Err(format!("Invalid Keysy setting: {}", value)),
        }
    }
}

impl TryFrom<String> for Keysy {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Off" => Ok(Self::Off),
            "Small Keysy" => Ok(Self::SmallKeysy),
            "Big Keysy" => Ok(Self::BigKeysy),
            "All Keysy" => Ok(Self::AllKeysy),
            _ => Err(format!("Invalid Keysy setting: {}", value)),
        }
    }
}

impl Display for Keysy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Off => "Off",
                Self::SmallKeysy => "Small Keysy",
                Self::BigKeysy => "Big Keysy",
                Self::AllKeysy => "All Keysy",
            }
        )
    }
}

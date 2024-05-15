use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Crackanity
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Cracksanity {
    /// Cracks are not shuffled
    #[default]
    Off,
    /// Cracks are shuffled, but remain in HyLo pairs
    CrossWorldPairs,
    /// Cracks are shuffled freely, and can lead to the same or opposite world
    AnyWorldPairs,
    /// Same as CrossWorldPairs, but each pair's vanilla counterparts will be in a matching pair
    MirroredCrossWorldPairs,
    /// Same as AnyWorldPairs, but each pair's vanilla counterparts will be in a matching pair
    MirroredAnyWorldPairs,
}

impl TryFrom<u8> for Cracksanity {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::CrossWorldPairs),
            2 => Ok(Self::AnyWorldPairs),
            3 => Ok(Self::MirroredCrossWorldPairs),
            4 => Ok(Self::MirroredAnyWorldPairs),
            _ => Err(format!("Invalid CrackShuffle setting: {}", value)),
        }
    }
}

impl TryFrom<String> for Cracksanity {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Off" => Ok(Self::Off),
            "Cross World Pairs" => Ok(Self::CrossWorldPairs),
            "Any World Pairs" => Ok(Self::AnyWorldPairs),
            "Mirrored Cross World Pairs" => Ok(Self::MirroredCrossWorldPairs),
            "Mirrored Any World Pairs" => Ok(Self::MirroredAnyWorldPairs),
            _ => Err(format!("Invalid CrackShuffle setting: {}", value)),
        }
    }
}

impl Display for Cracksanity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Off => "Off",
                Self::CrossWorldPairs => "Cross World Pairs",
                Self::AnyWorldPairs => "Any World Pairs",
                Self::MirroredCrossWorldPairs => "Mirrored Cross World Pairs",
                Self::MirroredAnyWorldPairs => "Mirrored Any World Pairs",
            }
        )
    }
}

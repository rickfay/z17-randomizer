use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Portal Shuffle Setting
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PortalShuffle {
    /// Portals are not shuffled
    #[default]
    Off,
    /// Portals are shuffled, but remain in HyLo pairs
    CrossWorldPairs,
    /// Portals are shuffled freely, and can lead to the same or opposite world
    AnyWorldPairs,
    /// Same as CrossWorldPairs, but each pair's vanilla counterparts will be in a matching pair
    MirroredCrossWorldPairs,
    /// Same as AnyWorldPairs, but each pair's vanilla counterparts will be in a matching pair
    MirroredAnyWorldPairs,
}

impl TryFrom<u8> for PortalShuffle {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::CrossWorldPairs),
            2 => Ok(Self::AnyWorldPairs),
            3 => Ok(Self::MirroredCrossWorldPairs),
            4 => Ok(Self::MirroredAnyWorldPairs),
            _ => Err("Invalid PortalShuffle index: {}".to_owned()),
        }
    }
}

impl Display for PortalShuffle {
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

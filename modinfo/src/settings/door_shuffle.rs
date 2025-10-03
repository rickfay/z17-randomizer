use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Crack Shuffle
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum DoorShuffle {
    /// Cracks are not shuffled
    #[default]
    Off,
    /// Cracks are shuffled, but remain in HyLo pairs
    DungeonEntrances,
}

impl TryFrom<u8> for DoorShuffle {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::DungeonEntrances),
            _ => Err("Invalid DoorShuffle index: {}".to_owned()),
        }
    }
}

impl Display for DoorShuffle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Off => "Off",
                Self::DungeonEntrances => "Dungeon Entrances",
            }
        )
    }
}

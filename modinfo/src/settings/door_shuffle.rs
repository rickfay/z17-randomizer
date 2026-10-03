use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Door Shuffle
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum DoorShuffle {
    /// Doors are not shuffled
    #[default]
    Off,
    /// Dungeon Doors are shuffled
    DungeonEntrances,
    /// All Doors are shuffled, and may cross between Hyrule and Lorule
    Crossed,
}

impl TryFrom<u8> for DoorShuffle {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::DungeonEntrances),
            2 => Ok(Self::Crossed),
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
                Self::Crossed => "Crossed",
            }
        )
    }
}

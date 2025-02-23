use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum WeatherVanes {
    /// Only the standard, complimentary Weather Vanes (Link's House & Vacant House) are active at game start
    #[default]
    Standard,
    /// Shuffles the Weather Vanes in pairs, so each activates a warp to a random Weather Vane.
    Shuffled,
    /// Activate all the Weather Vanes that don't affect logic, but do make getting around easier
    Convenient,
    /// Activate the 9 Hyrule Weather Vanes at game start
    Hyrule,
    /// Activate the 13 Lorule Weather Vanes at game start
    Lorule,
    /// Activate all 22 Weather Vanes at game start
    All,
}

impl TryFrom<u8> for WeatherVanes {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WeatherVanes::Standard),
            1 => Ok(WeatherVanes::Shuffled),
            2 => Ok(WeatherVanes::Convenient),
            3 => Ok(WeatherVanes::Hyrule),
            4 => Ok(WeatherVanes::Lorule),
            5 => Ok(WeatherVanes::All),
            _ => Err("Invalid Weather Vane Setting: {}".to_owned()),
        }
    }
}

impl Display for WeatherVanes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WeatherVanes::Standard => "Standard",
                WeatherVanes::Shuffled => "Shuffled",
                WeatherVanes::Convenient => "Convenient",
                WeatherVanes::Hyrule => "Hyrule",
                WeatherVanes::Lorule => "Lorule",
                WeatherVanes::All => "All",
            }
        )
    }
}

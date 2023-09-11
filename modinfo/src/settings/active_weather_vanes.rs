use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum ActiveWeatherVanes {
    /// Only the default Weather Vanes (Link's House & Vacant House) are active at game start
    #[default]
    Default,
    /// Activate the 9 Hyrule Weather Vanes at game start
    Hyrule,
    /// Activate the 13 Lorule Weather Vanes at game start
    Lorule,
    /// Activate all 22 Weather Vanes at game start
    All,
}

impl TryFrom<u8> for ActiveWeatherVanes {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActiveWeatherVanes::Default),
            1 => Ok(ActiveWeatherVanes::Hyrule),
            2 => Ok(ActiveWeatherVanes::Lorule),
            3 => Ok(ActiveWeatherVanes::All),
            _ => Err("Invalid Weather Vane Setting: {}".to_owned()),
        }
    }
}

impl Display for ActiveWeatherVanes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActiveWeatherVanes::Default => "Default",
                ActiveWeatherVanes::Hyrule => "Hyrule",
                ActiveWeatherVanes::Lorule => "Lorule",
                ActiveWeatherVanes::All => "All",
            }
        )
    }
}

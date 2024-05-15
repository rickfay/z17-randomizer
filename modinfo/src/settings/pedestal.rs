use crate::settings::pedestal::PedestalSetting::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PedestalSetting {
    Vanilla,
    #[default]
    Standard,
}

impl TryFrom<u8> for PedestalSetting {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Vanilla),
            3 => Ok(Standard),
            _ => Err(format!("Invalid Pedestal Requirement: {}", value)),
        }
    }
}

impl TryFrom<String> for PedestalSetting {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Vanilla" => Ok(Vanilla),
            "Standard" => Ok(Standard),
            _ => Err(format!("Invalid Pedestal Requirement: {}", value)),
        }
    }
}

impl Display for PedestalSetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Vanilla => "Vanilla",
                Standard => "Standard",
            }
        )
    }
}

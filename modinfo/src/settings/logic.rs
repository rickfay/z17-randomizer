use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum LogicMode {
    #[default]
    Normal,
    Hard,
    Glitched,
    AdvGlitched,
    Hell,
    NoLogic,
}

impl Display for LogicMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => "Normal",
                Self::Hard => "Hard",
                Self::Glitched => "Glitched",
                Self::AdvGlitched => "Advanced Glitched",
                Self::Hell => "Hell",
                Self::NoLogic => "No Logic",
            }
        )
    }
}

impl TryFrom<String> for LogicMode {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Normal" => Ok(Self::Normal),
            "Hard" => Ok(Self::Hard),
            "Glitched" => Ok(Self::Glitched),
            "Advanced Glitched" => Ok(Self::AdvGlitched),
            "Hell" => Ok(Self::Hell),
            "No Logic" => Ok(Self::NoLogic),
            _ => Err(format!("Invalid LogicMode: {}", value)),
        }
    }
}

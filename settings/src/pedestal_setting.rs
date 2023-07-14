use {
    crate::pedestal_setting::PedestalSetting::{Charmed, Standard, Vanilla},
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PedestalSetting {
    Vanilla,
    Charmed,
    Standard,
}

impl TryFrom<u8> for PedestalSetting {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Vanilla),
            3 => Ok(Charmed),
            4 => Ok(Standard),
            _ => Err("Invalid Pedestal Requirement: {}".to_owned()),
        }
    }
}

impl Default for PedestalSetting {
    fn default() -> Self {
        Standard
    }
}

impl Display for PedestalSetting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Vanilla => "Vanilla: Power + Wisdom",
            Charmed => "Charmed: Power + Wisdom + Charm",
            Standard => "Standard: Power + Wisdom + Courage",
        })
    }
}

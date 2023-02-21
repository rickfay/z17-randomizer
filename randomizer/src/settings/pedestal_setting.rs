use {
    crate::{
        fail,
        settings::pedestal_setting::PedestalSetting::{Charmed, Standard, Vanilla},
    },
    serde::{Deserialize, Serialize},
    std::fmt::{Display, Formatter},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PedestalSetting {
    Vanilla,
    Charmed,
    Standard,
}

impl From<u8> for PedestalSetting {
    fn from(value: u8) -> Self {
        match value {
            2 => Vanilla,
            3 => Charmed,
            4 => Standard,
            _ => {
                fail!("Invalid Pedestal Requirement: {}", value);
            }
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

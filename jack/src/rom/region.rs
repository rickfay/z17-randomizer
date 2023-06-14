use std::fmt::{Display, Formatter};

/// Game Region
pub(crate) enum TitleId {
    /// Japan
    JP,
    /// USA
    US,
    /// Europe
    EU,
    /// Taiwan
    TW,
    /// Korea
    KO,
    /// ???
    UNKNOWN,
}

impl From<u64> for TitleId {
    fn from(value: u64) -> Self {
        match value {
            0x00040000000EC200 => Self::JP,
            0x00040000000EC300 => Self::US,
            0x00040000000EC400 => Self::EU,
            0x0004000000115700 => Self::TW,
            0x0004000000115800 => Self::KO,
            _ => Self::UNKNOWN,
        }
    }
}

impl Display for TitleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TitleId::JP => "The Legend of Zelda: A Link Between Worlds (JAPAN)",
            TitleId::US => "The Legend of Zelda: A Link Between Worlds (USA)",
            TitleId::EU => "The Legend of Zelda: A Link Between Worlds (EUROPE)", // PAL? idk
            TitleId::TW => "The Legend of Zelda: A Link Between Worlds (TAIWAN)",
            TitleId::KO => "The Legend of Zelda: A Link Between Worlds (KOREA)",
            TitleId::UNKNOWN => "UNKNOWN"
        })
    }
}
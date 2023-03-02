use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum LogicMode {
    Normal,
    Hard,
    Glitched,
    AdvGlitched,
    Hell,
    NoLogic,
}

impl Default for LogicMode {
    fn default() -> Self {
        LogicMode::Normal
    }
}

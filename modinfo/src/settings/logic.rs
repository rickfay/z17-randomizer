use serde::{Deserialize, Serialize};

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

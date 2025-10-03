use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Trial's Door
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum TrialsDoor {
    /// The Trials Door will open by itself automatically, from inside LC only.
    OpenFromInsideOnly,

    /// The Trials Door will open by itself automatically, from inside LC or in Hilda's Study.
    /// This option may require entering LC early.
    OpenFromBothSides,

    /// Turns on 1 random trial.
    OneTrialRequired,

    /// Turns on 2 random trials.
    TwoTrialsRequired,

    /// Turns on 3 random trials.
    ThreeTrialsRequired,

    /// Turns on all trials.
    AllTrialsRequired,
    // /// The Trials door is sealed shut and cannot be opened from either direction.
    // /// The Lorule Castle Crack must be used to reach Yuga Ganon.
    // Sealed, todo
}

impl Default for TrialsDoor {
    fn default() -> Self {
        Self::OneTrialRequired
    }
}

impl TryFrom<u8> for TrialsDoor {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OpenFromInsideOnly),
            1 => Ok(Self::OneTrialRequired),
            2 => Ok(Self::TwoTrialsRequired),
            3 => Ok(Self::ThreeTrialsRequired),
            4 => Ok(Self::AllTrialsRequired),
            5 => Ok(Self::OpenFromBothSides),
            _ => Err("Invalid LcTrialsDoor index: {}".to_owned()),
        }
    }
}

impl Display for TrialsDoor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TrialsDoor::OpenFromInsideOnly => "Open From Inside Only",
                TrialsDoor::OneTrialRequired => "1 Trial Required",
                TrialsDoor::TwoTrialsRequired => "2 Trials Required",
                TrialsDoor::ThreeTrialsRequired => "3 Trials Required",
                TrialsDoor::AllTrialsRequired => "4 Trials Required",
                TrialsDoor::OpenFromBothSides => "Open From Both Sides",
            }
        )
    }
}

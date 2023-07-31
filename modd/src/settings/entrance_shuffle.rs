use serde::{Deserialize, Serialize};

/**
 * TODO Implement this
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum EntranceShuffle {
    NotShuffled, // Entrances are not shuffled

    Shuffled,      // Entrances are shuffled within their own world
    PortalShuffle, // Portals are shuffled (except Zaganaga)

    CrossShuffle,      // Entrances are shuffled Between Worlds (LUL)
    CrossPortalsanity, // Entrances and Portals are both shuffled within their categories
}

impl Default for EntranceShuffle {
    fn default() -> Self {
        Self::NotShuffled
    }
}

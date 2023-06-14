use {
    crate::{
        filler::progress::Progress,
        settings::logic_mode::{LogicMode, LogicMode::*},
    },
    std::{
        collections::HashMap,
        fmt::{Debug, Formatter},
        mem,
    },
};

/// Represents in-game Logic for reaching a [`Check`] or navigating a [`Path`] to another [`Location`]
/// TODO Externalize the logic, both for organization and to allow users to write custom logic.
///
/// [`Check`]: crate::world::check::Check
/// [`Path`]: randomizer::world::path::Path
/// [`Location`]: crate::filler::location::Location
#[derive(Clone, Default)]
pub struct Logic {
    logics: HashMap<LogicMode, Box<fn(&Progress) -> bool>>,
}

impl Logic {
    pub fn new() -> Self {
        const NUM_LOGIC_MODES: usize = mem::variant_count::<LogicMode>();
        Self { logics: HashMap::with_capacity(NUM_LOGIC_MODES) }
    }

    /// Combinator for adding a [`LogicMode`] definition
    pub fn add(&mut self, logic_mode: LogicMode, logic: fn(&Progress) -> bool) -> &mut Self {
        self.logics.insert(logic_mode, Box::new(logic));
        self
    }

    /// Determines if, with the current [`Progress`], the player satisfies the logical conditions.
    /// Logic is tiered, so as long as at least one condition is satisfied the player should have access.
    pub fn can_access(self, progress: &Progress) -> bool {
        // If no logic defined, considered accessible
        if self.logics.is_empty() {
            return true;
        }

        // LogicModes are tiered
        let logics = match progress.get_logic_mode() {
            Normal => vec![Normal],
            Hard => vec![Normal, Hard],
            Glitched => vec![Normal, Hard, Glitched],
            AdvGlitched => vec![Normal, Hard, Glitched, AdvGlitched],
            Hell => vec![Normal, Hard, Glitched, AdvGlitched, Hell],
            NoLogic => {
                return true;
            }
        };

        // Evaluate
        for logic in logics {
            if let Some(logic) = self.logics.get(&logic) {
                if (*logic)(progress) {
                    return true;
                }
            }
        }

        false
    }
}

impl Debug for Logic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[logic Logic]") // todo
    }
}

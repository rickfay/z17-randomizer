use crate::logic_mode::LogicMode::*;
use crate::progress::Progress;

// TODO I'd eventually like to externalize the logic, both for organization purposes and to allow users to write custom logic. But this is fine for now.

#[derive(Copy, Clone)]
pub struct Logic {
    pub normal: Option<fn(&Progress) -> bool>,
    pub hard: Option<fn(&Progress) -> bool>,
    pub glitch_basic: Option<fn(&Progress) -> bool>,
    pub glitch_advanced: Option<fn(&Progress) -> bool>,
    pub glitch_bees: Option<fn(&Progress) -> bool>,
}

impl Logic {
    pub fn new(normal: Option<fn(&Progress) -> bool>,
               hard: Option<fn(&Progress) -> bool>,
               glitch_basic: Option<fn(&Progress) -> bool>,
               glitch_advanced: Option<fn(&Progress) -> bool>,
               glitch_bees: Option<fn(&Progress) -> bool>) -> Self {
        Self {
            normal,
            hard,
            glitch_basic,
            glitch_advanced,
            glitch_bees,
        }
    }

    pub fn can_access(self, progress: &Progress) -> bool {

        // Progression is available if the current logic or a lower tiered logic passes
        for logic in match progress.get_settings().logic.mode {
            Normal => Vec::from([self.normal]),
            Hard => Vec::from([self.normal, self.hard]),
            GlitchBasic => Vec::from([self.normal, self.hard, self.glitch_basic]),
            GlitchAdvanced => Vec::from([self.normal, self.hard, self.glitch_basic, self.glitch_advanced]),
            GlitchBees => Vec::from([self.normal, self.hard, self.glitch_basic, self.glitch_advanced, self.glitch_bees]),
            NoLogic => { return true; }
        } {
            if logic.is_some() && (logic.unwrap())(progress) {
                return true;
            }
        }

        false
    }

    pub fn free() -> Self {
        Self {
            normal: accessible(),
            hard: accessible(),
            glitch_basic: accessible(),
            glitch_advanced: accessible(),
            glitch_bees: accessible(),
        }
    }
}

pub fn accessible() -> Option<fn(&Progress) -> bool> {
    Some(|_| true)
}
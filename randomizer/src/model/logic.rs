use {
    crate::{model::progress::Progress, settings::logic_mode::LogicMode::*},
    std::fmt::{Debug, Formatter},
};

// TODO I'd eventually like to externalize the logic, both for organization purposes and to allow users to write custom logic. But this is fine for now.

#[derive(Copy, Clone)]
pub struct Logic {
    pub normal: Option<fn(&Progress) -> bool>,
    pub hard: Option<fn(&Progress) -> bool>,
    pub glitched: Option<fn(&Progress) -> bool>,
    pub adv_glitched: Option<fn(&Progress) -> bool>,
    pub hell: Option<fn(&Progress) -> bool>,
}

impl Logic {
    pub fn new(
        normal: Option<fn(&Progress) -> bool>, hard: Option<fn(&Progress) -> bool>,
        glitched: Option<fn(&Progress) -> bool>, adv_glitched: Option<fn(&Progress) -> bool>,
        hell: Option<fn(&Progress) -> bool>,
    ) -> Self {
        Self { normal, hard, glitched, adv_glitched, hell }
    }

    pub fn can_access(self, progress: &Progress) -> bool {
        // Progression is available if the current logic or a lower tiered logic passes
        for logic in match progress.get_settings().logic.logic_mode {
            Normal => Vec::from([self.normal]),
            Hard => Vec::from([self.normal, self.hard]),
            Glitched => Vec::from([self.normal, self.hard, self.glitched]),
            AdvGlitched => Vec::from([self.normal, self.hard, self.glitched, self.adv_glitched]),
            Hell => {
                Vec::from([self.normal, self.hard, self.glitched, self.adv_glitched, self.hell])
            }
            NoLogic => {
                return true;
            }
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
            glitched: accessible(),
            adv_glitched: accessible(),
            hell: accessible(),
        }
    }
}

impl Debug for Logic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[logic Logic]") // todo
    }
}

pub fn accessible() -> Option<fn(&Progress) -> bool> {
    Some(|_| true)
}

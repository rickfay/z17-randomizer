use crate::{
    language::Language,
    Game, Result, SceneMeta,
};

#[derive(Debug)]
pub struct Course<'a> {
    game: &'a Game,
    id: CourseId,
}

impl<'a> Course<'a> {
    pub(crate) fn new(game: &'a Game, id: Id) -> Self {
        Self { game, id }
    }

    pub fn language(&self) -> Result<Language> {
        self.game.language(self.id)
    }

    pub fn scene(&self, index: u16) -> Result<Scene> {
        self.game.scene(self.id, index)
    }

    pub fn scene_meta(&self) -> Option<SceneMeta> {
        self.game.scene_meta(self.id)
    }

    pub fn stage(&self, index: u16) -> Result<Stage> {
        self.game.stage(self.id, index)
    }
}

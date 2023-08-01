use crate::{
    language::Language,
    scene::{Scene, Stage},
    Result, Rom, SceneMeta,
};

#[derive(Debug)]
pub struct Course<'a> {
    game: &'a Rom,
    id: game::Course,
}

impl<'a> Course<'a> {
    pub(crate) fn new(game: &'a Rom, id: game::Course) -> Self {
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

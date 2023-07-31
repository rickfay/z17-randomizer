use crate::{
    language::Language,
    scene::{Scene, Stage},
    Result, Rom, SceneMeta,
};

#[derive(Debug)]
pub struct Course<'a> {
    game: &'a Rom,
    id: Id,
}

impl<'a> Course<'a> {
    pub(crate) fn new(game: &'a Rom, id: Id) -> Self {
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

crate::int_map! {
    /// The id of a course.
    Id(u16) {
        FieldLight = 0x0,
        FieldDark = 0x1,
        IndoorLight = 0x2,
        IndoorDark = 0x3,
        CaveLight = 0x4,
        CaveDark = 0x5,
        LanguageBoot = 0x6, // TODO No idea if this ID is correct
        CrossBattle = 0x7,
        Demo = 0x8,
        DungeonEast = 0x9,
        DungeonWind = 0xA,
        DungeonHera = 0xB,
        DungeonCastle = 0xC,
        DungeonDark = 0xD,
        DungeonWater = 0xE,
        DungeonDokuro = 0xF, // Skull Woods
        DungeonHagure = 0x10, // Thieves Hideout
        DungeonIce = 0x11,
        DungeonSand = 0x12,
        DungeonKame = 0x13, // Turtle Rock
        DungeonGanon = 0x14,
        DungeonBoss = 0x15,
        AttractionLight = 0x16,
        AttractionDark = 0x17,
        EnemyAttackS = 0x18, // Treacherous Tower
        EnemyAttackM = 0x19, // Treacherous Tower
        EnemyAttackL = 0x1A, // Treacherous Tower
}}

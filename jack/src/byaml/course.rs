use {
    crate::byaml::{Flag, Set},
    serde::{self, Deserialize, Serialize},
};

///
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub struct Course {
    pub clp: Option<Vec<Vec<i32>>>,
    pub flr: Vec<i32>,
    pub icn: Vec<Icn>,
    pub kst: Option<Vec<Vec<i32>>>,
    pub retry: Option<Vec<i32>>,
    pub set: Vec<Set>,
}

/// The id of a course.
#[derive(Debug, Copy, Clone)]
pub enum CourseId {
    /// Hyrule Field
    FieldLight      = 0x0,
    /// Lorule Field
    FieldDark       = 0x1,
    /// Hyrule Indoors
    IndoorLight     = 0x2,
    /// Lorule Indoors
    IndoorDark      = 0x3,
    /// Hyrule Caves
    CaveLight       = 0x4,
    /// Lorule Caves
    CaveDark        = 0x5,
    /// todo no idea if this is the correct ID
    LanguageBoot    = 0x6,
    /// StreetPass Battles
    CrossBattle     = 0x7,
    /// Cutscenes
    Demo            = 0x8,
    /// Eastern Palace
    DungeonEast     = 0x9,
    /// House of Gales
    DungeonWind     = 0xA,
    /// Tower of Hera
    DungeonHera     = 0xB,
    /// Hyrule Castle
    DungeonCastle   = 0xC,
    /// Dark Palace
    DungeonDark     = 0xD,
    /// Swamp Palace
    DungeonWater    = 0xE,
    /// Skull Woods
    DungeonDokuro   = 0xF, // Skull Woods
    /// Thieves' Hideout
    DungeonHagure   = 0x10, // Thieves Hideout
    /// Ice Ruins
    DungeonIce      = 0x11,
    /// Desert Palace
    DungeonSand     = 0x12,
    /// Turtle Rock
    DungeonKame     = 0x13, // Turtle Rock
    /// Lorule Castle
    DungeonGanon    = 0x14,
    /// Final Boss
    DungeonBoss     = 0x15,
    /// Hyrule Treasure Dungeons
    AttractionLight = 0x16,
    /// Lorule Treasure Dungeons
    AttractionDark  = 0x17,
    /// Treacherous Tower Beginner
    EnemyAttackS    = 0x18,
    /// Treacherous Tower Intermediate
    EnemyAttackM    = 0x19,
    /// Treacherous Tower Advanced
    EnemyAttackL    = 0x1A,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub struct Icn {
    pub arg: IcnArgs,
    pub pos: Vec<f32>,
    pub scr: Vec<f32>,
    pub msg: Option<String>,
}

impl Icn {
    pub fn enable(&mut self) {
        self.arg.4 = 4;
        self.arg.6 = 1;
    }

    pub fn enable_on(&mut self, flag: Flag) {
        let (arg4, arg6) = flag.into_pair();
        self.arg.4 = arg4;
        self.arg.6 = arg6;
    }

    pub fn clear_enabled(&mut self) {
        self.arg.4 = 0;
        self.arg.6 = 0;
    }

    pub fn disable(&mut self) {
        self.arg.5 = 4;
        self.arg.7 = 1;
    }

    pub fn disable_on(&mut self, flag: Flag) {
        let (arg5, arg7) = flag.into_pair();
        self.arg.5 = arg5;
        self.arg.7 = arg7;
    }

    pub fn clear_disabled(&mut self) {
        self.arg.5 = 0;
        self.arg.7 = 0;
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(deny_unknown_fields)]
pub struct IcnArgs(pub i32, pub i32, pub i32, pub i32, pub u8, pub u8, pub u16, pub u16);

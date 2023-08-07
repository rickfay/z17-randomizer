crate::int_map! {
    /// The id of a course.
    Course(u16) {
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

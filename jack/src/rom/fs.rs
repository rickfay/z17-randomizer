//! Representation of the internal ALBW ROM File System

/// Contains most 3D game assets for individual stages, packaged as Yaz0-compressed SARC Archives (.szs files)
#[allow(non_snake_case)]
pub mod Archive {
    crate::file_paths!("Archive/",
        ACTOR_COMMON: "ActorCommon.szs",
        ACTOR_PROFILE: "ActorProfile.szs",
    );
}

/// Contains Region specific information for the US version of the game (shared between English/French/Spanish versions)
#[allow(non_snake_case)]
pub mod US {
    crate::file_paths!("US/",
        /// Standard ALBW Font
        MESSAGE_FONT: "MessageFont.bffnt",
        REGION_BOOT: "RegionBoot.szs",
    );

    /// Manual info, probably for the 3DS Home menu
    pub mod Manual {

        /// ???
        pub mod bcma {
            pub const MANUAL: &str = "US/Manual/bcma/Manual.bcma";
        }

        /// ???
        pub mod cfa {
            pub const MANUAL: &str = "US/Manual/cfa/Manual.cfa";
        }
    }
}

/// Contains English-language specific files, mostly FlowChart (MSBF) and Message (MSBT) files.
#[allow(non_snake_case)]
pub mod US_English {
    crate::file_paths!("US_English/",
        /// Lorule Treasure Dungeons
        ATTRACTION_DARK: "AttractionDark.szs",
        /// Hyrule Treasure Dungeons
        ATTRACTION_LIGHT: "AttractionLight.szs",
        /// Lorule Caves
        CAVE_DARK: "CaveDark.szs",
        /// Hyrule Caves
        CAVE_LIGHT: "CaveLight.szs",
        /// StreetPass Battles
        CROSS_BATTLE: "CrossBattle.szs",
        /// Cutscenes
        DEMO: "Demo.szs",
        /// Final Boss Fight
        DUNGEON_BOSS: "DungeonBoss.szs",
        /// Hyrule Castle
        DUNGEON_CASTLE: "DungeonCastle.szs",
        /// Dark Palace
        DUNGEON_DARK: "DungeonDark.szs",
        /// Skull Woods
        DUNGEON_DOKURO: "DungeonDokuro.szs",
        /// Eastern Palace
        DUNGEON_EAST: "DungeonEast.szs",
        /// Lorule Castle
        DUNGEON_GANON: "DungeonGanon.szs",
        /// Thieves' Hideout
        DUNGEON_HAGURE: "DungeonHagure.szs",
        /// Tower of Hera
        DUNGEON_HERA: "DungeonHera.szs",
        /// Ice Ruins
        DUNGEON_ICE: "DungeonIce.szs",
        /// Turtle Rock
        DUNGEON_KAME: "DungeonKame.szs",
        /// Desert Palace
        DUNGEON_SAND: "DungeonSand.szs",
        /// Swamp Palace
        DUNGEON_WATER: "DungeonWater.szs",
        /// House of Gales
        DUNGEON_WIND: "DungeonWind.szs",
        /// Treacherous Tower Advanced
        ENEMY_ATTACK_L: "EnemyAttackL.szs",
        /// Treacherous Tower Intermediate
        ENEMY_ATTACK_M: "EnemyAttackM.szs",
        /// Treacherous Tower Beginner
        ENEMY_ATTACK_S: "EnemyAttackS.szs",
        /// Lorule Field
        FIELD_DARK: "FieldDark.szs",
        /// Hyrule Field
        FIELD_LIGHT: "FieldLight.szs",
        /// Lorule Indoors
        INDOOR_DARK: "IndoorDark.szs",
        /// Hyrule Indoors
        INDOOR_LIGHT: "IndoorLight.szs",
        /// Common English-specific Files
        LANGUAGE_BOOT: "LanguageBoot.szs",
    );

    /// ???
    pub mod Layout {
        crate::file_paths!("US_English/Layout/",
            DRAW_00: "Draw_00.bflim",
            FAILURE_00: "Failure_00.bflim",
            FINISH_00: "Finish_00.bflim",
            GO_00: "GO_00.bflim",
            LOSE_00: "Lose_00.bflim",
            PLAY_BALL_00: "PlayBall_00.bflim",
            START_00: "Start_00.bflim",
            THE_END_00: "TheEnd_00.bflim",
            TIME_UP_00: "TimeUp_00.bflim",
            WIN_00: "Win_00.bflim",
        );
    }
}

/// Contains French-language specific files, mostly FlowChart (MSBF) and Message (MSBT) files.
#[allow(non_snake_case)]
pub mod US_French {
    crate::file_paths!("US_French/",
        /// Lorule Treasure Dungeons
        ATTRACTION_DARK: "AttractionDark.szs",
        /// Hyrule Treasure Dungeons
        ATTRACTION_LIGHT: "AttractionLight.szs",
        /// Lorule Caves
        CAVE_DARK: "CaveDark.szs",
        /// Hyrule Caves
        CAVE_LIGHT: "CaveLight.szs",
        /// StreetPass Battles
        CROSS_BATTLE: "CrossBattle.szs",
        /// Cutscenes
        DEMO: "Demo.szs",
        /// Final Boss Fight
        DUNGEON_BOSS: "DungeonBoss.szs",
        /// Hyrule Castle
        DUNGEON_CASTLE: "DungeonCastle.szs",
        /// Dark Palace
        DUNGEON_DARK: "DungeonDark.szs",
        /// Skull Woods
        DUNGEON_DOKURO: "DungeonDokuro.szs",
        /// Eastern Palace
        DUNGEON_EAST: "DungeonEast.szs",
        /// Lorule Castle
        DUNGEON_GANON: "DungeonGanon.szs",
        /// Thieves' Hideout
        DUNGEON_HAGURE: "DungeonHagure.szs",
        /// Tower of Hera
        DUNGEON_HERA: "DungeonHera.szs",
        /// Ice Ruins
        DUNGEON_ICE: "DungeonIce.szs",
        /// Turtle Rock
        DUNGEON_KAME: "DungeonKame.szs",
        /// Desert Palace
        DUNGEON_SAND: "DungeonSand.szs",
        /// Swamp Palace
        DUNGEON_WATER: "DungeonWater.szs",
        /// House of Gales
        DUNGEON_WIND: "DungeonWind.szs",
        /// Treacherous Tower Advanced
        ENEMY_ATTACK_L: "EnemyAttackL.szs",
        /// Treacherous Tower Intermediate
        ENEMY_ATTACK_M: "EnemyAttackM.szs",
        /// Treacherous Tower Beginner
        ENEMY_ATTACK_S: "EnemyAttackS.szs",
        /// Lorule Field
        FIELD_DARK: "FieldDark.szs",
        /// Hyrule Field
        FIELD_LIGHT: "FieldLight.szs",
        /// Lorule Indoors
        INDOOR_DARK: "IndoorDark.szs",
        /// Hyrule Indoors
        INDOOR_LIGHT: "IndoorLight.szs",
        /// Common English-specific Files
        LANGUAGE_BOOT: "LanguageBoot.szs",
    );

    /// ???
    pub mod Layout {
        crate::file_paths!("US_French/Layout/",
            DRAW_00: "Draw_00.bflim",
            FAILURE_00: "Failure_00.bflim",
            FINISH_00: "Finish_00.bflim",
            GO_00: "GO_00.bflim",
            LOSE_00: "Lose_00.bflim",
            PLAY_BALL_00: "PlayBall_00.bflim",
            START_00: "Start_00.bflim",
            THE_END_00: "TheEnd_00.bflim",
            TIME_UP_00: "TimeUp_00.bflim",
            WIN_00: "Win_00.bflim",
        );
    }
}

/// Contains Spanish-language specific files, mostly FlowChart (MSBF) and Message (MSBT) files.
#[allow(non_snake_case)]
pub mod US_Spanish {
    crate::file_paths!("US_Spanish/",
        /// Lorule Treasure Dungeons
        ATTRACTION_DARK: "AttractionDark.szs",
        /// Hyrule Treasure Dungeons
        ATTRACTION_LIGHT: "AttractionLight.szs",
        /// Lorule Caves
        CAVE_DARK: "CaveDark.szs",
        /// Hyrule Caves
        CAVE_LIGHT: "CaveLight.szs",
        /// StreetPass Battles
        CROSS_BATTLE: "CrossBattle.szs",
        /// Cutscenes
        DEMO: "Demo.szs",
        /// Final Boss Fight
        DUNGEON_BOSS: "DungeonBoss.szs",
        /// Hyrule Castle
        DUNGEON_CASTLE: "DungeonCastle.szs",
        /// Dark Palace
        DUNGEON_DARK: "DungeonDark.szs",
        /// Skull Woods
        DUNGEON_DOKURO: "DungeonDokuro.szs",
        /// Eastern Palace
        DUNGEON_EAST: "DungeonEast.szs",
        /// Lorule Castle
        DUNGEON_GANON: "DungeonGanon.szs",
        /// Thieves' Hideout
        DUNGEON_HAGURE: "DungeonHagure.szs",
        /// Tower of Hera
        DUNGEON_HERA: "DungeonHera.szs",
        /// Ice Ruins
        DUNGEON_ICE: "DungeonIce.szs",
        /// Turtle Rock
        DUNGEON_KAME: "DungeonKame.szs",
        /// Desert Palace
        DUNGEON_SAND: "DungeonSand.szs",
        /// Swamp Palace
        DUNGEON_WATER: "DungeonWater.szs",
        /// House of Gales
        DUNGEON_WIND: "DungeonWind.szs",
        /// Treacherous Tower Advanced
        ENEMY_ATTACK_L: "EnemyAttackL.szs",
        /// Treacherous Tower Intermediate
        ENEMY_ATTACK_M: "EnemyAttackM.szs",
        /// Treacherous Tower Beginner
        ENEMY_ATTACK_S: "EnemyAttackS.szs",
        /// Lorule Field
        FIELD_DARK: "FieldDark.szs",
        /// Hyrule Field
        FIELD_LIGHT: "FieldLight.szs",
        /// Lorule Indoors
        INDOOR_DARK: "IndoorDark.szs",
        /// Hyrule Indoors
        INDOOR_LIGHT: "IndoorLight.szs",
        /// Common English-specific Files
        LANGUAGE_BOOT: "LanguageBoot.szs",
    );

    /// ???
    pub mod Layout {
        crate::file_paths!("US_Spanish/Layout/",
            DRAW_00: "Draw_00.bflim",
            FAILURE_00: "Failure_00.bflim",
            FINISH_00: "Finish_00.bflim",
            GO_00: "GO_00.bflim",
            LOSE_00: "Lose_00.bflim",
            PLAY_BALL_00: "PlayBall_00.bflim",
            START_00: "Start_00.bflim",
            THE_END_00: "TheEnd_00.bflim",
            TIME_UP_00: "TimeUp_00.bflim",
            WIN_00: "Win_00.bflim",
        );
    }
}

/// Contains configuration files for individual scenes + cutscenes, environment info, common textures, common models,
/// animations, shaders, sounds, and basically everything else not covered by the other top-level folders.
#[allow(non_snake_case)]
pub mod World {
    /// Contains the BYAML files, which provide scene definitions (e.g. which Actors & MapActors go in which stage, what
    /// properties do they have, etc.)
    pub mod Byaml {
        crate::file_paths!("World/Byaml/",
            ATTRACTION_DARK_STAGE_1: "AttractionDark1_stage.byaml",
            ATTRACTION_DARK_STAGE_2: "AttractionDark2_stage.byaml",
            ATTRACTION_DARK_STAGE_3: "AttractionDark3_stage.byaml",
            ATTRACTION_DARK_COURSE: "AttractionDark_course.byaml",
            FIELD_LIGHT_STAGE_1: "FieldLight1_stage.byaml",
            FIELD_LIGHT_STAGE_30: "FieldLight30_stage.byaml",
            FIELD_LIGHT_STAGE_34: "FieldLight34_stage.byaml",
            INDOOR_LIGHT_STAGE_6: "IndoorLight6_stage.byaml",
            INDOOR_LIGHT_STAGE_10: "IndoorLight10_stage.byaml",
            // todo
        );

        /// IDK probably something to do with colors
        pub mod ColorSet {
            crate::file_paths!("World/Byaml/ColorSet/",
                CS_COMMON: "cs_common.byaml",
                CS_DGN_HERA_IN: "cs_dgn_Hera_in.byaml",
                CS_ICE: "cs_Ice.byaml",
                DGN_WATER_BOSS: "dgn_Water_Boss.byaml",
                SAGES_PLACE: "SagesPlace.byaml",
            );
        }
    }

    /// Contains common textures and models used between different stages of the same course
    pub mod Course {
        crate::file_paths!("World/Course/",
            CAVE_COMMON: "cave_Common.bch",
            // todo
        );
    }

    /// Cutscene assets (.bch) and definition files (.csv)
    pub mod Demo {
        crate::file_paths!("World/Demo/",
            DEMO_1_BCH: "Demo1.bch",
            DEMO_1_CSV: "Demo1.csv",
            // todo
        );
    }

    ///
    pub mod Environment {
        crate::file_paths!("World/Environment/",
            BATTLE_CHANGE: "BattleChange.bch",
            // todo
        );

        ///
        pub mod Fog {
            crate::file_paths!("World/Environment/Fog/",
                CAVE_LIFT_DARK: "cave_LiftDark.bch",
                // todo
            );
        }

        ///
        pub mod LightSet {
            crate::file_paths!("World/Environment/LightSet",
                BG_CAVE_BIG_STAGE: "bg_cave_BigStage.bch",
                // todo
            );
        }
    }

    /// Models of items Link holds overhead when he "Gets" them
    pub mod GetItem {
        crate::file_paths!("World/GetItem/",
            /// Bee Badge
            BADGE_BEE: "BadgeBee.bch",
            /// Bee in a Bottle
            BEE: "Bee.bch",
            /// Nice Bombs
            BOMB_M: "BombM.bch",
            /// Rented Bombs
            BOMB_R: "BombR.bch",
            /// Normal Bombs
            BOMB_S: "BombS.bch",
            /// Compass
            COMPASS: "Compass.bch",
            /// Captain's Shield (unused)
            DELIVER_SHIELD: "DeliverShield.bch",
            /// Sickle (unused...?)
            DELIVER_SICKLE: "DeliverSickle.bch",
            /// Broken Captain's Sword (unused)
            DELIVER_SWORD_BROKEN: "DeliverSwordBroken.bch",
            /// Monster Tail
            DEMON_BLUE: "DemonBlue.bch",
            /// Monster Guts
            DEMON_PURPLE: "DemonPurple.bch",
            /// Monster Horn
            DEMON_YELLOW: "DemonYellow.bch",
            /// Scoot Fruit
            FRUIT_ESCAPE: "FruitEscape.bch",
            /// Foul Fruit
            FRUIT_STOP: "FruitStop.bch",
            /// Bell
            GT_EV_BELL: "GtEvBell.bch",
            // todo
        );
    }

    ///
    pub mod Layout {
        pub const FIELD: &str = "World/Layout/Field.ctpk";

        ///
        pub mod MapTexture {
            crate::file_paths!("World/Layout/MapTexture/",
                LYT_FIELD_DARK: "Lyt_FieldDark.bch",
                LYT_FIELD_LIGHT: "Lyt_FieldLight.bch",
            );
        }
    }

    ///
    pub mod LinkAnim2 {
        crate::file_paths!("World/LinkAnm2/",
            LINK_BED_SLEEP: "Link_BedSleep.bch",
            // todo
        );
    }

    ///
    pub mod PlayerItem {
        crate::file_paths!("World/PlayerItem/",
            ARROW_A: "ArrowA.bch",
            ARROW_B: "ArrowB.bch",
            ARROW_R: "ArrowR.bch",
            BOOMERANG_A: "BoomerangA.bch",
            BOOMERANG_B: "BoomerangB.bch",
            BOOMERANG_R: "BoomerangR.bch",
            BOW_A: "BowA.bch",
            BOW_B: "BowB.bch",
            BOW_R: "BowR.bch",
            BRANCH: "Branch.bch",
            FLAT_LINK_BOX: "FlatLinkBow.bch",
            // todo
        );
    }

    ///
    pub mod Shader {
        pub const PRIMITIVE_RENDERER_CTR: &str = "primitive_renderer_ctr.bsm";
    }

    ///
    pub mod Sound {
        pub const JACK: &str = "Jack.bcsar";

        ///
        pub mod stream {
            crate::file_paths!("World/Sound/stream/",
                AFTER_CREDIT_HARD_MODE_FAN: "AfterCreditHardModeFan.ry.32.dspadpcm.bcstm",
                // todo
            );
        }
    }
}

///
#[macro_export]
macro_rules! file_paths {
    (
        $prefix:literal,
        $(
            $(#[$attr:meta])*
            $key:ident: $val:literal,
        )+
    ) => {
        $(
            $(#[$attr])*
            pub const $key: &str = concat!($prefix, $val);
        )+
    };
}

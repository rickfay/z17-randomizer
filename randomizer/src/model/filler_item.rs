use {
    crate::{hints::hint_color::HintColor::*, item_to_str},
    albw::{Item, Item::*},
    serde::{Deserialize, Serialize, Serializer},
    FillerItem::*,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum FillerItem {
    Empty,

    Bow01,
    Bow02,

    Boomerang01,
    Boomerang02,

    Hookshot01,
    Hookshot02,

    Bombs01,
    Bombs02,

    FireRod01,
    FireRod02,

    IceRod01,
    IceRod02,

    Hammer01,
    Hammer02,

    SandRod01,
    SandRod02,

    TornadoRod01,
    TornadoRod02,

    Bell,
    StaminaScroll,
    BowOfLight,
    PegasusBoots,
    Flippers,
    RaviosBracelet01,
    RaviosBracelet02,
    HylianShield,
    SmoothGem,
    LetterInABottle,
    PremiumMilk,
    Pouch,
    BeeBadge,
    HintGlasses,
    GreatSpin,
    RupeeGreen,
    RupeeBlue,
    RupeeRed,

    RupeePurple01,
    RupeePurple02,
    RupeePurple03,
    RupeePurple04,
    RupeePurple05,
    RupeePurple06,
    RupeePurple07,
    RupeePurple08,
    RupeePurple09,
    RupeePurple10,
    RupeePurple11,
    RupeePurple12,
    RupeePurple13,
    RupeePurple14,
    RupeePurple15,
    RupeePurple16,
    RupeePurple17,
    RupeePurple18,
    RupeePurple19,
    RupeePurple20,

    RupeeSilver01,
    RupeeSilver02,
    RupeeSilver03,
    RupeeSilver04,
    RupeeSilver05,
    RupeeSilver06,
    RupeeSilver07,
    RupeeSilver08,
    RupeeSilver09,
    RupeeSilver10,
    RupeeSilver11,
    RupeeSilver12,
    RupeeSilver13,
    RupeeSilver14,
    RupeeSilver15,
    RupeeSilver16,
    RupeeSilver17,
    RupeeSilver18,
    RupeeSilver19,
    RupeeSilver20,
    RupeeSilver21,
    RupeeSilver22,
    RupeeSilver23,
    RupeeSilver24,
    RupeeSilver25,
    RupeeSilver26,
    RupeeSilver27,
    RupeeSilver28,
    RupeeSilver29,
    RupeeSilver30,
    RupeeSilver31,
    RupeeSilver32,
    RupeeSilver33,
    RupeeSilver34,
    RupeeSilver35,
    RupeeSilver36,
    RupeeSilver37,
    RupeeSilver38,
    RupeeSilver39,
    RupeeSilver40,
    RupeeSilver41,

    RupeeGold01,
    RupeeGold02,
    RupeeGold03,
    RupeeGold04,
    RupeeGold05,
    RupeeGold06,
    RupeeGold07,
    RupeeGold08,
    RupeeGold09,
    RupeeGold10,

    Maiamai001,
    Maiamai002,
    Maiamai003,
    Maiamai004,
    Maiamai005,
    Maiamai006,
    Maiamai007,
    Maiamai008,
    Maiamai009,
    Maiamai010,
    Maiamai011,
    Maiamai012,
    Maiamai013,
    Maiamai014,
    Maiamai015,
    Maiamai016,
    Maiamai017,
    Maiamai018,
    Maiamai019,
    Maiamai020,
    Maiamai021,
    Maiamai022,
    Maiamai023,
    Maiamai024,
    Maiamai025,
    Maiamai026,
    Maiamai027,
    Maiamai028,
    Maiamai029,
    Maiamai030,
    Maiamai031,
    Maiamai032,
    Maiamai033,
    Maiamai034,
    Maiamai035,
    Maiamai036,
    Maiamai037,
    Maiamai038,
    Maiamai039,
    Maiamai040,
    Maiamai041,
    Maiamai042,
    Maiamai043,
    Maiamai044,
    Maiamai045,
    Maiamai046,
    Maiamai047,
    Maiamai048,
    Maiamai049,
    Maiamai050,
    Maiamai051,
    Maiamai052,
    Maiamai053,
    Maiamai054,
    Maiamai055,
    Maiamai056,
    Maiamai057,
    Maiamai058,
    Maiamai059,
    Maiamai060,
    Maiamai061,
    Maiamai062,
    Maiamai063,
    Maiamai064,
    Maiamai065,
    Maiamai066,
    Maiamai067,
    Maiamai068,
    Maiamai069,
    Maiamai070,
    Maiamai071,
    Maiamai072,
    Maiamai073,
    Maiamai074,
    Maiamai075,
    Maiamai076,
    Maiamai077,
    Maiamai078,
    Maiamai079,
    Maiamai080,
    Maiamai081,
    Maiamai082,
    Maiamai083,
    Maiamai084,
    Maiamai085,
    Maiamai086,
    Maiamai087,
    Maiamai088,
    Maiamai089,
    Maiamai090,
    Maiamai091,
    Maiamai092,
    Maiamai093,
    Maiamai094,
    Maiamai095,
    Maiamai096,
    Maiamai097,
    Maiamai098,
    Maiamai099,
    Maiamai100,

    MonsterGuts,
    MonsterHorn,
    MonsterTail,

    // 28 Heart Pieces
    HeartPiece01,
    HeartPiece02,
    HeartPiece03,
    HeartPiece04,
    HeartPiece05,
    HeartPiece06,
    HeartPiece07,
    HeartPiece08,
    HeartPiece09,
    HeartPiece10,
    HeartPiece11,
    HeartPiece12,
    HeartPiece13,
    HeartPiece14,
    HeartPiece15,
    HeartPiece16,
    HeartPiece17,
    HeartPiece18,
    HeartPiece19,
    HeartPiece20,
    HeartPiece21,
    HeartPiece22,
    HeartPiece23,
    HeartPiece24,
    HeartPiece25,
    HeartPiece26,
    HeartPiece27,
    HeartPiece28,

    // 10 Heart Containers
    HeartContainer01,
    HeartContainer02,
    HeartContainer03,
    HeartContainer04,
    HeartContainer05,
    HeartContainer06,
    HeartContainer07,
    HeartContainer08,
    HeartContainer09,
    HeartContainer10,

    // 5 Bottles
    Bottle01,
    Bottle02,
    Bottle03,
    Bottle04,
    Bottle05,

    // 2 Lamps
    Lamp01,
    Lamp02,

    // 4 Swords (Adventures!)
    Sword01,
    Sword02,
    Sword03,
    Sword04,
    //Sword05,

    // 2 Gloves
    Glove01,
    Glove02,

    // 2 Nets
    Net01,
    Net02,

    // 2 Mails
    Mail01,
    Mail02,

    // 4 Master Ore
    OreYellow,
    OreGreen,
    OreBlue,
    OreRed,

    // Sanctuary Keys
    HyruleSanctuaryKey,
    LoruleSanctuaryKey,

    // Eastern Palace
    EasternCompass,
    EasternKeyBig,
    EasternKeySmall01,
    EasternKeySmall02,

    // House of Gales
    GalesCompass,
    GalesKeyBig,
    GalesKeySmall01,
    GalesKeySmall02,
    GalesKeySmall03,
    GalesKeySmall04,

    // Tower of Hera
    HeraCompass,
    HeraKeyBig,
    HeraKeySmall01,
    HeraKeySmall02,

    // Dark Palace
    DarkCompass,
    DarkKeyBig,
    DarkKeySmall01,
    DarkKeySmall02,
    DarkKeySmall03,
    DarkKeySmall04,

    // Swamp Palace
    SwampCompass,
    SwampKeyBig,
    SwampKeySmall01,
    SwampKeySmall02,
    SwampKeySmall03,
    SwampKeySmall04,

    // Skull Woods
    SkullCompass,
    SkullKeyBig,
    SkullKeySmall01,
    SkullKeySmall02,
    SkullKeySmall03,

    // Thieves' Hideout
    ThievesCompass,
    ThievesKeyBig,
    ThievesKeySmall,

    // Ice Ruins
    IceCompass,
    IceKeyBig,
    IceKeySmall01,
    IceKeySmall02,
    IceKeySmall03,

    // Desert Palace
    DesertCompass,
    DesertKeyBig,
    DesertKeySmall01,
    DesertKeySmall02,
    DesertKeySmall03,
    DesertKeySmall04,
    DesertKeySmall05,

    // Turtle Rock
    TurtleCompass,
    TurtleKeyBig,
    TurtleKeySmall01,
    TurtleKeySmall02,
    TurtleKeySmall03,

    // Lorule Castle
    LoruleCastleCompass,
    LoruleCastleKeySmall01,
    LoruleCastleKeySmall02,
    LoruleCastleKeySmall03,
    LoruleCastleKeySmall04,
    LoruleCastleKeySmall05,

    // Dungeon Prizes
    PendantOfPower,
    PendantOfWisdom,
    PendantOfCourage01,
    PendantOfCourage02,
    SageGulley,
    SageOren,
    SageSeres,
    SageOsfala,
    SageRosso,
    SageIrene,
    SageImpa,

    // Shop Items (treated as Quest Items) ---------------------------------------------------------

    // Kakariko
    ScootFruit01,
    FoulFruit01,
    Shield01,

    // Lakeside
    ScootFruit02,
    FoulFruit02,
    Shield02,

    // Mysterious Man
    GoldBee01,

    // Thieves' Town
    Bee01,
    GoldBee02,
    Fairy01,
    Shield03,

    // Lorule Lakeside
    Bee02,
    GoldBee03,
    Fairy02,
    Shield04,

    // Quest Items ---------------------------------------------------------------------------------

    // Bosses -------
    Yuga,
    Margomill,
    Moldorm,
    ZeldasThrone,
    GemesaurKing,
    Arrghus,
    Knucklemaster,
    Stalblind,
    Grinexx,
    Zaganaga,
    Dharkstare,

    // The rest ------
    OpenSanctuaryDoors,
    ShadyGuyTrigger,
    BigBombFlower,
    StylishWomansHouseOpen,
    WomanRoofMaiamai,
    SkullEyeRight,
    SkullEyeLeft,
    ThievesB1DoorOpen,
    ThievesB2DoorOpen,
    ThievesB3WaterDrained,
    TurtleFlipped,
    TurtleAttacked,
    TurtleWall,
    AccessPotionShop,
    AccessMilkBar,
    #[allow(unused)]
    AccessFairyFountain, // todo add to world graph
    AccessHyruleBlacksmith,
    AccessLoruleCastleField,
    LcBombTrial,
    LcBallTrial,
    LcLampTrial,
    LcHookTrial,
    Triforce,

    // Hyrule Overworld Hint Ghosts (36) -----------------------------------------------------------
    HintGhostLostWoodsMaze1,
    HintGhostLostWoodsMaze2,
    HintGhostLostWoodsMaze3,
    HintGhostLostWoods,
    HintGhostSpectacleRock,
    HintGhostTowerOfHeraOutside,
    HintGhostFloatingIsland,
    HintGhostFireCave,
    HintGhostMoldormCave,
    HintGhostZorasDomain,
    HintGhostFortuneTellerHyrule,
    HintGhostSanctuary,
    HintGhostGraveyardHyrule,
    HintGhostWaterfallCave,
    HintGhostWell,
    HintGhostShadyGuy,
    HintGhostStylishWoman,
    HintGhostBlacksmithCave,
    HintGhostEasternRuinsPegs,
    HintGhostEasternRuinsCave,
    HintGhostEasternRuinsEntrance,
    HintGhostRupeeRushHyrule,
    HintGhostCuccos,
    HintGhostSouthBridge,
    HintGhostSouthernRuins,
    HintGhostHouseOfGalesIsland,
    HintGhostHyruleHotfoot,
    HintGhostLetter,
    HintGhostStreetPassTree,
    HintGhostBlacksmithBehind,
    HintGhostGraveyardLedge,
    HintGhostDesertEast,
    HintGhostDesertCenter,
    HintGhostDesertSouthWest,
    HintGhostHyruleCastleRocks,
    HintGhostWitchsHouse,

    // Lorule Overworld Hint Ghosts (20) -----------------------------------------------------------
    HintGhostSkullWoodsCuccos,
    HintGhostTreacherousTower,
    HintGhostIceRuinsOutside,
    HintGhostLoruleGraveyard,
    HintGhostDarkRuinsNorth,
    HintGhostSkullWoodsSouth,
    HintGhostFortunesChoice,
    HintGhostVeteranThief,
    HintGhostFortuneTellerLorule,
    HintGhostDarkMaze,
    HintGhostRupeeRushLorule,
    HintGhostGreatRupeeFairy,
    HintGhostOctoballDerby,
    HintGhostVacantHouse,
    HintGhostMiseryMireLedge,
    HintGhostSwampPalaceOutsideLeft,
    HintGhostTurtleBullied,
    HintGhostTurtleWall,
    HintGhostTurtleRockOutside,
    HintGhostDarkPalaceOutside,
    HintGhostSwampPalaceOutsideRight,
    HintGhostMiseryMireBridge,
}

impl FillerItem {
    pub(crate) fn get_all_ghosts() -> Vec<Self> {
        vec![
            HintGhostLostWoodsMaze1,
            HintGhostLostWoodsMaze2,
            HintGhostLostWoodsMaze3,
            HintGhostLostWoods,
            HintGhostSpectacleRock,
            HintGhostTowerOfHeraOutside,
            HintGhostFloatingIsland,
            HintGhostFireCave,
            HintGhostMoldormCave,
            HintGhostZorasDomain,
            HintGhostFortuneTellerHyrule,
            HintGhostSanctuary,
            HintGhostGraveyardHyrule,
            HintGhostWaterfallCave,
            HintGhostWell,
            HintGhostShadyGuy,
            HintGhostStylishWoman,
            HintGhostBlacksmithCave,
            HintGhostEasternRuinsPegs,
            HintGhostEasternRuinsCave,
            HintGhostEasternRuinsEntrance,
            HintGhostRupeeRushHyrule,
            HintGhostCuccos,
            HintGhostSouthBridge,
            HintGhostSouthernRuins,
            HintGhostHouseOfGalesIsland,
            HintGhostHyruleHotfoot,
            HintGhostLetter,
            HintGhostStreetPassTree,
            HintGhostBlacksmithBehind,
            HintGhostGraveyardLedge,
            HintGhostDesertEast,
            HintGhostDesertCenter,
            HintGhostDesertSouthWest,
            HintGhostHyruleCastleRocks,
            HintGhostWitchsHouse,
            HintGhostSkullWoodsCuccos,
            HintGhostTreacherousTower,
            HintGhostIceRuinsOutside,
            HintGhostLoruleGraveyard,
            HintGhostDarkRuinsNorth,
            HintGhostSkullWoodsSouth,
            HintGhostFortunesChoice,
            HintGhostVeteranThief,
            HintGhostFortuneTellerLorule,
            HintGhostDarkMaze,
            HintGhostRupeeRushLorule,
            HintGhostGreatRupeeFairy,
            HintGhostOctoballDerby,
            HintGhostVacantHouse,
            HintGhostMiseryMireLedge,
            HintGhostSwampPalaceOutsideLeft,
            HintGhostTurtleBullied,
            HintGhostTurtleWall,
            HintGhostTurtleRockOutside,
            HintGhostDarkPalaceOutside,
            HintGhostSwampPalaceOutsideRight,
            HintGhostMiseryMireBridge,
        ]
    }

    pub fn is_hint_ghost(self) -> bool {
        match self {
            HintGhostLostWoodsMaze1
            | HintGhostLostWoodsMaze2
            | HintGhostLostWoodsMaze3
            | HintGhostLostWoods
            | HintGhostSpectacleRock
            | HintGhostTowerOfHeraOutside
            | HintGhostFloatingIsland
            | HintGhostFireCave
            | HintGhostMoldormCave
            | HintGhostZorasDomain
            | HintGhostFortuneTellerHyrule
            | HintGhostSanctuary
            | HintGhostGraveyardHyrule
            | HintGhostWaterfallCave
            | HintGhostWell
            | HintGhostShadyGuy
            | HintGhostStylishWoman
            | HintGhostBlacksmithCave
            | HintGhostEasternRuinsPegs
            | HintGhostEasternRuinsCave
            | HintGhostEasternRuinsEntrance
            | HintGhostRupeeRushHyrule
            | HintGhostCuccos
            | HintGhostSouthBridge
            | HintGhostSouthernRuins
            | HintGhostHouseOfGalesIsland
            | HintGhostHyruleHotfoot
            | HintGhostLetter
            | HintGhostStreetPassTree
            | HintGhostBlacksmithBehind
            | HintGhostGraveyardLedge
            | HintGhostDesertEast
            | HintGhostDesertCenter
            | HintGhostDesertSouthWest
            | HintGhostHyruleCastleRocks
            | HintGhostWitchsHouse
            | HintGhostSkullWoodsCuccos
            | HintGhostTreacherousTower
            | HintGhostIceRuinsOutside
            | HintGhostLoruleGraveyard
            | HintGhostDarkRuinsNorth
            | HintGhostSkullWoodsSouth
            | HintGhostFortunesChoice
            | HintGhostVeteranThief
            | HintGhostFortuneTellerLorule
            | HintGhostDarkMaze
            | HintGhostRupeeRushLorule
            | HintGhostGreatRupeeFairy
            | HintGhostOctoballDerby
            | HintGhostVacantHouse
            | HintGhostMiseryMireLedge
            | HintGhostSwampPalaceOutsideLeft
            | HintGhostTurtleBullied
            | HintGhostTurtleWall
            | HintGhostTurtleRockOutside
            | HintGhostDarkPalaceOutside
            | HintGhostSwampPalaceOutsideRight
            | HintGhostMiseryMireBridge => true,
            _ => false,
        }
    }

    pub fn is_progression(self) -> bool {
        match self {
            // Empty |
            Bow01 |
            Bow02 |
            Boomerang01 |
            Boomerang02 |
            Hookshot01 |
            Hookshot02 |
            Bombs01 |
            Bombs02 |
            FireRod01 |
            FireRod02 |
            IceRod01 |
            IceRod02 |
            Hammer01 |
            Hammer02 |
            SandRod01 |
            SandRod02 |
            TornadoRod01 |
            TornadoRod02 |
            Bell |
            StaminaScroll |
            BowOfLight |
            PegasusBoots |
            Flippers |
            RaviosBracelet01 |
            RaviosBracelet02 |
            HylianShield |
            SmoothGem |
            LetterInABottle |
            PremiumMilk |
            // FillerItem::Pouch |
            // BeeBadge |
            // FillerItem::HintGlasses |
            GreatSpin |
            // RupeeGreen |
            // RupeeBlue |
            // RupeeRed |
            // RupeePurple01 |
            // RupeePurple02 |
            // RupeePurple03 |
            // RupeePurple04 |
            // RupeePurple05 |
            // RupeePurple06 |
            // RupeePurple07 |
            // RupeePurple08 |
            // RupeePurple09 |
            // RupeePurple10 |
            // RupeePurple11 |
            // RupeePurple12 |
            // RupeePurple13 |
            // RupeePurple14 |
            // RupeePurple15 |
            // RupeePurple16 |
            // RupeePurple17 |
            // RupeePurple18 |
            // RupeePurple19 |
            // RupeePurple20 |
            // RupeeSilver01 |
            // RupeeSilver02 |
            // RupeeSilver03 |
            // RupeeSilver04 |
            // RupeeSilver05 |
            // RupeeSilver06 |
            // RupeeSilver07 |
            // RupeeSilver08 |
            // RupeeSilver09 |
            // RupeeSilver10 |
            // RupeeSilver11 |
            // RupeeSilver12 |
            // RupeeSilver13 |
            // RupeeSilver14 |
            // RupeeSilver15 |
            // RupeeSilver16 |
            // RupeeSilver17 |
            // RupeeSilver18 |
            // RupeeSilver19 |
            // RupeeSilver20 |
            // RupeeSilver21 |
            // RupeeSilver22 |
            // RupeeSilver23 |
            // RupeeSilver24 |
            // RupeeSilver25 |
            // RupeeSilver26 |
            // RupeeSilver27 |
            // RupeeSilver28 |
            // RupeeSilver29 |
            // RupeeSilver30 |
            // RupeeSilver31 |
            // RupeeSilver32 |
            // RupeeSilver33 |
            // RupeeSilver34 |
            // RupeeSilver35 |
            // RupeeSilver36 |
            // RupeeSilver37 |
            // RupeeSilver38 |
            // RupeeSilver39 |
            // RupeeSilver40 |
            // RupeeSilver41 |
            // RupeeGold01 |
            // RupeeGold02 |
            // RupeeGold03 |
            // RupeeGold04 |
            // RupeeGold05 |
            // RupeeGold06 |
            // RupeeGold07 |
            // RupeeGold08 |
            // RupeeGold09 |
            // RupeeGold10 |
            // Maiamai001 |
            // Maiamai002 |
            // Maiamai003 |
            // Maiamai004 |
            // Maiamai005 |
            // Maiamai006 |
            // Maiamai007 |
            // Maiamai008 |
            // Maiamai009 |
            // Maiamai010 |
            // Maiamai011 |
            // Maiamai012 |
            // Maiamai013 |
            // Maiamai014 |
            // Maiamai015 |
            // Maiamai016 |
            // Maiamai017 |
            // Maiamai018 |
            // Maiamai019 |
            // Maiamai020 |
            // Maiamai021 |
            // Maiamai022 |
            // Maiamai023 |
            // Maiamai024 |
            // Maiamai025 |
            // Maiamai026 |
            // Maiamai027 |
            // Maiamai028 |
            // Maiamai029 |
            // Maiamai030 |
            // Maiamai031 |
            // Maiamai032 |
            // Maiamai033 |
            // Maiamai034 |
            // Maiamai035 |
            // Maiamai036 |
            // Maiamai037 |
            // Maiamai038 |
            // Maiamai039 |
            // Maiamai040 |
            // Maiamai041 |
            // Maiamai042 |
            // Maiamai043 |
            // Maiamai044 |
            // Maiamai045 |
            // Maiamai046 |
            // Maiamai047 |
            // Maiamai048 |
            // Maiamai049 |
            // Maiamai050 |
            // Maiamai051 |
            // Maiamai052 |
            // Maiamai053 |
            // Maiamai054 |
            // Maiamai055 |
            // Maiamai056 |
            // Maiamai057 |
            // Maiamai058 |
            // Maiamai059 |
            // Maiamai060 |
            // Maiamai061 |
            // Maiamai062 |
            // Maiamai063 |
            // Maiamai064 |
            // Maiamai065 |
            // Maiamai066 |
            // Maiamai067 |
            // Maiamai068 |
            // Maiamai069 |
            // Maiamai070 |
            // Maiamai071 |
            // Maiamai072 |
            // Maiamai073 |
            // Maiamai074 |
            // Maiamai075 |
            // Maiamai076 |
            // Maiamai077 |
            // Maiamai078 |
            // Maiamai079 |
            // Maiamai080 |
            // Maiamai081 |
            // Maiamai082 |
            // Maiamai083 |
            // Maiamai084 |
            // Maiamai085 |
            // Maiamai086 |
            // Maiamai087 |
            // Maiamai088 |
            // Maiamai089 |
            // Maiamai090 |
            // Maiamai091 |
            // Maiamai092 |
            // Maiamai093 |
            // Maiamai094 |
            // Maiamai095 |
            // Maiamai096 |
            // Maiamai097 |
            // Maiamai098 |
            // Maiamai099 |
            // Maiamai100 |
            // MonsterGuts |
            // MonsterHorn |
            // MonsterTail |
            // HeartPiece01 |
            // HeartPiece02 |
            // HeartPiece03 |
            // HeartPiece04 |
            // HeartPiece05 |
            // HeartPiece06 |
            // HeartPiece07 |
            // HeartPiece08 |
            // HeartPiece09 |
            // HeartPiece10 |
            // HeartPiece11 |
            // HeartPiece12 |
            // HeartPiece13 |
            // HeartPiece14 |
            // HeartPiece15 |
            // HeartPiece16 |
            // HeartPiece17 |
            // HeartPiece18 |
            // HeartPiece19 |
            // HeartPiece20 |
            // HeartPiece21 |
            // HeartPiece22 |
            // HeartPiece23 |
            // HeartPiece24 |
            // HeartPiece25 |
            // HeartPiece26 |
            // HeartPiece27 |
            // HeartPiece28 |
            // HeartContainer01 |
            // HeartContainer02 |
            // HeartContainer03 |
            // HeartContainer04 |
            // HeartContainer05 |
            // HeartContainer06 |
            // HeartContainer07 |
            // HeartContainer08 |
            // HeartContainer09 |
            // HeartContainer10 |
            Bottle01 |
            Bottle02 |
            Bottle03 |
            Bottle04 |
            Bottle05 |
            Lamp01 |
            Lamp02 |
            Sword01 |
            Sword02 |
            Sword03 |
            Sword04 |
            Glove01 |
            Glove02 |
            Net01 |
            Net02 |
            Mail01 |
            Mail02 |
            FillerItem::OreYellow |
            FillerItem::OreGreen |
            FillerItem::OreBlue |
            FillerItem::OreRed |
            HyruleSanctuaryKey |
            LoruleSanctuaryKey |
            // EasternCompass |
            EasternKeyBig |
            EasternKeySmall01 |
            EasternKeySmall02 |
            // GalesCompass |
            GalesKeyBig |
            GalesKeySmall01 |
            GalesKeySmall02 |
            GalesKeySmall03 |
            GalesKeySmall04 |
            // HeraCompass |
            HeraKeyBig |
            HeraKeySmall01 |
            HeraKeySmall02 |
            // DarkCompass |
            DarkKeyBig |
            DarkKeySmall01 |
            DarkKeySmall02 |
            DarkKeySmall03 |
            DarkKeySmall04 |
            // SwampCompass |
            SwampKeyBig |
            SwampKeySmall01 |
            SwampKeySmall02 |
            SwampKeySmall03 |
            SwampKeySmall04 |
            // SkullCompass |
            SkullKeyBig |
            SkullKeySmall01 |
            SkullKeySmall02 |
            SkullKeySmall03 |
            // ThievesCompass |
            ThievesKeyBig |
            ThievesKeySmall |
            // IceCompass |
            IceKeyBig |
            IceKeySmall01 |
            IceKeySmall02 |
            IceKeySmall03 |
            // DesertCompass |
            DesertKeyBig |
            DesertKeySmall01 |
            DesertKeySmall02 |
            DesertKeySmall03 |
            DesertKeySmall04 |
            DesertKeySmall05 |
            // TurtleCompass |
            TurtleKeyBig |
            TurtleKeySmall01 |
            TurtleKeySmall02 |
            TurtleKeySmall03 |
            // LoruleCastleCompass |
            LoruleCastleKeySmall01 |
            LoruleCastleKeySmall02 |
            LoruleCastleKeySmall03 |
            LoruleCastleKeySmall04 |
            LoruleCastleKeySmall05 |
            PendantOfPower |
            PendantOfWisdom |
            PendantOfCourage01 |
            PendantOfCourage02 |
            FillerItem::SageGulley |
            FillerItem::SageOren |
            FillerItem::SageSeres |
            FillerItem::SageOsfala |
            FillerItem::SageRosso |
            FillerItem::SageIrene |
            FillerItem::SageImpa |
            ScootFruit01 |
            // FoulFruit01 |
            // Shield01 |
            ScootFruit02 |
            // FoulFruit02 |
            // Shield02 |
            GoldBee01 |
            // Bee01 |
            // GoldBee02 |
            // Fairy01 |
            // Shield03 |
            // Bee02 |
            // GoldBee03 |
            // Fairy02 |
            // Shield04 |
            // EasternComplete |
            // DarkComplete |
            // ThievesComplete |
            // OpenSanctuaryDoors |
            // ShadyGuyTrigger |
            // BigBombFlower |
            // StylishWomansHouseOpen |
            // WomanRoofMaiamai |
            // SkullEyeRight |
            // SkullEyeLeft |
            // ThievesB1DoorOpen |
            // ThievesB2DoorOpen |
            // ThievesB3WaterDrained |
            // TurtleFlipped |
            // TurtleAttacked |
            // TurtleWall |
            // AccessPotionShop |
            // AccessMilkBar |
            // AccessFairyFountain |
            // AccessHyruleBlacksmith |
            // AccessLoruleCastleField |
            // LcBombTrial |
            // LcBallTrial |
            // LcLampTrial |
            // LcHookTrial |
            Triforce => true,
            _ => false
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            HyruleSanctuaryKey => "Hyrule Sanctuary Small Key",
            LoruleSanctuaryKey => "Lorule Sanctuary Small Key",
            EasternCompass => "Eastern Palace Compass",
            EasternKeyBig => "Eastern Palace Big Key",
            EasternKeySmall01 | EasternKeySmall02 => "Eastern Palace Small Key",
            GalesCompass => "House of Gales Compass",
            GalesKeyBig => "House of Gales Big Key",
            GalesKeySmall01 | GalesKeySmall02 | GalesKeySmall03 | GalesKeySmall04 => {
                "House of Gales Small Key"
            }
            HeraCompass => "Tower of Hera Compass",
            HeraKeyBig => "Tower of Hera Big Key",
            HeraKeySmall01 | HeraKeySmall02 => "Tower of Hera Small Key",
            DarkCompass => "Dark Palace Compass",
            DarkKeyBig => "Dark Palace Big Key",
            DarkKeySmall01 | DarkKeySmall02 | DarkKeySmall03 | DarkKeySmall04 => {
                "Dark Palace Small Key"
            }
            SwampCompass => "Swamp Palace Compass",
            SwampKeyBig => "Swamp Palace Big Key",
            SwampKeySmall01 | SwampKeySmall02 | SwampKeySmall03 | SwampKeySmall04 => {
                "Swamp Palace Small Key"
            }
            SkullCompass => "Skull Woods Compass",
            SkullKeyBig => "Skull Woods Big Key",
            SkullKeySmall01 | SkullKeySmall02 | SkullKeySmall03 => "Skull Woods Small Key",
            ThievesCompass => "Thieves' Hideout Compass",
            ThievesKeyBig => "Thieves' Hideout Big Key",
            ThievesKeySmall => "Thieves' Hideout Small Key",
            IceCompass => "Ice Ruins Compass",
            IceKeyBig => "Ice Ruins Big Key",
            IceKeySmall01 | IceKeySmall02 | IceKeySmall03 => "Ice Ruins Small Key",
            DesertCompass => "Desert Palace Compass",
            DesertKeyBig => "Desert Palace Big Key",
            DesertKeySmall01 | DesertKeySmall02 | DesertKeySmall03 | DesertKeySmall04
            | DesertKeySmall05 => "Desert Palace Small Key",
            TurtleCompass => "Turtle Rock Compass",
            TurtleKeyBig => "Turtle Rock Big Key",
            TurtleKeySmall01 | TurtleKeySmall02 | TurtleKeySmall03 => "Turtle Rock Small Key",
            LoruleCastleCompass => "Lorule Castle Compass",
            LoruleCastleKeySmall01
            | LoruleCastleKeySmall02
            | LoruleCastleKeySmall03
            | LoruleCastleKeySmall04
            | LoruleCastleKeySmall05 => "Lorule Castle Small Key",

            Yuga => "Yuga",
            Margomill => "Margomill",
            Moldorm => "Moldorm",
            ZeldasThrone => "Zelda's Throne",

            GemesaurKing => "Gemesaur King",
            Arrghus => "Arrghus",
            Knucklemaster => "Knucklemaster",
            Stalblind => "Stalblind",
            Grinexx => "Grinexx",
            Zaganaga => "Zaganaga",
            Dharkstare => "Dharkstare",

            OpenSanctuaryDoors => "Sanctuary Doors Opened",
            ShadyGuyTrigger => "Shady Guy Trigger",
            BigBombFlower => "Big Bomb Flower",
            StylishWomansHouseOpen => "Stylish Woman's House Opened",
            WomanRoofMaiamai => "Woman's Roof Maiamai",
            SkullEyeRight => "Skull Woods Right Eye",
            SkullEyeLeft => "Skull Woods Left Eye",
            ThievesB1DoorOpen => "Thieves' Hideout B1 Door Open",
            ThievesB2DoorOpen => "Thieves' Hideout B2 Door Open",
            ThievesB3WaterDrained => "Thieves' Hideout B3 Water Drained",
            TurtleFlipped => "Turtle Flipped",
            TurtleAttacked => "Turtle Bullied",
            TurtleWall => "Turtle Wall",
            AccessPotionShop => "Potion Shop Access",
            AccessMilkBar => "Milk Bar Access",
            AccessFairyFountain => "Fairy Fountain Access",
            AccessHyruleBlacksmith => "Hyrule Blacksmith Access",
            AccessLoruleCastleField => "Lorule Castle Field Access",
            LcBombTrial => "Bomb Trial Complete",
            LcBallTrial => "Ball Trial Complete",
            LcLampTrial => "Lamp Trial Complete",
            LcHookTrial => "Hook Trial Complete",
            Triforce => "Triforce",

            HintGhostLostWoodsMaze1 => "Lost Woods Maze Ghost 1",
            HintGhostLostWoodsMaze2 => "Lost Woods Maze Ghost 2",
            HintGhostLostWoodsMaze3 => "Lost Woods Maze Ghost 3",
            HintGhostLostWoods => "Lost Woods Ghost",
            HintGhostSpectacleRock => "Spectacle Rock Ghost",
            HintGhostTowerOfHeraOutside => "Outside Tower of Hera Ghost",
            HintGhostFloatingIsland => "Floating Island Ghost",
            HintGhostFireCave => "Fire Cave Ghost",
            HintGhostMoldormCave => "Moldorm Cave Ghost",
            HintGhostZorasDomain => "Zora's Domain Ghost",
            HintGhostFortuneTellerHyrule => "Hyrule Fortune-Teller Ghost",
            HintGhostSanctuary => "Sanctuary Ghost",
            HintGhostGraveyardHyrule => "Hyrule Graveyard Ghost",
            HintGhostWaterfallCave => "Waterfall Cave Ghost",
            HintGhostWell => "Kakariko Well Ghost",
            HintGhostShadyGuy => "Shady Guy Ghost",
            HintGhostStylishWoman => "Stylish Woman Ghost",
            HintGhostBlacksmithCave => "Blacksmith Cave Ghost",
            HintGhostEasternRuinsPegs => "Eastern Ruins Pegs Ghost",
            HintGhostEasternRuinsCave => "Eastern Ruins Cave Ghost",
            HintGhostEasternRuinsEntrance => "Eastern Ruins Entrance Ghost",
            HintGhostRupeeRushHyrule => "Hyrule Rupee Rush Ghost",
            HintGhostCuccos => "Dodge the Cuccos Ghost",
            HintGhostSouthBridge => "Southern Bridge Ghost",
            HintGhostSouthernRuins => "Southern Ruins Ghost",
            HintGhostHouseOfGalesIsland => "House of Gales Island Ghost",
            HintGhostHyruleHotfoot => "Hyrule Hotfoot Ghost",
            HintGhostLetter => "Letter in a Bottle Ghost",
            HintGhostStreetPassTree => "StreetPass Tree Ghost",
            HintGhostBlacksmithBehind => "Behind Blacksmith Ghost",
            HintGhostGraveyardLedge => "Graveyard Ledge Ghost",
            HintGhostDesertEast => "Desert East Ghost",
            HintGhostDesertCenter => "Desert Center Ghost",
            HintGhostDesertSouthWest => "Desert South West Ghost",
            HintGhostHyruleCastleRocks => "Hyrule Castle Rocks Ghost",
            HintGhostWitchsHouse => "Witch's House Ghost",

            HintGhostSkullWoodsCuccos => "Skull Woods Cuccos Ghost",
            HintGhostTreacherousTower => "Treacherous Tower Ghost",
            HintGhostIceRuinsOutside => "Ice Ruins Outside Ghost",
            HintGhostLoruleGraveyard => "Lorule Graveyard Ghost",
            HintGhostDarkRuinsNorth => "Dark Ruins North Ghost",
            HintGhostSkullWoodsSouth => "Skull Woods South Ghost",
            HintGhostFortunesChoice => "Fortune's Choice Ghost",
            HintGhostVeteranThief => "Veteran Thief Ghost",
            HintGhostFortuneTellerLorule => "Lorule Fortune-Teller Ghost",
            HintGhostDarkMaze => "Dark Maze Ghost",
            HintGhostRupeeRushLorule => "Lorule Rupee Rush Ghost",
            HintGhostGreatRupeeFairy => "Great Rupee Fairy Ghost",
            HintGhostOctoballDerby => "Octoball Derby Ghost",
            HintGhostVacantHouse => "Vacant House Ghost",
            HintGhostMiseryMireLedge => "Misery Mire Ledge Ghost",
            HintGhostSwampPalaceOutsideLeft => "Swamp Palace Outside Left Ghost",
            HintGhostTurtleBullied => "Turtle Bullied Ghost",
            HintGhostTurtleWall => "Turtle Wall Ghost",
            HintGhostTurtleRockOutside => "Turtle Rock Outside Ghost",
            HintGhostDarkPalaceOutside => "Dark Palace Outside Ghost",
            HintGhostSwampPalaceOutsideRight => "Swamp Palace Outside Right Ghost",
            HintGhostMiseryMireBridge => "Misery Mire Bridge Ghost",

            _ => item_to_str(&(convert(self).unwrap())),
        }
    }

    pub fn as_str_colorized(&self) -> String {
        match self {
            Yuga => Green,
            Margomill => Blue,
            Moldorm => Attention,
            GemesaurKing => Green,
            Arrghus => Beige,
            Knucklemaster => Blue,
            Stalblind => Beige,
            Grinexx => Purple,
            Zaganaga => Name,
            Dharkstare => Attention,
            _ => Name,
        }
        .format(Self::as_str(*self))
    }
}

impl Serialize for FillerItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

pub fn convert(fill_item: FillerItem) -> Option<Item> {
    match fill_item {
        FillerItem::Empty => Some(Item::Empty),
        Bow01 | Bow02 => Some(ItemBow),
        Boomerang01 | Boomerang02 => Some(ItemBoomerang),
        Hookshot01 | Hookshot02 => Some(ItemHookShot),
        Bombs01 | Bombs02 => Some(ItemBomb),
        FireRod01 | FireRod02 => Some(ItemFireRod),
        IceRod01 | IceRod02 => Some(ItemIceRod),
        Hammer01 | Hammer02 => Some(ItemHammer),
        Bell => Some(ItemBell),
        StaminaScroll => Some(GanbariPowerUp),
        SandRod01 | SandRod02 => Some(ItemSandRod),
        TornadoRod01 | TornadoRod02 => Some(ItemTornadeRod),
        BowOfLight => Some(ItemBowLight),
        PegasusBoots => Some(DashBoots),
        Flippers => Some(ItemMizukaki),
        RaviosBracelet01 => Some(RingRental),
        RaviosBracelet02 => Some(RingRental),
        HylianShield => Some(HyruleShield),
        SmoothGem => Some(ItemStoneBeauty),
        LetterInABottle => Some(MessageBottle),
        PremiumMilk => Some(MilkMatured),
        FillerItem::Pouch => Some(Item::Pouch),
        BeeBadge => Some(BadgeBee),
        FillerItem::HintGlasses => Some(Item::HintGlasses),

        HeartPiece01 | HeartPiece02 | HeartPiece03 | HeartPiece04 | HeartPiece05 | HeartPiece06
        | HeartPiece07 | HeartPiece08 | HeartPiece09 | HeartPiece10 | HeartPiece11
        | HeartPiece12 | HeartPiece13 | HeartPiece14 | HeartPiece15 | HeartPiece16
        | HeartPiece17 | HeartPiece18 | HeartPiece19 | HeartPiece20 | HeartPiece21
        | HeartPiece22 | HeartPiece23 | HeartPiece24 | HeartPiece25 | HeartPiece26
        | HeartPiece27 | HeartPiece28 => Some(HeartPiece),

        HeartContainer01 | HeartContainer02 | HeartContainer03 | HeartContainer04
        | HeartContainer05 | HeartContainer06 | HeartContainer07 | HeartContainer08
        | HeartContainer09 | HeartContainer10 => Some(HeartContainer),

        Bottle01 | Bottle02 | Bottle03 | Bottle04 | Bottle05 => Some(ItemBottle),

        Lamp01 | Lamp02 => Some(ItemKandelaar),

        Sword01 | Sword02 | Sword03 | Sword04 => Some(ItemSwordLv1),

        Glove01 | Glove02 => Some(PowerGlove),

        Net01 | Net02 => Some(ItemInsectNet),

        Mail01 | Mail02 => Some(ClothesBlue),

        FillerItem::OreYellow => Some(Item::OreYellow),
        FillerItem::OreGreen => Some(Item::OreGreen),
        FillerItem::OreBlue => Some(Item::OreBlue),
        FillerItem::OreRed => Some(Item::OreRed),

        // Small Keys
        HyruleSanctuaryKey
        | LoruleSanctuaryKey
        | EasternKeySmall01
        | EasternKeySmall02
        | GalesKeySmall01
        | GalesKeySmall02
        | GalesKeySmall03
        | GalesKeySmall04
        | HeraKeySmall01
        | HeraKeySmall02
        | DarkKeySmall01
        | DarkKeySmall02
        | DarkKeySmall03
        | DarkKeySmall04
        | SwampKeySmall01
        | SwampKeySmall02
        | SwampKeySmall03
        | SwampKeySmall04
        | SkullKeySmall01
        | SkullKeySmall02
        | SkullKeySmall03
        | ThievesKeySmall
        | IceKeySmall01
        | IceKeySmall02
        | IceKeySmall03
        | DesertKeySmall01
        | DesertKeySmall02
        | DesertKeySmall03
        | DesertKeySmall04
        | DesertKeySmall05
        | TurtleKeySmall01
        | TurtleKeySmall02
        | TurtleKeySmall03
        | LoruleCastleKeySmall01
        | LoruleCastleKeySmall02
        | LoruleCastleKeySmall03
        | LoruleCastleKeySmall04
        | LoruleCastleKeySmall05 => Some(KeySmall),

        // Big Keys
        EasternKeyBig | GalesKeyBig | HeraKeyBig | DarkKeyBig | SwampKeyBig | SkullKeyBig
        | ThievesKeyBig | IceKeyBig | DesertKeyBig | TurtleKeyBig => Some(KeyBoss),

        // Compasses
        EasternCompass | GalesCompass | HeraCompass | DarkCompass | SwampCompass | SkullCompass
        | ThievesCompass | IceCompass | DesertCompass | TurtleCompass | LoruleCastleCompass => {
            Some(Compass)
        }

        GreatSpin => Some(SpecialMove),
        RupeeGreen => Some(RupeeG),
        RupeeBlue => Some(RupeeB),
        RupeeRed => Some(RupeeR),

        RupeePurple01 | RupeePurple02 | RupeePurple03 | RupeePurple04 | RupeePurple05
        | RupeePurple06 | RupeePurple07 | RupeePurple08 | RupeePurple09 | RupeePurple10
        | RupeePurple11 | RupeePurple12 | RupeePurple13 | RupeePurple14 | RupeePurple15
        | RupeePurple16 | RupeePurple17 | RupeePurple18 | RupeePurple19 | RupeePurple20 => {
            Some(RupeePurple)
        }

        RupeeSilver01 | RupeeSilver02 | RupeeSilver03 | RupeeSilver04 | RupeeSilver05
        | RupeeSilver06 | RupeeSilver07 | RupeeSilver08 | RupeeSilver09 | RupeeSilver10
        | RupeeSilver11 | RupeeSilver12 | RupeeSilver13 | RupeeSilver14 | RupeeSilver15
        | RupeeSilver16 | RupeeSilver17 | RupeeSilver18 | RupeeSilver19 | RupeeSilver20
        | RupeeSilver21 | RupeeSilver22 | RupeeSilver23 | RupeeSilver24 | RupeeSilver25
        | RupeeSilver26 | RupeeSilver27 | RupeeSilver28 | RupeeSilver29 | RupeeSilver30
        | RupeeSilver31 | RupeeSilver32 | RupeeSilver33 | RupeeSilver34 | RupeeSilver35
        | RupeeSilver36 | RupeeSilver37 | RupeeSilver38 | RupeeSilver39 | RupeeSilver40
        | RupeeSilver41 => Some(RupeeSilver),

        RupeeGold01 | RupeeGold02 | RupeeGold03 | RupeeGold04 | RupeeGold05 | RupeeGold06
        | RupeeGold07 | RupeeGold08 | RupeeGold09 | RupeeGold10 => Some(RupeeGold),

        Maiamai001 | Maiamai002 | Maiamai003 | Maiamai004 | Maiamai005 | Maiamai006
        | Maiamai007 | Maiamai008 | Maiamai009 | Maiamai010 | Maiamai011 | Maiamai012
        | Maiamai013 | Maiamai014 | Maiamai015 | Maiamai016 | Maiamai017 | Maiamai018
        | Maiamai019 | Maiamai020 | Maiamai021 | Maiamai022 | Maiamai023 | Maiamai024
        | Maiamai025 | Maiamai026 | Maiamai027 | Maiamai028 | Maiamai029 | Maiamai030
        | Maiamai031 | Maiamai032 | Maiamai033 | Maiamai034 | Maiamai035 | Maiamai036
        | Maiamai037 | Maiamai038 | Maiamai039 | Maiamai040 | Maiamai041 | Maiamai042
        | Maiamai043 | Maiamai044 | Maiamai045 | Maiamai046 | Maiamai047 | Maiamai048
        | Maiamai049 | Maiamai050 | Maiamai051 | Maiamai052 | Maiamai053 | Maiamai054
        | Maiamai055 | Maiamai056 | Maiamai057 | Maiamai058 | Maiamai059 | Maiamai060
        | Maiamai061 | Maiamai062 | Maiamai063 | Maiamai064 | Maiamai065 | Maiamai066
        | Maiamai067 | Maiamai068 | Maiamai069 | Maiamai070 | Maiamai071 | Maiamai072
        | Maiamai073 | Maiamai074 | Maiamai075 | Maiamai076 | Maiamai077 | Maiamai078
        | Maiamai079 | Maiamai080 | Maiamai081 | Maiamai082 | Maiamai083 | Maiamai084
        | Maiamai085 | Maiamai086 | Maiamai087 | Maiamai088 | Maiamai089 | Maiamai090
        | Maiamai091 | Maiamai092 | Maiamai093 | Maiamai094 | Maiamai095 | Maiamai096
        | Maiamai097 | Maiamai098 | Maiamai099 | Maiamai100 => Some(Kinsta),

        MonsterGuts => Some(LiverPurple),
        MonsterHorn => Some(LiverYellow),
        MonsterTail => Some(LiverBlue),

        // Dungeon Items
        PendantOfPower => Some(PendantPower),
        PendantOfWisdom => Some(PendantWisdom),
        PendantOfCourage01 | PendantOfCourage02 => Some(PendantCourage),
        FillerItem::SageGulley => Some(Item::SageGulley),
        FillerItem::SageOren => Some(Item::SageOren),
        FillerItem::SageSeres => Some(Item::SageSeres),
        FillerItem::SageOsfala => Some(Item::SageOsfala),
        FillerItem::SageImpa => Some(Item::SageImpa),
        FillerItem::SageIrene => Some(Item::SageIrene),
        FillerItem::SageRosso => Some(Item::SageRosso),

        // Shop Items
        ScootFruit01 | ScootFruit02 => Some(EscapeFruit),
        FoulFruit01 | FoulFruit02 => Some(StopFruit),
        Shield01 | Shield02 | Shield03 | Shield04 => Some(ItemShield),
        Bee01 | Bee02 => Some(Bee),
        GoldBee01 | GoldBee02 | GoldBee03 => Some(GoldenBeeForSale),
        Fairy01 | Fairy02 => Some(Fairy),

        // Quest Items don't translate
        Yuga
        | Margomill
        | Moldorm
        | ZeldasThrone
        | GemesaurKing
        | Arrghus
        | Knucklemaster
        | Stalblind
        | Grinexx
        | Zaganaga
        | Dharkstare
        | ShadyGuyTrigger
        | OpenSanctuaryDoors
        | BigBombFlower
        | StylishWomansHouseOpen
        | WomanRoofMaiamai
        | SkullEyeRight
        | SkullEyeLeft
        | ThievesB1DoorOpen
        | ThievesB2DoorOpen
        | ThievesB3WaterDrained
        | TurtleFlipped
        | TurtleAttacked
        | TurtleWall
        | AccessLoruleCastleField
        | AccessHyruleBlacksmith
        | AccessPotionShop
        | AccessFairyFountain
        | AccessMilkBar
        | LcBombTrial
        | LcBallTrial
        | LcLampTrial
        | LcHookTrial
        | Triforce => None,

        // Hint Ghosts don't map either
        HintGhostLostWoodsMaze1
        | HintGhostLostWoodsMaze2
        | HintGhostLostWoodsMaze3
        | HintGhostLostWoods
        | HintGhostSpectacleRock
        | HintGhostTowerOfHeraOutside
        | HintGhostFloatingIsland
        | HintGhostFireCave
        | HintGhostMoldormCave
        | HintGhostZorasDomain
        | HintGhostFortuneTellerHyrule
        | HintGhostSanctuary
        | HintGhostGraveyardHyrule
        | HintGhostWaterfallCave
        | HintGhostWell
        | HintGhostShadyGuy
        | HintGhostStylishWoman
        | HintGhostBlacksmithCave
        | HintGhostEasternRuinsPegs
        | HintGhostEasternRuinsCave
        | HintGhostEasternRuinsEntrance
        | HintGhostRupeeRushHyrule
        | HintGhostCuccos
        | HintGhostSouthBridge
        | HintGhostSouthernRuins
        | HintGhostHouseOfGalesIsland
        | HintGhostHyruleHotfoot
        | HintGhostLetter
        | HintGhostStreetPassTree
        | HintGhostBlacksmithBehind
        | HintGhostGraveyardLedge
        | HintGhostDesertEast
        | HintGhostDesertCenter
        | HintGhostDesertSouthWest
        | HintGhostHyruleCastleRocks
        | HintGhostWitchsHouse
        | HintGhostSkullWoodsCuccos
        | HintGhostTreacherousTower
        | HintGhostIceRuinsOutside
        | HintGhostLoruleGraveyard
        | HintGhostDarkRuinsNorth
        | HintGhostSkullWoodsSouth
        | HintGhostFortunesChoice
        | HintGhostVeteranThief
        | HintGhostFortuneTellerLorule
        | HintGhostDarkMaze
        | HintGhostRupeeRushLorule
        | HintGhostGreatRupeeFairy
        | HintGhostOctoballDerby
        | HintGhostVacantHouse
        | HintGhostMiseryMireLedge
        | HintGhostSwampPalaceOutsideLeft
        | HintGhostTurtleBullied
        | HintGhostTurtleWall
        | HintGhostTurtleRockOutside
        | HintGhostDarkPalaceOutside
        | HintGhostSwampPalaceOutsideRight
        | HintGhostMiseryMireBridge => None,
    }
}

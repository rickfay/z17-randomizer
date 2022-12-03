use albw::Item;
use albw::Item::*;
use FillerItem::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    //GreatSpin,

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
    //RupeeSilver41,

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
    //HeartPiece28,

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

    // Quest Items -----------------------------------------------------------------------------

    PendantOfCourage,
    PendantOfWisdom,
    PendantOfPower,
    SageGulley,
    SageOren,
    SageSeres,
    SageOsfala,
    SageRosso,
    SageIrene,
    SageImpa,
    EasternComplete,
    DarkComplete,
    ThievesComplete,
    ScootFruit,
    FoulFruit,
    Shield,
    GoldBee,
    OpenSanctuaryDoors,
    ShadyGuyTrigger,
    BigBombFlower,
    StylishWomansHouseOpen,
    SkullEyeRight,
    SkullEyeLeft,
    TurtleFlipped,
    TurtleAttacked,
    TurtleWall,
    AccessPotionShop,
    AccessMilkBar,
    AccessHyruleBlacksmith,
    AccessLoruleCastleField,
    LcBombTrial,
    LcBallTrial,
    LcLampTrial,
    LcHookTrial,
    Triforce,
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

        HeartPiece01 |
        HeartPiece02 |
        HeartPiece03 |
        HeartPiece04 |
        HeartPiece05 |
        HeartPiece06 |
        HeartPiece07 |
        HeartPiece08 |
        HeartPiece09 |
        HeartPiece10 |
        HeartPiece11 |
        HeartPiece12 |
        HeartPiece13 |
        HeartPiece14 |
        HeartPiece15 |
        HeartPiece16 |
        HeartPiece17 |
        HeartPiece18 |
        HeartPiece19 |
        HeartPiece20 |
        HeartPiece21 |
        HeartPiece22 |
        HeartPiece23 |
        HeartPiece24 |
        HeartPiece25 |
        HeartPiece26 |
        HeartPiece27 => Some(HeartPiece),

        HeartContainer01 |
        HeartContainer02 |
        HeartContainer03 |
        HeartContainer04 |
        HeartContainer05 |
        HeartContainer06 |
        HeartContainer07 |
        HeartContainer08 |
        HeartContainer09 |
        HeartContainer10 => Some(HeartContainer),

        Bottle01 |
        Bottle02 |
        Bottle03 |
        Bottle04 |
        Bottle05 => Some(ItemBottle),

        Lamp01 |
        Lamp02 => Some(ItemKandelaar),

        Sword01 |
        Sword02 |
        Sword03 |
        Sword04 => Some(ItemSwordLv1),

        Glove01 |
        Glove02 => Some(PowerGlove),

        Net01 |
        Net02 => Some(ItemInsectNet),

        Mail01 |
        Mail02 => Some(ClothesBlue),

        FillerItem::OreYellow => Some(Item::OreYellow),
        FillerItem::OreGreen => Some(Item::OreGreen),
        FillerItem::OreBlue => Some(Item::OreBlue),
        FillerItem::OreRed => Some(Item::OreRed),

        // Small Keys
        FillerItem::HyruleSanctuaryKey |
        FillerItem::LoruleSanctuaryKey |
        EasternKeySmall01 |
        EasternKeySmall02 |
        GalesKeySmall01 |
        GalesKeySmall02 |
        GalesKeySmall03 |
        GalesKeySmall04 |
        HeraKeySmall01 |
        HeraKeySmall02 |
        DarkKeySmall01 |
        DarkKeySmall02 |
        DarkKeySmall03 |
        DarkKeySmall04 |
        SwampKeySmall01 |
        SwampKeySmall02 |
        SwampKeySmall03 |
        SwampKeySmall04 |
        SkullKeySmall01 |
        SkullKeySmall02 |
        SkullKeySmall03 |
        FillerItem::ThievesKeySmall |
        IceKeySmall01 |
        IceKeySmall02 |
        IceKeySmall03 |
        DesertKeySmall01 |
        DesertKeySmall02 |
        DesertKeySmall03 |
        DesertKeySmall04 |
        DesertKeySmall05 |
        TurtleKeySmall01 |
        TurtleKeySmall02 |
        TurtleKeySmall03 |
        LoruleCastleKeySmall01 |
        LoruleCastleKeySmall02 |
        LoruleCastleKeySmall03 |
        LoruleCastleKeySmall04 |
        LoruleCastleKeySmall05 => Some(KeySmall),

        // Big Keys
        FillerItem::EasternKeyBig |
        FillerItem::GalesKeyBig |
        FillerItem::HeraKeyBig |
        FillerItem::DarkKeyBig |
        FillerItem::SwampKeyBig |
        FillerItem::SkullKeyBig |
        FillerItem::ThievesKeyBig |
        FillerItem::IceKeyBig |
        FillerItem::DesertKeyBig |
        FillerItem::TurtleKeyBig => Some(KeyBoss),

        // Compasses
        FillerItem::EasternCompass |
        FillerItem::GalesCompass |
        FillerItem::HeraCompass |
        FillerItem::DarkCompass |
        FillerItem::SwampCompass |
        FillerItem::SkullCompass |
        FillerItem::ThievesCompass |
        FillerItem::IceCompass |
        FillerItem::DesertCompass |
        FillerItem::TurtleCompass |
        FillerItem::LoruleCastleCompass => Some(Compass),

        //GreatSpin => Some(SpecialMove),
        RupeeGreen => Some(RupeeG),
        RupeeBlue => Some(RupeeB),
        RupeeRed => Some(RupeeR),

        RupeePurple01 | RupeePurple02 | RupeePurple03 | RupeePurple04 | RupeePurple05 |
        RupeePurple06 | RupeePurple07 | RupeePurple08 | RupeePurple09 | RupeePurple10 |
        RupeePurple11 | RupeePurple12 | RupeePurple13 | RupeePurple14 | RupeePurple15 |
        RupeePurple16 | RupeePurple17 | RupeePurple18 => Some(RupeePurple),

        RupeeSilver01 | RupeeSilver02 | RupeeSilver03 | RupeeSilver04 | RupeeSilver05 |
        RupeeSilver06 | RupeeSilver07 | RupeeSilver08 | RupeeSilver09 | RupeeSilver10 |
        RupeeSilver11 | RupeeSilver12 | RupeeSilver13 | RupeeSilver14 | RupeeSilver15 |
        RupeeSilver16 | RupeeSilver17 | RupeeSilver18 | RupeeSilver19 | RupeeSilver20 |
        RupeeSilver21 | RupeeSilver22 | RupeeSilver23 | RupeeSilver24 | RupeeSilver25 |
        RupeeSilver26 | RupeeSilver27 | RupeeSilver28 | RupeeSilver29 | RupeeSilver30 |
        RupeeSilver31 | RupeeSilver32 | RupeeSilver33 | RupeeSilver34 | RupeeSilver35 |
        RupeeSilver36 | RupeeSilver37 | RupeeSilver38 | RupeeSilver39 | RupeeSilver40
        => Some(RupeeSilver),

        RupeeGold01 | RupeeGold02 | RupeeGold03 | RupeeGold04 | RupeeGold05 | RupeeGold06 |
        RupeeGold07 | RupeeGold08 | RupeeGold09 | RupeeGold10 => Some(RupeeGold),

        Maiamai001 | Maiamai002 | Maiamai003 | Maiamai004 | Maiamai005 | Maiamai006 | Maiamai007 |
        Maiamai008 | Maiamai009 | Maiamai010 | Maiamai011 | Maiamai012 | Maiamai013 | Maiamai014 |
        Maiamai015 | Maiamai016 | Maiamai017 | Maiamai018 | Maiamai019 | Maiamai020 | Maiamai021 |
        Maiamai022 | Maiamai023 | Maiamai024 | Maiamai025 | Maiamai026 | Maiamai027 | Maiamai028 |
        Maiamai029 | Maiamai030 | Maiamai031 | Maiamai032 | Maiamai033 | Maiamai034 | Maiamai035 |
        Maiamai036 | Maiamai037 | Maiamai038 | Maiamai039 | Maiamai040 | Maiamai041 | Maiamai042 |
        Maiamai043 | Maiamai044 | Maiamai045 | Maiamai046 | Maiamai047 | Maiamai048 | Maiamai049 |
        Maiamai050 | Maiamai051 | Maiamai052 | Maiamai053 | Maiamai054 | Maiamai055 | Maiamai056 |
        Maiamai057 | Maiamai058 | Maiamai059 | Maiamai060 | Maiamai061 | Maiamai062 | Maiamai063 |
        Maiamai064 | Maiamai065 | Maiamai066 | Maiamai067 | Maiamai068 | Maiamai069 | Maiamai070 |
        Maiamai071 | Maiamai072 | Maiamai073 | Maiamai074 | Maiamai075 | Maiamai076 | Maiamai077 |
        Maiamai078 | Maiamai079 | Maiamai080 | Maiamai081 | Maiamai082 | Maiamai083 | Maiamai084 |
        Maiamai085 | Maiamai086 | Maiamai087 | Maiamai088 | Maiamai089 | Maiamai090 | Maiamai091 |
        Maiamai092 | Maiamai093 | Maiamai094 | Maiamai095 | Maiamai096 | Maiamai097 | Maiamai098 |
        Maiamai099 | Maiamai100 => Some(Kinsta),

        MonsterGuts => Some(LiverPurple),
        MonsterHorn => Some(LiverYellow),
        MonsterTail => Some(LiverBlue),

        // Dungeon Items
        PendantOfCourage => Some(PendantCourage),
        PendantOfWisdom => Some(PendantWisdom),
        PendantOfPower => Some(PendantPower),
        FillerItem::SageGulley => Some(Item::SageGulley),
        FillerItem::SageOren => Some(Item::SageOren),
        FillerItem::SageSeres => Some(Item::SageSeres),
        FillerItem::SageOsfala => Some(Item::SageOsfala),
        FillerItem::SageImpa => Some(Item::SageImpa),
        FillerItem::SageIrene => Some(Item::SageIrene),
        FillerItem::SageRosso => Some(Item::SageRosso),


        // Quest Items don't translate
        EasternComplete | DarkComplete | ThievesComplete |
        ScootFruit | FoulFruit | Shield |
        ShadyGuyTrigger | OpenSanctuaryDoors |
        GoldBee | BigBombFlower | StylishWomansHouseOpen |
        SkullEyeRight | SkullEyeLeft |
        TurtleFlipped | TurtleAttacked | TurtleWall |
        AccessLoruleCastleField | AccessHyruleBlacksmith | AccessPotionShop | AccessMilkBar |
        LcBombTrial | LcBallTrial | LcLampTrial | LcHookTrial |
        Triforce => None
    }
}
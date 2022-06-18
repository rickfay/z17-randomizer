use albw::Item;
use albw::Item::*;
use FillerItem::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillerItem {
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
    RupeePurple,
    RupeeSilver,
    RupeeGold,

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

    ScootFruit,
    GoldBee,
    BigBombFlower,
    StylishWomansHouseOpen,
    SkullEyeRight,
    SkullEyeLeft,
    AccessHildaBarrier,

    // TODO add Hyrule Blacksmith reachable
}

pub fn convert(fill_item: FillerItem) -> Option<Item> {
    match fill_item {
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
        HeartPiece27 |
        HeartPiece28 => Some(HeartPiece),

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

        GreatSpin => Some(SpecialMove),
        RupeeGreen => Some(RupeeG),
        RupeeBlue => Some(RupeeB),
        RupeeRed => Some(RupeeR),
        FillerItem::RupeePurple => Some(Item::RupeePurple),
        FillerItem::RupeeSilver => Some(Item::RupeeSilver),
        FillerItem::RupeeGold => Some(Item::RupeeGold),
        MonsterGuts => Some(LiverPurple),
        MonsterHorn => Some(LiverYellow),
        MonsterTail => Some(LiverBlue),

        // Quest Items don't translate
        PendantOfCourage | PendantOfWisdom | PendantOfPower |
        SageGulley | SageOren | SageSeres | SageOsfala | SageRosso | SageIrene | SageImpa |
        ScootFruit | GoldBee | BigBombFlower | StylishWomansHouseOpen |
        SkullEyeRight | SkullEyeLeft | AccessHildaBarrier => None
    }
}
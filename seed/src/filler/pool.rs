use crate::filler::filler_item::{FillerItem, FillerItem::*};

///
pub type Pool = Vec<FillerItem>;

///
pub fn get_base_progression_pool() -> Vec<FillerItem> {
    let mut progression_pool = vec![
        GreatSpin, // Y Button Items
        Lamp01, Lamp02, Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Hammer01,
        Hammer02, Bombs01, Bombs02, FireRod01, FireRod02, IceRod01, IceRod02, TornadoRod01,
        TornadoRod02, SandRod01, SandRod02, Net01, Net02, HintGlasses, // 5 Bottles
        Bottle01, Bottle02, Bottle03, Bottle04, Bottle05, RaviosBracelet01, RaviosBracelet02, Bell,
        StaminaScroll, BowOfLight, PegasusBoots, Flippers, HylianShield, SmoothGem, PremiumMilk,
        LetterInABottle, Pouch, // 2 Gloves
        Glove01, Glove02, // 2 Mails
        Mail01, Mail02, // 4 Master Ore
        OreYellow, OreGreen, OreBlue, OreRed, // Shop Items
        ScootFruit01, ScootFruit02, FoulFruit01, FoulFruit02, Shield01, Shield02, Shield03,
        Shield04, GoldBee01,
        // GoldBee02,
        // GoldBee03,
    ];

    progression_pool.extend(get_gold_rupee_pool());
    progression_pool.extend(get_silver_rupee_pool());
    progression_pool.extend(get_purple_rupee_pool());
    progression_pool.extend(get_maiamai_pool());
    progression_pool
}

///
pub fn get_dungeon_prize_pool() -> Vec<FillerItem> {
    vec![
        PendantOfPower, PendantOfWisdom, PendantOfCourage01, PendantOfCourage02, SageGulley,
        SageOren, SageSeres, SageOsfala, SageImpa, SageIrene, SageRosso,
    ]
}

///
pub fn get_big_key_pool() -> Vec<FillerItem> {
    vec![
        EasternKeyBig, GalesKeyBig, HeraKeyBig, DarkKeyBig, SwampKeyBig, SkullKeyBig,
        ThievesKeyBig, IceKeyBig, DesertKeyBig, TurtleKeyBig,
    ]
}

///
pub fn get_small_key_pool() -> Vec<FillerItem> {
    vec![
        HyruleSanctuaryKey, LoruleSanctuaryKey, EasternKeySmall01, EasternKeySmall02,
        GalesKeySmall01, GalesKeySmall02, GalesKeySmall03, GalesKeySmall04, HeraKeySmall01,
        HeraKeySmall02, DarkKeySmall01, DarkKeySmall02, DarkKeySmall03, DarkKeySmall04,
        SwampKeySmall01, SwampKeySmall02, SwampKeySmall03, SwampKeySmall04, SkullKeySmall01,
        SkullKeySmall02, SkullKeySmall03, ThievesKeySmall, IceKeySmall01, IceKeySmall02,
        IceKeySmall03, DesertKeySmall01, DesertKeySmall02, DesertKeySmall03, DesertKeySmall04,
        DesertKeySmall05, TurtleKeySmall01, TurtleKeySmall02, TurtleKeySmall03,
        LoruleCastleKeySmall01, LoruleCastleKeySmall02, LoruleCastleKeySmall03,
        LoruleCastleKeySmall04, LoruleCastleKeySmall05,
    ]
}

///
pub fn get_compass_pool() -> Vec<FillerItem> {
    vec![
        EasternCompass, GalesCompass, HeraCompass, DarkCompass, SwampCompass, SkullCompass,
        ThievesCompass, TurtleCompass, DesertCompass, IceCompass, LoruleCastleCompass,
    ]
}

///
pub fn get_gold_rupee_pool() -> Vec<FillerItem> {
    vec![
        RupeeGold01, RupeeGold02, RupeeGold03, RupeeGold04, RupeeGold05, RupeeGold06, RupeeGold07,
        RupeeGold08, RupeeGold09, RupeeGold10,
    ]
}

///
pub fn get_silver_rupee_pool() -> Vec<FillerItem> {
    vec![
        RupeeSilver01, RupeeSilver02, RupeeSilver03, RupeeSilver04, RupeeSilver05, RupeeSilver06,
        RupeeSilver07, RupeeSilver08, RupeeSilver09, RupeeSilver10, RupeeSilver11, RupeeSilver12,
        RupeeSilver13, RupeeSilver14, RupeeSilver15, RupeeSilver16, RupeeSilver17, RupeeSilver18,
        RupeeSilver19, RupeeSilver20, RupeeSilver21, RupeeSilver22, RupeeSilver23, RupeeSilver24,
        RupeeSilver25, RupeeSilver26, RupeeSilver27, RupeeSilver28, RupeeSilver29, RupeeSilver30,
        RupeeSilver31, RupeeSilver32, RupeeSilver33, RupeeSilver34, RupeeSilver35, RupeeSilver36,
        RupeeSilver37, RupeeSilver38, // Turtle Rock B1
        RupeeSilver39, // Cucco Dungeon
        RupeeSilver40, // Ku's Domain
        RupeeSilver41, // Hotfoot 1st Race
    ]
}

///
pub fn get_purple_rupee_pool() -> Vec<FillerItem> {
    vec![
        RupeePurple01, RupeePurple02, RupeePurple03, RupeePurple04, RupeePurple05, RupeePurple06,
        RupeePurple07, RupeePurple08, RupeePurple09, RupeePurple10, RupeePurple11, RupeePurple12,
        RupeePurple13, RupeePurple14, RupeePurple15, RupeePurple16, RupeePurple17, RupeePurple18,
        RupeePurple19, RupeePurple20,
    ]
}

///
fn get_base_junk_pool() -> Vec<FillerItem> {
    vec![
        // The Greg Twins
        RupeeGreen, RupeeGreen, // 8 Blue Rupees
        RupeeBlue, RupeeBlue, RupeeBlue, RupeeBlue, RupeeBlue, RupeeBlue, RupeeBlue, RupeeBlue,
        // 20 Red Rupees
        RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed,
        RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed, RupeeRed,
        RupeeRed, RupeeRed, // 4 Monster Tails
        MonsterTail, MonsterTail, MonsterTail, MonsterTail, // 3 Monster Horns
        MonsterHorn, MonsterHorn, MonsterHorn, // 12 Monster Guts
        MonsterGuts, MonsterGuts, MonsterGuts, MonsterGuts, MonsterGuts, MonsterGuts, MonsterGuts,
        MonsterGuts, MonsterGuts, MonsterGuts, MonsterGuts, MonsterGuts, // Heart Pieces
        HeartPiece01, HeartPiece02, HeartPiece03, HeartPiece04, HeartPiece05, HeartPiece06,
        HeartPiece07, HeartPiece08, HeartPiece09, HeartPiece10, HeartPiece11, HeartPiece12,
        HeartPiece13, HeartPiece14, HeartPiece15, HeartPiece16, HeartPiece17, HeartPiece18,
        HeartPiece19, HeartPiece20, HeartPiece21, HeartPiece22, HeartPiece23, HeartPiece24,
        HeartPiece25, HeartPiece26, HeartPiece27, HeartPiece28, // Heart Containers
        HeartContainer01, HeartContainer02, HeartContainer03, HeartContainer04, HeartContainer05,
        HeartContainer06, HeartContainer07, HeartContainer08, HeartContainer09,
        HeartContainer10,
        /*
         * Extra Items
         * +1 location:  Osfala in Chamber of Sages (not adding rental Sand Rod)
         * +1 location:  Blacksmith Table (not adding PackageSword)
         * -2 locations: 2nd Bracelet added to pool without a vanilla location
         * -------------
         * =0 extra items added to junk pool
         */
    ]
}

///
pub fn get_maiamai_pool() -> Vec<FillerItem> {
    vec![
        Maiamai001, Maiamai002, Maiamai003, Maiamai004, Maiamai005, Maiamai006, Maiamai007,
        Maiamai008, Maiamai009, Maiamai010, Maiamai011, Maiamai012, Maiamai013, Maiamai014,
        Maiamai015, Maiamai016, Maiamai017, Maiamai018, Maiamai019, Maiamai020, Maiamai021,
        Maiamai022, Maiamai023, Maiamai024, Maiamai025, Maiamai026, Maiamai027, Maiamai028,
        Maiamai029, Maiamai030, Maiamai031, Maiamai032, Maiamai033, Maiamai034, Maiamai035,
        Maiamai036, Maiamai037, Maiamai038, Maiamai039, Maiamai040, Maiamai041, Maiamai042,
        Maiamai043, Maiamai044, Maiamai045, Maiamai046, Maiamai047, Maiamai048, Maiamai049,
        Maiamai050, Maiamai051, Maiamai052, Maiamai053, Maiamai054, Maiamai055, Maiamai056,
        Maiamai057, Maiamai058, Maiamai059, Maiamai060, Maiamai061, Maiamai062, Maiamai063,
        Maiamai064, Maiamai065, Maiamai066, Maiamai067, Maiamai068, Maiamai069, Maiamai070,
        Maiamai071, Maiamai072, Maiamai073, Maiamai074, Maiamai075, Maiamai076, Maiamai077,
        Maiamai078, Maiamai079, Maiamai080, Maiamai081, Maiamai082, Maiamai083, Maiamai084,
        Maiamai085, Maiamai086, Maiamai087, Maiamai088, Maiamai089, Maiamai090, Maiamai091,
        Maiamai092, Maiamai093, Maiamai094, Maiamai095, Maiamai096, Maiamai097, Maiamai098,
        Maiamai099, Maiamai100,
    ]
}

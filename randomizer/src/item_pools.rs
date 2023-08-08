use modinfo::settings::{logic::LogicMode, Settings};
use rand::{rngs::StdRng, Rng};

use crate::{filler_util::shuffle, model::filler_item::Item};

pub type Pool = Vec<Item>;

/**
 * Builds the Progression and Junk item pools according to the settings<br /><br />
 *
 * The total number of items returned between both pools should match the total number of locations
 * in the world graph, including locations that statically set their contents.
 */
pub(crate) fn get_item_pools(settings: &Settings, rng: &mut StdRng) -> (Pool, Pool) {
    let mut progression_items = get_base_progression_pool();
    let dungeon_prizes = get_dungeon_prize_pool();
    let big_keys = get_big_key_pool();
    let small_keys = get_small_key_pool();
    let compasses = get_compass_pool();
    let mut junk_pool = get_base_junk_pool();

    // Choose either Letter in a Bottle or Premium Milk to include in the seed
    progression_items.push(choose_trade_item(rng));

    // Remove the Bee Badge from Hell Logic to keep Bee Boosting viable
    match settings.logic.logic_mode {
        LogicMode::Hell => junk_pool.push(Item::Empty),
        _ => progression_items.push(Item::BeeBadge),
    };

    // Swordless Mode
    if settings.logic.swordless_mode {
        junk_pool.extend_from_slice(&[Item::Empty, Item::Empty, Item::Empty, Item::Empty]);
    } else {
        progression_items.extend_from_slice(&[
            Item::Sword01,
            Item::Sword02,
            Item::Sword03,
            Item::Sword04,
        ]);
    }

    (
        shuffle_order_progression_pools(
            rng, dungeon_prizes, big_keys, small_keys, compasses, progression_items,
        ),
        shuffle(rng, junk_pool),
    )
}

/**
 * Shuffle item categories amongst themselves, then order them as follows:
 * - Dungeon Prizes
 * - Big Keys
 * - Small Keys
 * - Compasses
 * - All other progression
 */
fn shuffle_order_progression_pools(
    rng: &mut StdRng, dungeon_prizes: Vec<Item>, big_keys: Vec<Item>, small_keys: Vec<Item>,
    compasses: Vec<Item>, progression: Vec<Item>,
) -> Pool {
    let mut progression_pool;

    progression_pool = shuffle(rng, dungeon_prizes);
    progression_pool.extend(shuffle(rng, big_keys));
    progression_pool.extend(shuffle(rng, small_keys));
    progression_pool.extend(shuffle(rng, compasses));
    progression_pool.extend(shuffle(rng, progression));

    progression_pool
}

/// Randomly chooses one trade item to include in the seed.
/// - If [`LetterInABottle`] is chosen, it must be turned in at the Milk Bar to get the [`PremiumMilk`].
/// - If [`PremiumMilk`] is chosen, the Milk Bar is irrelevant and there simply is no [`LetterInABottle`] in the seed.
fn choose_trade_item(rng: &mut StdRng) -> Item {
    [Item::LetterInABottle, Item::PremiumMilk][rng.gen_range(0..2)]
}

fn get_base_progression_pool() -> Vec<Item> {
    use Item::*;
    let mut progression_pool = vec![
        GreatSpin, Lamp01, Lamp02, Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01, Hookshot02,
        Hammer01, Hammer02, Bombs01, Bombs02, FireRod01, FireRod02, IceRod01, IceRod02,
        TornadoRod01, TornadoRod02, SandRod01, SandRod02, Net01, Net02, HintGlasses, Bottle01,
        Bottle02, Bottle03, Bottle04, RaviosBracelet01, RaviosBracelet02, Bell, StaminaScroll,
        BowOfLight, PegasusBoots, Flippers, HylianShield, SmoothGem, Pouch, Glove01, Glove02,
        Mail01, Mail02, OreYellow, OreGreen, OreBlue, OreRed, ScootFruit01, ScootFruit02,
        FoulFruit01, FoulFruit02, Shield01, Shield02, Shield03, Shield04, GoldBee01,
    ];

    progression_pool.extend(get_gold_rupee_pool());
    progression_pool.extend(get_silver_rupee_pool());
    progression_pool.extend(get_purple_rupee_pool());
    progression_pool.extend(get_maiamai_pool());
    progression_pool
}

fn get_dungeon_prize_pool() -> Vec<Item> {
    use Item::*;
    vec![
        PendantOfPower, PendantOfWisdom, PendantOfCourage01, PendantOfCourage02, SageGulley,
        SageOren, SageSeres, SageOsfala, SageImpa, SageIrene, SageRosso,
    ]
}

fn get_big_key_pool() -> Vec<Item> {
    use Item::*;
    vec![
        EasternKeyBig, GalesKeyBig, HeraKeyBig, DarkKeyBig, SwampKeyBig, SkullKeyBig,
        ThievesKeyBig, IceKeyBig, DesertKeyBig, TurtleKeyBig,
    ]
}

fn get_small_key_pool() -> Vec<Item> {
    use Item::*;
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

fn get_compass_pool() -> Vec<Item> {
    use Item::*;
    vec![
        EasternCompass, GalesCompass, HeraCompass, DarkCompass, SwampCompass, SkullCompass,
        ThievesCompass, TurtleCompass, DesertCompass, IceCompass, LoruleCastleCompass,
    ]
}

pub(crate) fn get_gold_rupee_pool() -> Vec<Item> {
    use Item::*;
    vec![
        RupeeGold01, RupeeGold02, RupeeGold03, RupeeGold04, RupeeGold05, RupeeGold06, RupeeGold07,
        RupeeGold08, RupeeGold09, RupeeGold10,
    ]
}

pub(crate) fn get_silver_rupee_pool() -> Vec<Item> {
    use Item::*;
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

pub(crate) fn get_purple_rupee_pool() -> Vec<Item> {
    use Item::*;
    vec![
        RupeePurple01, RupeePurple02, RupeePurple03, RupeePurple04, RupeePurple05, RupeePurple06,
        RupeePurple07, RupeePurple08, RupeePurple09, RupeePurple10, RupeePurple11, RupeePurple12,
        RupeePurple13, RupeePurple14, RupeePurple15, RupeePurple16, RupeePurple17, RupeePurple18,
        RupeePurple19, RupeePurple20,
    ]
}

fn get_base_junk_pool() -> Vec<Item> {
    use Item::*;
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
        HeartContainer06, HeartContainer07, HeartContainer08, HeartContainer09, HeartContainer10,
        /*
         * Extra Items
         * +1 location:  Osfala in Chamber of Sages (not adding rental Sand Rod)
         * +1 location:  Blacksmith Table (not adding PackageSword)
         * +1 location:  Bouldering Guy's Emptied Bottle
         * -2 locations: 2nd Bracelet added to pool without a vanilla location
         * -------------
         * =1 extra items added to junk pool
         */
        RupeeBlue,
    ]
}

pub(crate) fn get_maiamai_pool() -> Vec<Item> {
    use Item::*;
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

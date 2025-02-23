use crate::filler::cracks::Crack;
use crate::filler::filler_item::Item;
use crate::filler::filler_item::Item::*;
use crate::filler::filler_item::Vane;
use crate::filler::util::shuffle;
use crate::SeedInfo;
use modinfo::settings::cracks::Cracks;
use modinfo::settings::keysy::Keysy;
use modinfo::settings::logic::LogicMode;
use modinfo::settings::nice_items::NiceItems;
use modinfo::Settings;
use rand::{rngs::StdRng, Rng};
use std::cmp::Ordering;
use std::iter::repeat;

pub type Pool = Vec<Item>;

/**
 * Builds the Progression and Junk item pools according to the settings<br /><br />
 *
 * The total number of items returned between both pools should match the total number of locations
 * in the world graph, including locations that statically set their contents.
 */
pub(crate) fn get_item_pools(rng: &mut StdRng, SeedInfo { settings, .. }: &SeedInfo) -> (Pool, Pool) {
    let mut progression_items = get_base_progression_pool();
    let minor_progression = get_minor_progression_pool();
    let dungeon_prizes = get_dungeon_prize_pool();
    let big_keys = get_big_key_pool(settings);
    let small_keys = get_small_key_pool(settings);
    let compasses = get_compass_pool();

    let mut delta_junk_items = 0;

    progression_items.push(if settings.progressive_bow_of_light { Bow03 } else { BowOfLight });

    // Choose either Letter in a Bottle or Premium Milk to include in the seed
    progression_items.push(choose_trade_item(rng));

    // Cracks
    if settings.cracks == Cracks::Closed {
        delta_junk_items -= 1;
        progression_items.push(Quake);
    }

    // Nice Items
    match settings.nice_items {
        NiceItems::Vanilla | NiceItems::Shuffled => progression_items.extend_from_slice(&[
            Bow02, Boomerang02, Hookshot02, Hammer02, Bombs02, FireRod02, IceRod02, TornadoRod02, SandRod02,
        ]),
        NiceItems::Off => delta_junk_items += 9,
    }

    // Replaces two pieces of junk with an extra Lamp and Net
    if settings.super_items {
        delta_junk_items -= 2;
        progression_items.push(Lamp02);
        progression_items.push(Net02);
    }

    // Ravio's Bracelets
    if settings.start_with_merge {
        delta_junk_items += 2;
    } else {
        progression_items.push(RaviosBracelet01);
        progression_items.push(RaviosBracelet02);
    }

    // Pouch
    if settings.start_with_pouch {
        delta_junk_items += 1;
    } else {
        progression_items.push(Pouch);
    }

    // Remove the Bee Badge from Hell Logic to keep Bee Boosting viable
    if settings.logic_mode == LogicMode::Hell {
        delta_junk_items += 1;
    } else {
        progression_items.push(BeeBadge);
    };

    // Swordless Mode
    if settings.swordless_mode {
        delta_junk_items += 4;
    } else {
        progression_items.extend_from_slice(&[Sword01, Sword02, Sword03, Sword04]);
    }

    // Junk Pool. Add or remove elements from the junk pool based on chosen settings.
    let junk_pool = get_base_junk_pool(rng);
    let mut junk_pool = shuffle(rng, junk_pool);

    match delta_junk_items.cmp(&0) {
        Ordering::Greater => (0..delta_junk_items).for_each(|_| add_random_junk_item(rng, &mut junk_pool)),
        Ordering::Less => (0..-delta_junk_items).for_each(|_| {
            junk_pool.pop();
        }),
        Ordering::Equal => {},
    }

    (
        shuffle_order_progression_pools(
            rng,
            vec![dungeon_prizes, big_keys, small_keys, compasses, progression_items, minor_progression],
        ),
        junk_pool,
    )
}

/**
 * Shuffle item categories amongst themselves, then order them as follows:
 * - Dungeon Prizes
 * - Big Keys
 * - Small Keys
 * - Compasses
 * - Progression
 * - Minor progression
 */
fn shuffle_order_progression_pools(rng: &mut StdRng, pools: Vec<Vec<Item>>) -> Pool {
    pools.iter().flat_map(|pool| shuffle(rng, pool.to_vec())).collect::<Pool>()
}

/// Randomly chooses one trade item to include in the seed.
/// - If [`LetterInABottle`] is chosen, it must be turned in at the Milk Bar to get the [`PremiumMilk`].
/// - If [`PremiumMilk`] is chosen, the Milk Bar is irrelevant and there simply is no [`LetterInABottle`] in the seed.
fn choose_trade_item(rng: &mut StdRng) -> Item {
    [LetterInABottle, PremiumMilk][rng.gen_range(0..2)]
}

fn get_base_progression_pool() -> Vec<Item> {
    let progression_pool = vec![
        GreatSpin, Lamp01, Bow01, Boomerang01, Hookshot01, Hammer01, Bombs01, FireRod01, IceRod01, TornadoRod01,
        SandRod01, Net01, HintGlasses, Bottle01, Bottle02, Bottle03, Bottle04, Bell, StaminaScroll, PegasusBoots,
        Flippers, HylianShield, SmoothGem, Glove01, Glove02, Mail01, Mail02, OreYellow, OreGreen, OreBlue, OreRed,
        ScootFruit01, ScootFruit02, FoulFruit01, FoulFruit02, Shield01, Shield02, Shield03, Shield04, GoldBee01, Charm,
    ];

    progression_pool
}

pub(crate) fn get_minor_progression_pool() -> Vec<Item> {
    let mut minor_progression_pool = vec![];

    minor_progression_pool.extend(get_maiamai_pool());
    minor_progression_pool.extend(get_heart_containers());
    minor_progression_pool.extend(get_heart_pieces());
    minor_progression_pool.extend(get_gold_rupee_pool());
    minor_progression_pool.extend(get_silver_rupee_pool());
    minor_progression_pool.extend(get_purple_rupee_pool());
    minor_progression_pool.extend(get_greg_pool());

    minor_progression_pool
}

fn get_greg_pool() -> Vec<Item> {
    vec![RupeeGreen, RupeeGreen]
}

fn get_dungeon_prize_pool() -> Vec<Item> {
    vec![
        PendantOfPower, PendantOfWisdom, PendantOfCourage, SageGulley, SageOren, SageSeres, SageOsfala, SageImpa,
        SageIrene, SageRosso,
    ]
}

/// List of "normal" up-facing Hyrule cracks
pub(crate) fn get_hyrule_up_cracks() -> Vec<Crack> {
    use crate::filler::cracks::Crack::*;
    vec![
        StylishWoman, YourHouse, WaterfallHyrule, EasternRuinsSE, LostWoodsPillar, RossosHouse, MiseryMireEntrance,
        DesertMiddle, DesertSW, DesertNorth, DesertPillarLeft, DesertPillarRight, DesertPalace, DeathWestHyrule,
        FloatingIslandHyrule, RiverHyrule, LakeHylia, HyruleHotfoot, Sanctuary, GraveyardLedgeHyrule,
        RossosOreMineHyrule, HyruleCastle,
    ]
}

/// List of "normal" up-facing Lorule cracks
pub(crate) fn get_lorule_up_cracks() -> Vec<Crack> {
    use crate::filler::cracks::Crack::*;
    vec![
        ThievesTown, VacantHouse, WaterfallLorule, DarkRuinsSE, SkullWoodsPillar, DestroyedHouse, MiseryMireExit,
        MireMiddle, MireSW, MireNorth, MirePillarLeft, MirePillarRight, Zaganaga, DeathWestLorule,
        FloatingIslandLorule, RiverLorule, LoruleLake, LoruleHotfoot, Philosopher, GraveyardLedgeLorule,
        RossosOreMineLorule, LoruleCastle,
    ]
}

/// List of down-facing Hyrule cracks
pub(crate) fn get_hyrule_down_cracks() -> Vec<Crack> {
    use crate::filler::cracks::Crack::*;
    vec![ParadoxRightHyrule, ParadoxLeftHyrule, EasternRuinsPillar, SahasrahlasHouse, SwampPillarHyrule, ZorasDomain]
}

/// List of down-facing Lorule cracks
pub(crate) fn get_lorule_down_cracks() -> Vec<Crack> {
    use crate::filler::cracks::Crack::*;
    vec![ParadoxRightLorule, ParadoxLeftLorule, DarkRuinsPillar, NShapedHouse, SwampPillarLorule, KusDomain]
}

pub(crate) fn get_weather_vanes() -> Vec<Vane> {
    use Vane::*;
    vec![
        YourHouseWV, KakarikoVillageWV, EasternPalaceWV, HouseOfGalesWV, TowerOfHeraWV, WitchsHouseWV,
        DeathMountainHyruleWV, DesertPalaceWV, SanctuaryWV, SkullWoodsWV, TreacherousTowerWV, IceRuinsWV,
        LoruleCastleWV, GraveyardWV, ThievesTownWV, DarkPalaceWV, BlacksmithWV, VacantHouseWV, MiseryMireWV,
        SwampPalaceWV, TurtleRockWV, DeathMountainLoruleWV,
    ]
}

fn get_big_key_pool(settings: &Settings) -> Vec<Item> {
    let big_keys = vec![
        EasternKeyBig, GalesKeyBig, HeraKeyBig, DarkKeyBig, SwampKeyBig, SkullKeyBig, ThievesKeyBig, IceKeyBig,
        DesertKeyBig, TurtleKeyBig,
    ];
    match settings.keysy {
        Keysy::BigKeysy | Keysy::AllKeysy => repeat(RupeeBlue).take(big_keys.len()).collect(),
        _ => big_keys,
    }
}

fn get_small_key_pool(settings: &Settings) -> Vec<Item> {
    let small_keys = vec![
        HyruleSanctuaryKey, LoruleSanctuaryKey, EasternKeySmall01, EasternKeySmall02, GalesKeySmall01, GalesKeySmall02,
        GalesKeySmall03, GalesKeySmall04, HeraKeySmall01, HeraKeySmall02, DarkKeySmall01, DarkKeySmall02,
        DarkKeySmall03, DarkKeySmall04, SwampKeySmall01, SwampKeySmall02, SwampKeySmall03, SwampKeySmall04,
        SkullKeySmall01, SkullKeySmall02, SkullKeySmall03, ThievesKeySmall, IceKeySmall01, IceKeySmall02,
        IceKeySmall03, DesertKeySmall01, DesertKeySmall02, DesertKeySmall03, DesertKeySmall04, DesertKeySmall05,
        TurtleKeySmall01, TurtleKeySmall02, TurtleKeySmall03, LoruleCastleKeySmall01, LoruleCastleKeySmall02,
        LoruleCastleKeySmall03, LoruleCastleKeySmall04, LoruleCastleKeySmall05,
    ];
    match settings.keysy {
        Keysy::SmallKeysy | Keysy::AllKeysy => repeat(RupeeBlue).take(small_keys.len()).collect(),
        _ => small_keys,
    }
}

fn get_compass_pool() -> Vec<Item> {
    vec![
        EasternCompass, GalesCompass, HeraCompass, DarkCompass, SwampCompass, SkullCompass, ThievesCompass,
        TurtleCompass, DesertCompass, IceCompass, LoruleCastleCompass,
    ]
}

pub fn get_gold_rupee_pool() -> Vec<Item> {
    vec![
        RupeeGold01, RupeeGold02, RupeeGold03, RupeeGold04, RupeeGold05, RupeeGold06, RupeeGold07, RupeeGold08,
        RupeeGold09, RupeeGold10,
    ]
}

pub fn get_silver_rupee_pool() -> Vec<Item> {
    vec![
        RupeeSilver01, RupeeSilver02, RupeeSilver03, RupeeSilver04, RupeeSilver05, RupeeSilver06, RupeeSilver07,
        RupeeSilver08, RupeeSilver09, RupeeSilver10, RupeeSilver11, RupeeSilver12, RupeeSilver13, RupeeSilver14,
        RupeeSilver15, RupeeSilver16, RupeeSilver17, RupeeSilver18, RupeeSilver19, RupeeSilver20, RupeeSilver21,
        RupeeSilver22, RupeeSilver23, RupeeSilver24, RupeeSilver25, RupeeSilver26, RupeeSilver27, RupeeSilver28,
        RupeeSilver29, RupeeSilver30, RupeeSilver31, RupeeSilver32, RupeeSilver33, RupeeSilver34, RupeeSilver35,
        RupeeSilver36, RupeeSilver37, RupeeSilver38, // Turtle Rock B1
        RupeeSilver39, // Cucco Dungeon
        RupeeSilver40, // Ku's Domain
        RupeeSilver41, // Hotfoot 1st Race
    ]
}

pub fn get_purple_rupee_pool() -> Vec<Item> {
    vec![
        RupeePurple01, RupeePurple02, RupeePurple03, RupeePurple04, RupeePurple05, RupeePurple06, RupeePurple07,
        RupeePurple08, RupeePurple09, RupeePurple10, RupeePurple11, RupeePurple12, RupeePurple13, RupeePurple14,
        RupeePurple15, RupeePurple16, RupeePurple17, RupeePurple18, RupeePurple19, RupeePurple20,
    ]
}

/// 28 Heart Pieces
pub fn get_heart_pieces() -> Vec<Item> {
    vec![
        HeartPiece01, HeartPiece02, HeartPiece03, HeartPiece04, HeartPiece05, HeartPiece06, HeartPiece07, HeartPiece08,
        HeartPiece09, HeartPiece10, HeartPiece11, HeartPiece12, HeartPiece13, HeartPiece14, HeartPiece15, HeartPiece16,
        HeartPiece17, HeartPiece18, HeartPiece19, HeartPiece20, HeartPiece21, HeartPiece22, HeartPiece23, HeartPiece24,
        HeartPiece25, HeartPiece26, HeartPiece27, HeartPiece28,
    ]
}

/// 10 Heart Containers
pub fn get_heart_containers() -> Vec<Item> {
    vec![
        HeartContainer01, HeartContainer02, HeartContainer03, HeartContainer04, HeartContainer05, HeartContainer06,
        HeartContainer07, HeartContainer08, HeartContainer09, HeartContainer10,
    ]
}

/// Junk Pool
fn get_base_junk_pool(rng: &mut StdRng) -> Vec<Item> {
    const BLUES: usize = 8;
    const REDS: usize = 20;
    const TAILS: usize = 4;
    const HORNS: usize = 3;
    const GUTS: usize = 12;
    const EXTRAS: usize = 3; // Osfala, Blacksmith Table, Bouldering Guy's Emptied Bottle

    let mut junk = Vec::with_capacity(BLUES + REDS + TAILS + HORNS + GUTS + EXTRAS);

    junk.extend(repeat(RupeeBlue).take(BLUES));
    junk.extend(repeat(RupeeRed).take(REDS));
    junk.extend(repeat(MonsterTail).take(TAILS));
    junk.extend(repeat(MonsterHorn).take(HORNS));
    junk.extend(repeat(MonsterGuts).take(GUTS));

    add_random_junk_item(rng, &mut junk); // replaces Captain's Sword
    add_random_junk_item(rng, &mut junk); // replaces Bouldering Guy Extra

    junk
}

fn add_random_junk_item(rng: &mut StdRng, junk_pool: &mut Vec<Item>) {
    const POSSIBLE_EXTRA_ITEMS: [Item; 3] = [MonsterTail, MonsterHorn, MonsterGuts];
    junk_pool.push(POSSIBLE_EXTRA_ITEMS[rng.gen_range(0..POSSIBLE_EXTRA_ITEMS.len())]);
}

pub fn get_maiamai_pool() -> Vec<Item> {
    vec![
        Maiamai001, Maiamai002, Maiamai003, Maiamai004, Maiamai005, Maiamai006, Maiamai007, Maiamai008, Maiamai009,
        Maiamai010, Maiamai011, Maiamai012, Maiamai013, Maiamai014, Maiamai015, Maiamai016, Maiamai017, Maiamai018,
        Maiamai019, Maiamai020, Maiamai021, Maiamai022, Maiamai023, Maiamai024, Maiamai025, Maiamai026, Maiamai027,
        Maiamai028, Maiamai029, Maiamai030, Maiamai031, Maiamai032, Maiamai033, Maiamai034, Maiamai035, Maiamai036,
        Maiamai037, Maiamai038, Maiamai039, Maiamai040, Maiamai041, Maiamai042, Maiamai043, Maiamai044, Maiamai045,
        Maiamai046, Maiamai047, Maiamai048, Maiamai049, Maiamai050, Maiamai051, Maiamai052, Maiamai053, Maiamai054,
        Maiamai055, Maiamai056, Maiamai057, Maiamai058, Maiamai059, Maiamai060, Maiamai061, Maiamai062, Maiamai063,
        Maiamai064, Maiamai065, Maiamai066, Maiamai067, Maiamai068, Maiamai069, Maiamai070, Maiamai071, Maiamai072,
        Maiamai073, Maiamai074, Maiamai075, Maiamai076, Maiamai077, Maiamai078, Maiamai079, Maiamai080, Maiamai081,
        Maiamai082, Maiamai083, Maiamai084, Maiamai085, Maiamai086, Maiamai087, Maiamai088, Maiamai089, Maiamai090,
        Maiamai091, Maiamai092, Maiamai093, Maiamai094, Maiamai095, Maiamai096, Maiamai097, Maiamai098, Maiamai099,
        Maiamai100,
    ]
}

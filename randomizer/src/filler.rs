use std::collections::{HashMap, HashSet};
use std::io;
use std::process::exit;
use log::{error, info};
use queue::Queue;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use albw::Item;
use crate::{convert, Layout, LocationInfo, Settings};
use crate::check::Check;
use crate::FillerItem;
use crate::FillerItem::*;
use crate::location::Location;
use crate::location_node::LocationNode;
use crate::progress::Progress;
use crate::world::build_world_graph;

/// Filler Algorithm
pub fn fill_stuff(settings: &Settings, seed: u64) -> Vec<(LocationInfo, Item)> {
    info!("New filler start!");

    let mut rng = StdRng::seed_from_u64(seed);

    info!("Building World Graph...");

    let mut world_graph = build_world_graph();

    info!("Verifying all locations accessible...");

    verify_all_locations_accessible(&mut world_graph);

    info!("Beginning Assumed Fill Algorithm...");

    let mut check_map = assumed_fill(&mut world_graph, &mut rng);

    for (key, val) in &check_map {
        if !val.is_none() {
            info!("{0: <50} {1:?}", *key, val.unwrap());
        }
    }

    info!("Filling in trash items...");

    fill_trash(&mut check_map, &mut rng);

    info!("OMG we got here");

    for (key, val) in check_map.clone() {
        info!("{0: <50}: {1:?}", key, val.unwrap());
    }

    map_to_result(world_graph, check_map)
}

/// Super dirty mapping I hate it
fn map_to_result(world_graph: HashMap<Location, LocationNode>, check_map: HashMap<&str, Option<FillerItem>>) -> Vec<(LocationInfo, Item)> {
    let mut result : Vec<(LocationInfo, Item)> = Vec::new();
    for (_, location_node) in world_graph {
        for check in location_node.get_checks() {
            if check.get_location_info().is_some() {
                result.push((
                    check.get_location_info().unwrap(),
                    convert(check_map.get(check.get_name()).unwrap().unwrap()).unwrap()));
            }
        }
    }
    result
}

fn get_progression_items() -> Vec<FillerItem> {
    vec![
        Bow01,
        Boomerang01,
        Hookshot01,
        Bombs01,
        FireRod01,
        IceRod01,
        Hammer01,
        SandRod01,
        TornadoRod01,
        RaviosBracelet01,
        Bell,
        StaminaScroll,
        BowOfLight,
        PegasusBoots,
        Flippers,
        HylianShield,
        SmoothGem,
        LetterInABottle,
        PremiumMilk,

        // Heart Pieces
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
        // HeartPiece28, // Not yet randomized, in Fortune's Choice minigame

        // Heart Containers
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

        // Eastern Palace Keys
        EasternCompass,
        EasternKeyBig,
        EasternKeySmall01,
        EasternKeySmall02,

        // House of Gales Keys
        GalesCompass,
        GalesKeyBig,
        GalesKeySmall01,
        GalesKeySmall02,
        GalesKeySmall03,
        GalesKeySmall04,

        // Tower of Hera Keys
        HeraCompass,
        HeraKeyBig,
        HeraKeySmall01,
        HeraKeySmall02,

        // Dark Palace Keys
        DarkCompass,
        DarkKeyBig,
        DarkKeySmall01,
        DarkKeySmall02,
        DarkKeySmall03,
        DarkKeySmall04,

        // Swamp Palace Keys
        SwampCompass,
        SwampKeyBig,
        SwampKeySmall01,
        SwampKeySmall02,
        SwampKeySmall03,
        SwampKeySmall04,

        // Skull Woods Keys
        SkullCompass,
        SkullKeyBig,
        SkullKeySmall01,
        SkullKeySmall02,
        SkullKeySmall03,

        // Thieves' Hideout Keys
        ThievesCompass,
        ThievesKeyBig,
        ThievesKeySmall,

        // Ice Ruins Keys
        IceCompass,
        IceKeyBig,
        IceKeySmall01,
        IceKeySmall02,
        IceKeySmall03,

        // Desert Palace Keys
        DesertCompass,
        DesertKeyBig,
        DesertKeySmall01,
        DesertKeySmall02,
        DesertKeySmall03,
        DesertKeySmall04,
        DesertKeySmall05,

        // Turtle Rock Keys
        TurtleCompass,
        TurtleKeyBig,
        TurtleKeySmall01,
        TurtleKeySmall02,
        TurtleKeySmall03,

        // Lorule Castle Keys
        LoruleCastleCompass,
        LoruleCastleKeySmall01,
        LoruleCastleKeySmall02,
        LoruleCastleKeySmall03,
        LoruleCastleKeySmall04,
        LoruleCastleKeySmall05,
    ]
}

fn is_dungeon_item(item: FillerItem) -> bool {
    match item {
        HyruleSanctuaryKey |
        LoruleSanctuaryKey |
        EasternCompass |
        EasternKeyBig |
        EasternKeySmall01 |
        EasternKeySmall02 |
        GalesCompass |
        GalesKeyBig |
        GalesKeySmall01 |
        GalesKeySmall02 |
        GalesKeySmall03 |
        GalesKeySmall04 |
        HeraCompass |
        HeraKeyBig |
        HeraKeySmall01 |
        HeraKeySmall02 |
        DarkCompass |
        DarkKeyBig |
        DarkKeySmall01 |
        DarkKeySmall02 |
        DarkKeySmall03 |
        DarkKeySmall04 |
        SwampCompass |
        SwampKeyBig |
        SwampKeySmall01 |
        SwampKeySmall02 |
        SwampKeySmall03 |
        SwampKeySmall04 |
        SkullCompass |
        SkullKeyBig |
        SkullKeySmall01 |
        SkullKeySmall02 |
        SkullKeySmall03 |
        ThievesCompass |
        ThievesKeyBig |
        ThievesKeySmall |
        IceCompass |
        IceKeyBig |
        IceKeySmall01 |
        IceKeySmall02 |
        IceKeySmall03 |
        DesertCompass |
        DesertKeyBig |
        DesertKeySmall01 |
        DesertKeySmall02 |
        DesertKeySmall03 |
        DesertKeySmall04 |
        DesertKeySmall05 |
        TurtleCompass |
        TurtleKeyBig |
        TurtleKeySmall01 |
        TurtleKeySmall02 |
        TurtleKeySmall03 |
        LoruleCastleCompass |
        LoruleCastleKeySmall01 |
        LoruleCastleKeySmall02 |
        LoruleCastleKeySmall03 |
        LoruleCastleKeySmall04 |
        LoruleCastleKeySmall05 => true,
        _ => false,
    }
}

fn get_trash_items() -> Vec<FillerItem> {
    vec![
        Pouch,
        BeeBadge,
        HintGlasses,

        // 2 Green Rupees
        RupeeGreen,
        RupeeGreen,

        // 8 Blue Rupees
        RupeeBlue,
        RupeeBlue,
        RupeeBlue,
        RupeeBlue,
        RupeeBlue,
        RupeeBlue,
        RupeeBlue,
        RupeeBlue,

        // 19 Red Rupees
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        RupeeRed,
        //RupeeRed, // TODO ????
        // RupeeRed, // Irene Removed

        // 18 Purple Rupees
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,
        RupeePurple,

        // 38 Silver Rupees
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,
        RupeeSilver,

        // 8 Gold Rupees
        RupeeGold,
        RupeeGold,
        RupeeGold,
        RupeeGold,
        RupeeGold,
        RupeeGold,
        RupeeGold,
        RupeeGold,

        // 4 Monster Tails
        MonsterTail,
        MonsterTail,
        MonsterTail,
        MonsterTail,

        // 3 Monster Horns
        MonsterHorn,
        MonsterHorn,
        MonsterHorn,

        // 12 Monster Guts
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
        MonsterGuts,
    ]
}

fn fill_trash(check_map: &mut HashMap<&str, Option<FillerItem>>, rng: &mut StdRng) {
    let trash_items = get_trash_items();

    info!("Begin the trash\n\n");

    let mut empty_check_keys = Vec::new();
    for (key, val) in check_map.clone() {
        if val.is_none() {
            empty_check_keys.push(key);
        }
    }

    if empty_check_keys.len() != trash_items.len() {
        error!("There are {} empty checks and {} trash items", empty_check_keys.len(), trash_items.len());
        exit(1);
    }

    for trash in trash_items {
        check_map.insert(empty_check_keys.remove(rng.gen_range(0..empty_check_keys.len())), Some(trash));
    }
}

fn place_item_randomly(item: FillerItem, checks: &Vec<Check>, check_map: &mut HashMap<&str, Option<FillerItem>>, rng: &mut StdRng) {
    let index = rng.gen_range(0..checks.len());
    check_map.insert(checks.get(index).unwrap().get_name(), Some(item));
}

fn filter_empty_checks(checks: &mut Vec<Check>, check_map: &mut HashMap<&str, Option<FillerItem>>) -> Vec<Check> {
    checks.iter().filter(|&x| check_map.get(x.get_name()).unwrap().is_none()).cloned().collect()
}

fn filter_dungeon_checks(item: FillerItem, eligible_checks: &mut Vec<Check>) -> Vec<Check> {
    eligible_checks.iter().filter(|&x| x.get_name().starts_with(match item {
        HyruleSanctuaryKey => "[HS]",
        LoruleSanctuaryKey => "[LS]",

        EasternCompass | EasternKeyBig | EasternKeySmall01 | EasternKeySmall02 => "[EP]",
        GalesCompass | GalesKeyBig | GalesKeySmall01 | GalesKeySmall02 | GalesKeySmall03 | GalesKeySmall04 => "[HoG]",
        HeraCompass | HeraKeyBig | HeraKeySmall01 | HeraKeySmall02 => "[ToH]",

        DarkCompass | DarkKeyBig | DarkKeySmall01 | DarkKeySmall02 | DarkKeySmall03 | DarkKeySmall04 => "[PoD]",
        SwampCompass | SwampKeyBig | SwampKeySmall01 | SwampKeySmall02 | SwampKeySmall03 | SwampKeySmall04 => "[SP]",
        SkullCompass | SkullKeyBig | SkullKeySmall01 | SkullKeySmall02 | SkullKeySmall03 => "[SW]",
        ThievesCompass | ThievesKeyBig | ThievesKeySmall => "[TH]",
        IceCompass | IceKeyBig | IceKeySmall01 | IceKeySmall02 | IceKeySmall03 => "[IR]",
        DesertCompass | DesertKeyBig | DesertKeySmall01 | DesertKeySmall02 | DesertKeySmall03 | DesertKeySmall04 | DesertKeySmall05 => "[DP]",
        TurtleCompass | TurtleKeyBig | TurtleKeySmall01 | TurtleKeySmall02 | TurtleKeySmall03 => "[TR]",

        LoruleCastleCompass | LoruleCastleKeySmall01 | LoruleCastleKeySmall02 | LoruleCastleKeySmall03 | LoruleCastleKeySmall04 | LoruleCastleKeySmall05 => "[LC]",

        _ => { panic!("Item {:?} is not a dungeon item", item); }
    })).cloned().collect()
}

fn exist_empty_reachable_check(checks: &Vec<Check>, check_map: &HashMap<&str, Option<FillerItem>>) -> bool {
    for check in checks {
        match check_map.get(check.get_name()).unwrap() {
            None => { return true; }
            Some(_) => {}
        }
    }

    false
}

/// Prefills a map with all checks as defined by the world graph with no values yet assigned
fn prefill_check_map(world_graph: &mut HashMap<Location, LocationNode>) -> HashMap<&'static str, Option<FillerItem>> {
    let mut check_map = HashMap::new();

    for (_, location_node) in world_graph {
        for check in location_node.clone().get_checks() {
            if check_map.insert(check.get_name(), match check.get_quest() {
                None => None,
                Some(quest) => Some(quest) // Quest items are static so just set them right away
            }).is_some() {
                error!("Multiple checks have duplicate name: {}", check.get_name());
                exit(1);
            }
        }
    }

    check_map
}

/// This translation is probably adding unnecessary overhead, oh well
fn build_progress_from_items(items: &Vec<FillerItem>) -> Progress {
    let mut progress = Progress::new();
    for item in items {
        progress.add_item(*item);
    }

    progress
}

fn verify_all_locations_accessible(loc_map: &mut HashMap<Location, LocationNode>) {
    let mut check_map = prefill_check_map(loc_map);

    let reachable_checks = assumed_search(loc_map, &get_progression_items(), &mut check_map); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    const TOTAL_CHECKS: usize = 267; // all checks + quest checks
    if reachable_checks.len() != TOTAL_CHECKS {

        // for rc in &reachable_checks {
        //     info!("Reachable Check: {}", rc.get_name());
        // }

        error!("Only {}/{} checks were reachable in the world graph", reachable_checks.len(), TOTAL_CHECKS);
        exit(1);
    }
}

/// Find all checks reachable with the given Progress
fn find_reachable_checks(loc_map: &mut HashMap<Location, LocationNode>, progress: &Progress, check_map: &mut HashMap<&str, Option<FillerItem>>) -> Vec<Check> {
    let start_node = Location::RavioShop;
    let mut loc_queue: Queue<Location> = Queue::from(vec![start_node]);
    let mut visited: HashSet<Location> = HashSet::new();
    let mut reachable_checks: Vec<Check> = Vec::new(); // possibly switch to HashSet to avoid dupes

    visited.insert(start_node);

    while !loc_queue.is_empty() {
        let location = loc_queue.dequeue().unwrap();

        // Grab the location from the map, verify it is defined
        let location_node = match loc_map.get_mut(&location) {
            Some(loc) => loc,
            None => {
                info!("Location Undefined: {:?}", location);
                exit(1);
            }
        };

        // Iterate over the location's checks
        for check in location_node.clone().get_checks() {
            if check.can_access(progress) {
                reachable_checks.push(check);
            }
        }

        // Queue new paths reachable from this location
        for path in location_node.clone().get_paths() {
            let destination = path.get_destination();
            if !visited.contains(&destination) && path.can_travel(progress) {
                loc_queue.queue(destination).expect("TODO: panic message");
                visited.insert(destination);
            }
        }
    }

    reachable_checks
}

fn get_items_from_reachable_checks(reachable_checks: &Vec<Check>, check_map: &mut HashMap<&str, Option<FillerItem>>) -> Progress {
    let mut progress = Progress::new();

    for check in reachable_checks {

        // Items already placed in the world that can be picked up
        let placed_item = check_map.get(check.get_name()).unwrap();
        match placed_item {
            None => {}
            Some(item) => progress.add_item(*item)
        }

        // Quest items that will always be at a given check
        match check.get_quest() {
            None => {}
            Some(quest) => { progress.add_item(quest) }
        }
    }

    progress
}

fn count_empty_checks(reachable_checks: &Vec<Check>, check_map: &HashMap<&str, Option<FillerItem>>) {
    let mut count = 0;
    for reachable_check in reachable_checks {
        if check_map.get(&reachable_check.get_name()).unwrap().is_none() {
            count += 1;
        }
    }

    info!("Empty checks: {}", count);
}

fn assumed_fill(mut world_graph: &mut HashMap<Location, LocationNode>, mut rng: &mut StdRng) -> HashMap<&'static str, Option<FillerItem>> {
    let mut check_map = prefill_check_map(&mut world_graph);

    let mut items_owned = get_progression_items();
    //let mut items_not_owned: Vec<Item> = Vec::new();
    let mut reachable_checks = assumed_search(&mut world_graph, &items_owned, &mut check_map);

    //info!("Reachable checks in assumed_fill: {}", reachable_checks.len());


    let mut placed_items = 0;

    //
    while exist_empty_reachable_check(&reachable_checks, &check_map) && !items_owned.is_empty() {
        let item = items_owned.remove(rng.gen_range(0..items_owned.len()));
        reachable_checks = assumed_search(&mut world_graph, &items_owned, &mut check_map);

        //info!("1st");
        //count_empty_checks(&reachable_checks, &check_map);

        let mut filtered_checks = filter_empty_checks(&mut reachable_checks, &mut check_map);

        //info!("2nd");
        //count_empty_checks(&reachable_checks, &check_map);

        if is_dungeon_item(item) {
            filtered_checks = filter_dungeon_checks(item, &mut filtered_checks);

            //info!("Filtered Reachable Checks: {}", reachable_checks.len());
            // for reachable_check in &reachable_checks {
            //     info!("Dungeon Reachable Check: {}", reachable_check.get_name());
            // }
        }

        if filtered_checks.len() == 0 {
            info!("No reachable checks found to place: {:?}", item);
        }

        place_item_randomly(item, &filtered_checks, &mut check_map, &mut rng);
        placed_items += 1;
    }

    info!("Conditions: 1:{}, 2:{}", exist_empty_reachable_check(&reachable_checks, &check_map), !items_owned.is_empty());

    info!("Placed Items: {}", placed_items);

    check_map
}

fn assumed_search(loc_map: &mut HashMap<Location, LocationNode>, items_owned: &Vec<FillerItem>, mut check_map: &mut HashMap<&str, Option<FillerItem>>) -> Vec<Check> {
    let mut current_items = build_progress_from_items(&items_owned.clone());
    let mut reachable_checks: Vec<Check>;

    loop {
        reachable_checks = find_reachable_checks(loc_map, &current_items, &mut check_map);
        let reachable_items = get_items_from_reachable_checks(&reachable_checks, &mut check_map);

        let new_items = reachable_items.difference(&current_items);

        if new_items.is_empty() {
            break;
        }

        for new_item in new_items {
            //info!("New item from search: {:?}", new_item);
            current_items.add_item(new_item);
        }
    }

    reachable_checks
}










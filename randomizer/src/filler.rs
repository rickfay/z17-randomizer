use std::collections::{HashMap, HashSet};
use std::process::exit;
use log::{error, info};
use queue::Queue;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use albw::Item;
use crate::{convert, LocationInfo, Seed, Settings};
use crate::check::Check;
use crate::FillerItem;
use crate::FillerItem::*;
use crate::location::Location;
use crate::location_node::LocationNode;
use crate::progress::Progress;
use crate::world::build_world_graph;

/// Filler Algorithm
pub fn fill_stuff(settings: &Settings, seed: Seed) -> Vec<(LocationInfo, Item)> {


    info!("Seed:                           {}", seed);
    //info!("Hash:                           {}", settings.hash().0);
    info!("Logic:                          Normal"); // if settings.logic.glitched_logic {"Glitched"} else {"Normal"});
    info!("Swords:                         {}", if settings.logic.swordless_mode {"Swordless Mode - No Swords"} else {"Normal"});
    info!("Super Items:                    {}", if settings.logic.super_items {"Included"} else {"Not Included"});
    info!("Trials:                         {}\n", if settings.logic.skip_trials {"Skipped"} else {"Normal"});


    let mut rng = StdRng::seed_from_u64(seed as u64);

    info!("Building World Graph...");

    let mut world_graph = build_world_graph();
    let mut check_map = prefill_check_map(&mut world_graph);
    let (mut progression_pool, mut trash_pool) = get_items(settings);

    verify_all_locations_accessible(&mut world_graph, &progression_pool, settings);

    preplace_items(&mut check_map, settings, &mut rng, &mut progression_pool, &mut trash_pool);

    info!("Placing Progression Items...");

    assumed_fill(&mut world_graph, &mut rng, &mut progression_pool, &mut check_map, settings);

    info!("Placing Trash Items...");

    fill_trash(&mut check_map, &mut rng, &trash_pool);

    map_to_result(world_graph, check_map)
}

/// Place static items ahead of the randomly filled ones
fn preplace_items<'a>(check_map: &mut HashMap<&'a str, Option<FillerItem>>,
                      settings: &'a Settings,
                      rng: &mut StdRng,
                      progression: &mut Vec<FillerItem>,
                      trash: &mut Vec<FillerItem>) {

    handle_exclusions(check_map, settings, rng, trash);

    check_map.insert("Shore", Some(LetterInABottle));
    progression.retain(|x| *x != LetterInABottle);

    check_map.insert("Merchant (Right)", Some(SmoothGem));
    progression.retain(|x| *x != SmoothGem);

    let mut shop_positions: Vec<&str> = Vec::new();
    let mut lorule_castle_positions: Vec<&str> = Vec::new();

    // Shut up your code is worse
    let mut zelda_excluded = false;
    let opt = settings.exclusions.0.get("exclusions");
    if opt.is_some() {
        zelda_excluded = opt.unwrap().contains("[LC] Zelda");
    }

    for (check_name, _) in check_map.clone() {
        if check_name.starts_with("[LC]") &&
            !(zelda_excluded && check_name.eq("[LC] Zelda")) { // gross
            let _ = &lorule_castle_positions.push(check_name);
        } else if check_name.starts_with("Ravio") && !check_name.contains("6") {
            let _ = &shop_positions.push(check_name);
        }
    }

    if settings.logic.bow_of_light_in_castle {
        check_map.insert(lorule_castle_positions.remove(rng.gen_range(0..lorule_castle_positions.len())), Some(BowOfLight));
        progression.retain(|x| *x != BowOfLight);
    }

    // Assured weapon takes away too much from progression, sticking with assured sword
    // if settings.logic.assured_weapon {
    //     let weapon = *Vec::from([
    //         Sword01, Sword02, Sword03, Sword04, Bow01, Bombs01, FireRod01, IceRod01, Hammer01
    //     ]).get(rng.gen_range(0..9)).unwrap();
    //
    //     check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(weapon));
    //     progression.retain(|x| *x != weapon);
    // }

    if settings.logic.sword_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Sword01));
        progression.retain(|x| *x != Sword01);
    }

    if settings.logic.bell_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Bell));
        progression.retain(|x| *x != Bell);
    }

    if settings.logic.pouch_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Pouch));
        progression.retain(|x| *x != Pouch);
    }

    if settings.logic.boots_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(PegasusBoots));
        progression.retain(|x| *x != PegasusBoots);
    }

    if settings.logic.minigames_excluded {
        exclude("Cucco Ranch", rng, check_map, trash);
        exclude("Hyrule Hotfoot", rng, check_map, trash);
        exclude("Rupee Rush (Hyrule)", rng, check_map, trash);
        exclude("Rupee Rush (Lorule)", rng, check_map, trash);
        exclude("Octoball Derby", rng, check_map, trash);
        exclude("Treacherous Tower (Intermediate)", rng, check_map, trash);
    }
}

fn exclude(check_name: &'static str, rng: &mut StdRng, check_map: &mut HashMap<&str, Option<FillerItem>>, trash: &mut Vec<FillerItem>) {
    check_map.insert(check_name, Some(trash.remove(rng.gen_range(0..trash.len()))));
}


fn handle_exclusions<'a>(check_map: &mut HashMap<&'a str, Option<FillerItem>>,
                         settings: &'a Settings,
                         rng: &mut StdRng,
                         trash_pool: &mut Vec<FillerItem>) {

    let opt = settings.exclusions.0.get("exclusions");
    if opt.is_none() {
        return;
    }

    let exclusions = opt.unwrap();

    for exclusion in exclusions {
        if check_map.contains_key(&exclusion.as_str()) {
            check_map.insert(&exclusion.as_str(), Some(trash_pool.remove(rng.gen_range(0..trash_pool.len()))));
        } else {
            error!("Cannot exclude \"{}\", no matching check found with that name.", &exclusion.as_str());
            error!("Consult a spoiler log for a list of valid check names.");
            exit(1);
        }
    }
}

/// Super dirty mapping I hate it
fn map_to_result(world_graph: HashMap<Location, LocationNode>, check_map: HashMap<&str, Option<FillerItem>>) -> Vec<(LocationInfo, Item)> {
    let mut result: Vec<(LocationInfo, Item)> = Vec::new();
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

fn get_items(settings: &Settings) -> (Vec<FillerItem>, Vec<FillerItem>) {
    let mut progression =
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
            RaviosBracelet02,
            Bell,
            StaminaScroll,
            BowOfLight,
            PegasusBoots,
            Flippers,
            HylianShield,
            PremiumMilk,
            SmoothGem,
            LetterInABottle,
            Lamp01,
            Net01,
            Pouch,

            // 5 Bottles
            Bottle01,
            Bottle02,
            Bottle03,
            Bottle04,
            Bottle05,

            // 2 Gloves
            Glove01,
            Glove02,

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
        ];


    let mut trash = vec![
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
        //RupeeRed, // Removed for ????
        //RupeeRed, // Removed for 2nd Bracelet
        //RupeeRed, // Irene Removed

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
    ];

    // Swordless Mode
    if settings.logic.swordless_mode {
        trash.push(RupeeRed);
        trash.push(RupeeRed);
        trash.push(RupeeRed);
        trash.push(RupeeRed);
    } else {
        progression.push(Sword01);
        progression.push(Sword02);
        progression.push(Sword03);
        progression.push(Sword04);
    }

    // Super Items
    if settings.logic.super_items {
        progression.push(Lamp02);
        progression.push(Net02);
    } else {
        trash.push(RupeePurple);
        trash.push(RupeePurple);
    }

    (progression, trash)
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

fn fill_trash(check_map: &mut HashMap<&str, Option<FillerItem>>, rng: &mut StdRng, trash_items: &Vec<FillerItem>) {
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
        check_map.insert(empty_check_keys.remove(rng.gen_range(0..empty_check_keys.len())), Some(*trash));
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
fn build_progress_from_items(items: &Vec<FillerItem>, settings: &Settings) -> Progress {
    let mut progress = Progress::new(settings.clone());
    for item in items {
        progress.add_item(*item);
    }

    progress
}

fn verify_all_locations_accessible(loc_map: &mut HashMap<Location, LocationNode>,
                                   progression_pool: &Vec<FillerItem>,
                                   settings: &Settings) {
    info!("Verifying all locations accessible...");

    let mut check_map = prefill_check_map(loc_map);

    let reachable_checks = assumed_search(loc_map, progression_pool, &mut check_map, settings); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    let total_checks: usize = if settings.logic.swordless_mode { 267 } else { 269 }; // all checks + quest checks
    if reachable_checks.len() != total_checks {

        // for rc in &reachable_checks {
        //     info!("Reachable Check: {}", rc.get_name());
        // }

        error!("Only {}/{} checks were reachable in the world graph", reachable_checks.len(), total_checks);
        exit(1);
    }
}

/// Find all checks reachable with the given Progress
fn find_reachable_checks(loc_map: &mut HashMap<Location, LocationNode>, progress: &Progress) -> Vec<Check> {
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

fn get_items_from_reachable_checks(reachable_checks: &Vec<Check>,
                                   check_map: &mut HashMap<&str, Option<FillerItem>>,
                                   settings: &Settings) -> Progress {
    let mut progress = Progress::new(settings.clone());

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

fn assumed_fill(mut world_graph: &mut HashMap<Location, LocationNode>,
                mut rng: &mut StdRng,
                items_owned: &mut Vec<FillerItem>,
                mut check_map: &mut HashMap<&str, Option<FillerItem>>,
                settings: &Settings) {
    let mut reachable_checks = assumed_search(&mut world_graph, &items_owned, &mut check_map, settings);

    while exist_empty_reachable_check(&reachable_checks, &check_map) && !items_owned.is_empty() {
        let item = items_owned.remove(rng.gen_range(0..items_owned.len()));
        reachable_checks = assumed_search(&mut world_graph, &items_owned, &mut check_map, settings);


        let mut filtered_checks = filter_empty_checks(&mut reachable_checks, &mut check_map);

        // Filter reachable locations for dungeons items down to just their dungeon
        if is_dungeon_item(item) {
            filtered_checks = filter_dungeon_checks(item, &mut filtered_checks);
        }

        if filtered_checks.len() == 0 {
            info!("No reachable checks found to place: {:?}", item);
        }

        place_item_randomly(item, &filtered_checks, &mut check_map, &mut rng);
    }
}

fn assumed_search(loc_map: &mut HashMap<Location, LocationNode>,
                  items_owned: &Vec<FillerItem>,
                  mut check_map: &mut HashMap<&str, Option<FillerItem>>,
                  settings: &Settings) -> Vec<Check> {
    let mut current_items = build_progress_from_items(&items_owned.clone(), settings);
    let mut reachable_checks: Vec<Check>;

    loop {
        reachable_checks = find_reachable_checks(loc_map, &current_items);
        let reachable_items = get_items_from_reachable_checks(&reachable_checks, &mut check_map, settings);

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










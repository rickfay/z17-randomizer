use {
    crate::{
        convert, fail,
        filler_util::shuffle,
        item_pools::{get_item_pools, get_maiamai_pool},
        model::{
            check::Check, location::Location, location_node::LocationNode, progress::Progress,
        },
        patch::util::is_sage,
        settings::logic_mode::LogicMode::*,
        world::build_world_graph,
        FillerItem::{self, *},
        LocationInfo, Metrics, Seed, Settings,
    },
    albw::Item,
    log::{error, info},
    queue::Queue,
    rand::{rngs::StdRng, Rng, SeedableRng},
    std::collections::{BTreeMap, HashMap, HashSet},
};

/// Filler Algorithm
pub fn fill_stuff(settings: &Settings, seed: Seed) -> (Vec<(LocationInfo, Item)>, Metrics) {
    settings.log(seed);
    prevalidate(settings);

    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut world_graph = build_world_graph();
    let mut check_map = prefill_check_map(&mut world_graph);
    let (mut progression_pool, mut junk_pool) = get_item_pools(settings, &mut rng);

    verify_all_locations_accessible(&mut world_graph, &progression_pool, settings);
    handle_exclusions(&mut check_map, settings, &mut rng, &mut junk_pool);
    preplace_items(&mut check_map, settings, &mut rng, &mut progression_pool, &mut junk_pool);
    assumed_fill(&mut world_graph, &mut rng, &mut progression_pool, &mut check_map, settings);
    fill_junk(&mut check_map, &mut rng, &junk_pool);

    let metrics = calculate_metrics(&mut world_graph, &mut check_map, settings, &mut rng);

    (map_to_result(world_graph, check_map), metrics)
}

fn prevalidate(settings: &Settings) {
    // LC Requirement
    if !(0..=7).contains(&settings.logic.lc_requirement) {
        fail!("Invalid LC Requirement: {}\nExiting...", settings.logic.lc_requirement);
    }

    // Yuganon Requirement
    if !(0..=7).contains(&settings.logic.yuganon_requirement) {
        fail!("Invalid Yuga Ganon Requirement: {}\nExiting...", settings.logic.yuganon_requirement);
    }
}

/// Place static items ahead of the randomly filled ones
fn preplace_items<'a>(
    check_map: &mut HashMap<&'a str, Option<FillerItem>>, settings: &'a Settings, rng: &mut StdRng,
    progression: &mut Vec<FillerItem>, junk: &mut Vec<FillerItem>,
) {
    // Vanilla Dungeon Prizes
    if !settings.logic.randomize_dungeon_prizes {
        place_static(check_map, progression, PendantOfCourage01, "Eastern Palace Prize");
        place_static(check_map, progression, PendantOfWisdom, "House of Gales Prize");
        place_static(check_map, progression, PendantOfPower, "Tower of Hera Prize");
        place_static(check_map, progression, PendantOfCourage02, "Hyrule Castle Prize");
        place_static(check_map, progression, SageGulley, "Dark Palace Prize");
        place_static(check_map, progression, SageOren, "Swamp Palace Prize");
        place_static(check_map, progression, SageSeres, "Skull Woods Prize");
        place_static(check_map, progression, SageOsfala, "Thieves' Hideout Prize");
        place_static(check_map, progression, SageImpa, "Turtle Rock Prize");
        place_static(check_map, progression, SageIrene, "Desert Palace Prize");
        place_static(check_map, progression, SageRosso, "Ice Ruins Prize");
    }

    // Place un-randomized items
    place_static(check_map, progression, LetterInABottle, "Southeastern Shore");
    place_static(check_map, progression, RupeeSilver40, "Hyrule Hotfoot (Second Race)");
    place_static(check_map, progression, RupeeSilver41, "[TR] (1F) Under Center");
    place_static(check_map, progression, RupeeGold09, "[TR] (B1) Under Center");
    place_static(check_map, progression, RupeeGold10, "[PD] (2F) South Hidden Room");
    place_static(check_map, progression, HeartPiece28, "Fortune's Choice");

    // Kakariko Item Shop
    place_static(check_map, progression, ScootFruit01, "Kakariko Item Shop (1)");
    place_static(check_map, progression, FoulFruit01, "Kakariko Item Shop (2)");
    place_static(check_map, progression, Shield01, "Kakariko Item Shop (3)");

    // Lakeside Item Shop
    place_static(check_map, progression, ScootFruit02, "Lakeside Item Shop (1)");
    place_static(check_map, progression, FoulFruit02, "Lakeside Item Shop (2)");
    place_static(check_map, progression, Shield02, "Lakeside Item Shop (3)");

    // Mysterious Man
    place_static(check_map, progression, GoldBee01, "Mysterious Man");

    // Thieves' Town Item Shop
    place_static(check_map, progression, Bee01, "Thieves' Town Item Shop (1)");
    place_static(check_map, progression, GoldBee02, "Thieves' Town Item Shop (2)");
    place_static(check_map, progression, Fairy01, "Thieves' Town Item Shop (3)");
    place_static(check_map, progression, Shield03, "Thieves' Town Item Shop (4)");

    // Lorule Lake Item Shop
    place_static(check_map, progression, Bee02, "Lorule Lakeside Item Shop (1)");
    place_static(check_map, progression, GoldBee03, "Lorule Lakeside Item Shop (2)");
    place_static(check_map, progression, Fairy02, "Lorule Lakeside Item Shop (3)");
    place_static(check_map, progression, Shield04, "Lorule Lakeside Item Shop (4)");

    // Super Items
    if settings.logic.super_items {
        exclude("Treacherous Tower Advanced (1)", rng, check_map, junk);
        exclude("Treacherous Tower Advanced (2)", rng, check_map, junk);
    } else {
        place_static(check_map, progression, Lamp02, "Treacherous Tower Advanced (1)");
        place_static(check_map, progression, Net02, "Treacherous Tower Advanced (2)");
    }

    // Nice Mode
    if settings.logic.nice_mode {
        exclude(" 10 Maiamai", rng, check_map, junk);
        exclude(" 20 Maiamai", rng, check_map, junk);
        exclude(" 30 Maiamai", rng, check_map, junk);
        exclude(" 40 Maiamai", rng, check_map, junk);
        exclude(" 50 Maiamai", rng, check_map, junk);
        exclude(" 60 Maiamai", rng, check_map, junk);
        exclude(" 70 Maiamai", rng, check_map, junk);
        exclude(" 80 Maiamai", rng, check_map, junk);
        exclude(" 90 Maiamai", rng, check_map, junk);
    } else {
        place_static(check_map, progression, Bow02, " 10 Maiamai");
        place_static(check_map, progression, Boomerang02, " 20 Maiamai");
        place_static(check_map, progression, Hookshot02, " 30 Maiamai");
        place_static(check_map, progression, Hammer02, " 40 Maiamai");
        place_static(check_map, progression, Bombs02, " 50 Maiamai");
        place_static(check_map, progression, FireRod02, " 60 Maiamai");
        place_static(check_map, progression, IceRod02, " 70 Maiamai");
        place_static(check_map, progression, TornadoRod02, " 80 Maiamai");
        place_static(check_map, progression, SandRod02, " 90 Maiamai");
    }
    exclude("100 Maiamai", rng, check_map, junk);

    let mut shop_positions: Vec<&str> = Vec::new();
    let mut bow_light_positions: Vec<&str> = Vec::from(["Final Zelda"]);
    let mut maiamai_positions: Vec<&str> = Vec::new();

    for (check_name, item) in check_map.clone() {
        if check_name.starts_with("[LC]") && item.is_none() {
            let _ = &bow_light_positions.push(check_name);
        } else if check_name.starts_with("Ravio") && !check_name.contains("6") {
            let _ = &shop_positions.push(check_name);
        } else if check_name.starts_with("[Mai]") {
            let _ = &maiamai_positions.push(check_name);
        }
    }

    if settings.logic.bow_of_light_in_castle {
        check_map.insert(
            bow_light_positions.remove(rng.gen_range(0..bow_light_positions.len())),
            Some(BowOfLight),
        );
        progression.retain(|x| *x != BowOfLight);
    }

    // Assures a weapon will be available in Ravio's Shop
    if settings.logic.assured_weapon {
        let mut weapons = Vec::from([Bow01, Bombs01, FireRod01, IceRod01, Hammer01]);

        if !settings.logic.swordless_mode {
            weapons.extend_from_slice(&[Sword01]);
        }

        match settings.logic.mode {
            Normal => {}
            _ => {
                weapons.extend_from_slice(&[Lamp01, Net01]);
            }
        }

        let weapon = *weapons.get(rng.gen_range(0..weapons.len())).unwrap();

        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(weapon));
        progression.retain(|x| *x != weapon);
    }

    // Bell in Shop
    if settings.logic.bell_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Bell));
        progression.retain(|x| *x != Bell);
    }

    // Pouch in Shop
    if settings.logic.pouch_in_shop {
        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Pouch));
        progression.retain(|x| *x != Pouch);
    }

    // Boots in Shop
    if settings.logic.boots_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(PegasusBoots),
        );
        progression.retain(|x| *x != PegasusBoots);
    }

    // Exclude Minigames
    if settings.logic.minigames_excluded {
        exclude("Dodge the Cuccos", rng, check_map, junk);
        exclude("Hyrule Hotfoot (First Race)", rng, check_map, junk);
        exclude("Rupee Rush (Hyrule)", rng, check_map, junk);
        exclude("Rupee Rush (Lorule)", rng, check_map, junk);
        exclude("Octoball Derby", rng, check_map, junk);
        exclude("Treacherous Tower Intermediate", rng, check_map, junk);

        // For Maiamai Madness, also turn the rupee rush maiamai into random junk
        if settings.logic.maiamai_madness {
            exclude("[Mai] Hyrule Rupee Rush Wall", rng, check_map, junk);
            exclude("[Mai] Lorule Rupee Rush Wall", rng, check_map, junk);
        }
    }

    // For non-Maiamai Madness seeds, default them to Maiamai
    // FIXME Inefficient to add Maiamai to progression pool, shuffle, then remove them
    if !settings.logic.maiamai_madness {
        let mut maiamai_items = get_maiamai_pool();
        for check_name in maiamai_positions {
            place_static(check_map, progression, maiamai_items.remove(0), check_name);
        }
    }
}

// Statically place an item in a give location, then remove it from the item pool provided
fn place_static<'a>(
    check_map: &mut HashMap<&'a str, Option<FillerItem>>, pool: &mut Vec<FillerItem>,
    item: FillerItem, check_name: &'a str,
) {
    check_map.insert(check_name, Some(item));
    pool.retain(|x| *x != item);
}

// Exclude a location by placing a random junk item there
fn exclude(
    check_name: &'static str, rng: &mut StdRng, check_map: &mut HashMap<&str, Option<FillerItem>>,
    junk: &mut Vec<FillerItem>,
) {
    if check_map.insert(check_name, Some(junk.remove(rng.gen_range(0..junk.len())))).is_none() {
        fail!("Check not found: {}", check_name);
    }
}

fn handle_exclusions<'a>(
    check_map: &mut HashMap<&'a str, Option<FillerItem>>, settings: &'a Settings, rng: &mut StdRng,
    junk_pool: &mut Vec<FillerItem>,
) {
    let opt = settings.exclusions.0.get("exclusions");
    if opt.is_none() {
        return;
    }

    let exclusions = opt.unwrap();

    for exclusion in exclusions {
        if check_map.contains_key(&exclusion.as_str()) {
            check_map.insert(
                &exclusion.as_str(),
                Some(junk_pool.remove(rng.gen_range(0..junk_pool.len()))),
            );
        } else {
            error!(
                "Cannot exclude \"{}\", no matching check found with that name.",
                &exclusion.as_str()
            );
            fail!("Consult a spoiler log for a list of valid check names.");
        }
    }
}

/// Super dirty mapping I hate it
fn map_to_result(
    world_graph: HashMap<Location, LocationNode>, check_map: HashMap<&str, Option<FillerItem>>,
) -> Vec<(LocationInfo, Item)> {
    let mut result: Vec<(LocationInfo, Item)> = Vec::new();
    for (_, location_node) in world_graph {
        for check in location_node.get_checks() {
            if let Some(loc_info) = check.get_location_info() {
                result.push((
                    loc_info,
                    convert(check_map.get(check.get_name()).unwrap().unwrap()).unwrap(),
                ));
            }
        }
    }
    result
}

fn is_dungeon_prize(item: FillerItem) -> bool {
    match item {
        PendantOfPower | PendantOfWisdom | PendantOfCourage01 | PendantOfCourage02 | SageGulley
        | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso => true,
        _ => false,
    }
}

fn is_dungeon_item(item: FillerItem) -> bool {
    match item {
        HyruleSanctuaryKey
        | LoruleSanctuaryKey
        | EasternCompass
        | EasternKeyBig
        | EasternKeySmall01
        | EasternKeySmall02
        | GalesCompass
        | GalesKeyBig
        | GalesKeySmall01
        | GalesKeySmall02
        | GalesKeySmall03
        | GalesKeySmall04
        | HeraCompass
        | HeraKeyBig
        | HeraKeySmall01
        | HeraKeySmall02
        | DarkCompass
        | DarkKeyBig
        | DarkKeySmall01
        | DarkKeySmall02
        | DarkKeySmall03
        | DarkKeySmall04
        | SwampCompass
        | SwampKeyBig
        | SwampKeySmall01
        | SwampKeySmall02
        | SwampKeySmall03
        | SwampKeySmall04
        | SkullCompass
        | SkullKeyBig
        | SkullKeySmall01
        | SkullKeySmall02
        | SkullKeySmall03
        | ThievesCompass
        | ThievesKeyBig
        | ThievesKeySmall
        | IceCompass
        | IceKeyBig
        | IceKeySmall01
        | IceKeySmall02
        | IceKeySmall03
        | DesertCompass
        | DesertKeyBig
        | DesertKeySmall01
        | DesertKeySmall02
        | DesertKeySmall03
        | DesertKeySmall04
        | DesertKeySmall05
        | TurtleCompass
        | TurtleKeyBig
        | TurtleKeySmall01
        | TurtleKeySmall02
        | TurtleKeySmall03
        | LoruleCastleCompass
        | LoruleCastleKeySmall01
        | LoruleCastleKeySmall02
        | LoruleCastleKeySmall03
        | LoruleCastleKeySmall04
        | LoruleCastleKeySmall05 => true,
        _ => false,
    }
}

fn fill_junk(
    check_map: &mut HashMap<&str, Option<FillerItem>>, rng: &mut StdRng,
    junk_items: &Vec<FillerItem>,
) {
    info!("Placing Junk Items...");

    let mut empty_check_keys = Vec::new();
    for (key, val) in check_map.clone() {
        if val.is_none() {
            empty_check_keys.push(key);
        }
    }

    if empty_check_keys.len() != junk_items.len() {
        println!();

        for key in &empty_check_keys {
            info!("Empty Check: {}", key);
        }

        println!();

        for key in junk_items {
            info!("Junk: {}", convert(*key).unwrap().as_str());
        }

        fail!(
            "Number of empty checks: {} does not match available junk items: {}",
            empty_check_keys.len(),
            junk_items.len()
        );
    }

    for junk in junk_items {
        check_map
            .insert(empty_check_keys.remove(rng.gen_range(0..empty_check_keys.len())), Some(*junk));
    }
}

fn place_item_randomly(
    item: FillerItem, checks: &Vec<Check>, check_map: &mut HashMap<&str, Option<FillerItem>>,
    rng: &mut StdRng,
) {
    let index = rng.gen_range(0..checks.len());
    check_map.insert(checks.get(index).unwrap().get_name(), Some(item));
}

fn filter_checks(
    item: FillerItem, checks: &mut Vec<Check>, check_map: &mut HashMap<&str, Option<FillerItem>>,
) -> Vec<Check> {
    // Filter out non-empty checks
    let mut filtered_checks = checks
        .iter()
        .filter(|&x| check_map.get(x.get_name()).unwrap().is_none())
        .cloned()
        .collect();

    // Filter checks by item type
    if is_dungeon_prize(item) {
        filtered_checks = filter_dungeon_prize_checks(&mut filtered_checks);
    } else if is_dungeon_item(item) {
        let is_keysanity = false; // No keysanity yet, hardcode to false
        if !is_keysanity {
            filtered_checks = filter_dungeon_checks(item, &mut filtered_checks);
        }
    }

    filtered_checks
}

fn filter_dungeon_prize_checks(eligible_checks: &mut Vec<Check>) -> Vec<Check> {
    eligible_checks.iter().filter(|&x| x.get_name().contains("Prize")).cloned().collect()
}

fn filter_dungeon_checks(item: FillerItem, eligible_checks: &mut Vec<Check>) -> Vec<Check> {
    eligible_checks
        .iter()
        .filter(|&x| {
            x.get_name().starts_with(match item {
                HyruleSanctuaryKey => "[HS]",
                LoruleSanctuaryKey => "[LS]",
                EasternCompass | EasternKeyBig | EasternKeySmall01 | EasternKeySmall02 => "[EP]",
                GalesCompass | GalesKeyBig | GalesKeySmall01 | GalesKeySmall02
                | GalesKeySmall03 | GalesKeySmall04 => "[HG]",
                HeraCompass | HeraKeyBig | HeraKeySmall01 | HeraKeySmall02 => "[TH]",
                DarkCompass | DarkKeyBig | DarkKeySmall01 | DarkKeySmall02 | DarkKeySmall03
                | DarkKeySmall04 => "[PD]",
                SwampCompass | SwampKeyBig | SwampKeySmall01 | SwampKeySmall02
                | SwampKeySmall03 | SwampKeySmall04 => "[SP]",
                SkullCompass | SkullKeyBig | SkullKeySmall01 | SkullKeySmall02
                | SkullKeySmall03 => "[SW]",
                ThievesCompass | ThievesKeyBig | ThievesKeySmall => "[T'H]",
                IceCompass | IceKeyBig | IceKeySmall01 | IceKeySmall02 | IceKeySmall03 => "[IR]",
                DesertCompass | DesertKeyBig | DesertKeySmall01 | DesertKeySmall02
                | DesertKeySmall03 | DesertKeySmall04 | DesertKeySmall05 => "[DP]",
                TurtleCompass | TurtleKeyBig | TurtleKeySmall01 | TurtleKeySmall02
                | TurtleKeySmall03 => "[TR]",
                LoruleCastleCompass
                | LoruleCastleKeySmall01
                | LoruleCastleKeySmall02
                | LoruleCastleKeySmall03
                | LoruleCastleKeySmall04
                | LoruleCastleKeySmall05 => "[LC]",

                _ => {
                    fail!("Item {:?} is not a dungeon item", item);
                }
            })
        })
        .cloned()
        .collect()
}

fn exist_empty_reachable_check(
    checks: &Vec<Check>, check_map: &HashMap<&str, Option<FillerItem>>,
) -> bool {
    for check in checks {
        match check_map.get(check.get_name()).unwrap() {
            None => {
                return true;
            }
            Some(_) => {}
        }
    }

    false
}

/// Prefills a map with all checks as defined by the world graph with no values yet assigned
fn prefill_check_map(
    world_graph: &mut HashMap<Location, LocationNode>,
) -> HashMap<&'static str, Option<FillerItem>> {
    let mut check_map = HashMap::new();

    for (_, location_node) in world_graph {
        for check in location_node.clone().get_checks() {
            if check_map
                .insert(check.get_name(), match check.get_quest() {
                    None => None,
                    Some(quest) => Some(quest), // Quest items are static so just set them right away
                })
                .is_some()
            {
                fail!("Multiple checks have duplicate name: {}", check.get_name());
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

fn verify_all_locations_accessible(
    loc_map: &mut HashMap<Location, LocationNode>, progression_pool: &Vec<FillerItem>,
    settings: &Settings,
) {
    info!("Verifying all locations accessible...");

    let mut check_map = prefill_check_map(loc_map);

    let reachable_checks = assumed_search(loc_map, progression_pool, &mut check_map, settings); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    /**
     * 427 TOTAL CHECKS <br /><br />
     *
     * 413 are considered in logic:
     * - 254 Standard Checks
     * - 100 Maiamai
     * - 11 Dungeon Prizes
     * - 29 Quest (non-items that are still progression)
     * - 19 Statically Placed Items:
     *     - 12x Shop Items (not including 9,999 items)
     *     - 3x Obscure Gold/Silver Rupees
     *     - Mysterious Man
     *     - FIXME: Letter in a Bottle
     *     - FIXME: Hyrule Hotfoot Second Race
     *     - FIXME: Fortune's Choice
     */
    const TOTAL_CHECKS: usize = 427;

    /**
     * Checks intentionally considered unreachable:
     * - FIXME: 10 Maiamai Rewards
     * - 2 Golden Bees for 9,999 Rupees
     * - 2 Treacherous Tower Advanced
     */
    const UNREACHABLE_CHECKS: usize = 14;

    if reachable_checks.len() != TOTAL_CHECKS - UNREACHABLE_CHECKS {
        let reachable_check_names: Vec<&str> =
            reachable_checks.iter().map(|c| c.get_name()).collect();
        for (check, _) in &check_map {
            if !reachable_check_names.contains(check) {
                info!("Unreachable Check: {}", check);
            }
        }

        fail!(
            "Only {}/{} checks were reachable in the world graph",
            reachable_checks.len(),
            TOTAL_CHECKS - UNREACHABLE_CHECKS
        );
    }
}

/// Find all checks reachable with the given Progress
fn find_reachable_checks(
    loc_map: &mut HashMap<Location, LocationNode>, progress: &Progress,
) -> Vec<Check> {
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
                fail!("Location Undefined: {:?}", location);
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
            if !visited.contains(&destination) && path.can_access(progress) {
                loc_queue.queue(destination).expect("TODO: panic message");
                visited.insert(destination);
            }
        }
    }

    reachable_checks
}

fn get_items_from_reachable_checks(
    reachable_checks: &Vec<Check>, check_map: &mut HashMap<&str, Option<FillerItem>>,
    settings: &Settings,
) -> Progress {
    let mut progress = Progress::new(settings.clone());

    for check in reachable_checks {
        // Items already placed in the world that can be picked up
        let placed_item = check_map.get(check.get_name()).unwrap();
        match placed_item {
            None => {}
            Some(item) => progress.add_item(*item),
        }

        // Quest items that will always be at a given check
        match check.get_quest() {
            None => {}
            Some(quest) => progress.add_item(quest),
        }
    }

    progress
}

/// The Assumed Fill algorithm
///
/// Randomly places `items_owned` into the `check_map` in a completable manner as informed by the
/// logic defined in the `world_graph` and `settings`.
///
/// Items are placed "backwards", *assuming* that all items that have yet to be placed are
/// available without the item currently being placed.
///
/// An assumed search algorithm is used to identify all locations reachable without the item
/// currently being placed.
///
/// * `world_graph` - A graph representing the comprehensive structure of the game world
/// * `rng` - The RNG seed
/// * `items_owned` - The pool of all progression-granting items
/// * `check_map` - A map representing all checks and items assigned to them
/// * `settings` - Game settings
fn assumed_fill(
    mut world_graph: &mut HashMap<Location, LocationNode>, mut rng: &mut StdRng,
    items_owned: &mut Vec<FillerItem>, mut check_map: &mut HashMap<&str, Option<FillerItem>>,
    settings: &Settings,
) {
    info!("Placing Progression Items...");

    let mut reachable_checks =
        assumed_search(&mut world_graph, &items_owned, &mut check_map, settings);

    while exist_empty_reachable_check(&reachable_checks, &check_map) && !items_owned.is_empty() {
        let item = items_owned.remove(0);

        //
        reachable_checks = assumed_search(&mut world_graph, &items_owned, &mut check_map, settings);

        let filtered_checks = filter_checks(item, &mut reachable_checks, &mut check_map);

        if filtered_checks.len() == 0 {
            info!("No reachable checks found to place: {:?}", item);
        }

        place_item_randomly(item, &filtered_checks, &mut check_map, &mut rng);
    }
}

/// The Assumed Search algorithm.
///
/// Gets all reachable checks available with the `items_owned`, assuming all items yet to be
/// placed will be available.
///
/// A loop is performed to expand the considered items to include not just the `items_owned` but
/// also all items already placed that are reachable with the currently considered items, until
/// all such items have been exhausted.
///
fn assumed_search(
    loc_map: &mut HashMap<Location, LocationNode>, items_owned: &Vec<FillerItem>,
    mut check_map: &mut HashMap<&str, Option<FillerItem>>, settings: &Settings,
) -> Vec<Check> {
    let mut considered_items = build_progress_from_items(&items_owned.clone(), settings);
    let mut reachable_checks: Vec<Check>;

    loop {
        reachable_checks = find_reachable_checks(loc_map, &considered_items);
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, &mut check_map, settings);

        let new_items = reachable_items.difference(&considered_items);

        if new_items.is_empty() {
            break;
        }

        for new_item in new_items {
            considered_items.add_item(new_item);
        }
    }

    reachable_checks
}

fn calculate_metrics(
    world_graph: &mut HashMap<Location, LocationNode>,
    check_map: &mut HashMap<&str, Option<FillerItem>>, settings: &Settings, rng: &mut StdRng,
) -> Metrics {
    let playthrough = sphere_search(world_graph, check_map, settings);

    let hints = generate_path_hints(world_graph, check_map, settings, rng);

    Metrics::new(playthrough.len(), playthrough, hints)
}

/// Sphere Search
fn sphere_search<'a>(
    world_graph: &mut HashMap<Location, LocationNode>,
    mut check_map: &mut HashMap<&str, Option<FillerItem>>, settings: &Settings,
) -> BTreeMap<String, BTreeMap<&'static str, &'static str>> {
    let mut progress = Progress::new(settings.clone());
    let mut reachable_checks: Vec<Check>;
    let mut spheres = BTreeMap::new();
    let mut sphere_num = 0;

    loop {
        reachable_checks = find_reachable_checks(world_graph, &progress);
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, &mut check_map, settings);

        let new_items = reachable_items.difference(&progress);

        if new_items.is_empty() {
            break;
        }

        for new_item in &new_items {
            progress.add_item(*new_item);
        }

        let mut sphere = BTreeMap::new();
        for reachable_check in reachable_checks {
            let filler_item = check_map.get(reachable_check.get_name()).unwrap().unwrap();
            if new_items.contains(&filler_item) && filler_item.is_progression() {
                sphere.insert(reachable_check.get_name(), filler_item.as_str());
            }
        }
        if sphere.is_empty() {
            continue; // hide spheres with only minor progression items
        }

        spheres.insert(format!("Sphere {:02}", sphere_num), sphere);
        sphere_num += 1;
    }

    spheres
}

fn generate_path_hints(
    world_graph: &mut HashMap<Location, LocationNode>,
    check_map: &mut HashMap<&str, Option<FillerItem>>, settings: &Settings, rng: &mut StdRng,
) -> Vec<String> {
    let mut bosses_and_prize_locations: Vec<(FillerItem, &str)> = vec![
        (Yuga, "Eastern Palace Prize"),
        (Margomill, "House of Gales Prize"),
        (Moldorm, "Tower of Hera Prize"),
        (GemesaurKing, "Dark Palace Prize"),
        (Arrghus, "Swamp Palace Prize"),
        (Knucklemaster, "Skull Woods Prize"),
        (Stalblind, "Thieves' Hideout Prize"),
        (Grinexx, "Turtle Rock Prize"),
        (Zaganaga, "Desert Palace Prize"),
        (Dharkstare, "Ice Ruins Prize"),
    ];

    bosses_and_prize_locations = shuffle(bosses_and_prize_locations, rng);

    let mut taken_path_checks: Vec<Check> = Vec::new();
    let mut path_hints = Vec::new();
    for (boss, prize_loc) in bosses_and_prize_locations {
        if is_sage(convert(check_map.get(prize_loc).unwrap().unwrap()).unwrap()) {
            if let Some(path_check) =
                get_path_check(boss, world_graph, check_map, settings, rng, &taken_path_checks)
            {
                taken_path_checks.push(path_check);
                path_hints.push(format!(
                    "They say there's a link between {} and {}.",
                    path_check.get_location_info().unwrap().region(),
                    boss.as_str()
                ));
            }
        }
    }

    path_hints
}

fn get_path_check(
    boss: FillerItem, world_graph: &mut HashMap<Location, LocationNode>,
    check_map: &mut HashMap<&str, Option<FillerItem>>, settings: &Settings, rng: &mut StdRng,
    taken_path_checks: &Vec<Check>,
) -> Option<Check> {
    let path_checks = get_path_checks(boss, world_graph, check_map, settings, taken_path_checks);

    return if path_checks.is_empty() {
        None
    } else {
        Some(*path_checks.get(rng.gen_range(0..path_checks.len())).unwrap())
    };
}

/// Get Path Checks
fn get_path_checks<'a>(
    boss: FillerItem, world_graph: &mut HashMap<Location, LocationNode>,
    mut check_map: &mut HashMap<&str, Option<FillerItem>>, settings: &Settings,
    taken_path_checks: &Vec<Check>,
) -> Vec<Check> {
    let mut progress = Progress::new(settings.clone());
    let mut reachable_checks: Vec<Check>;
    let mut path_checks = Vec::new();

    let mut potential_path_checks = HashSet::new();

    // Find candidate Path Checks with a modified sphere search
    loop {
        reachable_checks = find_reachable_checks(world_graph, &progress);
        potential_path_checks.extend(&reachable_checks);
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, &mut check_map, settings);

        let new_items = reachable_items.difference(&progress);

        if new_items.is_empty() {
            fail!("No possible path to defeat {}", boss.as_str());
        }

        for new_item in &new_items {
            progress.add_item(*new_item);
        }

        if progress.has(boss) {
            break;
        }
    }

    // Limit potential paths to locations with valid Path Items that haven't yet been taken
    potential_path_checks.retain(|check| {
        !taken_path_checks.contains(check)
            && POSSIBLE_PATH_ITEMS.contains(&check_map.get(check.get_name()).unwrap().unwrap())
    });

    // Test candidate items to see if Boss can be defeated without them
    for check in potential_path_checks {
        // Reset Progression
        progress = Progress::new(settings.clone());

        loop {
            reachable_checks = find_reachable_checks(world_graph, &progress);

            // Remove Potential Path Location
            reachable_checks.retain(|c| check.ne(c));

            let reachable_items =
                get_items_from_reachable_checks(&reachable_checks, &mut check_map, settings);

            let new_items = reachable_items.difference(&progress);

            if new_items.is_empty() {
                if !progress.has(boss) {
                    // Boss couldn't be reached without the item on this check, therefore it's path
                    path_checks.push(check);
                }
                break;
            }

            for new_item in &new_items {
                progress.add_item(*new_item);
            }
        }
    }

    path_checks
}

const POSSIBLE_PATH_ITEMS: [FillerItem; 48] = [
    Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Bombs01, Bombs02, FireRod01,
    FireRod02, IceRod01, IceRod02, Hammer01, Hammer02, SandRod01, SandRod02, TornadoRod01,
    TornadoRod02, Bell, StaminaScroll, PegasusBoots, Flippers, HylianShield, SmoothGem,
    //LetterInABottle,
    PremiumMilk, HintGlasses, GreatSpin, Bottle01, Bottle02, Bottle03, Bottle04, Bottle05, Lamp01,
    Lamp02, Sword01, Sword02, Sword03, Sword04, Glove01, Glove02, Net01, Net02, Mail01, Mail02,
    OreYellow, OreGreen, OreBlue, OreRed,
];

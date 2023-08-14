use std::collections::{BTreeMap, HashSet};

use log::{error, info};
use macros::fail;
use modinfo::settings::{logic::LogicMode::*, Settings};
use queue::Queue;
use rand::{rngs::StdRng, Rng};

use crate::{
    item_pools::{get_maiamai_pool, Pool},
    model::{
        check::Check,
        filler_item::{FillerItem, Item},
        location::Location,
        progress::Progress,
    },
    world::WorldGraph,
    CheckMap, LocationInfo,
};

/// Fill Seed such that All Locations are Reachable
///
/// This is the "standard" filler algorithm for ALBWR.
pub fn fill_all_locations_reachable(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap, progression_pool: &mut Pool,
    junk_pool: &mut Pool, settings: &Settings, rng: &mut StdRng,
) -> Vec<(LocationInfo, game::Item)> {
    verify_all_locations_accessible(world_graph, check_map, progression_pool, settings);
    handle_exclusions(check_map, settings, rng, junk_pool);
    preplace_items(check_map, settings, rng, progression_pool, junk_pool);
    assumed_fill(world_graph, rng, progression_pool, check_map, settings);
    fill_junk(check_map, rng, junk_pool);
    map_to_result(world_graph, check_map)
}

/// Place static items ahead of the randomly filled ones
fn preplace_items(
    check_map: &mut CheckMap, settings: &Settings, rng: &mut StdRng, progression: &mut Vec<Item>,
    junk: &mut Vec<Item>,
) {
    // Vanilla Dungeon Prizes
    if !settings.logic.randomize_dungeon_prizes {
        place_static(check_map, progression, Item::PendantOfCourage01, "Eastern Palace Prize");
        place_static(check_map, progression, Item::PendantOfWisdom, "House of Gales Prize");
        place_static(check_map, progression, Item::PendantOfPower, "Tower of Hera Prize");
        place_static(check_map, progression, Item::PendantOfCourage02, "Hyrule Castle Prize");
        place_static(check_map, progression, Item::SageGulley, "Dark Palace Prize");
        place_static(check_map, progression, Item::SageOren, "Swamp Palace Prize");
        place_static(check_map, progression, Item::SageSeres, "Skull Woods Prize");
        place_static(check_map, progression, Item::SageOsfala, "Thieves' Hideout Prize");
        place_static(check_map, progression, Item::SageImpa, "Turtle Rock Prize");
        place_static(check_map, progression, Item::SageIrene, "Desert Palace Prize");
        place_static(check_map, progression, Item::SageRosso, "Ice Ruins Prize");
    } else if settings.logic.vanilla_charm {
        // Vanilla Charm
        place_static(check_map, progression, Item::PendantOfCourage02, "Hyrule Castle Prize");
    }

    // Place un-randomized items
    place_static(check_map, progression, Item::RupeeSilver40, "Hyrule Hotfoot (Second Race)");
    place_static(check_map, progression, Item::RupeeSilver41, "[TR] (1F) Under Center");
    place_static(check_map, progression, Item::RupeeGold09, "[TR] (B1) Under Center");
    place_static(check_map, progression, Item::RupeeGold10, "[PD] (2F) South Hidden Room");
    place_static(check_map, progression, Item::HeartPiece28, "Fortune's Choice");

    // Kakariko Item Shop
    place_static(check_map, progression, Item::ScootFruit01, "Kakariko Item Shop (1)");
    place_static(check_map, progression, Item::FoulFruit01, "Kakariko Item Shop (2)");
    place_static(check_map, progression, Item::Shield01, "Kakariko Item Shop (3)");

    // Lakeside Item Shop
    place_static(check_map, progression, Item::ScootFruit02, "Lakeside Item Shop (1)");
    place_static(check_map, progression, Item::FoulFruit02, "Lakeside Item Shop (2)");
    place_static(check_map, progression, Item::Shield02, "Lakeside Item Shop (3)");

    // Mysterious Man
    place_static(check_map, progression, Item::GoldBee01, "Mysterious Man");

    // Thieves' Town Item Shop
    place_static(check_map, progression, Item::Bee01, "Thieves' Town Item Shop (1)");
    place_static(check_map, progression, Item::GoldBee02, "Thieves' Town Item Shop (2)");
    place_static(check_map, progression, Item::Fairy01, "Thieves' Town Item Shop (3)");
    place_static(check_map, progression, Item::Shield03, "Thieves' Town Item Shop (4)");

    // Lorule Lake Item Shop
    place_static(check_map, progression, Item::Bee02, "Lorule Lakeside Item Shop (1)");
    place_static(check_map, progression, Item::GoldBee03, "Lorule Lakeside Item Shop (2)");
    place_static(check_map, progression, Item::Fairy02, "Lorule Lakeside Item Shop (3)");
    place_static(check_map, progression, Item::Shield04, "Lorule Lakeside Item Shop (4)");

    // Super Items
    if settings.logic.super_items {
        exclude("Treacherous Tower Advanced (1)", rng, check_map, junk);
        exclude("Treacherous Tower Advanced (2)", rng, check_map, junk);
    } else {
        place_static(check_map, progression, Item::Lamp02, "Treacherous Tower Advanced (1)");
        place_static(check_map, progression, Item::Net02, "Treacherous Tower Advanced (2)");
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
        place_static(check_map, progression, Item::Bow02, " 10 Maiamai");
        place_static(check_map, progression, Item::Boomerang02, " 20 Maiamai");
        place_static(check_map, progression, Item::Hookshot02, " 30 Maiamai");
        place_static(check_map, progression, Item::Hammer02, " 40 Maiamai");
        place_static(check_map, progression, Item::Bombs02, " 50 Maiamai");
        place_static(check_map, progression, Item::FireRod02, " 60 Maiamai");
        place_static(check_map, progression, Item::IceRod02, " 70 Maiamai");
        place_static(check_map, progression, Item::TornadoRod02, " 80 Maiamai");
        place_static(check_map, progression, Item::SandRod02, " 90 Maiamai");
    }
    exclude("100 Maiamai", rng, check_map, junk);

    let mut shop_positions: Vec<String> = Vec::new();
    let mut bow_light_positions: Vec<String> = Vec::from(["Zelda".to_owned()]);
    let mut maiamai_positions: Vec<String> = Vec::new();

    for (check_name, item) in check_map.clone() {
        if check_name.starts_with("[LC]") && item.is_none() {
            let _ = &bow_light_positions.push(check_name.clone());
        } else if check_name.starts_with("Ravio") && !check_name.contains('6') {
            let _ = &shop_positions.push(check_name.clone());
        } else if check_name.starts_with("[Mai]") {
            let _ = &maiamai_positions.push(check_name.clone());
        }
    }

    if settings.logic.bow_of_light_in_castle {
        check_map.insert(
            bow_light_positions.remove(rng.gen_range(0..bow_light_positions.len())),
            Some(Item::BowOfLight.into()),
        );
        progression.retain(|x| *x != Item::BowOfLight);
    }

    // Bell in Shop
    if settings.logic.bell_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(Item::Bell.into()),
        );
        progression.retain(|x| *x != Item::Bell);
    }

    // Pouch in Shop
    if settings.logic.pouch_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(Item::Pouch.into()),
        );
        progression.retain(|x| *x != Item::Pouch);
    }

    // Sword in Shop
    if settings.logic.sword_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(Item::Sword01.into()),
        );
        progression.retain(|x| *x != Item::Sword01);
    }

    // Boots in Shop
    if settings.logic.boots_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(Item::PegasusBoots.into()),
        );
        progression.retain(|x| *x != Item::PegasusBoots);
    }

    // Assures a weapon will be available in Ravio's Shop
    if (!settings.logic.sword_in_shop && !settings.logic.boots_in_shop)
        && settings.logic.assured_weapon
    {
        let mut weapons = Vec::from([
            Item::Bow01,
            Item::Bombs01,
            Item::FireRod01,
            Item::IceRod01,
            Item::Hammer01,
            Item::PegasusBoots,
        ]);

        if !settings.logic.swordless_mode {
            weapons.extend_from_slice(&[Item::Sword01]);
        }

        match settings.logic.logic_mode {
            Normal => {}
            _ => {
                weapons.extend_from_slice(&[Item::Lamp01, Item::Net01]);
            }
        }

        let weapon = *weapons.get(rng.gen_range(0..weapons.len())).unwrap();

        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(weapon.into()),
        );
        progression.retain(|x| *x != weapon);
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
            place_static(check_map, progression, maiamai_items.remove(0), &check_name);
        }
    }
}

// Statically place an item in a given location, then remove it from the item pool provided
fn place_static(check_map: &mut CheckMap, pool: &mut Pool, item: Item, check_name: &str) {
    check_map.insert(check_name.to_owned(), Some(item.into()));
    pool.retain(|x| *x != item);
}

// Exclude a location by placing a random junk item there
fn exclude(check_name: &str, rng: &mut StdRng, check_map: &mut CheckMap, junk: &mut Pool) {
    if check_map
        .insert(check_name.to_owned(), Some(junk.remove(rng.gen_range(0..junk.len())).into()))
        .is_none()
    {
        fail!("Check not found: {}", check_name);
    }
}

fn handle_exclusions(
    check_map: &mut CheckMap, settings: &Settings, rng: &mut StdRng, junk_pool: &mut Vec<Item>,
) {
    let opt = settings.exclusions.0.get("exclusions");
    if opt.is_none() {
        return;
    }

    let exclusions = opt.unwrap();

    for exclusion in exclusions {
        if check_map.contains_key(exclusion) {
            let rng_index = rng.gen_range(0..junk_pool.len());
            check_map.insert(exclusion.clone(), Some(junk_pool.remove(rng_index).into()));
        } else {
            error!("Cannot exclude \"{}\", no matching check found with that name.", exclusion);
            fail!("Consult a spoiler log for a list of valid check names.");
        }
    }
}

/// Super dirty mapping I hate it
fn map_to_result(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap,
) -> Vec<(LocationInfo, game::Item)> {
    let mut result = Vec::new();
    for location_node in world_graph.values_mut() {
        for check in location_node.clone().get_checks() {
            if let Some(loc_info) = check.get_location_info() {
                if let FillerItem::Item(item) = check_map.get(check.get_name()).unwrap().unwrap() {
                    result.push((loc_info, item.to_game_item()));
                }
            }
        }
    }
    result
}

fn is_dungeon_prize(item: Item) -> bool {
    matches!(
        item,
        Item::PendantOfPower
            | Item::PendantOfWisdom
            | Item::PendantOfCourage01
            | Item::PendantOfCourage02
            | Item::SageGulley
            | Item::SageOren
            | Item::SageSeres
            | Item::SageOsfala
            | Item::SageImpa
            | Item::SageIrene
            | Item::SageRosso
    )
}

fn is_dungeon_item(item: Item) -> bool {
    matches!(
        item,
        Item::HyruleSanctuaryKey
            | Item::LoruleSanctuaryKey
            | Item::EasternCompass
            | Item::EasternKeyBig
            | Item::EasternKeySmall01
            | Item::EasternKeySmall02
            | Item::GalesCompass
            | Item::GalesKeyBig
            | Item::GalesKeySmall01
            | Item::GalesKeySmall02
            | Item::GalesKeySmall03
            | Item::GalesKeySmall04
            | Item::HeraCompass
            | Item::HeraKeyBig
            | Item::HeraKeySmall01
            | Item::HeraKeySmall02
            | Item::DarkCompass
            | Item::DarkKeyBig
            | Item::DarkKeySmall01
            | Item::DarkKeySmall02
            | Item::DarkKeySmall03
            | Item::DarkKeySmall04
            | Item::SwampCompass
            | Item::SwampKeyBig
            | Item::SwampKeySmall01
            | Item::SwampKeySmall02
            | Item::SwampKeySmall03
            | Item::SwampKeySmall04
            | Item::SkullCompass
            | Item::SkullKeyBig
            | Item::SkullKeySmall01
            | Item::SkullKeySmall02
            | Item::SkullKeySmall03
            | Item::ThievesCompass
            | Item::ThievesKeyBig
            | Item::ThievesKeySmall
            | Item::IceCompass
            | Item::IceKeyBig
            | Item::IceKeySmall01
            | Item::IceKeySmall02
            | Item::IceKeySmall03
            | Item::DesertCompass
            | Item::DesertKeyBig
            | Item::DesertKeySmall01
            | Item::DesertKeySmall02
            | Item::DesertKeySmall03
            | Item::DesertKeySmall04
            | Item::DesertKeySmall05
            | Item::TurtleCompass
            | Item::TurtleKeyBig
            | Item::TurtleKeySmall01
            | Item::TurtleKeySmall02
            | Item::TurtleKeySmall03
            | Item::LoruleCastleCompass
            | Item::LoruleCastleKeySmall01
            | Item::LoruleCastleKeySmall02
            | Item::LoruleCastleKeySmall03
            | Item::LoruleCastleKeySmall04
            | Item::LoruleCastleKeySmall05
    )
}

fn fill_junk(check_map: &mut CheckMap, rng: &mut StdRng, junk_items: &mut Pool) {
    info!("Placing Junk Items...");

    let mut empty_check_keys = Vec::new();
    for (key, val) in check_map.clone() {
        if val.is_none() {
            empty_check_keys.push(key);
        }
    }

    if empty_check_keys.len() != junk_items.len() {
        fail!(
            "Number of empty checks: {} does not match available junk items: {}",
            empty_check_keys.len(),
            junk_items.len()
        );
    }

    for junk in junk_items {
        check_map.insert(
            empty_check_keys.remove(rng.gen_range(0..empty_check_keys.len())),
            Some((*junk).into()),
        );
    }
}

fn place_item_randomly(
    item: Item, checks: &Vec<Check>, check_map: &mut CheckMap, rng: &mut StdRng,
) {
    check_map.insert(
        checks.get(rng.gen_range(0..checks.len())).unwrap().get_name().to_owned(),
        Some(item.into()),
    );
}

fn filter_checks(item: Item, checks: &[Check], check_map: &mut CheckMap) -> Vec<Check> {
    // Filter out non-empty checks
    let mut filtered_checks = checks
        .iter()
        .filter(|&x| check_map.get(x.get_name()).unwrap().is_none())
        .cloned()
        .collect::<Vec<_>>();

    // Filter checks by item type
    if is_dungeon_prize(item) {
        filtered_checks = filter_dungeon_prize_checks(&filtered_checks);
    } else if is_dungeon_item(item) {
        let is_keysanity = false; // No keysanity yet, hardcode to false
        if !is_keysanity {
            filtered_checks = filter_dungeon_checks(item, &filtered_checks);
        }
    }

    filtered_checks
}

fn filter_dungeon_prize_checks(eligible_checks: &[Check]) -> Vec<Check> {
    eligible_checks.iter().filter(|&x| x.get_name().contains("Prize")).cloned().collect()
}

fn filter_dungeon_checks(item: Item, eligible_checks: &[Check]) -> Vec<Check> {
    use Item::*;
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

fn exist_empty_reachable_check(checks: &Vec<Check>, check_map: &mut CheckMap) -> bool {
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
pub fn prefill_check_map(world_graph: &mut WorldGraph) -> CheckMap {
    let mut check_map = BTreeMap::new();

    for location_node in world_graph.values_mut() {
        for check in location_node.clone().get_checks() {
            if check_map.insert(check.get_name().to_owned(), check.get_quest()).is_some() {
                fail!("Multiple checks have duplicate name: {}", check.get_name());
            }
        }
    }

    check_map
}

/// This translation is probably adding unnecessary overhead, oh well
fn build_progress_from_items(items: &Pool, settings: &Settings) -> Progress {
    let mut progress = Progress::new(settings.clone());
    for item in items {
        progress.add_item(*item);
    }

    progress
}

fn verify_all_locations_accessible(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap, progression_pool: &mut Pool,
    settings: &Settings,
) {
    if NoLogic.eq(&settings.logic.logic_mode) {
        return; // Skip this check on No Logic
    }

    info!("Verifying all locations accessible...");
    let reachable_checks = assumed_search(world_graph, progression_pool, check_map, settings); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    /**
     * 384 In-Logic Checks
     *
     * - 253 Standard Checks
     * - 100 Maiamai
     * - 11 Dungeon Prizes
     * - 20 Statically Placed Items:
     *     - 12x Shop Items (not including 9,999 items)
     *     - 3x Obscure Gold/Silver Rupees
     *     - Mysterious Man
     *     - Bouldering Guy Bottle
     *     - TODO: Letter in a Bottle
     *     - TODO: Hyrule Hotfoot Second Race
     *     - TODO: Fortune's Choice
     *
     * 14 Out-of-Logic checks NOT included:
     * - TODO: 10 Maiamai Rewards
     * - 2 Golden Bees for 9,999 Rupees
     * - 2 Treacherous Tower Advanced
     */
    const IN_LOGIC_CHECKS: usize = 384;

    /// "Progression Events" (non-item checks that are still progression)
    const PROGRESSION_EVENTS: usize = 33;

    /// Hint Ghosts (Overworld)
    const HINT_GHOSTS_OW: usize = 58;

    if reachable_checks.len() != IN_LOGIC_CHECKS + PROGRESSION_EVENTS + HINT_GHOSTS_OW {
        let reachable_check_names: Vec<&str> =
            reachable_checks.iter().map(|c| c.get_name()).collect();
        for check in check_map.keys() {
            if !reachable_check_names.contains(&check.as_str()) {
                info!("Unreachable Check: {}", check);
            }
        }

        fail!(
            "Only {}/{} checks were reachable in the world graph",
            reachable_checks.len(),
            IN_LOGIC_CHECKS + PROGRESSION_EVENTS
        );
    }
}

/// Find all checks reachable with the given Progress
pub(crate) fn find_reachable_checks(
    world_graph: &mut WorldGraph, progress: &Progress,
) -> Vec<Check> {
    let start_node = Location::RavioShop;
    let mut loc_queue: Queue<Location> = Queue::from(vec![start_node]);
    let mut visited: HashSet<Location> = HashSet::new();
    let mut reachable_checks: Vec<Check> = Vec::new(); // possibly switch to HashSet to avoid dupes

    visited.insert(start_node);

    while !loc_queue.is_empty() {
        let location = loc_queue.dequeue().unwrap();

        // Grab the location from the map, verify it is defined
        let location_node = match world_graph.get_mut(&location) {
            Some(loc) => loc,
            None => {
                fail!("Location Undefined: {:?}", location);
            }
        };

        // Iterate over the location's checks
        for check in location_node.clone().get_checks() {
            if check.can_access(progress) {
                reachable_checks.push(*check);
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

pub(crate) fn get_items_from_reachable_checks(
    reachable_checks: &Vec<Check>, check_map: &mut CheckMap, settings: &Settings,
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
    world_graph: &mut WorldGraph, rng: &mut StdRng, items_owned: &mut Pool,
    check_map: &mut CheckMap, settings: &Settings,
) {
    info!("Placing Progression Items...");

    let mut reachable_checks = assumed_search(world_graph, items_owned, check_map, settings);

    while exist_empty_reachable_check(&reachable_checks, check_map) && !items_owned.is_empty() {
        let item = items_owned.remove(0);

        //
        reachable_checks = assumed_search(world_graph, items_owned, check_map, settings);

        let filtered_checks = filter_checks(item, &reachable_checks, check_map);

        if filtered_checks.is_empty() {
            info!("No reachable checks found to place: {:?}", item);
        }

        place_item_randomly(item, &filtered_checks, check_map, rng);
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
    world_graph: &mut WorldGraph, items_owned: &mut Pool, check_map: &mut CheckMap,
    settings: &Settings,
) -> Vec<Check> {
    let mut considered_items = build_progress_from_items(&items_owned.clone(), settings);
    let mut reachable_checks: Vec<Check>;

    loop {
        reachable_checks = find_reachable_checks(world_graph, &considered_items);
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, check_map, settings);

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

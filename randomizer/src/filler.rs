use std::collections::{BTreeMap, HashSet};

use log::{error, info};
use queue::Queue;
use rand::{rngs::StdRng, Rng};
use rom::Item;
use settings::logic_mode::LogicMode::*;

use crate::{
    convert,
    item_pools::{get_maiamai_pool, Pool},
    model::{check::Check, location::Location, progress::Progress},
    world::WorldGraph,
    CheckMap, Error,
    FillerItem::{self, *},
    LocationKey, Result, Settings,
};

/// Fill Seed such that All Locations are Reachable
///
/// This is the "standard" filler algorithm for ALBWR.
pub fn fill_all_locations_reachable(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap, progression_pool: &mut Pool,
    junk_pool: &mut Pool, settings: &Settings, rng: &mut StdRng,
) -> Result<Vec<(LocationKey, Item)>> {
    verify_all_locations_accessible(world_graph, check_map, progression_pool, settings)?;
    handle_exclusions(check_map, settings, rng, junk_pool)?;
    preplace_items(check_map, settings, rng, progression_pool, junk_pool)?;
    assumed_fill(world_graph, rng, progression_pool, check_map, settings)?;
    fill_junk(check_map, rng, junk_pool)?;
    Ok(map_to_result(world_graph, check_map))
}

/// Place static items ahead of the randomly filled ones
fn preplace_items(
    check_map: &mut CheckMap, settings: &Settings, rng: &mut StdRng,
    progression: &mut Vec<FillerItem>, junk: &mut Vec<FillerItem>,
) -> Result<()> {
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
    } else if settings.logic.vanilla_charm {
        // Vanilla Charm
        place_static(check_map, progression, PendantOfCourage02, "Hyrule Castle Prize");
    }

    // Place un-randomized items
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
        exclude("Treacherous Tower Advanced (1)", rng, check_map, junk)?;
        exclude("Treacherous Tower Advanced (2)", rng, check_map, junk)?;
    } else {
        place_static(check_map, progression, Lamp02, "Treacherous Tower Advanced (1)");
        place_static(check_map, progression, Net02, "Treacherous Tower Advanced (2)");
    }

    // Nice Mode
    if settings.logic.nice_mode {
        exclude(" 10 Maiamai", rng, check_map, junk)?;
        exclude(" 20 Maiamai", rng, check_map, junk)?;
        exclude(" 30 Maiamai", rng, check_map, junk)?;
        exclude(" 40 Maiamai", rng, check_map, junk)?;
        exclude(" 50 Maiamai", rng, check_map, junk)?;
        exclude(" 60 Maiamai", rng, check_map, junk)?;
        exclude(" 70 Maiamai", rng, check_map, junk)?;
        exclude(" 80 Maiamai", rng, check_map, junk)?;
        exclude(" 90 Maiamai", rng, check_map, junk)?;
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
    exclude("100 Maiamai", rng, check_map, junk)?;

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
            Some(BowOfLight),
        );
        progression.retain(|x| *x != BowOfLight);
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

    // Sword in Shop
    if settings.logic.sword_in_shop {
        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Sword01));
        progression.retain(|x| *x != Sword01);
    }

    // Boots in Shop
    if settings.logic.boots_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(PegasusBoots),
        );
        progression.retain(|x| *x != PegasusBoots);
    }

    // Assures a weapon will be available in Ravio's Shop
    if (!settings.logic.sword_in_shop && !settings.logic.boots_in_shop)
        && settings.logic.assured_weapon
    {
        let mut weapons = Vec::from([Bow01, Bombs01, FireRod01, IceRod01, Hammer01, PegasusBoots]);

        if !settings.logic.swordless_mode {
            weapons.extend_from_slice(&[Sword01]);
        }

        match settings.logic.logic_mode {
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

    // Exclude Minigames
    if settings.logic.minigames_excluded {
        exclude("Dodge the Cuccos", rng, check_map, junk)?;
        exclude("Hyrule Hotfoot (First Race)", rng, check_map, junk)?;
        exclude("Rupee Rush (Hyrule)", rng, check_map, junk)?;
        exclude("Rupee Rush (Lorule)", rng, check_map, junk)?;
        exclude("Octoball Derby", rng, check_map, junk)?;
        exclude("Treacherous Tower Intermediate", rng, check_map, junk)?;

        // For Maiamai Madness, also turn the rupee rush maiamai into random junk
        if settings.logic.maiamai_madness {
            exclude("[Mai] Hyrule Rupee Rush Wall", rng, check_map, junk)?;
            exclude("[Mai] Lorule Rupee Rush Wall", rng, check_map, junk)?;
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
    Ok(())
}

// Statically place an item in a given location, then remove it from the item pool provided
fn place_static(check_map: &mut CheckMap, pool: &mut Pool, item: FillerItem, check_name: &str) {
    check_map.insert(check_name.to_owned(), Some(item));
    pool.retain(|x| *x != item);
}

// Exclude a location by placing a random junk item there
fn exclude(
    check_name: &str, rng: &mut StdRng, check_map: &mut CheckMap, junk: &mut Pool,
) -> Result<()> {
    check_map
        .insert(check_name.to_owned(), Some(junk.remove(rng.gen_range(0..junk.len()))))
        .ok_or_else(|| Error::new(format!("Check not found: {}", check_name)))
        .map(|_| ())
}

fn handle_exclusions(
    check_map: &mut CheckMap, settings: &Settings, rng: &mut StdRng,
    junk_pool: &mut Vec<FillerItem>,
) -> Result<()> {
    let opt = settings.exclusions.0.get("exclusions");
    if opt.is_none() {
        return Ok(());
    }

    let exclusions = opt.unwrap();

    for exclusion in exclusions {
        if check_map.contains_key(exclusion) {
            let rng_index = rng.gen_range(0..junk_pool.len());
            check_map.insert(exclusion.clone(), Some(junk_pool.remove(rng_index)));
        } else {
            error!("Cannot exclude \"{}\", no matching check found with that name.", exclusion);
            return Err(Error::new("Consult a spoiler log for a list of valid check names."));
        }
    }
    Ok(())
}

/// Super dirty mapping I hate it
fn map_to_result(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap,
) -> Vec<(LocationKey, Item)> {
    let mut result: Vec<(LocationKey, Item)> = Vec::new();
    for location_node in world_graph.values_mut() {
        for check in location_node.clone().get_checks() {
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
    matches!(
        item,
        PendantOfPower
            | PendantOfWisdom
            | PendantOfCourage01
            | PendantOfCourage02
            | SageGulley
            | SageOren
            | SageSeres
            | SageOsfala
            | SageImpa
            | SageIrene
            | SageRosso
    )
}

fn is_dungeon_item(item: FillerItem) -> bool {
    matches!(
        item,
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
            | LoruleCastleKeySmall05
    )
}

fn fill_junk(check_map: &mut CheckMap, rng: &mut StdRng, junk_items: &mut Pool) -> Result<()> {
    info!("Placing Junk Items...");

    let mut empty_check_keys = Vec::new();
    for (key, val) in check_map.clone() {
        if val.is_none() {
            empty_check_keys.push(key);
        }
    }

    if empty_check_keys.len() != junk_items.len() {
        return Err(Error::new(format!(
            "Number of empty checks: {} does not match available junk items: {}",
            empty_check_keys.len(),
            junk_items.len()
        )));
    }

    for junk in junk_items {
        check_map
            .insert(empty_check_keys.remove(rng.gen_range(0..empty_check_keys.len())), Some(*junk));
    }
    Ok(())
}

fn place_item_randomly(
    item: FillerItem, checks: &Vec<Check>, check_map: &mut CheckMap, rng: &mut StdRng,
) {
    check_map.insert(
        checks.get(rng.gen_range(0..checks.len())).unwrap().get_name().to_owned(),
        Some(item),
    );
}

fn filter_checks(
    item: FillerItem, checks: &[Check], check_map: &mut CheckMap,
) -> Result<Vec<Check>> {
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
            filtered_checks = filter_dungeon_checks(item, &filtered_checks)?;
        }
    }

    Ok(filtered_checks)
}

fn filter_dungeon_prize_checks(eligible_checks: &[Check]) -> Vec<Check> {
    eligible_checks.iter().filter(|&x| x.get_name().contains("Prize")).cloned().collect()
}

fn filter_dungeon_checks(item: FillerItem, eligible_checks: &[Check]) -> Result<Vec<Check>> {
    eligible_checks
        .iter()
        .map(|x| {
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
                    return Err(Error::new(format!("Item {:?} is not a dungeon item", item)));
                }
            });
            Ok(*x)
        })
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
pub fn prefill_check_map(world_graph: &mut WorldGraph) -> Result<CheckMap> {
    let mut check_map = BTreeMap::new();

    for location_node in world_graph.values_mut() {
        for check in location_node.clone().get_checks() {
            if check_map.insert(check.get_name().to_owned(), check.get_quest()).is_some() {
                return Err(Error::new(format!(
                    "Multiple checks have duplicate name: {}",
                    check.get_name()
                )));
            }
        }
    }

    Ok(check_map)
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
) -> Result<()> {
    if NoLogic.eq(&settings.logic.logic_mode) {
        return Ok(()); // Skip this check on No Logic
    }

    info!("Verifying all locations accessible...");
    let reachable_checks = assumed_search(world_graph, progression_pool, check_map, settings)?; //find_reachable_checks(loc_map, &everything, &mut check_map); //

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

        return Err(Error::new(format!(
            "Only {}/{} checks were reachable in the world graph",
            reachable_checks.len(),
            IN_LOGIC_CHECKS + PROGRESSION_EVENTS
        )));
    }
    Ok(())
}

/// Find all checks reachable with the given Progress
pub(crate) fn find_reachable_checks(
    world_graph: &mut WorldGraph, progress: &Progress,
) -> Result<Vec<Check>> {
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
                return Err(Error::new(format!("Location Undefined: {:?}", location)));
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

    Ok(reachable_checks)
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
) -> Result<()> {
    info!("Placing Progression Items...");

    let mut reachable_checks = assumed_search(world_graph, items_owned, check_map, settings)?;

    while exist_empty_reachable_check(&reachable_checks, check_map) && !items_owned.is_empty() {
        let item = items_owned.remove(0);

        //
        reachable_checks = assumed_search(world_graph, items_owned, check_map, settings)?;

        let filtered_checks = filter_checks(item, &reachable_checks, check_map)?;

        if filtered_checks.is_empty() {
            info!("No reachable checks found to place: {:?}", item);
        }

        place_item_randomly(item, &filtered_checks, check_map, rng);
    }
    Ok(())
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
) -> Result<Vec<Check>> {
    let mut considered_items = build_progress_from_items(&items_owned.clone(), settings);
    let mut reachable_checks: Vec<Check>;

    loop {
        reachable_checks = find_reachable_checks(world_graph, &considered_items)?;
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

    Ok(reachable_checks)
}

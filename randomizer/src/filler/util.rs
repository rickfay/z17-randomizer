use std::error::Error;
use {
    crate::{CheckMap, filler::build_progress_from_items, old_world::WorldGraph},
    macros::fail,
    queue::Queue,
    rand::{prelude::StdRng, Rng},
    seed::{
        filler::{location::LocationId, pool::Pool, progress::Progress},
        settings::Settings,
        world::check::Check,
    },
    std::collections::HashSet,
};
use seed::Seed;
use seed::world::{CheckId, LocationId};

/// Shuffles elements of a Vec
pub(crate) fn shuffle<K>(rng: &mut StdRng, mut vec: Vec<K>) -> Vec<K> {
    let mut shuffled: Vec<K> = Vec::new();

    while !vec.is_empty() {
        shuffled.push(vec.remove(rng.gen_range(0..vec.len())));
    }

    shuffled
}

/// Find all checks reachable with the given Progress
pub(crate) fn find_reachable_checks(
    seed_world: &mut Seed, progress: &Progress,
) -> Result<Vec<CheckId>, Box<dyn Error>> {
    let start_node = LocationId::RavioShop;
    let mut loc_queue: Queue<LocationId> = Queue::from(vec![start_node]);
    let mut visited: HashSet<LocationId> = HashSet::new();
    let mut reachable_checks: Vec<CheckId> = Vec::new(); // possibly switch to HashSet to avoid dupes

    visited.insert(start_node);

    while !loc_queue.is_empty() {
        let location_id = loc_queue.dequeue().unwrap();
        let location = seed_world.get_location(location_id);

        // Iterate over the location's checks
        for check_id in location.get_checks() {
            let check = seed_world.get_check(check_id);
            if check.can_access(progress) {
                reachable_checks.push(check_id.into());
            }
        }

        // Queue new paths reachable from this location
        for path in location.get_paths() {
            let destination = path.get_destination();
            if !visited.contains(&destination) && path.can_access(progress) {
                loc_queue.queue(destination.clone())?;
                visited.insert(destination);
            }
        }
    }

    Ok(reachable_checks)
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
pub(crate) fn assumed_search(
    seed_world: &mut Seed, mut items_owned: Pool, settings: &Settings,
) -> Result<Vec<CheckId>, Box<dyn Error>> {
    let mut considered_items = build_progress_from_items(items_owned, settings);
    let mut reachable_checks;

    loop {
        reachable_checks = find_reachable_checks(seed_world, &considered_items)?;
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, seed_world, settings);

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

/// This translation is probably adding unnecessary overhead, oh well
fn build_progress_from_items(items: Pool, settings: &Settings) -> Progress {
    let mut progress = Progress::new(settings.clone());
    for item in items {
        progress.add_item(*item);
    }

    progress
}

///
pub(crate) fn get_items_from_reachable_checks(
    reachable_checks: &Vec<CheckId>, seed_world: &mut Seed, settings: &Settings,
) -> Progress {
    let mut progress = Progress::new(settings.clone());

    for check_id in reachable_checks {

        // Items already placed in the world that can be picked up
        if let Some(item) = seed_world.get_check(check_id).get_item() {
            progress.add_item(*item)
        };
    }

    progress
}

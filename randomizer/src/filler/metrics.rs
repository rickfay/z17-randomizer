use {
    crate::{CheckMap, filler, old_world::WorldGraph},
    log::info,
    seed::{
        filler::progress::Progress,
        Metrics,
        settings::Settings,
        world::check::Check,
    },
    serde::Serialize,
    std::collections::BTreeMap,
};
use seed::Seed;

/// Perform any post-generation analysis for a seed here
pub fn calculate_metrics(seed_world: &mut Seed, settings: &Settings) {
    info!("Calculating Metrics...");
    let playthrough = sphere_search(seed_world, settings);
    seed_world.set_metrics(Metrics::new(playthrough));
}

/// Sphere Search
fn sphere_search<'a>(
    seed_world: &mut Seed, settings: &Settings,
) -> BTreeMap<String, BTreeMap<&'static str, &'static str>> {
    info!("Generating Playthrough...");

    let mut progress = Progress::new(settings.clone());
    let mut reachable_checks: Vec<Check>;
    let mut spheres = BTreeMap::new();
    let mut sphere_num = 0;

    loop {
        reachable_checks = filler::util::find_reachable_checks(world_graph, &progress);
        let reachable_items =
            filler::util::get_items_from_reachable_checks(&reachable_checks, check_map, settings);

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

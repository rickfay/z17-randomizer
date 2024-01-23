use crate::filler::check::Check;
use crate::filler::progress::Progress;
use crate::{filler, CheckMap, SeedInfo};
use log::info;
use rom::Error;
use serde::Serialize;
use std::collections::BTreeMap;

/// Perform any post-generation analysis for a seed here
pub fn calculate_metrics(seed_info: &mut SeedInfo, check_map: &mut CheckMap) -> Result<(), Error> {
    info!("Calculating Metrics...");

    let playthrough = sphere_search(seed_info, check_map);

    seed_info.metrics = Metrics { spheres: playthrough.len(), playthrough };

    Ok(())
}

/// Sphere Search
fn sphere_search(
    seed_info: &mut SeedInfo, check_map: &mut CheckMap,
) -> BTreeMap<String, BTreeMap<&'static str, &'static str>> {
    info!("Generating Playthrough...");

    let mut progress = Progress::new(&seed_info.settings);
    let mut reachable_checks: Vec<Check>;
    let mut spheres = BTreeMap::new();
    let mut sphere_num = 0;

    loop {
        reachable_checks = filler::find_reachable_checks(seed_info, &progress);
        let reachable_items = filler::get_items_from_reachable_checks(seed_info, &reachable_checks, check_map);

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
            if new_items.contains(&filler_item) && filler_item.include_in_sphere_search(&seed_info.settings) {
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

#[derive(Default, Debug, Clone, Serialize)]
pub struct Metrics {
    spheres: usize,
    playthrough: Playthrough,
}

pub type Playthrough = BTreeMap<String, BTreeMap<&'static str, &'static str>>;

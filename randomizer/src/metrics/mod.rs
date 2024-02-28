use crate::filler::check::Check;
use crate::filler::filler_item::{Goal, Item, Randomizable, Vane};
use crate::filler::portals::Portal;
use crate::filler::progress::Progress;
use crate::{filler, CheckMap, SeedInfo};
use game::ghosts::HintGhost;
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

#[derive(Debug, Clone, Serialize)]
pub struct Sphere {
    items: BTreeMap<String, Item>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    ghosts: BTreeMap<String, HintGhost>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    goals: BTreeMap<String, Goal>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    portals: BTreeMap<String, Portal>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    weather_vanes: BTreeMap<String, Vane>,
}

impl Sphere {
    fn new() -> Self {
        Self {
            items: Default::default(),
            ghosts: Default::default(),
            goals: Default::default(),
            portals: Default::default(),
            weather_vanes: Default::default(),
        }
    }

    fn add(&mut self, check_name: &str, item: Randomizable) {
        match item {
            Randomizable::Item(item) => {
                self.items.insert(String::from(check_name), item);
            },
            Randomizable::Goal(goal) => {
                self.goals.insert(String::from(check_name), goal);
            },
            Randomizable::HintGhost(ghost) => {
                self.ghosts.insert(String::from(check_name), ghost);
            },
            Randomizable::Vane(vane) => {
                self.weather_vanes.insert(String::from(check_name), vane);
            },
            Randomizable::Portal(portal) => {
                self.portals.insert(String::from(check_name), portal);
            },
        };
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
            && self.ghosts.is_empty()
            && self.goals.is_empty()
            && self.portals.is_empty()
            && self.weather_vanes.is_empty()
    }
}

/// Sphere Search
fn sphere_search(seed_info: &mut SeedInfo, check_map: &mut CheckMap) -> BTreeMap<String, Sphere> {
    info!("Generating Playthrough...");

    let mut progress = Progress::new(seed_info);
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

        let mut sphere = Sphere::new();
        for reachable_check in reachable_checks {
            let filler_item = check_map.get(reachable_check.get_name()).unwrap().unwrap();
            if new_items.contains(&filler_item) && filler_item.include_in_sphere_search() {
                sphere.add(reachable_check.get_name(), filler_item);
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

pub type Playthrough = BTreeMap<String, Sphere>;

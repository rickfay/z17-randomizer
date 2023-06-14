use {
    crate::{
        filler,
        old_world::{self, WorldGraph},
        CheckMap, LocationInfo,
    },
    albw::Item,
    item_pools::{get_maiamai_pool, Pool},
    jack::item::Item,
    log::{error, info},
    macros::fail,
    queue::Queue,
    rand::{prelude::StdRng, Rng},
    seed::{
        filler::{
            filler_item::{
                convert,
                FillerItem::{self, *},
            },
            location::LocationId,
            progress::Progress,
        },
        settings::{logic_mode::LogicMode::NoLogic, Settings},
        world::{check::Check, CheckId},
        Seed, SeedHash, SeedInfo,
    },
    std::collections::{HashMap, HashSet},
};

mod alr;
mod hints;
pub mod item_pools;
pub mod metrics;
mod util;

/// Generates all details of a randomized [`Seed`].
pub(crate) fn generate_seed(
    seed_num: u32, settings: &Settings, hash: SeedHash, rng: &mut StdRng,
) -> Result<Seed, Box<dyn error::Error>> {
    println!();
    info!("Generating Seed...");

    let mut seed = Seed::init(seed_num, hash, settings, rng);

    alr::fill_all_locations_reachable(&mut seed, settings, rng);
    hints::generate_hints(&mut seed, settings, rng);
    metrics::calculate_metrics(&mut seed, settings);

    Ok(seed)
}

fn place_item_randomly(
    item: FillerItem, checks: &Vec<Check>, check_map: &mut CheckMap, rng: &mut StdRng,
) {
    let index = rng.gen_range(0..checks.len());
    check_map.insert(checks.get(index).unwrap().get_name().to_owned(), Some(item));
}

fn filter_checks(seed: &mut Seed, item: FillerItem, check_ids: Vec<CheckId>) -> Vec<Check> {
    // Filter out non-empty checks
    let mut filtered_checks = check_ids
        .iter()
        .filter(|&x| check_map.get(x.get_name()).unwrap().is_none())
        .cloned()
        .collect();

    // Filter checks by item type
    if item.is_dungeon_prize() {
        filtered_checks = filter_dungeon_prize_checks(&mut filtered_checks);
    } else if item.is_dungeon_item() {
        let is_keysanity = false; // No keysanity yet, hardcode to false
        if !is_keysanity {
            filtered_checks = filter_dungeon_checks(item, &mut filtered_checks);
        }
    }

    filtered_checks
}

fn filter_dungeon_prize_checks(eligible_checks: &mut Vec<Check>) -> Vec<Check> {
    // fixme don't base this off the name
    eligible_checks.iter().filter(|&x| x.get_name().contains("Prize")).cloned().collect()
}

fn filter_dungeon_checks(item: FillerItem, eligible_checks: &mut Vec<Check>) -> Vec<Check> {
    // fixme don't base this off the name
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

/// Prefills a map with all checks as defined by the world graph with no values yet assigned
pub fn prefill_check_map(world_graph: &mut WorldGraph) -> CheckMap {
    let mut check_map = HashMap::new();

    for (_, location_node) in world_graph {
        for check in location_node.clone().get_checks() {
            if check_map
                .insert(check.get_name().to_owned(), match check.get_quest() {
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

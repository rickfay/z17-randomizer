use {
    crate::{
        CheckMap,
        filler::{find_reachable_checks, get_items_from_reachable_checks, util},
        filler_util::shuffle,
        old_world::WorldGraph,
        patch::util::is_sage,
    },
    log::{debug, info},
    macros::fail,
    rand::{
        prelude::{IteratorRandom, StdRng},
        Rng,
    },
    seed::{
        filler::{
            filler_item::{
                convert,
                FillerItem::{self, *},
            },
            progress::Progress,
        },
        hints::{BowOfLightHint, Hints, LocationHint, PathHint},
        settings::Settings,
        world::check::Check,
    },
    std::collections::HashSet,
};
use seed::Seed;

/// Generates Always, Path, and Sometimes Hints based on settings
pub fn generate_hints(seed_world: &Seed, settings: &Settings, rng: &mut StdRng) -> Hints {
    info!("Generating Hints...");

    const NUM_TOTAL_HINTS: usize = 29;
    let mut taken_checks: Vec<&'static str> = Vec::new();
    let mut taken_ghosts: Vec<FillerItem> = Vec::new();

    let mut always_hints = generate_always_hints(
        settings, world_graph, check_map, &mut taken_checks, &mut taken_ghosts, rng,
    );
    let mut path_hints = generate_path_hints(
        settings, rng, world_graph, check_map, &mut taken_checks, &mut taken_ghosts,
    );

    let num_sometimes_hints = NUM_TOTAL_HINTS - always_hints.len() - path_hints.len();
    let mut sometimes_hints = generate_sometimes_hints(
        settings, world_graph, rng, check_map, num_sometimes_hints, &mut taken_checks,
        &mut taken_ghosts,
    );

    duplicate_hints(
        &mut taken_ghosts, &mut always_hints, &mut path_hints, &mut sometimes_hints,
        NUM_TOTAL_HINTS, rng,
    );

    let bow_of_light_hint = Some(generate_bow_of_light_hint(world_graph, check_map));

    Hints { path_hints, always_hints, sometimes_hints, bow_of_light_hint }
}

fn duplicate_hints(
    taken_ghosts: &mut Vec<FillerItem>, always_hints: &mut Vec<LocationHint>,
    path_hints: &mut Vec<PathHint>, sometimes_hints: &mut Vec<LocationHint>,
    num_total_hints: usize, rng: &mut StdRng,
) {
    assert_eq!(
        taken_ghosts.len(),
        num_total_hints,
        "Only {} of the expected {} hint ghosts were taken",
        taken_ghosts.len(),
        num_total_hints
    );
    let hint_count = always_hints.len() + path_hints.len() + sometimes_hints.len();
    assert_eq!(
        hint_count, num_total_hints,
        "Only {} of the expected {} hints were actually created",
        hint_count, num_total_hints
    );

    // todo probably don't need to duplicate this

    let mut ghosts = FillerItem::get_all_ghosts();
    ghosts.retain(|ghost| !taken_ghosts.contains(ghost));

    for hint in always_hints {
        hint.ghosts.push(ghosts.remove(rng.gen_range(0..ghosts.len())));
    }

    for hint in path_hints {
        hint.ghosts.push(ghosts.remove(rng.gen_range(0..ghosts.len())));
    }

    for hint in sometimes_hints {
        hint.ghosts.push(ghosts.remove(rng.gen_range(0..ghosts.len())));
    }

    assert_eq!(ghosts.len(), 0, "There were leftover Hint Ghosts: {:?}", ghosts);
}

/// Generates the Bow of Light Hint
/// todo need a generic "find where item be at" function
fn generate_bow_of_light_hint(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap,
) -> BowOfLightHint {
    for (_, location_node) in world_graph {
        for &check in location_node.clone().get_checks() {
            if BowOfLight.eq(&check_map.get(check.get_name()).unwrap().unwrap()) {
                return BowOfLightHint { check };
            }
        }
    }

    panic!("Failed to generate Bow of Light Hint");
}

/**
 * Always Hints
 * Generates hints for checks that should always be hinted, depending on settings.
 */
fn generate_always_hints(
    settings: &Settings, world_graph: &mut WorldGraph, check_map: &mut CheckMap,
    taken_checks: &mut Vec<&'static str>, taken_ghosts: &mut Vec<FillerItem>, rng: &mut StdRng,
) -> Vec<LocationHint> {
    let mut always_checks =
        vec!["Master Sword Pedestal", "Great Rupee Fairy", "Blacksmith (Lorule)", "Bouldering Guy"];

    // todo
    // if settings.logic.nice_mode {
    //     always_checks.extend(vec![" 30 Maiamai", " 40 Maiamai", " 50 Maiamai"]);
    // }

    if !settings.logic.minigames_excluded {
        always_checks.extend(vec!["Octoball Derby", "Treacherous Tower Intermediate"]);
    }

    let mut always_hints = Vec::new();
    for check_name in always_checks {
        let mut location_hint =
            generate_location_hint(check_name, settings, world_graph, check_map);
        if location_hint.choose_ghost(rng, taken_ghosts).is_err() {
            continue;
        }
        always_hints.push(location_hint);
        taken_checks.push(check_name);
    }

    always_hints
}

fn generate_location_hint(
    check_name: &'static str, settings: &Settings, world_graph: &mut WorldGraph,
    check_map: &mut CheckMap,
) -> LocationHint {
    // fixme this sucks
    let mut check = None;
    'outer: for (_, loc_node) in world_graph.clone() {
        for c in loc_node.get_checks().clone() {
            if check_name.eq(c.get_name()) {
                check = Some(c);
                break 'outer;
            }
        }
    }

    let (item, check) = if let Some(check) = check {
        (check_map.get(check.get_name()).unwrap().unwrap(), check)
    } else {
        fail!("Failed to lookup Check from check_name: {}", check_name);
    };

    let logical_ghosts = find_checks_before_goal(settings, world_graph, check_map, item)
        .iter()
        .filter_map(|check| {
            if let Some(quest) = check.get_quest() {
                if quest.is_hint_ghost() {
                    return Some(quest);
                }
            };
            return None;
        })
        .collect::<Vec<_>>();

    LocationHint { item, check, logical_ghosts, ghosts: vec![] }
}

/**
 * Sometimes Hints
 * Generates hints for checks that are only "sometimes" hinted, depending on settings. The checks
 * that get hinted are chosen randomly.
 */
fn generate_sometimes_hints(
    settings: &Settings, world_graph: &mut WorldGraph, rng: &mut StdRng, check_map: &mut CheckMap,
    num_sometimes_hints: usize, taken_checks: &mut Vec<&'static str>,
    taken_ghosts: &mut Vec<FillerItem>,
) -> Vec<LocationHint> {
    let mut sometimes_checks = vec![
        "Bee Guy (2)",
        "Behind Ice Gimos",
        "Bird Lover",
        "Blacksmith",
        "Blacksmith Cave",
        "Cucco Treasure Dungeon",
        "Death Mountain Treasure Dungeon",
        "Donkey Cave Pegs",
        "Eastern Ruins Peg Circle",
        "Eastern Ruins Treasure Dungeon",
        "Fire Cave Pillar",
        "Floating Island",
        "Graveyard Ledge Cave",
        "Ice Gimos Fight",
        "Ice Rod Cave",
        "Irene",
        "Ku's Domain Fight",
        "Lorule Field Treasure Dungeon",
        "Milk Bar Owner",
        "Misery Mire Ledge",
        "Misery Mire Treasure Dungeon",
        "Osfala",
        "Philosopher's Cave",
        "Queen Oren",
        "Rosso",
        "Rosso Rocks",
        "Shady Guy",
        "Spectacle Rock",
        "Southern Ruins Treasure Dungeon",
        "Street Merchant (Right)",
        "Thief Girl Cave",
        "Waterfall Cave",
        "Wildlife Clearing Stump",
        "Woman",
        "Zelda",
        "Zora's River Treasure Dungeon",
        "[DP] (2F) Under Rock (Ball Room)",
        "[DP] (2F) Under Rock (Left)",
        "[DP] (2F) Under Rock (Right)",
        "[EP] (1F) Escape Chest",
        "[HC] Battlement",
        "[HC] West Wing",
        "[HG] (3F) Fire Bubbles",
        "[HG] (2F) Fire Ring",
        "[IR] (B2) Long Merge Chest",
        "[IR] (B4) Southeast Chest (Fall)",
        "[LC] (3F) Ball Trial (Puzzle)",
        "[LC] (3F) Bomb Trial (Behind Rock)",
        "[LC] (4F) Hookshot Trial (Eyes)",
        "[LC] (4F) Lamp Trial",
        "[PD] (2F) Big Chest (Hidden)",
        "[PD] (B1) Big Chest (Switches)",
        "[SP] (B1) Big Chest (Secret)",
        "[SW] (B1) Big Chest (Eyes)",
        "[SW] (B1) South Chest",
        "[T'H] (B2) Eyegores",
        "[T'H] (B3) Big Chest (Hidden)",
        "[TH] (8F) Fairy Room",
        "[TR] (B1) Big Chest (Center)",
        "[TR] (1F) Defeat Flamolas",
    ];

    // Maiamai Madness
    if settings.logic.maiamai_madness {
        sometimes_checks.extend(vec![
            "[Mai] Blacksmith Tornado Tile",
            "[Mai] Buried in the Desert",
            "[Mai] Buried near Desert Palace",
            "[Mai] Cucco Treasure Dungeon Big Rock",
            "[Mai] Dark Ruins South Area Wall",
            "[Mai] Death Mountain East Ledge Rock",
            "[Mai] Eastern Ruins Big Rock",
            "[Mai] Hyrule Castle Tornado Tile",
            "[Mai] Hyrule Hotfoot Big Rock",
            "[Mai] Hyrule Rupee Rush Wall",
            "[Mai] Island Tornado Tile",
            "[Mai] Kakariko Sand",
            "[Mai] Ku's Domain Water",
            "[Mai] Lorule Death Mountain East Skull",
            "[Mai] Lorule Death Mountain West Big Rock",
            "[Mai] Lorule Fortune-Teller Big Rock",
            "[Mai] Lorule Graveyard Peninsula Tree",
            "[Mai] Lorule Lake Big Rock",
            "[Mai] Lorule Lake Skull",
            "[Mai] Lorule Rupee Rush Wall",
            "[Mai] Rosso's Ore Mine Rock",
            "[Mai] Skull Woods Big Rock",
            "[Mai] Southern Ruins Big Rock",
            "[Mai] Southern Ruins Bomb Cave",
        ]);
    }

    // Nice Mode
    // todo
    // if settings.logic.nice_mode {
    //     sometimes_checks.extend(vec![" 20 Maiamai"]);
    // }

    // Minigames
    if !settings.logic.minigames_excluded {
        sometimes_checks.extend(vec![
            "Dodge the Cuccos",
            "Rupee Rush (Hyrule)",
            "Rupee Rush (Lorule)",
            "Hyrule Hotfoot (First Race)",
        ]);
    }

    sometimes_checks.retain(|check| !taken_checks.contains(check));

    let mut sometimes_hints = Vec::new();
    let mut sometimes_hint_count = 0;
    loop {
        if sometimes_hint_count >= num_sometimes_hints {
            break;
        }

        if sometimes_checks.is_empty() {
            debug!("Ran out of possible Sometimes Hints");
            break;
        }

        let selected_hint = sometimes_checks.remove(rng.gen_range(0..sometimes_checks.len()));
        let mut location_hint =
            generate_location_hint(selected_hint, settings, world_graph, check_map);
        if location_hint.choose_ghost(rng, taken_ghosts).is_err() {
            continue;
        }
        sometimes_hints.push(location_hint);

        sometimes_hint_count += 1;
    }

    sometimes_hints
}

/**
 * Path Hints
 *
 * Generates up to 6 Path Hints for each Boss guarding a Sage Portrait.
 *
 * A "Path Hint" is a hint that specifies the location of a "Path Item" that is required to reach
 * and defeat a certain Boss, according the chosen Logic Mode and Settings.
 */
fn generate_path_hints(
    settings: &Settings, rng: &mut StdRng, world_graph: &mut WorldGraph, check_map: &mut CheckMap,
    taken_checks: &mut Vec<&'static str>, taken_ghosts: &mut Vec<FillerItem>,
) -> Vec<PathHint> {
    let mut bosses_and_prize_locations: Vec<(FillerItem, &str)> = vec![
        (Yuga, "Eastern Palace Prize"),
        (Margomill, "House of Gales Prize"),
        (Moldorm, "Tower of Hera Prize"),
        // (ZeldasThrone, "Hyrule Castle Prize"),
        (GemesaurKing, "Dark Palace Prize"),
        (Arrghus, "Swamp Palace Prize"),
        (Knucklemaster, "Skull Woods Prize"),
        (Stalblind, "Thieves' Hideout Prize"),
        (Grinexx, "Turtle Rock Prize"),
        (Zaganaga, "Desert Palace Prize"),
        (Dharkstare, "Ice Ruins Prize"),
    ];

    bosses_and_prize_locations = util::shuffle(rng, bosses_and_prize_locations);

    let mut chosen_paths: Vec<PathHint> = Vec::new();
    let mut backup_paths: Vec<PathHint> = Vec::new();
    let mut extra_paths_needed = 0;

    for (goal, prize_loc) in bosses_and_prize_locations {
        if is_sage(convert(check_map.get(prize_loc).unwrap().unwrap()).unwrap()) {
            let mut potential_paths =
                get_potential_path_hints(settings, rng, world_graph, check_map, taken_checks, goal);
            if let Some(chosen_path) =
                choose_path_hint(&mut potential_paths, taken_checks, taken_ghosts, rng)
            {
                chosen_paths.push(chosen_path);
                backup_paths.extend(potential_paths);
            } else {
                debug!("No Path Hints possible for Goal: {}", goal.as_str());
                extra_paths_needed += 1;
            }
        }
    }

    // Add extra paths if some bosses didn't have any path items
    if extra_paths_needed > 0 {
        backup_paths = util::shuffle(rng, backup_paths);
        let mut extra_paths_added = 0;
        loop {
            if extra_paths_added >= extra_paths_needed || backup_paths.is_empty() {
                break;
            }

            if let Some(backup_path) =
                choose_path_hint(&mut backup_paths, taken_checks, taken_ghosts, rng)
            {
                chosen_paths.push(backup_path);
                extra_paths_added += 1;
            }
        }
    }

    chosen_paths
}

fn choose_path_hint(
    potential_paths: &mut Vec<PathHint>, taken_checks: &mut Vec<&'static str>,
    taken_ghosts: &mut Vec<FillerItem>, rng: &mut StdRng,
) -> Option<PathHint> {
    potential_paths.retain(|path| !taken_checks.contains(&path.check.get_name()));

    for chosen_path in potential_paths {
        // Choose a random Ghost for this hint of the ones not already taken
        match chosen_path
            .logical_ghosts
            .iter()
            .filter(|&ghost| !taken_ghosts.contains(ghost))
            .choose_stable(rng)
        {
            None => {
                info!("No available Ghosts to give Hint: {:?}", chosen_path);
            }
            Some(chosen_ghost) => {
                chosen_path.ghosts.push(*chosen_ghost);
                taken_checks.push(chosen_path.check.get_name());
                taken_ghosts.push(*chosen_ghost);
                return Some(chosen_path.clone());
            }
        }
    }

    return None;
}

/// Finds all checks available before a given Quest Goal using a modified Sphere Search.
fn find_checks_before_goal(
    settings: &Settings, world_graph: &mut WorldGraph, check_map: &mut CheckMap, goal: FillerItem,
) -> HashSet<Check> {
    let mut progress = Progress::new(settings.clone());
    let mut reachable_checks: Vec<Check>;
    let mut potential_path_checks: HashSet<Check> = HashSet::new();

    // Find candidate Path Checks with a modified sphere search
    loop {
        reachable_checks = util::find_reachable_checks(world_graph, &progress);
        potential_path_checks.extend(&reachable_checks);
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, check_map, settings);

        let new_items = reachable_items.difference(&progress);

        if reachable_items.has(goal) || new_items.is_empty() {
            break;
        }

        for new_item in &new_items {
            progress.add_item(*new_item);
        }
    }

    potential_path_checks
}

/// Determines the possible Path Hints for a given goal, if any exist. Paths are returned in a random order.
fn get_potential_path_hints(
    settings: &Settings, rng: &mut StdRng, world_graph: &mut WorldGraph, check_map: &mut CheckMap,
    taken_checks: &mut Vec<&str>, goal: FillerItem,
) -> Vec<PathHint> {
    let mut reachable_checks: Vec<Check>;
    let mut potential_paths: Vec<PathHint> = Vec::new();

    let mut potential_path_checks = find_checks_before_goal(settings, world_graph, check_map, goal);

    // Limit potential paths to locations with valid Path Items that haven't yet been taken
    potential_path_checks.retain(|check| {
        !taken_checks.contains(&check.get_name())
            && POSSIBLE_PATH_ITEMS.contains(&check_map.get(check.get_name()).unwrap().unwrap())
    });

    // Test candidate items to see if Boss can be defeated without them
    for check in potential_path_checks {
        // Reset Progression
        let mut progress = Progress::new(settings.clone());

        loop {
            reachable_checks = util::find_reachable_checks(world_graph, &progress);

            // Remove Potential Path Location
            reachable_checks.retain(|c| check.ne(c));

            let reachable_items =
                get_items_from_reachable_checks(&reachable_checks, check_map, settings);

            let new_items = reachable_items.difference(&progress);

            if new_items.is_empty() {
                // Item could be Path if goal couldn't be reached without it
                if !progress.has(goal) {
                    let hint_locations = util::shuffle(
                        rng,
                        reachable_items
                            .get_items()
                            .iter()
                            .filter_map(
                                |&item| if item.is_hint_ghost() { Some(item) } else { None },
                            )
                            .collect::<_>(),
                    );

                    potential_paths.push(PathHint {
                        goal,
                        check,
                        ghosts: vec![],
                        logical_ghosts: hint_locations,
                    });
                }
                break;
            }

            for new_item in &new_items {
                progress.add_item(*new_item);
            }
        }
    }

    util::shuffle(rng, potential_paths)
}

const POSSIBLE_PATH_ITEMS: [FillerItem; 48] = [
    Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Bombs01, Bombs02, FireRod01,
    FireRod02, IceRod01, IceRod02, Hammer01, Hammer02, SandRod01, SandRod02, TornadoRod01,
    TornadoRod02, Bell, StaminaScroll, PegasusBoots, Flippers, HylianShield,
    SmoothGem, //LetterInABottle,
    PremiumMilk, HintGlasses, GreatSpin, Bottle01, Bottle02, Bottle03, Bottle04, Bottle05, Lamp01,
    Lamp02, Sword01, Sword02, Sword03, Sword04, Glove01, Glove02, Net01, Net02, Mail01, Mail02,
    OreYellow, OreGreen, OreBlue, OreRed,
];

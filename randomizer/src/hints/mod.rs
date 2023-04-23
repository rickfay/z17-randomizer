use {
    crate::{
        convert, fail,
        filler::{find_reachable_checks, get_items_from_reachable_checks},
        filler_util::shuffle,
        item_to_str,
        model::{check::Check, progress::Progress, Hints},
        patch::util::is_sage,
        world::WorldGraph,
        CheckMap,
        FillerItem::{self, *},
        Settings,
    },
    log::{debug, info},
    rand::{rngs::StdRng, Rng},
    serde::{Serialize, Serializer},
    std::collections::{HashMap, HashSet},
};
pub mod formatting;
pub mod hint_color;
mod hints;

/// Generates Always, Path, and Sometimes Hints based on settings
pub fn generate_hints(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap, settings: &Settings, rng: &mut StdRng,
) -> Hints {
    info!("Generating Hints...");

    const NUM_TOTAL_HINTS: usize = 29;
    let mut taken_checks: Vec<&'static str> = Vec::new();
    let mut taken_ghosts: Vec<FillerItem> = Vec::new();

    let always_hints = generate_always_hints(settings, check_map, &mut taken_checks);
    let path_hints = generate_path_hints(
        settings, rng, world_graph, check_map, &mut taken_checks, &mut taken_ghosts,
    );

    let num_sometimes_hints = NUM_TOTAL_HINTS - always_hints.len() - path_hints.len();
    let sometimes_hints =
        generate_sometimes_hints(settings, rng, check_map, num_sometimes_hints, &mut taken_checks);

    let bow_of_light_hint = generate_bow_of_light_hint(world_graph, check_map);

    Hints { path_hints, always_hints, sometimes_hints, bow_of_light_hint }
}

/// Generates the Bow of Light Hint
/// todo need a generic "find where item be at" function
fn generate_bow_of_light_hint(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap,
) -> Vec<&'static str> {
    for (_, location_node) in world_graph {
        for check in location_node.clone().get_checks() {
            if BowOfLight.eq(&check_map.get(check.get_name()).unwrap().unwrap()) {
                return vec![check.get_location_info().unwrap().region()];
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
    settings: &Settings, check_map: &mut CheckMap, taken_checks: &mut Vec<&'static str>,
) -> HashMap<&'static str, &'static str> {
    let mut always_checks =
        vec!["Master Sword Pedestal", "Great Rupee Fairy", "Blacksmith (Lorule)", "Bouldering Guy"];

    // todo
    // if settings.logic.nice_mode {
    //     always_checks.extend(vec![" 30 Maiamai", " 40 Maiamai", " 50 Maiamai"]);
    // }

    if !settings.logic.minigames_excluded {
        always_checks.extend(vec!["Octoball Derby", "Treacherous Tower Intermediate"]);
    }

    let mut always_hints = HashMap::new();
    for check_name in always_checks {
        taken_checks.push(check_name);
        let filler_item = check_map.get(check_name).unwrap().unwrap();
        always_hints.insert(check_name, item_to_str(&convert(filler_item).unwrap()));
    }

    always_hints
}

/**
 * Sometimes Hints
 * Generates hints for checks that are only "sometimes" hinted, depending on settings. The checks
 * that get hinted are chosen randomly.
 */
fn generate_sometimes_hints(
    settings: &Settings, rng: &mut StdRng, check_map: &mut CheckMap, num_sometimes_hints: usize,
    taken_checks: &mut Vec<&'static str>,
) -> HashMap<&'static str, &'static str> {
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

    let mut sometimes_hints = HashMap::new();
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
        let filler_item = check_map.get(selected_hint).unwrap().unwrap();
        sometimes_hints.insert(selected_hint, item_to_str(&convert(filler_item).unwrap()));

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

    bosses_and_prize_locations = shuffle(rng, bosses_and_prize_locations);

    let mut chosen_paths: Vec<PathHint> = Vec::new();
    let mut backup_paths: Vec<PotentialPathHint> = Vec::new();
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
        backup_paths = shuffle(rng, backup_paths);
        let mut i = 0;
        loop {
            if i >= extra_paths_needed || backup_paths.is_empty() {
                break;
            }

            if let Some(backup_path) =
                choose_path_hint(&mut backup_paths, taken_checks, taken_ghosts, rng)
            {
                chosen_paths.push(backup_path);
                i += 1;
            }
        }
    }

    chosen_paths
}

fn choose_path_hint(
    potential_paths: &mut Vec<PotentialPathHint>, taken_checks: &mut Vec<&'static str>,
    taken_ghosts: &mut Vec<FillerItem>, rng: &mut StdRng,
) -> Option<PathHint> {
    let mut chosen_path;
    let mut chosen_ghost;
    'outer: loop {
        if potential_paths.is_empty() {
            debug!("No more potential paths");
            return None;
        }

        chosen_path = potential_paths.remove(rng.gen_range(0..potential_paths.len()));

        // Prevent reusing existing check
        if taken_checks.contains(&chosen_path.check.get_name()) {
            continue;
        }

        loop {
            if chosen_path.ghosts.is_empty() {
                debug!("No available Ghosts to give Hint: {:?}", chosen_path);
                continue 'outer;
            }

            chosen_ghost = chosen_path.ghosts.remove(rng.gen_range(0..chosen_path.ghosts.len()));
            if !taken_ghosts.contains(&chosen_ghost) {
                break;
            }
        }

        taken_checks.push(chosen_path.check.get_name());
        taken_ghosts.push(chosen_ghost);

        break;
    }

    Some(PathHint { goal: chosen_path.goal, check: chosen_path.check, ghost: chosen_ghost })
}

/// Struct containing information about a possible Hint and where it could be placed
#[derive(Debug, Clone)]
pub struct PotentialPathHint {
    /// Goal this Path Hint leads to
    pub goal: FillerItem,
    /// Specific location Check containing the Path Item
    pub check: Check,
    /// Hint Ghosts that will could potentially give out this hint
    pub ghosts: Vec<FillerItem>,
}

/// Path Hint
#[derive(Debug, Clone)]
pub struct PathHint {
    /// Goal this Path Hint leads to
    pub goal: FillerItem,
    /// Specific location Check containing the Path Item
    pub check: Check,
    /// Hint Ghost that will give out this hint
    pub ghost: FillerItem,
}

impl PathHint {
    pub fn to_str(&self) -> String {
        format!(
            "It says here that {} is on the path to {}.",
            self.check.get_location_info().unwrap().region(),
            self.goal.as_str()
        )
    }

    pub fn to_hint(&self) -> String {
        format!(
            "It says here that {}\nis on the path to {}.",
            &self.check.get_location_info().unwrap().region_colorized(),
            &self.goal.as_str_colorized()
        )
    }
}

impl Serialize for PathHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // todo I don't really like this...
        serializer.serialize_str(format!("{} [{}]", self.to_str(), self.ghost.as_str()).as_str())
    }
}

/// Determines the possible Path Hints for a given goal, if any exist.
fn get_potential_path_hints(
    settings: &Settings, rng: &mut StdRng, world_graph: &mut WorldGraph, check_map: &mut CheckMap,
    taken_checks: &mut Vec<&str>, goal: FillerItem,
) -> Vec<PotentialPathHint> {
    let mut progress = Progress::new(settings.clone());
    let mut reachable_checks: Vec<Check>;
    let mut potential_paths: Vec<PotentialPathHint> = Vec::new();

    let mut potential_path_checks: HashSet<Check> = HashSet::new();

    // Find candidate Path Checks with a modified sphere search
    loop {
        reachable_checks = find_reachable_checks(world_graph, &progress);
        potential_path_checks.extend(&reachable_checks);
        let reachable_items =
            get_items_from_reachable_checks(&reachable_checks, check_map, settings);

        let new_items = reachable_items.difference(&progress);

        if new_items.is_empty() {
            if progress.has(goal) {
                break; // success
            } else {
                fail!("No possible path to reach Goal: {}", goal.as_str());
            }
        }

        for new_item in &new_items {
            progress.add_item(*new_item);
        }
    }

    // Limit potential paths to locations with valid Path Items that haven't yet been taken
    potential_path_checks.retain(|check| {
        !taken_checks.contains(&check.get_name())
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
                get_items_from_reachable_checks(&reachable_checks, check_map, settings);

            let new_items = reachable_items.difference(&progress);

            if new_items.is_empty() {
                // Item could be Path if goal couldn't be reached without it
                if !progress.has(goal) {
                    let hint_locations = shuffle(
                        rng,
                        reachable_items
                            .get_items()
                            .iter()
                            .filter_map(
                                |&item| if item.is_hint_ghost() { Some(item) } else { None },
                            )
                            .collect::<_>(),
                    );

                    potential_paths.push(PotentialPathHint { goal, check, ghosts: hint_locations });
                }
                break;
            }

            for new_item in &new_items {
                progress.add_item(*new_item);
            }
        }
    }

    potential_paths
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

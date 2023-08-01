use log::{debug, info};
use modd::{
    hints::{formatting, Hint, HintColor, Hints},
    Settings,
};
use rand::{rngs::StdRng, seq::IteratorRandom, Rng};

use crate::{
    convert,
    filler::{find_reachable_checks, get_items_from_reachable_checks},
    filler_util::shuffle,
    model::{check::Check, progress::Progress},
    patch::util::is_sage,
    world::WorldGraph,
    CheckMap, Error,
    FillerItem::{self, *},
    Result,
};

mod text;

/// A [`Hint`] that exposes the item at a certain location
#[derive(Debug, Clone)]
pub struct LocationHint {
    /// The hinted item
    pub item: FillerItem,

    /// The specific [`Check`] containing the hinted item.
    pub check: Check,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    pub logical_ghosts: Vec<FillerItem>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    pub ghosts: Vec<FillerItem>,
}

impl LocationHint {
    pub(crate) fn choose_ghost(
        &mut self, rng: &mut StdRng, taken_ghosts: &mut Vec<FillerItem>,
    ) -> Result<(), &'static str> {
        match self
            .logical_ghosts
            .iter()
            .filter(|ghost| !taken_ghosts.contains(ghost))
            .choose_stable(rng)
        {
            None => Err("No Ghosts available to place this hint"),
            Some(ghost) => {
                self.ghosts.push(*ghost);
                taken_ghosts.push(*ghost);
                Ok(())
            }
        }
    }
}

impl Hint for LocationHint {
    fn get_hint(&self) -> Option<String> {
        let article = self.item.get_article().ok()?;
        Some(format!(
            "{}\nhas {}{}{}.",
            &self.check.get_location_info().unwrap().name,
            article,
            if article.is_empty() { "" } else { " " },
            &self.item.as_str_colorized()
        ))
    }

    fn get_hint_spoiler(&self) -> Option<String> {
        let article = self.item.get_article().ok()?;
        Some(format!(
            "{} has {}{}{}.",
            &self.check.get_location_info().unwrap().name,
            article,
            if article.is_empty() { "" } else { " " },
            &self.item.as_str()
        ))
    }

    fn ghosts(&self) -> &[FillerItem] {
        &self.ghosts
    }
}

/// A [`Hint`] that tells where an item needed to reach a specific `gaol` is located.
#[derive(Debug, Clone)]
pub struct PathHint {
    /// The specific [`Check`] containing the hinted item.
    pub check: Check,

    /// The goal that this hint leads to.
    pub goal: FillerItem,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    pub logical_ghosts: Vec<FillerItem>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    pub ghosts: Vec<FillerItem>,
}

impl Hint for PathHint {
    fn get_hint(&self) -> Option<String> {
        Some(format!(
            "{}\nis on the path to\n{}",
            &HintColor::Name.format(self.check.get_location_info().unwrap().region()),
            &self.goal.as_str_colorized()
        ))
    }

    fn get_hint_spoiler(&self) -> Option<String> {
        Some(format!(
            "{} is on the path to {}",
            self.check.get_location_info().unwrap().region(),
            self.goal.as_str()
        ))
    }

    fn ghosts(&self) -> &[FillerItem] {
        &self.ghosts
    }
}

/// A [`Hint`] specifically for the Bow of Light.
#[derive(Debug, Clone)]
pub struct BowOfLightHint {
    /// The specific [`Check`] containing the Bow of Light.
    pub check: Check,
}

impl Hint for BowOfLightHint {
    fn get_hint(&self) -> Option<String> {
        Some(format!(
            "Did you find the {}\nin {}?",
            formatting::name("Bow of Light"),
            &HintColor::Name.format(self.check.get_location_info().unwrap().region()),
        ))
    }

    fn get_hint_spoiler(&self) -> Option<String> {
        Some(format!(
            "Did you find the Bow of Light in {}?",
            &self.check.get_location_info().unwrap().region()
        ))
    }

    fn ghosts(&self) -> &[FillerItem] {
        &[]
    }
}

/// Generates Always, Path, and Sometimes Hints based on settings
pub fn generate_hints(
    world_graph: &mut WorldGraph, check_map: &mut CheckMap, settings: &Settings, rng: &mut StdRng,
) -> Result<Hints> {
    info!("Generating Hints...");

    const NUM_TOTAL_HINTS: usize = 29;
    let mut taken_checks: Vec<&'static str> = Vec::new();
    let mut taken_ghosts: Vec<FillerItem> = Vec::new();

    let always_hints = generate_always_hints(
        settings, world_graph, check_map, &mut taken_checks, &mut taken_ghosts, rng,
    )?;
    let path_hints = generate_path_hints(
        settings, rng, world_graph, check_map, &mut taken_checks, &mut taken_ghosts,
    )?;

    let num_sometimes_hints = NUM_TOTAL_HINTS - always_hints.len() - path_hints.len();
    let sometimes_hints = generate_sometimes_hints(
        settings, world_graph, rng, check_map, num_sometimes_hints, &taken_checks,
        &mut taken_ghosts,
    )?;

    let bow_of_light_hint =
        Some(Box::new(generate_bow_of_light_hint(world_graph, check_map)) as Box<_>);

    Ok(Hints { path_hints, always_hints, sometimes_hints, bow_of_light_hint })
}

#[allow(unused)]
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
    for location_node in world_graph.values_mut() {
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
) -> Result<Vec<Box<dyn Hint>>> {
    let mut always_checks = vec![
        "Master Sword Pedestal", "Great Rupee Fairy", "Blacksmith (Lorule)", "Bouldering Guy",
        "Irene", "Rosso", "Osfala", "Wildlife Clearing Stump",
    ];

    // todo
    // if settings.logic.nice_mode {
    //     always_checks.extend(vec![" 30 Maiamai", " 40 Maiamai", " 50 Maiamai"]);
    // }

    if settings.logic.reverse_sage_events {
        always_checks.extend(vec!["Queen Oren", "Hyrule Castle Battlement"]);
    }

    if !settings.logic.minigames_excluded {
        always_checks.extend(vec!["Octoball Derby", "Treacherous Tower Intermediate"]);
    }

    let mut always_hints = Vec::new();
    for check_name in always_checks {
        let mut location_hint =
            generate_location_hint(check_name, settings, world_graph, check_map)?;
        if location_hint.choose_ghost(rng, taken_ghosts).is_err() {
            continue;
        }
        always_hints.push(Box::new(location_hint) as Box<_>);
        taken_checks.push(check_name);
    }

    Ok(always_hints)
}

fn generate_location_hint(
    check_name: &'static str, settings: &Settings, world_graph: &mut WorldGraph,
    check_map: &mut CheckMap,
) -> Result<LocationHint> {
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
        return Err(Error::new(format!("Failed to lookup Check from check_name: {}", check_name)));
    };

    let logical_ghosts = find_checks_before_goal(settings, world_graph, check_map, item)?
        .iter()
        .filter_map(|check| {
            if let Some(quest) = check.get_quest() {
                if quest.is_hint_ghost() {
                    return Some(quest);
                }
            };
            None
        })
        .collect::<Vec<_>>();

    Ok(LocationHint { item, check, logical_ghosts, ghosts: vec![] })
}

/**
 * Sometimes Hints
 * Generates hints for checks that are only "sometimes" hinted, depending on settings. The checks
 * that get hinted are chosen randomly.
 */
fn generate_sometimes_hints(
    settings: &Settings, world_graph: &mut WorldGraph, rng: &mut StdRng, check_map: &mut CheckMap,
    num_sometimes_hints: usize, taken_checks: &[&'static str], taken_ghosts: &mut Vec<FillerItem>,
) -> Result<Vec<Box<dyn Hint>>> {
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
        "Ku's Domain Fight",
        "Lorule Field Treasure Dungeon",
        "Misery Mire Ledge",
        "Misery Mire Treasure Dungeon",
        "Philosopher's Cave",
        "Queen Oren",
        "Rosso Rocks",
        "Shady Guy",
        "Spectacle Rock",
        "Southern Ruins Treasure Dungeon",
        "Street Merchant (Right)",
        "Thief Girl",
        "Waterfall Cave",
        "Woman",
        "Zelda",
        "Zora's River Treasure Dungeon",
        "[DP] (2F) Under Rock (Ball Room)",
        "[DP] (2F) Under Rock (Left)",
        "[DP] (2F) Under Rock (Right)",
        "[EP] (1F) Escape Chest",
        "Hyrule Castle Battlement",
        "Hyrule Castle West Wing",
        "[HG] (3F) Fire Bubbles",
        "[HG] (2F) Fire Ring",
        "[IR] (B2) Long Merge Chest",
        "[IR] (B4) Southeast Chest (Fall)",
        "[LC] (3F) Spike Ball Chest",
        "[LC] (3F) Big Bomb Flower Chest",
        "[LC] (4F) Eyeball Chest",
        "[LC] (4F) Lamp Trial Chest",
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
            "[Mai] Cucco Dungeon Big Rock",
            "[Mai] Dark Ruins South Area Wall",
            "[Mai] Death Mountain East Ledge",
            "[Mai] Eastern Ruins Big Rock",
            "[Mai] Hyrule Castle Tornado Tile",
            "[Mai] Hyrule Hotfoot Big Rock",
            "[Mai] Hyrule Rupee Rush Wall",
            "[Mai] Island Tornado Tile",
            "[Mai] Kakariko Sand",
            "[Mai] Ku's Domain Water",
            "[Mai] Lorule Mountain E Skull",
            "[Mai] Lorule Mountain W Big Rock",
            "[Mai] Near Lorule Fortune-Teller",
            "[Mai] Lorule Graveyard Tree",
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
            generate_location_hint(selected_hint, settings, world_graph, check_map)?;
        if location_hint.choose_ghost(rng, taken_ghosts).is_err() {
            continue;
        }
        sometimes_hints.push(Box::new(location_hint) as Box<_>);

        sometimes_hint_count += 1;
    }

    Ok(sometimes_hints)
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
) -> Result<Vec<Box<dyn Hint>>> {
    let mut bosses_and_prize_locations: Vec<(FillerItem, &str)> = vec![
        (Yuga, "Eastern Palace Prize"),
        (Margomill, "House of Gales Prize"),
        (Moldorm, "Tower of Hera Prize"),
        (ZeldasThrone, "Hyrule Castle Prize"),
        (GemesaurKing, "Dark Palace Prize"),
        (Arrghus, "Swamp Palace Prize"),
        (Knucklemaster, "Skull Woods Prize"),
        (Stalblind, "Thieves' Hideout Prize"),
        (Grinexx, "Turtle Rock Prize"),
        (Zaganaga, "Desert Palace Prize"),
        (Dharkstare, "Ice Ruins Prize"),
    ];

    bosses_and_prize_locations = shuffle(rng, bosses_and_prize_locations);

    let mut chosen_paths = Vec::new();
    let mut backup_paths = Vec::new();
    let mut extra_paths_needed = 0;

    for (goal, prize_loc) in bosses_and_prize_locations {
        if is_sage(convert(check_map.get(prize_loc).unwrap().unwrap()).unwrap()) {
            let mut potential_paths = get_potential_path_hints(
                settings, rng, world_graph, check_map, taken_checks, goal,
            )?;

            // fixme slow
            potential_paths.iter_mut().for_each(|i: &mut PathHint| {
                i.logical_ghosts.sort_by_key(|p| p.as_str());
            });

            if let Some(chosen_path) =
                choose_path_hint(&mut potential_paths, taken_checks, taken_ghosts, rng)
            {
                chosen_paths.push(Box::new(chosen_path) as Box<_>);
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
        let mut extra_paths_added = 0;
        loop {
            if extra_paths_added >= extra_paths_needed || backup_paths.is_empty() {
                break;
            }

            if let Some(backup_path) =
                choose_path_hint(&mut backup_paths, taken_checks, taken_ghosts, rng)
            {
                chosen_paths.push(Box::new(backup_path) as Box<_>);
                extra_paths_added += 1;
            }
        }
    }

    Ok(chosen_paths)
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

    None
}

/// Finds all checks available before a given Quest Goal using a modified Sphere Search.
fn find_checks_before_goal(
    settings: &Settings, world_graph: &mut WorldGraph, check_map: &mut CheckMap, goal: FillerItem,
) -> Result<Vec<Check>> {
    let mut progress = Progress::new(settings.clone());
    let mut reachable_checks: Vec<Check>;
    let mut potential_path_checks: Vec<Check> = Vec::new();

    // Find candidate Path Checks with a modified sphere search
    loop {
        reachable_checks = find_reachable_checks(world_graph, &progress)?;
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

    Ok(potential_path_checks)
}

/// Determines the possible Path Hints for a given goal, if any exist. Paths are returned in a random order.
fn get_potential_path_hints(
    settings: &Settings, rng: &mut StdRng, world_graph: &mut WorldGraph, check_map: &mut CheckMap,
    taken_checks: &[&str], goal: FillerItem,
) -> Result<Vec<PathHint>> {
    let mut reachable_checks: Vec<Check>;
    let mut potential_paths: Vec<PathHint> = Vec::new();

    let mut potential_path_checks =
        find_checks_before_goal(settings, world_graph, check_map, goal)?;

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
            reachable_checks = find_reachable_checks(world_graph, &progress)?;

            // Remove Potential Path Location
            reachable_checks.retain(|c| check.ne(c));

            let reachable_items =
                get_items_from_reachable_checks(&reachable_checks, check_map, settings);

            let new_items = reachable_items.difference(&progress);

            if new_items.is_empty() {
                // Item could be Path if goal couldn't be reached without it
                if !progress.has(goal) {
                    let hint_locations = reachable_items
                        .get_items()
                        .iter()
                        .filter_map(|&item| if item.is_hint_ghost() { Some(item) } else { None })
                        .collect::<_>();

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

    Ok(shuffle(rng, potential_paths))
}

const POSSIBLE_PATH_ITEMS: [FillerItem; 48] = [
    Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Bombs01, Bombs02, FireRod01,
    FireRod02, IceRod01, IceRod02, Hammer01, Hammer02, SandRod01, SandRod02, TornadoRod01,
    TornadoRod02, Bell, StaminaScroll, PegasusBoots, Flippers, HylianShield, SmoothGem,
    LetterInABottle, PremiumMilk, HintGlasses, GreatSpin, Bottle01, Bottle02, Bottle03, Bottle04,
    Lamp01, Lamp02, Sword01, Sword02, Sword03, Sword04, Glove01, Glove02, Net01, Net02, Mail01,
    Mail02, OreYellow, OreGreen, OreBlue, OreRed,
];

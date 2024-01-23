use crate::filler::check::Check;
use crate::filler::filler_item::{FillerItem, Goal, Item};
use crate::filler::portals::Portal;
use crate::filler::progress::Progress;
use crate::filler::util::shuffle;
use crate::filler::{find_reachable_checks, get_items_from_reachable_checks};
use crate::hints::formatting::name;
use crate::patch::util::is_sage;
use crate::{CheckMap, DashSet, SeedInfo};
use game::ghosts::HintGhost;
use log::{debug, info};
use macros::fail;
use modinfo::settings::portal_shuffle::PortalShuffle;
use rand::seq::IteratorRandom;
use rand::{rngs::StdRng, Rng};
use rom::Error;
use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};
use strum::IntoEnumIterator;
use Item::*;

pub mod formatting;
pub mod hint_color;
mod hints;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Hints {
    pub path_hints: Vec<PathHint>,
    pub always_hints: Vec<LocationHint>,
    pub sometimes_hints: Vec<LocationHint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bow_of_light_hint: Option<BowOfLightHint>,
}

/// Basic functionality for all in-game hints.
pub(crate) trait Hint: Serialize {
    fn get_hint(&self) -> String;
    fn get_hint_spoiler(&self) -> String;
}

/// A [`Hint`] that exposes the item at a certain location
#[derive(Debug, Clone)]
pub struct LocationHint {
    /// The hinted item
    pub item: Item,

    /// The specific [`Check`] containing the hinted item.
    pub check: Check,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    pub logical_ghosts: Vec<HintGhost>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    pub ghosts: Vec<HintGhost>,
}

impl LocationHint {
    pub(crate) fn choose_ghost(
        &mut self, rng: &mut StdRng, taken_ghosts: &mut Vec<HintGhost>,
    ) -> Result<(), &'static str> {
        match self.logical_ghosts.iter().filter(|ghost| !taken_ghosts.contains(ghost)).choose_stable(rng) {
            None => Err("No Ghosts available to place this hint"),
            Some(ghost) => {
                self.ghosts.push(*ghost);
                taken_ghosts.push(*ghost);
                Ok(())
            },
        }
    }
}

impl Hint for LocationHint {
    fn get_hint(&self) -> String {
        let article = self.item.get_article();
        format!(
            "{}\nhas {}{}{}.",
            &self.check.get_location_info().unwrap().name(),
            article,
            if article.is_empty() { "" } else { " " },
            &self.item.as_str_colorized()
        )
    }

    fn get_hint_spoiler(&self) -> String {
        let article = self.item.get_article();
        format!(
            "{} has {}{}{}.",
            &self.check.get_location_info().unwrap().name(),
            article,
            if article.is_empty() { "" } else { " " },
            &self.item.as_str()
        )
    }
}

impl Serialize for LocationHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("LocationHint", 2)?;
        ser.serialize_field("hint", &self.get_hint_spoiler())?;
        ser.serialize_field("ghosts", &SerializeGhosts(&self.ghosts))?;
        ser.end()
    }
}

/// A [`Hint`] that tells where an item needed to reach a specific `gaol` is located.
#[derive(Debug, Clone)]
pub struct PathHint {
    /// The specific [`Check`] containing the hinted item.
    pub check: Check,

    /// The goal that this hint leads to.
    pub goal: Goal,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    pub logical_ghosts: Vec<HintGhost>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    pub ghosts: Vec<HintGhost>,

    /// The underlying Path Item (hidden in-game, visible in Spoiler Log)
    pub path_item: FillerItem,
}

impl Hint for PathHint {
    fn get_hint(&self) -> String {
        format!(
            "{}\nis on the path to\n{}",
            &self.check.get_location_info().unwrap().region_colorized(),
            &self.goal.as_str_colorized()
        )
    }

    fn get_hint_spoiler(&self) -> String {
        format!("{} is on the path to {}", self.check.get_location_info().unwrap().region(), self.goal.as_str())
    }
}

impl Serialize for PathHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("PathHint", 4)?;
        ser.serialize_field("hint", &self.get_hint_spoiler())?;
        ser.serialize_field("path_item", &self.path_item.as_str())?;
        ser.serialize_field("path_item_location", &self.check.get_name())?;
        ser.serialize_field("ghosts", &SerializeGhosts(&self.ghosts))?;
        ser.end()
    }
}

/// A [`Hint`] that reveals where a certain Portal leads
#[derive(Debug, Clone, Serialize)]
pub struct PortalHint {
    /// The Portal whose destination will be hinted
    pub portal: Portal,

    /// The
    pub destination: Portal,

    /// List of Hint Ghosts that are guaranteed to be logically reachable before the hinted item.
    #[serde(skip_serializing)]
    pub logical_ghosts: Vec<HintGhost>,

    /// Hint Ghosts that will give out this hint. <br />
    /// Only one of these is guaranteed to be from `logical_ghosts`, the other(s) are placed completely at random.
    #[serde(skip_serializing)]
    pub ghosts: Vec<HintGhost>,
}

impl Hint for PortalHint {
    fn get_hint(&self) -> String {
        format!("The {} leads to\n{}.", name(self.portal.as_str()), name(self.destination.as_str()))
    }

    fn get_hint_spoiler(&self) -> String {
        format!("The {} leads to\n{}.", self.portal, self.destination)
    }
}

/// A [`Hint`] specifically for the Bow of Light.
#[derive(Debug, Clone)]
pub struct BowOfLightHint {
    /// The specific [`Check`] containing the Bow of Light.
    pub check: Check,
}

impl Hint for BowOfLightHint {
    fn get_hint(&self) -> String {
        format!(
            "Did you find the {}\nin {}?",
            name("Bow of Light"),
            &self.check.get_location_info().unwrap().region_colorized(),
        )
    }

    fn get_hint_spoiler(&self) -> String {
        format!("Did you find the Bow of Light in {}?", &self.check.get_location_info().unwrap().region())
    }
}

impl Serialize for BowOfLightHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.get_hint_spoiler())
    }
}

/// Generates Always, Path, and Sometimes Hints based on settings
pub fn generate_hints(rng: &mut StdRng, seed_info: &mut SeedInfo, check_map: &mut CheckMap) -> Result<(), Error> {
    info!("Generating Hints...");
    const NUM_TOTAL_HINTS: usize = 29;

    //
    let mut taken_checks = seed_info.settings.exclusions.iter().map(|s| s.clone()).collect();
    let mut taken_ghosts = Vec::new();

    // let mut portal_hints =
    //     generate_portal_hints(settings, portal_map, world_graph, check_map, &mut taken_checks, &mut taken_ghosts, rng);

    let mut always_hints = generate_always_hints(rng, seed_info, check_map, &mut taken_checks, &mut taken_ghosts);
    let mut path_hints = generate_path_hints(rng, seed_info, check_map, &mut taken_checks, &mut taken_ghosts);

    let num_sometimes_hints = NUM_TOTAL_HINTS - always_hints.len() - path_hints.len();
    let mut sometimes_hints =
        generate_sometimes_hints(rng, seed_info, check_map, num_sometimes_hints, &taken_checks, &mut taken_ghosts);

    duplicate_hints(&mut taken_ghosts, &mut always_hints, &mut path_hints, &mut sometimes_hints, NUM_TOTAL_HINTS, rng);

    let bow_of_light_hint = generate_bow_of_light_hint(seed_info, check_map);

    seed_info.hints = Hints { path_hints, always_hints, sometimes_hints, bow_of_light_hint };

    Ok(())
}

/// Portal Hints
#[allow(unused)]
fn generate_portal_hints(
    rng: &mut StdRng, seed_info: &mut SeedInfo, check_map: &mut CheckMap, taken_checks: &mut Vec<&str>,
    taken_ghosts: &mut Vec<HintGhost>,
) -> Vec<PortalHint> {
    if seed_info.settings.portal_shuffle == PortalShuffle::Off {
        return Vec::with_capacity(0);
    }

    let portals_to_hint: Vec<Portal> =
        vec![Portal::HyruleCastle, Portal::LoruleCastle, Portal::DesertPalace, Portal::Zaganaga, Portal::RossosHouse];

    for portal in portals_to_hint {
        let portal_hint = PortalHint {
            portal,
            destination: *seed_info.portal_map.get(&portal).expect(&format!("portal_map entry for {}", portal)),
            logical_ghosts: vec![],
            ghosts: vec![],
        };
    }

    todo!()
}

fn duplicate_hints(
    taken_ghosts: &mut Vec<HintGhost>, always_hints: &mut Vec<LocationHint>, path_hints: &mut Vec<PathHint>,
    sometimes_hints: &mut Vec<LocationHint>, num_total_hints: usize, rng: &mut StdRng,
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

    let mut ghosts = HintGhost::iter().collect::<Vec<_>>();
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
    SeedInfo { settings, world_graph, .. }: &SeedInfo, check_map: &mut CheckMap,
) -> Option<BowOfLightHint> {
    if settings.progressive_bow_of_light {
        return None;
    }

    for location_node in world_graph.values() {
        for &check in location_node.clone().get_checks().iter().flatten().collect::<Vec<&Check>>() {
            if let FillerItem::Item(item) = check_map.get(check.get_name()).unwrap().unwrap() {
                if BowOfLight.eq(&item) {
                    return Some(BowOfLightHint { check });
                }
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
    rng: &mut StdRng, seed_info: &SeedInfo, check_map: &mut CheckMap, taken_checks: &mut Vec<String>,
    taken_ghosts: &mut Vec<HintGhost>,
) -> Vec<LocationHint> {
    let mut always_checks = vec![
        "Master Sword Pedestal", "Great Rupee Fairy", "Blacksmith (Lorule)", "Bouldering Guy", "Irene", "Rosso I",
        "Rosso II", "Osfala", "Haunted Grove Stump", "Queen Oren", "Shady Guy", "Street Merchant (Right)",
    ];

    if !seed_info.settings.minigames_excluded {
        always_checks.extend(vec!["Octoball Derby", "Treacherous Tower Intermediate"]);
    }

    always_checks.retain(|check| !taken_checks.contains(&check.to_string()));

    let mut always_hints = Vec::new();
    for check_name in always_checks {
        let mut location_hint = generate_location_hint(check_name, seed_info, check_map);
        if location_hint.choose_ghost(rng, taken_ghosts).is_err() {
            continue;
        }
        always_hints.push(location_hint);
        taken_checks.push(check_name.to_string());
    }

    always_hints
}

fn generate_location_hint(check_name: &'static str, seed_info: &SeedInfo, check_map: &mut CheckMap) -> LocationHint {
    // fixme this sucks
    let mut check = None;
    'outer: for (_, loc_node) in seed_info.world_graph.clone() {
        let checks = loc_node.get_checks().clone();
        for c in checks.into_iter().flatten().collect::<Vec<Check>>() {
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

    let logical_ghosts = find_checks_before_goal(seed_info, check_map, item)
        .iter()
        .filter_map(|check| {
            if let Some(FillerItem::HintGhost(ghost)) = check.get_quest() {
                return Some(ghost);
            };
            None
        })
        .collect::<Vec<_>>();

    LocationHint { item: item.as_item().unwrap(), check: check.clone(), logical_ghosts, ghosts: vec![] }
}

/**
 * Sometimes Hints
 * Generates hints for checks that are only "sometimes" hinted, depending on settings. The checks
 * that get hinted are chosen randomly.
 */
fn generate_sometimes_hints(
    rng: &mut StdRng, seed_info: &SeedInfo, check_map: &mut CheckMap, num_sometimes_hints: usize,
    taken_checks: &[String], taken_ghosts: &mut Vec<HintGhost>,
) -> Vec<LocationHint> {
    let mut sometimes_checks = vec![
        "Bee Guy (2)", "Behind Ice Gimos", "Bird Lover", "Blacksmith", "Blacksmith Cave", "Cucco Mini-Dungeon",
        "Hookshot Mini-Dungeon", "Donkey Cave", "Eastern Ruins Peg Circle", "Merge Mini-Dungeon", "Fire Cave Pillar",
        "Floating Island", "Graveyard Ledge Cave", "Ice Gimos Fight", "Ice Rod Cave", "Ku's Domain Fight",
        "Pegasus Boots Pyramid", "Misery Mire Ledge", "Sand Mini-Dungeon", "Philosopher's Cave", "Queen Oren",
        "Spectacle Rock", "Flippers Mini-Dungeon", "Thief Girl", "Waterfall Cave", "Woman", "[LC] Zelda",
        "River Mini-Dungeon", "[DP] (2F) Under Rock (Ball Room)", "[DP] (2F) Under Rock (Left)",
        "[DP] (2F) Under Rock (Right)", "[EP] (1F) Escape Chest", "[HC] Battlement", "[HC] West Wing",
        "[HG] (3F) Fire Bubbles", "[HG] (2F) Fire Ring", "[IR] (B2) Long Merge Chest",
        "[IR] (B4) Southeast Chest (Fall)", "[LC] Tile Trial II", "[LC] Bomb Trial II", "[LC] Hook Trial II",
        "[LC] Lamp Trial", "[PD] (2F) Big Chest (Hidden)", "[PD] (B1) Bomb Bowling", "[SP] (B1) Big Chest (Secret)",
        "[SW] (B1) Big Chest (Eyes)", "[SW] (B1) South Chest", "[TT] (B2) Eyegores", "[TT] (B3) Big Chest (Hidden)",
        "[TH] (8F) Fairy Room", "[TR] (B1) Big Chest (Center)", "[TR] (1F) Defeat Flamolas",
    ];

    // Maiamai Madness
    if seed_info.settings.maiamai_madness {
        sometimes_checks.extend(vec![
            "[Mai] Blacksmith Tiles", "[Mai] Buried in the Desert", "[Mai] Buried near Desert Palace",
            "[Mai] Outside Cucco Mini-Dungeon", "[Mai] Dark Ruins South Wall", "[Mai] Fire Cave Ledge",
            "[Mai] Eastern Ruins Rock", "[Mai] Hyrule Castle Tiles", "[Mai] Hyrule Hotfoot Rock",
            "[Mai] Hyrule Rupee Rush Wall", "[Mai] Lake Hylia Island Tile", "[Mai] Kakariko Sand",
            "[Mai] Ku's Domain Water", "[Mai] Ice Cave Ledge", "[Mai] Lorule Mountain W Big Rock",
            "[Mai] Lorule Fortune-Teller Rock", "[Mai] Lorule Graveyard Tree", "[Mai] Lorule Lake Rock",
            "[Mai] Lorule Lake Skull", "[Mai] Lorule Rupee Rush Wall", "[Mai] Rosso's Ore Mine",
            "[Mai] Skull Woods Rock", "[Mai] Southern Ruins Big Rock", "[Mai] Southern Ruins Bomb Cave",
        ]);
    }

    // Minigames
    if !seed_info.settings.minigames_excluded {
        sometimes_checks
            .extend(vec!["Dodge the Cuccos", "Rupee Rush (Hyrule)", "Rupee Rush (Lorule)", "Hyrule Hotfoot 75s"]);
    }

    sometimes_checks.retain(|check| !taken_checks.contains(&check.to_string()));

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
        let mut location_hint = generate_location_hint(selected_hint, seed_info, check_map);
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
    rng: &mut StdRng, seed_info: &SeedInfo, check_map: &mut CheckMap, taken_checks: &mut Vec<String>,
    taken_ghosts: &mut Vec<HintGhost>,
) -> Vec<PathHint> {
    let mut bosses_and_prize_locations = vec![
        (Goal::Yuga, "[EP] Prize"),
        (Goal::Margomill, "[HG] Prize"),
        (Goal::Moldorm, "[TH] Prize"),
        (Goal::GemesaurKing, "[PD] Prize"),
        (Goal::Arrghus, "[SP] Prize"),
        (Goal::Knucklemaster, "[SW] Prize"),
        (Goal::Stalblind, "[TT] Prize"),
        (Goal::Grinexx, "[TR] Prize"),
        (Goal::Zaganaga, "[DP] Prize"),
        (Goal::Dharkstare, "[IR] Prize"),
    ];

    bosses_and_prize_locations = shuffle(rng, bosses_and_prize_locations);

    let mut chosen_paths: Vec<PathHint> = Vec::new();
    let mut backup_paths: Vec<PathHint> = Vec::new();
    let mut extra_paths_needed = 0;

    for (goal, prize_loc) in bosses_and_prize_locations {
        if is_sage(check_map.get(prize_loc).unwrap().unwrap()) {
            let mut potential_paths = get_potential_path_hints(rng, seed_info, check_map, taken_checks, goal);

            if let Some(chosen_path) = choose_path_hint(&mut potential_paths, taken_checks, taken_ghosts, rng) {
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
        let mut extra_paths_added = 0;
        loop {
            if extra_paths_added >= extra_paths_needed || backup_paths.is_empty() {
                break;
            }

            if let Some(backup_path) = choose_path_hint(&mut backup_paths, taken_checks, taken_ghosts, rng) {
                chosen_paths.push(backup_path);
                extra_paths_added += 1;
            }
        }
    }

    chosen_paths
}

fn choose_path_hint(
    potential_paths: &mut Vec<PathHint>, taken_checks: &mut Vec<String>, taken_ghosts: &mut Vec<HintGhost>,
    rng: &mut StdRng,
) -> Option<PathHint> {
    potential_paths.retain(|path| !taken_checks.contains(&path.check.get_name().to_string()));

    for chosen_path in potential_paths {
        // Choose a random Ghost for this hint of the ones not already taken
        match chosen_path.logical_ghosts.iter().filter(|&ghost| !taken_ghosts.contains(ghost)).choose_stable(rng) {
            None => {
                info!("No available Ghosts to give Hint: {:?}", chosen_path);
            },
            Some(chosen_ghost) => {
                chosen_path.ghosts.push(*chosen_ghost);
                taken_checks.push(chosen_path.check.get_name().to_string());
                taken_ghosts.push(*chosen_ghost);
                return Some(chosen_path.clone());
            },
        }
    }

    None
}

/// Finds all checks available before a given Quest Goal using a modified Sphere Search.
fn find_checks_before_goal(
    seed_info: &SeedInfo, check_map: &mut CheckMap, goal: impl Into<FillerItem>,
) -> DashSet<Check> {
    let goal = goal.into();
    let mut progress = Progress::new(&seed_info.settings);
    let mut reachable_checks: Vec<Check>;
    let mut potential_path_checks: DashSet<Check> = Default::default();

    // Find candidate Path Checks with a modified sphere search
    loop {
        reachable_checks = find_reachable_checks(seed_info, &progress);
        potential_path_checks.extend(&reachable_checks);
        let reachable_items = get_items_from_reachable_checks(seed_info, &reachable_checks, check_map);

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
    rng: &mut StdRng, seed_info: &SeedInfo, check_map: &mut CheckMap, taken_checks: &mut Vec<String>, goal: Goal,
) -> Vec<PathHint> {
    let mut reachable_checks: Vec<Check>;
    let mut potential_paths: Vec<PathHint> = Vec::new();

    let mut potential_path_checks = find_checks_before_goal(seed_info, check_map, goal);

    // Limit potential paths to locations with valid Path Items that haven't yet been taken
    potential_path_checks.retain(|check| {
        if let Some(Some(FillerItem::Item(item))) = check_map.get(check.get_name()) {
            !taken_checks.contains(&check.get_name().to_string()) && POSSIBLE_PATH_ITEMS.contains(&item)
        } else {
            false
        }
    });

    // Test candidate items to see if Boss can be defeated without them
    for check in potential_path_checks {
        // Reset Progression
        let mut progress = Progress::new(&seed_info.settings);

        loop {
            reachable_checks = find_reachable_checks(seed_info, &progress);

            // Remove Potential Path Location
            reachable_checks.retain(|c| check.ne(c));

            let reachable_items = get_items_from_reachable_checks(seed_info, &reachable_checks, check_map);

            let new_items = reachable_items.difference(&progress);

            if new_items.is_empty() {
                // Item could be Path if goal couldn't be reached without it
                if !progress.has(goal) {
                    let hint_locations = reachable_items
                        .get_items()
                        .iter()
                        .filter_map(|&item| if let FillerItem::HintGhost(ghost) = item { Some(ghost) } else { None })
                        .collect::<_>();

                    let path_item = check_map
                        .get(check.get_name())
                        .expect("Path check should be in Check Map")
                        .expect("Path check should have Path Item");

                    potential_paths.push(PathHint {
                        goal,
                        check,
                        ghosts: vec![],
                        logical_ghosts: hint_locations,
                        path_item,
                    });
                }
                break;
            }

            for new_item in &new_items {
                progress.add_item(*new_item);
            }
        }
    }

    shuffle(rng, potential_paths)
}

struct SerializeGhosts<'a>(&'a [HintGhost]);

impl Serialize for SerializeGhosts<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for ghost in self.0.iter() {
            seq.serialize_element(hint_ghost_name(ghost))?;
        }
        seq.end()
    }
}

pub(crate) fn hint_ghost_name(ghost: &HintGhost) -> &'static str {
    match ghost {
        HintGhost::LostWoodsMaze1 => "Lost Woods Maze Ghost 1",
        HintGhost::LostWoodsMaze2 => "Lost Woods Maze Ghost 2",
        HintGhost::LostWoodsMaze3 => "Lost Woods Maze Ghost 3",
        HintGhost::LostWoods => "Lost Woods Ghost",
        HintGhost::SpectacleRock => "Spectacle Rock Ghost",
        HintGhost::TowerOfHeraOutside => "Outside Tower of Hera Ghost",
        HintGhost::FloatingIsland => "Floating Island Ghost",
        HintGhost::FireCave => "Fire Cave Ghost",
        HintGhost::MoldormCave => "Moldorm Cave Ghost",
        HintGhost::ZorasDomain => "Zora's Domain Ghost",
        HintGhost::FortuneTellerHyrule => "Hyrule Fortune-Teller Ghost",
        HintGhost::Sanctuary => "Sanctuary Ghost",
        HintGhost::GraveyardHyrule => "Hyrule Graveyard Ghost",
        HintGhost::WaterfallCave => "Waterfall Cave Ghost",
        HintGhost::Well => "Kakariko Well Ghost",
        HintGhost::ShadyGuy => "Shady Guy Ghost",
        HintGhost::StylishWoman => "Stylish Woman Ghost",
        HintGhost::BlacksmithCave => "Blacksmith Cave Ghost",
        HintGhost::EasternRuinsPegs => "Eastern Ruins Pegs Ghost",
        HintGhost::EasternRuinsCave => "Eastern Ruins Cave Ghost",
        HintGhost::EasternRuinsEntrance => "Eastern Ruins Entrance Ghost",
        HintGhost::RupeeRushHyrule => "Hyrule Rupee Rush Ghost",
        HintGhost::Cuccos => "Dodge the Cuccos Ghost",
        HintGhost::SouthBridge => "Southern Bridge Ghost",
        HintGhost::SouthernRuins => "Southern Ruins Ghost",
        HintGhost::HouseOfGalesIsland => "House of Gales Island Ghost",
        HintGhost::HyruleHotfoot => "Hyrule Hotfoot Ghost",
        HintGhost::Letter => "Letter in a Bottle Ghost",
        HintGhost::StreetPassTree => "StreetPass Tree Ghost",
        HintGhost::BlacksmithBehind => "Behind Blacksmith Ghost",
        HintGhost::GraveyardLedge => "Graveyard Ledge Ghost",
        HintGhost::DesertEast => "Desert East Ghost",
        HintGhost::DesertCenter => "Desert Center Ghost",
        HintGhost::DesertSouthWest => "Desert South West Ghost",
        HintGhost::HyruleCastleRocks => "Hyrule Castle Rocks Ghost",
        HintGhost::WitchsHouse => "Witch's House Ghost",

        HintGhost::SkullWoodsCuccos => "Skull Woods Cuccos Ghost",
        HintGhost::TreacherousTower => "Treacherous Tower Ghost",
        HintGhost::IceRuinsOutside => "Ice Ruins Outside Ghost",
        HintGhost::LoruleGraveyard => "Lorule Graveyard Ghost",
        HintGhost::DarkRuinsNorth => "Dark Ruins North Ghost",
        HintGhost::SkullWoodsSouth => "Skull Woods South Ghost",
        HintGhost::FortunesChoice => "Fortune's Choice Ghost",
        HintGhost::VeteranThief => "Veteran Thief Ghost",
        HintGhost::FortuneTellerLorule => "Lorule Fortune-Teller Ghost",
        HintGhost::DarkMaze => "Dark Maze Ghost",
        HintGhost::RupeeRushLorule => "Lorule Rupee Rush Ghost",
        HintGhost::GreatRupeeFairy => "Great Rupee Fairy Ghost",
        HintGhost::OctoballDerby => "Octoball Derby Ghost",
        HintGhost::VacantHouse => "Vacant House Ghost",
        HintGhost::MiseryMireLedge => "Misery Mire Ledge Ghost",
        HintGhost::SwampPalaceOutsideLeft => "Swamp Palace Outside Left Ghost",
        HintGhost::TurtleBullied => "Turtle Bullied Ghost",
        HintGhost::TurtleWall => "Turtle Wall Ghost",
        HintGhost::TurtleRockOutside => "Turtle Rock Outside Ghost",
        HintGhost::DarkPalaceOutside => "Dark Palace Outside Ghost",
        HintGhost::SwampPalaceOutsideRight => "Swamp Palace Outside Right Ghost",
        HintGhost::MiseryMireBridge => "Misery Mire Bridge Ghost",
    }
}

const POSSIBLE_PATH_ITEMS: [Item; 49] = [
    Bow01, Bow02, Bow03, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Bombs01, Bombs02, FireRod01, FireRod02,
    IceRod01, IceRod02, Hammer01, Hammer02, SandRod01, SandRod02, TornadoRod01, TornadoRod02, Bell, StaminaScroll,
    PegasusBoots, Flippers, HylianShield, SmoothGem, LetterInABottle, PremiumMilk, HintGlasses, GreatSpin, Bottle01,
    Bottle02, Bottle03, Bottle04, Lamp01, Lamp02, Sword01, Sword02, Sword03, Sword04, Glove01, Glove02, Net01, Net02,
    Mail01, Mail02, OreYellow, OreGreen, OreBlue, OreRed,
];

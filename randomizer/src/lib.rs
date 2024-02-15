use crate::filler::filler_item::Vane;
use crate::filler::trials::TrialsConfig;
use crate::filler::{portals, text, treacherous_tower, trials, vanes};
use crate::world::WorldGraph;
use crate::{
    constants::VERSION,
    hints::{formatting::*, Hints},
    metrics::Metrics,
    patch::lms::msbf::MsbfKey,
    system::UserConfig,
};
use filler::filler_item::Randomizable;
use filler::portals::Portal;
use game::tower_stage::TowerStage;
use game::Item::{self};
use log::{debug, error, info};
use macros::fail;
use modinfo::Settings;
use patch::Patcher;
use path_absolutize::*;
use rand::{rngs::StdRng, SeedableRng};
use regions::Subregion;
use rom::Rom;
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::{
    error::Error as StdError,
    fs::File,
    hash::{Hash, Hasher},
    io::{self, Write},
    ops::Deref,
};
use twox_hash::XxHash64;

pub mod constants;
pub mod filler;
mod hints;
mod metrics;
mod patch;
pub mod regions;
pub mod system;
mod world;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    fn internal<S>(err: S) -> Self
    where
        S: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { kind: ErrorKind::Internal, inner: err.into() }
    }

    fn game<S>(err: S) -> Self
    where
        S: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { kind: ErrorKind::Game, inner: err.into() }
    }

    fn io<S>(err: S) -> Self
    where
        S: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { kind: ErrorKind::Io, inner: err.into() }
    }

    /// Gets the type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Converts this error into its inner value.
    pub fn into_inner(self) -> Box<dyn StdError + Send + Sync + 'static> {
        self.inner
    }
}

impl From<rom::Error> for Error {
    fn from(err: rom::Error) -> Self {
        let kind = match err.kind() {
            rom::ErrorKind::Io => ErrorKind::Io,
            rom::ErrorKind::Rom => ErrorKind::Game,
        };
        Self { kind, inner: err.into_inner() }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self { kind: ErrorKind::Io, inner: err.into() }
    }
}

impl From<system::Error> for Error {
    fn from(err: system::Error) -> Self {
        Self { kind: ErrorKind::Internal, inner: err.into() }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Internal,
    Game,
    Io,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LocationInfo {
    subregion: &'static Subregion,
    name: &'static str,
}

impl LocationInfo {
    pub const fn new(name: &'static str, subregion: &'static Subregion) -> Self {
        Self { subregion, name }
    }

    pub fn world(&self) -> regions::World {
        self.subregion.world()
    }

    pub fn region(&self) -> &'static str {
        self.subregion.name()
    }

    pub fn region_colorized(&self) -> String {
        self.subregion.name_colorized()
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// A world layout for the patcher.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Layout {
    #[serde(rename = "Hyrule", serialize_with = "serialize_category")]
    hyrule: Category,
    #[serde(rename = "Lorule", serialize_with = "serialize_category")]
    lorule: Category,
    #[serde(rename = "Dungeons", serialize_with = "serialize_category")]
    dungeons: Category,
}

impl Layout {
    fn category(&self, id: regions::World) -> &Category {
        match id {
            regions::World::Hyrule => &self.hyrule,
            regions::World::Lorule => &self.lorule,
            regions::World::Dungeons => &self.dungeons,
        }
    }

    fn category_mut(&mut self, id: regions::World) -> &mut Category {
        match id {
            regions::World::Hyrule => &mut self.hyrule,
            regions::World::Lorule => &mut self.lorule,
            regions::World::Dungeons => &mut self.dungeons,
        }
    }

    fn get_node_mut(&mut self, node: &'static Subregion) -> &mut DashMap<&'static str, Randomizable> {
        self.category_mut(node.world()).entry(node.name()).or_insert_with(Default::default)
    }

    fn get(&self, location: &LocationInfo) -> Option<Randomizable> {
        let LocationInfo { subregion: node, name } = location;
        self.category(node.world()).get(node.name()).and_then(|region| region.get(name).copied())
    }

    #[allow(unused)]
    fn get_item(&self, name: &'static str, subregion: &'static Subregion) -> Randomizable {
        self.get(&LocationInfo::new(name, subregion)).unwrap()
    }

    #[allow(unused)]
    fn find(&self, item: Item) -> Vec<&'static str> {
        todo!()
    }

    /// This just highlights why we need to redo [`Layout`]
    #[allow(unused)]
    fn find_single(&self, find_item: Randomizable) -> Option<(&'static str, &'static str)> {
        for (region_name, region) in &self.hyrule {
            for (loc_name, item) in region {
                if find_item.eq(item) {
                    return Some((region_name, loc_name));
                }
            }
        }

        for (region_name, region) in &self.lorule {
            for (loc_name, item) in region {
                if find_item.eq(item) {
                    return Some((region_name, loc_name));
                }
            }
        }

        for (region_name, region) in &self.dungeons {
            for (loc_name, item) in region {
                if find_item.eq(item) {
                    return Some((region_name, loc_name));
                }
            }
        }

        None
    }

    pub fn set(&mut self, location: LocationInfo, item: Randomizable) {
        let LocationInfo { subregion: node, name } = location;
        self.get_node_mut(node).insert(name, item);
        debug!("Placed {} in {}/{}", item.as_str(), location.subregion.name(), location.name);
    }

    pub fn set_item<T>(&mut self, location: &'static str, subregion: &'static Subregion, item: T)
    where
        T: Into<Randomizable>,
    {
        self.set(LocationInfo::new(location, subregion), item.into());
    }
}

pub type Category = DashMap<&'static str, DashMap<&'static str, Randomizable>>;

fn serialize_category<S>(region: &Category, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    struct Wrap<'a>(&'a DashMap<&'static str, Randomizable>);

    impl<'a> Serialize for Wrap<'a> {
        fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let ordered = self.0.iter().map(|(&k, &v)| (k, v)).collect::<BTreeMap<_, _>>();
            let mut map = ser.serialize_map(Some(ordered.len()))?;
            for (k, v) in ordered {
                map.serialize_entry(k, v.as_str())?;
            }
            map.end()
        }
    }

    let ordered = region.iter().map(|(&k, v)| (k, v)).collect::<BTreeMap<_, _>>();
    let mut map = ser.serialize_map(Some(ordered.len()))?;
    for (k, v) in ordered {
        map.serialize_entry(k, &Wrap(v))?;
    }
    map.end()
}

/// Align JSON Key-Values for readability
/// Can't find a decent library for this, so we're doing it manually
fn align_json_values(json: &mut String) {
    const KEY_ALIGNMENT: usize = 56;
    let mut index_colon = 0;
    while index_colon < json.len() {
        let index_colon_opt = json[index_colon..].find(':');
        if index_colon_opt.is_none() {
            break;
        }
        index_colon += index_colon_opt.unwrap();
        if ['{', '['].contains(&json[index_colon..].chars().nth(2).unwrap()) {
            index_colon += 1;
            continue;
        }

        let index_prev_new_line = json[..index_colon].rfind('\n').unwrap_or_else(|| {
            fail!("Couldn't fine new line character before index: {}", index_colon);
        });
        let line_length_up_to_value = &index_colon - index_prev_new_line;

        if KEY_ALIGNMENT < line_length_up_to_value {
            error!("Failed to write Spoiler Log");
            error!(
                "JSON Key Alignment value smaller than line length up to that point: {} < {}",
                KEY_ALIGNMENT, line_length_up_to_value
            );
            fail!("Problem line: {}", &json[index_prev_new_line..index_colon]);
        }

        let spaces_to_add = KEY_ALIGNMENT - line_length_up_to_value;

        json.insert_str(&index_colon + 1, (0..spaces_to_add).map(|_| " ").collect::<String>().as_str());
        index_colon += 1;
    }
}

#[derive(Serialize, Default, Debug)]
pub struct Text {
    credits: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeedInfo {
    #[serde(default)]
    pub seed: u32,

    pub version: String,

    #[serde(skip_deserializing)]
    pub hash: SeedHash,

    pub settings: Settings,

    /// The list of exclusions provided by the user in [`settings`], enhanced by the randomizer based on settings.
    #[serde(skip_deserializing)]
    pub full_exclusions: BTreeSet<String>,

    #[serde(skip_deserializing)]
    pub treacherous_tower_floors: Vec<TowerStage>,

    #[serde(skip_deserializing)]
    pub trials_config: TrialsConfig,

    #[serde(skip_deserializing)]
    pub layout: Layout,

    #[serde(skip_deserializing)]
    pub portal_map: PortalMap,

    #[serde(skip_deserializing, rename = "weather_vane_map")]
    pub vane_map: VaneMap,

    #[serde(skip_deserializing)]
    pub text: Text,

    #[serde(skip_deserializing)]
    pub metrics: Metrics,

    #[serde(skip_deserializing)]
    pub hints: Hints,

    #[serde(skip_deserializing, skip_serializing)]
    pub world_graph: WorldGraph,
}

impl SeedInfo {
    fn new(
        seed: u32, hash: SeedHash, settings: Settings, portal_map: PortalMap, vane_map: VaneMap,
        trials_config: TrialsConfig, world_graph: WorldGraph, text: Text, treacherous_tower_floors: Vec<TowerStage>,
    ) -> Self {
        Self {
            seed,
            version: VERSION.to_owned(),
            hash,
            settings,
            full_exclusions: Default::default(),
            vane_map,
            portal_map,
            layout: Default::default(),
            metrics: Default::default(),
            hints: Default::default(),
            trials_config,
            world_graph,
            text,
            treacherous_tower_floors,
        }
    }

    pub fn is_excluded(&self, check_name: &str) -> bool {
        self.full_exclusions.contains(check_name)
    }
}

impl Default for SeedInfo {
    fn default() -> Self {
        Self {
            seed: 0,
            version: "".to_owned(),
            hash: Default::default(),
            settings: Default::default(),
            full_exclusions: Default::default(),
            portal_map: Default::default(),
            vane_map: Default::default(),
            layout: Default::default(),
            metrics: Default::default(),
            hints: Default::default(),
            trials_config: Default::default(),
            world_graph: Default::default(),
            treacherous_tower_floors: Default::default(),
            text: Default::default(),
        }
    }
}

/// Main entry point to generate one ALBWR Seed.
pub fn generate_seed(
    seed: u32, settings: Settings, user_config: &UserConfig, no_patch: bool, no_spoiler: bool,
) -> Result<()> {
    validate_settings(&settings)?;

    let rng = &mut StdRng::seed_from_u64(seed.clone() as u64);

    let hash = SeedHash::new(seed.clone(), &settings);

    info!("Hash:                           {}", hash.text_hash);

    // settings.log_settings();

    let seed_info = &calculate_seed_info(seed, settings, hash, rng)?;
    patch_seed(seed_info, user_config, no_patch, no_spoiler)?;

    Ok(())
}

/// A hash used in-game to quickly verify that two players are playing the same seed.
///
/// The hash is calculated as `u64`, truncated to `u16` (5 digits), then converted to a Symbolic form that can be
/// displayed in-game as well as in the spoiler log.
#[derive(Default, Debug)]
pub struct SeedHash {
    item_hash: String,
    text_hash: String,
}

impl SeedHash {
    pub fn new(seed: u32, settings: &Settings) -> Self {
        // Calculate underlying Hash
        let mut hasher = XxHash64::default();
        (seed, settings, VERSION).hash(&mut hasher);
        let mut hash = hasher.finish() % 100_000;

        // Convert to Item Hash
        let hash_item_lut: Vec<(&String, &str)> = vec![
            (A_BUTTON.deref(), "(A)"),
            (B_BUTTON.deref(), "(B)"),
            (X_BUTTON.deref(), "(X)"),
            (Y_BUTTON.deref(), "(Y)"),
            (L_BUTTON.deref(), "(L)"),
            (R_BUTTON.deref(), "(R)"),
            (RAVIO.deref(), "(Ravio)"),
            (SYMBOL_BOW.deref(), "(Bow)"),
            (SYMBOL_BOMBS.deref(), "(Bomb)"),
            (SYMBOL_FIRE_ROD.deref(), "(Fire)"),
        ];

        const HASH_LEN: usize = 5;
        let mut digit = Vec::with_capacity(HASH_LEN);
        for _ in 0..HASH_LEN {
            digit.push(hash_item_lut.get((&hash % 10) as usize).unwrap());
            hash /= 10;
        }

        let item_hash = format!("{} {} {} {} {}", digit[4].0, digit[3].0, digit[2].0, digit[1].0, digit[0].0);
        let text_hash = format!("{} {} {} {} {}", digit[4].1, digit[3].1, digit[2].1, digit[1].1, digit[0].1);

        Self { item_hash, text_hash }
    }
}

impl Serialize for SeedHash {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.text_hash.as_str())
    }
}

/// Validates the Settings to make sure the user hasn't made incompatible selections
fn validate_settings(settings: &Settings) -> Result<()> {
    // LC Requirement
    if !(0..=7).contains(&settings.lc_requirement) {
        fail!("Invalid Lorule Castle Requirement: \"{}\" was not between 0-7, inclusive.", settings.lc_requirement);
    }

    // Yuganon Requirement
    // if !(0..=7).contains(&settings.logic.yuganon_requirement) {
    //     fail!("Invalid Yuga Ganon Requirement: \"{}\" was not between 0-7, inclusive.", settings.logic.yuganon_requirement);
    // }

    if settings.yuganon_requirement != settings.lc_requirement {
        fail!(
            "Yuga Ganon Requirement: \"{}\" is different than Lorule Castle Requirement: \"{}\"\n\
        Different values for these settings are not yet supported!",
            settings.yuganon_requirement,
            settings.lc_requirement
        );
    }

    // Progressive Bow of Light
    if settings.progressive_bow_of_light && settings.bow_of_light_in_castle {
        fail!("The progressive_bow_of_light and bow_of_light_in_castle settings cannot both be enabled.");
    }

    // Swords
    if settings.sword_in_shop && settings.swordless_mode {
        fail!("The sword_in_shop and swordless_mode settings cannot both be enabled.");
    }

    // Assured Weapons
    if settings.assured_weapon && (settings.sword_in_shop || settings.boots_in_shop) {
        fail!(
            "The assured_weapon setting cannot be enabled when either sword_in_shop or boots_in_shop is also enabled."
        );
    }

    Ok(())
}

/// "Deterministic `HashMap`" that uses a hashing algorithm not based on any random number generation, unlike the Rust
/// default which is non-deterministic for security reasons not relevant for our purposes.
pub type DashMap<K, V> = HashMap<K, V, BuildHasherDefault<XxHash64>>;

/// "Deterministic `HashSet`" that uses a hashing algorithm not based on any random number generation, unlike the Rust
/// default which is non-deterministic for security reasons not relevant for our purposes.
pub type DashSet<T> = HashSet<T, BuildHasherDefault<XxHash64>>;

/// Map of all checks (as Strings) to their held item
pub type CheckMap = DashMap<String, Option<Randomizable>>;

/// Map of all Portals to their destination Portals. Map is not bidirectional to allow for (eventual) decoupled shuffle,
/// so each Portal and its destination must have a corresponding reversed entry.
pub type PortalMap = BTreeMap<Portal, Portal>;

/// Map of all Weather Vanes to the destination Vanes they unlock.
pub type VaneMap = BTreeMap<Vane, Vane>;

fn calculate_seed_info(seed: u32, settings: Settings, hash: SeedHash, rng: &mut StdRng) -> Result<SeedInfo> {
    println!();
    info!("Calculating Seed Info...");

    let portal_map = portals::build_portal_map(&settings, rng)?;
    let vane_map = vanes::build_vanes_map(&settings, rng)?;
    let text = text::generate(rng)?;
    let trials_config = trials::configure(rng, &settings)?;
    let treacherous_tower_floors = treacherous_tower::choose_floors(&settings, rng)?;
    let world_graph = world::build_world_graph(&portal_map);

    let mut seed_info = SeedInfo::new(
        seed, hash, settings, portal_map, vane_map, trials_config, world_graph, text, treacherous_tower_floors,
    );

    // Check Map and Item Pools
    let check_map = &mut filler::prefill_check_map(&mut seed_info.world_graph);

    // Filler Algorithm
    filler::fill_all_locations_reachable(rng, &mut seed_info, check_map)?;

    // Post-analysis: Metrics and Hints
    metrics::calculate_metrics(&mut seed_info, check_map)?;
    hints::generate_hints(rng, &mut seed_info, check_map)?;

    Ok(seed_info)
}

pub fn patch_seed(seed_info: &SeedInfo, user_config: &UserConfig, no_patch: bool, no_spoiler: bool) -> Result<()> {
    println!();

    if !no_patch {
        info!("Starting Patch Process...");

        let game = Rom::load(user_config.rom())?;
        let mut patcher = Patcher::new(game)?;

        info!("ROM Loaded.\n");

        // patch::lms::msbf::research(&mut patcher, None, "HintGhost", vec![], true)?;

        // patch::research_msbf_msbt(&mut patcher,
        //     game::Course::IndoorLight, "FieldLight_22_BlackSmith", // MSBF
        //     game::Course::IndoorLight, "FieldLight_22",        // MSBT
        //     true);

        regions::patch(&mut patcher, seed_info)?;
        let patches = patcher.prepare(seed_info)?;
        patches.dump(user_config.output())?;
    }
    if !no_spoiler {
        let path = user_config.output().join(format!("{:0>10}_spoiler.json", seed_info.seed));
        info!("Writing Spoiler Log to:         {}", &path.absolutize()?.display());

        //let spoiler = Spoiler::from(seed_info);

        let mut serialized = serde_json::to_string_pretty(&seed_info).unwrap();
        align_json_values(&mut serialized);

        write!(File::create(path)?, "{}", serialized).expect("Could not write the spoiler log.");
    }
    Ok(())
}

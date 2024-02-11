use crate::filler::check::Check;
use crate::filler::filler_item::Item;
use crate::filler::filler_item::Randomizable::{Portal, Vane};
use crate::filler::item_pools::{get_maiamai_pool, Pool};
use crate::filler::location::Location;
use crate::filler::progress::Progress;
use crate::{world::WorldGraph, CheckMap, DashMap, SeedInfo};
use log::{error, info, warn};
use macros::fail;
use modinfo::settings::logic::LogicMode;
use path::Path;
use queue::Queue;
use rand::{rngs::StdRng, Rng};
use rom::Error;
use std::collections::HashSet;

pub mod check;
pub mod filler_item;
pub mod item_pools;
mod loading_zone_pair;
pub mod location;
pub mod location_node;
pub mod logic;
pub mod path;
pub mod portals;
pub mod progress;
pub(crate) mod treacherous_tower;
pub(crate) mod trials;
pub mod util;
pub mod vanes;

/// Fill Seed such that All Locations are Reachable
///
/// This is the "standard" filler algorithm for ALBWR.
pub fn fill_all_locations_reachable(
    rng: &mut StdRng, seed_info: &mut SeedInfo, check_map: &mut CheckMap,
) -> crate::Result<()> {
    let (mut progression_pool, mut junk_pool) = item_pools::get_item_pools(rng, seed_info);

    place_portals(seed_info, check_map);
    place_weather_vanes(seed_info, check_map);

    verify_all_locations_accessible(seed_info, check_map, &mut progression_pool)?;

    preplace_items(rng, seed_info, check_map, &mut progression_pool);
    handle_exclusions(rng, seed_info, check_map, &mut junk_pool);

    assumed_fill(rng, seed_info, check_map, &mut progression_pool)?;
    fill_junk(rng, check_map, &mut junk_pool);

    build_layout(seed_info, check_map)?;

    Ok(())
}

/// Portal randomization
fn place_portals(SeedInfo { portal_map, .. }: &SeedInfo, check_map: &mut CheckMap) {
    use crate::filler::portals::Portal::*;
    let portal_lut = vec![
        ("[HC] Portal", HyruleCastle),
        ("Stylish Woman Portal", StylishWoman),
        ("Your House Portal", YourHouse),
        ("Hyrule Right Paradox Portal", ParadoxRightHyrule),
        ("Hyrule Left Paradox Portal", ParadoxLeftHyrule),
        ("Hyrule Waterfall Portal", WaterfallHyrule),
        ("Eastern Ruins Pillar Portal", EasternRuinsPillar),
        ("Eastern Ruins SE Portal", EasternRuinsSE),
        ("Lost Woods Pillar Portal", LostWoodsPillar),
        ("Sahasrahla's House Portal", SahasrahlasHouse),
        ("Rosso's House Portal", RossosHouse),
        ("Misery Mire Entrance Portal", MiseryMireEntrance),
        ("Desert Right Pillar Portal", DesertPillarRight),
        ("Desert Left Pillar Portal", DesertPillarLeft),
        ("Desert Middle Portal", DesertMiddle),
        ("Desert SW Portal", DesertSW),
        ("Desert Palace Portal", DesertPalace),
        ("Desert North Portal", DesertNorth),
        ("Hyrule Death West Portal", DeathWestHyrule),
        ("Hyrule Floating Island Portal", FloatingIslandHyrule),
        ("Hyrule River Portal", RiverHyrule),
        ("Lake Hylia Portal", LakeHylia),
        ("Hyrule Hotfoot Portal", HyruleHotfoot),
        ("Sanctuary Portal", Sanctuary),
        ("Hyrule Graveyard Ledge Portal", GraveyardLedgeHyrule),
        ("Hyrule Rosso's Ore Mine Portal", RossosOreMineHyrule),
        ("Hyrule Swamp Pillar Portal", SwampPillarHyrule),
        ("Zora's Domain Portal", ZorasDomain),
        // --- //
        ("[LC] Portal", LoruleCastle),
        ("Thieves' Town Portal", ThievesTown),
        ("Vacant House Portal", VacantHouse),
        ("Lorule Right Paradox Portal", ParadoxRightLorule),
        ("Lorule Left Paradox Portal", ParadoxLeftLorule),
        ("Lorule Waterfall Portal", WaterfallLorule),
        ("Dark Ruins Pillar Portal", DarkRuinsPillar),
        ("Dark Ruins SE Portal", DarkRuinsSE),
        ("Skull Woods Pillar Portal", SkullWoodsPillar),
        ("n-Shaped House Portal", NShapedHouse),
        ("Destroyed House Portal", DestroyedHouse),
        ("Misery Mire Exit Portal", MiseryMireExit),
        ("Mire Right Pillar Portal", MirePillarRight),
        ("Mire Left Pillar Portal", MirePillarLeft),
        ("Mire Middle Portal", MireMiddle),
        ("Mire SW Portal", MireSW),
        ("Zaganaga Portal", Zaganaga),
        ("Mire North Portal", MireNorth),
        ("Lorule Death West Portal", DeathWestLorule),
        ("Lorule Floating Island Portal", FloatingIslandLorule),
        ("Lorule River Portal", RiverLorule),
        ("Lorule Lake Portal", LoruleLake),
        ("Lorule Hotfoot Portal", LoruleHotfoot),
        ("Philosopher's Cave Portal", Philosopher),
        ("Lorule Graveyard Ledge Portal", GraveyardLedgeLorule),
        ("Lorule Rosso's Ore Mine Portal", RossosOreMineLorule),
        ("Lorule Swamp Pillar Portal", SwampPillarLorule),
        ("Ku's Domain Portal", KusDomain),
    ];

    for (check_name, portal) in portal_lut {
        check_map.insert(check_name.to_owned(), Some(Portal(*portal_map.get(&portal).unwrap())));
    }
}

/// Weather Vane randomization
fn place_weather_vanes(SeedInfo { vane_map, .. }: &SeedInfo, check_map: &mut CheckMap) {
    use crate::filler::filler_item::Vane::*;
    let vane_lut = vec![
        ("Your House Weather Vane", YourHouseWV),
        ("Kakariko Village Weather Vane", KakarikoVillageWV),
        ("Eastern Palace Weather Vane", EasternPalaceWV),
        ("House of Gales Weather Vane", HouseOfGalesWV),
        ("Tower of Hera Weather Vane", TowerOfHeraWV),
        ("Witch's House Weather Vane", WitchsHouseWV),
        ("Death Mountain (Hyrule) Weather Vane", DeathMountainHyruleWV),
        ("Desert Palace Weather Vane", DesertPalaceWV),
        ("Sanctuary Weather Vane", SanctuaryWV),
        ("Skull Woods Weather Vane", SkullWoodsWV),
        ("Treacherous Tower Weather Vane", TreacherousTowerWV),
        ("Ice Ruins Weather Vane", IceRuinsWV),
        ("Lorule Castle Weather Vane", LoruleCastleWV),
        ("Graveyard Weather Vane", GraveyardWV),
        ("Thieves' Town Weather Vane", ThievesTownWV),
        ("Dark Palace Weather Vane", DarkPalaceWV),
        ("Blacksmith Weather Vane", BlacksmithWV),
        ("Vacant House Weather Vane", VacantHouseWV),
        ("Misery Mire Weather Vane", MiseryMireWV),
        ("Swamp Palace Weather Vane", SwampPalaceWV),
        ("Turtle Rock Weather Vane", TurtleRockWV),
        ("Death Mountain (Lorule) Weather Vane", DeathMountainLoruleWV),
    ];

    for (check_name, vane) in vane_lut {
        check_map.insert(check_name.to_owned(), Some(Vane(*vane_map.get(&vane).unwrap())));
    }
}

/// Place static items ahead of the randomly filled ones
fn preplace_items(
    rng: &mut StdRng, SeedInfo { settings, .. }: &SeedInfo, check_map: &mut CheckMap, progression: &mut Vec<Item>,
) {
    // Vanilla Dungeon Prizes
    if !&settings.dungeon_prize_shuffle {
        place_static(check_map, progression, Item::PendantOfCourage, "[EP] Prize");
        place_static(check_map, progression, Item::PendantOfWisdom, "[HG] Prize");
        place_static(check_map, progression, Item::PendantOfPower, "[TH] Prize");
        place_static(check_map, progression, Item::SageGulley, "[PD] Prize");
        place_static(check_map, progression, Item::SageOren, "[SP] Prize");
        place_static(check_map, progression, Item::SageSeres, "[SW] Prize");
        place_static(check_map, progression, Item::SageOsfala, "[TT] Prize");
        place_static(check_map, progression, Item::SageImpa, "[TR] Prize");
        place_static(check_map, progression, Item::SageIrene, "[DP] Prize");
        place_static(check_map, progression, Item::SageRosso, "[IR] Prize");
    }

    // Place un-randomized items
    place_static(check_map, progression, Item::RupeeSilver41, "[TR] (1F) Under Center");
    place_static(check_map, progression, Item::RupeeGold09, "[TR] (B1) Under Center");
    place_static(check_map, progression, Item::RupeeGold10, "[PD] (2F) South Hidden Room");
    place_static(check_map, progression, Item::HeartPiece28, "Fortune's Choice");

    // Kakariko Item Shop
    place_static(check_map, progression, Item::ScootFruit01, "Kakariko Item Shop (1)");
    place_static(check_map, progression, Item::FoulFruit01, "Kakariko Item Shop (2)");
    place_static(check_map, progression, Item::Shield01, "Kakariko Item Shop (3)");

    // Lakeside Item Shop
    place_static(check_map, progression, Item::ScootFruit02, "Lakeside Item Shop (1)");
    place_static(check_map, progression, Item::FoulFruit02, "Lakeside Item Shop (2)");
    place_static(check_map, progression, Item::Shield02, "Lakeside Item Shop (3)");

    // Mysterious Man
    place_static(check_map, progression, Item::GoldBee01, "Mysterious Man");

    // Thieves' Town Item Shop
    place_static(check_map, progression, Item::Bee01, "Thieves' Town Item Shop (1)");
    place_static(check_map, progression, Item::GoldBee02, "Thieves' Town Item Shop (2)");
    place_static(check_map, progression, Item::Fairy01, "Thieves' Town Item Shop (3)");
    place_static(check_map, progression, Item::Shield03, "Thieves' Town Item Shop (4)");

    // Lorule Lake Item Shop
    place_static(check_map, progression, Item::Bee02, "Lorule Lakeside Item Shop (1)");
    place_static(check_map, progression, Item::GoldBee03, "Lorule Lakeside Item Shop (2)");
    place_static(check_map, progression, Item::Fairy02, "Lorule Lakeside Item Shop (3)");
    place_static(check_map, progression, Item::Shield04, "Lorule Lakeside Item Shop (4)");

    // Nice Mode
    if !&settings.nice_mode {
        place_static(check_map, progression, Item::Bow02, "Maiamai Bow Upgrade");
        place_static(check_map, progression, Item::Boomerang02, "Maiamai Boomerang Upgrade");
        place_static(check_map, progression, Item::Hookshot02, "Maiamai Hookshot Upgrade");
        place_static(check_map, progression, Item::Hammer02, "Maiamai Hammer Upgrade");
        place_static(check_map, progression, Item::Bombs02, "Maiamai Bombs Upgrade");
        place_static(check_map, progression, Item::FireRod02, "Maiamai Fire Rod Upgrade");
        place_static(check_map, progression, Item::IceRod02, "Maiamai Ice Rod Upgrade");
        place_static(check_map, progression, Item::TornadoRod02, "Maiamai Tornado Rod Upgrade");
        place_static(check_map, progression, Item::SandRod02, "Maiamai Sand Rod Upgrade");
    }

    let mut shop_positions: Vec<String> = Vec::new();
    let mut bow_light_positions: Vec<String> = Vec::from(["[LC] Zelda".to_owned()]);
    let mut maiamai_positions: Vec<String> = Vec::new();

    for (check_name, item) in check_map.clone() {
        if check_name.starts_with("[LC]") && item.is_none() {
            let _ = &bow_light_positions.push(check_name.clone());
        } else if check_name.starts_with("Ravio's Shop") && !check_name.contains('6') {
            let _ = &shop_positions.push(check_name.clone());
        } else if check_name.starts_with("[Mai]") {
            let _ = &maiamai_positions.push(check_name.clone());
        }
    }

    if !settings.progressive_bow_of_light && settings.bow_of_light_in_castle {
        check_map.insert(
            bow_light_positions.remove(rng.gen_range(0..bow_light_positions.len())),
            Some(Item::BowOfLight.into()),
        );
        progression.retain(|x| *x != Item::BowOfLight);
    }

    // Bell in Shop
    if settings.bell_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Item::Bell.into()));
        progression.retain(|x| *x != Item::Bell);
    }

    // Pouch in Shop
    // if settings.logic.pouch_in_shop {
    //     check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Item::Pouch.into()));
    //     progression.retain(|x| *x != Item::Pouch);
    // }

    // Sword in Shop
    if settings.sword_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Item::Sword01.into()));
        progression.retain(|x| *x != Item::Sword01);
    }

    // Boots in Shop
    if settings.boots_in_shop {
        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Item::PegasusBoots.into()));
        progression.retain(|x| *x != Item::PegasusBoots);
    }

    // Assures a weapon will be available in Ravio's Shop
    if (!&settings.sword_in_shop && !&settings.boots_in_shop) && settings.assured_weapon {
        let mut weapons = Vec::from([
            Item::Bow01,
            Item::Bombs01,
            Item::FireRod01,
            Item::IceRod01,
            Item::Hammer01,
            Item::PegasusBoots,
        ]);

        if !&settings.swordless_mode {
            weapons.extend_from_slice(&[Item::Sword01]);
        }

        match settings.logic_mode {
            LogicMode::Normal => {},
            _ => {
                weapons.extend_from_slice(&[Item::Lamp01, Item::Net01]);
            },
        }

        let weapon = *weapons.get(rng.gen_range(0..weapons.len())).unwrap();

        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(weapon.into()));
        progression.retain(|x| *x != weapon);
    }

    // For non-Maiamai Madness seeds, default them to Maiamai
    // FIXME Inefficient to add Maiamai to progression pool, shuffle, then remove them
    if !&settings.maiamai_madness {
        let mut maiamai_items = get_maiamai_pool();
        for check_name in maiamai_positions {
            place_static(check_map, progression, maiamai_items.remove(0), &check_name);
        }
    }
}

/// Handle Exclusions
fn handle_exclusions(rng: &mut StdRng, seed_info: &mut SeedInfo, check_map: &mut CheckMap, junk: &mut Vec<Item>) {
    seed_info.full_exclusions = seed_info.settings.user_exclusions.clone();

    // todo
    seed_info.full_exclusions.insert("100 Maiamai".to_string());

    // Exclude Minigames
    if seed_info.settings.minigames_excluded {
        seed_info.full_exclusions.insert("Dodge the Cuccos".to_string());
        seed_info.full_exclusions.insert("Hyrule Hotfoot 75s".to_string());
        seed_info.full_exclusions.insert("Hyrule Hotfoot 65s".to_string());
        seed_info.full_exclusions.insert("Rupee Rush (Hyrule)".to_string());
        seed_info.full_exclusions.insert("Rupee Rush (Lorule)".to_string());
        seed_info.full_exclusions.insert("Octoball Derby".to_string());
        seed_info.full_exclusions.insert("Treacherous Tower".to_string());

        // For Maiamai Madness, also turn the rupee rush maiamai into random junk
        if seed_info.settings.maiamai_madness {
            seed_info.full_exclusions.insert("[Mai] Hyrule Rupee Rush Wall".to_string());
            seed_info.full_exclusions.insert("[Mai] Lorule Rupee Rush Wall".to_string());
        }
    }

    for exclusion in &seed_info.full_exclusions {
        if check_map.contains_key(exclusion) {
            let check_name = exclusion.to_owned();

            if let Some(Some(_)) = check_map.get(&check_name) {
                println!();
                warn!("Other settings prevented excluding: \"{}\"\n", check_name);
                continue;
            }

            let index = rng.gen_range(0..junk.len());
            check_map.insert(check_name, Some(junk.remove(index).into()));
        } else {
            println!();
            error!("Could not exclude \"{}\", no matching check found with that name.", exclusion);
            fail!("Consult a spoiler log for a list of valid check names.");
        }
    }
}

// Statically place an item in a given location, then remove it from the item pool provided
fn place_static(check_map: &mut CheckMap, pool: &mut Pool, item: Item, check_name: &str) {
    check_map.insert(check_name.to_owned(), Some(item.into()));
    pool.retain(|x| *x != item);
}

/// Super dirty mapping I hate it
fn build_layout(SeedInfo { layout, world_graph, .. }: &mut SeedInfo, check_map: &mut CheckMap) -> Result<(), Error> {
    for location_node in world_graph.values() {
        for check in location_node.clone().get_checks().iter().flatten().collect::<Vec<&Check>>() {
            if let Some(loc_info) = check.get_location_info() {
                let item = check_map.get(check.get_name()).unwrap().unwrap();
                layout.set(loc_info, item);
            }
        }
    }

    Ok(())
}

fn is_dungeon_prize(item: Item) -> bool {
    match item {
        Item::PendantOfPower
        | Item::PendantOfWisdom
        | Item::PendantOfCourage
        | Item::SageGulley
        | Item::SageOren
        | Item::SageSeres
        | Item::SageOsfala
        | Item::SageImpa
        | Item::SageIrene
        | Item::SageRosso => true,
        _ => false,
    }
}

fn is_dungeon_item(item: Item) -> bool {
    matches!(
        item,
        Item::HyruleSanctuaryKey
            | Item::LoruleSanctuaryKey
            | Item::EasternCompass
            | Item::EasternKeyBig
            | Item::EasternKeySmall01
            | Item::EasternKeySmall02
            | Item::GalesCompass
            | Item::GalesKeyBig
            | Item::GalesKeySmall01
            | Item::GalesKeySmall02
            | Item::GalesKeySmall03
            | Item::GalesKeySmall04
            | Item::HeraCompass
            | Item::HeraKeyBig
            | Item::HeraKeySmall01
            | Item::HeraKeySmall02
            | Item::DarkCompass
            | Item::DarkKeyBig
            | Item::DarkKeySmall01
            | Item::DarkKeySmall02
            | Item::DarkKeySmall03
            | Item::DarkKeySmall04
            | Item::SwampCompass
            | Item::SwampKeyBig
            | Item::SwampKeySmall01
            | Item::SwampKeySmall02
            | Item::SwampKeySmall03
            | Item::SwampKeySmall04
            | Item::SkullCompass
            | Item::SkullKeyBig
            | Item::SkullKeySmall01
            | Item::SkullKeySmall02
            | Item::SkullKeySmall03
            | Item::ThievesCompass
            | Item::ThievesKeyBig
            | Item::ThievesKeySmall
            | Item::IceCompass
            | Item::IceKeyBig
            | Item::IceKeySmall01
            | Item::IceKeySmall02
            | Item::IceKeySmall03
            | Item::DesertCompass
            | Item::DesertKeyBig
            | Item::DesertKeySmall01
            | Item::DesertKeySmall02
            | Item::DesertKeySmall03
            | Item::DesertKeySmall04
            | Item::DesertKeySmall05
            | Item::TurtleCompass
            | Item::TurtleKeyBig
            | Item::TurtleKeySmall01
            | Item::TurtleKeySmall02
            | Item::TurtleKeySmall03
            | Item::LoruleCastleCompass
            | Item::LoruleCastleKeySmall01
            | Item::LoruleCastleKeySmall02
            | Item::LoruleCastleKeySmall03
            | Item::LoruleCastleKeySmall04
            | Item::LoruleCastleKeySmall05
    )
}

fn fill_junk(rng: &mut StdRng, check_map: &mut CheckMap, junk_items: &mut Pool) {
    info!("Placing Junk Items...");

    let mut empty_check_keys = Vec::new();
    for (key, val) in check_map.clone() {
        if val.is_none() {
            empty_check_keys.push(key);
        }
    }

    if empty_check_keys.len() != junk_items.len() {
        fail!(
            "Number of empty checks: {} does not match available junk items: {}",
            empty_check_keys.len(),
            junk_items.len()
        );
    }

    for junk in junk_items {
        check_map.insert(empty_check_keys.remove(rng.gen_range(0..empty_check_keys.len())), Some((*junk).into()));
    }
}

fn place_item_randomly(item: Item, checks: &Vec<Check>, check_map: &mut CheckMap, rng: &mut StdRng) {
    check_map.insert(checks.get(rng.gen_range(0..checks.len())).unwrap().get_name().to_owned(), Some(item.into()));
}

fn filter_checks(item: Item, checks: &[Check], check_map: &mut CheckMap) -> Vec<Check> {
    // Filter out non-empty checks
    let mut filtered_checks =
        checks.iter().filter(|&x| check_map.get(x.get_name()).unwrap().is_none()).cloned().collect::<Vec<_>>();

    // Filter checks by item type
    if is_dungeon_prize(item) {
        filtered_checks = filter_dungeon_prize_checks(&filtered_checks);
    } else if is_dungeon_item(item) {
        let is_keysanity = false; // No keysanity yet, hardcode to false
        if !is_keysanity {
            filtered_checks = filter_dungeon_checks(item, filtered_checks);
        }
    }

    filtered_checks
}

fn filter_dungeon_prize_checks(eligible_checks: &Vec<Check>) -> Vec<Check> {
    let dungeon_prize_checks = vec![
        "[EP] Prize", "[HG] Prize", "[TH] Prize", "[PD] Prize", "[SP] Prize", "[SW] Prize", "[TT] Prize", "[TR] Prize",
        "[DP] Prize", "[IR] Prize",
    ];

    eligible_checks
        .iter()
        .filter_map(|check| if dungeon_prize_checks.contains(&check.get_name()) { Some(*check) } else { None })
        .collect()
}

fn filter_dungeon_checks(item: Item, eligible_checks: Vec<Check>) -> Vec<Check> {
    use Item::*;

    let dungeon_checks = match item {
        HyruleSanctuaryKey => vec!["[HS] Entrance", "[HS] Ledge", "[HS] Lower Chest", "[HS] Upper Chest"],
        LoruleSanctuaryKey => vec!["[LS] Entrance Chest", "[LS] Ledge", "[LS] Lower Chest", "[LS] Upper Chest"],
        EasternCompass | EasternKeyBig | EasternKeySmall01 | EasternKeySmall02 => vec![
            "[EP] (1F) Merge Chest", "[EP] (1F) Left Door Chest", "[EP] (1F) Popo Room", "[EP] (1F) Secret Room",
            "[EP] (1F) Switch Room", "[EP] (2F) Ball Room", "[EP] (2F) Defeat Popos", "[EP] (2F) Switch Room",
            "[EP] (2F) Big Chest", "[EP] Yuga (1)", "[EP] Yuga (2)", "[EP] (3F) Escape Chest",
            "[EP] (1F) Escape Chest",
        ],
        GalesCompass | GalesKeyBig | GalesKeySmall01 | GalesKeySmall02 | GalesKeySmall03 | GalesKeySmall04 => vec![
            "[HG] (1F) Torches", "[HG] (1F) Switch Room", "[HG] (1F) Fire Bubbles", "[HG] (1F) West Room",
            "[HG] (1F) West Room Secret", "[HG] (2F) Big Chest", "[HG] (2F) Narrow Ledge", "[HG] (2F) Fire Ring",
            "[HG] (3F) Rat Room", "[HG] (3F) Fire Bubbles", "[HG] Margomill",
        ],
        HeraCompass | HeraKeyBig | HeraKeySmall01 | HeraKeySmall02 => vec![
            "[TH] (1F) Outside", "[TH] (1F) Center", "[TH] (3F) Platform", "[TH] (5F) Red/Blue Switches",
            "[TH] (6F) Left Mole", "[TH] (6F) Right Mole", "[TH] (7F) Outside (Ledge)", "[TH] (8F) Fairy Room",
            "[TH] (11F) Big Chest", "[TH] Moldorm",
        ],
        DarkCompass | DarkKeyBig | DarkKeySmall01 | DarkKeySmall02 | DarkKeySmall03 | DarkKeySmall04 => vec![
            "[PD] (1F) Right Pit", "[PD] (1F) Left Pit", "[PD] (1F) Switch Puzzle", "[PD] (1F) Hidden Room (Upper)",
            "[PD] (1F) Hidden Room (Lower)", "[PD] (B1) Fall From 1F", "[PD] (B1) Glow-in-the-Dark Maze",
            "[PD] (B1) Helmasaur Room", "[PD] (B1) Helmasaur Room (Fall)", "[PD] (2F) Big Chest (Hidden)",
            "[PD] (2F) Alcove", "[PD] (1F) Fall From 2F", "[PD] (2F) South Hidden Room", "[PD] (B1) Bomb Bowling",
            "[PD] Gemesaur King",
        ],
        SwampCompass | SwampKeyBig | SwampKeySmall01 | SwampKeySmall02 | SwampKeySmall03 | SwampKeySmall04 => vec![
            "[SP] (B1) Center", "[SP] (B1) Raft Room (Left)", "[SP] (B1) Raft Room (Right)",
            "[SP] (B1) Raft Room (Pillar)", "[SP] (B1) Gyorm", "[SP] (B1) Waterfall Room",
            "[SP] (B1) Big Chest (Secret)", "[SP] (1F) Water Puzzle", "[SP] (1F) East Room", "[SP] (1F) West Room",
            "[SP] (1F) Big Chest (Fire)", "[SP] Arrghus",
        ],
        SkullCompass | SkullKeyBig | SkullKeySmall01 | SkullKeySmall02 | SkullKeySmall03 => vec![
            "[SW] (B1) Gibdo Room (Lower)", "[SW] (B1) South Chest", "[SW] (B1) Gibdo Room (Hole)",
            "[SW] (B1) Grate Room", "[SW] (B2) Moving Platform Room", "[SW] (B1) Big Chest (Upper)",
            // "[SW] Outdoor Chest",
            "[SW] (B1) Big Chest (Eyes)", "[SW] Knucklemaster",
        ],
        ThievesCompass | ThievesKeyBig | ThievesKeySmall => vec![
            "[TT] (B1) Jail Cell", "[TT] (B1) Grate Chest", "[TT] (B2) Grate Chest (Fall)",
            "[TT] (B2) Switch Puzzle Room", "[TT] (B2) Jail Cell", "[TT] (B2) Eyegores", "[TT] (B1) Behind Wall",
            "[TT] (B1) Big Chest (Entrance)", "[TT] (B3) Underwater",
            "[TT] (B3) Big Chest (Hidden)",
            // "[TT] Stalblind",
        ],
        IceCompass | IceKeyBig | IceKeySmall01 | IceKeySmall02 | IceKeySmall03 => vec![
            "[IR] (1F) Hidden Chest", "[IR] (B3) Grate Chest (Left)", "[IR] (B3) Grate Chest (Right)",
            "[IR] (B4) Ice Pillar", "[IR] (B5) Big Chest", "[IR] (B1) East Chest", "[IR] (B1) Narrow Ledge",
            "[IR] (B1) Upper Chest", "[IR] (B3) Big Chest (Puzzle)", "[IR] (B4) Switches",
            "[IR] (B4) Southwest Chest (Fall)", "[IR] (B4) Narrow Platform", "[IR] (B2) Long Merge Chest",
            "[IR] (B4) Southeast Chest (Fall)", "[IR] Dharkstare",
        ],
        DesertCompass | DesertKeyBig | DesertKeySmall01 | DesertKeySmall02 | DesertKeySmall03 | DesertKeySmall04
        | DesertKeySmall05 => vec![
            "[DP] (1F) Entrance", "[DP] (1F) Sand Room (South)", "[DP] (1F) Sand Switch Room",
            "[DP] (1F) Sand Room (North)", "[DP] (1F) Behind Rocks", "[DP] (1F) Big Chest (Behind Wall)",
            "[DP] (2F) Under Rock (Left)", "[DP] (2F) Beamos Room", "[DP] (2F) Under Rock (Right)",
            "[DP] (2F) Under Rock (Ball Room)", "[DP] (2F) Big Chest (Puzzle)", "[DP] (2F) Red/Blue Switches",
            "[DP] (2F) Leever Room", "[DP] (3F) Behind Falling Sand", "[DP] (3F) Armos Room",
            // "[DP] Zaganaga",
        ],
        TurtleCompass | TurtleKeyBig | TurtleKeySmall01 | TurtleKeySmall02 | TurtleKeySmall03 => vec![
            "[TR] (1F) Center", "[TR] (1F) Grate Chest", "[TR] (1F) Portal Room NW", "[TR] (1F) Northeast Ledge",
            "[TR] (1F) Southeast Chest", "[TR] (1F) Defeat Flamolas",
            // "[TR] Left Balcony",
            "[TR] (1F) Under Center", "[TR] (B1) Under Center", "[TR] (B1) Northeast Room", "[TR] (B1) Platform",
            "[TR] (B1) Grate Chest (Small)", "[TR] (B1) Big Chest (Center)", "[TR] (B1) Big Chest (Top)",
            "[TR] Grinexx",
        ],
        LoruleCastleCompass
        | LoruleCastleKeySmall01
        | LoruleCastleKeySmall02
        | LoruleCastleKeySmall03
        | LoruleCastleKeySmall04
        | LoruleCastleKeySmall05 => vec![
            "[LC] (1F) Ledge", "[LC] (1F) Center", "[LC] (2F) Near Torches", "[LC] (2F) Hidden Path",
            "[LC] (2F) Ledge", "[LC] (4F) Center", "[LC] (4F) Hidden Path", "[LC] Bomb Trial I", "[LC] Bomb Trial II",
            "[LC] Tile Trial I", "[LC] Tile Trial II", "[LC] Lamp Trial", "[LC] Hook Trial I",
            "[LC] Hook Trial II",
            // "[LC] Zelda",
        ],

        _ => {
            fail!("Item {:?} is not a dungeon item", item);
        },
    };

    eligible_checks
        .iter()
        .filter_map(|check| if dungeon_checks.contains(&check.get_name()) { Some(*check) } else { None })
        .collect()
}

fn exist_empty_reachable_check(checks: &Vec<Check>, check_map: &mut CheckMap) -> bool {
    for check in checks {
        match check_map.get(check.get_name()).unwrap() {
            None => {
                return true;
            },
            Some(_) => {},
        }
    }

    false
}

/// Prefills a map with all checks as defined by the world graph with no values yet assigned
pub fn prefill_check_map(world_graph: &mut WorldGraph) -> CheckMap {
    let mut check_map: DashMap<_, _> = Default::default();

    for location_node in world_graph.values_mut() {
        for check in location_node.clone().get_checks().iter().flatten().collect::<Vec<&Check>>() {
            if check_map.insert(check.get_name().to_owned(), check.get_quest()).is_some() {
                fail!("Multiple checks have duplicate name: {}", check.get_name());
            }
        }
    }

    check_map
}

/// This translation is probably adding unnecessary overhead, oh well
fn build_progress_from_items<'s>(items: &Pool, seed_info: &'s SeedInfo) -> Progress<'s> {
    let mut progress = Progress::new(seed_info);
    for item in items {
        progress.add_item(*item);
    }

    progress
}

/// Verifies that, assuming we have all possible pieces of player progression, all locations in the world graph can be
/// reached. This is the baseline check for an uncompletable seed, usually because of something like Entrance or Portal
/// randomization resulting in a layout that renders certain locations inaccessible.
fn verify_all_locations_accessible(
    seed_info: &SeedInfo, check_map: &mut CheckMap, progression_pool: &mut Pool,
) -> Result<(), Error> {
    if LogicMode::NoLogic.eq(&seed_info.settings.logic_mode) {
        return Ok(()); // Skip this check on No Logic
    }

    info!("Verifying all locations accessible...");
    let reachable_checks = assumed_search(seed_info, progression_pool, check_map); //find_reachable_checks(loc_map, &everything, &mut check_map); //

    const STANDARD_CHECKS: usize = 264;
    const MAIAMAI: usize = 100;
    const DUNGEON_PRIZES: usize = 10;
    const STATIC_ITEMS: usize = 20;
    const PROGRESSION_EVENTS: usize = 36; // "Progression Events" (non-item checks that are still progression)
    const WEATHER_VANES: usize = 22;
    const HINT_GHOSTS_OW: usize = 58; // Hint Ghosts (Overworld)

    // (2 Golden Bees for 9,999 Rupees NOT included)
    const IN_LOGIC_CHECKS: usize = STANDARD_CHECKS + MAIAMAI + DUNGEON_PRIZES + STATIC_ITEMS;

    /// Total count of expected, reachable checks
    const EXPECTED_REACHABLE: usize = IN_LOGIC_CHECKS + PROGRESSION_EVENTS + WEATHER_VANES + HINT_GHOSTS_OW;

    if reachable_checks.len() != EXPECTED_REACHABLE {
        // let reachable_check_names: Vec<&str> = reachable_checks.iter().map(|c| c.get_name()).collect();
        // for check in check_map.keys() {
        //     if reachable_check_names.contains(&check.as_str()) {
        //         // info!("Reachable Check: {}", check);
        //     } else {
        //         if !check.contains("Portal") {
        //             info!("Unreachable Check: {}", check);
        //         }
        //     }
        // }

        Err(Error::new(format!(
            "Only {}/{} checks were reachable in the world graph",
            reachable_checks.len(),
            EXPECTED_REACHABLE
        )))
    } else {
        Ok(())
    }
}

/// Find all checks reachable with the given Progress
pub(crate) fn find_reachable_checks(SeedInfo { world_graph, .. }: &SeedInfo, progress: &Progress) -> Vec<Check> {
    let start_node = Location::RavioShop;
    let mut loc_queue: Queue<Location> = Queue::from(vec![start_node]);
    let mut visited: HashSet<Location> = HashSet::new();
    let mut reachable_checks: Vec<Check> = Vec::new(); // possibly switch to HashSet to avoid dupes

    visited.insert(start_node);

    while !loc_queue.is_empty() {
        let location = loc_queue.dequeue().unwrap();

        // Grab the location from the map, verify it is defined
        let location_node = match world_graph.get(&location) {
            Some(loc) => loc,
            None => {
                fail!("Location Undefined: {:?}", location);
            },
        };

        // Iterate over the location's checks
        for check in location_node.clone().get_checks().iter().flatten().collect::<Vec<&Check>>() {
            if check.can_access(progress) {
                reachable_checks.push(*check);
            }
        }

        // Queue new paths reachable from this location
        for path in location_node.clone().get_paths().iter().flatten().collect::<Vec<&Path>>() {
            let destination = path.get_destination();
            if !visited.contains(&destination) && path.can_access(progress) {
                loc_queue.queue(destination).expect("TODO: panic message");
                visited.insert(destination);
            }
        }
    }

    reachable_checks
}

pub(crate) fn get_items_from_reachable_checks<'s>(
    seed_info: &'s SeedInfo, reachable_checks: &Vec<Check>, check_map: &mut CheckMap,
) -> Progress<'s> {
    let mut progress = Progress::new(seed_info);

    for check in reachable_checks {
        // Items already placed in the world that can be picked up
        let placed_item = check_map.get(check.get_name()).unwrap();
        match placed_item {
            None => {},
            Some(item) => match item {
                Vane(vane) => progress.add_item(*seed_info.vane_map.get(vane).unwrap()),
                _ => progress.add_item(*item),
            },
        }

        // Quest items that will always be at a given check
        if let Some(quest) = check.get_quest() {
            match quest {
                Vane(vane) => progress.add_item(*seed_info.vane_map.get(&vane).unwrap()),
                _ => progress.add_item(quest),
            }
        }
    }

    progress
}

/// The Assumed Fill algorithm
///
/// Randomly places `items_owned` into the `check_map` in a completable manner as informed by the
/// logic defined in the `world_graph` and `settings`.
///
/// Items are placed "backwards", *assuming* that all items that have yet to be placed are
/// available without the item currently being placed.
///
/// An assumed search algorithm is used to identify all locations reachable without the item
/// currently being placed.
///
/// * `world_graph` - A graph representing the comprehensive structure of the game world
/// * `rng` - The RNG seed
/// * `items_owned` - The pool of all progression-granting items
/// * `check_map` - A map representing all checks and items assigned to them
/// * `settings` - Game settings
fn assumed_fill(
    rng: &mut StdRng, seed_info: &SeedInfo, check_map: &mut CheckMap, items_owned: &mut Pool,
) -> crate::Result<()> {
    info!("Placing Progression Items...");

    let mut reachable_checks = assumed_search(seed_info, items_owned, check_map);

    while exist_empty_reachable_check(&reachable_checks, check_map) && !items_owned.is_empty() {
        let item = items_owned.remove(0);

        //
        reachable_checks = assumed_search(seed_info, items_owned, check_map);

        let filtered_checks = filter_checks(item, &reachable_checks, check_map);

        if filtered_checks.is_empty() {
            info!("item:            {:?}", item);
            info!("filtered_checks: {:?}", filtered_checks);
            info!("check_map:       {:?}", check_map);

            return Err(crate::Error::game(format!("No reachable checks found to place: {:?}", item)));
        }

        place_item_randomly(item, &filtered_checks, check_map, rng);
    }

    Ok(())
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
fn assumed_search(seed_info: &SeedInfo, items_owned: &Pool, check_map: &mut CheckMap) -> Vec<Check> {
    let mut considered_items = build_progress_from_items(items_owned, seed_info);
    let mut reachable_checks: Vec<Check>;

    loop {
        reachable_checks = find_reachable_checks(seed_info, &considered_items);
        let reachable_items = get_items_from_reachable_checks(seed_info, &reachable_checks, check_map);

        let new_items = reachable_items.difference(&considered_items);

        if new_items.is_empty() {
            break;
        }

        for new_item in new_items {
            considered_items.add_item(new_item);
        }
    }

    reachable_checks
}

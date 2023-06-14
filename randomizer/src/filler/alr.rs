use {
    crate::{
        filler,
        filler::{item_pools, util},
    },
    log::{error, info},
    macros::fail,
    rand::{prelude::StdRng, Rng},
    seed::{
        filler::{
            filler_item::{FillerItem, FillerItem::*},
            pool::{self, Pool},
        },
        settings::{
            logic_mode::LogicMode::{self, *},
            Settings,
        },
        world::{CheckId, CheckId::*},
        Seed,
    },
};

/// Fill Seed such that All Locations are Reachable
///
/// This is the "standard" filler algorithm for ALBWR.
pub(crate) fn fill_all_locations_reachable(seed: &mut Seed, settings: &Settings, rng: &mut StdRng) {
    let (mut progression_pool, mut junk_pool) = item_pools::get_item_pools(settings, rng);

    verify_all_locations_accessible(seed, &mut progression_pool, settings)?;
    handle_exclusions(seed, settings, rng, &mut junk_pool);
    preplace_items(seed, settings, rng, &mut progression_pool, &mut junk_pool);
    assumed_fill(seed, settings, rng, &mut progression_pool)?;
    fill_junk(seed, rng, &mut junk_pool);
}

///
fn verify_all_locations_accessible(
    seed: &mut Seed, progression_pool: &Pool, settings: &Settings,
) -> Result<(), Box<dyn error::Error>> {
    // Skip this check on No Logic
    if NoLogic.eq(&settings.logic.logic_mode) {
        return Ok(());
    }

    info!("Verifying all locations accessible...");
    let reachable_checks = util::assumed_search(seed, progression_pool.clone(), settings)?;

    /**
     * 384 In-Logic Checks
     *
     * - 254 Standard Checks
     * - 100 Maiamai
     * - 11 Dungeon Prizes
     * - 19 Statically Placed Items:
     *     - 12x Shop Items (not including 9,999 items)
     *     - 3x Obscure Gold/Silver Rupees
     *     - Mysterious Man
     *     - TODO: Letter in a Bottle
     *     - TODO: Hyrule Hotfoot Second Race
     *     - TODO: Fortune's Choice
     *
     * 14 Out-of-Logic checks NOT included:
     * - TODO: 10 Maiamai Rewards
     * - 2 Golden Bees for 9,999 Rupees
     * - 2 Treacherous Tower Advanced
     */
    const IN_LOGIC_CHECKS: usize = 384;

    /// "Progression Events" (non-item checks that are still progression)
    const PROGRESSION_EVENTS: usize = 33;

    /// Hint Ghosts (Overworld)
    const HINT_GHOSTS_OW: usize = 58;

    const TOTAL_CHECKS: usize = IN_LOGIC_CHECKS + PROGRESSION_EVENTS + HINT_GHOSTS_OW;

    if reachable_checks.len() != TOTAL_CHECKS {
        for (check_id, _) in seed.get_check_map() {
            if !reachable_checks.contains(check_id) {
                info!("Unreachable Check: {:?}", check_id);
            }
        }

        fail!(
            "Only {}/{} checks were reachable in the world graph",
            reachable_checks.len(),
            TOTAL_CHECKS
        );
    }

    Ok(())
}

///
fn handle_exclusions(seed: &mut Seed, settings: &Settings, rng: &mut StdRng, junk_pool: &mut Pool) {
    let mut exclusions = if let Some(exclusions) = settings.exclusions.0.get("exclusions") {
        exclusions
    } else {
        return;
    };

    for (check_id, check) in seed.get_check_map() {
        for exclusion in exclusions {
            if check.get_name().eq(exclusion) {
                let junk_pool_len = junk_pool.len();
                check.set_item(junk_pool.remove(rng.get_range(0..junk_pool_len)));
                exclusions.remove(exclusion); // does rust handle mid-iteration removes well...?
            } else {
                error!("Cannot exclude \"{}\", no matching check found with that name.", exclusion);
                fail!("Consult a spoiler log for a list of valid check names.");
            }
        }
    }
}

/// Place static items ahead of the randomly filled ones
fn preplace_items(
    seed: &mut Seed, settings: &Settings, rng: &mut StdRng, progression: &mut Pool, junk: &mut Pool,
) {
    // Vanilla Dungeon Prizes
    if !settings.logic.randomize_dungeon_prizes {
        place_static(seed, progression, PendantOfCourage01, "Eastern Palace Prize");
        place_static(seed, progression, PendantOfWisdom, "House of Gales Prize");
        place_static(seed, progression, PendantOfPower, "Tower of Hera Prize");
        place_static(seed, progression, PendantOfCourage02, "Hyrule Castle Prize");
        place_static(seed, progression, SageGulley, "Dark Palace Prize");
        place_static(seed, progression, SageOren, "Swamp Palace Prize");
        place_static(seed, progression, SageSeres, "Skull Woods Prize");
        place_static(seed, progression, SageOsfala, "Thieves' Hideout Prize");
        place_static(seed, progression, SageImpa, "Turtle Rock Prize");
        place_static(seed, progression, SageIrene, "Desert Palace Prize");
        place_static(seed, progression, SageRosso, "Ice Ruins Prize");
    }

    // Place un-randomized items
    place_static(seed, progression, LetterInABottle, "Southeastern Shore");
    place_static(seed, progression, RupeeSilver40, "Hyrule Hotfoot (Second Race)");
    place_static(seed, progression, RupeeSilver41, "[TR] (1F) Under Center");
    place_static(seed, progression, RupeeGold09, "[TR] (B1) Under Center");
    place_static(seed, progression, RupeeGold10, "[PD] (2F) South Hidden Room");
    place_static(seed, progression, HeartPiece28, "Fortune's Choice");

    // Kakariko Item Shop
    place_static(seed, progression, ScootFruit01, "Kakariko Item Shop (1)");
    place_static(seed, progression, FoulFruit01, "Kakariko Item Shop (2)");
    place_static(seed, progression, Shield01, "Kakariko Item Shop (3)");

    // Lakeside Item Shop
    place_static(seed, progression, ScootFruit02, "Lakeside Item Shop (1)");
    place_static(seed, progression, FoulFruit02, "Lakeside Item Shop (2)");
    place_static(seed, progression, Shield02, "Lakeside Item Shop (3)");

    // Mysterious Man
    place_static(seed, progression, GoldBee01, "Mysterious Man");

    // Thieves' Town Item Shop
    place_static(seed, progression, Bee01, "Thieves' Town Item Shop (1)");
    place_static(seed, progression, GoldBee02, "Thieves' Town Item Shop (2)");
    place_static(seed, progression, Fairy01, "Thieves' Town Item Shop (3)");
    place_static(seed, progression, Shield03, "Thieves' Town Item Shop (4)");

    // Lorule Lake Item Shop
    place_static(seed, progression, Bee02, "Lorule Lakeside Item Shop (1)");
    place_static(seed, progression, GoldBee03, "Lorule Lakeside Item Shop (2)");
    place_static(seed, progression, Fairy02, "Lorule Lakeside Item Shop (3)");
    place_static(seed, progression, Shield04, "Lorule Lakeside Item Shop (4)");

    // Super Items
    if settings.logic.super_items {
        exclude("Treacherous Tower Advanced (1)", seed, rng, junk);
        exclude("Treacherous Tower Advanced (2)", seed, rng, junk);
    } else {
        place_static(seed, progression, Lamp02, "Treacherous Tower Advanced (1)");
        place_static(seed, progression, Net02, "Treacherous Tower Advanced (2)");
    }

    // Nice Mode
    if settings.logic.nice_mode {
        exclude(Maiamai10, seed, rng, junk);
        exclude(Maiamai20, seed, rng, junk);
        exclude(Maiamai30, seed, rng, junk);
        exclude(Maiamai40, seed, rng, junk);
        exclude(Maiamai50, seed, rng, junk);
        exclude(Maiamai60, seed, rng, junk);
        exclude(Maiamai70, seed, rng, junk);
        exclude(Maiamai80, seed, rng, junk);
        exclude(Maiamai90, seed, rng, junk);
    } else {
        // hacky
        place_static(seed, progression, Bow02, Maiamai10);
        place_static(seed, progression, Boomerang02, Maiamai20);
        place_static(seed, progression, Hookshot02, Maiamai30);
        place_static(seed, progression, Hammer02, Maiamai40);
        place_static(seed, progression, Bombs02, Maiamai50);
        place_static(seed, progression, FireRod02, Maiamai60);
        place_static(seed, progression, IceRod02, Maiamai70);
        place_static(seed, progression, TornadoRod02, Maiamai80);
        place_static(seed, progression, SandRod02, Maiamai90);
    }
    exclude(Maiamai100, seed, rng, junk);

    let mut shop_positions: Vec<CheckId> =
        Vec::from([Ravio1, Ravio2, Ravio3, Ravio4, Ravio5, Ravio7, Ravio8, Ravio9]);
    let mut bow_light_positions: Vec<CheckId> = Vec::from([/* todo */]);
    let mut maiamai_positions: Vec<CheckId> = Vec::from([/* todo */]);

    if settings.logic.bow_of_light_in_castle {
        check_map.insert(
            bow_light_positions.remove(rng.gen_range(0..bow_light_positions.len())),
            Some(BowOfLight),
        );
        progression.retain(|x| *x != BowOfLight);
    }

    // Bell in Shop
    if settings.logic.bell_in_shop {
        check_map.insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Bell));
        progression.retain(|x| *x != Bell);
    }

    // Pouch in Shop
    if settings.logic.pouch_in_shop {
        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Pouch));
        progression.retain(|x| *x != Pouch);
    }

    // Sword in Shop
    if settings.logic.sword_in_shop {
        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(Sword01));
        progression.retain(|x| *x != Sword01);
    }

    // Boots in Shop
    if settings.logic.boots_in_shop {
        check_map.insert(
            shop_positions.remove(rng.gen_range(0..shop_positions.len())),
            Some(PegasusBoots),
        );
        progression.retain(|x| *x != PegasusBoots);
    }

    // Assures a weapon will be available in Ravio's Shop
    if (!settings.logic.sword_in_shop && !settings.logic.boots_in_shop)
        && settings.logic.assured_weapon
    {
        let mut weapons = Vec::from([Bow01, Bombs01, FireRod01, IceRod01, Hammer01, PegasusBoots]);

        if !settings.logic.swordless_mode {
            weapons.extend_from_slice(&[Sword01]);
        }

        match settings.logic.logic_mode {
            Normal => {}
            _ => {
                weapons.extend_from_slice(&[Lamp01, Net01]);
            }
        }

        let weapon = *weapons.get(rng.gen_range(0..weapons.len())).unwrap();

        check_map
            .insert(shop_positions.remove(rng.gen_range(0..shop_positions.len())), Some(weapon));
        progression.retain(|x| *x != weapon);
    }

    // Exclude Minigames
    if settings.logic.minigames_excluded {
        exclude(DodgeTheCuccos, seed, rng, junk);
        exclude("Hyrule Hotfoot (First Race)", seed, rng, junk);
        exclude("Rupee Rush (Hyrule)", seed, rng, junk);
        exclude("Rupee Rush (Lorule)", seed, rng, junk);
        exclude("Octoball Derby", seed, rng, junk);
        exclude("Treacherous Tower Intermediate", seed, rng, junk);

        // For Maiamai Madness, also turn the rupee rush maiamai into random junk
        if settings.logic.maiamai_madness {
            exclude("[Mai] Hyrule Rupee Rush Wall", seed, rng, junk);
            exclude("[Mai] Lorule Rupee Rush Wall", seed, rng, junk);
        }
    }

    // For non-Maiamai Madness seeds, default them to Maiamai
    // FIXME Inefficient to add Maiamai to progression pool, shuffle, then remove them
    if !settings.logic.maiamai_madness {
        let mut maiamai_items = pool::get_maiamai_pool();
        for check_id in maiamai_positions {
            place_static(check_map, progression, maiamai_items.remove(0), check_id);
        }
    }
}

// Statically place an item in a given location, then remove it from the item pool provided
fn place_static(seed: &mut Seed, pool: &mut Pool, item: FillerItem, check_id: CheckId) {
    seed.get_check(&check_id).set_item(item);
    pool.retain(|x| *x != item);
}

// Exclude a location by placing a random junk item there
fn exclude(check_id: CheckId, seed: &mut Seed, rng: &mut StdRng, junk: &mut Pool) {
    seed.get_check(&check_id).set_item(junk.remove(rng.gen_range(0..junk.len())));
}

/// The Assumed Fill algorithm
///
/// Randomly places `items_owned` into the `seed` in a completable manner.
///
/// Items are placed "backwards", *assuming* that all items that have yet to be placed are
/// available without the item currently being placed.
///
/// An assumed search algorithm is used to identify all locations reachable without the item
/// currently being placed.
///
/// * `seed` - The SeedWorld
/// * `settings` - Game settings
/// * `rng` - The RNG seed
/// * `items_owned` - The pool of all progression-granting items
fn assumed_fill(
    seed: &mut Seed, settings: &Settings, rng: &mut StdRng, items_owned: &mut Pool,
) -> Result<(), Box<dyn error::Error>> {
    info!("Placing Progression Items...");

    let mut reachable_check_ids = util::assumed_search(seed, items_owned.clone(), settings)?;

    while exist_empty_reachable_check(&reachable_check_ids, seed) && !items_owned.is_empty() {
        let item = items_owned.remove(0);

        reachable_check_ids = util::assumed_search(seed, items_owned.clone(), settings)?;

        let filtered_checks = filler::filter_checks(seed, item, reachable_check_ids);

        if filtered_checks.len() == 0 {
            info!("No reachable checks found to place: {:?}", item);
        }

        filler::place_item_randomly(item, &filtered_checks, check_map, rng);
    }

    Ok(())
}

/// Are there any logically reachable empty checks?
fn exist_empty_reachable_check(reachable_check_ids: &Vec<CheckId>, seed: &mut Seed) -> bool {
    for check_id in reachable_check_ids {
        if seed.get_check(check_id).get_item().is_none() {
            return true;
        }
    }

    false
}

/// Fill in all remaining empty checks with random junk
fn fill_junk(seed: &mut Seed, rng: &mut StdRng, junk_items: &mut Pool) {
    info!("Placing Junk Items...");

    let mut empty_checks = Vec::new();
    for (check_id, check) in seed.get_check_map() {
        if check.get_item().is_none() {
            empty_checks.push(check_id);
        }
    }

    if empty_checks.len() != junk_items.len() {
        fail!(
            "Number of empty checks: {} does not match available junk items: {}",
            empty_checks.len(),
            junk_items.len()
        );
    }

    for junk in junk_items {
        let rng_index = rng.gen_range(0..empty_checks.len());
        check_map.insert(empty_checks.remove(rng_index), Some(*junk));
    }
}

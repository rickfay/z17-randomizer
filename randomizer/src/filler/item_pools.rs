use {
    crate::{filler::util::shuffle, filler_util::shuffle},
    rand::prelude::StdRng,
    seed::{
        filler::{
            filler_item::{FillerItem, FillerItem::*},
            pool::*,
        },
        settings::{logic_mode::LogicMode, Settings},
    },
};

/**
 * Builds the Progression and Junk item pools according to the settings<br /><br />
 *
 * The total number of items returned between both pools should match the total number of locations
 * in the world graph, including locations that statically set their contents.
 */
pub fn get_item_pools(settings: &Settings, rng: &mut StdRng) -> (Pool, Pool) {
    let mut progression_items = get_base_progression_pool();
    let dungeon_prizes = get_dungeon_prize_pool();
    let big_keys = get_big_key_pool();
    let small_keys = get_small_key_pool();
    let compasses = get_compass_pool();
    let mut junk_pool = get_base_junk_pool();

    // Remove the Bee Badge from Hell Logic to keep Bee Boosting viable
    match settings.logic.logic_mode {
        LogicMode::Hell => junk_pool.push(Empty),
        _ => progression_items.push(BeeBadge),
    };

    // Swordless Mode
    if settings.logic.swordless_mode {
        junk_pool.extend_from_slice(&[Empty, Empty, Empty, Empty]);
    } else {
        progression_items.extend_from_slice(&[Sword01, Sword02, Sword03, Sword04]);
    }

    (
        shuffle_order_progression_pools(
            rng, dungeon_prizes, big_keys, small_keys, compasses, progression_items,
        ),
        shuffle(rng, junk_pool),
    )
}

/**
 * Shuffle item categories amongst themselves, then order them as follows:
 * - Dungeon Prizes
 * - Big Keys
 * - Small Keys
 * - Compasses
 * - All other progression
 */
fn shuffle_order_progression_pools(
    rng: &mut StdRng, dungeon_prizes: Vec<FillerItem>, big_keys: Vec<FillerItem>,
    small_keys: Vec<FillerItem>, compasses: Vec<FillerItem>, progression: Vec<FillerItem>,
) -> Pool {
    let mut progression_pool;

    progression_pool = shuffle(rng, dungeon_prizes);
    progression_pool.extend(shuffle(rng, big_keys));
    progression_pool.extend(shuffle(rng, small_keys));
    progression_pool.extend(shuffle(rng, compasses));
    progression_pool.extend(shuffle(rng, progression));

    progression_pool
}

use crate::filler::tower_stage::TowerStage;
use crate::Result;
use game::Course::{EnemyAttackL, EnemyAttackM, EnemyAttackS};
use macros::fail;
use modinfo::Settings;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

/// Choose which floors of Treacherous Tower to use
pub fn choose_floors(settings: &Settings, rng: &mut StdRng) -> Result<Vec<TowerStage>> {
    let mut chosen_floors = Vec::with_capacity(settings.treacherous_tower_floors);

    if settings.treacherous_tower_floors < 2 {
        fail!("Treacherous Tower must have at least 2 floors.");
    } else if settings.treacherous_tower_floors > 66 {
        fail!("Treacherous Tower may have at most 66 floors.");
    }

    // Choose (n - 2) random filler floors.
    // Since Advanced floors drastically outnumber Intermediate and Beginner floors, use a weighted factor to keep the
    // result set in roughly an even Advanced/Intermediate/Beginner split, if possible. This won't work as effectively
    // with more than 15 floors as we'll run out of Beginner and Intermediate floors.
    floors()
        .choose_multiple_weighted(rng, settings.treacherous_tower_floors - 2, |(course, _)| match course {
            EnemyAttackS => 16.0,
            EnemyAttackM => 48.0 / 13.0,
            EnemyAttackL => 1.0,
            _ => unreachable!(),
        })
        .unwrap()
        .for_each(|(course, stage)| chosen_floors.push(TowerStage::new(*course, *stage as usize)));

    // Sort so we (theoretically) have an increasing difficulty curve
    chosen_floors.sort();

    // First floor is always an actual 1st Floor because I can't figure out how to set the Stage # in the MSBF commands :P
    let first_course = [EnemyAttackS, EnemyAttackM, EnemyAttackL].choose(rng).unwrap();
    chosen_floors.insert(0, TowerStage::new(*first_course, 1));

    // Regular Moldorm is always the last floor
    chosen_floors.push(TowerStage::new(EnemyAttackS, 5));

    Ok(chosen_floors)
}

fn floors() -> Vec<(game::Course, u32)> {
    vec![
        (EnemyAttackS, 2),
        (EnemyAttackS, 3),
        (EnemyAttackS, 4),
        (EnemyAttackM, 2),
        (EnemyAttackM, 3),
        (EnemyAttackM, 4),
        (EnemyAttackM, 5),
        (EnemyAttackM, 6),
        (EnemyAttackM, 7),
        (EnemyAttackM, 8),
        (EnemyAttackM, 9),
        (EnemyAttackM, 10),
        (EnemyAttackM, 11),
        (EnemyAttackM, 12),
        (EnemyAttackM, 13),
        (EnemyAttackM, 14),
        (EnemyAttackL, 2),
        (EnemyAttackL, 3),
        (EnemyAttackL, 4),
        (EnemyAttackL, 5),
        (EnemyAttackL, 6),
        (EnemyAttackL, 7),
        (EnemyAttackL, 8),
        (EnemyAttackL, 9),
        (EnemyAttackL, 10),
        (EnemyAttackL, 11),
        (EnemyAttackL, 12),
        (EnemyAttackL, 13),
        (EnemyAttackL, 14),
        (EnemyAttackL, 15),
        (EnemyAttackL, 16),
        (EnemyAttackL, 17),
        (EnemyAttackL, 18),
        (EnemyAttackL, 19),
        (EnemyAttackL, 20),
        (EnemyAttackL, 21),
        (EnemyAttackL, 22),
        (EnemyAttackL, 23),
        (EnemyAttackL, 24),
        (EnemyAttackL, 25),
        (EnemyAttackL, 26),
        (EnemyAttackL, 27),
        (EnemyAttackL, 28),
        (EnemyAttackL, 29),
        (EnemyAttackL, 30),
        (EnemyAttackL, 31),
        (EnemyAttackL, 32),
        (EnemyAttackL, 33),
        (EnemyAttackL, 34),
        (EnemyAttackL, 35),
        (EnemyAttackL, 36),
        (EnemyAttackL, 37),
        (EnemyAttackL, 38),
        (EnemyAttackL, 39),
        (EnemyAttackL, 40),
        (EnemyAttackL, 41),
        (EnemyAttackL, 42),
        (EnemyAttackL, 43),
        (EnemyAttackL, 44),
        (EnemyAttackL, 45),
        (EnemyAttackL, 46),
        (EnemyAttackL, 47),
        (EnemyAttackL, 48),
        (EnemyAttackL, 49),
    ]
}

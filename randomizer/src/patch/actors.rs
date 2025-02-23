use crate::patch::Patcher;
use crate::Result;
use crate::SeedInfo;
use game::Course::{FieldLight, IndoorDark};
use modinfo::Settings;

/// Add Actors to scenes that don't originally have them
pub fn patch(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    patch_dev_stuff(patcher, seed_info)?;

    patch_letter_in_a_bottle(patcher)?;
    patch_bow_of_light_hint(patcher, &seed_info.settings)?;

    Ok(())
}

#[allow(unused_variables)]
fn patch_dev_stuff(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    if !seed_info.settings.dev_mode {
        return Ok(());
    }

    // Add chest actor to Maiamai cave
    // let chest = patcher.scene(game::Course::DungeonHera, 0)?.actors().get_actor_bch("TreasureBoxS")?;
    // patcher.scene(game::Course::CaveLight, 14)?.actors_mut().add(chest)?;

    Ok(())
}

/// Add Hint Ghost to Hilda's Study to give out Bow of Light Hint
fn patch_bow_of_light_hint(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    if !settings.progressive_bow_of_light {
        let hint_ghost = patcher.scene(IndoorDark, 15)?.actors().get_actor_bch("HintGhost")?;
        patcher.scene(IndoorDark, 4)?.actors_mut().add(hint_ghost)?;
    }
    Ok(())
}

/// Add Heart Piece actor to vanilla Letter in a Bottle area
fn patch_letter_in_a_bottle(patcher: &mut Patcher) -> Result<()> {
    let heart_piece = patcher.scene(FieldLight, 29)?.actors().get_actor_bch("HeartPiece")?;
    patcher.scene(FieldLight, 35)?.actors_mut().add(heart_piece)?;
    Ok(())
}

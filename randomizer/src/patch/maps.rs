use albw::{
    course::{Id, Id::*},
    scene::{Flag, Icn, IcnArgs, StageMeta},
    Item,
};
use log::info;

use crate::{
    patch::{util::*, DungeonPrizes},
    Patcher, Result,
};

#[non_exhaustive]
struct Icon;

#[allow(unused)]
impl Icon {
    pub const TREASURE_BOX: i32 = 0; // Chest
    pub const KEY: i32 = 1;
    pub const LOCKED: i32 = 2;
    pub const BOSS_DOOR: i32 = 3;
    pub const STAIR_UP: i32 = 4;
    pub const STAIR_DOWN: i32 = 5;
    // pub const SIX: i32 = 6; // RaceGoal ?
    pub const ENTRANCE: i32 = 7;
    pub const WARP_POINT: i32 = 8;
    pub const PROPELLER: i32 = 9; // Gales
    pub const VALVE: i32 = 10; // Swamp
                               // pub const ELEVEN: i32 = 11; // ?
                               // pub const TWELVE: i32 = 12; // ?
                               // pub const THIRTEEN: i32 = 13; // ?
                               // pub const FOURTEEN: i32 = 14; // ?
                               // pub const FIFTEEN: i32 = 15; // ?
                               // pub const SIXTEEN: i32 = 16; // ?
                               // pub const SEVENTEEN: i32 = 17; // ?
                               // pub const EIGHTEEN: i32 = 18; // ?
                               // pub const NINETEEN: i32 = 19; // ?
    pub const DESTINATION: i32 = 20; // Red X
    pub const SAVE_POINT: i32 = 21; // Weather Vane
    pub const PORTAL: i32 = 22;
}

pub fn patch_maps(patcher: &mut Patcher, prizes: &DungeonPrizes) -> Result<()> {
    info!("Patching Maps...");
    patch_hyrule_maps(patcher, prizes)?;
    patch_lorule_maps(patcher, prizes)?;
    add_compass_chests(patcher);
    Ok(())
}

fn patch_hyrule_maps(patcher: &mut Patcher, prizes: &DungeonPrizes) -> Result<()> {
    const HYRULE_MAPS: [Id; 4] = [AttractionLight, CaveLight, FieldLight, IndoorLight];

    for map in HYRULE_MAPS {
        let stage_meta = patcher.scene_meta(map).stage_meta_mut().get_mut();

        disable_icn(stage_meta, 10); // Sanctuary
        disable_icn(stage_meta, 17); // Sahasrahla
        disable_icn(stage_meta, 22); // Blacksmith

        mark_by_prize(stage_meta, prizes.ep_prize, 21)?; // Eastern
        mark_by_prize(stage_meta, prizes.hg_prize, 39)?; // Gales
        mark_by_prize(stage_meta, prizes.th_prize, 6)?; // Hera
        mark_by_prize(stage_meta, prizes.hc_prize, 19)?; // Hyrule Castle
    }
    Ok(())
}

fn patch_lorule_maps(patcher: &mut Patcher, prizes: &DungeonPrizes) -> Result<()> {
    const LORULE_MAPS: [Id; 7] =
        [AttractionDark, CaveDark, EnemyAttackS, EnemyAttackM, EnemyAttackL, FieldDark, IndoorDark];

    for map in LORULE_MAPS {
        let stage_meta = patcher.scene_meta(map).stage_meta_mut().get_mut();

        mark_by_prize(stage_meta, prizes.pd_prize, 22)?; // Dark
        mark_by_prize(stage_meta, prizes.sp_prize, 40)?; // Swamp
        mark_by_prize(stage_meta, prizes.sw_prize, 1)?; // Skull
        mark_by_prize(stage_meta, prizes.tt_prize, 16)?; // Thieves'
        mark_by_prize(stage_meta, prizes.tr_prize, 43)?; // Turtle
        mark_by_prize(stage_meta, prizes.dp_prize, 30)?; // Desert
        mark_by_prize(stage_meta, prizes.ir_prize, 10)?; // Ice

        // Lorule Castle
        let icn = stage_meta.icn.get_mut(20).unwrap();
        icn.enable_on(Flag::Event(670));
        icn.clear_disabled();
    }
    Ok(())
}

fn mark_by_prize(stage_meta: &mut StageMeta, prize: Item, icn_index: usize) -> Result<()> {
    let icn = stage_meta.icn.get_mut(icn_index).unwrap();
    if is_sage(prize) {
        icn.enable();
        icn.disable_on(prize_flag(prize)?);
    } else {
        icn.disable();
    }
    Ok(())
}

fn disable_icn(stage_meta: &mut StageMeta, icn_index: usize) {
    stage_meta.icn.get_mut(icn_index).unwrap().disable();
}

/// Add icons for dungeon chests that contained the compass in vanilla
fn add_compass_chests(patcher: &mut Patcher) {
    // Eastern
    patcher.scene_meta(DungeonEast).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 2),
        pos: vec![-22.0, 2.5, -47.0],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Gales
    patcher.scene_meta(DungeonWind).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 3),
        pos: vec![-19.0, 2.5, -23.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Hera
    patcher.scene_meta(DungeonHera).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 1),
        pos: vec![0.0, 5.0, -1.9],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Dark
    patcher.scene_meta(DungeonDark).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 1, 0, 0, 3, 0, 43),
        pos: vec![-24.5, 2.5, -47.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Swamp
    patcher.scene_meta(DungeonWater).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -1, 1, 0, 0, 3, 0, 64),
        pos: vec![0.0, 0.0, -48.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Skull
    patcher.scene_meta(DungeonDokuro).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -1, 0, 0, 0, 3, 0, 3),
        pos: vec![18.0, 0.0, -58.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Thieves'
    patcher.scene_meta(DungeonHagure).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -2, 0, 0, 0, 3, 0, 106),
        pos: vec![6.5, 0.0, -31.25],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Turtle
    patcher.scene_meta(DungeonKame).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 5),
        pos: vec![0.0, 5.0, -33.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Desert
    patcher.scene_meta(DungeonSand).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 1),
        pos: vec![-22.0, 5.0, -67.0],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Ice
    patcher.scene_meta(DungeonIce).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -1, 0, 0, 0, 3, 0, 17),
        pos: vec![23.0, 75.0, -2.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Lorule Castle
    patcher.scene_meta(DungeonGanon).stage_meta_mut().get_mut().icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 4, 0, 0, 0, 3, 0, 215),
        pos: vec![0.0, 40.0, -2.77344],
        scr: vec![0.0, 0.0],
        msg: None,
    });
}

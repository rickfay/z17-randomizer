use crate::filler::filler_item::Randomizable;
use crate::{
    patch::{util::*, DungeonPrizes},
    Patcher, SeedInfo,
};
use game::Course::{self, *};
use log::info;
use modinfo::settings::keysy::Keysy;
use rom::flag::Flag;
use rom::scene::{Icn, IcnArgs, StageMeta};

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
    pub const CRACK: i32 = 22;
}

pub fn patch(patcher: &mut Patcher, prizes: &DungeonPrizes, seed_info: &SeedInfo) {
    info!("Patching Course BYAML...");
    patch_hyrule_maps(patcher, prizes);
    patch_lorule_maps(patcher, prizes);
    patch_eastern_maps(patcher, seed_info);
    patch_gales_maps(patcher, seed_info);
    patch_hera_maps(patcher, seed_info);
    patch_hyrule_castle_maps(patcher);
    patch_dark_maps(patcher, seed_info);
    patch_swamp_maps(patcher, seed_info);
    patch_skull_maps(patcher, seed_info);
    patch_thieves_maps(patcher, seed_info);
    patch_turtle_maps(patcher, seed_info);
    patch_desert_maps(patcher, seed_info);
    patch_ice_maps(patcher, seed_info);
    patch_lorule_castle_maps(patcher, seed_info);
}

/// Hyrule Field Maps
fn patch_hyrule_maps(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    const HYRULE_MAPS: [Course; 4] = [AttractionLight, CaveLight, FieldLight, IndoorLight];

    for map in HYRULE_MAPS {
        let stage_meta = patcher.scene_meta(map).stage_meta_mut().get_mut();

        disable_icn(stage_meta, 10); // Sanctuary
        disable_icn(stage_meta, 17); // Sahasrahla
        disable_icn(stage_meta, 22); // Blacksmith
        disable_icn(stage_meta, 19); // Hyrule Castle

        mark_by_prize(stage_meta, prizes.ep_prize, 21); // Eastern
        mark_by_prize(stage_meta, prizes.hg_prize, 39); // Gales
        mark_by_prize(stage_meta, prizes.th_prize, 6); // Hera
    }
}

/// Lorule Field Maps
fn patch_lorule_maps(patcher: &mut Patcher, prizes: &DungeonPrizes) {
    const LORULE_MAPS: [Course; 7] =
        [AttractionDark, CaveDark, EnemyAttackS, EnemyAttackM, EnemyAttackL, FieldDark, IndoorDark];

    for map in LORULE_MAPS {
        let stage_meta = patcher.scene_meta(map).stage_meta_mut().get_mut();

        mark_by_prize(stage_meta, prizes.pd_prize, 22); // Dark
        mark_by_prize(stage_meta, prizes.sp_prize, 40); // Swamp
        mark_by_prize(stage_meta, prizes.sw_prize, 1); // Skull
        mark_by_prize(stage_meta, prizes.tt_prize, 16); // Thieves'
        mark_by_prize(stage_meta, prizes.tr_prize, 43); // Turtle
        mark_by_prize(stage_meta, prizes.dp_prize, 30); // Desert
        mark_by_prize(stage_meta, prizes.ir_prize, 10); // Ice

        // Lorule Castle
        let icn = stage_meta.icn.get_mut(20).unwrap();
        icn.enable_on(Flag::Event(670));
        icn.clear_disabled();
    }
}

fn mark_by_prize(stage_meta: &mut StageMeta, prize: Randomizable, icn_index: usize) {
    let icn = stage_meta.icn.get_mut(icn_index).unwrap();
    if is_sage(prize) {
        icn.enable();
        icn.disable_on(prize_flag(prize));
    } else {
        icn.disable();
    }
}

fn disable_icn(stage_meta: &mut StageMeta, icn_index: usize) {
    stage_meta.icn.get_mut(icn_index).unwrap().disable();
}

fn handle_small_keysy_dungeon_icons(stage_meta: &mut StageMeta, seed_info: &SeedInfo) {
    match seed_info.settings.keysy {
        Keysy::SmallKeysy | Keysy::AllKeysy => stage_meta.icn.retain(|icn| icn.arg.0 != Icon::LOCKED),
        _ => {},
    };
}

fn handle_big_keysy_dungeon_icons(stage_meta: &mut StageMeta, seed_info: &SeedInfo) {
    match seed_info.settings.keysy {
        Keysy::BigKeysy | Keysy::AllKeysy => stage_meta.icn.retain(|icn| icn.arg.0 != Icon::BOSS_DOOR),
        _ => {},
    };
}

/// Eastern Palace Maps
fn patch_eastern_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let eastern_meta = patcher.scene_meta(DungeonEast).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    eastern_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 2),
        pos: vec![-22.0, 2.5, -47.0],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    // Make post-boss chest icons visible without defeating boss
    eastern_meta.icn.get_mut(5).unwrap().clear_enabled(); // 1F Escape Chest
    eastern_meta.icn.get_mut(6).unwrap().clear_enabled(); // 1F Merge Chest
    eastern_meta.icn.get_mut(17).unwrap().clear_enabled(); // 3F Escape Chest

    handle_small_keysy_dungeon_icons(eastern_meta, seed_info);
    handle_big_keysy_dungeon_icons(eastern_meta, seed_info);
}

/// House of Gales Maps
fn patch_gales_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let gales_meta = patcher.scene_meta(DungeonWind).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    gales_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 3),
        pos: vec![-19.0, 2.5, -23.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(gales_meta, seed_info);
    handle_big_keysy_dungeon_icons(gales_meta, seed_info);
}

/// Tower of Hera Maps
fn patch_hera_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let hera_meta = patcher.scene_meta(DungeonHera).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    hera_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 1),
        pos: vec![0.0, 5.0, -1.9],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(hera_meta, seed_info);
    handle_big_keysy_dungeon_icons(hera_meta, seed_info);
}

/// Hyrule Castle Maps
fn patch_hyrule_castle_maps(patcher: &mut Patcher) {
    let hc_meta = patcher.scene_meta(DungeonCastle).stage_meta_mut().get_mut();

    // Use the course flag to control the green Warp icons, not Flag 510.
    let warp_2f = hc_meta.icn.get_mut(1).unwrap();
    warp_2f.enable_on(Flag::Course(31));

    let warp_7f = hc_meta.icn.get_mut(11).unwrap();
    warp_7f.enable_on(Flag::Course(31));
}

/// Dark Palace Maps
fn patch_dark_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let dark_meta = patcher.scene_meta(DungeonDark).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    dark_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 1, 0, 0, 3, 0, 43),
        pos: vec![-24.5, 2.5, -47.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(dark_meta, seed_info);
    handle_big_keysy_dungeon_icons(dark_meta, seed_info);
}

/// Swamp Palace Maps
fn patch_swamp_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let swamp_meta = patcher.scene_meta(DungeonWater).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    swamp_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -1, 1, 0, 0, 3, 0, 64),
        pos: vec![0.0, 0.0, -48.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(swamp_meta, seed_info);
    handle_big_keysy_dungeon_icons(swamp_meta, seed_info);
}

/// Skull Woods Maps
fn patch_skull_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let skull_meta = patcher.scene_meta(DungeonDokuro).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    skull_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -1, 0, 0, 0, 3, 0, 3),
        pos: vec![18.0, 0.0, -58.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(skull_meta, seed_info);
    handle_big_keysy_dungeon_icons(skull_meta, seed_info);
}

/// Thieves' Hideout Maps
fn patch_thieves_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let thieves_meta = patcher.scene_meta(DungeonHagure).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    thieves_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -2, 0, 0, 0, 3, 0, 106),
        pos: vec![6.5, 0.0, -31.25],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(thieves_meta, seed_info);
    handle_big_keysy_dungeon_icons(thieves_meta, seed_info);
}

/// Turtle Rock Maps
fn patch_turtle_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let turtle_meta = patcher.scene_meta(DungeonKame).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    turtle_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 5),
        pos: vec![0.0, 5.0, -33.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(turtle_meta, seed_info);
    handle_big_keysy_dungeon_icons(turtle_meta, seed_info);
}

/// Desert Palace Maps
fn patch_desert_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let desert_meta = patcher.scene_meta(DungeonSand).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    desert_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 1, 0, 0, 0, 3, 0, 1),
        pos: vec![-22.0, 5.0, -67.0],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(desert_meta, seed_info);
    handle_big_keysy_dungeon_icons(desert_meta, seed_info);
}

/// Ice Ruins Maps
fn patch_ice_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let ice_meta = patcher.scene_meta(DungeonIce).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    ice_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, -1, 0, 0, 0, 3, 0, 17),
        pos: vec![23.0, 75.0, -2.5],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(ice_meta, seed_info);
    handle_big_keysy_dungeon_icons(ice_meta, seed_info);
}

/// Lorule Castle Maps
fn patch_lorule_castle_maps(patcher: &mut Patcher, seed_info: &SeedInfo) {
    let lc_meta = patcher.scene_meta(DungeonGanon).stage_meta_mut().get_mut();

    // Add vanilla compass chest icon
    lc_meta.icn.push(Icn {
        arg: IcnArgs(Icon::TREASURE_BOX, 4, 0, 0, 0, 3, 0, 215),
        pos: vec![0.0, 40.0, -2.77344],
        scr: vec![0.0, 0.0],
        msg: None,
    });

    handle_small_keysy_dungeon_icons(lc_meta, seed_info);
}

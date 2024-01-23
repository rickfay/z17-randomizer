use crate::filler::filler_item::FillerItem::Item;
use crate::filler::filler_item::Item::{PendantOfPower, PendantOfWisdom};
use crate::{
    hints::{formatting::*, Hint},
    patch::messages::{hint_ghosts::HintGhost, msbt::load_msbt},
    LocationInfo, Patcher, Result, SeedInfo,
};
use game::Course::{self, *};
use log::info;
use std::collections::btree_map::BTreeMap;

mod hint_ghosts;
mod msbt;

/// Patch MSBT Message Files
pub fn patch_messages(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    info!("Patching MSBT Files...");

    patch_file_select(patcher, seed_info)?;
    // patch_pause_screen(patcher)?; TODO
    patch_item_names(patcher)?;
    patch_actions(patcher)?;
    // patch_ravio(patcher)?;
    patch_great_rupee_fairy(patcher)?;
    patch_thief_girl(patcher)?;

    patch_street_merchant(patcher, seed_info)?;
    patch_sahasrahla(patcher, seed_info)?;
    patch_general_hint_ghosts(patcher, seed_info)?;
    patch_hint_ghosts(patcher, seed_info)?;
    patch_bow_of_light(patcher, seed_info)?;

    Ok(())
}

/// Prints out all String Values and their indexed Label Keys for a given MSBT File
pub fn research(patcher: &mut Patcher, course: Course, file: &str, edotor: bool) -> Vec<(String, String)> {
    load_msbt(patcher, course, file).unwrap().research(edotor)
}

fn patch_file_select(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let mut file_select_b = load_msbt(patcher, LanguageBoot, "Mn_FileSelectB").unwrap();
    file_select_b.set("HeadLineText_00", &seed_info.hash.item_hash);
    file_select_b.set("HeadLineText_01", &seed_info.hash.item_hash);
    file_select_b.set("HeadLineText_10", &seed_info.hash.item_hash);
    patcher.update(file_select_b.dump())?;

    // let mut file_select_t = load_msbt(patcher, LanguageBoot, "Mn_FileSelectT").unwrap();
    // file_select_t.set("T_FileNumber_00", format!("Hash: {:0>5}", seed_info.hash));
    // file_select_t.set("T_FileNumber_Hard_00", format!("Hash: {:0>5}", seed_info.hash));
    // patcher.update(file_select_t.dump())?;

    Ok(())
}

/// Pause Screen
#[allow(unused)]
fn patch_pause_screen(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, LanguageBoot, "Gm_PauseT").unwrap();
    msbt.set("L_Btn_01_T_GmOvr_00", "Warp");
    msbt.set("T_Message_00", "Warp to Link's House?");
    patcher.update(msbt.dump())?;

    Ok(())
}

/// Item Names
fn patch_item_names(patcher: &mut Patcher) -> Result<()> {
    // Item names in textboxes
    let mut item_name = load_msbt(patcher, LanguageBoot, "ItemName").unwrap();

    // Repurpose unused strings as Rupee names so they show up in the shop
    // FIXME - This is hacky, add these as new strings when size problem is fixed
    item_name.set("item_name_tornaderod_rental", "Green Rupee");
    item_name.set("item_name_icerod_rental", "Blue Rupee");
    item_name.set("item_name_bfirerod_rental", "Red Rupee");
    item_name.set("item_name_boomerang_rental", "Purple Rupee");
    item_name.set("item_name_hookshot_rental", "Silver Rupee");
    item_name.set("item_name_sandrod_rental", "Gold Rupee");

    // Earthquake - Repurpose Triforce of Courage item
    // item_name.set("item_name_triforce_courage", "Quake Medallion");

    patcher.update(item_name.dump())?;

    // Item descriptions when picked up
    // let mut event_item_get = load_msbt(patcher, LanguageBoot, "EventItemGet").unwrap();
    // event_item_get.set("triforce_courage", &format!("a thing happened")); // Earthquake
    // patcher.update(event_item_get.dump())?;

    // Item names in textboxes
    // let mut item_name_upper = load_msbt(patcher, LanguageBoot, "ItemNameUpper").unwrap();
    // item_name_upper.set("item_name_triforce_courage", "Earthquake");
    // patcher.update(item_name_upper.dump())?;

    Ok(())
}

/// Action icon text
fn patch_actions(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, LanguageBoot, "Action").unwrap();
    msbt.set("cmn_action_throw", "Yeet");
    patcher.update(msbt.dump())?;

    Ok(())
}

#[allow(unused)]
fn patch_ravio(patcher: &mut Patcher) -> Result<()> {
    let mut ravio_shop = load_msbt(patcher, IndoorLight, "FieldLight_2C").unwrap();

    ravio_shop.set(
        "lgt_NpcRental_08",
        &format!(
            "Huh? Not interested?\nIf you don't have enough rupees, I'll\ngive you your first item {}.",
            name("for free")
        ),
    );
    ravio_shop.set("lgt_RentalKeeper_Field_2C_03", "stuff and things");
    patcher.update(ravio_shop.dump())?;

    Ok(())
}

fn patch_great_rupee_fairy(patcher: &mut Patcher) -> Result<()> {
    let mut grf = load_msbt(patcher, CaveDark, "Cave").unwrap();
    grf.set("CaveDark29_LuckyFairy_00", &format!("Throw Rupees into the fountain?\n{}", *CHOICE_2));
    grf.set("CaveDark29_LuckyFairy_01", "Throw 3000");
    grf.set("CaveDark29_LuckyFairy_02", "Don't throw any");
    grf.set("CaveDark29_LuckyFairy_03", "1234567"); // shorten string so file matches OG size FIXME
    patcher.update(grf.dump())?;

    Ok(())
}

/// Thief Girl
fn patch_thief_girl(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, DungeonHagure, "Hagure").unwrap();

    // Shorten initial Thief Girl text to just the last textbox.
    msbt.set("Hagure_girl_03", "Come on. Let's hurry out of here. This\nplace gives me the chills.");
    patcher.update(msbt.dump())?;

    Ok(())
}

/// Street Merchant - Shorten text & show the item names
fn patch_street_merchant(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let item_left = seed_info
        .layout
        .get(&LocationInfo::new("Street Merchant (Left)", crate::regions::hyrule::kakariko::village::SUBREGION))
        .unwrap()
        .as_str();
    let item_right = seed_info
        .layout
        .get(&LocationInfo::new("Street Merchant (Right)", crate::regions::hyrule::kakariko::village::SUBREGION))
        .unwrap()
        .as_str();

    let mut street_merchant = load_msbt(patcher, FieldLight, "FieldLight_18").unwrap();
    street_merchant.set(
        "lgt_NpcStand_BottleEmpty_00_select",
        &format!("That's a {}.\nUseful for a bunch of things.\nHow about {}?{}", name(item_left), *PRICE, *CHOICE_2),
    );

    street_merchant.set(
        "lgt_NpcStand_ZoraTreasure_00_select",
        &format!(
            "Ah, yes! A {}\nof remarkable quality. Smooth as silk!\nAnd for you? Only {}!{}",
            name(item_right),
            *PRICE,
            *CHOICE_2
        ),
    );
    street_merchant.set(
        "lgt_NpcStand_ZoraTreasure_01",
        &format!(
            "Sorry to see it go, actually. I just\ncouldn't stop touching that\nsmooth, smooth {}.",
            name(item_right)
        ),
    );

    patcher.update(street_merchant.dump())?;

    Ok(())
}

/// Sahasrahla gives out the locations of the Red & Blue Pendants
#[allow(unused)]
fn patch_sahasrahla(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let (pow_region, _) = seed_info.layout.find_single(Item(PendantOfWisdom)).unwrap();
    let (pop_region, _) = seed_info.layout.find_single(Item(PendantOfPower)).unwrap();

    let mut sahasrahla = load_msbt(patcher, FieldLight, "FieldLight_1B")?;
    sahasrahla.set(
        "lgt_NpcSahasrahla_Field1B_08",
        &format!(
            "The {} has been\nenshrined in the {}.\n\nAnd the {}, in the\n{}.",
            name("Pendant of Wisdom"),
            name(pow_region),
            attention("Pendant of Power"),
            attention(pop_region)
        ),
    );

    patcher.update(sahasrahla.dump())?;

    Ok(())
}

fn patch_general_hint_ghosts(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let price = seed_info.settings.hint_ghost_price;

    let mut hint_ghost = load_msbt(patcher, LanguageBoot, "HintGhost")?;
    hint_ghost.set(
        "HintGhost_02_select",
        &format!(
            "Buy a {} for {}?{}",
            blue("Ghost Hint"),
            attention(&format!("{} {}", price, if price == 1 { "Rupee" } else { "Rupees" })),
            *CHOICE_2
        ),
    );
    hint_ghost.set("HintGhost_02_select_00", "Buy");
    hint_ghost.set("HintGhost_07", &format!("Looks like you don't have enough\n{}!", attention("Rupees")));
    patcher.update(hint_ghost.dump())?;
    Ok(())
}

const EMPTY_MSG: &str = "\0\0";

fn patch_hint_ghosts(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    if seed_info.hints.always_hints.is_empty() {
        info!("No Ghost Hints generated.");
        return Ok(());
    } else {
        info!("Patching Hint Ghosts...");
    }

    // Organize Hints by the MSBT File they need to update
    let mut msbt_hint_map = BTreeMap::new();

    // Path Hints
    for path_hint in &seed_info.hints.path_hints {
        // Make mutable copy of hint for processing
        let path_hint = &mut path_hint.clone();

        for ghost in &path_hint.ghosts {
            let hint_ghost = HintGhost::from(*ghost);
            let entry = msbt_hint_map.entry((hint_ghost.course, hint_ghost.msbt_file)).or_insert_with(BTreeMap::new);
            entry.insert(hint_ghost.msg_label, path_hint.get_hint());
        }
    }

    // Always Hints
    for always_hint in &seed_info.hints.always_hints {
        // Make mutable copy of hint for processing
        let always_hint = &mut always_hint.clone();

        for ghost in &always_hint.ghosts {
            let hint_ghost = HintGhost::from(*ghost);
            let entry = msbt_hint_map.entry((hint_ghost.course, hint_ghost.msbt_file)).or_insert_with(BTreeMap::new);
            entry.insert(hint_ghost.msg_label, always_hint.get_hint());
        }
    }

    // Sometimes Hints
    for sometimes_hint in &seed_info.hints.sometimes_hints {
        // Make mutable copy of hint for processing
        let sometimes_hint = &mut sometimes_hint.clone();

        for ghost in &sometimes_hint.ghosts {
            let hint_ghost = HintGhost::from(*ghost);
            let entry = msbt_hint_map.entry((hint_ghost.course, hint_ghost.msbt_file)).or_insert_with(BTreeMap::new);
            entry.insert(hint_ghost.msg_label, sometimes_hint.get_hint());
        }
    }

    // FIXME extremely dumb. Clear out some unused messages in Lost Woods to keep file size down.
    msbt_hint_map.get_mut(&(FieldLight, "FieldLight_00")).unwrap().extend(BTreeMap::from([
        ("lgt_MayoinoHintObake_Msg3", EMPTY_MSG.to_owned()),
        ("lgt_MayoinoHintObake_Msg5", EMPTY_MSG.to_owned()),
        ("lgt_MayoinoHintObake_Msg7", EMPTY_MSG.to_owned()),
        ("lgt_MayoinoHintObake_Msg9", EMPTY_MSG.to_owned()),
    ]));

    // Update the MSBT Files with the generated Hints
    for ((course, msbt_file), labels) in msbt_hint_map {
        let mut msbt_file = load_msbt(patcher, course, msbt_file)?;

        let mut og_text_size = 0;
        let mut hint_text_size = 0;

        for (label, hint) in labels {
            og_text_size += msbt_file.get(label).unwrap().len();
            hint_text_size += hint.len();
            msbt_file.set(label, &hint);
        }

        // fixme band-aid fix to verify we haven't bloated the hint text to the point where the game crashes
        if hint_text_size > og_text_size {
            return Err(crate::Error::io("Generated Hint text was too long."));
        }

        patcher.update(msbt_file.dump())?;
    }

    Ok(())
}

fn patch_bow_of_light(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    if let Some(bow_of_light_hint) = seed_info.hints.bow_of_light_hint.as_ref() {
        let mut msbt = load_msbt(patcher, IndoorDark, "HintGhostDark")?;
        // Most of HintGhostDark.msbt is a duplicate of the identical file under FieldDark, but it's not used. Choosing
        // an easily testable ghost Key to repurpose for a new Ghost in Hilda's Study.
        msbt.set("HintGhost_FieldDark_2C_014", &bow_of_light_hint.get_hint());
        // fixme also dumb: clear out unused messages to keep filesize down.
        msbt.set("HintGhost_FieldDark_02_001", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_03_002", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_07_003", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_14_004", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_16_005", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_18_006", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_1A_009", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_1E_010", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_28_011", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_29_012", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_2A_013", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_30_015", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_33_016", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_35_017", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_35_018", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_35_019", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_1E_020", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_33_021", EMPTY_MSG);
        msbt.set("HintGhost_FieldDark_33_022", EMPTY_MSG);
        patcher.update(msbt.dump())?;
    }

    Ok(())
}

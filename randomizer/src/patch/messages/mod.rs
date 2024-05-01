use crate::filler::filler_item::Item::{
    PendantOfCourage, PendantOfPower, PendantOfWisdom, SageGulley, SageImpa, SageIrene, SageOren, SageOsfala,
    SageRosso, SageSeres,
};
use crate::{
    hints::{formatting::*, Hint},
    patch::messages::{hint_ghosts::HintGhost, msbt::load_msbt},
    regions, Patcher, Result, SeedInfo,
};
use game::Course::{self, *};
use log::info;
use std::collections::btree_map::BTreeMap;

mod hint_ghosts;
mod msbt;

/// Patch MSBT Message Files
pub fn patch_messages(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    info!("Patching MSBT Files...");

    // patch_flavor_text(patcher, seed_info)?;
    patch_file_select(patcher, seed_info)?;
    // patch_pause_screen(patcher)?; TODO
    patch_item_names(patcher)?;
    patch_event_item_get(patcher)?;
    patch_collect(patcher, seed_info)?;
    // patch_actions(patcher)?; This is causing issues for some reason
    patch_ravio(patcher, seed_info)?;
    patch_impa(patcher)?;
    patch_great_rupee_fairy(patcher)?;
    patch_treacherous_tower(patcher, seed_info)?;
    patch_thief_girl(patcher)?;
    patch_cross_old_man(patcher)?;

    patch_street_merchant(patcher, seed_info)?;
    patch_sahasrahla(patcher, seed_info)?;
    patch_hint_ghosts(patcher, seed_info)?;
    // patch_mother_maiamai_sign(patcher, seed_info)?;
    patch_bow_of_light(patcher, seed_info)?;

    Ok(())
}

/// Prints out all String Values and their indexed Label Keys for a given MSBT File
pub fn research(patcher: &mut Patcher, course: Course, file: &str, edotor: bool) -> Vec<(String, String)> {
    load_msbt(patcher, course, file).unwrap().research(edotor)
}

/// Flavor Text
#[allow(unused)]
fn patch_flavor_text(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let mut msbt = load_msbt(patcher, LanguageBoot, "Ed_StaffCreditMessageT")?;
    msbt.set("T_Text_00", &seed_info.text.credits);
    patcher.update(msbt.dump())?;

    Ok(())
}

/// File Select Screen
fn patch_file_select(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let mut file_select_b = load_msbt(patcher, LanguageBoot, "Mn_FileSelectB")?;
    file_select_b.set("HeadLineText_00", &seed_info.hash.item_hash);
    file_select_b.set("HeadLineText_01", &seed_info.hash.item_hash);
    file_select_b.set("HeadLineText_10", &seed_info.hash.item_hash);
    patcher.update(file_select_b.dump())?;

    // let mut file_select_t = load_msbt(patcher, LanguageBoot, "Mn_FileSelectT")?;
    // file_select_t.set("T_FileNumber_00", format!("Hash: {:0>5}", seed_info.hash));
    // file_select_t.set("T_FileNumber_Hard_00", format!("Hash: {:0>5}", seed_info.hash));
    // patcher.update(file_select_t.dump())?;

    Ok(())
}

/// Pause Screen
#[allow(unused)]
fn patch_pause_screen(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, LanguageBoot, "Gm_PauseT")?;
    msbt.set("L_Btn_01_T_GmOvr_00", "Warp");
    msbt.set("T_Message_00", "Warp to Link's House?");
    patcher.update(msbt.dump())?;

    Ok(())
}

/// Item Names
fn patch_item_names(patcher: &mut Patcher) -> Result<()> {
    // Item names in textboxes
    let mut item_name = load_msbt(patcher, LanguageBoot, "ItemName")?;

    // Repurpose unused strings as Rupee names so they show up in the shop
    // FIXME - This is hacky, add these as new strings when size problem is fixed
    item_name.set("item_name_tornaderod_rental", "Green Rupee");
    item_name.set("item_name_icerod_rental", "Blue Rupee");
    item_name.set("item_name_bfirerod_rental", "Red Rupee");
    item_name.set("item_name_boomerang_rental", "Purple Rupee");
    item_name.set("item_name_hookshot_rental", "Silver Rupee");
    item_name.set("item_name_sandrod_rental", "Gold Rupee");

    // Quake - Repurpose
    item_name.set("item_name_gamecoin", "Quake");

    patcher.update(item_name.dump())?;

    // Item descriptions when picked up
    // let mut event_item_get = load_msbt(patcher, LanguageBoot, "EventItemGet")?;
    // event_item_get.set("triforce_courage", &format!("a thing happened")); // Earthquake
    // patcher.update(event_item_get.dump())?;

    // Item names in textboxes
    // let mut item_name_upper = load_msbt(patcher, LanguageBoot, "ItemNameUpper")?;
    // item_name_upper.set("item_name_triforce_courage", "Earthquake");
    // patcher.update(item_name_upper.dump())?;

    Ok(())
}

/// Item Descriptions
fn patch_event_item_get(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, LanguageBoot, "EventItemGet")?;

    msbt.set("none", "A quake shakes the kingdom!"); // ehh

    msbt.set("item_bow", "You got the bow!");
    msbt.set("item_boomerang", "You got the boomerang!");
    msbt.set("item_hookshot", "You got the Hookshot!");
    msbt.set("item_hammer", "You got the hammer!");
    msbt.set("item_bomb", &format!("You got {}!", *BOMBS));
    msbt.set("item_firerod", "You got the Fire Rod!");
    msbt.set("item_icerod", "You got the Ice Rod!");
    msbt.set("item_tornaderod", "You got the Tornado Rod!");
    msbt.set("item_sandrod", "You got the Sand Rod!");

    msbt.set("kandelaar", "You got the lamp!");
    msbt.set("zelda_amulet", "You got a special charm!"); // Cut " from Princess Zelda"

    patcher.update(msbt.dump())?;

    Ok(())
}

/// Gear Descriptions
fn patch_collect(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let power = seed_info.layout.find_single(PendantOfPower).unwrap().0;
    let wisdom = seed_info.layout.find_single(PendantOfWisdom).unwrap().0;
    let courage = seed_info.layout.find_single(PendantOfCourage).unwrap().0;

    let mut msbt = load_msbt(patcher, LanguageBoot, "Collect")?;

    msbt.set("cl_instruction_power", &format!("A pendant you found at the\n{}.", power));
    msbt.set("cl_instruction_wisdom", &format!("A pendant you found at the\n{}.", wisdom));
    msbt.set("cl_instruction_courage", &format!("A pendant you found at the\n{}.", courage));

    patcher.update(msbt.dump())?;

    Ok(())
}

/// Action icon text
#[allow(unused)]
fn patch_actions(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, LanguageBoot, "Action")?;
    msbt.set("cmn_action_throw", "Yeet");
    patcher.update(msbt.dump())?;

    Ok(())
}

/// Ravio
fn patch_ravio(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let (gulley, _) = seed_info.layout.find_single(SageGulley).unwrap();
    let (oren, _) = seed_info.layout.find_single(SageOren).unwrap();
    let (seres, _) = seed_info.layout.find_single(SageSeres).unwrap();
    let (osfala, _) = seed_info.layout.find_single(SageOsfala).unwrap();
    let (impa, _) = seed_info.layout.find_single(SageImpa).unwrap();
    let (irene, _) = seed_info.layout.find_single(SageIrene).unwrap();
    let (rosso, _) = seed_info.layout.find_single(SageRosso).unwrap();

    let first_intro = &format!("What's that? You're looking for the\n{}?", name("Seven Sages"));
    let second_intro = &format!("Yeah...if you're looking for those\n{}?", name("Seven Sages"));

    let gulley = &format!("\n{} is in the {}.", green("Gulley"), green(gulley));
    let oren = &format!("\n{} is in the {}.", beige("Queen Oren"), beige(oren));
    let seres = &format!("\n{} is in the {}.", blue("Seres"), blue(seres));
    let osfala = &format!("\n{} is in the {}.", beige("Osfala"), beige(osfala));
    let impa = &format!("\n{} is in the {}.", purple("Lady Impa"), purple(impa));
    let irene = &format!("\n{} is in the {}.", name("Irene"), name(irene));
    let rosso = &format!("\n{} is in the {}.", attention("Rosso"), attention(rosso));

    let mut ravio_shop = load_msbt(patcher, IndoorLight, "FieldLight_2C")?;

    ravio_shop.set("lgt_RentalKeeper_Field_2C_03", first_intro);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_04", second_intro);

    ravio_shop.set("lgt_RentalKeeper_Field_2C_06", gulley);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_00", oren);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_01", seres);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_07", osfala);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_09", impa);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_05", irene);
    ravio_shop.set("lgt_RentalKeeper_Field_2C_08", rosso);

    patcher.update(ravio_shop.dump())?;

    Ok(())
}

/// Impa in Hyrule Castle dialogue
fn patch_impa(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, IndoorLight, "FieldLight_1B")?;
    msbt.set(
        "FieldLight_1B_Impa_ACT3_10",
        &format!(
            "The princess left you that {}\nbecause she sensed something in\nyou, {}.\nDon't let her down.",
            name("chest"),
            *PLAYER_NAME
        ),
    );
    msbt.set("FieldLight_1B_Soldier_ACT2_19", "So was Lady Impa looking as\nlovely as usual today?");
    patcher.update(msbt.dump())?;

    Ok(())
}

fn patch_great_rupee_fairy(patcher: &mut Patcher) -> Result<()> {
    let mut grf = load_msbt(patcher, CaveDark, "Cave")?;
    grf.set("CaveDark29_LuckyFairy_00", &format!("Throw Rupees into the fountain?\n{}", *CHOICE_2));
    grf.set("CaveDark29_LuckyFairy_01", "Throw 3000");
    grf.set("CaveDark29_LuckyFairy_02", "Don't throw any");
    grf.clear("CaveDark29_LuckyFairy_03");
    patcher.update(grf.dump())?;

    Ok(())
}

/// Treacherous Tower
fn patch_treacherous_tower(patcher: &mut Patcher, SeedInfo { settings, .. }: &SeedInfo) -> Result<()> {
    let mut msbt = load_msbt(patcher, FieldDark, "FieldDark_05")?;

    msbt.set(
        "fd_GameTower_expert_select",
        &format!(
            "Sword boy is gonna be the next\ncontestant on the Random course,\nright?! Ya ready for this?{}",
            *CHOICE_2
        ),
    );
    msbt.set("fd_GameTower_expert_select_00", "Pay 200");
    msbt.set("fd_GameTower_expert_select_01", "I'll pass");
    msbt.set("fd_GameTower_expert_select_02", ""); // clear unused
    msbt.set("fd_GameTower_expert_select_03", ""); // clear unused
    msbt.set(
        "fd_GameTower_expert_00",
        &format!(
            "Well, well, well! Double boom in the\nroom! I'm so impressed! The\nRandom course has {}!",
            name(&format!("{} floors", settings.treacherous_tower_floors))
        ),
    );

    patcher.update(msbt.dump())?;

    Ok(())
}

/// Thief Girl
fn patch_thief_girl(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, DungeonHagure, "Hagure")?;

    // Shorten initial Thief Girl text to just the last textbox.
    msbt.set("Hagure_girl_03", "Come on. Let's hurry out of here. This\nplace gives me the chills.");
    patcher.update(msbt.dump())?;

    Ok(())
}

/// Gramps
fn patch_cross_old_man(patcher: &mut Patcher) -> Result<()> {
    let mut msbt = load_msbt(patcher, FieldLight, "CrossOldMan")?;

    msbt.set(
        "cross_old_man_05_select",
        &format!("Well then. Want to try your hand\nat battling {}?{}", name("me"), *CHOICE_2),
    );
    msbt.set("cross_old_man_10_select", &format!("Do you want to try battling me again?{}", *CHOICE_2));

    patcher.update(msbt.dump())?;

    Ok(())
}

/// Street Merchant - Shorten text & show the item names
fn patch_street_merchant(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let item_left =
        seed_info.layout.get_unsafe("Street Merchant (Left)", regions::hyrule::kakariko::village::SUBREGION).as_str();
    let item_right =
        seed_info.layout.get_unsafe("Street Merchant (Right)", regions::hyrule::kakariko::village::SUBREGION).as_str();

    let mut street_merchant = load_msbt(patcher, FieldLight, "FieldLight_18")?;
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
fn patch_sahasrahla(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let (power, _) = seed_info.layout.find_single(PendantOfPower).unwrap();
    let (wisdom, _) = seed_info.layout.find_single(PendantOfWisdom).unwrap();
    let (courage, _) = seed_info.layout.find_single(PendantOfCourage).unwrap();

    let mut sahasrahla = load_msbt(patcher, IndoorLight, "FieldLight_18")?;

    sahasrahla.set(
        "lgt_Sahasrahla_first_03",
        &format!("The {} is in the\n{}.", attention("Pendant of Power"), attention(power)),
    );
    sahasrahla
        .set("lgt_Sahasrahla_first_12", &format!("The {} is in the\n{}.", name("Pendant of Wisdom"), name(wisdom)));
    sahasrahla.set(
        "lgt_Sahasrahla_first_10",
        &format!("And the {} is in the\n{}.", green("Pendant of Courage"), green(courage)),
    );

    patcher.update(sahasrahla.dump())?;

    Ok(())
}

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
        ("lgt_MayoinoHintObake_Msg3", String::from("")),
        ("lgt_MayoinoHintObake_Msg5", String::from("")),
        ("lgt_MayoinoHintObake_Msg7", String::from("")),
        ("lgt_MayoinoHintObake_Msg9", String::from("")),
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

/// Mother Maiamai Sign
#[allow(unused)]
fn patch_mother_maiamai_sign(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let major_item_count = [
        "Maiamai Bow Upgrade", "Maiamai Boomerang Upgrade", "Maiamai Hookshot Upgrade", "Maiamai Hammer Upgrade",
        "Maiamai Bombs Upgrade", "Maiamai Fire Rod Upgrade", "Maiamai Ice Rod Upgrade", "Maiamai Tornado Rod Upgrade",
        "Maiamai Sand Rod Upgrade", "100 Maiamai",
    ]
    .iter()
    .flat_map(|&loc| {
        if let Some(item) = seed_info.layout.get(loc, regions::hyrule::lake::cave::SUBREGION) {
            if item.is_major_item() {
                return Some(());
            }
        }
        None
    })
    .count();

    let mut msbt = load_msbt(patcher, FieldLight, "FieldLight_35")?;
    msbt.set(
        "SNBD_lgt_FieldLight35_Kinsta_SignBoard_00",
        &format!(
            "Do Not Enter! Very Strange Creature\nwith {} Inside!",
            attention(&format!("{} Major Item{}", major_item_count, if major_item_count == 1 { "" } else { "s" }))
        ),
    );
    patcher.update(msbt.dump())?;

    Ok(())
}

fn patch_bow_of_light(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    if let Some(bow_of_light_hint) = seed_info.hints.bow_of_light_hint.as_ref() {
        let mut msbt = load_msbt(patcher, IndoorDark, "HintGhostDark")?;
        // Most of HintGhostDark.msbt is a duplicate of the identical file under FieldDark, but it's not used. Choosing
        // an easily testable ghost Key to repurpose for a new Ghost in Hilda's Study.
        msbt.set("HintGhost_FieldDark_2C_014", &bow_of_light_hint.get_hint());
        // fixme also dumb: clear out unused messages to keep filesize down.
        msbt.clear("HintGhost_FieldDark_02_001");
        msbt.clear("HintGhost_FieldDark_03_002");
        msbt.clear("HintGhost_FieldDark_07_003");
        msbt.clear("HintGhost_FieldDark_14_004");
        msbt.clear("HintGhost_FieldDark_16_005");
        msbt.clear("HintGhost_FieldDark_18_006");
        msbt.clear("HintGhost_FieldDark_1A_009");
        msbt.clear("HintGhost_FieldDark_1E_010");
        msbt.clear("HintGhost_FieldDark_28_011");
        msbt.clear("HintGhost_FieldDark_29_012");
        msbt.clear("HintGhost_FieldDark_2A_013");
        msbt.clear("HintGhost_FieldDark_30_015");
        msbt.clear("HintGhost_FieldDark_33_016");
        msbt.clear("HintGhost_FieldDark_35_017");
        msbt.clear("HintGhost_FieldDark_35_018");
        msbt.clear("HintGhost_FieldDark_35_019");
        msbt.clear("HintGhost_FieldDark_1E_020");
        msbt.clear("HintGhost_FieldDark_33_021");
        msbt.clear("HintGhost_FieldDark_33_022");
        patcher.update(msbt.dump())?;
    }

    Ok(())
}

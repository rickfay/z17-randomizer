use std::collections::BTreeMap;

use game::{
    Course::{self, *},
    Item::{PendantPower, PendantWisdom},
};
use log::info;
use modinfo::text::{Color, Colored, Control};

use crate::{
    hints::Hint,
    patch::messages::{hint_ghosts::HintGhost, msbt::load_msbt},
    LocationInfo, Patcher, Result, SeedInfo,
};

mod hint_ghosts;
mod msbt;

/// Patch MSBT Message Files
pub fn patch_messages(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    info!("Patching MSBT Files...");

    // debug(patcher, FieldLight, "FieldLight_05");

    patch_file_select(patcher, seed_info)?;
    // patch_ravio(patcher)?;
    patch_great_rupee_fairy(patcher)?;

    // patch_street_merchant(patcher, seed_info)?;
    patch_sahasrahla(patcher, seed_info)?;
    patch_general_hint_ghosts(patcher, seed_info)?;
    patch_hint_ghosts(patcher, seed_info)?;
    patch_bow_of_light(patcher, seed_info)?;

    Ok(())
}

/// Prints out all String Values and their indexed Label Keys for a given MSBT File
#[allow(unused)]
#[deprecated]
fn debug(patcher: &mut Patcher, course: Course, file: &str) {
    load_msbt(patcher, course, file).unwrap().debug();
    info!("Early Debug Exit");
    std::process::exit(0);
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

#[allow(unused)]
fn patch_ravio(patcher: &mut Patcher) -> Result<()> {
    let mut ravio_shop = load_msbt(patcher, IndoorLight, "FieldLight_2C").unwrap();

    ravio_shop.set(
        "lgt_NpcRental_08",
        &format!(
            "Huh? Not interested?\nIf you don't have enough rupees, I'll\ngive you your first item {}.",
            Colored::new(Color::Name, "for free"),
        ),
    );
    ravio_shop.set("lgt_RentalKeeper_Field_2C_03", "stuff and things");
    patcher.update(ravio_shop.dump())?;

    Ok(())
}

fn patch_great_rupee_fairy(patcher: &mut Patcher) -> Result<()> {
    let mut grf = load_msbt(patcher, CaveDark, "Cave").unwrap();
    grf.set(
        "CaveDark29_LuckyFairy_00",
        &format!("Throw Rupees into the fountain?\n{}", Control::Choice2),
    );
    grf.set("CaveDark29_LuckyFairy_01", "Throw 3000");
    grf.set("CaveDark29_LuckyFairy_02", "Don't throw any");
    grf.set("CaveDark29_LuckyFairy_03", "1234567"); // shorten string so file matches OG size FIXME
    patcher.update(grf.dump())?;

    Ok(())
}

#[allow(unused)]
fn patch_street_merchant(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let item_left = seed_info
        .layout
        .get(&LocationInfo::new(
            crate::regions::hyrule::kakariko::village::SUBREGION,
            "Street Merchant (Left)",
        ))
        .unwrap()
        .as_str();
    let item_right = seed_info
        .layout
        .get(&LocationInfo::new(
            crate::regions::hyrule::kakariko::village::SUBREGION,
            "Street Merchant (Right)",
        ))
        .unwrap()
        .as_str();

    let mut street_merchant = load_msbt(patcher, FieldLight, "FieldLight_18").unwrap();
    street_merchant.set(
        "lgt_NpcStand_BottleEmpty_00_select",
        &format!(
            "That's a {}.\nUseful for a bunch of things.\nHow about {}?{}",
            Colored::new(Color::Name, item_left),
            Control::Price,
            Control::Choice2
        ),
    );

    street_merchant.set(
        "lgt_NpcStand_ZoraTreasure_00_select",
        &format!(
            "Ah, yes! A {} \n\
        of remarkable quality. Smooth as silk!\n\
        And for you? Only {}!{}",
            Colored::new(Color::Name, item_right),
            Control::Price,
            Control::Choice2
        ),
    );
    street_merchant.set(
        "lgt_NpcStand_ZoraTreasure_01",
        &format!(
            "Sorry to see it go, actually. I just\n\
        couldn't stop touching that\n\
        smooth, smooth {}.\n\
        Oh it's so VERY smooth! I shouldn't\n\
        have let it go at such a bargain.",
            Colored::new(Color::Name, item_right),
        ),
    );

    patcher.update(street_merchant.dump())?;

    Ok(())
}

/// Sahasrahla gives out the locations of the Red & Blue Pendants
#[allow(unused)]
fn patch_sahasrahla(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let (pow_region, _) = seed_info.layout.find_single(PendantWisdom).unwrap();
    let (pop_region, _) = seed_info.layout.find_single(PendantPower).unwrap();

    let mut sahasrahla = load_msbt(patcher, FieldLight, "FieldLight_1B")?;
    sahasrahla.set(
        "lgt_NpcSahasrahla_Field1B_08",
        &format!(
            "The {} has been\n\
        enshrined in the {}.\n\
        \n\
        And the {}, in the\n\
        {}.",
            Colored::new(Color::Name, "Pendant of Wisdom"),
            Colored::new(Color::Name, pow_region),
            Colored::new(Color::Attention, "Pendant of Power"),
            Colored::new(Color::Attention, pop_region)
        ),
    );

    patcher.update(sahasrahla.dump())?;

    Ok(())
}

fn patch_general_hint_ghosts(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let price = seed_info.settings.logic.hint_ghost_price.to_string();

    let mut hint_ghost = load_msbt(patcher, LanguageBoot, "HintGhost")?;
    hint_ghost.set(
        "HintGhost_02_select",
        &format!(
            "Buy a {} for {}?{}",
            Colored::new(Color::Blue, "Ghost Hint"),
            Colored::new(Color::Attention, format!("{} Rupees", price.as_str()).as_str()),
            Control::Choice2,
        ),
    );
    hint_ghost.set("HintGhost_02_select_00", "Buy");
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
            let entry = msbt_hint_map
                .entry((hint_ghost.course, hint_ghost.msbt_file))
                .or_insert_with(BTreeMap::new);
            entry.insert(hint_ghost.msg_label, path_hint.text().to_game_text());
        }
    }

    // Always Hints
    for always_hint in &seed_info.hints.always_hints {
        // Make mutable copy of hint for processing
        let always_hint = &mut always_hint.clone();

        for ghost in &always_hint.ghosts {
            let hint_ghost = HintGhost::from(*ghost);
            let entry = msbt_hint_map
                .entry((hint_ghost.course, hint_ghost.msbt_file))
                .or_insert_with(BTreeMap::new);
            entry.insert(hint_ghost.msg_label, always_hint.text().to_game_text());
        }
    }

    // Sometimes Hints
    for sometimes_hint in &seed_info.hints.sometimes_hints {
        // Make mutable copy of hint for processing
        let sometimes_hint = &mut sometimes_hint.clone();

        for ghost in &sometimes_hint.ghosts {
            let hint_ghost = HintGhost::from(*ghost);
            let entry = msbt_hint_map
                .entry((hint_ghost.course, hint_ghost.msbt_file))
                .or_insert_with(BTreeMap::new);
            entry.insert(hint_ghost.msg_label, sometimes_hint.text().to_game_text());
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
        msbt.set("HintGhost_FieldDark_2C_014", &bow_of_light_hint.text().to_game_text());
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

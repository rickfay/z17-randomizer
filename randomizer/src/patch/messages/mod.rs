use {
    crate::{
        hints::{formatting::*, Hint},
        patch::messages::{hint_ghosts::HintGhost, msbt::load_msbt},
        LocationInfo, Patcher, Result, SeedInfo,
    },
    albw::{
        course::Id::*,
        Item::{PendantPower, PendantWisdom},
    },
    log::info,
    std::collections::HashMap,
};

mod hint_ghosts;
mod msbt;

/// Patch MSBT Message Files
pub fn patch_messages(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    info!("Patching MSBT Files...");

    // debug(patcher, FieldLight, "Field");

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
fn debug(patcher: &mut Patcher, course: albw::course::Id, file: &str) {
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
            name("for free")
        ),
    );
    ravio_shop.set("lgt_RentalKeeper_Field_2C_03", &format!("stuff and things"));
    patcher.update(ravio_shop.dump())?;

    Ok(())
}

fn patch_great_rupee_fairy(patcher: &mut Patcher) -> Result<()> {
    let mut grf = load_msbt(patcher, CaveDark, "Cave").unwrap();
    grf.set("CaveDark29_LuckyFairy_00", &format!("Throw Rupees into the fountain?\n{}", *CHOICE_2));
    grf.set("CaveDark29_LuckyFairy_01", "Throw 3000");
    grf.set("CaveDark29_LuckyFairy_02", "Don't throw any");
    grf.set("CaveDark29_LuckyFairy_03", "1234567"); // shorten string so file matches OG size
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
            name(item_left),
            *PRICE,
            *CHOICE_2
        ),
    );

    street_merchant.set(
        "lgt_NpcStand_ZoraTreasure_00_select",
        &format!(
            "Ah, yes! A {} \n\
        of remarkable quality. Smooth as silk!\n\
        And for you? Only {}!{}",
            name(item_right),
            *PRICE,
            *CHOICE_2
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
            name(item_right)
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
    let price = seed_info.settings.logic.hint_ghost_price.to_string();

    let mut hint_ghost = load_msbt(patcher, LanguageBoot, "HintGhost")?;
    hint_ghost.set(
        "HintGhost_02_select",
        &format!("Buy a {} for {}?{}", blue("Ghost Hint"), attention(price.as_str()), *CHOICE_2),
    );
    hint_ghost.set("HintGhost_02_select_00", "Buy");
    patcher.update(hint_ghost.dump())?;
    Ok(())
}

fn patch_hint_ghosts(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    if seed_info.hints.path_hints.is_empty() {
        info!("No Ghost Hints generated.");
    } else {
        info!("Patching Hint Ghosts...");
    }

    // Organize Hints by the MSBT File they need to update
    let mut msbt_hint_map = HashMap::new();

    // Path Hints
    for path_hint in &seed_info.hints.path_hints {
        // Make mutable copy of hint for processing
        let path_hint = &mut path_hint.clone();

        for ghost in &path_hint.ghosts {
            let hint_ghost = HintGhost::from(*ghost);
            let entry = msbt_hint_map
                .entry((hint_ghost.course, hint_ghost.msbt_file))
                .or_insert_with(|| HashMap::new());
            entry.insert(hint_ghost.msg_label, path_hint.get_hint());
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
                .or_insert_with(|| HashMap::new());
            entry.insert(hint_ghost.msg_label, always_hint.get_hint());
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
                .or_insert_with(|| HashMap::new());
            entry.insert(hint_ghost.msg_label, sometimes_hint.get_hint());
        }
    }

    // Update the MSBT Files with the generated Hints
    for ((course, msbt_file), labels) in msbt_hint_map {
        let mut msbt_file = load_msbt(patcher, course, msbt_file)?;
        for (label, hint) in labels {
            msbt_file.set(label, &hint);
        }
        patcher.update(msbt_file.dump())?;
    }

    Ok(())
}

fn patch_bow_of_light(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    let mut msbt = load_msbt(patcher, DungeonBoss, "Ganon")?;
    msbt.set("gnn_yumiya_020", &seed_info.hints.bow_of_light_hint.as_ref().unwrap().get_hint());
    patcher.update(msbt.dump())?;

    Ok(())
}

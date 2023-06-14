use {
    crate::{
        hints::{formatting::*, Hint},
        patch::messages::hint_ghosts::HintGhost,
        LocationInfo, Result, SeedInfo,
    },
    albw::{
        course::Id::*,
        Item::{PendantPower, PendantWisdom},
    },
    jack::{lms::msbt::MsbtFile, rom::fs::US_English, sead::SzsFile, JackFile},
    log::info,
    macros::fail,
    patcher::Patcher,
    std::{collections::HashMap, io::Error},
};

mod hint_ghosts;

/// Patch MSBT Message Files
pub fn patch_messages(patcher: &mut Patcher, seed_info: &SeedInfo) -> Result<()> {
    info!("Patching MSBT Files...");

    let mut cave_dark = patcher.get::<SzsFile>(US_English::CAVE_DARK)?;
    let mut dungeon_boss = patcher.get::<SzsFile>(US_English::DUNGEON_BOSS)?;
    let mut field_dark = patcher.get::<SzsFile>(US_English::FIELD_DARK)?;
    let mut field_light = patcher.get::<SzsFile>(US_English::FIELD_LIGHT)?;
    let mut indoor_dark = patcher.get::<SzsFile>(US_English::INDOOR_DARK)?;
    let mut language_boot = patcher.get::<SzsFile>(US_English::LANGUAGE_BOOT)?;

    // debug(&mut patcher.game, "US_English/FieldLight.szs", "US_English/Field.msbt")?;

    patch_file_select(&mut language_boot, seed_info)?;
    // patch_ravio(patcher)?;
    patch_great_rupee_fairy(&mut cave_dark)?;

    // patch_street_merchant(patcher, seed_info)?;
    patch_sahasrahla(&mut field_light, seed_info)?;
    patch_general_hint_ghosts(&mut language_boot, seed_info)?;
    patch_hint_ghosts(&mut field_dark, &mut field_light, &mut indoor_dark, seed_info)?;
    patch_bow_of_light(&mut dungeon_boss, seed_info)?;

    // todo not actually doing anything with the SARC files here now

    Ok(())
}

/// Prints out all String Values and their indexed Label Keys for a given MSBT File
#[allow(unused)]
#[deprecated]
fn debug(patcher: &mut Patcher, archive_name: &str, filename: &str) -> Result<(), Error> {
    let mut archive = patcher.get::<SzsFile>(archive_name)?;
    archive.open::<MsbtFile>(filename)?.debug();
    info!("Success! Stopping...");
    std::process::exit(0);
}

fn patch_file_select(
    language_boot: &mut JackFile<SzsFile>, seed_info: &SeedInfo,
) -> Result<(), Error> {
    let mut file_select_b = language_boot.open::<MsbtFile>("US_English/Mn_FileSelectB.msbt")?;

    file_select_b.set("HeadLineText_00", &seed_info.hash.item_hash);
    file_select_b.set("HeadLineText_01", &seed_info.hash.item_hash);
    file_select_b.set("HeadLineText_10", &seed_info.hash.item_hash);

    language_boot.update(file_select_b);

    // let mut file_select_t = msbt::load(patcher, LanguageBoot, "Mn_FileSelectT").unwrap();
    // file_select_t.set("T_FileNumber_00", format!("Hash: {:0>5}", seed_info.hash));
    // file_select_t.set("T_FileNumber_Hard_00", format!("Hash: {:0>5}", seed_info.hash));
    // patcher.update(file_select_t.dump())?;

    Ok(())
}

#[allow(unused)]
fn patch_ravio(indoor_light: &mut JackFile<SzsFile>, patcher: &mut Patcher) -> Result<()> {
    let mut ravio_shop = indoor_light.open::<MsbtFile>("US_English/FieldLight_2C.msbt")?;

    ravio_shop.set(
        "lgt_NpcRental_08",
        &format!(
            "Huh? Not interested?\nIf you don't have enough rupees, I'll\ngive you your first item {}.",
            name("for free")
        ),
    );
    ravio_shop.set("lgt_RentalKeeper_Field_2C_03", &format!("stuff and things"));

    indoor_light.update(ravio_shop);

    Ok(())
}

fn patch_great_rupee_fairy(cave_dark: &mut JackFile<SzsFile>) -> Result<()> {
    let mut grf = cave_dark.open::<MsbtFile>("US_English/Cave.msbt")?;

    grf.set("CaveDark29_LuckyFairy_00", &format!("Throw Rupees into the fountain?\n{}", *CHOICE_2));
    grf.set("CaveDark29_LuckyFairy_01", "Throw 3000");
    grf.set("CaveDark29_LuckyFairy_02", "Don't throw any");
    grf.set("CaveDark29_LuckyFairy_03", "1234567"); // shorten string so file matches OG size

    cave_dark.update(grf);

    Ok(())
}

#[allow(unused)]
fn patch_street_merchant(field_light: &mut JackFile<SzsFile>, seed_info: &SeedInfo) -> Result<()> {
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

    let mut street_merchant = field_light.open::<MsbtFile>("US_English/FieldLight_18.msbt")?;

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

    field_light.update(street_merchant);

    Ok(())
}

/// Sahasrahla gives out the locations of the Red & Blue Pendants
#[allow(unused)]
fn patch_sahasrahla(field_light: &mut JackFile<SzsFile>, seed_info: &SeedInfo) -> Result<()> {
    let (pow_region, _) = seed_info.layout.find_single(PendantWisdom).unwrap();
    let (pop_region, _) = seed_info.layout.find_single(PendantPower).unwrap();

    let mut sahasrahla = field_light.open::<MsbtFile>("US_English/FieldLight_1B.msbt")?;

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

    field_light.update(sahasrahla);

    Ok(())
}

fn patch_general_hint_ghosts(
    language_boot: &mut JackFile<SzsFile>, seed_info: &SeedInfo,
) -> Result<()> {
    let price = seed_info.settings.logic.hint_ghost_price.to_string();

    let mut hint_ghost = language_boot.open::<MsbtFile>("US_English/HintGhost.msbt")?;

    hint_ghost.set(
        "HintGhost_02_select",
        &format!("Buy a {} for {}?{}", blue("Ghost Hint"), attention(price.as_str()), *CHOICE_2),
    );
    hint_ghost.set("HintGhost_02_select_00", "Buy");

    language_boot.update(hint_ghost);

    Ok(())
}

fn patch_hint_ghosts(
    field_dark: &mut JackFile<SzsFile>, field_light: &mut JackFile<SzsFile>,
    indoor_dark: &mut JackFile<SzsFile>, seed_info: &SeedInfo,
) -> Result<()> {
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
    for ((course, msbt_filename), labels) in msbt_hint_map {
        let course = match course {
            IndoorDark => &mut *indoor_dark,
            FieldDark => &mut *field_dark,
            FieldLight => &mut *field_light,
            _ => fail!("No overworld Hint Ghosts in Course: {}", course.as_str()),
        };

        let mut msbt_file = course.open::<MsbtFile>(&msbt_filename)?;
        for (label, hint) in labels {
            msbt_file.set(label, &hint);
        }

        course.update(msbt_file);
    }

    Ok(())
}

fn patch_bow_of_light(dungeon_boss: &mut JackFile<SzsFile>, seed_info: &SeedInfo) -> Result<()> {
    let mut ganon = dungeon_boss.open::<MsbtFile>("US_English/Ganon.msbt")?;
    ganon.set("gnn_yumiya_020", &seed_info.hints.bow_of_light_hint.as_ref().unwrap().get_hint());
    dungeon_boss.update(ganon);
    Ok(())
}

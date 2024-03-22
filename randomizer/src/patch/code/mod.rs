use super::Patcher;
use crate::filler::filler_item::Item::*;
use crate::filler::filler_item::Randomizable;
use crate::patch::code::arm::data::{add, cmp, mov};
use crate::patch::code::arm::ls::{ldr, ldrb, str_, strb};
use crate::patch::code::arm::lsm::{pop, push};
use crate::patch::code::arm::Register::*;
use crate::patch::code::arm::{b, bl, Instruction, LR, PC, SP};
use crate::{patch::util::prize_flag, regions, Layout, Result, SeedInfo};
use game::Item;
use game::Item::*;
use modinfo::settings::{pedestal::PedestalSetting::*, Settings};
use rom::flag::Flag;
use rom::scene::SpawnPoint;
use rom::ExHeader;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

mod arm;

#[derive(Debug)]
pub struct Code {
    text: u32,
    rodata: u32,
    ips: Ips,
}

impl Code {
    pub fn new(exheader: &ExHeader) -> Self {
        let entry = exheader.get_text_address();
        let text = entry + exheader.get_text_size();
        let rodata = exheader.get_rodata_address() + exheader.get_rodata_size();
        let ips = Ips::new(entry);
        Self { text, rodata, ips }
    }

    pub fn text(&mut self) -> Segment {
        Segment { address: &mut self.text, ips: &mut self.ips }
    }

    pub fn rodata(&mut self) -> Segment {
        Segment { address: &mut self.rodata, ips: &mut self.ips }
    }

    pub fn patch<const N: usize>(&mut self, addr: u32, instructions: [Instruction; N]) -> u32 {
        let code = arm::assemble(addr, instructions);
        let len = code.len() as u32;
        self.overwrite(addr, code);
        len
    }

    #[allow(unused)]
    pub fn addr(&mut self, addr: u32, data: u32) {
        self.overwrite(addr, u32::to_le_bytes(data));
    }

    pub fn overwrite<T>(&mut self, addr: u32, data: T)
    where
        T: Into<Box<[u8]>>,
    {
        self.ips.append(addr, data.into());
    }

    pub fn dump<P>(self, path: P, exheader: &ExHeader) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut exheader = exheader.clone();
        exheader.set_text_size(self.text - exheader.get_text_address());
        exheader.set_rodata_size(self.rodata - exheader.get_rodata_address());
        self.ips.write(File::create(path.join("code.ips"))?)?;
        fs::write(path.join("exheader.bin"), exheader.as_ref())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Segment<'a> {
    address: &'a mut u32,
    ips: &'a mut Ips,
}

impl<'a> Segment<'a> {
    pub fn declare<T>(&mut self, data: T) -> u32
    where
        T: Into<Vec<u8>>,
    {
        let mut data = data.into();
        let addr = *self.address;
        let len = data.len() as u32;
        let padded = (len + 3) & 0xFFFFFFFC;
        data.resize(padded as usize, 0);
        self.ips.append(addr, data);
        *self.address += padded;
        addr
    }

    pub fn define<const N: usize>(&mut self, instructions: [Instruction; N]) -> u32 {
        let addr = *self.address;
        let len = self.patch(addr, instructions);
        *self.address += len;
        addr
    }

    pub fn patch<const N: usize>(&mut self, addr: u32, instructions: [Instruction; N]) -> u32 {
        let code = arm::assemble(addr, instructions);
        let len = code.len() as u32;
        self.write(addr, code);
        len
    }

    pub fn write<T>(&mut self, addr: u32, data: T)
    where
        T: Into<Box<[u8]>>,
    {
        self.ips.append(addr, data.into());
    }
}

#[derive(Debug)]
pub struct Ips {
    buf: Vec<u8>,
    offset: u32,
}

impl Ips {
    pub fn new(offset: u32) -> Self {
        Self { buf: vec![], offset }
    }

    pub fn append<T>(&mut self, offset: u32, data: T)
    where
        T: AsRef<[u8]>,
    {
        let offset = offset - self.offset;
        let data = data.as_ref();
        self.buf.extend_from_slice(&offset.to_be_bytes()[1..4]);
        self.buf.extend_from_slice(&data.len().to_be_bytes()[6..8]);
        self.buf.extend(data);
    }

    pub fn write<W>(self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        writer.write_all(b"PATCH")?;
        writer.write_all(&self.buf)?;
        writer.write_all(b"EOF")?;
        Ok(())
    }
}

pub fn create(patcher: &Patcher, seed_info: &SeedInfo) -> Code {
    let mut code = Code::new(patcher.game.exheader());
    let actor_names = actor_names(&mut code);
    let item_names = item_names(&mut code);

    do_dev_stuff(&mut code, seed_info);

    warp(&mut code);
    //shield_without_sword(&mut code);
    // swordless_beams(&mut code);
    quake(&mut code);

    // Start with Pouch
    if seed_info.settings.start_with_pouch {
        code.text().patch(0x47b28c, [mov(R0, 1)]);
    }

    // Enable Y Button
    code.text().patch(0x47B2C8, [mov(R0, 1)]);

    // instant text
    code.overwrite(0x17A430, [0xFF]);

    rental_items(&mut code);
    progressive_items(&mut code);
    bracelet(&mut code, &seed_info.settings);
    ore_progress(&mut code);
    merchant(&mut code);
    configure_pedestal_requirements(&mut code, &seed_info.settings);
    night_mode(&mut code, &seed_info.settings);
    show_hint_ghosts(&mut code);
    mother_maiamai(&mut code, &seed_info.layout, &item_names);
    pause_menu_warp(&mut code);
    purple_potion_bottles(&mut code, &seed_info.settings);
    // golden_bees(&mut code);
    // file_select_screen_background(&mut code);

    // Show Maiamai on Gear Screen even when you have zero
    code.patch(0x426490, [b(0x4264a0)]);

    // Correct Master Ore display count
    code.patch(0x4637b4, [b(0x463800)]);

    // Tear down Barrier automatically when obtaining Tempered Sword
    let set_barrier_flag = code.text().define([
        ldr(R0, EVENT_FLAG_PTR),
        ldr(R1, Flag::HC_BARRIER.get_value()),
        mov(R2, 1),
        ldr(R0, (R0, 0)),
        bl(0x4CDF40),
        mov(R0, 1),
        pop([R4, R5, R6, PC]),
    ]);
    code.patch(0x344E7C, [b(set_barrier_flag)]);

    // don't lose Bow of Light on defeat
    code.patch(0x502DD8, [mov(R0, R0)]);

    // Infinite Scoot Fruit
    code.patch(0x38D59C, [mov(R2, 0x2)]);

    // Infinite Foul Fruit
    code.patch(0x38D728, [mov(R0, R0)]); // Don't clear equipped slot
    code.patch(0x38D734, [mov(R2, 0x2)]); // Keep fruit

    // blacksmith
    let get_sword_flag1 = code.text().define([
        push([LR]),
        ldr(R0, EVENT_FLAG_PTR),
        ldr(R0, (R0, 0)),
        ldr(R1, 0x375),
        bl(0x584A80),
        add(R0, R0, 3),
        pop([PC]),
    ]);
    let get_sword_flag2 = code.text().define([
        push([LR]),
        ldr(R0, 0x70C8E0),
        ldr(R0, (R0, 0)),
        mov(R1, 0xCE),
        mov(R2, 3),
        bl(0x5822A0),
        add(R0, R0, 4),
        pop([PC]),
    ]);
    code.patch(0x243DE8, [bl(get_sword_flag1)]);
    code.patch(0x30E160, [bl(get_sword_flag2)]);

    let overwrite_rentals = code.text;
    let mut actor_offset = 0;
    let mut name_offset = 0x714608;
    for rental in patcher.rentals.iter() {
        let actor = actor_names
            .get(rental)
            .copied()
            .unwrap_or_else(|| panic!("Could not find actor name for {}", rental.as_str()));
        code.text().define([
            ldr(R1, actor),
            str_(R4, (R0, actor_offset)),
            str_(R1, (R0, actor_offset + 4)),
            add(PC, PC, 0), // bad hack
        ]);
        actor_offset += 8;
        let name = item_names.get(rental).copied().unwrap_or(0x6F9B1A);
        code.overwrite(name_offset, name.to_le_bytes());
        name_offset += 4;
    }
    code.text().define([b(0x5D68F4)]);
    code.patch(0x5D688C, [b(overwrite_rentals)]);
    let rentals = patcher.rentals.iter().map(|item| *item as u8).collect::<Vec<_>>();
    code.overwrite(0x6A0348, rentals);
    let sold_out = 0x5D6B84u32;
    let merchant_left = patcher.merchant[0];
    let merchant_left_actor = code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata().declare(actor_names.get(&merchant_left).unwrap().to_le_bytes());
    code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata().declare(sold_out.to_le_bytes());
    code.overwrite(0x707DD4, merchant_left_actor.to_le_bytes());
    code.overwrite(0x6A03E0, [merchant_left as u8]);
    let merchant_right = patcher.merchant[2];
    let merchant_right_actor = code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata().declare(actor_names.get(&merchant_right).unwrap().to_le_bytes());
    code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata().declare(sold_out.to_le_bytes());
    code.overwrite(0x707DE0, merchant_right_actor.to_le_bytes());
    code.overwrite(0x6A03E8, [merchant_right as u8]);

    // Hearts
    code.patch(0x33497C, [ldr(R1, (R4, 0x2E)), mov(R0, R0)]);

    // Keys
    code.patch(0x192E58, [ldr(R1, (R4, 0x2E))]);

    // Maiamai
    code.patch(0x514254, [ldr(R1, (R4, 0x30))]);

    // Silver and Gold Rupees
    code.patch(0x1D6DBC, [ldr(R1, (R4, 0x2E)), mov(R0, R0)]);

    // Premium Milk
    if seed_info.layout.find_single(LetterInABottle).is_none() {
        // This code makes the Premium Milk work correctly when picked up without having first picked up the Letter.
        // This patch is only applied when the Milk is shuffled in the rando instead of the Letter.
        // If it's desired to have both shuffled at once then this code needs to be re-written.

        code.patch(0x3455B8, [b(0x345578)]); // Repurpose Letter In a Bottle code
        code.patch(0x255930, [mov(R0, 0xD)]); // Give Milk instead of Letter
    }
    code.patch(0x345588, [b(0x34559C)]); // Skip setting Flag 916

    // Pendant Redirection - Get destination coordinates from Byaml
    let redirect_pendants = code.text().define([
        mov(R7, 0x1), // ???
        strb(R7, (SP, 0xA)),
        ldrb(R0, (R4, 0x42)), // scene = arg10
        str_(R0, (SP, 0x0)),
        ldrb(R0, (R4, 0x44)), // scene index = arg11
        str_(R0, (SP, 0x4)),
        ldrb(R0, (R4, 0x2C)), // spawn point = arg0
        strb(R0, (SP, 0x8)),
        b(0x143a78),
    ]);
    code.patch(0x143a3c, [b(redirect_pendants)]);

    // Pendant of Courage - Set Flag 251 when picked up
    let set_courage_flag = code.text().define([
        ldr(R0, EVENT_FLAG_PTR),
        mov(R2, 1),
        ldr(R1, Flag::EASTERN_COMPLETE.get_value()),
        ldr(R0, (R0, 0)),
        bl(FN_SET_EVENT_FLAG),
        b(0x344F00),
    ]);
    code.patch(0x344d9c, [b(set_courage_flag)]);

    // Great Spin Fix to work with and not disappear when obtaining Forgotten Sword
    let great_spin_fix =
        code.text().define([ldr(R0, (R4, 0x4E4)), cmp(R0, 0x3), mov(R2, 0x2).ne(), mov(R2, 0x3).eq(), b(0x344df0)]);
    code.patch(0x344dec, [b(great_spin_fix)]);

    code
}

#[allow(unused_variables)]
fn do_dev_stuff(code: &mut Code, seed_info: &SeedInfo) {
    if !seed_info.settings.dev_mode {
        return;
    }

    // Make each Maiamai worth more (for testing only)
    let amount = 25;
    code.patch(0x2559bc, [add(R1, R1, amount)]);
    code.patch(0x2559c0, [add(R2, R2, amount)]);
}

/// File Select Screen Background
/// TODO Figure out how to make background permanent
#[allow(unused)]
fn file_select_screen_background(code: &mut Code) {
    let sp = SpawnPoint::new(game::Course::Demo, 5, 0); // TODO use argument

    //code.text().patch(0x29d2c0, [mov(R10, 0x0)]); // Standstill camera on FSS over spawn point (ehh)
    code.text().patch(0x29d258, [mov(R0, 0x1)]); // Force always show same scene on FSS
    code.text().patch(0x29d270, [mov(R1, sp.course as u32)]);
    code.text().patch(0x29d278, [mov(R2, sp.scene as u32 - 1)]);
    code.text().patch(0x29d260, [mov(R6, sp.spawn as u32)]);
    let reset_r6 = code.text().define([mov(R6, 0x0), b(0x29d2a4)]);
    code.text().patch(0x29d284, [b(reset_r6)]);
}

fn warp(code: &mut Code) {
    // code.text().patch(0x441ec0, [b(0x442044)]); // Makes quit identical to cancel!
    // code.text().patch(0x4424b0, [mov(R0, R0)]); // Remove SE from "Quit" button!

    // code.text().patch(0x442498, [b(0x4424e0)]); // Quit button acts like Continue!

    let kill_player = code.text().define([
        // todo kill player here
        // no workie yet
        // ldr(R0, PLAYER_OBJECT_SINGLETON),
        // ldr(R0, (R0, 0x0)),
        // mov(R2, 0x0),
        // strb(R2, (R0, 0x598)),

        // bl(0x1973e4),

        // ldr(R0, 0x197440),
        // mov(R2, 0x1),
        // mov(R3, 0x0),
        //
        // mov(R1, SP),
        // ldr(R0, (R0, 0x0)),
        // bl(0x004ef418), // LoadScene
        b(0x4424e0), // Acts like player hit the "Continue" button
    ]);
    code.text().patch(0x442498, [b(kill_player)]);

    // code.text().patch(0x12cc7c, [mov(R0, R0)]); // Don't play SE_S_SELECT sound effect when hit continue button
    // Continue button sets: FUN_002317c0(0x3f800000,param_1 + 0x50);
}

/// Create new item that sets Flag 510
fn quake(code: &mut Code) {
    let earthquake = code.text().define([
        // TODO Play Earthquake Noise:
        // 0x6f8f24 - SE_EVENT_EARTQAUAKE
        // 0x587f74 - PlaySoundEffect Function

        // Set Flag
        ldr(R0, EVENT_FLAG_PTR),
        mov(R2, 0x1),
        ldr(R1, Flag::QUAKE.get_value()), // Quake
        ldr(R0, (R0, 0x0)),
        bl(FN_SET_EVENT_FLAG),
        b(0x344f00),
    ]);
    code.addr(0x344848, earthquake); // Empty -> Quake
}

/// Mother Maiamai Stuff
fn mother_maiamai(code: &mut Code, layout: &Layout, item_names: &HashMap<Item, u32>) {
    /// Use flags 302-311 (not 305) to record whether we've picked up that item's upgrade.
    /// The "inventory index" (see table: 0x6a6170) of each item gets added to this:
    /// * 0x4 = Bow
    /// * 0x3 = Boomerang
    /// * 0xB = Hookshot
    /// * 0x6 = Hammer
    /// * 0x2 = Bombs
    /// * 0x8 = Fire Rod
    /// * 0x9 = Ice Rod
    /// * 0xA = Tornado Rod
    /// * 0x7 = Sand Rod
    const NEW_LOCAL_FLAGS_START_IDX: u32 = 300;

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Great Spin final Nice Item check (?)
    // Accept Nice Items in addition to their regular counterparts for the check to see if we own anything upgradable.
    // code.patch(0x30fdcc, [b(0x30fef0).ge()]);
    let fn_get_maiamai_flag3 = code.text().define([
        add(R1, R4, NEW_LOCAL_FLAGS_START_IDX),
        ldr(R0, MAP_MANAGER_INSTANCE),
        ldr(R0, (R0, 0x0)),
        ldr(R0, (R0, 0x40)),
        bl(FN_GET_LOCAL_FLAG_3),
        b(0x30fdc4),
    ]);
    code.patch(0x30fdb8, [b(fn_get_maiamai_flag3)]);
    code.patch(0x30fdc4, [cmp(R0, 0x0)]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Rewrite ::::caseD_6 to check local flags for (I think?) 100 Maiamai item giveout
    let thing = code.text().define([
        add(R1, R4, NEW_LOCAL_FLAGS_START_IDX),
        ldr(R0, MAP_MANAGER_INSTANCE),
        ldr(R0, (R0, 0x0)),
        ldr(R0, (R0, 0x40)),
        bl(FN_GET_LOCAL_FLAG_3),
        b(0x30fee4),
    ]);
    code.patch(0x30fed8, [b(thing)]);
    code.patch(0x30fee4, [cmp(R0, 0x1)]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    /*
     * Determines if the player has enough Maiamai compared with how many have been spent to show the upgrade dialog,
     * using the following formula:
     *
     * 0 < floor((total_maiamai_obtained - total_maiamai_on_hand) / (10 - num_nice_items_obtained))
     *
     * For randomizer, we need to replace `num_nice_items_obtained` with a count of our current flags that have been set.
     */
    code.patch(0x30fdf8, [b(0x30fe0c)]); // Skip Great Spin item check
    let fn_get_maiamai_flag3 = code.text().define([
        add(R1, R4, NEW_LOCAL_FLAGS_START_IDX),
        ldr(R0, MAP_MANAGER_INSTANCE),
        ldr(R0, (R0, 0x0)),
        ldr(R0, (R0, 0x40)),
        bl(FN_GET_LOCAL_FLAG_3),
        b(0x30fe48),
    ]);
    code.patch(0x30fe3c, [b(fn_get_maiamai_flag3)]);
    code.patch(0x30fe48, [cmp(R0, 0x1)]);
    code.patch(0x30fe4c, [add(R5, R5, 0x1).eq()]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Skip FUN_00583b1c - Suck/Spit Old/New Item Animation
    code.patch(0x30ffe4, [mov(R0, 1)]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Check our newly created local flags to determine if items can appear on MM's list of items to upgrade
    let fn_get_maiamai_flag3 = code.text().define([
        add(R1, R1, NEW_LOCAL_FLAGS_START_IDX),
        ldr(R0, MAP_MANAGER_INSTANCE),
        ldr(R0, (R0, 0x0)),
        ldr(R0, (R0, 0x40)),
        bl(FN_GET_LOCAL_FLAG_3),
        cmp(R0, 0x0),
        b(0x46d848).eq(),
        b(0x46d888),
    ]);
    code.patch(0x46d840, [b(fn_get_maiamai_flag3).ge()]);
    code.patch(0x46d844, [b(0x46d888)]);

    // Allow getting upgrades if you already have the Nice Item for this slot
    code.patch(0x30feb4, [mov(R0, 0).eq()]);

    // Skip Sound: SE_ShopManKinSta_VACUUM
    code.patch(0x3105c8, [mov(R0, 0x0)]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Ingoing Item Model
    // code.patch(0x30FF8C, [mov(R1, 0x11)]);
    // code.patch(0x30FF94, [mov(R1, 0x11)]);
    // code.patch(0x30FF9C, [mov(R1, 0x11)]);
    // code.patch(0x30FFA4, [mov(R1, 0x11)]);
    // code.patch(0x30FFAC, [mov(R1, 0x11)]);
    // code.patch(0x30FFB4, [mov(R1, 0x11)]);
    // code.patch(0x30FFBC, [mov(R1, 0x11)]);
    // code.patch(0x30FFC4, [mov(R1, 0x11)]);
    // code.patch(0x30FFCC, [mov(R1, 0x11)]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Record upgrades received with a new course flag for each one.
    let bow = layout.get_unsafe("Maiamai Bow Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let boomerang = layout.get_unsafe("Maiamai Boomerang Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let hookshot = layout.get_unsafe("Maiamai Hookshot Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let hammer = layout.get_unsafe("Maiamai Hammer Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let bombs = layout.get_unsafe("Maiamai Bombs Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let fire_rod = layout.get_unsafe("Maiamai Fire Rod Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let ice_rod = layout.get_unsafe("Maiamai Ice Rod Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let tornado_rod = layout.get_unsafe("Maiamai Tornado Rod Upgrade", regions::hyrule::lake::cave::SUBREGION);
    let sand_rod = layout.get_unsafe("Maiamai Sand Rod Upgrade", regions::hyrule::lake::cave::SUBREGION);

    for (offset, addr, item) in [
        (304, 0x3100f8, bow),
        (303, 0x3100f0, boomerang),
        (311, 0x310128, hookshot),
        (306, 0x310100, hammer),
        (302, 0x310130, bombs),
        (308, 0x310110, fire_rod),
        (309, 0x310118, ice_rod),
        (310, 0x310120, tornado_rod),
        (307, 0x310108, sand_rod),
    ] {
        let fn_set_local3_flag_for_this_upgrade = code.text().define([
            ldr(R0, MAP_MANAGER_INSTANCE),
            ldr(R0, (R0, 0x0)),
            ldr(R0, (R0, 0x40)),
            ldr(R1, offset),
            mov(R2, 0x1),
            bl(FN_SET_LOCAL_FLAG_3),
            mov(R0, item.as_item_index()),
            b(0x310134),
        ]);
        code.patch(addr, [b(fn_set_local3_flag_for_this_upgrade)]);
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Item Names
    let maiamai_item_name_table = code.rodata().declare(
        [ice_rod, sand_rod, tornado_rod, bombs, fire_rod, hookshot, boomerang, hammer, bow]
            .iter()
            .flat_map(|item| {
                u32::to_le_bytes(
                    *item_names
                        .get(&Randomizable::normalize(*item))
                        .unwrap_or_else(|| panic!("No item_name for: {item:?}")),
                )
            })
            .collect::<Vec<_>>(),
    );

    let fn_get_maiamai_item_name = code.text().define([
        push([R1, LR]),
        // Discount multiply R0 by 4 (aka, left shift 2)
        add(R0, R0, R0),
        add(R0, R0, R0),
        // R0 = maiamai_item_name_table[R0 * 4]
        ldr(R1, maiamai_item_name_table),
        ldr(R0, (R1, R0)),
        pop([R1, PC]),
    ]);

    code.patch(0x46d858, [bl(fn_get_maiamai_item_name)]);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
}

fn pause_menu_warp(code: &mut Code) {
    // Pause Menu...?
    // code.patch(0x441ee8, [mov(R0, R0)]); // Don't call function to return to FSS?
    // code.patch(0x441eec, [mov(R0, R0)]); // Don't call function to return to FSS?

    let _fn_load_scene_links_house = code.text().define([
        mov(R0, 0x0),
        bl(0x4eefa0),
        mov(R0, 0x1), // ???
        strb(R0, (SP, 0xA)),
        mov(R0, 0x2), // scene = IndoorLight
        str_(R0, (SP, 0x0)),
        mov(R0, 0x0), // index = 1
        str_(R0, (SP, 0x4)),
        mov(R0, 0x1), // spawn = 0
        strb(R0, (SP, 0x8)),
        mov(R3, 0x0),
        mov(R2, 0x0),
        mov(R1, SP),
        ldr(R0, 0x709df8),
        // ldr(R0, (R0, 0x0)),
        bl(0x4ef418), // Load Scene Function
        b(0x441eec),
    ]);

    // code.patch(0x441ee8, [b(fn_load_scene_links_house)]);

    // different attempt...
    // let fn_death_warp = code.text().define([
    //     ldr(R0, (R4, 0x0)), // ???
    //     bl(0x502d24),
    // ]);
    //
    // code.patch(0x441edc, [b(fn_death_warp)]);
    //
    // code.patch(0x0, 0x0);
}

fn purple_potion_bottles(code: &mut Code, settings: &Settings) {
    if settings.purple_potion_bottles {
        code.patch(0x255210, [mov(R1, 0x3)]);
    }
}

/// Golden Bee stuff
#[allow(unused)]
fn golden_bees(code: &mut Code) {
    // Alter Odds of a bee being a golden bee
    let golden_bee_chance = 3; // Choose percentage chance 0-100 (in vanilla it's 3)
    code.patch(0x4cbb8c, [cmp(R0, golden_bee_chance)]);
}

/// Show Hint Ghosts always, without the need for the Hint Glasses
fn show_hint_ghosts(code: &mut Code) {
    // Allow talking to Hint Ghosts without glasses
    code.patch(0x1cb3c8, [mov(R0, 0x1)]);

    // Skip checking if Hint Glasses are taken off
    // Do not change state to "cState_Disappear" (5) or "cState_DisappearWait" (6)
    code.patch(0x1cb70c, [b(0x1cb74c)]);

    // Set (initial?) state to "cState_Wait" (0) instead of "cState_DisappearWait" (6)
    code.patch(0x1cbf9c, [mov(R2, 0x0), b(0x1cbfac)]);
}

fn night_mode(code: &mut Code, settings: &Settings) {
    if settings.night_mode {
        // Keeps Flag 964 from being unset
        code.patch(0x3a8624, [mov(R2, 0x1)]);
    }
}

fn configure_pedestal_requirements(code: &mut Code, settings: &Settings) {
    const FLAG_PEDESTAL: u32 = 375;
    const RETURN_LABEL: u32 = 0x1439c8;

    let ped_instructions = match settings.ped_requirement {
        Vanilla => {
            code.text().define([
                // Power
                ldr(R0, EVENT_FLAG_PTR),
                ldr(R0, (R0, 0x0)),
                ldr(R1, prize_flag(PendantOfPower.into()).get_value() as u32),
                bl(FN_GET_EVENT_FLAG),
                cmp(R0, 0x0),
                b(RETURN_LABEL).eq(),
                // Wisdom
                ldr(R0, EVENT_FLAG_PTR),
                ldr(R0, (R0, 0x0)),
                ldr(R1, prize_flag(PendantOfWisdom.into()).get_value() as u32),
                bl(FN_GET_EVENT_FLAG),
                cmp(R0, 0x0),
                b(RETURN_LABEL).eq(),
                // Set Flag
                ldr(R0, EVENT_FLAG_PTR),
                mov(R2, 0x1),
                ldr(R1, FLAG_PEDESTAL),
                ldr(R0, (R0, 0x0)),
                bl(FN_SET_EVENT_FLAG),
                b(RETURN_LABEL),
            ])
        },
        Standard => {
            code.text().define([
                // Power
                ldr(R0, EVENT_FLAG_PTR),
                ldr(R0, (R0, 0x0)),
                ldr(R1, prize_flag(Randomizable::Item(PendantOfPower)).get_value() as u32),
                bl(FN_GET_EVENT_FLAG),
                cmp(R0, 0x0),
                b(RETURN_LABEL).eq(),
                // Wisdom
                ldr(R0, EVENT_FLAG_PTR),
                ldr(R0, (R0, 0x0)),
                ldr(R1, prize_flag(Randomizable::Item(PendantOfWisdom)).get_value() as u32),
                bl(FN_GET_EVENT_FLAG),
                cmp(R0, 0x0),
                b(RETURN_LABEL).eq(),
                // Courage
                ldr(R0, EVENT_FLAG_PTR),
                ldr(R0, (R0, 0x0)),
                ldr(R1, prize_flag(Randomizable::Item(PendantOfCourage)).get_value() as u32),
                bl(FN_GET_EVENT_FLAG),
                cmp(R0, 0x0),
                b(RETURN_LABEL).eq(),
                // Set Flag
                ldr(R0, EVENT_FLAG_PTR),
                mov(R2, 0x1),
                ldr(R1, FLAG_PEDESTAL),
                ldr(R0, (R0, 0x0)),
                bl(FN_SET_EVENT_FLAG),
                b(RETURN_LABEL),
            ])
        },
    };
    code.patch(0x143968, [b(ped_instructions)]);
}

fn merchant(code: &mut Code) {
    let get_merchant_event_flag =
        code.text().define([ldr(R0, EVENT_FLAG_PTR), ldr(R0, (R0, 0)), ldr(R1, 0x143), b(FN_GET_EVENT_FLAG)]);
    code.patch(0x19487C, [bl(get_merchant_event_flag)]);
}

#[allow(unused)]
fn shield_without_sword(code: &mut Code) {
    let enable_shield_fn = code.text().define([
        mov(R3, 0),
        add(R0, R4, 0x400),
        add(R0, R0, 0x294),
        mov(R2, R3),
        mov(R1, R3),
        bl(0x2d455c), // Enables Shield ???
        b(0x344f00),
    ]);
    code.text().patch(0x3450fc, [b(enable_shield_fn)]);
}

#[allow(unused)]
fn swordless_beams(code: &mut Code) {
    let _ = code.text().define([
        mov(R3, 0),
        add(R0, R4, 0x400),
        add(R0, R0, 0x294),
        mov(R2, R3),
        mov(R1, R3),
        bl(0x2d455c), // Enables Shield ???
        b(0x344f00),
    ]);
}

fn rental_items(code: &mut Code) {
    let map_rental_item = 0x194BFC;
    let flag_offset = 0xF0;
    let getter = code.text().define([
        push([LR]),
        ldr(R0, 0x70C8E0),
        ldr(R0, (R0, 0)),
        add(R1, R1, flag_offset),
        mov(R2, 3),
        bl(0x5822A0),
        cmp(R0, 1),
        mov(R0, 2).eq(),
        pop([PC]),
    ]);
    code.patch(0x194728, [bl(getter)]);
    code.patch(0x311CE4, [bl(getter)]);
    code.patch(0x311EAC, [bl(getter)]);
    code.patch(0x31261C, [bl(getter)]);
    code.patch(0x312660, [bl(getter)]);
    let setter = code.text().define([
        ldrb(R0, (R4, 0x9D0)),
        bl(map_rental_item),
        mov(R6, R1),
        mov(R1, R0),
        add(R1, R1, flag_offset),
        ldr(R0, 0x70C8E0),
        ldr(R0, (R0, 0)),
        mov(R2, 3),
        mov(R3, 1),
        bl(0x4AD9E8),
        b(0x652E70),
    ]);
    code.patch(0x652E34, [b(setter).eq()]);
}

fn progressive_items(code: &mut Code) {
    let return_label = 0x2922C4;
    /*let first_sword = code.text().define([
        ldr(R0, (R0, 0x4C4)),
        cmp(R0, 0),
        mov(R5, 0x3D).eq(),
        mov(R5, 0x1B).ne(),
        b(return_label),
    ]);*/
    let progressive_sword = //if settings.items.captains_sword.is_skipped() {
        code.text().define([
            cmp(R5, 0x1B),
            cmp(R5, 0x1C).ne(),
            b(return_label).ne(),
            ldr(R3, (R0, 0x434)),
            cmp(R3, 0),
            mov(R3, 1).eq(),
            add(R5, R3, 0x1A),
            b(return_label)
        ]);
    /*} else {
        code.text().define([
            cmp(R5, 0x1B),
            cmp(R5, 0x1C).ne(),
            cmp(R5, 0x3D).ne(),
            b(return_label).ne(),
            ldr(R3, (R0, 0x434)),
            cmp(R3, 0),
            b(first_sword).eq(),
            add(R5, R3, 0x1A),
            b(return_label),
        ])
    };*/
    let progressive_bracelet = code.text().define([
        cmp(R5, 0x2A),
        b(progressive_sword).ne(),
        ldr(R0, (R0, 0x490)),
        cmp(R0, 0),
        mov(R5, 0x2A).eq(),
        mov(R5, 0x2B).ne(),
        b(return_label),
    ]);
    let progressive_glove = code.text().define([
        cmp(R5, 0x2F),
        b(progressive_bracelet).ne(),
        ldr(R0, (R0, 0x4AC)),
        cmp(R0, 0),
        mov(R5, 0x2F).eq(),
        mov(R5, 0x31).ne(),
        b(return_label),
    ]);
    let progressive_mail = code.text().define([
        cmp(R5, 0x3F),
        b(progressive_glove).ne(),
        ldr(R0, (R0, 0x4B0)),
        cmp(R0, 2),
        mov(R5, 0x3F).eq(),
        mov(R5, 0x40).ne(),
        b(return_label),
    ]);
    let progressive_lamp = code.text().define([
        cmp(R5, 0x1A),
        b(progressive_mail).ne(),
        ldr(R0, (R0, 0x464)),
        cmp(R0, 0),
        mov(R5, 0x1A).eq(),
        mov(R5, 0x58).ne(),
        b(return_label),
    ]);
    let progressive_bow = code.text().define([
        cmp(R5, 0x11),
        b(progressive_lamp).ne(),
        ldr(R0, (R0, 0x444)),
        cmp(R0, 0),
        mov(R5, 0x11).eq(), // Bow
        b(return_label).eq(),
        cmp(R0, 2),
        mov(R5, 0x55).eq(), // Nice Bow
        mov(R5, 0x5C).ne(), // Bow of Light
        b(return_label),
    ]);
    let progressive_boomerang = code.text().define([
        cmp(R5, 0xF),
        b(progressive_bow).ne(),
        ldr(R0, (R0, 0x440)),
        cmp(R0, 0),
        mov(R5, 0xF).eq(),
        mov(R5, 0x53).ne(),
        b(return_label),
    ]);
    let progressive_hookshot = code.text().define([
        cmp(R5, 0xE),
        b(progressive_boomerang).ne(),
        ldr(R0, (R0, 0x460)),
        cmp(R0, 0),
        mov(R5, 0xE).eq(),
        mov(R5, 0x52).ne(),
        b(return_label),
    ]);
    let progressive_hammer = code.text().define([
        cmp(R5, 0x10),
        b(progressive_hookshot).ne(),
        ldr(R0, (R0, 0x44C)),
        cmp(R0, 0),
        mov(R5, 0x10).eq(),
        mov(R5, 0x54).ne(),
        b(return_label),
    ]);
    let progressive_bombs = code.text().define([
        cmp(R5, 0xC),
        b(progressive_hammer).ne(),
        ldr(R0, (R0, 0x43C)),
        cmp(R0, 0),
        mov(R5, 0xC).eq(),
        mov(R5, 0x50).ne(),
        b(return_label),
    ]);
    let progressive_fire_rod = code.text().define([
        cmp(R5, 0xD),
        b(progressive_bombs).ne(),
        ldr(R0, (R0, 0x454)),
        cmp(R0, 0),
        mov(R5, 0xD).eq(),
        mov(R5, 0x51).ne(),
        b(return_label),
    ]);
    let progressive_ice_rod = code.text().define([
        cmp(R5, 0x9),
        b(progressive_fire_rod).ne(),
        ldr(R0, (R0, 0x458)),
        cmp(R0, 0),
        mov(R5, 0x9).eq(),
        mov(R5, 0x4D).ne(),
        b(return_label),
    ]);
    let progressive_tornado_rod = code.text().define([
        cmp(R5, 0xB),
        b(progressive_ice_rod).ne(),
        ldr(R0, (R0, 0x45C)),
        cmp(R0, 0),
        mov(R5, 0xB).eq(),
        mov(R5, 0x4F).ne(),
        b(return_label),
    ]);
    let progressive_sand_rod = code.text().define([
        cmp(R5, 0xA),
        b(progressive_tornado_rod).ne(),
        ldr(R0, (R0, 0x450)),
        cmp(R0, 0),
        mov(R5, 0xA).eq(),
        mov(R5, 0x4E).ne(),
        b(return_label),
    ]);
    let progressive_net = code.text().define([
        cmp(R5, 0x30),
        b(progressive_sand_rod).ne(),
        ldr(R0, (R0, 0x468)),
        cmp(R0, 0),
        mov(R5, 0x30).eq(),
        mov(R5, 0x59).ne(),
        b(return_label),
    ]);
    let progressive_charm = code.text().define([
        cmp(R5, 0x3E),
        b(progressive_net).ne(),
        ldr(R0, (R0, 0x4A0)),
        cmp(R0, 0),
        mov(R5, 0x3E).eq(),
        mov(R5, 0x19).ne(),
        b(return_label),
    ]);

    code.patch(0x2922A0, [b(progressive_charm)]);
}

fn bracelet(code: &mut Code, settings: &Settings) {
    if settings.start_with_merge {
        // Check Flag 1 (always set) instead of Flag 250 to see if we can merge.
        code.patch(0x4266c8, [mov(R1, 0x1)]);
        code.patch(0x537c40, [mov(R1, 0x1)]);
        return;
    }

    /*
     * FIXME
     *
     * In vanilla, the game determines whether you can merge not by checking your inventory but by
     * checking Flag 250 (Yuga 1 defeated). This patch adds an entry in the Add() function for
     * RingHekiga (full bracelet) and raises the Bracelet level in the inventory to 3 (impossible in
     * vanilla, where it's either 0 if you don't have it or 2 if you have RingRental). The patch
     * then changes all the locations where Flag 250 is checked and instead has them check the
     * player inventory for Bracelet level 3.
     *
     * This works in rando, but breaks the ability to Merge if a vanilla file is loaded as the
     * player inventory for the Bracelet can never be set to 3. Low priority, but change this.
     */

    let item_set_value = 0x255494;
    let add_ring_hekiga = code.text().define([
        add(R0, R4, 0x400),
        mov(R2, 3),
        mov(R1, 0x17),
        add(R0, R0, 0xC),
        bl(item_set_value),
        b(0x344F00),
    ]);
    code.overwrite(0x3448F4, add_ring_hekiga.to_le_bytes());
    let can_merge = code.text().define([
        push([LR]),
        ldr(R0, PLAYER_OBJECT_SINGLETON),
        ldr(R0, (R0, 0)),
        mov(R1, 0x17),
        bl(FN_GET_ITEM_LEVEL),
        cmp(R0, 3),
        mov(R0, 1).eq(),
        mov(R0, 0).ne(),
        pop([PC]),
    ]);
    code.patch(0x1DCA8C, [bl(can_merge)]);
    code.patch(0x4266D0, [bl(can_merge)]);
    code.patch(0x52E654, [bl(can_merge)]);
}

fn ore_progress(code: &mut Code) {
    let get_sword_fake = code.text().define([
        push([R4, LR]),
        mov(R4, 1),
        ldr(R0, 0x70C8E0),
        ldr(R0, (R0, 0)),
        add(R0, R0, 0x400),
        add(R0, R0, 0x88),
        mov(R1, 0xCE),
        bl(0x52A05C),
        add(R4, R4, R0),
        ldr(R0, EVENT_FLAG_PTR),
        ldr(R0, (R0, 0)),
        ldr(R1, 0x375),
        bl(0x584A80),
        add(R4, R4, R0),
        mov(R0, R4),
        pop([R4, PC]),
    ]);
    code.patch(0x4637B8, [bl(get_sword_fake)]);
}

fn actor_names(code: &mut Code) -> HashMap<Item, u32> {
    let mut map = IntoIterator::into_iter(ACTOR_NAME_OFFSETS).collect::<HashMap<_, _>>();
    map.extend(IntoIterator::into_iter(ACTOR_NAMES).map(|(item, name)| {
        let name = format!("{}\0", name);
        (item, code.rodata().declare(name.as_bytes()))
    }));
    map
}

fn item_names(code: &mut Code) -> HashMap<Item, u32> {
    let mut map = IntoIterator::into_iter(ITEM_NAME_OFFSETS).collect::<HashMap<_, _>>();
    map.extend(IntoIterator::into_iter(ITEM_NAMES).map(|(item, name)| {
        let name = format!("item_name_{}\0", name);
        (item, code.rodata().declare(name.as_bytes()))
    }));
    // log::info!("{map:?}");
    map
}

const ACTOR_NAME_OFFSETS: [(Item, u32); 33] = [
    (ItemStoneBeauty, 0x5D2060),
    (RupeeR, 0x5D639C),
    (RupeeG, 0x5D639C),
    (RupeeB, 0x5D639C),
    (RupeePurple, 0x5D639C),
    (RupeeSilver, 0x5D63A4),
    (KeySmall, 0x5D6580),
    (ItemIceRod, 0x5D6AFC),
    (ItemSandRod, 0x5D6B08),
    (ItemTornadeRod, 0x5D6B18),
    (ItemBomb, 0x5D6B28),
    (ItemFireRod, 0x5D6B30),
    (ItemHookShot, 0x5D6B40),
    (ItemBoomerang, 0x5D6B50),
    (ItemHammer, 0x5D6B60),
    (ItemBow, 0x5D6B6C),
    (ItemShield, 0x5D6B78),
    (ItemBottle, 0x5D7048),
    (Item::HintGlasses, 0x5D70AC),
    (RupeeGold, 0x5D7144),
    (ItemSwordLv1, 0x5D7178),
    (ItemSwordLv2, 0x5D7178),
    (ItemSwordLv3, 0x5D7178),
    (ItemSwordLv4, 0x5D7178),
    (LiverPurple, 0x5D762C),
    (LiverYellow, 0x5D7640),
    (LiverBlue, 0x5D7654),
    (MessageBottle, 0x5D76A0),
    (MilkMatured, 0x5D76A0),
    (Item::Pouch, 0x5D7734),
    (ItemBowLight, 0x5D776C),
    (HeartContainer, 0x5D7B7C),
    (HeartPiece, 0x5D7B94),
];

const ACTOR_NAMES: [(Item, &str); 44] = [
    (KeyBoss, "KeyBoss"),
    (TriforceCourage, "BadgeBee"),
    (Compass, "Compass"),
    (ItemKandelaar, "GtEvKandelaar"),
    (ItemKandelaarLv2, "GtEvKandelaar"),
    (ItemMizukaki, "GtEvFin"),
    (RingRental, "RingRental"),
    (RingHekiga, "RingRental"),
    (ItemBell, "GtEvBell"),
    (PowerGlove, "GtEvGloveA"),
    (ItemInsectNet, "GtEvNet"),
    (ItemInsectNetLv2, "GtEvNet"),
    (BadgeBee, "BadgeBee"),
    (ClothesBlue, "GtEvCloth"),
    (Heart, "Heart"),
    (HyruleShield, "GtEvShieldB"),
    (Item::OreYellow, "OreSword"),
    (Item::OreGreen, "OreSword"),
    (Item::OreBlue, "OreSword"),
    (GanbariPowerUp, "PowerUp"),
    (DashBoots, "GtEvBoots"),
    (Item::OreRed, "OreSword"),
    (ItemIceRodLv2, "GtEvRodIceB"),
    (ItemSandRodLv2, "GtEvRodSandB"),
    (ItemTornadeRodLv2, "GtEvTornadoB"),
    (ItemBombLv2, "BombM"),
    (ItemFireRodLv2, "GtEvRodFireB"),
    (ItemHookShotLv2, "GtEvHookshotB"),
    (ItemBoomerangLv2, "GtEvBoomerangB"),
    (ItemHammerLv2, "GtEvHammerB"),
    (ItemBowLv2, "GtEvBowB"),
    (Milk, "GtEvBottleMedicine"),
    (Kinsta, "KinSta"),
    (PendantPower, "Pendant"),
    (PendantWisdom, "Pendant"),
    (PendantCourage, "Pendant"),
    (ZeldaAmulet, "Pendant"),
    (Item::Empty, "KeyBoss"),
    (EscapeFruit, "FruitEscape"),
    (StopFruit, "FruitStop"),
    (SpecialMove, "SwordD"),
    (Fairy, "GtEvBottleFairy"),
    (Bee, "GtEvBottleBee"),
    (GoldenBee, "GtEvBottleBee"),
];

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

const ITEM_NAME_OFFSETS: [(Item, u32); 20] = [
    (ItemBomb, 0x6F9A9A),
    (ItemSandRod, 0x6F9AD0),
    (ItemIceRod, 0x6F9AE2),
    (ItemTornadeRod, 0x6F9AF3),
    (ItemFireRod, 0x6F9B08),
    (LiverPurple, 0x6F9B55),
    (ItemBottle, 0x6F9B6C),
    (LiverBlue, 0x6F9B94),
    (ItemBoomerang, 0x6F9BA9),
    (ItemHammer, 0x6F9CCC),
    (ItemHookShot, 0x6F9CDD),
    (ItemBow, 0x6F9D08),         // item_name_bow
    (LiverYellow, 0x6F9D2F),     // item_name_liver_yellow
    (ItemStoneBeauty, 0x6F9D56), // item_name_stonebeauty
    (RupeeR, 0x6f9c2f),          // item_name_bfirerod_rental
    (RupeeG, 0x6f9c13),          // item_name_tornaderod_rental
    (RupeeB, 0x6f9bfb),          // item_name_icerod_rental
    (RupeePurple, 0x6f9c49),     // item_name_boomerang_rental
    (RupeeSilver, 0x6f9c7c),     // item_name_hookshot_rental
    (RupeeGold, 0x6f9be2),       // item_name_sandrod_rental
];

const ITEM_NAMES: [(Item, &str); 57] = [
    (BadgeBee, "beebadge"),
    (Compass, "compass"),
    (ItemBell, "bell"),
    (ItemBowLight, "bow_light"),
    (RingRental, "bracelet"),
    (RingHekiga, "bracelet"),
    (ClothesBlue, "clothes_blue"),
    (EscapeFruit, "doron"),
    (StopFruit, "durian"),
    (GanbariPowerUp, "ganbari_power_up"),
    (HeartContainer, "heartcontioner"),
    (HeartPiece, "heartpiece"),
    (Item::HintGlasses, "hintglass"),
    (HyruleShield, "hyrule_shield"),
    (KeyBoss, "keyboss"),
    (TriforceCourage, "triforce_courage"),
    (KeySmall, "keysmall"),
    (Kinsta, "kinsta"),
    (ItemKandelaar, "lantern"),
    (ItemKandelaarLv2, "lantern_lv2"),
    (ItemSwordLv1, "mastersword"),
    (ItemSwordLv2, "mastersword"),
    (ItemSwordLv3, "mastersword"),
    (ItemSwordLv4, "mastersword"),
    (Item::Empty, "gamecoin"),
    (MessageBottle, "messagebottle"),
    (Milk, "milk"),
    (MilkMatured, "milk_matured"),
    (ItemInsectNet, "net"),
    (ItemInsectNetLv2, "net_lv2"),
    (Item::OreYellow, "ore"),
    (Item::OreGreen, "ore"),
    (Item::OreBlue, "ore"),
    (Item::OreRed, "ore"),
    (DashBoots, "pegasus"),
    (Heart, "potshop_heart"),
    (PendantCourage, "courage"),
    (PendantPower, "power"),
    (PendantWisdom, "wisdom"),
    (Item::Pouch, "pouch"),
    (PowerGlove, "powergloves"),
    (ItemShield, "shield"),
    (SpecialMove, "special_move"),
    (ItemBombLv2, "bomb_LV2"),
    (ItemBoomerangLv2, "boomerang_LV2"),
    (ItemBowLv2, "bow_LV2"),
    (ItemFireRodLv2, "firerod_LV2"),
    (ItemHammerLv2, "hammer_LV2"),
    (ItemHookShotLv2, "hookshot_LV2"),
    (ItemIceRodLv2, "icerod_LV2"),
    (ItemSandRodLv2, "sandrod_LV2"),
    (ItemTornadeRodLv2, "tornaderod_LV2"),
    (ItemMizukaki, "web"),
    (ZeldaAmulet, "charm"),
    (Fairy, "fairy"),
    (Bee, "bee"),
    (GoldenBee, "goldenbee"),
];

const EVENT_FLAG_PTR: u32 = 0x70B728;
const FN_GET_ITEM_LEVEL: u32 = 0x55696C;
const FN_GET_EVENT_FLAG: u32 = 0x584B94;
const FN_SET_EVENT_FLAG: u32 = 0x4CDF40;

/// r0: PlayerObjectSingleton <br />
/// r1: flag index
const FN_GET_LOCAL_FLAG_3: u32 = 0x52a05c;

/// r0: PlayerObjectSingleton <br />
/// r1: flag index <br />
/// r2: new flag value (0 or 1)
const FN_SET_LOCAL_FLAG_3: u32 = 0x1bb724;

const MAP_MANAGER_INSTANCE: u32 = 0x70c8e0;
// const PTR_MAP_MANAGER_INSTANCE: u32 = 0x27320c;
const PLAYER_OBJECT_SINGLETON: u32 = 0x70FB60;
const VTABLE_STRING: u32 = 0x6F5988;

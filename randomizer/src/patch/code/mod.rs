use std::{
    array,
    collections::HashMap,
    fs::{self, File},
    io::prelude::*,
    path::Path,
};

use albw::{ExHeader, Item};

use crate::{Result, Settings};

mod arm;

use arm::*;

use super::Patcher;

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
        Segment {
            address: &mut self.text,
            ips: &mut self.ips,
        }
    }

    pub fn rodata(&mut self) -> Segment {
        Segment {
            address: &mut self.rodata,
            ips: &mut self.ips,
        }
    }

    pub fn patch<const N: usize>(&mut self, addr: u32, instructions: [Instruction; N]) -> u32 {
        let code = arm::assemble(addr, instructions);
        let len = code.len() as u32;
        self.overwrite(addr, code);
        len
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
        Self {
            buf: vec![],
            offset,
        }
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

pub fn create(patcher: &Patcher, settings: &Settings) -> Code {
    let mut code = Code::new(patcher.game.exheader());
    if settings.modifications.y_button_enabled {
        code.text().patch(0x47B2C8, [mov(R0, 1)]);
    }
    // instant text
    code.overwrite(0x17A430, [0xFF]);
    rental_items(&mut code);
    progressive_items(&mut code, settings);
    bracelet(&mut code);
    ore_progress(&mut code);
    merchant(&mut code);
    // fix castle barrer?
    let master_sword_flag = code.text().define([
        ldr(R0, EVENT_FLAG_PTR),
        ldr(R1, 410),
        mov(R2, 1),
        ldr(R0, (R0, 0)),
        bl(0x4CDF40),
        mov(R0, 1),
        pop([R4, R5, R6, PC]),
    ]);
    code.patch(0x344E50, [b(master_sword_flag)]);
    // don't lose Bow of Light on defeat
    code.patch(0x502DD8, [mov(R0, R0)]);
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
    let actor_names = actor_names(&mut code);
    let item_names = item_names(&mut code);
    let overwrite_rentals = code.text;
    let mut actor_offset = 0;
    let mut name_offset = 0x714608;
    for rental in patcher.rentals.iter() {
        let actor = actor_names
            .get(rental)
            .copied()
            .expect("Could not find actor name.");
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
    let rentals = patcher
        .rentals
        .iter()
        .map(|item| *item as u8)
        .collect::<Vec<_>>();
    code.overwrite(0x6A0348, rentals);
    let sold_out = 0x5D6B84u32;
    let merchant_left = patcher.merchant[0];
    let merchant_left_actor = code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata()
        .declare(actor_names.get(&merchant_left).unwrap().to_le_bytes());
    code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata().declare(sold_out.to_le_bytes());
    code.overwrite(0x707DD4, merchant_left_actor.to_le_bytes());
    code.overwrite(0x6A03E0, [merchant_left as u8]);
    let merchant_right = patcher.merchant[2];
    let merchant_right_actor = code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata()
        .declare(actor_names.get(&merchant_right).unwrap().to_le_bytes());
    code.rodata().declare(VTABLE_STRING.to_le_bytes());
    code.rodata().declare(sold_out.to_le_bytes());
    code.overwrite(0x707DE0, merchant_right_actor.to_le_bytes());
    code.overwrite(0x6A03E8, [merchant_right as u8]);
    // Hearts
    code.patch(0x33497C, [ldr(R1, (R4, 0x2E)), mov(R0, R0)]);
    // Keys
    code.patch(0x192E58, [ldr(R1, (R4, 0x2E))]);
    // Premium milk
    let premium_milk = code.text().define([
        ldr(R0, EVENT_FLAG_PTR),
        mov(R2, 1),
        ldr(R1, 0x395),
        ldr(R0, (R0, 0)),
        bl(SET_EVENT_FLAG_FN),
        b(0x344F00),
    ]);
    code.patch(0x3455C4, [b(premium_milk)]);
    // Do not add message bottle or premium milk to inventory
    code.patch(0x345580, [mov(R0, 0xFF)]);
    code.patch(0x3455C0, [mov(R0, 0xFF)]);
    code
}

fn merchant(code: &mut Code) {
    let get_merchant_event_flag = code.text().define([
        ldr(R0, EVENT_FLAG_PTR),
        ldr(R0, (R0, 0)),
        ldr(R1, 0x143),
        b(GET_EVENT_FLAG_FN),
    ]);
    code.patch(0x19487C, [bl(get_merchant_event_flag)]);
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

fn progressive_items(code: &mut Code, settings: &Settings) {
    let return_label = 0x2922C4;
    let first_sword = code.text().define([
        ldr(R0, (R0, 0x4C4)),
        cmp(R0, 0),
        mov(R5, 0x3D).eq(),
        mov(R5, 0x1B).ne(),
        b(return_label),
    ]);
    let progressive_sword = if settings.items.captains_sword.is_skipped() {
        code.text().define([
            cmp(R5, 0x1B),
            cmp(R5, 0x1C).ne(),
            b(return_label).ne(),
            ldr(R3, (R0, 0x434)),
            cmp(R3, 0),
            mov(R3, 1).eq(),
            add(R5, R3, 0x1A),
            b(return_label),
        ])
    } else {
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
    };
    let progressive_bracelet = //if settings.items.first_bracelet.is_skipped() {
        code.text().define([
            cmp(R5, 0x2A),
            cmp(R5, 0x2B).ne(),
            b(progressive_sword).ne(),
            mov(R5, 0x2B),
            b(return_label),
        ]);
    // } else {
    //     code.text().define([
    //         cmp(R5, 0x2A),
    //         cmp(R5, 0x2B).ne(),
    //         b(progressive_sword).ne(),
    //         ldr(R0, (R0, 0x490)),
    //         cmp(R0, 0),
    //         mov(R5, 0x2A).eq(),
    //         mov(R5, 0x2B).ne(),
    //         b(return_label),
    //     ])
    // };
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
    let progressive_net = code.text().define([
        cmp(R5, 0x30),
        b(progressive_lamp).ne(),
        ldr(R0, (R0, 0x468)),
        cmp(R0, 0),
        mov(R5, 0x30).eq(),
        mov(R5, 0x59).ne(),
        b(return_label),
    ]);
    code.patch(0x2922A0, [b(progressive_net)]);
}

fn bracelet(code: &mut Code) {
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
    let get_item_value = 0x55696C;
    let is_ring_hekiga = code.text().define([
        push([LR]),
        ldr(R0, 0x70FB60),
        ldr(R0, (R0, 0)),
        mov(R1, 0x17),
        bl(get_item_value),
        cmp(R0, 3),
        mov(R0, 1).eq(),
        mov(R0, 0).ne(),
        pop([PC]),
    ]);
    code.patch(0x1DCA8C, [bl(is_ring_hekiga)]);
    code.patch(0x4266D0, [bl(is_ring_hekiga)]);
    code.patch(0x52E654, [bl(is_ring_hekiga)]);
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
    let mut map = array::IntoIter::new(ACTOR_NAME_OFFSETS).collect::<HashMap<_, _>>();
    map.extend(array::IntoIter::new(ACTOR_NAMES).map(|(item, name)| {
        let name = format!("{}\0", name);
        (item, code.rodata().declare(name.as_bytes()))
    }));
    map
}

fn item_names(code: &mut Code) -> HashMap<Item, u32> {
    let mut map = array::IntoIter::new(ITEM_NAME_OFFSETS).collect::<HashMap<_, _>>();
    map.extend(array::IntoIter::new(ITEM_NAMES).map(|(item, name)| {
        let name = format!("item_name_{}\0", name);
        (item, code.rodata().declare(name.as_bytes()))
    }));
    map
}

const ACTOR_NAME_OFFSETS: [(Item, u32); 29] = [
    (Item::ItemStoneBeauty, 0x5D2060),
    (Item::RupeeR, 0x5D639C),
    (Item::RupeeG, 0x5D639C),
    (Item::RupeeB, 0x5D639C),
    (Item::RupeePurple, 0x5D639C),
    (Item::RupeeSilver, 0x5D63A4),
    (Item::KeySmall, 0x5D6580),
    (Item::ItemIceRod, 0x5D6AFC),
    (Item::ItemSandRod, 0x5D6B08),
    (Item::ItemTornadeRod, 0x5D6B18),
    (Item::ItemBomb, 0x5D6B28),
    (Item::ItemFireRod, 0x5D6B30),
    (Item::ItemHookShot, 0x5D6B40),
    (Item::ItemBoomerang, 0x5D6B50),
    (Item::ItemHammer, 0x5D6B60),
    (Item::ItemBow, 0x5D6B6C),
    (Item::ItemShield, 0x5D6B78),
    (Item::ItemBottle, 0x5D7048),
    (Item::HintGlasses, 0x5D70AC),
    (Item::RupeeGold, 0x5D7144),
    (Item::ItemSwordLv2, 0x5D7178),
    (Item::LiverPurple, 0x5D762C),
    (Item::LiverYellow, 0x5D7640),
    (Item::LiverBlue, 0x5D7654),
    (Item::MessageBottle, 0x5D76A0),
    (Item::Pouch, 0x5D7734),
    (Item::ItemBowLight, 0x5D776C),
    (Item::HeartContainer, 0x5D7B7C),
    (Item::HeartPiece, 0x5D7B94),
];

const ACTOR_NAMES: [(Item, &str); 17] = [
    (Item::KeyBoss, "KeyBoss"),
    (Item::Compass, "Compass"),
    (Item::ItemKandelaar, "GtEvKandelaar"),
    (Item::ItemMizukaki, "GtEvFin"),
    (Item::RingHekiga, "RingRental"),
    (Item::ItemBell, "GtEvBell"),
    (Item::PowerGlove, "GtEvGloveA"),
    (Item::ItemInsectNet, "GtEvNet"),
    (Item::BadgeBee, "BadgeBee"),
    (Item::ClothesBlue, "GtEvCloth"),
    (Item::HyruleShield, "GtEvShieldB"),
    (Item::OreYellow, "OreSword"),
    (Item::OreGreen, "OreSword"),
    (Item::OreBlue, "OreSword"),
    (Item::GanbariPowerUp, "PowerUp"),
    (Item::DashBoots, "GtEvBoots"),
    (Item::OreRed, "OreSword"),
    //(Item::MilkMatured, "GtEvBottleMedicine"),
];

const ITEM_NAME_OFFSETS: [(Item, u32); 14] = [
    (Item::ItemBomb, 0x6F9A9A),
    (Item::ItemSandRod, 0x6F9AD0),
    (Item::ItemIceRod, 0x6F9AE2),
    (Item::ItemTornadeRod, 0x6F9AF3),
    (Item::ItemFireRod, 0x6F9B08),
    (Item::LiverPurple, 0x6F9B55),
    (Item::ItemBottle, 0x6F9B6C),
    (Item::LiverBlue, 0x6F9B94),
    (Item::ItemBoomerang, 0x6F9BA9),
    (Item::ItemHammer, 0x6F9CCC),
    (Item::ItemHookShot, 0x6F9CDD),
    (Item::ItemBow, 0x6F9D08),
    (Item::LiverYellow, 0x6F9D2F),
    (Item::ItemStoneBeauty, 0x6F9D56),
];

const ITEM_NAMES: [(Item, &str); 23] = [
    (Item::HeartContainer, "heartcontioner"),
    (Item::HeartPiece, "heartpiece"),
    (Item::ItemBell, "bell"),
    (Item::ItemBowLight, "bow_light"),
    (Item::RingHekiga, "bracelet"),
    (Item::ClothesBlue, "clothes_blue"),
    (Item::GanbariPowerUp, "ganbari_power_up"),
    (Item::ItemKandelaar, "lantern"),
    (Item::ItemSwordLv2, "mastersword"),
    (Item::MessageBottle, "messagebottle"),
    (Item::MilkMatured, "milk_matured"),
    (Item::ItemInsectNet, "net"),
    (Item::DashBoots, "pegasus"),
    (Item::Pouch, "pouch"),
    (Item::PowerGlove, "powergloves"),
    (Item::SpecialMove, "special_move"),
    (Item::BadgeBee, "beebadge"),
    (Item::ItemMizukaki, "web"),
    (Item::HyruleShield, "hyrule_shield"),
    (Item::OreYellow, "ore"),
    (Item::OreGreen, "ore"),
    (Item::OreBlue, "ore"),
    (Item::OreRed, "ore"),
];

const SET_EVENT_FLAG_FN: u32 = 0x4CDF40;
const GET_EVENT_FLAG_FN: u32 = 0x584B94;
const VTABLE_STRING: u32 = 0x6F5988;
const EVENT_FLAG_PTR: u32 = 0x70B728;

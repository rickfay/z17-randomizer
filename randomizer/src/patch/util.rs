#![allow(clippy::type_complexity)]

use super::DungeonPrizes;
use crate::filler::filler_item;
use crate::filler::filler_item::Randomizable;
use crate::filler::filler_item::Randomizable::Item;
use crate::{regions, Layout};
use macros::fail;
use rom::flag::Flag;
use rom::scene::{Obj, Rail, SpawnPoint, Vec3};

pub(crate) fn call<F>(unq: u16, action: F) -> (u16, Box<dyn Fn(&mut Obj)>)
where
    F: Fn(&mut Obj) + 'static,
{
    (unq, Box::new(action))
}

pub(crate) fn call_rail<F>(unq: u16, action: F) -> (u16, Box<dyn Fn(&mut Rail)>)
where
    F: Fn(&mut Rail) + 'static,
{
    (unq, Box::new(action))
}

pub(crate) fn set_46_args(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.set_active_flag(flag)))
}

#[allow(unused)]
pub(crate) fn set_57_args(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.set_inactive_flag(flag)))
}

pub(crate) fn enable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| obj.enable()))
}

pub(crate) fn disable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| obj.disable()))
}

pub(crate) fn set_enable_flag(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.set_enable_flag(flag)))
}

pub(crate) fn set_disable_flag(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.set_disable_flag(flag)))
}

pub(crate) fn set_flags(unq: u16, enable_flag: Flag, disable_flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (
        unq,
        Box::new(move |obj: &mut Obj| {
            obj.set_enable_flag(enable_flag);
            obj.set_disable_flag(disable_flag);
        }),
    )
}

pub(crate) fn clear_enable_flag(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.clear_enable_flag()))
}

pub(crate) fn clear_disable_flag(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.clear_disable_flag()))
}

#[allow(unused)]
pub(crate) fn clear_active_args(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.clear_active_args()))
}

#[allow(unused)]
pub(crate) fn clear_inactive_args(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.clear_inactive_args()))
}

pub(crate) fn redirect(unq: u16, sp: SpawnPoint) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.redirect(sp)))
}

pub(crate) fn add_rail(unq: u16, rail: (i32, i32)) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.ril.push(rail)))
}

#[allow(unused)]
pub(crate) fn remove_collision(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| obj.srt.scale = Vec3::ZERO))
}

pub fn is_sage(item: Randomizable) -> bool {
    use filler_item::Item::*;
    matches!(item, Item(SageGulley | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso))
}

pub(crate) fn is_pendant(item: Randomizable) -> bool {
    use filler_item::Item::*;
    matches!(item, Item(PendantOfPower | PendantOfWisdom | PendantOfCourage))
}

pub(crate) fn prize_flag(prize: Randomizable) -> Flag {
    use filler_item::Item::*;
    match prize {
        Item(PendantOfCourage) => Flag::EASTERN_COMPLETE,
        Item(PendantOfWisdom) => Flag::GALES_COMPLETE,
        Item(PendantOfPower) => Flag::HERA_COMPLETE,
        Item(SageGulley) => Flag::SAGE_GULLEY,
        Item(SageOren) => Flag::SAGE_OREN,
        Item(SageSeres) => Flag::SAGE_SERES,
        Item(SageOsfala) => Flag::SAGE_OSFALA,
        Item(SageRosso) => Flag::SAGE_ROSSO,
        Item(SageIrene) => Flag::SAGE_IRENE,
        Item(SageImpa) => Flag::SAGE_IMPA,
        prize => fail!("{} is not a Dungeon Prize", prize.as_str()),
    }
}

/// Fetch the placed Dungeon Rewards
/// <br />TODO really need to clean up the Layout data structure...
pub(crate) fn get_dungeon_prizes(layout: &Layout) -> DungeonPrizes {
    DungeonPrizes {
        ep_prize: layout.get_unsafe("[EP] Prize", regions::dungeons::eastern::palace::SUBREGION),
        hg_prize: layout.get_unsafe("[HG] Prize", regions::dungeons::house::gales::SUBREGION),
        th_prize: layout.get_unsafe("[TH] Prize", regions::dungeons::tower::hera::SUBREGION),
        pd_prize: layout.get_unsafe("[PD] Prize", regions::dungeons::dark::palace::SUBREGION),
        sp_prize: layout.get_unsafe("[SP] Prize", regions::dungeons::swamp::palace::SUBREGION),
        sw_prize: layout.get_unsafe("[SW] Prize", regions::dungeons::skull::woods::SUBREGION),
        tt_prize: layout.get_unsafe("[TT] Prize", regions::dungeons::thieves::hideout::SUBREGION),
        tr_prize: layout.get_unsafe("[TR] Prize", regions::dungeons::turtle::rock::SUBREGION),
        dp_prize: layout.get_unsafe("[DP] Prize", regions::dungeons::desert::palace::SUBREGION),
        ir_prize: layout.get_unsafe("[IR] Prize", regions::dungeons::ice::ruins::SUBREGION),
    }
}

use {
    crate::{patch::DungeonPrizes, regions, Layout, LocationInfo},
    albw::{
        scene::{Dest, Flag, Obj, Rail, Vec3},
        Item::{self, *},
    },
    macros::fail,
};

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

pub(crate) fn redirect(unq: u16, dest: Dest) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.redirect(dest)))
}

pub(crate) fn add_rail(unq: u16, rail: (i32, i32)) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| obj.ril.push(rail)))
}

#[allow(unused)]
pub(crate) fn remove_collision(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| obj.srt.scale = Vec3::ZERO))
}

pub fn is_sage(item: Item) -> bool {
    matches!(
        item,
        SageGulley | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso
    )
}

pub(crate) fn is_pendant(item: Item) -> bool {
    matches!(item, PendantPower | PendantWisdom | PendantCourage | ZeldaAmulet)
}

pub(crate) fn prize_flag(prize: Item) -> Flag {
    match prize {
        PendantPower => Flag::Event(372),
        PendantWisdom => Flag::Event(342),
        PendantCourage => Flag::Event(251),
        SageGulley => Flag::Event(536),
        SageOren => Flag::Event(556),
        SageSeres => Flag::Event(576),
        SageOsfala => Flag::Event(596),
        SageRosso => Flag::Event(616),
        SageIrene => Flag::Event(636),
        SageImpa => Flag::Event(656),
        _ => fail!("{} is Charm or not a Dungeon Prize", prize.as_str()),
    }
}

/// Fetch the placed Dungeon Rewards
/// <br />TODO really need to clean up the Layout data structure...
pub(crate) fn get_dungeon_prizes(layout: &Layout) -> DungeonPrizes {
    DungeonPrizes {
        ep_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::eastern::palace::SUBREGION,
                "Eastern Palace Prize",
            ))
            .unwrap(),
        hg_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::house::gales::SUBREGION,
                "House of Gales Prize",
            ))
            .unwrap(),
        th_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::tower::hera::SUBREGION,
                "Tower of Hera Prize",
            ))
            .unwrap(),
        hc_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::hyrule::castle::SUBREGION,
                "Hyrule Castle Prize",
            ))
            .unwrap(),
        pd_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::dark::palace::SUBREGION,
                "Dark Palace Prize",
            ))
            .unwrap(),
        sp_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::swamp::palace::SUBREGION,
                "Swamp Palace Prize",
            ))
            .unwrap(),
        sw_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::skull::woods::SUBREGION,
                "Skull Woods Prize",
            ))
            .unwrap(),
        tt_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::thieves::hideout::SUBREGION,
                "Thieves' Hideout Prize",
            ))
            .unwrap(),
        tr_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::turtle::rock::SUBREGION,
                "Turtle Rock Prize",
            ))
            .unwrap(),
        dp_prize: layout
            .get(&LocationInfo::new(
                regions::dungeons::desert::palace::SUBREGION,
                "Desert Palace Prize",
            ))
            .unwrap(),
        ir_prize: layout
            .get(&LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "Ice Ruins Prize"))
            .unwrap(),
    }
}

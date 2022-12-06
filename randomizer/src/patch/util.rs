use albw::Item;
use albw::Item::*;
use albw::scene::{Flag, Obj, Rail, Vec3};

pub(crate) fn call<F>(unq: u16, action: F) -> (u16, Box<dyn Fn(&mut Obj)>)
    where
        F: Fn(&mut Obj) + 'static
{
    (unq, Box::new(action))
}

pub(crate) fn call_rail<F>(unq: u16, action: F) -> (u16, Box<dyn Fn(&mut Rail)>)
    where
        F: Fn(&mut Rail) + 'static
{
    (unq, Box::new(action))
}

pub(crate) fn set_46_args(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_active_flag(flag) }))
}

#[allow(unused)]
pub(crate) fn set_57_args(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_inactive_flag(flag) }))
}

pub(crate) fn enable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.enable() }))
}

pub(crate) fn disable(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.disable() }))
}

pub(crate) fn set_enable_flag(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_enable_flag(flag) }))
}

pub(crate) fn set_disable_flag(unq: u16, flag: Flag) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_disable_flag(flag) }))
}

#[allow(unused)]
pub(crate) fn clear_enable_flag(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_enable_flag(None) }))
}

pub(crate) fn clear_disable_flag(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.set_disable_flag(None) }))
}

pub(crate) fn redirect(unq: u16, spawn_point: i32, scene: i32, scene_index: i32) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.redirect(spawn_point, scene, scene_index) }))
}

pub(crate) fn add_rail(unq: u16, rail: (i32, i32)) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(move |obj: &mut Obj| { obj.ril.push(rail) }))
}

pub(crate) fn remove_collision(unq: u16) -> (u16, Box<dyn Fn(&mut Obj)>) {
    (unq, Box::new(|obj: &mut Obj| { obj.srt.scale = Vec3 { x: 0.0, y: 0.0, z: 0.0 } }))
}

pub(crate) fn is_sage(item: Item) -> bool {
    match item {
        SageGulley | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso => true,
        _ => false
    }
}

pub(crate) fn is_pendant(item: Item) -> bool {
    match item {
        PendantPower | PendantWisdom | PendantCourage => true,
        _ => false
    }
}

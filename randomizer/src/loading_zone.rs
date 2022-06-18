use albw::course::Id;

pub struct LoadingZone {
    scene: Id,
    scene_index: u16,
    unq: u16,
    arg0: u16,
    arg10: u16,
    arg11: u16,
}

impl LoadingZone {
    pub fn new(scene: Id, scene_index: u16, unq: u16, arg0: u16, arg10: u16, arg11: u16) -> Self {
        Self { scene, scene_index, unq, arg0, arg10, arg11 }
    }
}
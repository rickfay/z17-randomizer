use crate::loading_zone::LoadingZone;

pub struct LoadingZonePair {
    entrance: LoadingZone,
    exit: LoadingZone,
}

impl LoadingZonePair {
    pub fn new(entrance: LoadingZone, exit: LoadingZone) -> Self {
        Self { entrance, exit }
    }
}
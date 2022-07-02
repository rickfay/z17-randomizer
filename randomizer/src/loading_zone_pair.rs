use crate::loading_zone::LoadingZone;

#[allow(dead_code)]
pub struct LoadingZonePair {
    entrance: LoadingZone,
    exit: LoadingZone,
}

impl LoadingZonePair {
    #[allow(dead_code)]
    pub fn new(entrance: LoadingZone, exit: LoadingZone) -> Self {
        Self { entrance, exit }
    }
}
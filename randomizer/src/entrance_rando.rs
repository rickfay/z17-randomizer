use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum EntranceShuffleSetting {
    NotShuffled, // Entrances are not shuffled

    Shuffled,
    // Entrances are shuffled within their own world
    PortalShuffle, // Portals are shuffled (except Zaganaga)

    CrossShuffle,
    // Entrances are shuffled Between Worlds (lord)
    CrossPortalsanity, // Entrances and Portals are both shuffled within their categories
}

impl Default for EntranceShuffleSetting {
    fn default() -> Self {
        Self::NotShuffled
    }
}

// pub fn lz_lut() -> (&'static mut HashMap<LoadingZoneId, LoadingZone>, &'static mut HashMap<LoadingZoneId, LoadingZone>) {
//     let lz_lut = (&mut HashMap::new(), &mut HashMap::new());
//
//     let he = LoadingZones::hyrule_entrances();
//     lz_lut.0.extend(he.0);
//     lz_lut.1.extend(he.1);
//
//     let le = LoadingZones::lorule_entrances();
//     lz_lut.0.extend(le.0);
//     lz_lut.1.extend(le.1);
//
//     let pe = LoadingZones::portals();
//     lz_lut.0.extend(pe.0);
//     lz_lut.1.extend(pe.1);
//
//     lz_lut
// }
//
// pub fn generate_door_map(rng: &mut StdRng, settings: &Settings) -> HashMap<LoadingZoneId, Location> {
//
//     let door_map = HashMap::new();
//
//     let (hyrule_entrances, hyrule_exits) = LoadingZones::hyrule_entrances();
//     let (lorule_entrances, lorule_exits) = LoadingZones::lorule_entrances();
//     let (hyrule_portals, lorule_portals) = LoadingZones::portals();
//
//     for (id, lz) in entries {
//
//     }
//
//     randomize_loading_zone_group(rng, vec![LoadingZones::HYRULE_DOORS]);
//
//     door_map
// }
//
//
// fn randomize_loading_zone_group(rng: &mut StdRng, lz_groups: Vec<&[(LoadingZone, LoadingZone)]>) -> Vec<(&LoadingZone, &LoadingZone)> {
//     let mut entrances = Vec::<&LoadingZone>::new();
//     let mut exits = Vec::<&LoadingZone>::new();
//
//     for lz_group in lz_groups {
//         for (entrance, exit) in lz_group {
//             entrances.push(&entrance);
//             exits.push(&exit);
//         }
//     }
//
//     exits = shuffle(exits, rng);
//
//     let mut map = Vec::<(&LoadingZone, &LoadingZone)>::new();
//     for (entrance, exit) in zip(entrances, exits) {
//         map.push((entrance, exit));
//     }
//
//     map
// }

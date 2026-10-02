use std::collections::BTreeMap;
use crate::VaneMap;
use crate::filler::item_pools;
use crate::filler::util::pair_randomly;
use log::info;
use modinfo::Settings;
use modinfo::settings::weather_vanes::WeatherVanes;
use rand::rngs::StdRng;
use modinfo::settings::DoorShuffle;
use crate::filler::filler_item::Vane;

/// Build the Weather Vane Map
pub fn build_vanes_map(settings: &Settings, rng: &mut StdRng) -> crate::Result<VaneMap> {
    info!("Building Weather Vane Map...");
    let mut weather_vanes_keys = item_pools::get_weather_vanes();
    match settings.weather_vanes {
        WeatherVanes::Shuffled => {

            let mut map: BTreeMap<_, _> = Default::default();

            if settings.door_shuffle == DoorShuffle::Crossed {
                weather_vanes_keys = weather_vanes_keys.iter().filter_map(|vane| {
                    match vane {
                        Vane::YourHouseWV | Vane::VacantHouseWV => {
                            map.insert(Vane::YourHouseWV, Vane::YourHouseWV);
                            map.insert(Vane::VacantHouseWV, Vane::VacantHouseWV);
                            None
                        },
                        _ => Some(*vane),
                    }
                }).collect();
            }

            map.extend(pair_randomly(rng, weather_vanes_keys)?);
            Ok(map)
        },
        _ => Ok(weather_vanes_keys.iter().copied().zip(item_pools::get_weather_vanes().iter().copied()).collect::<_>()),
    }
}

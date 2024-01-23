use crate::filler::item_pools;
use crate::filler::util::pair_randomly;
use crate::VaneMap;
use log::info;
use modinfo::settings::weather_vanes::WeatherVanes;
use modinfo::Settings;
use rand::rngs::StdRng;

/// Build the Weather Vane Map
pub fn build_vanes_map(settings: &Settings, rng: &mut StdRng) -> crate::Result<VaneMap> {
    info!("Building Weather Vane Map...");
    let weather_vanes_keys = item_pools::get_weather_vanes();
    match settings.weather_vanes {
        WeatherVanes::Shuffled => pair_randomly(rng, item_pools::get_weather_vanes()),
        _ => Ok(weather_vanes_keys
            .iter()
            .map(|&vane| vane)
            .zip(item_pools::get_weather_vanes().iter().map(|&vane| vane))
            .collect::<_>()),
    }
}

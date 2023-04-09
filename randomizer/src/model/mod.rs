use {crate::hints::PathHint, serde::Serialize, std::collections::HashMap};

pub mod check;
pub mod filler_item;
mod loading_zone_pair;
pub mod location;
pub mod location_node;
pub mod logic;
pub mod progress;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Hints {
    pub path_hints: Vec<PathHint>,
    pub always_hints: HashMap<&'static str, &'static str>,
    pub sometimes_hints: HashMap<&'static str, &'static str>,
    pub bow_of_light_hint: Vec<&'static str>,
}

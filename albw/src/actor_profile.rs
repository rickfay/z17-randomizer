use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// TODO.... make this work..


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActorProfile {
    pub collision: Vec<Collision>,
    pub general: HashMap<String, String>, // nope
    pub reaction: Vec<Reaction>,
    pub shadow: Vec<Shadow>,
}

impl ActorProfile {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Collision {
    pub scale: Vec<String>, // needs to be a dictionary
}

// #[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
// #[serde(rename_all = "PascalCase")]
// pub struct Type {
//     pub r#type: i32,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Scale {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


//noinspection SpellCheckingInspection
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct General {
    executable_distance: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_name: Option<String>,
    ground_color: u16,
    is_clip: u8,
    is_valid: u8,
    light_type: u16,
    mass_byte_size: u8,
    rotaion_scale: f32,
    rotate_north_scale: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reaction {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shadow {}
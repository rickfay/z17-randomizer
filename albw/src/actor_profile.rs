use {
    crate::{byaml, files::sarc::Sarc, File},
    serde::{Deserialize, Serialize},
};

// TODO.... make this work..

pub struct ActorProfiles {
    archive: File<Sarc>,
}

impl ActorProfiles {
    pub fn new(archive: File<Sarc>) -> Self {
        Self { archive }
    }

    pub fn contains(&self, actor_profile: &str) -> bool {
        self.archive.get().contains(format!("{}.byaml", actor_profile)).unwrap_or(false)
    }

    pub fn get_actor_profile(&mut self, profile: &str) -> File<ActorProfile> {
        self.archive
            .get()
            .read(format!("{}.byaml", profile))
            .unwrap()
            .try_map(|data| byaml::from_bytes(&data))
            .unwrap()
    }

    pub fn into_archive(self) -> File<Sarc> {
        self.archive.map(Sarc::compress)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActorProfile {
    pub collision: Vec<Collision>,
    pub general: General,
    pub reaction: Option<Vec<Reaction>>,
    pub shadow: Option<Vec<Shadow>>,
}

impl ActorProfile {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Collision {
    pub scale: Scale,
    pub r#type: u8,
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

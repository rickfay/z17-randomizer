use {
    crate::byaml::Vec3,
    regex::Regex,
    serde::{self, Deserialize, Serialize},
    std::io::{Error, ErrorKind},
};

/// DTO for `World/Byaml/GetItem.byaml`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetItem(
    pub String,
    pub String,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub String,
    pub String,
    pub String,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
);

impl GetItem {
    ///
    pub fn actor_name(&self) -> Result<&str, Error> {
        Regex::new(r"^Actor/([A-Za-z]+)\.bch$")
            .unwrap()
            .captures(&self.1)
            .and_then(|captures| captures.get(1))
            .map(|match_| match_.as_str())
            .ok_or_else(|| {
                Error::new(ErrorKind::NotFound, format!("Invalid actor name: '{}'", &self.1))
            })
    }

    ///
    pub fn get_scale_factor(&self) -> f32 {
        self.2
    }

    ///
    pub fn get_rotate(&self) -> Vec3 {
        Vec3 { x: self.9, y: self.10, z: self.11 }
    }
}

impl From<Vec<u8>> for GetItem {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for GetItem {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

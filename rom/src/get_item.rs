use std::sync::OnceLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{actors::Actor, scene::Vec3, Error, Result, Rom};

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
    pub fn actor(&self, game: &Rom) -> Option<Actor> {
        if self.1.is_empty() {
            None
        } else {
            Some(game.get_item_actor(self.actor_name().unwrap()).unwrap())
        }
    }

    pub fn actor_name(&self) -> Result<&str> {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        REGEX
            .get_or_init(|| Regex::new(r"^Actor/([A-Za-z]+)\.bch$").unwrap())
            .captures(&self.1)
            .and_then(|captures| captures.get(1))
            .map(|match_| match_.as_str())
            .ok_or_else(|| Error::new(format!("Invalid actor name: '{}'", &self.1)))
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.2
    }

    pub fn get_rotate(&self) -> Vec3 {
        Vec3 { x: self.9, y: self.10, z: self.11 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod actor_name {
        use super::*;

        #[test]
        fn it_works() -> Result<()> {
            let get_item = GetItem(
                String::new(),
                "Actor/Test.bch".to_string(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                String::new(),
                String::new(),
                String::new(),
                0,
                0,
                0,
                0,
                0,
            );
            assert_eq!(get_item.actor_name()?, "Test");
            Ok(())
        }
    }
}

use {
    crate::{actors::Actor, scene::Vec3, Error, Result, Rom},
    once_cell::sync::Lazy,
    regex::Regex,
    serde::{Deserialize, Serialize},
};

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
    /// Effects: light, twinkle, Pendant, RingRental, GtEvBottleMedicine, RupeeGold, RupeeSilver, GtEvNet, wait_get, fly, Bee
    pub String,
    pub String,
    pub String,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    /// 0 = Gold/Red
    /// 1 = Silver/Blue
    /// 2 = Brown
    /// 3 = Great Spin
    pub i32,
);

impl GetItem {
    pub fn actor(&self, game: &Rom) -> Option<Actor> {
        if self.1.is_empty() {
            let thing = game.get_item_actor("KeyBoss").ok(); // fixme dirty hack for Quake
            thing
        } else {
            game.get_item_actor(self.actor_name().unwrap()).ok()
        }
    }

    pub fn actor_name(&self) -> Result<&str> {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Actor/([A-Za-z]+)\.bch$").unwrap());
        REGEX
            .captures(&self.1)
            .and_then(|captures| captures.get(1))
            .map(|match_| match_.as_str())
            .or(Some("KeyBoss")) // fixme dirty hack for Quake
            .ok_or(Error::new(format!("Invalid actor name: '{}'", &self.1)))
        // .ok_or_else(|| Error::new(format!("Invalid actor name: '{}'", &self.1)))
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.2
    }

    pub fn set_345(&mut self, t: Vec3) {
        self.3 = t.x;
        self.4 = t.y;
        self.5 = t.z;
    }

    pub fn set_678(&mut self, s: Vec3) {
        self.6 = s.x;
        self.7 = s.y;
        self.8 = s.z;
    }

    pub fn get_rotate(&self) -> Vec3 {
        Vec3 { x: self.9, y: self.10, z: self.11 }
    }

    pub fn set_rotate(&mut self, r: Vec3) {
        self.9 = r.x;
        self.10 = r.y;
        self.11 = r.z;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetItems(pub Vec<GetItem>);

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

use {
    crate::byaml::course::CourseId,
    serde::{de, ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer},
    std::fmt,
};

pub mod actor_profile;
pub mod course;
pub mod get_item;
pub mod stage;
pub mod flow_chart;

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    React(u16),   // 0 - Reactions with system objects, not persisted
    Session(u16), // 1 - Flag persists until game is reset (I think?)
    Two(u16),     // 2 - ???
    Course(u16),  // 3 - Course-specific, shared between scenes of the same course
    Event(u16),   // 4 - Global
}

impl Flag {
    pub fn get_type(self) -> u8 {
        match self {
            Flag::React(_) => 0,
            Flag::Session(_) => 1,
            Flag::Two(_) => 2,
            Flag::Course(_) => 3,
            Flag::Event(_) => 4,
        }
    }

    pub fn get_value(self) -> u16 {
        match self {
            Flag::React(flag) => flag,
            Flag::Session(flag) => flag,
            Flag::Two(flag) => flag,
            Flag::Course(flag) => flag,
            Flag::Event(flag) => flag,
        }
    }

    pub fn into_pair(self) -> (u8, u16) {
        match self {
            Flag::React(flag) => (0, flag),
            Flag::Session(flag) => (1, flag),
            Flag::Two(flag) => (2, flag),
            Flag::Course(flag) => (3, flag),
            Flag::Event(flag) => (4, flag),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub struct Set {
    pub nme: Option<String>,
    pub pos: Vec<f32>,
}

/// Destination Coordinate
#[derive(Debug, Copy, Clone)]
pub struct Dest {
    pub scene: CourseId,
    pub scene_index: i32,
    pub spawn_point: i32,
}

impl Dest {
    pub fn new(scene: CourseId, scene_index: i32, spawn_point: i32) -> Self {
        Self { scene, scene_index, spawn_point }
    }
}

///
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub scale: Vec3,
    pub rotate: Vec3,
    pub translate: Vec3,
}

impl Transform {
    /// Adds the values of another Transform to this one
    pub fn add(&mut self, other: Transform) {
        self.scale.add(other.scale);
        self.rotate.add(other.rotate);
        self.translate.add(other.translate);
    }
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TransformVisitor;

        impl<'de> de::Visitor<'de> for TransformVisitor {
            type Value = Transform;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a nine-element tuple")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                Ok(Transform {
                    scale: Vec3 {
                        x: seq.next_element()?.ok_or_else(|| de::Error::missing_field("S.x"))?,
                        y: seq.next_element()?.ok_or_else(|| de::Error::missing_field("S.y"))?,
                        z: seq.next_element()?.ok_or_else(|| de::Error::missing_field("S.z"))?,
                    },
                    rotate: Vec3 {
                        x: seq.next_element()?.ok_or_else(|| de::Error::missing_field("R.x"))?,
                        y: seq.next_element()?.ok_or_else(|| de::Error::missing_field("R.y"))?,
                        z: seq.next_element()?.ok_or_else(|| de::Error::missing_field("R.z"))?,
                    },
                    translate: Vec3 {
                        x: seq.next_element()?.ok_or_else(|| de::Error::missing_field("T.x"))?,
                        y: seq.next_element()?.ok_or_else(|| de::Error::missing_field("T.y"))?,
                        z: seq.next_element()?.ok_or_else(|| de::Error::missing_field("T.z"))?,
                    },
                })
            }
        }

        deserializer.deserialize_tuple(9, TransformVisitor)
    }
}

impl Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple(9)?;
        s.serialize_element(&self.scale.x)?;
        s.serialize_element(&self.scale.y)?;
        s.serialize_element(&self.scale.z)?;
        s.serialize_element(&self.rotate.x)?;
        s.serialize_element(&self.rotate.y)?;
        s.serialize_element(&self.rotate.z)?;
        s.serialize_element(&self.translate.x)?;
        s.serialize_element(&self.translate.y)?;
        s.serialize_element(&self.translate.z)?;
        s.end()
    }
}

/// A 3D Vector
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Unit Vector
    pub const UNIT: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    /// Zero Vector
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    /// Adds the values of `other` to this Vec3
    pub fn add(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

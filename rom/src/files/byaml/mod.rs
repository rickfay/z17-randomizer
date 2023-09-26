pub mod actor_profile;
mod de;
pub mod scene_env;
mod ser;

pub use {
    de::{from_bytes, from_bytes_mut, Deserializer},
    ser::{to_writer, Serializer},
};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, ::serde_repr::Deserialize_repr, ::serde_repr::Serialize_repr,
)]
#[repr(u8)]
enum Kind {
    String = 0xA0,
    Array = 0xC0,
    Map = 0xC1,
    Strings = 0xC2,
    Boolean = 0xD0,
    Integer = 0xD1,
    Float = 0xD2,
    Null = 0xFF,
}

impl TryFrom<u8> for Kind {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xA0 => Ok(Self::String),
            0xC0 => Ok(Self::Array),
            0xC1 => Ok(Self::Map),
            0xC2 => Ok(Self::Strings),
            0xD0 => Ok(Self::Boolean),
            0xD1 => Ok(Self::Integer),
            0xD2 => Ok(Self::Float),
            0xFF => Ok(Self::Null),
            value => Err(crate::Error::new(format!(
                "Unrecognized value for type {}: {}",
                stringify!($type),
                value
            ))),
        }
    }
}

impl<'by> bytey::TryFromBytes<'by> for Kind {
    const SIZE: usize = <u8 as bytey::FromBytes>::SIZE;
    type Bytes = <u8 as bytey::FromBytes<'by>>::Bytes;

    fn try_from_bytes(bytes: &'_ Self::Bytes) -> ::bytey::Result<Self> {
        match <u8 as ::bytey::FromBytes>::from_bytes(bytes) {
            0xA0 => Ok(Self::String),
            0xC0 => Ok(Self::Array),
            0xC1 => Ok(Self::Map),
            0xC2 => Ok(Self::Strings),
            0xD0 => Ok(Self::Boolean),
            0xD1 => Ok(Self::Integer),
            0xD2 => Ok(Self::Float),
            0xFF => Ok(Self::Null),
            value => Err(bytey::Error::new(
                bytey::ErrorKind::InvalidData,
                format!("Unrecognized value for type {}: {}", stringify!($type), value),
            )),
        }
    }
}

const MAGIC: &[u8; 2] = b"YB";
const VERSION: u16 = 1;

#[cfg(test)]
mod tests {
    pub mod data {
        pub const ARRAY: &[u8] = &[
            b'Y', b'B', 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0,
            0xC0, 0x01, 0, 0, 0xD1, 0, 0, 0, 0x67, 0x45, 0x23, 0x01,
        ];
        pub const NESTED_ARRAY: &[u8] = &[
            b'Y', b'B', 0x01, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0,
            0xC0, 0x01, 0, 0, 0xC0, 0, 0, 0, 0x1C, 0x0, 0x0, 0x0, 0xC0, 0x01, 0x0, 0x0, 0xD1, 0, 0,
            0, 0x67, 0x45, 0x23, 0x01,
        ];
        pub const MAP: &[u8] = &[
            b'Y', b'B', 0x1, 0x0, 0x10, 0x0, 0x0, 0x0, 0x28, 0x0, 0x0, 0x0, 0x40, 0x0, 0x0, 0x0,
            0xC2, 0x02, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x14, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0, 0x0,
            b'b', b'a', b'r', 0, b'f', b'o', b'o', 0, 0xC2, 0x02, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0,
            0x14, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0, 0x0, b'B', b'a', b'r', 0, b'F', b'o', b'o', 0,
            0xC1, 0x02, 0x0, 0x0, 0x01, 0x0, 0x0, 0xA0, 0x1, 0x0, 0x0, 0x0, 0x00, 0x0, 0x0, 0xA0,
            0x0, 0x0, 0x0, 0x0,
        ];
    }
}

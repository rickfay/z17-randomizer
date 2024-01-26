pub use courses::Course;
pub use items::Item;
use std::error::Error as StdError;

mod courses;
pub mod ghosts;
mod items;
pub mod tower_stage;

/// An error resulting from trying to read the ROM file.
#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    fn new<T>(err: T) -> Self
    where
        T: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { inner: err.into() }
    }

    /// Converts this error into its inner error value.
    pub fn into_inner(self) -> Box<dyn StdError + Send + Sync + 'static> {
        self.inner
    }
}

/// Simple world enum
#[derive(Eq, PartialEq)]
pub enum World {
    Hyrule,
    Lorule,
}

#[doc(hidden)]
#[macro_export]
macro_rules! int_map {
    (
        $(#[$attr:meta])*
        $type:ident($repr:ident) {
            $(
                $(#[$attr_element:meta])*
                $variant:ident = $value:literal,
            )+
        }
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ::serde_repr::Deserialize_repr, ::serde_repr::Serialize_repr)]
        #[repr($repr)]
        pub enum $type {
            $(
                $(#[$attr_element])*
                $variant = $value,
            )+
        }

        impl $type {
            /// Iterates over all the variants of this enum.
            #[allow(unused)]
            pub fn iter() -> impl Iterator<Item = Self> {
                [$(Self::$variant,)+][..].into_iter().copied()
            }

            /// Returns the stringified name of the variant.
            #[allow(unused)]
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)+
                }
            }
        }

        impl ::core::convert::TryFrom<$repr> for $type {
            type Error = $crate::Error;

            fn try_from(value: $repr) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    value => Err($crate::Error::new(format!(
                        "Unrecognized value for type {}: {}",
                        stringify!($type),
                        value
                    ))),
                }
            }
        }

        impl<'by> ::bytey::TryFromBytes<'by> for $type {
            const SIZE: usize = <$repr as ::bytey::FromBytes>::SIZE;
            type Bytes = <$repr as ::bytey::FromBytes<'by>>::Bytes;

            fn try_from_bytes(bytes: &'_ Self::Bytes) -> ::bytey::Result<Self> {
                match <$repr as ::bytey::FromBytes>::from_bytes(bytes) {
                    $($value => Ok(Self::$variant),)+
                    value => Err(::bytey::Error::new(
                        ::bytey::ErrorKind::InvalidData,
                        format!(
                            "Unrecognized value for type {}: {}",
                            stringify!($type),
                            value
                        )
                    )),
                }
            }
        }
    };
}

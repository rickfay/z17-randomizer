//! A library for reading data from a The Legend of Zelda: A Link Between Worlds ROM.

use std::{
    cell::RefCell,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    fs, io,
    path::{Path, PathBuf},
};

use log::info;

mod actors;
pub mod course;
pub mod demo;
mod files;
pub mod flow;
mod item;
pub mod language;
pub mod scene;

use actors::{Actor, Actors};
pub use course::Course;
pub use demo::Demo;
pub use files::exheader::ExHeader;
use files::{byaml, romfs::RomFs, sarc::Sarc, Cxi};
pub use files::{File, IntoBytes};
use item::GetItem;
pub use item::Item;
pub use language::Language;
use language::{FlowChart, Load};
pub use scene::{Scene, Stage};

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

/// An error resulting from trying to read the ROM file.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    fn new<T>(err: T) -> Self
    where
        T: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self {
            kind: ErrorKind::Rom,
            inner: err.into(),
        }
    }

    /// Returns the type of this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Converts this error into its inner error value.
    pub fn into_inner(self) -> Box<dyn StdError + Send + Sync + 'static> {
        self.inner
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io,
            inner: err.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.kind {
            ErrorKind::Rom => None,
            ErrorKind::Io => Some(self.inner.as_ref()),
        }
    }
}

/// The kind of error contained within the `Error` type.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    /// An error within the provided ROM
    Rom,
    Io,
}

/// Game info, loaded from a ROM
#[derive(Debug)]
pub struct Game {
    path: PathBuf,
    id: u64,
    exheader: ExHeader,
    romfs: RefCell<RomFs<fs::File>>,
    flow_chart: File<FlowChart>,
    get_item: File<Vec<GetItem>>,
    message: File<Load>,
}

impl Game {
    /// Loads the game from a ROM.
    ///
    /// Fails if the ROM is invalid for any reason, including general
    /// corruption, mismatched IDs, invalid region, etc.
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        info!("Loading ROM from '{}'", path.display());
        let mut cxi = Cxi::open(&path)?;
        if cxi.id() == US_ID {
            let id = cxi.id();
            let exheader = cxi.exheader()?;
            let mut romfs = cxi.try_into_romfs()?;
            let region_boot = romfs.read("US/RegionBoot.szs")?.map(Sarc::from);
            let flow_chart = region_boot
                .get()
                .read("World/Byaml/FlowChart.byaml")?
                .try_map(|data| byaml::from_bytes(&data))?;
            let get_item = region_boot
                .get()
                .read("World/Byaml/GetItem.byaml")?
                .try_map(|data| byaml::from_bytes(&data))?;
            let message = region_boot
                .get()
                .read("World/Byaml/Message.byaml")?
                .try_map(|data| byaml::from_bytes(&data))?;
            Ok(Self {
                path,
                id,
                exheader,
                romfs: RefCell::new(romfs),
                flow_chart,
                get_item,
                message,
            })
        } else {
            Err(Error::new("Invalid ROM ID."))
        }
    }

    /// Gets the 64-bit title ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Gets the ROM's extended header.
    pub fn exheader(&self) -> &ExHeader {
        &self.exheader
    }

    pub fn get_item(&self) -> impl Iterator<Item = (Item, GetItem)> + '_ {
        Item::iter().zip(self.get_item.get()[1..].iter().cloned())
    }

    fn get_item_actor(&self, name: &str) -> Result<Actor> {
        self.romfs
            .borrow_mut()
            .read(format!("World/GetItem/{}.bch", name))
    }

    pub fn boot(&self) -> Result<Language> {
        let flow = self.flow_chart.get().load().boot()?.iter().cloned();
        let message = self.message.get().boot()?.iter().cloned();
        let archive = self
            .romfs
            .borrow_mut()
            .read("US/RegionBoot.szs")?
            .map(Sarc::from);
        Ok(Language::new(flow, message, archive))
    }

    pub fn common(&mut self) -> Result<Actors> {
        Ok(Actors::new(
            self.romfs
                .borrow_mut()
                .read("Archive/ActorCommon.szs")?
                .map(Sarc::from),
        ))
    }

    pub fn course(&self, id: course::Id) -> Course {
        Course::new(self, id)
    }

    pub fn demo(&self, index: u16) -> Result<File<Demo>> {
        self.romfs
            .borrow_mut()
            .read(format!("World/Demo/Demo{}.csv", index + 1))?
            .try_map(Demo::try_read)
    }

    pub(crate) fn language(&self, course: course::Id) -> Result<Language> {
        let flow = self
            .flow_chart
            .get()
            .load()
            .course(course)
            .unwrap_or_default()
            .iter()
            .cloned();
        let message = self
            .message
            .get()
            .course(course)
            .unwrap_or_default()
            .iter()
            .cloned();
        let archive = self
            .romfs
            .borrow_mut()
            .read(format!("US_English/{}.szs", course.as_str()))?
            .map(Sarc::from);
        Ok(Language::new(flow, message, archive))
    }

    pub(crate) fn scene(&self, course: course::Id, stage: u16) -> Result<Scene> {
        let name = format!("{}{}", course.as_str(), stage + 1);
        let mut romfs = self.romfs.borrow_mut();
        let stage = romfs
            .read(format!("World/Byaml/{}_stage.byaml", name))?
            .try_map(|data| byaml::from_bytes(&data))?;
        let actors = romfs.read(format!("Archive/{}.szs", name))?.map(Sarc::from);
        Ok(Scene::new(stage, actors))
    }

    pub(crate) fn stage(&self, course: course::Id, stage: u16) -> Result<Stage> {
        byaml::from_bytes(
            self.romfs
                .borrow_mut()
                .read(format!(
                    "World/Byaml/{}{}_stage.byaml",
                    course.as_str(),
                    stage + 1
                ))?
                .get(),
        )
    }
}

const US_ID: u64 = 0x00040000000EC300;

#[doc(hidden)]
#[macro_export]
macro_rules! int_map {
    ($(#[$attr:meta])*
    $type:ident($repr:ident) {
        $($variant:ident = $value:literal,)+
    }) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ::serde_repr::Deserialize_repr, ::serde_repr::Serialize_repr)]
        #[repr($repr)]
        pub enum $type {
            $($variant = $value,)+
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
            type Error = crate::Error;

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

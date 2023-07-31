//! A library for reading data from a The Legend of Zelda: A Link Between Worlds ROM.

use std::{
    cell::RefCell,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    fs, io,
    path::Path,
};

use game::Item;
use language::FlowChart;
use log::info;
use path_absolutize::*;
use strum::IntoEnumIterator;

use crate::{course::Id::LanguageBoot, language::Load, scene::SceneMeta};

pub use {
    actors::{Actor, Actors},
    course::Course,
    demo::Demo,
    files::{byaml, exheader::ExHeader, romfs::RomFs, sarc::Sarc, Cxi, File, IntoBytes},
    get_item::GetItem,
    language::Language,
    scene::{Scene, Stage},
};

pub mod actor_profile;
pub mod actors;
pub mod course;
pub mod demo;
mod files;
pub mod flow;
pub mod get_item;
pub mod language;
pub mod scene;

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
        Self { kind: ErrorKind::Rom, inner: err.into() }
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
        Self { kind: ErrorKind::Io, inner: err.into() }
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
#[allow(dead_code)]
pub struct Rom {
    id: u64,
    exheader: ExHeader,
    romfs: RefCell<RomFs<fs::File>>,
    flow_chart: File<FlowChart>,
    get_item: File<Vec<GetItem>>,
    message: File<Load>,
}

impl Rom {
    /// Loads the game from a ROM.
    ///
    /// Fails if the ROM is invalid for any reason, including general
    /// corruption, mismatched IDs, invalid region, etc.
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        info!("Loading ROM from:               {}", &path.absolutize()?.display());
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
            Ok(Self { id, exheader, romfs: RefCell::new(romfs), flow_chart, get_item, message })
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

    pub fn get_get_item(&mut self, item: Item) -> GetItem {
        self.get_item.get()[item as usize].clone()
    }

    pub fn get_items(&mut self) -> &mut File<Vec<GetItem>> {
        &mut self.get_item
    }

    pub fn dump_get_items(&mut self) -> File<Vec<GetItem>> {
        self.get_item.clone()
    }

    pub fn match_items_to_get_items(&self) -> impl Iterator<Item = (Item, GetItem)> + '_ {
        Item::iter().zip(self.get_item.get().iter().cloned())
    }

    pub fn open(&self, filename: &str) -> Vec<u8> {
        let file = self.romfs.borrow_mut().read(filename).unwrap();
        Vec::from(file.get().clone())
    }

    fn get_item_actor(&self, name: &str) -> Result<Actor> {
        self.romfs.borrow_mut().read(format!("World/GetItem/{}.bch", name))
    }

    pub fn get_player_item_actor(&self, name: &str) -> Result<Actor> {
        self.romfs.borrow_mut().read(format!("World/PlayerItem/{}.bch", name))
    }

    pub fn boot(&self) -> Result<Language> {
        let flow = self.flow_chart.get().load().boot()?.iter().cloned();
        let archive = self.romfs.borrow_mut().read("US/RegionBoot.szs")?.map(Sarc::from);
        Ok(Language::new(flow, archive))
    }

    pub fn common(&mut self) -> Result<Actors> {
        Ok(Actors::new(self.romfs.borrow_mut().read("Archive/ActorCommon.szs")?.map(Sarc::from)))
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

    pub fn language(&self, course: course::Id) -> Result<Language> {
        let flow = self.flow_chart.get().load().course(course).unwrap_or_default().iter().cloned();
        let archive = self
            .romfs
            .borrow_mut()
            .read(format!("US_English/{}.szs", course.as_ref()))?
            .map(Sarc::from);
        Ok(Language::new(flow, archive))
    }

    pub(crate) fn scene(&self, course: course::Id, stage: u16) -> Result<Scene> {
        let name = format!("{}{}", course.as_ref(), stage + 1);
        let mut romfs = self.romfs.borrow_mut();
        let stage = romfs
            .read(format!("World/Byaml/{}_stage.byaml", name))?
            .try_map(|data| byaml::from_bytes(&data))?;
        let actors = romfs.read(format!("Archive/{}.szs", name))?.map(Sarc::from);
        Ok(Scene::new(stage, actors))
    }

    pub(crate) fn scene_meta(&self, course: course::Id) -> Option<SceneMeta> {
        if LanguageBoot.eq(&course) {
            return None;
        }

        let mut romfs = self.romfs.borrow_mut();
        let stage_meta = romfs
            .read(format!("World/Byaml/{}_course.byaml", course.as_ref()))
            .unwrap()
            .try_map(|data| byaml::from_bytes(&data))
            .unwrap();
        Some(SceneMeta::new(stage_meta))
    }

    pub(crate) fn stage(&self, course: course::Id, stage: u16) -> Result<Stage> {
        byaml::from_bytes(
            self.romfs
                .borrow_mut()
                .read(format!("World/Byaml/{}{}_stage.byaml", course.as_ref(), stage + 1))?
                .get(),
        )
    }
}

const US_ID: u64 = 0x00040000000EC300;

#[macro_export]
macro_rules! string_constants {
    (
        $(#[$attr:meta])*
        $type:ident {
            $($variant:ident,)+
        }
    ) => {
        $(#[$attr])*
        pub struct $type;

        $(#[$attr])*
        impl $type {
            $(pub const $variant: &'static str = stringify!($variant);)+
        }
    }
}

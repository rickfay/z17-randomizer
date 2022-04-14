use bytey::*;
use data_encoding::HEXUPPER;
use log::{info, error};
use ring::digest::{Context, SHA256};
use serde::Serialize;
use std::{fs, io::{BufReader, prelude::*, stdin, stdout}, path::Path};

use crate::{Error, Result};

use self::{exheader::ExHeader, romfs::RomFs};

pub mod byaml;
pub mod exheader;
pub mod msgbn;
pub mod romfs;
pub mod sarc;

#[derive(Debug)]
pub struct Cxi<R> {
    file: R,
    id: u64,
    offset: u32,
}

impl<R> Cxi<R>
    where
        R: Read + Seek,
{
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn exheader(&mut self) -> Result<ExHeader> {
        Ok(ExHeader::read_from_offset(
            &mut self.file,
            self.offset + SIGNATURE_LEN as u32 + HEADER_LEN as u32,
        )?)
    }

    pub fn try_into_romfs(mut self) -> Result<RomFs<R>> {
        let media_units = u32::read_from_offset(&mut self.file, self.offset + 0x1B0)?;
        RomFs::load(self.file, self.offset + from_media_units(media_units))
    }
}

// FIXME unnecssary duplicate
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

impl Cxi<fs::File> {
    pub fn open<P>(path: P) -> Result<Self>
        where
            P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut file = match fs::File::open(path) {
            Ok(file) => file,
            Err(_) => {
                error!("Couldn't load ROM from: \"{}\"", path.display());
                error!("Please check that config.toml points to a valid ROM.");
                pause();
                std::process::exit(1);
            }
        };

        validate_rom(&file);

        bytey::typedef! { struct NCSD: TryFromBytes<'_> [HEADER_LEN] {
            #b"NCSD",
            [8] id: u64,
            [0x20] offset: u32,
        }}
        let header = NCSD::try_read_from_offset(&mut file, SIGNATURE_LEN)?;
        let offset = from_media_units(header.offset);
        bytey::typedef! { struct NCCH: TryFromBytes<'_> [HEADER_LEN] {
            #b"NCCH",
            [8] id: u64,
            [0x18] program_id: u64,
        }}
        let ncch = NCCH::try_read_from_offset(&mut file, offset + SIGNATURE_LEN as u32)?;
        cmp_id(ncch.id, header.id)?;
        cmp_id(ncch.program_id, header.id)?;
        Ok(Self {
            file,
            id: header.id,
            offset,
        })
    }
}

fn validate_rom(file: &fs::File) {
    //info!("Calculating Checksum...");

    let mut reader = BufReader::new(file);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let checksum = &HEXUPPER.encode(context.finish().as_ref());
    info!("SHA256 Checksum:                {}", checksum);

    // let valid_checksum = String::from("4071DC95F6948669C7A13D378509D5A224E167B77CF8FD2E163484BA9AF8B64D");
    // let encrypted_checksum = String::from("tbd"); // TODO determine this value
    //
    // if valid_checksum.eq(checksum) {
    //     info!("ROM is valid.");
    // } else {
    //     if encrypted_checksum.eq(checksum) {
    //         error!("ROM is encrypted. Please decrypt this ROM before using the randomizer.");
    //     } else {
    //         error!("Invalid checksum: {}", checksum);
    //         error!("ROM is invalid. Please provide a decrypted, North American ROM of The Legend of Zelda: A Link Between Worlds.");
    //     }
    //
    //     pause();
    //     std::process::exit(1);
    // }
}

#[derive(Clone, Debug)]
pub struct File<T> {
    path: String,
    inner: T,
}

impl<T> File<T> {
    fn new(path: String, inner: T) -> Self {
        Self { path, inner }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn rename(&mut self, name: String) {
        self.path = name;
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn map<F, U>(self, f: F) -> File<U>
        where
            F: FnOnce(T) -> U,
    {
        File::new(self.path, f(self.inner))
    }

    pub fn try_map<F, U>(self, f: F) -> Result<File<U>>
        where
            F: FnOnce(T) -> Result<U>,
    {
        Ok(File::new(self.path, f(self.inner)?))
    }
}

impl<T> File<T>
    where
        T: IntoBytes,
{
    pub fn into_bytes(self) -> File<Box<[u8]>> {
        File {
            path: self.path,
            inner: self.inner.into_bytes(),
        }
    }

    pub fn dump<P>(self, path: P) -> Result<()>
        where
            P: AsRef<Path>,
    {
        let path = path.as_ref().join(self.path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let bytes = self.inner.into_bytes();
        fs::write(path, bytes)?;
        Ok(())
    }
}

impl<T> File<T>
    where
        T: Serialize,
{
    pub fn serialize(self) -> File<Box<[u8]>> {
        let mut buf = vec![];
        byaml::to_writer(std::io::Cursor::new(&mut buf), &self.inner)
            .expect("Could not serialize.");
        File {
            path: self.path,
            inner: buf.into(),
        }
    }
}

impl<T> AsRef<T> for File<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

pub trait IntoBytes {
    fn into_bytes(self) -> Box<[u8]>;
}

impl IntoBytes for Box<[u8]> {
    fn into_bytes(self) -> Box<[u8]> {
        self
    }
}

impl IntoBytes for Vec<u8> {
    fn into_bytes(self) -> Box<[u8]> {
        self.into()
    }
}

impl IntoBytes for &[u8] {
    fn into_bytes(self) -> Box<[u8]> {
        self.into()
    }
}

impl IntoBytes for &mut [u8] {
    fn into_bytes(self) -> Box<[u8]> {
        (&*self).into()
    }
}

pub trait FromFile {
    type PathArgs: ?Sized;
    type Input;

    fn path(args: &Self::PathArgs) -> String;
    fn from_file(input: Self::Input) -> Result<Self>
        where
            Self: Sized;
}

fn cmp_id(left: u64, right: u64) -> Result<()> {
    if left == right {
        Ok(())
    } else {
        Err(Error::new("IDs did not match."))
    }
}

fn from_media_units(media_units: u32) -> u32 {
    media_units << MEDIA_UNIT_SHIFT
}

pub(crate) const fn align<const ALIGN: u32>(value: u32) -> u32 {
    let pad = ALIGN - 1;
    (value + pad) & !pad
}

const SIGNATURE_LEN: u64 = 0x100;
const HEADER_LEN: usize = 0x100;
const MEDIA_UNIT_SHIFT: u8 = 9;

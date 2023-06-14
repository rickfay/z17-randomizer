use {
    self::{exheader::ExHeader, romfs::RomFs},
    crate::{Error, Result},
    bytey::*,
    data_encoding::HEXUPPER,
    log::{error, info},
    ring::digest::{Context, SHA256},
    serde::Serialize,
    std::{
        fs,
        io::{prelude::*, stdin, stdout, BufReader},
        path::Path,
        process::exit,
    },
};
pub mod byaml;
pub mod msgbn;
pub mod sarc;



// FIXME unnecssary duplicate
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[derive(Clone, Debug)]
pub struct File<T> {
    path: String,
    inner: T,
}

impl<T> File<T> {
    pub fn new(path: String, inner: T) -> Self {
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
        File { path: self.path, inner: self.inner.into_bytes() }
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
        File { path: self.path, inner: buf.into() }
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
    if left == right { Ok(()) } else { Err(Error::new("IDs did not match.")) }
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

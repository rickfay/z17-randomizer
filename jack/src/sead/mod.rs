use crate::sead::sarc::Sarc;
use crate::sead::yaz0::{Compressed, Yaz0File};
use crate::IntoBytes;
use std::io::Error;

pub(crate) mod sarc;
mod yaz0;

/// Accepts the bytes of a compressed `szs_file` and attempts to decompress them and return the opened [`Sarc`]
/// archive. The `path` must be included, but will not be used to open the file.
pub(crate) fn open_szs(path: &str, szs_file: Box<[u8]>) -> Result<Sarc, Error> {
    Sarc::from(path, Yaz0File::<Compressed>::from(path, szs_file).decompress().into_bytes())
}

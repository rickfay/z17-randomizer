use crate::IntoBytes;

use self::yaz0::{Compressed, Yaz0File};
use sarc::Sarc;

pub mod sarc;
mod yaz0;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Sarc(#[from] sarc::Error),
    #[error(transparent)]
    Yaz0(#[from] ::yaz0::Error),
}

///
pub fn open_szs(path: &str, szs_file: Box<[u8]>) -> Result<Sarc> {
    let sarc_bytes = Yaz0File::<Compressed>::from(path, szs_file).decompress()?.into_bytes();
    Ok(Sarc::from(path, sarc_bytes)?)
}

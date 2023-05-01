use {
    crate::{
        sead::{
            sarc::Sarc,
            yaz0::{Compressed, Yaz0File},
        },
        IntoBytes,
    },
    std::io::Error,
};

pub(crate) mod sarc;
mod yaz0;

///
pub(crate) fn open_szs(path: &str, szs_file: Box<[u8]>) -> Result<Sarc, Error> {
    let sarc_bytes = Yaz0File::<Compressed>::from(path, szs_file).decompress().into_bytes();
    Sarc::from(path, sarc_bytes)
}

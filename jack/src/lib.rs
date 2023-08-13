use std::io;

use rom::Rom;

use sead::sarc::Sarc;

mod byaml;
pub mod sead;

/// Object-Safe Supertrait for all Jack Files
pub trait JackFile: Pathed + IntoBytes {}

/// Requires files know their own Path location on the filesystem
pub trait Pathed {
    fn get_path(&self) -> &str;
}

/// Requires files to be able to convert themselves into a byte representation
pub trait IntoBytes {
    fn into_bytes(self) -> Box<[u8]>
    where
        Self: Sized;
}

/// Opens an arbitrary file from the RomFS
pub fn open(game: &Rom, filename: &str) -> Box<[u8]> {
    game.open(filename).into()
}

/// Opens a Yaz0-compressed SARC Archive (.szs files)
pub fn open_szs(game: &Rom, filename: &str) -> io::Result<Sarc> {
    sead::open_szs(filename, open(game, filename))
}

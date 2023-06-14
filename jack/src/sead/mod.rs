use {
    crate::{
        sead::{
            sarc::SarcArchive,
            yaz0::{Compressed, Decompressed, Yaz0Data},
        },
        JackFile,
    },
    std::{
        io::{Error, ErrorKind},
        ops::{Deref, DerefMut},
    },
};

pub mod sarc;
mod yaz0;

/// SZS Files are Yaz0-compressed files containing SARC Archives that use the `.szs` suffix.
///
/// This struct acts like an API to the underlying [`SarcArchive`] and automatically handles the conversion between
/// Compressed and Decompressed [`Yaz0Data`].
pub struct SzsFile {
    archive: SarcArchive,
}

impl SzsFile {
    /// Opens a File from within the archive.
    pub fn open<T>(&self, filename: &str) -> Result<JackFile<T>, Error>
    where
        T: Into<Vec<u8>> + From<Vec<u8>>,
    {
        let file = self.archive.read(filename);
        if let Some(file) = file {
            Ok(JackFile::new(filename, T::from(file)))
        } else {
            Err(Error::new(ErrorKind::NotFound, filename))
        }
    }

    /// Adds a new unnamed [`JackFile`] to the archive. For the named variant, use [`SzsFile::add_named`].
    pub fn add<T>(&mut self, file: JackFile<T>)
    where
        T: Into<Vec<u8>> + From<Vec<u8>>,
    {
        self.archive.create(&file.path, file.file.into(), false);
    }

    /// Adds a new named [`JackFile`] to the archive. This is a special version of [`SzsFile::add`] for a few SZS files
    /// that bother to store the file's actual name (not just the hash).
    pub fn add_named<T>(&mut self, file: JackFile<T>)
    where
        T: Into<Vec<u8>> + From<Vec<u8>>,
    {
        self.archive.create(&file.path, file.file.into(), true);
    }

    /// Updates the contents of an already existing file within the archive.
    pub fn update<T>(&mut self, file: JackFile<T>)
    where
        T: Into<Vec<u8>> + From<Vec<u8>>,
    {
        self.archive.update(&file.path, file.file.into());
    }

    /// Deletes a file from the archive.
    pub fn delete(&mut self, filename: &str) {
        self.archive.delete(filename);
    }
}

impl Deref for SzsFile {
    type Target = SarcArchive;
    fn deref(&self) -> &Self::Target {
        &self.archive
    }
}

impl DerefMut for SzsFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.archive
    }
}

impl From<SarcArchive> for SzsFile {
    fn from(archive: SarcArchive) -> Self {
        Self { archive }
    }
}

impl From<Vec<u8>> for SzsFile {
    fn from(bytes: Vec<u8>) -> SzsFile {
        let bytes: Vec<u8> = Yaz0Data::<Compressed>::from(bytes).decompress().into();
        SzsFile::from(SarcArchive::from(bytes))
    }
}

impl Into<Vec<u8>> for SzsFile {
    fn into(self) -> Vec<u8> {
        let bytes: Vec<u8> = self.archive.into();
        Yaz0Data::<Decompressed>::from(bytes).compress(None).into()
    }
}

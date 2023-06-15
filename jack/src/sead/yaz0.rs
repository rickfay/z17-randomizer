use {
    crate::{IntoBytes, JackFile, Pathed},
    macros::fail,
    std::{io::Cursor, marker::PhantomData},
    yaz0::{CompressionLevel, Yaz0Archive, Yaz0Writer},
};

pub(crate) struct Compressed;
pub(crate) struct Decompressed;

/// Yaz0 Compressed File (usually .szs files)
pub(crate) struct Yaz0File<State = Compressed> {
    path: String,
    data: Box<[u8]>,
    state: PhantomData<State>,
}

impl JackFile for Yaz0File {}

impl Pathed for Yaz0File {
    fn get_path(&self) -> &str {
        &self.path
    }
}

impl<State> Yaz0File<State> {
    pub(crate) fn from(path: &str, data: Box<[u8]>) -> Yaz0File<State> {
        Yaz0File { path: path.to_owned(), data, state: PhantomData::<State> }
    }
}

impl<State> IntoBytes for Yaz0File<State> {
    fn into_bytes(self) -> Box<[u8]>
    where
        Self: Sized,
    {
        self.data
    }
}

impl Yaz0File<Compressed> {
    /// Perform the decompression
    pub(crate) fn decompress(self) -> Yaz0File<Decompressed> {
        let path = self.path.clone();
        let mut yaz0 = match Yaz0Archive::new(Cursor::new(self.into_bytes())) {
            Ok(yaz0) => yaz0,
            Err(err) => fail!("{}", err),
        };

        let decompressed = match yaz0.decompress() {
            Ok(decompressed) => decompressed,
            Err(err) => fail!("{}", err),
        };

        Yaz0File { path, data: decompressed.into(), state: PhantomData::<Decompressed> }
    }
}

impl Yaz0File<Decompressed> {
    /// Performs the Compression
    #[allow(unused)]
    pub(crate) fn compress<L>(self, level: L) -> Yaz0File<Compressed>
    where
        L: Into<Option<CompressionLevel>>,
    {
        let path = self.path.clone();
        let mut buffer = Vec::new();

        match level.into() {
            // Use "fake" compression that just intersperses markers throughout the data so it'll be readable by a
            // decompressor, without actually being compressed at all
            None => {
                // let bytes = &self.into_bytes();
                // let len = bytes.len();

                buffer.extend_from_slice(b"Yaz0");
                buffer.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
                buffer.extend_from_slice(&[0; 8]);
                let mut chunks = self.data.chunks_exact(8);
                while let Some(chunk) = chunks.next() {
                    buffer.push(0xFF);
                    buffer.extend_from_slice(chunk);
                }
                let remainder = chunks.remainder();
                let padding = 8 - remainder.len();
                if padding > 0 {
                    buffer.push(0xFF);
                    buffer.extend_from_slice(remainder);
                    buffer.resize(buffer.len() + padding, 0);
                }
            }
            // Real compression, takes a lot of time per file
            Some(level) => {
                Yaz0Writer::new(&mut buffer)
                    .compress_and_write(&self.into_bytes(), level)
                    .expect("Could not compress");
            }
        }

        Yaz0File { path, data: buffer.into(), state: PhantomData::<Compressed> }
    }
}

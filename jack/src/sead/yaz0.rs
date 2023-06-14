use {
    macros::fail,
    std::{io::Cursor, marker::PhantomData},
    yaz0::{CompressionLevel, Yaz0Archive, Yaz0Writer},
};

pub(crate) struct Compressed;
pub(crate) struct Decompressed;

/// Yaz0 Compressed Data
pub(crate) struct Yaz0Data<State = Compressed> {
    data: Vec<u8>,
    state: PhantomData<State>,
}

impl<State> From<Vec<u8>> for Yaz0Data<State> {
    fn from(data: Vec<u8>) -> Yaz0Data<State> {
        Yaz0Data { data, state: PhantomData::<State> }
    }
}

impl<State> Into<Vec<u8>> for Yaz0Data<State> {
    fn into(self) -> Vec<u8>
    where
        Self: Sized,
    {
        self.data
    }
}

impl Yaz0Data<Compressed> {
    /// Perform the decompression
    pub(crate) fn decompress(self) -> Yaz0Data<Decompressed> {
        let bytes: Vec<u8> = self.into();
        let mut yaz0 = match Yaz0Archive::new(Cursor::new(bytes)) {
            Ok(yaz0) => yaz0,
            Err(err) => fail!("{}", err),
        };

        let decompressed = match yaz0.decompress() {
            Ok(decompressed) => decompressed,
            Err(err) => fail!("{}", err),
        };

        Yaz0Data { data: decompressed.into(), state: PhantomData::<Decompressed> }
    }
}

impl Yaz0Data<Decompressed> {
    /// Performs the Compression
    pub(crate) fn compress<L>(self, level: L) -> Yaz0Data<Compressed>
    where
        L: Into<Option<CompressionLevel>>,
    {
        let mut buffer = Vec::new();

        match level.into() {
            // Use "fake" compression that just intersperses markers throughout the data so it'll be readable by a
            // decompressor, without actually being compressed at all
            None => {
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
                let bytes: Vec<u8> = self.into();
                Yaz0Writer::new(&mut buffer)
                    .compress_and_write(&bytes, level)
                    .expect("Could not compress");
            }
        }

        Yaz0Data { data: buffer.into(), state: PhantomData::<Compressed> }
    }
}

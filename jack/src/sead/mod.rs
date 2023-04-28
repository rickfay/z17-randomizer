use {
    macros::fail,
    std::{io::Cursor, marker::PhantomData},
    yaz0::Yaz0Archive,
};

///
pub struct SzsFile<State = Compressed> {
    data: SzsData,
    state: PhantomData<State>,
}

enum SzsData {
    Compressed(Box<[u8]>),
    Decompressed(SARC),
}

pub struct Compressed;
pub struct Decompressed;

impl From<Box<[u8]>> for SzsFile {
    fn from(data: Box<[u8]>) -> SzsFile<Compressed> {
        SzsFile { data: SzsData::Compressed(data), state: PhantomData::<Compressed> }
    }
}

impl SzsFile<Compressed> {
    fn data(&self) -> &Box<[u8]> {
        match &self.data {
            SzsData::Compressed(data) => data,
            SzsData::Decompressed(_) => unreachable!(),
        }
    }

    ///
    fn decompress(self) -> SzsFile<Decompressed> {
        let mut yaz0 = match Yaz0Archive::new(Cursor::new(self.data())) {
            Ok(yaz0) => yaz0,
            Err(err) => fail!("{}", err),
        };

        let decompressed = match yaz0.decompress() {
            Ok(decompressed) => decompressed,
            Err(err) => fail!("{}", err),
        };

        SzsFile {
            data: SzsData::Decompressed(SARC::from(decompressed)),
            state: PhantomData::<Decompressed>,
        }
    }

    ///
    fn dump(self) -> Box<[u8]> {
        todo!()
    }
}

impl SzsFile<Decompressed> {
    fn data(&self) -> &SARC {
        match &self.data {
            SzsData::Compressed(_) => unreachable!(),
            SzsData::Decompressed(data) => data,
        }
    }

    ///
    fn compress(self) -> SzsFile<Compressed> {
        todo!()
    }
}

///
struct SARC {
    data: Box<[u8]>,
}

impl<D: Into<Box<[u8]>>> From<D> for SARC {
    fn from(data: D) -> Self {
        Self { data: data.into() }
    }
}

///
#[allow(unused)]
pub fn get_szs_decompressed(raw: Box<[u8]>) -> SzsFile<Decompressed> {
    SzsFile::from(raw).decompress()
}

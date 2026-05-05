use {
    super::{align, File, FromFile, IntoBytes},
    crate::{Error, Result},
    bytey::*,
    log::debug,
    std::{
        cell::{Ref, RefCell},
        collections::BTreeMap,
        io::Cursor,
    },
};

#[derive(Debug)]
pub struct Sarc(RefCell<Inner>);

impl Sarc {
    pub fn from(data: Box<[u8]>) -> Self {
        Self(RefCell::new(Inner::Compressed(data)))
    }

    fn decompress(&self) -> Result<Ref<'_, Archive>> {
        self.0.borrow_mut().decompress()?;
        Ok(Ref::map(self.0.borrow(), |inner| match inner {
            Inner::Decompressed(archive) => archive,
            _ => unreachable!(),
        }))
    }

    fn decompress_mut(&mut self) -> Result<&mut Archive> {
        self.0.get_mut().decompress()
    }

    pub fn contains<P>(&self, path: P) -> Result<bool>
    where
        P: Into<String>,
    {
        let path = path.into();
        let archive = self.decompress()?;
        Ok(archive.get(&path).is_ok())
    }

    pub fn read<P>(&self, path: P) -> Result<File<Ref<'_, [u8]>>>
    where
        P: Into<String>,
    {
        let path = path.into();
        debug!("Reading {} from archive", &path);
        let archive = self.decompress()?;
        let data = Ref::filter_map(archive, |archive| archive.get(&path).ok())
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        Ok(File::new(path, data))
    }

    pub fn read_from_file<'a, T>(&'a self, args: &T::PathArgs) -> Result<File<T>>
    where
        T: FromFile<Input = Ref<'a, [u8]>>,
    {
        let path = T::path(args);
        debug!("Reading {} from archive", &path);
        let archive = self.decompress()?;
        let data = Ref::filter_map(archive, |archive| archive.get(&path).ok())
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        Ok(File::new(path, T::from_file(data)?))
    }

    pub fn extract<P>(&self, path: P) -> Result<File<Box<[u8]>>>
    where
        P: Into<String>,
    {
        let path = path.into();
        debug!("Extracting {} from archive", &path);
        let archive = self.decompress()?;
        let data = archive.get(&path)
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        Ok(File::new(path, data.into()))
    }

    pub fn open<P>(&mut self, path: P) -> Result<File<&mut [u8]>>
    where
        P: Into<String>,
    {
        let path = path.into();
        debug!("Opening {} from archive", &path);
        let data = self.decompress_mut()?.get_mut(&path)
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        Ok(File::new(path, data))
    }

    pub fn open_from_file<'s, T>(&'s mut self, args: &T::PathArgs) -> Result<File<T>>
    where
        T: FromFile<Input = &'s mut [u8]>,
    {
        let path = T::path(args);
        let data = self.decompress_mut()?.get_mut(&path)
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        Ok(File::new(path, T::from_file(data)?))
    }

    pub fn add(&mut self, file: File<Box<[u8]>>) -> Result<()> {
        self.decompress_mut()?.add(file);
        Ok(())
    }

    pub fn update(&mut self, file: File<Box<[u8]>>) -> Result<()> {
        self.decompress_mut()?.update(file);
        Ok(())
    }

    pub fn compress(self) -> Self {
        match self.0.into_inner() {
            Inner::Compressed(data) => Self::from(data),
            Inner::Decompressed(archive) => Self::from(compress(&archive.into_bytes())),
        }
    }
}

impl IntoBytes for Sarc {
    fn into_bytes(self) -> Box<[u8]> {
        match self.0.into_inner() {
            Inner::Compressed(data) => data,
            Inner::Decompressed(archive) => archive.into_bytes(),
        }
    }
}

impl IntoBytes for Ref<'_, [u8]> {
    fn into_bytes(self) -> Box<[u8]> {
        (&*self).into()
    }
}

#[derive(Debug)]
pub enum Inner {
    Compressed(Box<[u8]>),
    Decompressed(Archive),
}

impl Inner {
    fn decompress(&mut self) -> Result<&mut Archive> {
        if let Self::Compressed(data) = self {
            *self = Self::Decompressed(Archive::from(
                yaz0::Yaz0Archive::new(Cursor::new(&data))
                    .map_err(|_| Error::new("Archive could not be decompressed.".to_string()))?
                    .decompress()
                    .map_err(|_| Error::new("Archive could not be decompressed.".to_string()))?
                    .into(),
            )?);
        }
        match self {
            Self::Decompressed(archive) => Ok(archive),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Archive {
    multiplier: u32,
    files: BTreeMap<u32, Vec<u8>>,
}

impl Archive {
    pub fn from(file: Box<[u8]>) -> Result<Self> {
        typedef! { struct Header: TryFromBytes<'_> [0x14] {
            #b"SARC",
            [4] header_len: u16 where header_len == 0x14,
            [6] bom: u16 where bom == 0xFEFF,
            [8] len: u32,
            [0xC] offset: u32,
        }}
        let (header, sfat) = Header::try_from_slice(&file)?;
        if header.len as usize == file.len() {
            typedef! { struct SFAT: TryFromBytes<'_> [0xC] {
                #b"SFAT",
                [6] count: u16,
                [8] multiplier: u32,
            }}
            let (sfat, nodes) = SFAT::try_from_slice(sfat)?;
            let mut nodes: Vec<_> = nodes.into();
            nodes.truncate(0x10 * sfat.count as usize);
            let data = &file[header.offset as usize..];
            typedef! { struct Node: TryFromBytes<'_> [0x10] {
                [0] hash: u32,
                [4] attr: u32,
                [8] start: u32,
                [0xc] end: u32,
            }}
            let files = nodes
                .chunks(0x10)
                .map(|chunk| {
                    let (node, _) = Node::try_from_slice(chunk)?;
                    if node.attr != 0 {
                        Err(Error::new("Hash collision".to_string()))
                    } else {
                        let start = node.start as usize;
                        let end = node.end as usize;
                        Ok((node.hash, data[start..end].to_vec()))
                    }
                })
                .collect::<Result<_>>()?;
            Ok(Self { multiplier: sfat.multiplier, files })
        } else {
            Err(Error::new("unimpl113".to_string()))
        }
    }

    fn hash(&self, path: &str) -> u32 {
        path.chars().fold(0, |hash, ch| (ch as u32) + hash.wrapping_mul(self.multiplier))
    }

    fn get(&self, path: &str) -> Result<&[u8]> {
        self.files.get(&self.hash(path)).map(|file| &file[..]).ok_or(Error::new(format!("File not found: {path}")))
    }

    fn get_mut(&mut self, path: &str) -> Result<&mut [u8]> {
        self.files.get_mut(&self.hash(path)).map(|file| &mut file[..]).ok_or(Error::new(format!("File not found: {path}")))
    }

    fn update(&mut self, file: File<Box<[u8]>>) {
        debug!("Updating: {}", file.path);
        if self.files.contains_key(&self.hash(&file.path)) {
            self.files.insert(self.hash(&file.path), file.inner.into());
        } else {
            panic!("Can't update non-existent file: {}", file.path);
        }
    }

    fn add(&mut self, file: File<Box<[u8]>>) {
        debug!("Add {}", file.path);
        if self.files.contains_key(&self.hash(&file.path)) {
            debug!("Not adding duplicate file: {}", file.path);
        } else {
            self.files.insert(self.hash(&file.path), file.inner.into());
        }
    }
}

impl IntoBytes for Archive {
    fn into_bytes(self) -> Box<[u8]> {
        let count = self.files.len() as u16;
        let mut buf = vec![];
        buf.extend_from_slice(b"SARC");
        buf.extend_from_slice(&[0x14, 0, 0xFF, 0xFE]);
        buf.extend_from_slice(&[0, 0, 0, 0]);
        buf.extend_from_slice(&[0, 0, 0, 0]);
        buf.extend_from_slice(&[0, 1, 0, 0]);
        buf.extend_from_slice(b"SFAT");
        buf.extend_from_slice(&[0xC, 0]);
        buf.extend_from_slice(&count.to_le_bytes());
        buf.extend_from_slice(&self.multiplier.to_le_bytes());
        let mut offset = 0u32;
        for (hash, file) in &self.files {
            buf.extend_from_slice(&hash.to_le_bytes());
            buf.extend_from_slice(&[0, 0, 0, 0]);
            buf.extend_from_slice(&offset.to_le_bytes());
            offset = align::<0x80>(offset + file.len() as u32);
            buf.extend_from_slice(&offset.to_le_bytes());
        }
        buf.extend_from_slice(b"SFNT");
        buf.extend_from_slice(&[0x8, 0, 0, 0]);
        buf.resize(align::<0x80>(buf.len() as u32) as usize, 0);
        let file_offset = buf.len() as u32;
        buf[0xc..0x10].copy_from_slice(&file_offset.to_le_bytes());
        for (_, file) in &self.files {
            buf.extend_from_slice(&file);
            buf.resize(align::<0x80>(buf.len() as u32) as usize, 0);
        }
        let size = buf.len() as u32;
        buf[8..0xc].copy_from_slice(&size.to_le_bytes());
        buf.into()
    }
}

#[cfg(debug_assertions)]
fn compress(data: &[u8]) -> Box<[u8]> {
    let len = data.len() as u32;
    let mut buf = vec![];
    buf.extend_from_slice(b"Yaz0");
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(&[0; 8]);
    let mut chunks = data.chunks_exact(8);
    for chunk in chunks.by_ref() {
        buf.push(0xFF);
        buf.extend_from_slice(chunk);
    }
    let remainder = chunks.remainder();
    let padding = 8 - remainder.len();
    if padding > 0 {
        buf.push(0xFF);
        buf.extend_from_slice(remainder);
        buf.resize(buf.len() + padding, 0);
    }
    buf.into()
}

#[cfg(not(debug_assertions))]
fn compress(data: &[u8]) -> Box<[u8]> {
    let mut buf = vec![];
    yaz0::Yaz0Writer::new(&mut buf)
        .compress_and_write(&data, yaz0::CompressionLevel::Lookahead { quality: 1 })
        .expect("Yaz0 compression failed.");
    buf.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

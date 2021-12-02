use std::{
    cell::{Ref, RefCell},
    cmp::Ordering,
    io::Cursor,
};

use bytey::*;
use log::debug;

use super::{align, File, FromFile, IntoBytes};
use crate::{Error, Result};

#[derive(Debug)]
pub struct Sarc(RefCell<Inner>);

impl Sarc {
    pub fn from(data: Box<[u8]>) -> Self {
        Self(RefCell::new(Inner::Compressed(data)))
    }

    fn decompress(&self) -> Result<Ref<Archive>> {
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
        Ok(archive.find(&path).is_ok())
    }

    pub fn read<P>(&self, path: P) -> Result<File<Ref<[u8]>>>
    where
        P: Into<String>,
    {
        let path = path.into();
        debug!("Reading {} from archive", &path);
        let archive = self.decompress()?;
        let (start, end) = archive
            .find(&path)
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        let data = Ref::map(archive, |archive| {
            &archive.files[start as usize..end as usize]
        });
        Ok(File::new(path, data))
    }

    pub fn read_from_file<'a, T>(&'a self, args: &T::PathArgs) -> Result<File<T>>
    where
        T: FromFile<Input = Ref<'a, [u8]>>,
    {
        let path = T::path(args);
        debug!("Reading {} from archive", &path);
        let archive = self.decompress()?;
        let (start, end) = archive
            .find(&path)
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        let input = Ref::map(archive, |archive| {
            &archive.files[start as usize..end as usize]
        });
        Ok(File::new(path, T::from_file(input)?))
    }

    pub fn extract<P>(&self, path: P) -> Result<File<Box<[u8]>>>
    where
        P: Into<String>,
    {
        let path = path.into();
        debug!("Extracting {} from archive", &path);
        let archive = self.decompress()?;
        let data = archive.get(&path)?;
        Ok(File::new(path, data.into()))
    }

    pub fn open<P>(&mut self, path: P) -> Result<File<&mut [u8]>>
    where
        P: Into<String>,
    {
        let path = path.into();
        debug!("Opening {} from archive", &path);
        let data = self.decompress_mut()?.get_mut(&path)?;
        Ok(File::new(path, data))
    }

    pub fn open_from_file<'s, T>(&'s mut self, args: &T::PathArgs) -> Result<File<T>>
    where
        T: FromFile<Input = &'s mut [u8]>,
    {
        let path = T::path(args);
        let archive = self.decompress_mut()?;
        let (start, end) = archive
            .find(&path)
            .map_err(|_| Error::new(format!("File not found: '{}'.", path)))?;
        let input = &mut archive.files[start as usize..end as usize];
        Ok(File::new(path, T::from_file(input)?))
    }

    pub fn add(&mut self, file: File<Box<[u8]>>) -> Result<()> {
        self.decompress_mut()?.add(file);
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
    count: u16,
    multiplier: u32,
    nodes: Vec<u8>,
    files: Vec<u8>,
}

impl Archive {
    pub fn from(file: Box<[u8]>) -> Result<Self> {
        bytey::typedef! { struct Header: TryFromBytes<'_> [0x14] {
            #b"SARC",
            [4] header_len: u16 where header_len == 0x14,
            [6] bom: u16 where bom == 0xFEFF,
            [8] len: u32,
            [0xC] offset: u32,
        }}
        let (header, sfat) = Header::try_from_slice(&file)?;
        if header.len as usize == file.len() {
            bytey::typedef! { struct SFAT: TryFromBytes<'_> [0xC] {
                #b"SFAT",
                [6] count: u16,
                [8] multiplier: u32,
            }}
            let (sfat, nodes) = SFAT::try_from_slice(sfat)?;
            let mut nodes: Vec<_> = nodes.into();
            nodes.truncate(0x10 * sfat.count as usize);
            let files = file[header.offset as usize..].into();
            Ok(Self {
                count: sfat.count,
                multiplier: sfat.multiplier,
                nodes,
                files,
            })
        } else {
            Err(Error::new("unimpl113".to_string()))
        }
    }

    fn hash(&self, path: &str) -> u32 {
        path.chars().fold(0, |hash, ch| {
            (ch as u32) + hash.wrapping_mul(self.multiplier)
        })
    }

    fn get(&self, path: &str) -> Result<&[u8]> {
        if let Ok((start, end)) = self.find(path) {
            Ok(&self.files[start as usize..end as usize])
        } else {
            Err(Error::new("File not found."))
        }
    }

    fn get_mut(&mut self, path: &str) -> Result<&mut [u8]> {
        if let Ok((start, end)) = self.find(path) {
            Ok(&mut self.files[start as usize..end as usize])
        } else {
            Err(Error::new("File not found."))
        }
    }

    fn add(&mut self, file: File<Box<[u8]>>) {
        debug!("Add {}", file.path);
        match self.find(&file.path) {
            Ok(_) => {
                // TODO
            }
            Err(i) => {
                let i = i as usize * 0x10;
                let File { path, inner } = file;
                let hash = self.hash(&path);
                let mut buf: Vec<_> = inner.into();
                let start = align::<0x80>(self.files.len() as u32);
                self.files.resize(start as usize, 0);
                let end = start + buf.len() as u32;
                buf.resize(align::<0x80>(buf.len() as u32) as usize, 0);
                self.files.append(&mut buf);
                let mut node = vec![];
                node.extend_from_slice(&hash.to_le_bytes());
                node.extend_from_slice(&[0, 0, 0, 0]);
                node.extend_from_slice(&start.to_le_bytes());
                node.extend_from_slice(&end.to_le_bytes());
                self.nodes.splice(i..i, node);
                self.count += 1;
            }
        }
    }

    fn find(&self, path: &str) -> Result<(u32, u32), u16> {
        self.search(self.hash(path), 0, self.count - 1)
    }

    fn search(&self, hash: u32, start: u16, end: u16) -> Result<(u32, u32), u16> {
        if start <= end {
            let mid = (start + end) / 2;
            let index = (mid as usize) * 0x10;
            bytey::typedef! { struct Node: FromBytes<'_> [0x10] {
                [0] hash: u32,
                [8] start: u32,
                [0xC] end: u32,
            }}
            let node = unsafe { Node::from_slice_unchecked(&self.nodes[index..]) };
            match hash.cmp(&node.hash) {
                Ordering::Less => {
                    if mid == 0 {
                        Err(start)
                    } else {
                        self.search(hash, start, mid - 1)
                    }
                }
                Ordering::Equal => Ok((node.start, node.end)),
                Ordering::Greater => self.search(hash, mid + 1, end),
            }
        } else {
            Err(start)
        }
    }
}

impl IntoBytes for Archive {
    fn into_bytes(mut self) -> Box<[u8]> {
        let offset = (0x28 + self.nodes.len() as u32 + 0xFF) & !0xFF;
        let len = offset + self.files.len() as u32;
        let count = (self.nodes.len() / 0x10) as u16;
        let mut buf = vec![];
        buf.extend_from_slice(b"SARC");
        buf.extend_from_slice(&[0x14, 0, 0xFF, 0xFE]);
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(&offset.to_le_bytes());
        buf.extend_from_slice(&[0, 1, 0, 0]);
        buf.extend_from_slice(b"SFAT");
        buf.extend_from_slice(&[0xC, 0]);
        buf.extend_from_slice(&count.to_le_bytes());
        buf.extend_from_slice(&self.multiplier.to_le_bytes());
        buf.append(&mut self.nodes);
        buf.extend_from_slice(b"SFNT");
        buf.extend_from_slice(&[0x8, 0, 0, 0]);
        buf.resize(offset as usize, 0);
        buf.append(&mut self.files);
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
    while let Some(chunk) = chunks.next() {
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

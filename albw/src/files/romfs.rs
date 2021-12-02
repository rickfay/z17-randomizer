use std::io::prelude::*;

use bytey::*;

use super::File;
use crate::{Error, Result};

#[derive(Debug)]
pub struct RomFs<R> {
    file: R,
    directories: Section,
    files: Section,
    file_data: u32,
}

impl<R> RomFs<R>
where
    R: Read + Seek,
{
    pub fn load(mut file: R, offset: u32) -> Result<Self> {
        bytey::typedef! { struct IVFC: TryFromBytes<'_> [HEADER_LEN] {
            #b"IVFC",
            [0x4C] block_size: u32,
        }}
        let header = IVFC::try_read_from_offset(&mut file, offset)?;
        let block_size = 1 << header.block_size;
        let l3 = offset + block_size;
        bytey::typedef! { struct L3: FromBytes<'_> [L3_HEADER_LEN] {
            [4] directory: SectionHeader,
            [0x14] file: SectionHeader,
            [0x24] file_data: u32,
        }}
        let header = L3::read_from_offset(&mut file, l3)?;
        let directories = Section::read(&mut file, l3, header.directory)?;
        let files = Section::read(&mut file, l3, header.file)?;
        Ok(Self {
            file,
            directories,
            files,
            file_data: l3 + header.file_data,
        })
    }

    pub fn read<P>(&mut self, path: P) -> Result<File<Box<[u8]>>>
    where
        P: Into<String>,
    {
        let path = path.into();
        if path.ends_with('/') {
            panic!("Attempt to open a directory.")
        } else {
            let (parent, file) = {
                let path = path.strip_prefix('/').unwrap_or(&path);
                let mut split = path.rsplitn(2, '/');
                let file = split.next().expect("Attempt to read empty file name.");
                (self.find_dir(split.next(), 0)?, file)
            };
            let (offset, length) = self
                .find_file(parent, file)?
                .ok_or_else(|| Error::new(format!("File not found: '{}'.", path)))?;
            Ok(File::new(
                path,
                bytey::read_slice_from_offset(
                    &mut self.file,
                    self.file_data + offset as u32,
                    length as usize,
                )?,
            ))
        }
    }

    fn find_dir(&mut self, name: Option<&str>, parent: u32) -> Result<u32> {
        if let Some(path) = name {
            let mut split = path.splitn(2, '/');
            let dirname = split.next().unwrap();
            let child = split.next();
            let i = (hash(dirname, parent) % self.directories.count) as usize;
            let mut offset =
                unsafe { u32::from_slice_unchecked(&self.directories.hashtable[i * 4..]) };
            while offset != 0xFFFFFFFF {
                bytey::typedef! { struct Metadata: FromBytes<'_> [0x18] {
                    [0] parent: u32,
                    [0x10] next: u32,
                    [0x14] name_len: u32,
                }}
                let metadata =
                    Metadata::read_from_offset(&mut self.file, self.directories.metadata + offset)?;
                let name = bytey::read_slice(&mut self.file, metadata.name_len as usize)?;
                if metadata.parent == parent && path_eq(&name, dirname) {
                    return self.find_dir(child, offset);
                } else {
                    offset = metadata.next;
                }
            }
            Err(Error::new(format!("Directory not found: '{}'.", path)))
        } else {
            Ok(parent)
        }
    }

    fn find_file(&mut self, parent: u32, filename: &str) -> Result<Option<(u64, u64)>> {
        let i = (hash(filename, parent) % self.files.count) as usize;
        let mut offset = unsafe { u32::from_slice_unchecked(&self.files.hashtable[i * 4..]) };
        while offset != 0xFFFFFFFF {
            bytey::typedef! { struct Metadata: FromBytes<'_> [0x20] {
                [0] parent: u32,
                [8] offset: u64,
                [0x10] length: u64,
                [0x18] next: u32,
                [0x1C] name_len: u32,
            }}
            let metadata =
                Metadata::read_from_offset(&mut self.file, self.files.metadata + offset)?;
            let name = bytey::read_slice(&mut self.file, metadata.name_len as usize)?;
            if metadata.parent == parent && path_eq(&name, filename) {
                let offset = metadata.offset;
                let length = metadata.length;
                return Ok(Some((offset, length)));
            } else {
                offset = metadata.next;
            }
        }
        Ok(None)
    }
}

bytey::typedef! { struct SectionHeader: FromBytes<'_> [0x10] {
    [0] hashtable_offset: u32,
    [4] hashtable_len: u32,
    [8] metadata_offset: u32,
}}

#[derive(Debug)]
struct Section {
    hashtable: Box<[u8]>,
    count: u32,
    metadata: u32,
}

impl Section {
    fn read<R>(mut file: R, offset: u32, header: SectionHeader) -> Result<Self>
    where
        R: Read + Seek,
    {
        let hashtable = bytey::read_slice_from_offset(
            &mut file,
            offset + header.hashtable_offset,
            header.hashtable_len as usize,
        )?;
        let count = header.hashtable_len / 4;
        let metadata = offset + header.metadata_offset;
        Ok(Self {
            hashtable,
            count,
            metadata,
        })
    }
}

fn hash(name: &str, seed: u32) -> u32 {
    name.encode_utf16().fold(seed ^ 123456789, |hash, ch| {
        (hash.wrapping_shr(5) | hash.wrapping_shl(27)) ^ (ch as u32)
    })
}

fn path_eq(this: &[u8], other: &str) -> bool {
    this.chunks_exact(2)
        .map(|ch| unsafe { u16::from_slice_unchecked(ch) })
        .eq(other.encode_utf16())
}

const HEADER_LEN: usize = 0x5C;
const L3_HEADER_LEN: usize = 0x28;

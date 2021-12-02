use std::{mem::MaybeUninit, ops::Range};

use bytey::*;

use crate::{files::align, Error, Result};

type Ref<'file> = ::core::cell::Ref<'file, [u8]>;
type RefMut<'file> = &'file mut [u8];

type Section = ([u8; 4], Range<usize>);
type Sections<const COUNT: usize> = [Section; COUNT];

#[derive(Debug)]
pub struct MsgBn<T, const SECTIONS: usize> {
    file: T,
    sections: Sections<SECTIONS>,
}

impl<'file, const SECTIONS: usize> MsgBn<Ref<'file>, SECTIONS> {
    pub fn try_read(file: Ref<'file>, magic: &'static [u8; 8]) -> Result<Self> {
        let sections = sections::<SECTIONS>(&*file, magic)?;
        Ok(Self { file, sections })
    }

    pub fn get(&self, magic: &'static [u8; 4]) -> Option<Ref<'file>> {
        self.sections.iter().find_map(|(section_magic, section)| {
            (magic == section_magic).then(|| {
                Ref::map(Ref::clone(&self.file), |file| unsafe {
                    file.get_unchecked(section.clone())
                })
            })
        })
    }
}

impl<'file, const SECTIONS: usize> MsgBn<RefMut<'file>, SECTIONS> {
    pub fn try_read(file: RefMut<'file>, magic: &'static [u8; 8]) -> Result<Self> {
        let sections = sections::<SECTIONS>(&*file, magic)?;
        Ok(Self { file, sections })
    }

    pub fn into_section(self, magic: &'static [u8; 4]) -> Option<RefMut<'file>> {
        if let Some(section) = self
            .sections
            .iter()
            .find_map(|(section_magic, section)| (magic == section_magic).then(|| section))
        {
            unsafe { Some(self.file.get_unchecked_mut(section.clone())) }
        } else {
            None
        }
    }
}

pub fn sections<const SECTIONS: usize>(
    file: &[u8],
    magic: &'static [u8; 8],
) -> Result<Sections<SECTIONS>> {
    bytey::typedef! { struct Header<'h>: TryFromBytes<'h> [0x20] {
        [0] magic: &'h [u8; 8],
        [8] bom: u16 where bom == 0xFEFF,
        [0x12] size: u32,
    }}
    let mut sections = unsafe { MaybeUninit::<Sections<SECTIONS>>::zeroed().assume_init() };
    let (header, rest) = Header::try_from_slice(file)?;
    if header.magic == magic {
        if header.size as usize == file.len() {
            let mut rest = rest;
            let mut index = HEADER_LEN;
            for section_mut in sections.iter_mut() {
                bytey::typedef! { struct SectionHeader: FromBytes<'_> [0x20] {
                    [0] magic: [u8; 4],
                    [4] size: u32,
                }}
                let (header, section) = SectionHeader::try_from_slice(rest)?;
                let start = index + SECTION_HEADER_LEN;
                let aligned = align::<0x10>(header.size) as usize;
                let mid = start + aligned;
                if mid <= file.len() {
                    *section_mut = (header.magic, start..start + header.size as usize);
                    rest = &section[aligned..];
                    index = mid;
                } else {
                    return Err(Error::new("Ran out of data."));
                }
            }
            Ok(sections)
        } else {
            Err(Error::new("Size did not match."))
        }
    } else {
        Err(Error::new(format!(
                    "Error parsing file: expected magic number ({:X?}) did not match file's magic number ({:X?})",
                    magic,
                    header.magic,
                )))
    }
}

const HEADER_LEN: usize = 0x20;
const SECTION_HEADER_LEN: usize = 0x10;

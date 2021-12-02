use bytey::*;

/// The game's extended header.
#[derive(Clone, Debug)]
pub struct ExHeader([u8; LEN]);

impl ExHeader {
    pub fn get_text_address(&self) -> u32 {
        unsafe { u32::from_slice_unchecked(&self.0[0x10..]) }
    }

    pub fn get_text_size(&self) -> u32 {
        unsafe { u32::from_slice_unchecked(&self.0[0x18..]) }
    }

    pub fn set_text_size(&mut self, size: u32) {
        self.0[0x18..0x1C].copy_from_slice(&size.to_le_bytes());
    }

    pub fn get_rodata_address(&self) -> u32 {
        unsafe { u32::from_slice_unchecked(&self.0[0x20..]) }
    }

    pub fn get_rodata_size(&self) -> u32 {
        unsafe { u32::from_slice_unchecked(&self.0[0x28..]) }
    }

    pub fn set_rodata_size(&mut self, size: u32) {
        self.0[0x28..0x2C].copy_from_slice(&size.to_le_bytes());
    }
}

impl AsRef<[u8]> for ExHeader {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl FromBytes<'_> for ExHeader {
    const SIZE: usize = LEN;
    type Bytes = [u8; LEN];

    fn from_bytes(bytes: &Self::Bytes) -> Self {
        Self(FromBytes::from_bytes(bytes))
    }
}

pub const LEN: usize = 0x400;

#[cfg(test)]
mod tests {
    use bytey::*;

    use super::*;
    use crate::Result;

    fn new() -> ExHeader {
        let mut buf = [0u8; LEN];
        buf[0x10..0x14].copy_from_slice(&[0, 0, 0x10, 0]);
        ExHeader::from_bytes(&buf)
    }

    #[test]
    fn it_sets_text_size() -> Result<()> {
        let mut exheader = new();
        exheader.set_text_size(0x123456);
        assert_eq!(u32::try_from_slice(&exheader.0[0x18..0x1C])?.0, 0x123456);
        Ok(())
    }

    #[test]
    fn it_sets_rodata_size() -> Result<()> {
        let mut exheader = new();
        exheader.set_rodata_size(0x123456);
        assert_eq!(u32::try_from_slice(&exheader.0[0x28..0x2C])?.0, 0x123456);
        Ok(())
    }
}

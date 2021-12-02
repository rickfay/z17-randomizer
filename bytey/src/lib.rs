//! A library for dealing with reading binary data.

pub use std::io::{Error, ErrorKind, Result};
use std::{
    alloc,
    io::{prelude::*, SeekFrom},
    mem::{self, MaybeUninit},
    slice,
};

#[doc(hidden)]
pub use bytey_macro::strip_lifetime;

/// A trait for representing fixed-size sequence of bytes.
pub trait Bytes: AsRef<[u8]> + AsMut<[u8]> + Sized {
    /// The length of this sequence.
    const SIZE: usize;

    /// Attempts to convert part of a slice into this type.
    ///
    /// Fails if the slice is not long enough.
    fn from_slice(slice: &[u8]) -> Result<(&Self, &[u8])>;

    /// Converts a slice into the type without bounds checking.
    ///
    /// # Safety
    ///
    /// This function should not be used unless the size of the slice has already been checked.
    unsafe fn from_slice_unchecked(slice: &[u8]) -> &Self;
}

impl<const N: usize> Bytes for [u8; N] {
    const SIZE: usize = N;

    #[inline]
    fn from_slice(slice: &[u8]) -> Result<(&Self, &[u8])> {
        let buf = slice
            .get(0..N)
            .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "failed to fill whole buffer"))?;
        unsafe { Ok((Bytes::from_slice_unchecked(buf), slice.get_unchecked(N..))) }
    }

    #[inline]
    unsafe fn from_slice_unchecked(slice: &[u8]) -> &Self {
        let ptr = slice.as_ptr() as *const Self;
        &*ptr
    }
}

/// A trait for types that can be infallibly read from a borrowed fixed-size sequence of bytes.
pub trait FromBytes<'by>: Sized {
    /// The length of the byte sequence required.
    const SIZE: usize;
    type Bytes: Bytes;

    /// Reads this type from a byte sequence.
    fn from_bytes(bytes: &'by Self::Bytes) -> Self;

    /// Reads this type from a byte slice, without bounds-checking.
    ///
    /// # Safety
    ///
    /// This function should not be used unless the size of the slice is already known.
    unsafe fn from_slice_unchecked(slice: &'by [u8]) -> Self
    where
        Self: 'by,
    {
        Self::from_bytes(Bytes::from_slice_unchecked(slice))
    }
}

/// A trait for types that can be read from an owned sequence of bytes.
///
/// This trait is automatically implemented for any type that is `FromBytes` that does not have a lifetime.
pub trait FromBytesOwned: for<'by> FromBytes<'by> {
    #[inline]
    fn from_bytes_owned<const N: usize>(bytes: <Self as FromBytes>::Bytes) -> Self
    where
        Self: for<'by> FromBytes<'by, Bytes = [u8; N]>,
    {
        Self::from_bytes(&bytes)
    }

    /// Reads this type from a `Read` type, failing if there is not enough data.
    #[inline]
    fn read<R, const N: usize>(mut read: R) -> Result<Self>
    where
        R: Read,
        Self: for<'by> FromBytes<'by, Bytes = [u8; N]>,
    {
        let buf = MaybeUninit::<[u8; N]>::uninit();
        let mut buf = unsafe { buf.assume_init() };
        read.read_exact(buf.as_mut())?;
        Ok(Self::from_bytes_owned(buf))
    }

    /// Reads this type from a `Read` type with a provided offset.
    #[inline]
    fn read_from_offset<R, O, const N: usize>(mut read: R, offset: O) -> Result<Self>
    where
        R: Read + Seek,
        O: Into<u64>,
        Self: for<'by> FromBytes<'by, Bytes = [u8; N]>,
    {
        read.seek(SeekFrom::Start(offset.into()))?;
        Self::read(read)
    }
}

impl<T> FromBytesOwned for T where T: for<'by> FromBytes<'by> {}

/// A trait for types that can attempt to read from a borrowed fixed-size sequence of bytes.
pub trait TryFromBytes<'by>: Sized {
    /// The length of the byte sequence required.
    const SIZE: usize;
    type Bytes: Bytes;

    /// Attempts to read this type from a byte sequence.
    fn try_from_bytes(bytes: &'by Self::Bytes) -> Result<Self>;
}

/// A trait for types that can attempt to read from a slice.
///
/// This trait is automatically implemented for all types that implement `TryFromBytes`.
pub trait TryFromSlice<'by>: Sized {
    /// Attempts to read this type from a slice.
    fn try_from_slice(slice: &'by [u8]) -> Result<(Self, &'by [u8])>;

    /// Attempts to read this type from a slice without bounds-checking.
    ///
    /// # Safety
    ///
    /// This function should not be used unless the size of the slice is already known.
    unsafe fn try_from_slice_unchecked(slice: &'by [u8]) -> Result<Self>;
}

impl<'by, T, const N: usize> TryFromSlice<'by> for T
where
    T: TryFromBytes<'by, Bytes = [u8; N]>,
{
    #[inline]
    fn try_from_slice(slice: &'by [u8]) -> Result<(Self, &'by [u8])> {
        let (buf, rest) = Bytes::from_slice(slice)?;
        Ok((Self::try_from_bytes(buf)?, rest))
    }

    #[inline]
    unsafe fn try_from_slice_unchecked(slice: &'by [u8]) -> Result<Self> {
        Self::try_from_bytes(Bytes::from_slice_unchecked(slice))
    }
}

/// A trait for types that can attempt to read from an owned sequence of bytes.
///
/// This trait is automatically implemented for any type that is `TryFromBytes` that does not have a lifetime.
pub trait TryFromBytesOwned: for<'by> TryFromBytes<'by> {
    /// Attempts to read this type from a `Read` type.
    fn try_read<R, const N: usize>(mut read: R) -> Result<Self>
    where
        R: Read,
        Self: for<'by> TryFromBytes<'by, Bytes = [u8; N]>,
    {
        let buf = MaybeUninit::<[u8; N]>::uninit();
        let mut buf = unsafe { buf.assume_init() };
        read.read_exact(buf.as_mut())?;
        Self::try_from_bytes(&buf)
    }

    /// Attempts to read this type from a `Read` type with a provided offset.
    fn try_read_from_offset<R, O, const N: usize>(mut read: R, offset: O) -> Result<Self>
    where
        R: Read + Seek,
        O: Into<u64>,
        Self: for<'by> TryFromBytes<'by, Bytes = [u8; N]>,
    {
        read.seek(SeekFrom::Start(offset.into()))?;
        Self::try_read(read)
    }
}

impl<T> TryFromBytesOwned for T where T: for<'by> TryFromBytes<'by> {}

impl<'by, T, const N: usize> TryFromBytes<'by> for T
where
    T: FromBytes<'by, Bytes = [u8; N]>,
{
    const SIZE: usize = T::SIZE;
    type Bytes = <T as FromBytes<'by>>::Bytes;

    #[inline]
    fn try_from_bytes(bytes: &'by Self::Bytes) -> Result<Self> {
        Ok(Self::from_bytes(bytes))
    }
}

impl<const LEN: usize> FromBytes<'_> for [u8; LEN] {
    const SIZE: usize = LEN;
    type Bytes = [u8; LEN];

    #[inline]
    fn from_bytes(bytes: &Self::Bytes) -> Self {
        *bytes
    }
}

impl<'by, const LEN: usize> FromBytes<'by> for &'by [u8; LEN] {
    const SIZE: usize = LEN;
    type Bytes = [u8; LEN];

    #[inline]
    fn from_bytes(bytes: &'by Self::Bytes) -> Self {
        bytes
    }
}

impl<'by> TryFromSlice<'by> for &'by [u8] {
    #[inline]
    fn try_from_slice(slice: &'by [u8]) -> Result<(Self, &'by [u8])> {
        Ok((slice, &[]))
    }

    unsafe fn try_from_slice_unchecked(slice: &'by [u8]) -> Result<Self> {
        Ok(slice)
    }
}

macro_rules! impl_from_le_bytes {
    ($($type:ty)+) => {
        $(impl FromBytes<'_> for $type {
            const SIZE: usize = ::core::mem::size_of::<$type>();
            type Bytes = [u8; ::core::mem::size_of::<$type>()];

            #[inline]
            fn from_bytes(bytes: &Self::Bytes) -> Self {
                Self::from_le_bytes(*bytes)
            }
        })+
    };
}

impl_from_le_bytes!(f32 f64 i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

/// Defines a readable type with a simple interface.
///
/// Definitions for specific traits can be found on the respective trait pages.
#[macro_export]
macro_rules! typedef {
    ($vis:vis struct $type:ident$(<$lt:lifetime>)?: FromBytes<$trait_lt:lifetime> [$len:expr] {
        $([$index:literal] $property:ident: $kind:ty,)+
    }) => {
        $vis struct $type$(<$lt>)? {
            $($property: $kind,)+
        }

        impl$(<$lt>)? $crate::FromBytes<$trait_lt> for $type$(<$lt>)? {
            const SIZE: usize = $len;
            type Bytes = [u8; $len];

            fn from_bytes(bytes: &$trait_lt Self::Bytes) -> Self {
                $(let $property = {
                    const END: usize = $index + <$crate::strip_lifetime!($kind) as FromBytes>::SIZE;
                    const _: usize = $len - END;
                    unsafe {
                        <$kind as $crate::FromBytes>::from_bytes(
                            $crate::Bytes::from_slice_unchecked(&bytes[$index..END])
                        )
                    }
                };)+
                Self {
                    $($property,)+
                }
            }
        }
    };
    ($vis:vis struct $type:ident$(<$lt:lifetime>)?: TryFromBytes<$trait_lt:lifetime> [$len:expr] {
        $(#$magic:expr,)?
        $([$index:literal] $property:ident: $kind:ty $(where $where:expr)?,)+
    }) => {
        $vis struct $type$(<$lt>)? {
            $($property: $kind,)+
        }

        impl$(<$lt>)? $crate::TryFromBytes<$trait_lt> for $type$(<$lt>)? {
            const SIZE: usize = $len;
            type Bytes = [u8; $len];

            fn try_from_bytes(bytes: &$trait_lt Self::Bytes) -> $crate::Result<Self> {
                $({
                    const fn size_of_val<const N: usize>(_: &'static [u8; N]) -> usize {
                        N
                    }
                    unsafe fn get<'inner, const N: usize>(
                        bytes: &'inner [u8],
                        _: &'static [u8; N],
                    ) -> &'inner [u8; N] {
                        $crate::FromBytes::from_bytes($crate::Bytes::from_slice_unchecked(bytes))
                    }
                    const SIZE: usize = size_of_val($magic);
                    const _: usize = $len - SIZE;
                    let magic = unsafe { get(bytes.get_unchecked(0..SIZE), $magic)};
                    if magic == $magic {
                        Ok(())
                    } else {
                        Err($crate::Error::new(
                            $crate::ErrorKind::InvalidData,
                            "Did not match magic number."
                        ))
                    }
                }?;)?
                $(let $property = {
                    const END: usize = $index + <$crate::strip_lifetime!($kind) as $crate::TryFromBytes>::SIZE;
                    const _: usize = $len - END;
                    unsafe {
                        $crate::TryFromSlice::try_from_slice_unchecked(&bytes[$index..END])?
                    }
                };
                $(if !$where {
                    return Err($crate::Error::new(
                        $crate::ErrorKind::InvalidData,
                        "A predicate was failed."
                    ));
                })?)+
                Ok(Self {
                    $($property,)+
                })
            }
        }
    };
}

/// Attempts to read a slice with provided length from a `Read` type.
pub fn read_slice<R>(mut read: R, len: usize) -> Result<Box<[u8]>>
where
    R: Read,
{
    // TODO: replace with `Box::new_uninit_slice` once stable
    let mut buf = unsafe {
        mem::transmute::<_, Box<[u8]>>(slice::from_raw_parts(
            alloc::alloc(alloc::Layout::from_size_align(len, 1).unwrap()),
            len,
        ))
    };
    read.read_exact(&mut buf)?;
    Ok(buf)
}

/// Attempts to read a slice with provided length from a `Read` type, with a provided offset.
pub fn read_slice_from_offset<R, O>(mut read: R, offset: O, len: usize) -> Result<Box<[u8]>>
where
    R: Read + Seek,
    O: Into<u64>,
{
    read.seek(SeekFrom::Start(offset.into()))?;
    read_slice(read, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(u8::from_bytes(&[0]), 0);
        assert_eq!(i8::from_bytes(&[0xFF]), -1);
        assert!(u16::try_from_slice(&[0]).is_err());
    }
}

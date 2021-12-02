use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display, Formatter},
    num::NonZeroU32,
    str,
};

use bytey::*;
use serde::de::{
    self, value::BorrowedBytesDeserializer, DeserializeSeed, MapAccess, SeqAccess, Unexpected,
    Visitor,
};

use super::Kind;
use crate::{files::align, Error, Result};

impl Error {
    fn eof() -> Self {
        Self::new("End of file while attempting deserialization.")
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::new(msg.to_string())
    }
}

pub struct Deserializer<'de> {
    source: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(source: &'de [u8]) -> Self {
        Self { source }
    }

    fn invalid_root<V>(kind: &'static str) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom(format!(
            "Attempted to deserialize a {}",
            kind
        )))
    }

    fn document(&self) -> Result<Document<'de>> {
        Document::new(self.source)
    }

    fn array<'doc>(self, document: &'doc Document<'de>) -> Result<Array<'doc, 'de>> {
        document.root(Document::array)
    }

    fn map<'doc>(self, document: &'doc Document<'de>) -> Result<Map<'doc, 'de>> {
        document.root(Document::map)
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(root) = NonZeroU32::new(self.document()?.root) {
            let kind = (*self
                .source
                .get(root.get() as usize)
                .ok_or_else(Error::eof)?)
            .try_into()?;
            let document = self.document()?;
            match kind {
                Kind::Array => visitor.visit_seq(self.array(&document)?),
                Kind::Map => visitor.visit_map(self.map(&document)?),
                kind => Err(de::Error::invalid_value(
                    Unexpected::Other(kind.to_string().as_str()),
                    &"0xC0 [array] or 0xC1 [map]",
                )),
            }
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("bool")
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("i8")
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("i16")
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("i32")
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("i64")
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("u8")
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("u16")
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("u32")
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("u64")
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("f32")
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("f64")
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("char")
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("str")
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("string")
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("bytes")
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("byte_buf")
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if NonZeroU32::new(self.document()?.root).is_some() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let document = self.document()?;
        visitor.visit_seq(self.array(&document)?)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let document = self.document()?;
        visitor.visit_map(self.map(&document)?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("enum")
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Self::invalid_root::<V>("identifier")
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

#[derive(Debug)]
struct Document<'de> {
    source: &'de [u8],
    keys: Vec<&'de [u8]>,
    strings: Vec<&'de [u8]>,
    root: u32,
}

impl<'de> Document<'de> {
    fn new(source: &'de [u8]) -> Result<Self> {
        bytey::typedef! { struct Header: TryFromBytes<'_> [0x10] {
            #super::MAGIC,
            [2] version: u16 where version == super::VERSION,
            [4] keys: u32,
            [8] strings: u32,
            [0xC] root: u32,
        }}
        let (header, _) = Header::try_from_slice(source)?;
        let keys = Self::strings(source, header.keys)?;
        let strings = Self::strings(source, header.strings)?;
        Ok(Self {
            source,
            keys,
            strings,
            root: header.root,
        })
    }

    fn strings(source: &'de [u8], offset: u32) -> Result<Vec<&'de [u8]>> {
        if offset == 0 {
            Ok(vec![])
        } else {
            source
                .get(offset as usize..)
                .ok_or_else(Error::eof)
                .and_then(read_strings)
        }
    }

    fn key(&self, index: u32) -> Result<&'de [u8]> {
        self.keys.get(index as usize).copied().ok_or_else(|| {
            de::Error::invalid_value(
                Unexpected::Unsigned(index as u64),
                &format!("key index < {}", self.keys.len()).as_str(),
            )
        })
    }

    fn string(&self, index: u32) -> Result<&'de [u8]> {
        self.strings.get(index as usize).copied().ok_or_else(|| {
            de::Error::invalid_value(
                Unexpected::Unsigned(index as u64),
                &format!("string index < {}", self.strings.len()).as_str(),
            )
        })
    }

    fn root<'doc, T>(&'doc self, f: fn(&'doc Self, u32) -> Result<T>) -> Result<T> {
        f(self, self.root)
    }

    fn array<'doc>(&'doc self, offset: u32) -> Result<Array<'doc, 'de>> {
        Array::from(
            self,
            self.source.get(offset as usize..).ok_or_else(Error::eof)?,
        )
    }

    fn map<'doc>(&'doc self, offset: u32) -> Result<Map<'doc, 'de>> {
        Map::from(
            self,
            self.source.get(offset as usize..).ok_or_else(Error::eof)?,
        )
    }
}

#[derive(Debug)]
struct Node<'doc, 'de> {
    document: &'doc Document<'de>,
    kind: Kind,
    value: [u8; 4],
}

impl<'doc, 'de> Node<'doc, 'de> {
    fn new(document: &'doc Document<'de>, kind: Kind, value: [u8; 4]) -> Self {
        Self {
            document,
            kind,
            value,
        }
    }

    fn expect_kind(&self, expected: Kind) -> Result<()> {
        expect_kind(self.kind, expected)
    }

    fn string(self) -> Result<&'de [u8]> {
        self.expect_kind(Kind::String)?;
        self.document.string(u32::from_bytes(&self.value))
    }

    fn string_de(self) -> Result<BorrowedBytesDeserializer<'de, Error>> {
        self.string().map(BorrowedBytesDeserializer::new)
    }

    fn array(self) -> Result<Array<'doc, 'de>> {
        self.expect_kind(Kind::Array)?;
        self.document.array(u32::from_bytes(&self.value))
    }

    fn map(self) -> Result<Map<'doc, 'de>> {
        self.expect_kind(Kind::Map)?;
        self.document.map(u32::from_bytes(&self.value))
    }

    fn boolean(self) -> Result<bool> {
        self.expect_kind(Kind::Boolean)?;
        Ok(self.value != FALSE)
    }

    fn integer(self) -> Result<i32> {
        self.expect_kind(Kind::Integer)?;
        Ok(i32::from_bytes(&self.value))
    }

    fn try_integer<T>(self) -> Result<T>
    where
        T: TryFrom<i32>,
        <T as TryFrom<i32>>::Error: Display,
    {
        self.integer()?.try_into().map_err(de::Error::custom)
    }

    fn float(self) -> Result<f32> {
        self.expect_kind(Kind::Float)?;
        Ok(f32::from_bytes(&self.value))
    }

    fn is_null(&self) -> bool {
        self.kind == Kind::Null
    }
}

impl<'doc, 'de> de::Deserializer<'de> for Node<'doc, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.kind {
            Kind::String => visitor.visit_bytes(self.string()?),
            Kind::Array => visitor.visit_seq(self.array()?),
            Kind::Map => visitor.visit_map(self.map()?),
            Kind::Boolean => visitor.visit_bool(self.boolean()?),
            Kind::Integer => visitor.visit_i32(self.integer()?),
            Kind::Float => visitor.visit_f32(self.float()?),
            Kind::Null => visitor.visit_none(),
            kind => Err(de::Error::invalid_value(
                Unexpected::Unsigned(kind as u8 as u64),
                &"a supported node type",
            )),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.boolean()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.try_integer()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.try_integer()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.integer()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.integer()? as i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.try_integer()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.try_integer()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.try_integer()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.try_integer()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.float()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.float()? as f64)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.string_de()?.deserialize_char(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.string_de()?.deserialize_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.string_de()?.deserialize_string(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.string()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.string()?.to_vec())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self.array()?)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self.map()?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("Enum deserialization is not supported"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.string_de()?.deserialize_identifier(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

#[derive(Debug)]
struct Array<'doc, 'de> {
    document: &'doc Document<'de>,
    kinds: &'de [u8],
    values: &'de [u8],
}

impl<'doc, 'de> Array<'doc, 'de> {
    fn from(document: &'doc Document<'de>, source: &'de [u8]) -> Result<Self> {
        let (count, rest) = header(source, Kind::Array)?;
        let (kinds, values) = split(rest, align::<4>(count) as usize)?;
        let kinds = &kinds[0..count as usize];
        Ok(Self {
            document,
            kinds,
            values,
        })
    }
}

impl<'doc, 'de> SeqAccess<'de> for Array<'doc, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.kinds.is_empty() {
            Ok(None)
        } else {
            let (kind, kinds) = TryFromSlice::try_from_slice(self.kinds)?;
            let (value, values) = TryFromSlice::try_from_slice(self.values)?;
            self.kinds = kinds;
            self.values = values;
            seed.deserialize(Node::new(self.document, kind, value))
                .map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.kinds.len())
    }
}

#[derive(Debug)]
struct Map<'doc, 'de> {
    document: &'doc Document<'de>,
    entries: &'de [u8],
}

impl<'doc, 'de> Map<'doc, 'de> {
    fn from(document: &'doc Document<'de>, source: &'de [u8]) -> Result<Self> {
        let (count, rest) = header(source, Kind::Map)?;
        let entries = rest.get(0..8 * count as usize).ok_or_else(Error::eof)?;
        Ok(Map { document, entries })
    }
}

impl<'doc, 'de> MapAccess<'de> for Map<'doc, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.entries.is_empty() {
            Ok(None)
        } else {
            let (key, entries) = U24::try_from_slice(self.entries)?;
            self.entries = entries;
            seed.deserialize(BorrowedBytesDeserializer::new(self.document.key(key.0)?))
                .map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        bytey::typedef! { struct Entry: TryFromBytes<'_> [5] {
            [0] kind: Kind,
            [1] value: [u8; 4],
        }}
        let (entry, entries) = Entry::try_from_slice(self.entries)?;
        self.entries = entries;
        seed.deserialize(Node::new(self.document, entry.kind, entry.value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.entries.len() / 8)
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:02X} [{}]",
            *self as u8,
            match self {
                Kind::String => "string",
                Kind::Array => "array",
                Kind::Map => "map",
                Kind::Strings => "strings",
                Kind::Boolean => "boolean",
                Kind::Integer => "integer",
                Kind::Float => "float",
                Kind::Null => "null",
            }
        )
    }
}

#[derive(Debug)]
struct U24(u32);

impl FromBytes<'_> for U24 {
    const SIZE: usize = 3;
    type Bytes = [u8; 3];

    fn from_bytes(bytes: &'_ Self::Bytes) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }
}

fn read_strings(source: &[u8]) -> Result<Vec<&[u8]>> {
    let (count, rest) = header(source, Kind::Strings)?;
    if count == 0 {
        Ok(vec![])
    } else {
        let mut offsets = rest
            .get(0..4 * (count as usize + 1))
            .ok_or_else(Error::eof)?
            .chunks_exact(4)
            .map(|bytes| unsafe { u32::from_slice_unchecked(bytes) });
        let mut start = offsets.next().unwrap();
        let mut end = 0;
        (0..count)
            .map(|_| {
                end = offsets.next().unwrap();
                let bytes = source
                    .get(start as usize..end as usize)
                    .ok_or_else(Error::eof)?;
                let bytes = bytes.split(|&byte| byte == 0).next().unwrap();
                start = end;
                Ok(bytes)
            })
            .collect()
    }
}

fn split(slice: &[u8], mid: usize) -> Result<(&[u8], &[u8])> {
    if mid > slice.len() {
        Err(Error::eof())
    } else {
        Ok(slice.split_at(mid))
    }
}

fn header(source: &[u8], kind: Kind) -> Result<(u32, &[u8])> {
    bytey::typedef! { struct Header: TryFromBytes<'_> [4] {
        [0] kind: Kind,
        [1] count: U24,
    }}
    let (header, rest) = Header::try_from_slice(source)?;
    expect_kind(header.kind, kind)?;
    Ok((header.count.0, rest))
}

fn expect_kind(kind: Kind, expected: Kind) -> Result<()> {
    if kind == expected {
        Ok(())
    } else {
        Err(de::Error::invalid_value(
            Unexpected::Unsigned(kind as u64),
            &expected.to_string().as_str(),
        ))
    }
}

pub fn from_bytes<'de, T>(source: &'de [u8]) -> Result<T>
where
    T: de::Deserialize<'de>,
{
    T::deserialize(Deserializer::from_bytes(source))
}

const FALSE: [u8; 4] = [0, 0, 0, 0];

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Deserialize;

    use super::super::tests::data;

    #[test]
    fn it_deserializes_array() {
        assert_eq!(
            super::from_bytes::<Vec<i32>>(data::ARRAY).unwrap(),
            vec![0x01234567]
        );
    }

    #[test]
    fn it_deserializes_nested_array() {
        assert_eq!(
            super::from_bytes::<Vec<Vec<i32>>>(data::NESTED_ARRAY).unwrap(),
            vec![vec![0x01234567]]
        )
    }

    #[test]
    fn it_deserializes_map() {
        let map = super::from_bytes::<BTreeMap<String, String>>(data::MAP).unwrap();
        assert_eq!(map.get("foo").unwrap(), "Foo");
        assert_eq!(map.get("bar").unwrap(), "Bar");
    }

    #[test]
    fn it_deserializes_struct() {
        #[derive(Deserialize)]
        struct Struct {
            foo: String,
            bar: String,
        }

        let st = super::from_bytes::<Struct>(data::MAP).unwrap();
        assert_eq!(st.foo, "Foo");
        assert_eq!(st.bar, "Bar");
    }

    #[test]
    fn it_reads_strings() {
        let data = &[
            0xC2, 0x02, 0, 0, 0x10, 0, 0, 0, 0x14, 0, 0, 0, 0x18, 0, 0, 0, b'B', b'a', b'r', 0,
            b'F', b'o', b'o', 0,
        ];
        let strings = super::read_strings(data).unwrap();
        assert_eq!(strings[0], b"Bar");
        assert_eq!(strings[1], b"Foo");
    }
}

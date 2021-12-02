use std::{
    collections::BTreeMap,
    convert::TryInto,
    fmt::{self, Display, Formatter},
    io::{Seek, SeekFrom, Write},
};

use serde::ser::{
    self, Impossible, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
    SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};

use super::Kind;
use crate::files::align;

mod to_bytes {
    pub fn version(version: u16) -> [u8; 2] {
        version.to_le_bytes()
    }

    pub fn u24(value: u32) -> [u8; 3] {
        let bytes = value.to_le_bytes();
        [bytes[0], bytes[1], bytes[2]]
    }

    pub fn unsigned(value: u32) -> [u8; 4] {
        value.to_le_bytes()
    }

    pub fn signed(value: i32) -> [u8; 4] {
        value.to_le_bytes()
    }

    pub fn float(value: f32) -> [u8; 4] {
        value.to_le_bytes()
    }
}

use to_bytes::*;

type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ::std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self(msg.to_string())
    }
}

#[derive(Debug)]
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    fn invalid_root(kind: &'static str) -> Error {
        ser::Error::custom(format!("cannot serialize a {} at root", kind))
    }
}

impl<W> Serializer<W>
where
    W: Write + Seek,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn empty(self) -> Result<()> {
        Writer::empty(self.writer).map(|_| ())
    }

    fn array(self, count: Option<usize>) -> Result<RootArray<W>> {
        RootArray::new(self.writer, count)
    }

    fn map(self, count: Option<usize>) -> Result<RootMap<W>> {
        RootMap::new(self.writer, count)
    }
}

impl<W> ser::Serializer for Serializer<W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = RootArray<W>;
    type SerializeTuple = RootArray<W>;
    type SerializeTupleStruct = RootArray<W>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = RootMap<W>;
    type SerializeStruct = RootMap<W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("bool"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("i8"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("i16"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("i32"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("i64"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("u8"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("u16"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("u32"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("u64"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("f64"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("char"))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("str"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.empty()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.empty()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.empty()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Self::invalid_root("unit variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Self::invalid_root("newtype variant"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.array(len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.array(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.array(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Self::invalid_root("tuple variant"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.map(len)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Self::invalid_root("struct variant"))
    }
}

#[derive(Debug)]
struct Writer<W> {
    writer: W,
    keys: Vec<u32>,
    strings: Vec<u32>,
    end: u32,
}

impl<W> Writer<W>
where
    W: Write + Seek,
{
    fn empty(writer: W) -> Result<Self> {
        let mut this = Self {
            writer,
            keys: vec![],
            strings: vec![],
            end: 0,
        };
        this.write_magic()?;
        this.write(&[0u8; 0xC])?;
        Ok(this)
    }

    fn with_document(writer: W, document: Document) -> Result<Self> {
        let mut this = Self {
            writer,
            keys: vec![],
            strings: vec![],
            end: 0,
        };
        this.write_magic()?;
        let mut keys = vec![];
        let offset = this.write_strings(document.keys, &mut keys)?;
        this.keys = keys;
        this.seek(0x4)?;
        this.write(&unsigned(offset))?;
        let mut strings = vec![];
        let offset = this.write_strings(document.strings, &mut strings)?;
        this.strings = strings;
        this.seek(0x8)?;
        this.write(&unsigned(offset))?;
        Ok(this)
    }

    fn seek(&mut self, offset: u32) -> Result<()> {
        self.writer
            .seek(SeekFrom::Start(offset as u64))
            .map(|_| ())
            .map_err(ser::Error::custom)
    }

    fn position(&mut self) -> Result<u32> {
        check_offset(
            self.writer
                .seek(SeekFrom::Current(0))
                .map_err(|err| Error(err.to_string()))?,
        )
    }

    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer
            .write(bytes)
            .map(|_| ())
            .map_err(ser::Error::custom)
    }

    fn write_magic(&mut self) -> Result<()> {
        self.end = 0x10;
        self.seek(0)?;
        self.write(super::MAGIC)?;
        self.write(&version(super::VERSION))
    }

    fn write_strings(&mut self, strings: Strings, map: &mut Vec<u32>) -> Result<u32> {
        let count = strings.count();
        if count == 0 {
            Ok(0)
        } else {
            let mut sorted = strings.0.clone();
            sorted.sort();
            for string in strings.into_iter() {
                let index = sorted.iter().position(|other| &string == other).unwrap() as u32;
                map.push(index);
            }
            let offset = self.write_header(Kind::Strings, count)?;
            let offsets_offset = self.position()?;
            let mut offsets = vec![0u8; count as usize * 4];
            self.write(&offsets)?;
            self.write(&[0, 0, 0, 0])?;
            let mut start = (offsets.len() + 8) as u32;
            for (string, offset_mut) in sorted.into_iter().zip(offsets.chunks_mut(4)) {
                offset_mut.copy_from_slice(&unsigned(start));
                self.write(&string)?;
                self.write(&[0])?;
                start += string.len() as u32 + 1;
            }
            self.end = align::<4>(self.position()?);
            self.seek(offsets_offset)?;
            self.write(&offsets)?;
            self.write(&unsigned(start))?;
            Ok(offset)
        }
    }

    fn write_header(&mut self, kind: Kind, count: u32) -> Result<u32> {
        let offset = self.end;
        self.seek(self.end)?;
        self.write(&[kind as u8])?;
        self.write(&u24(count))?;
        Ok(offset)
    }

    fn write_root(&mut self, offset: u32) -> Result<()> {
        self.seek(0xC)?;
        self.write(&unsigned(offset))
    }

    fn write_array(&mut self, array: Array) -> Result<u32> {
        let count = array.count();
        let offset = self.write_header(Kind::Array, count)?;
        let kinds_offset = self.position()?;
        let mut kinds = vec![0u8; align::<4>(count) as usize];
        self.write(&kinds)?;
        let values_offset = self.position()?;
        let mut values = vec![0u8; count as usize * 4];
        self.write(&values)?;
        self.end = self.position()?;
        for (node, (kind_mut, value_mut)) in array
            .into_iter()
            .zip(kinds.iter_mut().zip(values.chunks_mut(4)))
        {
            let (kind, value) = self.write_node(node)?;
            *kind_mut = kind as u8;
            value_mut.copy_from_slice(&value);
        }
        self.seek(kinds_offset)?;
        self.write(&kinds)?;
        self.seek(values_offset)?;
        self.write(&values)?;
        Ok(offset)
    }

    fn write_map(&mut self, map: Map) -> Result<u32> {
        let count = map.count();
        let offset = self.write_header(Kind::Map, count)?;
        let entries_offset = self.position()?;
        let mut entries = vec![0u8; count as usize * 8];
        self.write(&entries)?;
        self.end = self.position()?;
        let map = map
            .into_iter()
            .map(|(key, value)| {
                let key = self.keys[key as usize];
                let (kind, value) = self.write_node(value)?;
                Ok((key, (kind, value)))
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        //entries.sort_by_key(|(key, _, _)| key);
        for ((key, (kind, value)), entry_mut) in map.into_iter().zip(entries.chunks_mut(8)) {
            entry_mut[0..3].copy_from_slice(&u24(key));
            entry_mut[3] = kind as u8;
            entry_mut[4..].copy_from_slice(&value);
        }
        self.seek(entries_offset)?;
        self.write(&entries)?;
        Ok(offset)
    }

    fn write_node(&mut self, node: Node) -> Result<(Kind, [u8; 4])> {
        Ok(match node {
            Node::String(index) => (Kind::String, {
                let index = self.strings[index as usize];
                unsigned(index)
            }),
            Node::Array(array) => {
                let offset = self.write_array(array)?;
                (Kind::Array, unsigned(offset))
            }
            Node::Map(map) => {
                let offset = self.write_map(map)?;
                (Kind::Map, unsigned(offset))
            }
            Node::Boolean(boolean) => (Kind::Boolean, unsigned(boolean.into())),
            Node::Integer(integer) => (Kind::Integer, signed(integer)),
            Node::Float(f) => (Kind::Float, float(f)),
            Node::Null => (Kind::Null, NULL),
        })
    }
}

#[derive(Debug)]
pub struct RootArray<W> {
    writer: W,
    document: Document,
    array: Array,
}

impl<W> RootArray<W> {
    fn new(writer: W, count: Option<usize>) -> Result<Self> {
        Ok(Self {
            writer,
            document: Default::default(),
            array: Array::with_count(count)?,
        })
    }

    fn write(self) -> Result<()>
    where
        W: Write + Seek,
    {
        let mut writer = Writer::with_document(self.writer, self.document)?;
        let offset = writer.write_array(self.array)?;
        writer.write_root(offset)
    }
}

impl<W> SerializeSeq for RootArray<W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.array.push(&mut self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write()
    }
}

impl<W> SerializeTuple for RootArray<W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.array.push(&mut self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write()
    }
}

impl<W> SerializeTupleStruct for RootArray<W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.array.push(&mut self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write()
    }
}

#[derive(Debug)]
pub struct RootMap<W> {
    writer: W,
    document: Document,
    map: Map,
}

impl<W> RootMap<W> {
    fn new(writer: W, count: Option<usize>) -> Result<Self> {
        Ok(Self {
            writer,
            document: Default::default(),
            map: Map::with_count(count)?,
        })
    }

    fn write(self) -> Result<()>
    where
        W: Write + Seek,
    {
        let mut writer = Writer::with_document(self.writer, self.document)?;
        let offset = writer.write_map(self.map)?;
        writer.write_root(offset)
    }
}

impl<W> SerializeMap for RootMap<W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_key(&mut self.document, key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_value(&mut self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write()
    }
}

impl<W> SerializeStruct for RootMap<W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_key(&mut self.document, key)?;
        self.map.insert_value(&mut self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.write()
    }
}

#[derive(Debug, Default)]
struct Document {
    keys: Strings,
    strings: Strings,
}

impl Document {
    fn string(&mut self, string: &[u8]) -> Result<Node> {
        self.strings.index(string).map(Node::String)
    }

    fn boolean(&self, boolean: bool) -> Node {
        Node::Boolean(boolean)
    }

    fn integer(&self, integer: i32) -> Node {
        Node::Integer(integer)
    }

    fn try_integer<T>(&self, integer: T) -> Result<Node>
    where
        T: TryInto<i32>,
        <T as TryInto<i32>>::Error: Display,
    {
        integer
            .try_into()
            .map_err(ser::Error::custom)
            .map(|integer| self.integer(integer))
    }

    fn float(&self, float: f32) -> Node {
        Node::Float(float)
    }

    fn null(&self) -> Node {
        Node::Null
    }
}

#[derive(Debug)]
enum Node {
    String(u32),
    Array(Array),
    Map(Map),
    Boolean(bool),
    Integer(i32),
    Float(f32),
    Null,
}

impl<'doc> ser::Serializer for &'doc mut Document {
    type Ok = Node;
    type Error = Error;
    type SerializeSeq = BArray<'doc>;
    type SerializeTuple = BArray<'doc>;
    type SerializeTupleStruct = BArray<'doc>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = BMap<'doc>;
    type SerializeStruct = BMap<'doc>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(self.boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(self.integer(v as i32))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(self.integer(v as i32))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(self.integer(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.try_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(self.integer(v as i32))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(self.integer(v as i32))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.try_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.try_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(self.float(v))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("cannot serialize an f64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.string(v.to_string().as_bytes())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.string(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.string(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.null())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.null())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(self.null())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom(
            "enum variant serialization not supported",
        ))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(ser::Error::custom(
            "enum variant serialization not supported",
        ))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        BArray::new(self, len)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        BArray::new(self, Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        BArray::new(self, Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom(
            "enum variant serialization not supported",
        ))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        BMap::new(self, len)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        BMap::new(self, Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom(
            "enum variant serialization not supported",
        ))
    }
}

#[derive(Debug, Default)]
struct Array(Vec<Node>);

impl Array {
    fn with_count(count: Option<usize>) -> Result<Self> {
        if let Some(count) = count {
            if is_count_ok(count) {
                Ok(Self(vec![]))
            } else {
                Err(ser::Error::custom("sequence too large to be serialized"))
            }
        } else {
            Err(ser::Error::custom(
                "cannot serialize a sequence of unknown length",
            ))
        }
    }

    fn count(&self) -> u32 {
        self.0.len() as u32
    }

    fn push<T: ?Sized>(&mut self, document: &mut Document, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.0.push(value.serialize(document)?);
        Ok(())
    }

    fn into_node(self) -> Node {
        Node::Array(self)
    }

    fn into_iter(self) -> impl Iterator<Item = Node> {
        self.0.into_iter()
    }
}

#[derive(Debug)]
struct BArray<'doc> {
    document: &'doc mut Document,
    array: Array,
}

impl<'doc> BArray<'doc> {
    fn new(document: &'doc mut Document, count: Option<usize>) -> Result<Self> {
        Ok(Self {
            document,
            array: Array::with_count(count)?,
        })
    }
}

impl<'doc> SerializeSeq for BArray<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.array.push(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.into_node())
    }
}

impl<'doc> SerializeTuple for BArray<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.array.push(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.into_node())
    }
}

impl<'doc> SerializeTupleStruct for BArray<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.array.push(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.into_node())
    }
}

impl<'doc> SerializeTupleVariant for BArray<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.array.push(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.array.into_node())
    }
}

#[derive(Debug, Default)]
struct Map {
    entries: Vec<(u32, Node)>,
    last_key: Option<u32>,
}

impl Map {
    fn with_count(count: Option<usize>) -> Result<Self> {
        if let Some(count) = count {
            if is_count_ok(count) {
                Ok(Default::default())
            } else {
                Err(ser::Error::custom("map too large to be serialized"))
            }
        } else {
            Err(ser::Error::custom(
                "cannot serialize a map of unknown length",
            ))
        }
    }

    fn count(&self) -> u32 {
        self.entries.len() as u32
    }

    fn insert_key<T: ?Sized>(&mut self, document: &mut Document, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.last_key = Some(key.serialize(&mut document.keys)?);
        Ok(())
    }

    fn insert_value<T: ?Sized>(&mut self, document: &mut Document, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        if let Some(key) = self.last_key.take() {
            self.entries.push((key, value.serialize(document)?));
            Ok(())
        } else {
            Err(ser::Error::custom(
                "called `serialize_value` before `serialize_key`",
            ))
        }
    }

    fn into_node(self) -> Node {
        Node::Map(self)
    }

    fn into_iter(self) -> impl Iterator<Item = (u32, Node)> {
        self.entries.into_iter()
    }
}

#[derive(Debug)]
struct BMap<'doc> {
    document: &'doc mut Document,
    map: Map,
}

impl<'doc> BMap<'doc> {
    fn new(document: &'doc mut Document, count: Option<usize>) -> Result<Self> {
        Ok(Self {
            document,
            map: Map::with_count(count)?,
        })
    }
}

impl<'doc> SerializeMap for BMap<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_key(self.document, key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_value(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map.into_node())
    }
}

impl<'doc> SerializeStruct for BMap<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_key(self.document, key)?;
        self.map.insert_value(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map.into_node())
    }
}

impl<'doc> SerializeStructVariant for BMap<'doc> {
    type Ok = Node;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        self.map.insert_key(self.document, key)?;
        self.map.insert_value(self.document, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.map.into_node())
    }
}

#[derive(Debug, Default)]
struct Strings(Vec<Box<[u8]>>);

impl Strings {
    fn count(&self) -> u32 {
        self.0.len() as u32
    }

    fn index(&mut self, string: &[u8]) -> Result<u32> {
        if let Some(index) = self.0.iter().position(|other| other.as_ref() == string) {
            Ok(index as u32)
        } else {
            let index = self.0.len();
            if index > MAX_COUNT {
                Err(ser::Error::custom("too many strings to be serialized"))
            } else {
                self.0.push(string.into());
                Ok(index as u32)
            }
        }
    }

    fn into_iter(self) -> impl Iterator<Item = Box<[u8]>> {
        self.0.into_iter()
    }

    fn invalid(kind: &'static str) -> Error {
        ser::Error::custom(format!("cannot serialize a {} as a key", kind))
    }
}

impl<'doc> ser::Serializer for &'doc mut Strings {
    type Ok = u32;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("bool"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("i8"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("i16"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("i32"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("i64"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("u8"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("u16"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("u32"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("u64"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("u64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.index(v.to_string().as_bytes())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.index(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.index(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("None"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Strings::invalid("Some"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("()"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Strings::invalid("unit variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Strings::invalid("newtype struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        Err(Strings::invalid("newtype variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Strings::invalid("seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Strings::invalid("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Strings::invalid("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Strings::invalid("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Strings::invalid("map"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Strings::invalid("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Strings::invalid("struct variant"))
    }
}

fn check_offset(offset: u64) -> Result<u32> {
    offset
        .try_into()
        .map_err(|_| ser::Error::custom("file size too large"))
}

fn is_count_ok(count: usize) -> bool {
    count <= MAX_COUNT
}

pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
where
    W: Write + Seek,
    T: Serialize,
{
    value.serialize(Serializer::new(writer))
}

const MAX_COUNT: usize = 0xFFFFFF;
const NULL: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

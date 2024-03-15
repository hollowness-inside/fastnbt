use std::io::Write;

use serde::ser::Impossible;

use crate::macros::array_as_other;
use crate::{error::Error, error::Result, Tag};

use super::{serializer::Serializer, write_nbt::WriteNbt};

/// ArraySerializer is for serializing the NBT Arrays ie ByteArray, IntArray and
/// LongArray.
pub(crate) struct ArraySerializer<'a, W: Write> {
    pub(crate) ser: &'a mut Serializer<W>,
    pub(crate) tag: Tag,
}

impl<'a, W: Write> serde::Serializer for ArraySerializer<'a, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        let stride = match self.tag {
            Tag::ByteArray => 1,
            Tag::IntArray => 4,
            Tag::LongArray => 8,
            _ => panic!(),
        };
        let len = v.len() / stride;
        self.ser.writer.write_len(len)?;
        self.ser.writer.write_all(v)?;
        Ok(())
    }

    array_as_other!(serialize_bool(bool));
    array_as_other!(serialize_i8(i8));
    array_as_other!(serialize_i16(i16));
    array_as_other!(serialize_i32(i32));
    array_as_other!(serialize_i64(i64));
    array_as_other!(serialize_i128(i128));
    array_as_other!(serialize_u8(u8));
    array_as_other!(serialize_u16(u16));
    array_as_other!(serialize_u32(u32));
    array_as_other!(serialize_u64(u64));
    array_as_other!(serialize_u128(u128));
    array_as_other!(serialize_f32(f32));
    array_as_other!(serialize_f64(f64));
    array_as_other!(serialize_char(char));
    array_as_other!(serialize_str(&str));
    array_as_other!(serialize_none());
    array_as_other!(serialize_some<T>(&T));
    array_as_other!(serialize_unit());
    array_as_other!(serialize_unit_struct(&'static str));
    array_as_other!(serialize_unit_variant(&'static str, u32, &'static str));
    array_as_other!(serialize_newtype_struct<T>(&'static str, &T));
    array_as_other!(serialize_newtype_variant<T>(&'static str, u32, &'static str, &T));
    array_as_other!(serialize_seq(Option<usize>) -> Self::SerializeSeq);
    array_as_other!(serialize_tuple(usize) -> Self::SerializeSeq);
    array_as_other!(serialize_map(Option<usize>) -> Self::SerializeMap);
    array_as_other!(serialize_tuple_struct(&'static str, usize) -> Self::SerializeTupleStruct);
    array_as_other!(serialize_struct(&'static str, usize) -> Self::SerializeStruct);

    array_as_other!(
        serialize_tuple_variant(
            &'static str,
            u32,
            &'static str,
            usize,
        ) -> Self::SerializeTupleVariant
    );

    array_as_other!(
        serialize_struct_variant(
            &'static str,
            u32,
            &'static str,
            usize,
        ) -> Self::SerializeStructVariant
    );
}

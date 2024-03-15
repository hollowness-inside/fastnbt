use byteorder::{NativeEndian, ReadBytesExt};
use serde::ser::Impossible;

use crate::error::Result;
use crate::macros::array_as_other;
use crate::{error::Error, ByteArray, IntArray, LongArray, Tag, Value};

use super::ser::Serializer;

/// ArraySerializer is for serializing the NBT Arrays ie ByteArray, IntArray and
/// LongArray.
pub struct ArraySerializer<'a> {
    pub ser: &'a mut Serializer,
    pub tag: Tag,
}

impl<'a> serde::Serializer for ArraySerializer<'a> {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        match self.tag {
            Tag::ByteArray => Ok(Value::ByteArray(ByteArray::from_bytes(v))),
            Tag::IntArray => Ok(Value::IntArray(IntArray::new(
                v.chunks_exact(4)
                    .map(|mut bs| bs.read_i32::<NativeEndian>())
                    .collect::<std::io::Result<Vec<i32>>>()?,
            ))),
            Tag::LongArray => Ok(Value::LongArray(LongArray::new(
                v.chunks_exact(8)
                    .map(|mut bs| bs.read_i64::<NativeEndian>())
                    .collect::<std::io::Result<Vec<i64>>>()?,
            ))),
            _ => unreachable!(),
        }
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

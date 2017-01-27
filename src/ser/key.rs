use serde::{Serialize, Serializer};
use ser::Error;
use std::borrow::Cow;
use std::str;

pub struct MapKeySerializer<'key>(&'key mut Option<Cow<'static, str>>);

impl<'key> MapKeySerializer<'key> {
    pub fn new(output: &'key mut Option<Cow<'static, str>>) -> Self {
        MapKeySerializer(output)
    }

    fn set_key<T>(&mut self, key: T) -> Result<(), Error>
        where T: Into<Cow<'static, str>>
    {
        *self.0 = Some(key.into());
        Ok(())
    }
}

impl<'key> Serializer for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.set_key(v.to_string())
    }

    fn serialize_str(mut self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.set_key(String::from(value))
    }

    fn serialize_bytes(mut self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        match str::from_utf8(value) {
            Ok(value) => self.set_key(String::from(value)),
            Err(err) => Err(Error::Utf8(err)),
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_unit_struct(
            mut self, name: &'static str)
            -> Result<Self::Ok, Self::Error> {
        self.set_key(name)
    }

    fn serialize_unit_variant(
            mut self,
            _name: &'static str,
            _variant_index: usize,
            variant: &'static str)
            -> Result<Self::Ok, Self::Error> {
        self.set_key(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
            self,
            _name: &'static str,
            value: &T)
            -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _value: &T)
            -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_seq_fixed_size(self, _size: usize) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_tuple_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeTuple, Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeSeq for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeTuple for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeTupleStruct for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeTupleVariant for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeMap for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn serialize_value<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeStruct for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl<'key> ::serde::ser::SerializeStructVariant for MapKeySerializer<'key> {
    type Ok = ();
    type Error = Error;


    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_key())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_key())
    }
}

impl Error {
    fn unsupported_key() -> Self {
        Error::Custom("unsupported key".to_string())
    }
}

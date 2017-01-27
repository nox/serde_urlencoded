use ser::Error;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::str;
use url::form_urlencoded;

pub struct ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    key: &'key mut Option<Cow<'static, str>>,
    serializer: &'target mut form_urlencoded::Serializer<Target>
}

impl<'key, 'target, Target> ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    pub fn new(
            key: &'key mut Option<Cow<'static, str>>,
            serializer: &'target mut form_urlencoded::Serializer<Target>)
            -> Result<Self, Error> {
        if key.is_some() {
            Ok(ValueSerializer {
                key: key,
                serializer: serializer,
            })
        } else {
            Err(Error::no_key())
        }
    }

    fn append_pair(&mut self, value: &str) -> Result<(), Error> {
        if let Some(key) = self.key.take() {
            self.serializer.append_pair(&key, value);
            Ok(())
        } else {
            Err(Error::no_key())
        }
    }
}

impl<'key, 'target, Target> Serializer for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.append_pair(if v { "true" } else { "false" })
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.append_pair(&v.to_string())
    }

    fn serialize_str(mut self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.append_pair(value)
    }

    fn serialize_bytes(mut self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        match str::from_utf8(value) {
            Ok(value) => self.append_pair(value),
            Err(err) => Err(Error::Utf8(err)),
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_unit_struct(
            mut self, name: &'static str)
            -> Result<Self::Ok, Self::Error> {
        self.append_pair(name)
    }

    fn serialize_unit_variant(
            mut self,
            _name: &'static str,
            _variant_index: usize,
            variant: &'static str)
            -> Result<Self::Ok, Self::Error> {
        self.append_pair(variant)
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
        Err(Error::unsupported_value())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        if let Some(_) = self.key.take() {
            Ok(())
        } else {
            Err(Error::no_key())
        }
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_seq_fixed_size(self, _size: usize) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_tuple_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: usize,
        _variant: &'static str,
        _len: usize)
        -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeSeq for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeTuple for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeTupleStruct for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeTupleVariant for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeMap for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn serialize_value<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeStruct for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl<'key, 'target, Target> ::serde::ser::SerializeStructVariant for ValueSerializer<'key, 'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;


    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_value())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_value())
    }
}

impl Error {
    fn no_key() -> Self {
        Error::Custom("tried to serialize a value before serializing key".to_string())
    }

    fn unsupported_value() -> Self {
        Error::Custom("unsupported value".to_string())
    }
}

use ser::{Error, key, value};
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use url::form_urlencoded;

pub struct PairSerializer<'target, Target>(&'target mut form_urlencoded::Serializer<Target>)
    where Target: 'target + form_urlencoded::Target;

impl<'target, Target> PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    pub fn new(serializer: &'target mut form_urlencoded::Serializer<Target>)-> Self {
        PairSerializer(serializer)
    }
}

impl<'target, Target> Serializer for PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = SerializeTuple<'target, Target>;
    type SerializeTupleStruct = SerializeTuple<'target, Target>;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::Custom("booleans are not supported values".to_string()))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str)
            -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
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
        Err(Error::unsupported_pair())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>)
                     -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_seq_fixed_size(self, _size: usize)
                                -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        if len == 2 {
            Ok(SerializeTuple(None, self.0))
        } else {
            Err(Error::unsupported_pair())
        }
    }

    fn serialize_tuple_struct(
            self, _name: &'static str, len: usize)
            -> Result<Self::SerializeTupleStruct, Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_map(
            self, _len: Option<usize>)
            -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeStruct, Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_pair())
    }
}

impl<'target, Target> ::serde::ser::SerializeSeq for PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }
}

pub struct SerializeTuple<'target, Target>(Option<Option<Cow<'static, str>>>, &'target mut form_urlencoded::Serializer<Target>)
    where Target: 'target + form_urlencoded::Target;

impl<'target, Target> ::serde::ser::SerializeTuple for SerializeTuple<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ::serde::ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        match self.0.take() {
            None => {
                let mut key = None;
                {
                    let key_serializer =
                        key::MapKeySerializer::new(&mut key);
                    try!(value.serialize(key_serializer));
                }
                self.0 = Some(key);
                Ok(())
            },
            Some(ref mut key) => {
                {
                    let value_serializer =
                        value::ValueSerializer::new(key, self.1).unwrap();
                    try!(value.serialize(value_serializer));
                }
                self.0 = Some(None);
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'target, Target> ::serde::ser::SerializeTupleStruct for SerializeTuple<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        ::serde::ser::SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'target, Target> ::serde::ser::SerializeTupleVariant for PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }
}

impl<'target, Target> ::serde::ser::SerializeMap for PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_value<T: ?Sized + ::serde::ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }
}

impl<'target, Target> ::serde::ser::SerializeStruct for PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }
}

impl<'target, Target> ::serde::ser::SerializeStructVariant for PairSerializer<'target, Target>
    where Target: 'target + form_urlencoded::Target
{
    type Ok = ();
    type Error = Error;


    fn serialize_field<T: ?Sized + ::serde::ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::unsupported_pair())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::unsupported_pair())
    }
}

impl Error {
    fn unsupported_pair() -> Self {
        Error::Custom("unsupported pair".to_string())
    }
}

//! Serialization support for the `application/x-www-form-urlencoded` format.

mod key;
mod pair;
mod value;

use serde::ser;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::str;
use url::form_urlencoded::Serializer as UrlEncodedSerializer;
use url::form_urlencoded::Target as UrlEncodedTarget;

/// Serializes a value into a `application/x-wwww-url-encoded` `String` buffer.
///
/// ```
/// let meal = &[
///     ("bread", "baguette"),
///     ("cheese", "comté"),
///     ("meat", "ham"),
///     ("fat", "butter"),
/// ];
///
/// assert_eq!(
///     serde_urlencoded::to_string(meal),
///     Ok("bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter".to_owned()));
/// ```
pub fn to_string<T: ser::Serialize>(input: &T) -> Result<String, Error> {
    let mut output = String::new();
    {
        let mut urlencoder = UrlEncodedSerializer::new(&mut output);
        try!(input.serialize(Serializer::new(&mut urlencoder)));
    }
    Ok(output)
}

/// A serializer for the `application/x-www-form-urlencoded` format.
///
/// * Supported top-level inputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Supported keys and values are integers, bytes (if convertible to strings),
///   unit structs and unit variants.
///
/// * Newtype structs defer to their inner values.
pub struct Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    urlencoder: &'output mut UrlEncodedSerializer<Target>
}

impl<'output, Target> Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    /// Returns a new `Serializer`.
    pub fn new(urlencoder: &'output mut UrlEncodedSerializer<Target>) -> Self {
        Serializer { urlencoder: urlencoder }
    }
}

/// Errors returned during serializing to `application/x-www-form-urlencoded`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Custom(String),
    Utf8(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Custom(ref msg) => msg.fmt(f),
            Error::Utf8(ref err) => write!(f, "invalid UTF-8: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Custom(ref msg) => msg,
            Error::Utf8(ref err) => error::Error::description(err),
        }
    }

    /// The lower-level cause of this error, in the case of a `Utf8` error.
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Custom(_) => None,
            Error::Utf8(ref err) => Some(err),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl<'output, Target> ser::Serializer for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = SerializeMap<'output, Target>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str)
            -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
            self,
            _name: &'static str,
            value: &T)
            -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _value: &T)
            -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(self)
    }

    fn serialize_seq_fixed_size(self, _length: usize) -> Result<Self::SerializeSeq, Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Err(Error::top_level())
    }

    fn serialize_tuple_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::top_level())
    }

    fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::top_level())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(SerializeMap { urlencoder: self.urlencoder, key: None })
    }

    fn serialize_struct(
            self, _name: &'static str, _len: usize)
            -> Result<Self::SerializeStruct, Error> {
        Err(Error::top_level())
    }

    fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: usize,
            _variant: &'static str,
            _len: usize)
            -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::top_level())
    }
}

impl<'output, Target> ser::SerializeSeq for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(pair::PairSerializer::new(self.urlencoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'output, Target> ser::SerializeTuple for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::top_level())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }
}

impl<'output, Target> ser::SerializeTupleStruct for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::top_level())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }
}

impl<'output, Target> ser::SerializeTupleVariant for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::top_level())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }
}

pub struct SerializeMap<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    urlencoder: &'output mut UrlEncodedSerializer<Target>,
    key: Option<Cow<'static, str>>,
}

impl<'output, Target> ser::SerializeMap for SerializeMap<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        key.serialize(key::MapKeySerializer::new(&mut self.key))
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let value_serializer =
            try!(value::ValueSerializer::new(&mut self.key, self.urlencoder));
        value.serialize(value_serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'output, Target> ser::SerializeStruct for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> {
        let mut key = Some(key.into());
        let value_serializer =
            value::ValueSerializer::new(&mut key, self.urlencoder).unwrap();
        value.serialize(value_serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'output, Target> ser::SerializeStructVariant for Serializer<'output, Target>
    where Target: 'output + UrlEncodedTarget
{
    type Ok = ();
    type Error = Error;


    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> {
        Err(Error::top_level())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::top_level())
    }
}

impl Error {
    fn top_level() -> Self {
        Error::Custom("top-level serializer supports only maps and structs".to_string())
    }
}

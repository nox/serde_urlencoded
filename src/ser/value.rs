use dtoa;
use itoa;
use ser::Error;
use serde::ser::Serialize;
use serde::ser;
use std::str;
use url::form_urlencoded::Serializer as UrlEncodedSerializer;
use url::form_urlencoded::Target as UrlEncodedTarget;

pub struct ValueSerializer<'key, 'target, Target>
    where Target: 'target + UrlEncodedTarget,
{
    urlencoder: &'target mut UrlEncodedSerializer<Target>,
    key: &'key str,
}

impl<'key, 'target, Target> ValueSerializer<'key, 'target, Target>
    where Target: 'target + UrlEncodedTarget,
{
    pub fn new(urlencoder: &'target mut UrlEncodedSerializer<Target>,
               key: &'key str)
               -> Self {
        ValueSerializer {
            urlencoder: urlencoder,
            key: key,
        }
    }

    fn serialize_str(self, value: &str) -> Result<(), Error> {
        self.urlencoder.append_pair(self.key, value);
        Ok(())
    }

    fn serialize_static_str(self, value: &'static str) -> Result<(), Error> {
        self.serialize_str(value)
    }

    fn serialize_string(self, value: String) -> Result<(), Error> {
        self.serialize_str(&value)
    }

    fn serialize_none(self) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self,
                                             value: &T)
                                             -> Result<(), Error> {
        value.serialize(self)
    }

    fn serialize_integer<I>(self, value: I) -> Result<(), Error>
        where I: itoa::Integer,
    {
        let mut buf = [b'\0'; 20];
        let len = itoa::write(&mut buf[..], value).unwrap();
        let part = unsafe { str::from_utf8_unchecked(&buf[0..len]) };
        ser::Serializer::serialize_str(self, part)
    }

    fn serialize_floating<F>(self, value: F) -> Result<(), Error>
        where F: dtoa::Floating,
    {
        let mut buf = [b'\0'; 24];
        let len = dtoa::write(&mut buf[..], value).unwrap();
        let part = unsafe { str::from_utf8_unchecked(&buf[0..len]) };
        ser::Serializer::serialize_str(self, part)
    }

    fn unsupported(self) -> Error {
        Error::Custom("unsupported value".into())
    }
}

impl<'key, 'target, Target> ser::Serializer for ValueSerializer<'key, 'target, Target>
    where Target: 'target + UrlEncodedTarget,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Error> {
        self.serialize_static_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Error> {
        self.serialize_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Error> {
        self.serialize_floating(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Error> {
        self.serialize_floating(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Error> {
        self.serialize_string(v.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Error> {
        self.serialize_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Error> {
        match str::from_utf8(value) {
            Ok(value) => self.serialize_str(value),
            Err(err) => Err(Error::Utf8(err)),
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok, Error> {
        Err(self.unsupported())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Error> {
        self.serialize_static_str(name.into())
    }

    fn serialize_unit_variant(self,
                              _name: &'static str,
                              _variant_index: u32,
                              variant: &'static str)
                              -> Result<Self::Ok, Error> {
        self.serialize_static_str(variant.into())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>
        (self,
         _name: &'static str,
         value: &T)
         -> Result<Self::Ok, Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>
        (self,
         _name: &'static str,
         _variant_index: u32,
         _variant: &'static str,
         _value: &T)
         -> Result<Self::Ok, Error> {
        Err(self.unsupported())
    }

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        self.serialize_none()
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self,
                                                  value: &T)
                                                  -> Result<Self::Ok, Error> {
        self.serialize_some(value)
    }

    fn serialize_seq(self,
                     _len: Option<usize>)
                     -> Result<Self::SerializeSeq, Error> {
        Ok(self)
    }

    fn serialize_tuple(self,
                       _len: usize)
                       -> Result<Self::SerializeTuple, Error> {
        Err(self.unsupported())
    }

    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              _len: usize)
                              -> Result<Self::SerializeTuple, Error> {
        Err(self.unsupported())
    }

    fn serialize_tuple_variant
        (self,
         _name: &'static str,
         _variant_index: u32,
         _variant: &'static str,
         _len: usize)
         -> Result<Self::SerializeTupleVariant, Error> {
        Err(self.unsupported())
    }

    fn serialize_map(self,
                     _len: Option<usize>)
                     -> Result<Self::SerializeMap, Error> {
        Err(self.unsupported())
    }

    fn serialize_struct(self,
                        _name: &'static str,
                        _len: usize)
                        -> Result<Self::SerializeStruct, Error> {
        Err(self.unsupported())
    }

    fn serialize_struct_variant
        (self,
         _name: &'static str,
         _variant_index: u32,
         _variant: &'static str,
         _len: usize)
         -> Result<Self::SerializeStructVariant, Error> {
        Err(self.unsupported())
    }
}

impl<'key, 'target, Target> ser::SerializeSeq for ValueSerializer<'key, 'target, Target>
    where Target: 'target + UrlEncodedTarget,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self,
                                                     value: &T)
                                                     -> Result<(), Error> {
        let v = ValueSerializer::new(self.urlencoder, self.key);
        value.serialize(v)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Error> {
        Ok(Self::Ok::default())
    }
}

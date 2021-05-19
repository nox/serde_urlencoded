use crate::ser::part::{PartSerializer, Sink};
use crate::ser::Error;
use form_urlencoded::Serializer as UrlEncodedSerializer;
use form_urlencoded::Target as UrlEncodedTarget;
use serde::ser::{Serialize, SerializeSeq};
use std::str;

pub struct ValueSink<'input, 'key, 'target, Target>
where
    Target: UrlEncodedTarget,
{
    urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
    key: &'key str,
}

impl<'input, 'key, 'target, Target> ValueSink<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    pub fn new(
        urlencoder: &'target mut UrlEncodedSerializer<'input, Target>,
        key: &'key str,
    ) -> Self {
        ValueSink { urlencoder, key }
    }
}

pub struct ValueSinkSeqSerializer<'input, 'key, 'target, Target>
where
    Target: UrlEncodedTarget,
{
    sink: ValueSink<'input, 'key, 'target, Target>
}

impl <'input, 'key, 'target, Target> SerializeSeq
for ValueSinkSeqSerializer<'input, 'key, 'target, Target> 
where Target: UrlEncodedTarget {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize {
        value.serialize(PartSerializer::new(ValueSink::new(self.sink.urlencoder, self.sink.key)))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'input, 'key, 'target, Target> Sink
    for ValueSink<'input, 'key, 'target, Target>
where
    Target: 'target + UrlEncodedTarget,
{
    type Ok = ();
    type SerSeq = ValueSinkSeqSerializer<'input, 'key, 'target, Target>;

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

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Error> {
        value.serialize(PartSerializer::new(self))
    }

    fn unsupported(self) -> Error {
        Error::Custom("unsupported value".into())
    }

    fn serialize_seq(self) -> Result<Self::SerSeq, Error> {
        Ok(ValueSinkSeqSerializer{sink: self})
    }
}

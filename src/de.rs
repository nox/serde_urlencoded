//! Deserialization support for the `application/x-www-form-urlencoded` format.

use serde::de;

#[doc(inline)]
pub use serde::de::value::Error;
use serde::de::value::MapDeserializer;
use serde::de::IntoDeserializer;

use std::borrow::Cow;
use std::io::Read;

use url::form_urlencoded::Parse as UrlEncodedParse;
use url::form_urlencoded::parse;



/// Deserializes a `application/x-wwww-url-encoded` value from a `&[u8]`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_urlencoded::from_bytes::<Vec<(String, String)>>(
///         b"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"),
///     Ok(meal));
/// ```
pub fn from_bytes<'de, T: de::Deserialize<'de>>(input: &'de [u8]) -> Result<T, Error> {
    T::deserialize(Deserializer::new(parse(input)))
}

/// Deserializes a `application/x-wwww-url-encoded` value from a `&str`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_urlencoded::from_str::<Vec<(String, String)>>(
///         "bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"),
///     Ok(meal));
/// ```
pub fn from_str<'de, T: de::Deserialize<'de>>(input: &'de str) -> Result<T, Error> {
    from_bytes(input.as_bytes())
}

/// Convenience function that reads all bytes from `reader` and deserializes
/// them with `from_bytes`.
pub fn from_reader<T, R>(mut reader: R) -> Result<T, Error>
    where T: de::DeserializeOwned, R: Read
{
    let mut buf = vec![];
    reader.read_to_end(&mut buf)
        .map_err(|e| {
            de::Error::custom(format_args!("could not read input: {}", e))
        })?;
    from_bytes(&buf)
}

pub struct ParseWrapper<'a>(UrlEncodedParse<'a>);

impl<'a> ::std::iter::Iterator for ParseWrapper<'a> {
    type Item = (ParsableStr<'a>, ParsableStr<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some((k, v)) => Some((ParsableStr(k), ParsableStr(v))),
            None => None
        }
    }
}

pub struct ParsableStr<'a>(Cow<'a, str>);
pub struct ParsableStrDeserializer<'a>(Cow<'a, str>);

impl<'de> IntoDeserializer<'de> for ParsableStr<'de> {
    type Deserializer = ParsableStrDeserializer<'de>;
    fn into_deserializer(self) -> Self::Deserializer {
        ParsableStrDeserializer(self.0)
    }
}

macro_rules! forward_parsable_to_deserialize_x {
    ($($ty:ident => $meth:ident,)*) => {
        $(
            fn $meth<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: de::Visitor<'de> {
                match self.0.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$meth(visitor),
                    Err(e) => Err(de::Error::custom(e))
                }
            }
        )*
    }
}

impl<'de> de::Deserializer<'de> for ParsableStrDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        self.0.into_deserializer().deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        char
        str
        string
        unit
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        map
        seq
        struct
        identifier
        tuple
        enum
        ignored_any
    }

    forward_parsable_to_deserialize_x! {
        bool => deserialize_bool,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }
}

/// A deserializer for the `application/x-www-form-urlencoded` format.
///
/// * Supported top-level outputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Main `deserialize` methods defers to `deserialize_map`.
///
/// * Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`
///   defers to `deserialize`.
pub struct Deserializer<'a> {
    inner: MapDeserializer<'a, ParseWrapper<'a>, Error>,
}

impl<'a> Deserializer<'a> {
    /// Returns a new `Deserializer`.
    pub fn new(parser: UrlEncodedParse<'a>) -> Self {
        Deserializer { inner: MapDeserializer::new(ParseWrapper(parser)) }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        visitor.visit_map(self.inner)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        visitor.visit_seq(self.inner)
    }

    forward_to_deserialize_any! {
        bool
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        unit
        option
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        identifier
        tuple
        enum
        ignored_any
    }
}

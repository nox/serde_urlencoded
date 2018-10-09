//! Deserialization support for the `application/x-www-form-urlencoded` format.

use serde::de::{self, IntoDeserializer};
use serde::de::Error as de_Error;
use serde::de::value::MapDeserializer;
use std::borrow::Cow;
use std::io::Read;
use url::form_urlencoded::Parse as UrlEncodedParse;
use url::form_urlencoded::parse;

#[doc(inline)]
pub use serde::de::value::Error;

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
pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, Error>
    where T: de::Deserialize<'de>,
{
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
pub fn from_str<'de, T>(input: &'de str) -> Result<T, Error>
    where T: de::Deserialize<'de>,
{
    from_bytes(input.as_bytes())
}

/// Convenience function that reads all bytes from `reader` and deserializes
/// them with `from_bytes`.
pub fn from_reader<T, R>(mut reader: R) -> Result<T, Error>
    where T: de::DeserializeOwned,
          R: Read,
{
    let mut buf = vec![];
    reader.read_to_end(&mut buf)
        .map_err(|e| {
            de::Error::custom(format_args!("could not read input: {}", e))
        })?;
    from_bytes(&buf)
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
pub struct Deserializer<'de> {
    parser: UrlEncodedParse<'de>,
}

impl<'de> Deserializer<'de> {
    /// Returns a new `Deserializer`.
    pub fn new(parser: UrlEncodedParse<'de>) -> Self {
        Deserializer {
            parser
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        let mut pairs = Vec::new();
        for (k, v) in self.parser {
            if let Some(index) = pairs.iter().position(|&(Part(ref pk), _): &(Part<'de>, Value<'de>)| *pk == k) {
                pairs[index].1.append(v);
            } else {
                pairs.push((Part(k), Value::Single(v)));
            }
        }
        let inner = MapDeserializer::new(pairs.into_iter());
        visitor.visit_map(inner)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        let inner = MapDeserializer::new(PartIterator(self.parser));
        visitor.visit_seq(inner)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V:  de::Visitor<'de>,
    {
        let inner = MapDeserializer::new(PartIterator(self.parser));
        inner.end()?;
        visitor.visit_unit()
    }

    forward_to_deserialize_any! {
        bool
        u8
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

struct PartIterator<'de>(UrlEncodedParse<'de>);

impl<'de> Iterator for PartIterator<'de> {
    type Item = (Part<'de>, Part<'de>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (Part(k), Part(v)))
    }
}

struct Part<'de>(Cow<'de, str>);

enum Value<'de> {
    Single(Cow<'de, str>),
    Multiple(Vec<Cow<'de, str>>),
}

impl<'de> Value<'de> {
    fn append(&mut self, value: Cow<'de, str>) {
        if let Value::Single(_) = self {
            let mut v = Value::Multiple(vec![]);
            ::std::mem::swap(self, &mut v);
            match self {
                Value::Multiple(ref mut values) =>
                    match v {
                        Value::Single(prev) => values.push(prev),
                        _ => {},
                    }
                _ => {},
            }
        }
        match self {
            Value::Multiple(ref mut values) => values.push(value),
            _ => {},
        }
    }
}

impl<'de> IntoDeserializer<'de> for Part<'de>
{
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> IntoDeserializer<'de> for Value<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_parsed_key {
    ($($ty:ident => $method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                match self.0.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$method(visitor),
                    Err(e) => Err(de::Error::custom(e))
                }
            }
        )*
    }
}

macro_rules! forward_parsed_value {
    ($($ty:ident => $method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: de::Visitor<'de>
            {
                match self {
                    Value::Single(s) =>
                        match s.parse::<$ty>() {
                            Ok(val) => val.into_deserializer().$method(visitor),
                            Err(e) => Err(de::Error::custom(e))
                        }
                    Value::Multiple(_) => {
                        Err(de::Error::duplicate_field(""))
                    }
                }
            }
        )*
    }
}

impl<'de> de::Deserializer<'de> for Part<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        self.0.into_deserializer().deserialize_any(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        visitor.visit_enum(ValueEnumAccess(self.0))
    }

    forward_to_deserialize_any! {
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        identifier
        tuple
        ignored_any
        seq
        map
    }

    forward_parsed_key! {
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

impl<'de> de::Deserializer<'de> for Value<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        match self {
            Value::Single(v) => v.into_deserializer().deserialize_any(visitor),
            Value::Multiple(s) => s.into_deserializer().deserialize_any(visitor),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        match self {
            Value::Single(v) => visitor.visit_enum(ValueEnumAccess(v)),
            Value::Multiple(_) => Err(de::Error::duplicate_field("")),
        }
    }

    forward_to_deserialize_any! {
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
        newtype_struct
        tuple_struct
        struct
        identifier
        tuple
        ignored_any
        seq
        map
    }

    forward_parsed_value! {
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

struct ValueEnumAccess<'de>(Cow<'de, str>);

impl<'de> de::EnumAccess<'de> for ValueEnumAccess<'de> {
    type Error = Error;
    type Variant = UnitOnlyVariantAccess;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error>
        where V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.0.into_deserializer())?;
        Ok((variant, UnitOnlyVariantAccess))
    }
}

struct UnitOnlyVariantAccess;

impl<'de> de::VariantAccess<'de> for UnitOnlyVariantAccess {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
        where T: de::DeserializeSeed<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }
}

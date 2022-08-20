//! Deserialization support for the `application/x-www-form-urlencoded` format.

use form_urlencoded::parse;
use form_urlencoded::Parse as UrlEncodedParse;
use serde::de::Error as de_Error;
use serde::de::{
    self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess,
};
use serde::forward_to_deserialize_any;
use std::borrow::Cow;
use std::io::Read;

#[doc(inline)]
pub use serde::de::value::Error;

/// Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`.
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
where
    T: de::Deserialize<'de>,
{
    T::deserialize(Deserializer::new(parse(input)))
}

/// Deserializes a `application/x-www-form-urlencoded` value from a `&str`.
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
where
    T: de::Deserialize<'de>,
{
    from_bytes(input.as_bytes())
}

/// Convenience function that reads all bytes from `reader` and deserializes
/// them with `from_bytes`.
pub fn from_reader<T, R>(mut reader: R) -> Result<T, Error>
where
    T: de::DeserializeOwned,
    R: Read,
{
    let mut buf = vec![];
    reader.read_to_end(&mut buf).map_err(|e| {
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
    inner: PartIterator<'de>,
}

impl<'de> Deserializer<'de> {
    /// Returns a new `Deserializer`.
    pub fn new(parser: UrlEncodedParse<'de>) -> Self {
        Deserializer {
            inner: PartIterator::new(parser),
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self.inner)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self.inner)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.inner.end()?;
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

struct PartIterator<'de> {
    iter: UrlEncodedParse<'de>,
    pair: Option<(Part<'de>, Part<'de>)>,
    count: usize,
}

impl<'de> PartIterator<'de> {
    fn new(iter: UrlEncodedParse<'de>) -> Self {
        PartIterator {
            iter,
            pair: None,
            count: 0,
        }
    }

    fn end(self) -> Result<(), Error> {
        let remaining = self.iter.count();
        if remaining == 0 {
            Ok(())
        } else {
            Err(Error::invalid_length(
                self.count + remaining,
                &"0 elements in sequence",
            ))
        }
    }
}

impl<'de> Iterator for PartIterator<'de> {
    type Item = (Part<'de>, Part<'de>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next().map(|(k, v)| (Part(k), Part(v))) {
            Some(pair) => {
                self.count += 1;
                Some(pair)
            }
            None => None,
        }
    }
}

impl<'de> MapAccess<'de> for PartIterator<'de> {
    type Error = Error;

    fn next_key_seed<K>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.next() {
            Some((key, value)) => {
                self.pair = Some((key.clone(), value));
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let (key, value) = self
            .pair
            .take()
            .expect("MapAccess::next_value called before next_key");

        seed.deserialize(value.into_deserializer()).map_err(|err| {
            Error::custom(format_args!(
                "failed to parse value for key '{}': {err}",
                key.0,
            ))
        })
    }
}

impl<'de> SeqAccess<'de> for PartIterator<'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.next() {
            Some((key, value)) => {
                let de = PairDeserializer(key, value);
                seed.deserialize(de).map(Some)
            }
            None => Ok(None),
        }
    }
}

struct PairDeserializer<'de>(Part<'de>, Part<'de>);

impl<'de> de::Deserializer<'de> for PairDeserializer<'de> {
    type Error = Error;

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct tuple_struct map
        struct enum identifier ignored_any
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let PairDeserializer(key, value) = self;

        let mut pair_visitor = PairVisitor(Some(key.clone()), Some(value));
        match visitor.visit_seq(&mut pair_visitor) {
            Ok(pair) if pair_visitor.1.is_none() => Ok(pair),
            Ok(_pair) => Err(de::Error::invalid_length(1, &"2")),
            Err(err) => Err(Error::custom(format_args!(
                "failed to parse value for key '{}': {err}",
                key.0
            ))),
        }
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if len == 2 {
            self.deserialize_seq(visitor)
        } else {
            Err(de::Error::invalid_length(len, &"a pair of values"))
        }
    }
}

struct PairVisitor<'de>(Option<Part<'de>>, Option<Part<'de>>);

impl<'de> SeqAccess<'de> for PairVisitor<'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(key) = self.0.take() {
            seed.deserialize(key.into_deserializer()).map(Some)
        } else if let Some(value) = self.1.take() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
struct Part<'de>(Cow<'de, str>);

impl<'de> IntoDeserializer<'de> for Part<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_parsed_value {
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

impl<'de> de::Deserializer<'de> for Part<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.0 {
            Cow::Borrowed(value) => visitor.visit_borrowed_str(value),
            Cow::Owned(value) => visitor.visit_string(value),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(ValueEnumAccess(self.0))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
        char
        str
        string
        unit
        bytes
        byte_buf
        unit_struct
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
    where
        V: de::DeserializeSeed<'de>,
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
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn tuple_variant<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::custom("expected unit variant"))
    }
}

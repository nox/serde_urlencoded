//! `x-www-form-urlencoded` meets Serde

#![warn(unused_extern_crates)]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::{from_bytes, from_reader, from_str, Deserializer};
#[doc(inline)]
pub use ser::{to_string, Serializer};

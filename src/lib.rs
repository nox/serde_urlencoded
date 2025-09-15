//! `x-www-form-urlencoded` meets Serde

#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]

extern crate serde_core as serde;

pub mod de;
pub mod ser;

#[doc(inline)]
pub use crate::de::{from_bytes, from_reader, from_str, Deserializer};
#[doc(inline)]
pub use crate::ser::{to_string, Serializer};

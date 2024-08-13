//! `x-www-form-urlencoded` meets Serde
#![no_std]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod de;
pub mod ser;

#[cfg(feature = "std")]
#[doc(inline)]
pub use crate::de::from_reader;
#[doc(inline)]
pub use crate::de::{from_bytes, from_str, Deserializer};
#[doc(inline)]
pub use crate::ser::{to_string, Serializer};

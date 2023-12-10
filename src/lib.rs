//! Wrapper types to enable optimized handling of `[u8; N]`, `&[u8]` and `Vec<u8>`.
//!
//! This crate is a fork of [`serde_bytes`](https://crates.io/crates/serde_bytes) that adds
//! support for `[u8; N]` and `&[u8; N]`.
//!
//! ```
//! # use serde_derive::{Deserialize, Serialize};
//!#[derive(Deserialize, Serialize)]
//! struct Efficient<'a> {
//!     #[serde(with = "serde_bytes_ng")]
//!     bytes: &'a [u8],
//!
//!     #[serde(with = "serde_bytes_ng")]
//!     byte_buf: Vec<u8>,
//!
//!     #[serde(with = "serde_bytes_ng")]
//!     byte_array: [u8; 188],
//!
//!     #[serde(with = "serde_bytes_ng", borrow)]
//!     byte_array_ref: &'a [u8; 188],
//!}
//! ```

#![doc(html_root_url = "https://docs.rs/serde_bytes_ng/0.11.12")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![allow(
    clippy::into_iter_without_iter,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_doctest_main
)]

mod bytes;
mod de;
mod ser;

#[cfg(any(feature = "std", feature = "alloc"))]
mod bytebuf;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "std", feature = "alloc"))]
use serde::Deserializer;

use serde::Serializer;

pub use crate::bytes::Bytes;
pub use crate::de::Deserialize;
pub use crate::ser::Serialize;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use crate::bytebuf::ByteBuf;

/// Serde `serialize_with` function to serialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_bytes_ng")]`
/// - `#[serde(serialize_with = "serde_bytes_ng::serialize")]`
///
/// ```
/// # use serde_derive::Serialize;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Efficient<'a> {
///     #[serde(with = "serde_bytes_ng")]
///     bytes: &'a [u8],
///
///     #[serde(with = "serde_bytes_ng")]
///     byte_buf: Vec<u8>,
/// }
/// ```
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + Serialize,
    S: Serializer,
{
    Serialize::serialize(bytes, serializer)
}

/// Serde `deserialize_with` function to deserialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_bytes_ng")]`
/// - `#[serde(deserialize_with = "serde_bytes_ng::deserialize")]`
///
/// ```
/// # use serde_derive::Deserialize;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Packet {
///     #[serde(with = "serde_bytes_ng")]
///     payload: Vec<u8>,
/// }
/// ```
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
}

// This is free and unencumbered software released into the public domain.

#![allow(unused_imports)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub use alloc::{borrow::Cow, string::String, vec::Vec};

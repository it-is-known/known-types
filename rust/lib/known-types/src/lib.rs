// This is free and unencumbered software released into the public domain.

//! C type aliases and shared social-handle validation.
//!
//! - [`c`] provides C-compatible primitive aliases and borrowed C strings.
//! - [`handle`] documents the common handle contract and provides validation
//!   helpers and errors.
//!
//! Platform types live in their own crates, such as
//! [`known-types-x`](https://docs.rs/known-types-x) and
//! [`known-types-linkedin`](https://docs.rs/known-types-linkedin); they are not
//! re-exported here. Both modules in this crate work without default features
//! or heap allocation.
//!
//! ```
//! use known_types::c;
//!
//! let name = c::Str::from_bytes_with_nul(b"known\0")?;
//! assert_eq!(name.to_bytes(), b"known");
//! assert!(c::Str::from_bytes_with_nul(b"missing terminator").is_err());
//! # Ok::<(), c::FromBytesWithNulError>(())
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::alloc_instead_of_core)]

//#[cfg(doctest)]
//#[doc = include_str!("../README.md")]
//pub struct ReadmeDoctests;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod c;
pub mod handle;

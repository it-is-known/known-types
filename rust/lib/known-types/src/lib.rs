// This is free and unencumbered software released into the public domain.

//! This crate provides well-known types.
//!
//! ```edition2024
//! # use known_types::*;
//! ```

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::alloc_instead_of_core)]

#[cfg(doctest)]
#[doc = include_str!("../../../README.md")]
pub struct ReadmeDoctests;

mod prelude;

pub mod c;

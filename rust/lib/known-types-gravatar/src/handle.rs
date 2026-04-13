// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use crate::prelude::String;
use derive_more::{AsRef, Display, From, FromStr};

#[derive(AsRef, Clone, Debug, Display, Eq, From, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GravatarHandle(String);

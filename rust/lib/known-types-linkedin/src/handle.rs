// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use crate::prelude::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
use percent_encoding::percent_decode_str;

#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinkedinHandle(String);

impl From<String> for LinkedinHandle {
    fn from(input: String) -> Self {
        Self((&input).into())
    }
}

impl From<&str> for LinkedinHandle {
    fn from(input: &str) -> Self {
        Self(input.parse().expect("should be a valid Linkedin handle"))
    }
}

impl FromStr for LinkedinHandle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        if input.is_empty() {
            return Err(());
        }
        let decoded = percent_decode_str(input)
            .decode_utf8()
            .map_err(|_| ())?
            .into_owned();
        Ok(Self(decoded))
    }
}

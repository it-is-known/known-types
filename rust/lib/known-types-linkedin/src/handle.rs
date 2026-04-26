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
        Self::from_str(&input).expect("should be a valid Linkedin handle")
    }
}

impl From<&str> for LinkedinHandle {
    fn from(input: &str) -> Self {
        Self::from_str(input).expect("should be a valid Linkedin handle")
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

#[test]
fn test_linkedin_handle_from_str() {
    assert_eq!(
        LinkedinHandle::from_str("foobar"),
        Ok(LinkedinHandle("foobar".into()))
    );
}

#[test]
fn test_linkedin_handle_parse_ascii() {
    assert_eq!(
        LinkedinHandle::from_str("foobar"),
        Ok(LinkedinHandle("foobar".into()))
    );
}

#[test]
fn test_linkedin_handle_parse_encoded() {
    assert_eq!(
        LinkedinHandle::from_str("bj%C3%B6rn"),
        Ok(LinkedinHandle("björn".into()))
    );
}

#[test]
fn test_linkedin_handle_from_str_encoded() {
    assert_eq!(
        LinkedinHandle::from("bj%C3%B6rn"),
        LinkedinHandle("björn".into())
    );
}

#[test]
fn test_linkedin_handle_from_string_encoded() {
    use crate::prelude::ToString;
    assert_eq!(
        LinkedinHandle::from("bj%C3%B6rn".to_string()),
        LinkedinHandle("björn".into())
    );
}

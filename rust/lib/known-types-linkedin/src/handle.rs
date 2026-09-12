// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
use percent_encoding::percent_decode_str;

/// A LinkedIn handle (aka username).
///
/// With the `async-graphql` feature, this is a string scalar named `LinkedinHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// Scalar input uses `FromStr` to validate and normalize the handle, while
/// cursors preserve the stored string verbatim.
#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct LinkedinHandle(String);

#[cfg(feature = "async-graphql")]
#[async_graphql::Scalar(name = "LinkedinHandle")]
impl async_graphql::ScalarType for LinkedinHandle {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(value) => Self::from_str(&value)
                .map_err(|_| async_graphql::InputValueError::custom("invalid LinkedIn handle")),
            value => Err(async_graphql::InputValueError::expected_type(value)),
        }
    }

    fn is_valid(value: &async_graphql::Value) -> bool {
        matches!(value, async_graphql::Value::String(value) if Self::from_str(value).is_ok())
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.clone())
    }
}

#[cfg(feature = "async-graphql")]
impl async_graphql::connection::CursorType for LinkedinHandle {
    type Error = core::convert::Infallible;

    fn decode_cursor(input: &str) -> Result<Self, Self::Error> {
        // Cursors contain stored text: parsing again would double-decode
        // percent escapes or trim meaningful whitespace.
        Ok(Self(input.into()))
    }

    fn encode_cursor(&self) -> String {
        self.0.clone()
    }
}

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
    use alloc::string::ToString;
    assert_eq!(
        LinkedinHandle::from("bj%C3%B6rn".to_string()),
        LinkedinHandle("björn".into())
    );
}

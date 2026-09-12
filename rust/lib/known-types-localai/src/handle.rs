// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use derive_more::{AsRef, Display, From, FromStr};

/// A local.ai handle (aka username).
///
/// With the `async-graphql` feature, this is a string scalar named `LocalaiHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// Cursors preserve the stored string verbatim.
#[derive(AsRef, Clone, Debug, Display, Eq, From, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct LocalaiHandle(String);

#[cfg(feature = "async-graphql")]
#[async_graphql::Scalar(name = "LocalaiHandle")]
impl async_graphql::ScalarType for LocalaiHandle {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::String(value) => Ok(Self(value)),
            value => Err(async_graphql::InputValueError::expected_type(value)),
        }
    }

    fn is_valid(value: &async_graphql::Value) -> bool {
        matches!(value, async_graphql::Value::String(_))
    }

    fn to_value(&self) -> async_graphql::Value {
        async_graphql::Value::String(self.0.clone())
    }
}

#[cfg(feature = "async-graphql")]
impl async_graphql::connection::CursorType for LocalaiHandle {
    type Error = core::convert::Infallible;

    fn decode_cursor(input: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(input))
    }

    fn encode_cursor(&self) -> String {
        self.0.clone()
    }
}

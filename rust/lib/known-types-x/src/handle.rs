// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use derive_more::{AsRef, Display, From, FromStr};

/// An X handle (aka username).
///
/// With the `sqlx` feature, this implements SQLx's `Type`, `Encode`, and
/// `Decode` traits transparently over `String`.
#[derive(AsRef, Clone, Debug, Display, Eq, From, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct XHandle(String);

#[cfg(feature = "async-graphql")]
impl async_graphql::connection::CursorType for XHandle {
    type Error = core::convert::Infallible;

    fn decode_cursor(input: &str) -> Result<Self, Self::Error> {
        Ok(Self::from(input))
    }

    fn encode_cursor(&self) -> String {
        self.0.clone()
    }
}

#[cfg(feature = "libsql")]
impl Into<libsql::Value> for XHandle {
    fn into(self) -> libsql::Value {
        libsql::Value::Text(self.0)
    }
}

#[cfg(feature = "libsql")]
impl Into<libsql::Value> for &XHandle {
    fn into(self) -> libsql::Value {
        libsql::Value::Text(self.0.clone())
    }
}

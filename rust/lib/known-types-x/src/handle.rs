// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use crate::prelude::String;
use derive_more::{AsRef, Display, From, FromStr};

#[derive(AsRef, Clone, Debug, Display, Eq, From, FromStr, Hash, Ord, PartialEq, PartialOrd)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl libsql::params::IntoValue for XHandle {
    fn into_value(self) -> libsql::Result<libsql::Value> {
        Ok(libsql::Value::Text(self.0))
    }
}

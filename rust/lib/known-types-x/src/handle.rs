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

#[cfg(feature = "sqlx-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-postgres")))]
impl sqlx::Type<sqlx::Postgres> for XHandle
where
    for<'x> &'x str: sqlx::Type<sqlx::Postgres>,
{
    #[inline]
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }

    #[inline]
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-postgres")))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for XHandle {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        sqlx::Encode::<'_, sqlx::Postgres>::encode_by_ref(&self.0.as_str(), buf)
    }

    #[inline]
    fn produces(&self) -> Option<<sqlx::Postgres as sqlx::Database>::TypeInfo> {
        <&str as sqlx::Encode<'_, sqlx::Postgres>>::produces(&self.0.as_str())
    }

    #[inline]
    fn size_hint(&self) -> usize {
        <&str as sqlx::Encode<'_, sqlx::Postgres>>::size_hint(&self.0.as_str())
    }
}

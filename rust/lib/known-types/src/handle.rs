// This is free and unencumbered software released into the public domain.

//! Shared validation and errors for social media handles.
//!
//! The platform crates expose owned handle types behind their `alloc` feature.
//! Each type documents its own syntax, normalization, length bounds, and upstream
//! references. See, for example, [`XHandle`] and [`LinkedinHandle`].
//!
//! # Construction and migration
//!
//! Construct a handle with [`FromStr`](core::str::FromStr), `TryFrom<&str>`, or
//! `TryFrom<String>`. The inner string is private: `as_str()` borrows the stored
//! spelling, `Display` prints it, and `into_string()` consumes the handle to
//! recover it. Parsing the stored spelling again preserves it exactly.
//!
//! Parsing and fallible conversions return [`ParseHandleError`]. Serde, GraphQL,
//! and SQLx decoders use the same parser and report failures through their own
//! error types; GraphQL cursor decoding returns `ParseHandleError` directly.
//! Validation is local: it does not establish account availability, ownership,
//! canonical capitalization, or acceptance of registration-only reserved names.
//!
//! To migrate from the former infallible `From` conversions (which could panic
//! for LinkedIn), replace `Handle::from(text)` / `text.into()` with
//! `text.parse()?` or `Handle::try_from(text)?`.
//!
//! # Length, normalization, and identity
//!
//! `MIN_LENGTH` and `MAX_LENGTH` are inclusive representational bounds, not
//! necessarily current registration limits. X accepts legacy handles up to 20
//! characters despite a current registration maximum of 15; Telegram includes
//! four-character collectible usernames. Intro.co, local.ai, and Luma use
//! documented 1–100 fallback bounds where an upstream contract is unavailable.
//!
//! Length counts Unicode scalar values after parser normalization, not UTF-8
//! bytes or grapheme clusters. Platforms accepting a displayed `@` remove it
//! exactly once. LinkedIn instead trims outer whitespace and strictly
//! percent-decodes UTF-8 once before validation; malformed escapes and decoded
//! forbidden characters are rejected.
//!
//! Equality, ordering, and hashing agree about identity. Some types lowercase
//! their stored spelling; others preserve it while comparing case-insensitively.
//! Facebook additionally ignores periods. The three fallback types are
//! case-sensitive. Consult the type's contract before using handles as keys.
//!
//! # Integrations
//!
//! These features are implemented by all handle crates and are opt-in. The
//! `all` feature is empty. Default features enable `std`, which enables `alloc`;
//! for `no_std` handles with Serde, explicitly select `alloc,serde`.
//!
//! Feature | Behavior
//! --- | ---
//! `serde` | Serialize as a string; validate and normalize when deserializing. Requires `alloc` for the handle type.
//! `async-graphql` | String scalar and connection cursor; enables `std` and `alloc` without requiring the handle crate's `serde` feature.
//! `sqlx` | `Type`, `Encode`, and validating `Decode` over `String`; enables `std` and `alloc`.
//! `sqlx-postgres` | Enables `sqlx`, the PostgreSQL driver, and text-array support.
//! `sqlx-mysql` | Enables `sqlx` and the MySQL driver.
//! `sqlx-sqlite` | Enables `sqlx` and the SQLite driver.
//!
//! GraphQL scalar names match the Rust type, such as `XHandle` or
//! `LinkedinHandle`. Scalars accept only strings and implement `ScalarType`,
//! `InputType`, and `OutputType`; handles also work in `InputObject`,
//! `SimpleObject`, `Option<Handle>`, and `Vec<Handle>`.
//!
//! `connection::CursorType` encodes the stored string verbatim, without base64,
//! and validates on decode. Handles can therefore be used in
//! `Connection<Handle, Node>` and `Edge<Handle, Node>`. The application must supply
//! deterministic ordering and decide how renames affect pagination; a handle
//! cursor is suitable when the handle is the connection's unique ordering key.
//!
//! SQLx accepts handles by value or reference and supports `Option<Handle>` for
//! nullable columns. PostgreSQL additionally supports `Vec<Handle>` with
//! `sqlx-postgres`. If the application already enables its SQLx driver, the
//! handle crate's `sqlx` feature is sufficient for scalar values. Applications
//! select the SQLx runtime appropriate to their executor.
//!
//! See the [`known-types-x` crate documentation][recipes] for executable Serde,
//! GraphQL, cursor, and SQLx examples; the same patterns apply to every handle.
//!
//! [`XHandle`]: https://docs.rs/known-types-x/latest/known_types_x/struct.XHandle.html
//! [`LinkedinHandle`]: https://docs.rs/known-types-linkedin/latest/known_types_linkedin/struct.LinkedinHandle.html
//! [recipes]: https://docs.rs/known-types-x/latest/known_types_x/

use core::fmt;

/// An invalid social media handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseHandleError {
    /// The handle has fewer than the required number of characters.
    TooShort { min: usize },
    /// The handle exceeds the maximum number of characters.
    TooLong { max: usize },
    /// The handle contains a character that the upstream does not permit.
    InvalidCharacter(char),
    /// The characters are individually permitted, but their arrangement is not.
    InvalidFormat,
    /// A percent escape is incomplete or contains non-hexadecimal digits.
    InvalidPercentEncoding,
    /// The percent-decoded bytes are not UTF-8.
    InvalidUtf8,
}

impl fmt::Display for ParseHandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort { min } => write!(f, "handle must contain at least {min} characters"),
            Self::TooLong { max } => write!(f, "handle must contain at most {max} characters"),
            Self::InvalidCharacter(c) => write!(f, "invalid character in handle: {c:?}"),
            Self::InvalidFormat => f.write_str("invalid handle format"),
            Self::InvalidPercentEncoding => f.write_str("invalid percent escape in handle"),
            Self::InvalidUtf8 => f.write_str("percent-decoded handle is not UTF-8"),
        }
    }
}

impl core::error::Error for ParseHandleError {}

/// Validate length in Unicode scalar values, rather than UTF-8 bytes.
pub fn validate_length(input: &str, min: usize, max: usize) -> Result<(), ParseHandleError> {
    let length = input.chars().take(max.saturating_add(1)).count();
    if length < min {
        Err(ParseHandleError::TooShort { min })
    } else if length > max {
        Err(ParseHandleError::TooLong { max })
    } else {
        Ok(())
    }
}

/// Validate an ASCII alphanumeric handle with an upstream-specific set of punctuation.
pub fn validate_ascii(input: &str, punctuation: &str) -> Result<(), ParseHandleError> {
    match input
        .chars()
        .find(|c| !c.is_ascii_alphanumeric() && !punctuation.contains(*c))
    {
        Some(c) => Err(ParseHandleError::InvalidCharacter(c)),
        None => Ok(()),
    }
}

// Kept in one place so that a new integration cannot accidentally construct an
// unchecked handle. Feature gates are evaluated in the invoking handle crate.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_handle {
    ($handle:ident, $min:expr, $max:expr, $scalar_name:literal) => {
        impl $handle {
            /// Minimum length of the normalized handle, in Unicode scalar values.
            pub const MIN_LENGTH: usize = $min;
            /// Maximum representational length, in Unicode scalar values.
            ///
            /// This is the widest known current or historical upstream bound,
            /// not necessarily the current registration limit.
            pub const MAX_LENGTH: usize = $max;

            /// Borrow the validated, normalized handle.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume the handle and return its stored spelling.
            pub fn into_string(self) -> alloc::string::String {
                self.0
            }
        }

        impl AsRef<str> for $handle {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl TryFrom<&str> for $handle {
            type Error = $crate::handle::ParseHandleError;

            fn try_from(input: &str) -> Result<Self, Self::Error> {
                input.parse()
            }
        }

        impl TryFrom<alloc::string::String> for $handle {
            type Error = $crate::handle::ParseHandleError;

            fn try_from(input: alloc::string::String) -> Result<Self, Self::Error> {
                input.parse()
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $handle {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $handle {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let input = <alloc::string::String as serde::Deserialize>::deserialize(deserializer)?;
                input.parse().map_err(serde::de::Error::custom)
            }
        }

        #[cfg(feature = "async-graphql")]
        #[async_graphql::Scalar(name = $scalar_name)]
        impl async_graphql::ScalarType for $handle {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                match value {
                    async_graphql::Value::String(input) => input
                        .parse()
                        .map_err(async_graphql::InputValueError::custom),
                    value => Err(async_graphql::InputValueError::expected_type(value)),
                }
            }

            fn is_valid(value: &async_graphql::Value) -> bool {
                matches!(value, async_graphql::Value::String(input) if input.parse::<Self>().is_ok())
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(self.0.clone())
            }
        }

        #[cfg(feature = "async-graphql")]
        impl async_graphql::connection::CursorType for $handle {
            type Error = $crate::handle::ParseHandleError;

            fn decode_cursor(input: &str) -> Result<Self, Self::Error> {
                input.parse()
            }

            fn encode_cursor(&self) -> alloc::string::String {
                self.0.clone()
            }
        }

        #[cfg(feature = "sqlx")]
        impl<DB: sqlx::Database> sqlx::Type<DB> for $handle
        where
            alloc::string::String: sqlx::Type<DB>,
        {
            fn type_info() -> DB::TypeInfo {
                <alloc::string::String as sqlx::Type<DB>>::type_info()
            }

            fn compatible(ty: &DB::TypeInfo) -> bool {
                <alloc::string::String as sqlx::Type<DB>>::compatible(ty)
            }
        }

        #[cfg(feature = "sqlx-postgres")]
        impl sqlx::postgres::PgHasArrayType for $handle {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                <alloc::string::String as sqlx::postgres::PgHasArrayType>::array_type_info()
            }

            fn array_compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                <alloc::string::String as sqlx::postgres::PgHasArrayType>::array_compatible(ty)
            }
        }

        #[cfg(feature = "sqlx")]
        impl<'q, DB: sqlx::Database> sqlx::Encode<'q, DB> for $handle
        where
            alloc::string::String: sqlx::Encode<'q, DB>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut DB::ArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                <alloc::string::String as sqlx::Encode<DB>>::encode_by_ref(&self.0, buf)
            }

            fn produces(&self) -> Option<DB::TypeInfo> {
                <alloc::string::String as sqlx::Encode<DB>>::produces(&self.0)
            }

            fn size_hint(&self) -> usize {
                <alloc::string::String as sqlx::Encode<DB>>::size_hint(&self.0)
            }
        }

        #[cfg(feature = "sqlx")]
        impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for $handle
        where
            alloc::string::String: sqlx::Decode<'r, DB>,
        {
            fn decode(value: DB::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
                let input = <alloc::string::String as sqlx::Decode<DB>>::decode(value)?;
                input.parse().map_err(Into::into)
            }
        }

        #[cfg(test)]
        mod handle_tests {
            use super::$handle;
            use alloc::string::ToString;

            #[test]
            fn length_bounds_and_fallible_conversions() {
                for length in [$handle::MIN_LENGTH, $handle::MAX_LENGTH] {
                    let input = "a".repeat(length);
                    let handle: $handle = input.parse().expect("valid boundary length");
                    assert_eq!(handle.as_str(), input);
                    assert_eq!(handle.to_string(), input);
                    assert_eq!(handle.clone().into_string(), input);
                    assert_eq!($handle::try_from(input.as_str()), Ok(handle.clone()));
                    assert_eq!($handle::try_from(input), Ok(handle));
                }
                for length in [0, $handle::MIN_LENGTH - 1, $handle::MAX_LENGTH + 1] {
                    let input = "a".repeat(length);
                    assert!(input.parse::<$handle>().is_err());
                    assert!($handle::try_from(input.as_str()).is_err());
                    assert!($handle::try_from(input).is_err());
                }
            }

            #[cfg(feature = "serde")]
            #[test]
            fn serde_cannot_bypass_validation() {
                use serde::{Deserialize, de::value::{Error, StringDeserializer}};

                for input in [alloc::string::String::new(), "a".repeat($handle::MAX_LENGTH + 1)] {
                    assert!($handle::deserialize(StringDeserializer::<Error>::new(input)).is_err());
                }
                let decoded = $handle::deserialize(StringDeserializer::<Error>::new("Alice123".into()))
                    .expect("valid handle");
                assert_eq!(decoded.as_str(), "Alice123".parse::<$handle>().expect("valid handle").as_str());
            }

            #[cfg(feature = "async-graphql")]
            #[test]
            fn graphql_and_cursors_cannot_bypass_validation() {
                use async_graphql::{InputType, ScalarType, Value, connection::CursorType};

                assert_eq!(<$handle as InputType>::type_name(), stringify!($handle));
                for input in [alloc::string::String::new(), "a".repeat($handle::MAX_LENGTH + 1)] {
                    let value = Value::String(input.clone());
                    assert!(!<$handle as ScalarType>::is_valid(&value));
                    assert!(<$handle as ScalarType>::parse(value).is_err());
                    assert!($handle::decode_cursor(&input).is_err());
                }
                let handle: $handle = "Alice123".parse().expect("valid handle");
                let decoded = $handle::decode_cursor(&handle.encode_cursor()).expect("valid cursor");
                assert_eq!(decoded.as_str(), handle.as_str());
            }
        }
    };
}

// Case-preserving handles supply a comparison key; all identity traits must
// use that same key, including when the upstream ignores punctuation as well.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_handle_comparison {
    ($handle:ident) => {
        impl PartialEq for $handle {
            fn eq(&self, other: &Self) -> bool {
                self.comparison_key() == other.comparison_key()
            }
        }

        impl PartialOrd for $handle {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $handle {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.comparison_key().cmp(&other.comparison_key())
            }
        }

        impl core::hash::Hash for $handle {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                core::hash::Hash::hash(&self.comparison_key(), state);
            }
        }

        #[cfg(test)]
        mod comparison_tests {
            extern crate std;

            use super::$handle;
            use std::collections::{BTreeSet, HashSet};

            #[test]
            fn case_preserving_identity_in_collections() {
                let upper: $handle = "Alice123".parse().expect("valid handle");
                let lower: $handle = "alice123".parse().expect("valid handle");
                assert_eq!(upper.as_str(), "Alice123");
                assert_eq!(lower.as_str(), "alice123");
                assert_eq!(upper, lower);
                assert_eq!(upper.cmp(&lower), core::cmp::Ordering::Equal);
                assert_eq!(upper.partial_cmp(&lower), Some(core::cmp::Ordering::Equal));
                assert_eq!(HashSet::from([upper.clone(), lower.clone()]).len(), 1);
                assert_eq!(BTreeSet::from([upper, lower]).len(), 1);
            }
        }
    };
}

// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// An X handle (aka username).
///
/// Contains 1–20 ASCII letters, digits, or underscores. `MAX_LENGTH` is the
/// compatibility ceiling: current X usernames are limited to 15 characters,
/// but legacy accounts include handles longer than that, such as
/// `@richardrushfield` (16 characters). Parsing drops one optional `@`.
/// Spelling is preserved, while equality, ordering, and hashing ignore ASCII
/// case.
///
/// See <https://help.x.com/en/managing-your-account/x-username-rules>.
///
/// With the `async-graphql` feature, this is a string scalar named `XHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// All input, including cursors, is validated using `FromStr`.
///
/// With the `sqlx` feature, this implements SQLx's `Type`, `Encode`, and
/// `Decode` traits over `String`, validating on decode.
#[derive(AsRef, Clone, Debug, Display, Eq)]
pub struct XHandle(String);

known_types::impl_handle!(XHandle, 1, 20, "XHandle");
known_types::impl_handle_comparison!(XHandle);

impl XHandle {
    /// Current X username maximum for new registrations and edits.
    pub const CURRENT_MAX_LENGTH: usize = 15;
    /// Historical compatibility maximum used by legacy Twitter accounts.
    pub const HISTORICAL_MAX_LENGTH: usize = 20;

    fn comparison_key(&self) -> unicase::Ascii<&str> {
        unicase::Ascii::new(self.as_str())
    }
}

impl FromStr for XHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix('@').unwrap_or(input);
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        validate_ascii(input, "_")?;
        Ok(Self(input.into()))
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

#[test]
fn test_x_handle_syntax() {
    for (input, stored) in [
        ("@PlayItAgainSam", "PlayItAgainSam"),
        ("@a", "a"),
        ("_", "_"),
        ("123", "123"),
        ("@_Some_User_", "_Some_User_"),
        ("abcdefghijklmno", "abcdefghijklmno"),
        ("@richardrushfield", "richardrushfield"),
        ("abcdefghijklmnopqrst", "abcdefghijklmnopqrst"),
    ] {
        assert_eq!(
            input.parse::<XHandle>().expect("valid handle").as_str(),
            stored
        );
    }
    for input in [
        "@",
        "@@alice",
        "alice@",
        " alice",
        "alice ",
        "a-b",
        "a.b",
        "a/b",
        "a\0b",
        "álîce",
        "abcdefghijklmnopqrstu",
    ] {
        assert!(input.parse::<XHandle>().is_err(), "accepted {input:?}");
    }
}

// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// An Instagram handle (aka username).
///
/// Contains 1–30 ASCII letters, digits, periods, or underscores. No different
/// historical Instagram ceiling was found, so the current 30-character limit
/// is also the representational ceiling. Periods cannot be leading, trailing,
/// or consecutive. Parsing drops one optional `@` and normalizes to lowercase.
///
/// With the `async-graphql` feature, this is a string scalar named `InstagramHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// All input, including cursors, is validated using `FromStr`.
#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InstagramHandle(String);

known_types::impl_handle!(InstagramHandle, 1, 30, "InstagramHandle");

impl FromStr for InstagramHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix('@').unwrap_or(input);
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        validate_ascii(input, "._")?;
        if input.starts_with('.') || input.ends_with('.') || input.contains("..") {
            return Err(ParseHandleError::InvalidFormat);
        }
        Ok(Self(input.to_ascii_lowercase()))
    }
}

#[test]
fn test_instagram_handle_syntax_and_normalization() {
    for (input, stored) in [("@Alice.Smith_", "alice.smith_"), ("_A", "_a"), ("1", "1")] {
        assert_eq!(
            input
                .parse::<InstagramHandle>()
                .expect("valid handle")
                .as_str(),
            stored
        );
    }
    assert_eq!(
        "Alice".parse::<InstagramHandle>(),
        "ALICE".parse::<InstagramHandle>()
    );
    for input in [
        "@",
        "@@alice",
        ".alice",
        "alice.",
        "al..ice",
        "alice-smith",
        "alice smith",
        "álîce",
        "alice\n",
    ] {
        assert!(
            input.parse::<InstagramHandle>().is_err(),
            "accepted {input:?}"
        );
    }
}

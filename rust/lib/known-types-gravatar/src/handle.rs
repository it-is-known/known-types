// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// A Gravatar handle (aka username).
///
/// Gravatar shares WordPress.com usernames: 4–60 ASCII letters or digits,
/// including at least one letter. Parsing normalizes to lowercase.
///
/// See <https://support.gravatar.com/custom-domains/change-your-profile-url/>
/// and <https://developer.wordpress.org/reference/functions/wpmu_validate_user_signup/>.
///
/// With the `async-graphql` feature, this is a string scalar named `GravatarHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// All input, including cursors, is validated using `FromStr`.
#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GravatarHandle(String);

known_types::impl_handle!(GravatarHandle, 4, 60, "GravatarHandle");

impl FromStr for GravatarHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        validate_ascii(input, "")?;
        if !input.bytes().any(|c| c.is_ascii_alphabetic()) {
            return Err(ParseHandleError::InvalidFormat);
        }
        Ok(Self(input.to_ascii_lowercase()))
    }
}

#[test]
fn test_gravatar_handle_syntax_and_normalization() {
    let handle: GravatarHandle = "Alice123".parse().expect("valid handle");
    assert_eq!(handle.as_str(), "alice123");
    assert_eq!(handle, "ALICE123".parse().expect("valid handle"));
    for input in [
        "abc",
        "1234",
        "@alice",
        "alice_smith",
        "alice-smith",
        "alice.smith",
        "alice smith",
        "álîce",
        "alice\n",
    ] {
        assert!(
            input.parse::<GravatarHandle>().is_err(),
            "accepted {input:?}"
        );
    }
}

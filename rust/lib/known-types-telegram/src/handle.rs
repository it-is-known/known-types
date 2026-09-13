// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// A Telegram handle (aka username).
///
/// Contains 4–32 ASCII letters, digits, or underscores, starting with a letter
/// and ending with a letter or digit. The minimum includes collectible handles;
/// basic usernames require five characters. Parsing drops one optional `@`.
/// Spelling is preserved; equality, ordering, and hashing ignore ASCII case.
///
/// See <https://core.telegram.org/method/account.checkUsername>,
/// <https://core.telegram.org/api/fragment>, and
/// <https://telegram.org/faq#q-what-can-i-use-as-my-username>.
///
/// With the `async-graphql` feature, this is a string scalar named `TelegramHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// All input, including cursors, is validated using `FromStr`.
#[derive(AsRef, Clone, Debug, Display, Eq)]
pub struct TelegramHandle(String);

known_types::impl_handle!(TelegramHandle, 4, 32, "TelegramHandle");
known_types::impl_handle_comparison!(TelegramHandle);

impl TelegramHandle {
    fn comparison_key(&self) -> unicase::Ascii<&str> {
        unicase::Ascii::new(self.as_str())
    }
}

impl FromStr for TelegramHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix('@').unwrap_or(input);
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        validate_ascii(input, "_")?;
        if !input.as_bytes()[0].is_ascii_alphabetic() || input.ends_with('_') {
            return Err(ParseHandleError::InvalidFormat);
        }
        Ok(Self(input.into()))
    }
}

#[test]
fn test_telegram_handle_syntax() {
    for (input, stored) in [
        ("@TeleGram", "TeleGram"),
        ("News", "News"),
        ("user_123", "user_123"),
    ] {
        assert_eq!(
            input
                .parse::<TelegramHandle>()
                .expect("valid handle")
                .as_str(),
            stored
        );
    }
    for input in [
        "@",
        "@@Telegram",
        "abc",
        "_alice",
        "1alice",
        "alice_",
        "alice.smith",
        "alice-smith",
        "alice smith",
        "álîce",
        "alice\n",
    ] {
        assert!(
            input.parse::<TelegramHandle>().is_err(),
            "accepted {input:?}"
        );
    }
}

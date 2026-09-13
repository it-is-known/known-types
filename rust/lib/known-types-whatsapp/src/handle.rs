// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// A WhatsApp handle (aka username).
///
/// Contains 3–35 ASCII letters, digits, periods, or underscores, starting with
/// a letter. This is the current username feature's range; WhatsApp had no
/// older username format to preserve. Periods cannot be leading, trailing, or
/// consecutive.
/// Web-address prefixes (`www.`) and suffixes (`.com`, `.net`) are forbidden.
/// Parsing drops one optional `@` and normalizes to lowercase.
///
/// See <https://www.whatsapp.com/usernames-faq/> and the
/// [format rules summarized by PickMyHandle](https://pickmyhandle.com/blog/whatsapp-username-rules)
/// (a third-party reference).
///
/// ```
/// use known_types_whatsapp::WhatsappHandle;
///
/// let handle: WhatsappHandle = "@Dev__Arjun".parse()?;
/// assert_eq!(handle.as_str(), "dev__arjun");
/// assert!("www.alice".parse::<WhatsappHandle>().is_err());
/// # Ok::<(), known_types_whatsapp::ParseHandleError>(())
/// ```
///
/// See the [shared handle contract](known_types::handle) for conversion and
/// integration behavior. With `async-graphql`, the scalar is named `WhatsappHandle`.
#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WhatsappHandle(String);

known_types::impl_handle!(WhatsappHandle, 3, 35, "WhatsappHandle");

impl FromStr for WhatsappHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix('@').unwrap_or(input);
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        validate_ascii(input, "._")?;
        let input = input.to_ascii_lowercase();
        if !input.as_bytes()[0].is_ascii_alphabetic()
            || input.starts_with('.')
            || input.ends_with('.')
            || input.contains("..")
            || input.starts_with("www.")
            || input.ends_with(".com")
            || input.ends_with(".net")
        {
            return Err(ParseHandleError::InvalidFormat);
        }
        Ok(Self(input))
    }
}

#[test]
fn test_whatsapp_handle_syntax_and_normalization() {
    for (input, stored) in [
        ("@Alice.Smith_", "alice.smith_"),
        ("Abc", "abc"),
        ("Dev__Arjun", "dev__arjun"),
    ] {
        assert_eq!(
            input
                .parse::<WhatsappHandle>()
                .expect("valid handle")
                .as_str(),
            stored
        );
    }
    assert_eq!(
        "Alice".parse::<WhatsappHandle>(),
        "ALICE".parse::<WhatsappHandle>()
    );
    for input in [
        "@",
        "@@alice",
        "ab",
        "123",
        "1alice",
        "_alice",
        ".alice",
        "alice.",
        "al..ice",
        "alice-smith",
        "alice smith",
        "álîce",
        "www.alice",
        "WWW.ALICE",
        "alice.COM",
        "alice.net",
        "alice\n",
    ] {
        assert!(
            input.parse::<WhatsappHandle>().is_err(),
            "accepted {input:?}"
        );
    }
}

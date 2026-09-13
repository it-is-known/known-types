// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// A Facebook handle (aka username).
///
/// Contains 5–50 ASCII letters, digits, or periods, with at least five
/// alphanumeric characters. The same 5–50 range is used for current and
/// legacy Facebook usernames; no wider legacy range was found. Parsing drops
/// one optional `@` and preserves spelling. Equality, ordering, and hashing
/// ignore case and periods.
///
/// See <https://www.facebook.com/help/105399436216001>.
///
/// ```
/// use known_types_facebook::FacebookHandle;
///
/// let dotted: FacebookHandle = "@Alice.Smith".parse()?;
/// assert_eq!(dotted.as_str(), "Alice.Smith");
/// assert_eq!(dotted, "alicesmith".parse::<FacebookHandle>()?);
/// # Ok::<(), known_types_facebook::ParseHandleError>(())
/// ```
///
/// See the [shared handle contract](known_types::handle) for conversion and
/// integration behavior. With `async-graphql`, the scalar is named `FacebookHandle`.
#[derive(AsRef, Clone, Debug, Display, Eq)]
pub struct FacebookHandle(String);

known_types::impl_handle!(FacebookHandle, 5, 50, "FacebookHandle");
known_types::impl_handle_comparison!(FacebookHandle);

impl FacebookHandle {
    fn comparison_key(&self) -> unicase::Ascii<String> {
        unicase::Ascii::new(self.0.replace('.', ""))
    }
}

impl FromStr for FacebookHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix('@').unwrap_or(input);
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        validate_ascii(input, ".")?;
        if input.bytes().filter(u8::is_ascii_alphanumeric).count() < Self::MIN_LENGTH {
            return Err(ParseHandleError::TooShort {
                min: Self::MIN_LENGTH,
            });
        }
        Ok(Self(input.into()))
    }
}

#[test]
fn test_facebook_handle_syntax_and_identity() {
    extern crate std;
    use std::collections::{BTreeSet, HashSet};

    let dotted: FacebookHandle = "@Alice.Smith".parse().expect("valid handle");
    let plain: FacebookHandle = "alicesmith".parse().expect("valid handle");
    assert_eq!(dotted.as_str(), "Alice.Smith");
    assert_eq!(dotted, plain);
    assert_eq!(HashSet::from([dotted.clone(), plain.clone()]).len(), 1);
    assert_eq!(BTreeSet::from([dotted, plain]).len(), 1);
    for input in [
        "@",
        "@@Alice",
        "abcd",
        "a.b.c.d",
        ".....",
        "Alice_Smith",
        "Alice-Smith",
        "Alice Smith",
        "Álice",
        "alice/",
        "alice\n",
    ] {
        assert!(
            input.parse::<FacebookHandle>().is_err(),
            "accepted {input:?}"
        );
    }
}

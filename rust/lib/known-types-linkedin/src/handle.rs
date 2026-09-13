// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::validate_length;
use percent_encoding::percent_decode_str;

/// A LinkedIn handle (aka username).
///
/// Contains 3–100 Unicode letters or numbers, or hyphens. The current 3–100
/// rule is also the widest legacy-compatible range found. Parsing trims outer
/// whitespace and percent-decodes UTF-8 exactly once before validating length
/// and characters. Malformed escapes and encoded whitespace are rejected.
/// Spelling is preserved; equality, ordering, and hashing use Unicode case folding.
///
/// See <https://www.linkedin.com/help/linkedin/answer/a542685/manage-your-public-profile-url>.
///
/// Trimming precedes decoding: `%20alice%20` is rejected, not trimmed a second
/// time. Invalid UTF-8, decoded spaces, slashes, and literal percent signs are
/// forbidden. `literal%2520handle` is rejected rather than decoded repeatedly.
/// Valid stored handles round-trip through parsing and every enabled integration
/// without changing their spelling.
///
/// ```
/// use known_types_linkedin::LinkedinHandle;
///
/// let handle: LinkedinHandle = " Bj%C3%96rn ".parse()?;
/// assert_eq!(handle.as_str(), "BjÖrn");
/// assert_eq!(handle, "björn".parse::<LinkedinHandle>()?);
/// let reparsed: LinkedinHandle = handle.as_str().parse()?;
/// assert_eq!(reparsed.as_str(), handle.as_str());
/// assert!("%20alice%20".parse::<LinkedinHandle>().is_err());
/// assert!("literal%2520handle".parse::<LinkedinHandle>().is_err());
/// # Ok::<(), known_types_linkedin::ParseHandleError>(())
/// ```
///
/// See the [shared handle contract](known_types::handle) for conversion and
/// integration behavior. With `async-graphql`, the scalar is named `LinkedinHandle`.
#[derive(AsRef, Clone, Debug, Display, Eq)]
pub struct LinkedinHandle(String);

known_types::impl_handle!(LinkedinHandle, 3, 100, "LinkedinHandle");
known_types::impl_handle_comparison!(LinkedinHandle);

impl LinkedinHandle {
    fn comparison_key(&self) -> unicase::UniCase<&str> {
        unicase::UniCase::new(self.as_str())
    }
}

impl FromStr for LinkedinHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        // percent-encoding deliberately leaves malformed escapes untouched;
        // handles reject them instead of treating them as literal percent signs.
        for escape in input.split('%').skip(1) {
            if escape.len() < 2 || !escape.as_bytes()[..2].iter().all(u8::is_ascii_hexdigit) {
                return Err(ParseHandleError::InvalidPercentEncoding);
            }
        }
        let decoded = percent_decode_str(input)
            .decode_utf8()
            .map_err(|_| ParseHandleError::InvalidUtf8)?;
        validate_length(&decoded, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        if let Some(c) = decoded.chars().find(|c| !c.is_alphanumeric() && *c != '-') {
            return Err(ParseHandleError::InvalidCharacter(c));
        }
        // Stored handles contain neither '%' nor whitespace, so parsing a
        // serialized handle is idempotent (including cursors and database text).
        Ok(Self(decoded.into_owned()))
    }
}

#[test]
fn test_linkedin_percent_decoding_and_length() {
    for (input, stored) in [
        ("foobar", "foobar"),
        (" bj%C3%B6rn ", "björn"),
        ("Bj%C3%96rn", "BjÖrn"),
        ("%62j%c3%b6rn", "björn"),
        ("John%2DSmith", "John-Smith"),
        ("%E6%9D%8E%E5%B0%8F%E9%BE%8D", "李小龍"),
        ("%61%62%63", "abc"),
    ] {
        let handle: LinkedinHandle = input.parse().expect("valid handle");
        assert_eq!(handle.as_str(), stored);
        assert_eq!(
            LinkedinHandle::try_from(String::from(input)),
            Ok(handle.clone())
        );
        assert_eq!(
            handle
                .as_str()
                .parse::<LinkedinHandle>()
                .expect("stored handle")
                .as_str(),
            stored
        );
    }
    for length in [3, 100] {
        // Measure decoded Unicode characters, not bytes or percent escapes.
        let stored = "ö".repeat(length);
        let handle: LinkedinHandle = "%C3%B6".repeat(length).parse().expect("valid boundary");
        assert_eq!(handle.as_str(), stored);
    }
    assert!("%C3%B6".repeat(101).parse::<LinkedinHandle>().is_err());
    assert!("%61%62".parse::<LinkedinHandle>().is_err());
}

#[test]
fn test_linkedin_invalid_encoded_characters() {
    for input in [
        " ",
        "ab",
        "@alice",
        "abc%",
        "abc%2",
        "abc%GG",
        "abc%%20",
        "abc%é",
        "abc%FF",
        "abc%C3%28",
        "abc%C0%AF",
        "abc%ED%A0%80",
        "abc%00",
        "abc%0A",
        "abc%20",
        "%20alice%20",
        "alice%2Fsmith",
        "alice%5Csmith",
        "alice%3Fquery",
        "alice%23fragment",
        "alice+smith",
        "alice.smith",
        "alice_smith",
        "alice smith",
        "abc%252D",
        "abc%25FF",
        "abc%F0%9F%98%80",
        "abc\0",
        "https://linkedin.com/in/alice",
    ] {
        assert!(
            input.parse::<LinkedinHandle>().is_err(),
            "accepted {input:?}"
        );
    }
    assert_eq!(
        "abc%GG".parse::<LinkedinHandle>(),
        Err(ParseHandleError::InvalidPercentEncoding)
    );
    assert_eq!(
        "abc%FF".parse::<LinkedinHandle>(),
        Err(ParseHandleError::InvalidUtf8)
    );
}

#[test]
fn test_linkedin_unicode_case_identity() {
    extern crate std;
    use std::collections::{BTreeSet, HashSet};

    for (left, right) in [("BjÖrn", "björn"), ("Straße", "STRASSE"), ("ΟΣΣ", "οσς")] {
        let left: LinkedinHandle = left.parse().expect("valid handle");
        let right: LinkedinHandle = right.parse().expect("valid handle");
        assert_eq!(left, right);
        assert_eq!(HashSet::from([left.clone(), right.clone()]).len(), 1);
        assert_eq!(BTreeSet::from([left, right]).len(), 1);
    }
}

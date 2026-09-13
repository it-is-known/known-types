// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::validate_length;

/// A local.ai handle (aka username).
///
/// Uses the fallback length range of 1–100 Unicode scalar values. local.ai does
/// not publish a handle-length history in the material available to this
/// crate, so these are conservative representational bounds rather than a
/// claimed upstream registration limit. Spelling is preserved and comparisons
/// are case-sensitive.
///
/// With the `async-graphql` feature, this is a string scalar named `LocalaiHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// All input, including cursors, is validated using `FromStr`.
#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalaiHandle(String);

known_types::impl_handle!(LocalaiHandle, 1, 100, "LocalaiHandle");

impl FromStr for LocalaiHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        Ok(Self(input.into()))
    }
}

#[test]
fn test_localai_fallback_bounds_and_case() {
    let input = "É".repeat(100);
    assert_eq!(
        input
            .parse::<LocalaiHandle>()
            .expect("valid handle")
            .as_str(),
        input
    );
    assert!("É".repeat(101).parse::<LocalaiHandle>().is_err());
    assert_ne!(
        "Alice".parse::<LocalaiHandle>(),
        "alice".parse::<LocalaiHandle>()
    );
}

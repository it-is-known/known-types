// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::validate_length;

/// A Luma handle (aka username).
///
/// Uses the fallback length range of 1–100 Unicode scalar values. Luma does not
/// publish a handle-length history in the material available to this crate,
/// so these are conservative representational bounds rather than a claimed
/// upstream registration limit. Spelling is preserved and comparisons are
/// case-sensitive. Only length is validated.
///
/// ```
/// use known_types_luma::LumaHandle;
///
/// let handle: LumaHandle = "Élodie".parse()?;
/// assert_eq!(handle.as_str(), "Élodie");
/// assert_ne!(handle, "élodie".parse::<LumaHandle>()?);
/// # Ok::<(), known_types_luma::ParseHandleError>(())
/// ```
///
/// See the [shared handle contract](known_types::handle) for conversion and
/// integration behavior. With `async-graphql`, the scalar is named `LumaHandle`.
#[derive(AsRef, Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LumaHandle(String);

known_types::impl_handle!(LumaHandle, 1, 100, "LumaHandle");

impl FromStr for LumaHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        Ok(Self(input.into()))
    }
}

#[test]
fn test_luma_fallback_bounds_and_case() {
    let input = "É".repeat(100);
    assert_eq!(
        input.parse::<LumaHandle>().expect("valid handle").as_str(),
        input
    );
    assert!("É".repeat(101).parse::<LumaHandle>().is_err());
    assert_ne!("Alice".parse::<LumaHandle>(), "alice".parse::<LumaHandle>());
}

// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "alloc"))]
compile_error!("this module requires the 'alloc' feature");

use alloc::string::String;
use core::str::FromStr;
use derive_more::{AsRef, Display};
pub use known_types::handle::ParseHandleError;
use known_types::handle::{validate_ascii, validate_length};

/// A GitHub handle (aka username).
///
/// Contains 1–39 ASCII letters, digits, or single interior hyphens. Enterprise
/// Managed Users may additionally have an `_` followed by a 3–8 character
/// alphanumeric enterprise shortcode. GitHub's current and legacy-compatible
/// ceiling is 39 characters (30 for some data-residency managed users).
/// Parsing drops one optional `@` and preserves spelling; equality, ordering,
/// and hashing ignore ASCII case.
///
/// See <https://docs.github.com/en/enterprise-cloud@latest/admin/managing-iam/iam-configuration-reference/username-considerations-for-external-authentication>.
///
/// With the `async-graphql` feature, this is a string scalar named `GithubHandle`
/// implementing `ScalarType`, `InputType`, `OutputType`, and `CursorType`.
/// All input, including cursors, is validated using `FromStr`.
#[derive(AsRef, Clone, Debug, Display, Eq)]
pub struct GithubHandle(String);

known_types::impl_handle!(GithubHandle, 1, 39, "GithubHandle");
known_types::impl_handle_comparison!(GithubHandle);

impl GithubHandle {
    fn comparison_key(&self) -> unicase::Ascii<&str> {
        unicase::Ascii::new(self.as_str())
    }
}

impl FromStr for GithubHandle {
    type Err = ParseHandleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix('@').unwrap_or(input);
        validate_length(input, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        let name = match input.split_once('_') {
            Some((name, shortcode)) => {
                validate_length(shortcode, 3, 8)?;
                validate_ascii(shortcode, "")?;
                name
            }
            None => input,
        };
        validate_ascii(name, "-")?;
        if name.is_empty() || name.starts_with('-') || name.ends_with('-') || name.contains("--") {
            return Err(ParseHandleError::InvalidFormat);
        }
        Ok(Self(input.into()))
    }
}

#[test]
fn test_github_handle_syntax() {
    for (input, stored) in [
        ("@Octo-Cat", "Octo-Cat"),
        ("a", "a"),
        ("123", "123"),
        ("@The-Octocat_octo", "The-Octocat_octo"),
        ("octo_admin", "octo_admin"),
    ] {
        assert_eq!(
            input
                .parse::<GithubHandle>()
                .expect("valid handle")
                .as_str(),
            stored
        );
    }
    for input in [
        "@",
        "@@Octocat",
        "-octocat",
        "octocat-",
        "octo--cat",
        "octo.cat",
        "octo cat",
        "octöcat",
        "_octo",
        "octo_",
        "octo_ab",
        "octo_abcdefghi",
        "octo_abc_def",
        "octo_ab-c",
        "octocat\n",
    ] {
        assert!(input.parse::<GithubHandle>().is_err(), "accepted {input:?}");
    }
}

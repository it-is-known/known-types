# Known Types

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types)
[![Documentation](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types)

**Well-known types for Rust.**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

## ✨ Features

- Zero default dependencies, only optional [integrations](#integrations).
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add known-types
```

<details>
<summary>Instructions for each crate</summary>

### Installation via Cargo (all crates)

```bash
cargo add known-types
cargo add known-types-anthropic
cargo add known-types-github
cargo add known-types-google
cargo add known-types-graphql
cargo add known-types-gravatar
cargo add known-types-ietf
cargo add known-types-instagram
cargo add known-types-linkedin
cargo add known-types-luma
cargo add known-types-nostr
cargo add known-types-openai
cargo add known-types-pypi
cargo add known-types-rubygems
cargo add known-types-w3c
cargo add known-types-x
```

</details>

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
known-types = "0"
```

<details>
<summary>Instructions for each crate</summary>

### Installation in `Cargo.toml` (with all features enabled, in all crates)

```toml
[dependencies]
known-types = "0"
known-types-anthropic = "0"
known-types-github = "0"
known-types-google = "0"
known-types-graphql = "0"
known-types-gravatar = "0"
known-types-ietf = "0"
known-types-instagram = "0"
known-types-linkedin = "0"
known-types-luma = "0"
known-types-nostr = "0"
known-types-openai = "0"
known-types-pypi = "0"
known-types-rubygems = "0"
known-types-w3c = "0"
known-types-x = "0"
```

</details>

Enable only specific features:

```toml
[dependencies]
known-types = { version = "0", default-features = false, features = ["serde"] }
```

<details>
<summary>Instructions for each crate</summary>

### Installation in `Cargo.toml` (with only specific features enabled, in all crates)

```toml
[dependencies]
known-types = { version = "0", default-features = false, features = ["serde"] }
known-types-anthropic = { version = "0", default-features = false, features = ["serde"] }
known-types-github = { version = "0", default-features = false, features = ["serde"] }
known-types-google = { version = "0", default-features = false, features = ["serde"] }
known-types-graphql = { version = "0", default-features = false, features = ["serde"] }
known-types-gravatar = { version = "0", default-features = false, features = ["serde"] }
known-types-ietf = { version = "0", default-features = false, features = ["serde"] }
known-types-instagram = { version = "0", default-features = false, features = ["serde"] }
known-types-linkedin = { version = "0", default-features = false, features = ["serde"] }
known-types-luma = { version = "0", default-features = false, features = ["serde"] }
known-types-nostr = { version = "0", default-features = false, features = ["serde"] }
known-types-openai = { version = "0", default-features = false, features = ["serde"] }
known-types-pypi = { version = "0", default-features = false, features = ["serde"] }
known-types-rubygems = { version = "0", default-features = false, features = ["serde"] }
known-types-w3c = { version = "0", default-features = false, features = ["serde"] }
known-types-x = { version = "0", default-features = false, features = ["serde"] }
```

</details>

## 👉 Examples

### Importing the Library

```rust
use known_types;
```

<details>
<summary>Instructions for each crate</summary>

### Importing the library (all crates)

```rust
use known_types;
use known_types_anthropic;
use known_types_github;
use known_types_google;
use known_types_graphql;
use known_types_gravatar;
use known_types_ietf;
use known_types_instagram;
use known_types_linkedin;
use known_types_luma;
use known_types_nostr;
use known_types_openai;
use known_types_pypi;
use known_types_rubygems;
use known_types_w3c;
use known_types_x;
```

</details>

### Validated social media handles

Construct handles with `FromStr` or `TryFrom<&str>` / `TryFrom<String>`. All
construction and decoding paths validate the same constraints and return
`ParseHandleError` on invalid input. The inner string is private; `as_str()`
borrows it and `into_string()` consumes the handle to recover its stored spelling.
`MIN_LENGTH` and `MAX_LENGTH` expose each type's inclusive representational
bounds. These are the widest known current or historical bounds needed to
round-trip handles that exist in the wild; they are not necessarily current
registration limits.

```rust
use known_types_x::{ParseHandleError, XHandle};

fn main() -> Result<(), ParseHandleError> {
    let handle: XHandle = "@PlayItAgainSam".parse()?;
    assert_eq!(handle.as_str(), "PlayItAgainSam");
    assert_eq!(handle, "playitagainsam".parse::<XHandle>()?);
    assert!("not a handle".parse::<XHandle>().is_err());
    Ok(())
}
```

Platform | Representable | Current upstream limit | Historical / legacy limit | Syntax and parsing | Comparison
:--- | :--- | :--- | :--- | :--- | :---
Facebook | 5–50 | 5–50 | No wider legacy limit found | ASCII letters/digits and periods; at least 5 alphanumeric characters; optional `@` | Ignores case and periods; preserves spelling
GitHub | 1–39 | 1–39 (30 for some data-residency managed users) | No wider legacy limit found | ASCII letters/digits and single interior hyphens; managed users may have an `_shortcode` suffix (3–8 alphanumeric characters); optional `@` | Ignores case; preserves spelling
Gravatar | 4–60 | WordPress.com username: 4–60 | WordPress multisite validation also documents 4–60 | ASCII letters/digits, including a letter; lowercased | Ignores case
Instagram | 1–30 | 1–30 | No different legacy limit found | ASCII letters/digits, `_`, `.`; no leading/trailing/consecutive periods; optional `@`; lowercased | Ignores case
Intro.co | 1–100 | Upstream limit not documented; fallback 1–100 | Upstream history not documented; fallback 1–100 | Fallback length bounds | Case-sensitive; preserves spelling
LinkedIn | 3–100 | 3–100 | 3–100 is also documented in available legacy help captures; no wider limit found | Unicode letters/numbers and `-`; outer whitespace trimmed; percent-decoded before validation | Unicode case-insensitive; preserves spelling
local.ai | 1–100 | Upstream limit not documented; fallback 1–100 | Upstream history not documented; fallback 1–100 | Fallback length bounds | Case-sensitive; preserves spelling
Luma | 1–100 | Upstream limit not documented; fallback 1–100 | Upstream history not documented; fallback 1–100 | Fallback length bounds | Case-sensitive; preserves spelling
Telegram | 4–32 | Basic: 5–32; collectible: 4–32 | Basic: 5–32; collectibles were introduced later | ASCII letters/digits and `_`; initial letter, final letter/digit; optional `@` | Ignores case; preserves spelling
WhatsApp | 3–35 | 3–35 (new username feature) | No older username format to preserve | ASCII letters/digits, `_`, `.`; initial letter; no leading/trailing/consecutive periods, `www.` prefix, or `.com`/`.net` suffix; optional `@`; lowercased | Ignores case
X | 1–20 | Maximum 15 for new registrations and edits | Legacy Twitter accounts can exceed 15; the compatibility ceiling is 20 | ASCII letters/digits and `_`; optional `@` | Ignores case; preserves spelling

The representational bounds deliberately include legacy values: X accepts up to
20 so existing handles such as `@richardrushfield` remain representable, and
Telegram accepts four-character collectible usernames even though basic
usernames require five. Account availability, ownership, and registration-only
reserved names are determined by the upstream. The fallback types apply the
requested 1–100 bounds where a more specific upstream contract is not known.
Length is measured in Unicode scalar values after normalization; an optional
`@` is removed exactly once on platforms that display it.

Case-insensitive handles use consistent `Eq`, `Ord`, and `Hash` implementations,
so hash maps and ordered collections agree about identity. Case-preserving
parsers keep the supplied spelling; they do not look up the account's canonical
capitalization.

LinkedIn accepts both `björn` and `bj%C3%B6rn`, storing `björn`. It rejects
malformed percent escapes, invalid UTF-8, and decoded forbidden characters such
as spaces, slashes, and literal percent signs. Escapes are decoded once:
`literal%2520handle` is rejected rather than decoded repeatedly. Stored handles
round-trip through parsing and every enabled integration.

This replaces the previous infallible (or panicking, for LinkedIn) `From`
conversions. Change `Handle::from(text)` / `text.into()` to `text.parse()?` or
`Handle::try_from(text)?`. Serde, GraphQL scalar/cursor input, and SQLx decoding
now report invalid handles as errors.

Upstream references: [Facebook](https://www.facebook.com/help/105399436216001),
[GitHub](https://docs.github.com/en/enterprise-cloud@latest/admin/managing-iam/iam-configuration-reference/username-considerations-for-external-authentication),
[Gravatar](https://support.gravatar.com/custom-domains/change-your-profile-url/),
[WordPress username validation](https://developer.wordpress.org/reference/functions/wpmu_validate_user_signup/),
[Instagram rules](https://www.handlegrab.com/blog/instagram-username-rules),
[LinkedIn](https://www.linkedin.com/help/linkedin/answer/a542685/manage-your-public-profile-url),
[Telegram](https://core.telegram.org/method/account.checkUsername),
[Telegram collectibles](https://core.telegram.org/api/fragment),
[WhatsApp](https://www.whatsapp.com/usernames-faq/),
[WhatsApp format rules](https://pickmyhandle.com/blog/whatsapp-username-rules),
and [X](https://help.x.com/en/managing-your-account/x-username-rules).

The X limits are corroborated by X's current help page (15 characters) and
[Twitter's archived 2010 help page](https://web.archive.org/web/20100718125730/http://support.twitter.com/entries/14609-how-to-change-your-username)
and [2016 help page](https://web.archive.org/web/20161203051256/https://support.twitter.com/articles/14609):
both describe 15-character usernames, and the 2010 page explicitly preserves
longer "early bird" usernames. The archived pages do not publish a hard upper
bound for early-bird exceptions; this library uses the historically reported
20-character compatibility ceiling and tests the known 16-character
[`richardrushfield`](https://x.com/richardrushfield) account. A handle longer
than 20 should be treated as an upstream exception rather than silently
truncated.

### Using handles with async-graphql

All handle crates support [async-graphql] 7.2 through the optional `async-graphql`
feature. It enables `std` (and `alloc`) and implements `ScalarType`, `InputType`,
`OutputType`, and `connection::CursorType`. It also works with
`default-features = false` and requires no `serde` feature on the handle crate.

For example, to use `XHandle` as a query argument and return value:

```toml
[dependencies]
known-types-x = { version = "0.1", default-features = false, features = ["async-graphql"] }
async-graphql = { version = "7.2", default-features = false }
```

```rust
use async_graphql::Object;
use known_types_x::XHandle;

struct Query;

#[Object]
impl Query {
    async fn handle(&self, input: XHandle) -> XHandle {
        input
    }
}
```

The resulting field is `handle(input: XHandle!): XHandle!`. Queries can pass
string literals or variables declared as `XHandle`, for example:

```graphql
query($handle: XHandle!) {
  handle(input: $handle)
}
```

With variables `{"handle": "Some_User"}`, this returns
`{"data": {"handle": "Some_User"}}`. Handles also work in `InputObject` and
`SimpleObject` fields, `Option<Handle>`, and `Vec<Handle>`.

Each handle has its own scalar name matching its Rust type: `XHandle`,
`FacebookHandle`, `GithubHandle`, `GravatarHandle`, `InstagramHandle`,
`IntrocoHandle`, `LinkedinHandle`, `LocalaiHandle`, `LumaHandle`, `TelegramHandle`,
and `WhatsappHandle`. Scalars accept only GraphQL strings and serialize to
strings. All handle inputs use `FromStr` validation and normalization as described
above; invalid input returns a GraphQL input error.

#### Handles as connection cursors

All handles implement `connection::CursorType` under the same feature, so they
can be used directly in `Connection<Handle, Node>` and `Edge<Handle, Node>`.
They encode the stored string verbatim and validate it with `FromStr` when
decoding, returning `ParseHandleError` for invalid cursors. Valid stored handles
round-trip without changing their spelling, including decoded LinkedIn handles.

Handle cursors are appropriate when the handle is the unique ordering key for
the connection. The application supplies deterministic pagination ordering and
decides how handle renames affect that ordering; `CursorType` supplies the
reversible encoding.

### Using handles with SQLx

All handle crates support [SQLx] 0.9 through optional features:

Feature | Effect
:--- | :---
`sqlx` | Implements `sqlx::{Type, Encode, Decode}` over `String`, validating on decode; enables `std`.
`sqlx-postgres` | Enables `sqlx` and the PostgreSQL driver, including PostgreSQL array support.
`sqlx-mysql` | Enables `sqlx` and the MySQL driver.
`sqlx-sqlite` | Enables `sqlx` and the SQLite driver.

For example, to use `XHandle` with PostgreSQL:

```toml
[dependencies]
known-types-x = { version = "0.1", features = ["sqlx-postgres"] }
sqlx = { version = "0.9", default-features = false, features = ["postgres", "runtime-tokio"] }
```

```rust
use known_types_x::XHandle;

async fn round_trip(pool: &sqlx::PgPool, handle: &XHandle) -> Result<XHandle, sqlx::Error> {
    sqlx::query_scalar("SELECT $1::text")
        .bind(handle)
        .fetch_one(pool)
        .await
}
```

Handles can be bound by value or reference, decoded from string columns, and
wrapped in `Option` for nullable columns. PostgreSQL also supports `Vec<Handle>`
for text arrays with `sqlx-postgres`. Decoding validates and normalizes database
text using `FromStr`; invalid text returns a SQLx column decoding error.

The same features are available for `FacebookHandle`, `GithubHandle`,
`GravatarHandle`, `InstagramHandle`, `IntrocoHandle`, `LinkedinHandle`,
`LocalaiHandle`, `LumaHandle`, `TelegramHandle`, and `WhatsappHandle` in their
respective crates. If the application already enables its database driver on
`sqlx`, enabling just `sqlx` on a handle crate is sufficient. SQLx support is
disabled by default and also works with `default-features = false`.

When using SQLx's compile-time query macros, use an explicit column type override
such as `SELECT handle AS "handle: XHandle"` with `query!`, or
`SELECT handle AS "handle: _"` with `query_as!` and a struct field of type `XHandle`.

## 📚 Reference

[docs.rs/known-types](https://docs.rs/known-types)

### Crates

Crate | Version | Docs | Summary
:--- | :--- | :--- | :---
[known-types] | [![known-types](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types) | [![known-types](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types/) | Well-known types.
[known-types-anthropic] | [![known-types-anthropic](https://img.shields.io/crates/v/known-types-anthropic)](https://crates.io/crates/known-types-anthropic) | [![known-types-anthropic](https://docs.rs/known-types-anthropic/badge.svg)](https://docs.rs/known-types-anthropic/) | Well-known types for Anthropic APIs.
[known-types-github] | [![known-types-github](https://img.shields.io/crates/v/known-types-github)](https://crates.io/crates/known-types-github) | [![known-types-github](https://docs.rs/known-types-github/badge.svg)](https://docs.rs/known-types-github/) | Well-known types for GitHub APIs.
[known-types-google] | [![known-types-google](https://img.shields.io/crates/v/known-types-google)](https://crates.io/crates/known-types-google) | [![known-types-google](https://docs.rs/known-types-google/badge.svg)](https://docs.rs/known-types-google/) | Well-known types for Google APIs.
[known-types-graphql] | [![known-types-graphql](https://img.shields.io/crates/v/known-types-graphql)](https://crates.io/crates/known-types-graphql) | [![known-types-graphql](https://docs.rs/known-types-graphql/badge.svg)](https://docs.rs/known-types-graphql/) | Well-known types for GraphQL specifications.
[known-types-gravatar] | [![known-types-gravatar](https://img.shields.io/crates/v/known-types-gravatar)](https://crates.io/crates/known-types-gravatar) | [![known-types-gravatar](https://docs.rs/known-types-gravatar/badge.svg)](https://docs.rs/known-types-gravatar/) | Well-known types for Gravatar APIs.
[known-types-ietf] | [![known-types-ietf](https://img.shields.io/crates/v/known-types-ietf)](https://crates.io/crates/known-types-ietf) | [![known-types-ietf](https://docs.rs/known-types-ietf/badge.svg)](https://docs.rs/known-types-ietf/) | Well-known types for IETF specifications.
[known-types-instagram] | [![known-types-instagram](https://img.shields.io/crates/v/known-types-instagram)](https://crates.io/crates/known-types-instagram) | [![known-types-instagram](https://docs.rs/known-types-instagram/badge.svg)](https://docs.rs/known-types-instagram/) | Well-known types for Instagram APIs.
[known-types-linkedin] | [![known-types-linkedin](https://img.shields.io/crates/v/known-types-linkedin)](https://crates.io/crates/known-types-linkedin) | [![known-types-linkedin](https://docs.rs/known-types-linkedin/badge.svg)](https://docs.rs/known-types-linkedin/) | Well-known types for LinkedIn APIs.
[known-types-luma] | [![known-types-luma](https://img.shields.io/crates/v/known-types-luma)](https://crates.io/crates/known-types-luma) | [![known-types-luma](https://docs.rs/known-types-luma/badge.svg)](https://docs.rs/known-types-luma/) | Well-known types for Luma APIs.
[known-types-nostr] | [![known-types-nostr](https://img.shields.io/crates/v/known-types-nostr)](https://crates.io/crates/known-types-nostr) | [![known-types-nostr](https://docs.rs/known-types-nostr/badge.svg)](https://docs.rs/known-types-nostr/) | Well-known types for the Nostr protocol.
[known-types-openai] | [![known-types-openai](https://img.shields.io/crates/v/known-types-openai)](https://crates.io/crates/known-types-openai) | [![known-types-openai](https://docs.rs/known-types-openai/badge.svg)](https://docs.rs/known-types-openai/) | Well-known types for OpenAI APIs.
[known-types-pypi] | [![known-types-pypi](https://img.shields.io/crates/v/known-types-pypi)](https://crates.io/crates/known-types-pypi) | [![known-types-pypi](https://docs.rs/known-types-pypi/badge.svg)](https://docs.rs/known-types-pypi/) | Well-known types for Python Package Index (PyPI) APIs.
[known-types-rubygems] | [![known-types-rubygems](https://img.shields.io/crates/v/known-types-rubygems)](https://crates.io/crates/known-types-rubygems) | [![known-types-rubygems](https://docs.rs/known-types-rubygems/badge.svg)](https://docs.rs/known-types-rubygems/) | Well-known types for RubyGems.org APIs.
[known-types-w3c] | [![known-types-w3c](https://img.shields.io/crates/v/known-types-w3c)](https://crates.io/crates/known-types-w3c) | [![known-types-w3c](https://docs.rs/known-types-w3c/badge.svg)](https://docs.rs/known-types-w3c/) | Well-known types for W3C specifications.
[known-types-x] | [![known-types-x](https://img.shields.io/crates/v/known-types-x)](https://crates.io/crates/known-types-x) | [![known-types-x](https://docs.rs/known-types-x/badge.svg)](https://docs.rs/known-types-x/) | Well-known types for X (formerly Twitter) APIs.
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

### Integrations

Crate (Feature) | Version | Usage | Summary
:--- | :--- | :--- | :---
[async-graphql] &nbsp;<sub>(`"async-graphql"`)</sub> | 7.2 | [![async-graphql](https://docs.rs/async-graphql/badge.svg)](https://docs.rs/async-graphql/) | Implements `ScalarType`, `InputType`, `OutputType`, and `connection::CursorType` for handles
[bincode] &nbsp;<sub>(`"bincode"`)</sub> | 2 | [![bincode](https://docs.rs/bincode/badge.svg)](https://docs.rs/bincode/) | Derives `bincode::{Encode, Decode}`
[borsh] &nbsp;<sub>(`"borsh"`)</sub> | 1.5 | [![borsh](https://docs.rs/borsh/badge.svg)](https://docs.rs/borsh/) | Derives `borsh::{BorshSerialize, BorshDeserialize}`
[musli] &nbsp;<sub>(`"musli"`)</sub> | 0.0.131 | [![musli](https://docs.rs/musli/badge.svg)](https://docs.rs/musli/) | Derives `musli::{Encode, Decode}`
[rasn] &nbsp;<sub>(`"rasn"`)</sub> | 0.26 | [![rasn](https://docs.rs/rasn/badge.svg)](https://docs.rs/rasn/) | Derives `rasn::AsnType` with `rasn(automatic_tags)`
[serde] &nbsp;<sub>(`"serde"`)</sub> | 1 | [![serde](https://docs.rs/serde/badge.svg)](https://docs.rs/serde/) | Derives `serde::{Serialize, Deserialize}`
[SQLx] &nbsp;<sub>(`"sqlx"`)</sub> | 0.9 | [![sqlx](https://docs.rs/sqlx/badge.svg)](https://docs.rs/sqlx/) | Implements `sqlx::{Type, Encode, Decode}` for handles
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

### See Also

| Package | Crate | Docs
| :------ | :---- | :---
| [known-errors](https://github.com/it-is-known/known-errors) | [![Package](https://img.shields.io/crates/v/known-errors)](https://crates.io/crates/known-errors) | [![Documentation](https://img.shields.io/docsrs/known-errors?label=docs.rs)](https://docs.rs/known-errors)
| [known-languages](https://github.com/it-is-known/known-languages) | [![Package](https://img.shields.io/crates/v/known-languages)](https://crates.io/crates/known-languages) | [![Documentation](https://img.shields.io/docsrs/known-languages?label=docs.rs)](https://docs.rs/known-languages)
| [known-paths](https://github.com/it-is-known/known-paths) | [![Package](https://img.shields.io/crates/v/known-paths)](https://crates.io/crates/known-paths) | [![Documentation](https://img.shields.io/docsrs/known-paths?label=docs.rs)](https://docs.rs/known-paths)
| [known-schemes](https://github.com/it-is-known/known-schemes) | [![Package](https://img.shields.io/crates/v/known-schemes)](https://crates.io/crates/known-schemes) | [![Documentation](https://img.shields.io/docsrs/known-schemes?label=docs.rs)](https://docs.rs/known-schemes)
| [known-types](https://github.com/it-is-known/known-types) | [![Package](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types) | [![Documentation](https://img.shields.io/docsrs/known-types?label=docs.rs)](https://docs.rs/known-types)

## 👨‍💻 Development

```bash
git clone https://github.com/it-is-known/known-types.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/it-is-known/known-types&text=Known%20Types)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/it-is-known/known-types&title=Known%20Types)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/it-is-known/known-types&t=Known%20Types)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/it-is-known/known-types)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/it-is-known/known-types)

[feature flags]: https://github.com/it-is-known/known-types/blob/master/lib/known-types/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Rust]: https://rust-lang.org
[async-graphql]: https://crates.io/crates/async-graphql
[bincode]: https://crates.io/crates/bincode
[borsh]: https://crates.io/crates/borsh
[musli]: https://crates.io/crates/musli
[rasn]: https://crates.io/crates/rasn
[serde]: https://crates.io/crates/serde
[SQLx]: https://crates.io/crates/sqlx

[known-types]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types
[known-types-anthropic]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-anthropic
[known-types-github]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-github
[known-types-google]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-google
[known-types-graphql]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-graphql
[known-types-gravatar]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-gravatar
[known-types-ietf]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-ietf
[known-types-instagram]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-instagram
[known-types-linkedin]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-linkedin
[known-types-luma]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-luma
[known-types-nostr]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-nostr
[known-types-openai]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-openai
[known-types-pypi]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-pypi
[known-types-rubygems]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-rubygems
[known-types-w3c]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-w3c
[known-types-x]: https://github.com/it-is-known/known-types/tree/master/rust/lib/known-types-x

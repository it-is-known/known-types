# Known Types

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types)
[![Documentation](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types/)

Well-known types for Rust.

## ✨ Features

- Zero default dependencies, only optional [integrations](#integrations).
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

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
cargo add known-types-google
cargo add known-types-graphql
cargo add known-types-ietf
cargo add known-types-nostr
cargo add known-types-openai
cargo add known-types-pypi
cargo add known-types-rubygems
cargo add known-types-w3c
cargo add known-types-x
```

</details>

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
known-types = "0.1"
```

<details>
<summary>Instructions for each crate</summary>

### Installation in `Cargo.toml` (with all features enabled, in all crates)

```toml
[dependencies]
known-types = "0.1"
known-types-anthropic = "0.1"
known-types-google = "0.1"
known-types-graphql = "0.1"
known-types-ietf = "0.1"
known-types-nostr = "0.1"
known-types-openai = "0.1"
known-types-pypi = "0.1"
known-types-rubygems = "0.1"
known-types-w3c = "0.1"
known-types-x = "0.1"
```

</details>

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
known-types = { version = "0.1", default-features = false, features = ["serde"] }
```

<details>
<summary>Instructions for each crate</summary>

### Installation in `Cargo.toml` (with only specific features enabled, in all crates)

```toml
[dependencies]
known-types = { version = "0.1", default-features = false, features = ["serde"] }
known-types-anthropic = { version = "0.1", default-features = false, features = ["serde"] }
known-types-google = { version = "0.1", default-features = false, features = ["serde"] }
known-types-graphql = { version = "0.1", default-features = false, features = ["serde"] }
known-types-ietf = { version = "0.1", default-features = false, features = ["serde"] }
known-types-nostr = { version = "0.1", default-features = false, features = ["serde"] }
known-types-openai = { version = "0.1", default-features = false, features = ["serde"] }
known-types-pypi = { version = "0.1", default-features = false, features = ["serde"] }
known-types-rubygems = { version = "0.1", default-features = false, features = ["serde"] }
known-types-w3c = { version = "0.1", default-features = false, features = ["serde"] }
known-types-x = { version = "0.1", default-features = false, features = ["serde"] }
```

</details>

## 👉 Examples

### Importing the library

```rust
use known_types;
```

<details>
<summary>Instructions for each crate</summary>

### Importing the library (all crates)

```rust
use known_types;
use known_types_anthropic;
use known_types_google;
use known_types_graphql;
use known_types_ietf;
use known_types_nostr;
use known_types_openai;
use known_types_pypi;
use known_types_rubygems;
use known_types_w3c;
use known_types_x;
```

</details>

## 📚 Reference

### Crates

Crate | Version | Docs | Summary
:--- | :--- | :--- | :---
[known-types] | [![known-types](https://img.shields.io/crates/v/known-types)](https://crates.io/crates/known-types) | [![known-types](https://docs.rs/known-types/badge.svg)](https://docs.rs/known-types/) | Well-known types.
[known-types-anthropic] | [![known-types-anthropic](https://img.shields.io/crates/v/known-types-anthropic)](https://crates.io/crates/known-types-anthropic) | [![known-types-anthropic](https://docs.rs/known-types-anthropic/badge.svg)](https://docs.rs/known-types-anthropic/) | Well-known types for Anthropic APIs.
[known-types-google] | [![known-types-google](https://img.shields.io/crates/v/known-types-google)](https://crates.io/crates/known-types-google) | [![known-types-google](https://docs.rs/known-types-google/badge.svg)](https://docs.rs/known-types-google/) | Well-known types for Google APIs.
[known-types-graphql] | [![known-types-graphql](https://img.shields.io/crates/v/known-types-graphql)](https://crates.io/crates/known-types-graphql) | [![known-types-graphql](https://docs.rs/known-types-graphql/badge.svg)](https://docs.rs/known-types-graphql/) | Well-known types for GraphQL specifications.
[known-types-ietf] | [![known-types-ietf](https://img.shields.io/crates/v/known-types-ietf)](https://crates.io/crates/known-types-ietf) | [![known-types-ietf](https://docs.rs/known-types-ietf/badge.svg)](https://docs.rs/known-types-ietf/) | Well-known types for IETF specifications.
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
[bincode] &nbsp;<sub>(`"bincode"`)</sub> | 2 | [![bincode](https://docs.rs/bincode/badge.svg)](https://docs.rs/bincode/) | Derives `bincode::{Encode, Decode}`
[borsh] &nbsp;<sub>(`"borsh"`)</sub> | 1.5 | [![borsh](https://docs.rs/borsh/badge.svg)](https://docs.rs/borsh/) | Derives `borsh::{BorshSerialize, BorshDeserialize}`
[musli] &nbsp;<sub>(`"musli"`)</sub> | 0.0.131 | [![musli](https://docs.rs/musli/badge.svg)](https://docs.rs/musli/) | Derives `musli::{Encode, Decode}`
[rasn] &nbsp;<sub>(`"rasn"`)</sub> | 0.26 | [![rasn](https://docs.rs/rasn/badge.svg)](https://docs.rs/rasn/) | Derives `rasn::AsnType` with `rasn(automatic_tags)`
[serde] &nbsp;<sub>(`"serde"`)</sub> | 1 | [![serde](https://docs.rs/serde/badge.svg)](https://docs.rs/serde/) | Derives `serde::{Serialize, Deserialize}`
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

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

[bincode]: https://crates.io/crates/bincode
[borsh]: https://crates.io/crates/borsh
[musli]: https://crates.io/crates/musli
[rasn]: https://crates.io/crates/rasn
[serde]: https://crates.io/crates/serde

[known-types]: https://github.com/it-is-known/known-types/tree/master/lib/known-types
[known-types-anthropic]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-anthropic
[known-types-google]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-google
[known-types-graphql]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-graphql
[known-types-ietf]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-ietf
[known-types-nostr]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-nostr
[known-types-openai]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-openai
[known-types-pypi]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-pypi
[known-types-rubygems]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-rubygems
[known-types-w3c]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-w3c
[known-types-x]: https://github.com/it-is-known/known-types/tree/master/lib/known-types-x

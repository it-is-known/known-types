# Rust workspace

Stay within this directory and its descendants; run Cargo here. Rust 2024
workspace: `lib/*` are crates; root `Cargo.toml` owns shared versions/dependencies.
`known-types` is shared core, not an umbrella re-export. Several crates are
scaffolds: inspect `src/lib.rs` exports before assuming an API exists.

## Where to edit

- `lib/known-types/src/{handle.rs,c.rs}`: shared handle errors/validators/macros;
  C aliases, respectively.
- `lib/known-types-*/src/handle.rs`: platform-specific parsing and identity.
  Crate roots re-export handles behind `alloc`.
- `lib/known-types-pypi/src/package.rs`, `lib/known-types-rubygems/src/gem.rs`:
  package metadata. OpenAI has separate generation rules below.

## Invariants

- Preserve `no_std` and crate lints: no unsafe, public types implement `Debug`,
  no `unwrap()`. Use `core` for primitives, `alloc` for owned data; gate
  heap-backed modules/re-exports with `alloc`. `std` enables `alloc`; `all` is empty.
- Inherit dependencies from `[workspace.dependencies]` with defaults disabled.
  Integrations requiring `std` must enable it explicitly.
- Keep handle strings private. `FromStr` owns validation; all fallible
  conversions and decoders must call it. Extend `impl_handle!` for integrations;
  its feature gates expand in calling crates, so update their manifests too.
- Keep `Eq`/`Ord`/`Hash` consistent. Preserve platform spelling, case-folding and
  punctuation rules. Count Unicode scalar values after parser normalization.
  Preserve legacy bounds: X 1–20, Telegram 4–32; Intro.co/local.ai/Luma 1–100
  fallback bounds. Consult type rustdoc/tests before changing validation.
- LinkedIn trims outer whitespace, strictly percent-decodes UTF-8 once, then
  validates. Stored handles must parse and round-trip without changing spelling.
- Put contracts, examples and upstream references in module/type rustdoc.
  Prefer shortening/correcting README; additions need exceptional value.
  Record public API/behavior changes in `CHANGES.md`.

## OpenAI generation

In `lib/known-types-openai/`, `Rakefile` + `.rake/` generate `src/schemas.rs`,
`src/groups.rs` and `src/groups/*.rs`. Fix the generator or handwritten
`src/schemas/*.rs` overrides (including `footer.rs`), never generated output alone.
Keep role-tag overrides consistent with `.rake/omit_fields.csv`.
Run `rake codegen` from that crate; it needs the ignored, unbundled `openapi.yaml`,
Ruby 3.4+ and gems `rake`, `codify.rb`, `hashie`.

## Verify Rust changes

Replace `CRATE` with the affected package. Start with `cargo test -p CRATE --locked`;
run `cargo clippy -p CRATE --locked --all-targets -- -D warnings`, enabling changed
integrations. Check each affected crate with defaults disabled, with `alloc`, and
with each changed feature independently; feature unification can hide mistakes.

For shared handle changes, test all callers:

```sh
cargo test --workspace --exclude known-types-openai --locked --features serde,async-graphql,sqlx-postgres,sqlx-mysql,sqlx-sqlite
cargo check --workspace --locked --no-default-features --features alloc
cargo fmt --all -- --check
```

- Add boundary/rejection/round-trip tests for behavior changes. X and LinkedIn
  `tests/` show integration patterns; SQLite tests need no external database.
- OpenAI: `cargo test -p known-types-openai --locked --test test_deser`.
- Avoid workspace `--all-features` on stable: it enables unstable C aliases via
  `known-types/nightly`. Stable rustfmt ignores `imports_granularity`.
  Keep formatting scoped; report baseline failures rather than sweeping fixes.

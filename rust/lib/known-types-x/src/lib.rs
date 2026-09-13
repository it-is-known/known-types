// This is free and unencumbered software released into the public domain.

//! Validated X (formerly Twitter) usernames.
//!
//! `XHandle` is available with `alloc` (enabled by the default `std` feature).
//! See the [shared handle contract](known_types::handle) for conversion, identity,
//! and validation rules. The Serde, GraphQL, and SQLx recipes also apply to other
//! platform handles, using their own parsing rules and GraphQL scalar names.
//!
//! # Parsing handles
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! # {
//! use known_types_x::XHandle;
//!
//! let handle: XHandle = "@PlayItAgainSam".parse()?;
//! assert_eq!(handle.as_str(), "PlayItAgainSam");
//! assert_eq!(handle, "playitagainsam".parse::<XHandle>()?);
//! assert!("not a handle".parse::<XHandle>().is_err());
//! # }
//! # Ok::<(), known_types::handle::ParseHandleError>(())
//! ```
//!
//! # Serde
//!
//! Enable `serde` to encode handles as strings and validate on decode. For
//! `no_std` applications, also enable `alloc` explicitly:
//!
//! ```toml
//! [dependencies]
//! known-types-x = { version = "0.1", default-features = false, features = ["alloc", "serde"] }
//! serde_json = { version = "1", default-features = false, features = ["alloc"] }
//! ```
//!
//! ```
//! # #[cfg(all(feature = "alloc", feature = "serde"))]
//! # {
//! use known_types_x::XHandle;
//!
//! let handle: XHandle = serde_json::from_str(r#""@Some_User""#)?;
//! assert_eq!(handle.as_str(), "Some_User");
//! assert_eq!(serde_json::to_string(&handle)?, r#""Some_User""#);
//! assert!(serde_json::from_str::<XHandle>(r#""bad handle""#).is_err());
//! # }
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! # GraphQL
//!
//! The `async-graphql` feature enables `std` and works without `serde` on this
//! crate. Scalars accept only strings and validate them with `FromStr`.
//!
//! ```toml
//! [dependencies]
//! known-types-x = { version = "0.1", default-features = false, features = ["async-graphql"] }
//! async-graphql = { version = "7.2", default-features = false }
//! ```
//!
//! ```
//! # #[cfg(feature = "async-graphql")]
//! # futures_executor::block_on(async {
//! use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, value};
//! use known_types_x::XHandle;
//!
//! struct Query;
//!
//! #[Object]
//! impl Query {
//!     async fn handle(&self, input: XHandle) -> XHandle {
//!         input
//!     }
//! }
//!
//! let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
//! let response = schema.execute(r#"{ handle(input: "@Some_User") }"#).await;
//! assert!(response.errors.is_empty());
//! assert_eq!(response.data, value!({"handle": "Some_User"}));
//! # });
//! ```
//!
//! The field is `handle(input: XHandle!): XHandle!`. Variables must use the
//! scalar's name, for example `query($handle: XHandle!) { handle(input: $handle) }`
//! with `{"handle": "Some_User"}`. Handles also work in input/output objects,
//! optional fields, and lists.
//!
//! ## Connection cursors
//!
//! Cursors encode the stored spelling verbatim and validate on decode. Use a
//! handle as a cursor when it is the connection's unique ordering key; ordering
//! and rename handling remain the application's responsibility.
//!
//! ```
//! # #[cfg(feature = "async-graphql")]
//! # {
//! use async_graphql::connection::{Connection, CursorType, Edge};
//! use known_types_x::XHandle;
//!
//! let handle: XHandle = "Some_User".parse()?;
//! let mut connection: Connection<XHandle, XHandle> = Connection::new(false, false);
//! connection.edges.push(Edge::new(handle.clone(), handle.clone()));
//! assert_eq!(handle.encode_cursor(), "Some_User");
//! assert_eq!(XHandle::decode_cursor(&handle.encode_cursor())?, handle);
//! # }
//! # Ok::<(), known_types::handle::ParseHandleError>(())
//! ```
//!
//! # SQLx
//!
//! Select `sqlx-postgres`, `sqlx-mysql`, or `sqlx-sqlite`; each enables `std`.
//! If the application already enables a SQLx driver, `sqlx` alone supplies the
//! scalar traits. Handles bind by value or reference, support `Option<XHandle>`
//! for nullable columns, and validate database text on decode. PostgreSQL text
//! arrays additionally support `Vec<XHandle>` with `sqlx-postgres`.
//!
//! ```toml
//! [dependencies]
//! known-types-x = { version = "0.1", default-features = false, features = ["sqlx-postgres"] }
//! sqlx = { version = "0.9", default-features = false, features = ["postgres", "runtime-tokio"] }
//! ```
//!
//! ```no_run
//! # #[cfg(feature = "sqlx-postgres")]
//! # {
//! use known_types_x::XHandle;
//!
//! async fn round_trip(pool: &sqlx::PgPool, handle: &XHandle) -> Result<XHandle, sqlx::Error> {
//!     sqlx::query_scalar("SELECT $1::text")
//!         .bind(handle)
//!         .fetch_one(pool)
//!         .await
//! }
//! # }
//! ```
//!
//! For SQLite, enable `sqlx-sqlite` instead. This in-memory example needs no
//! external database:
//!
//! ```
//! # #[cfg(feature = "sqlx-sqlite")]
//! # {
//! # futures_executor::block_on(async {
//! use known_types_x::XHandle;
//! use sqlx::{Connection, SqliteConnection};
//!
//! let mut connection = SqliteConnection::connect("sqlite::memory:").await?;
//! let handle: XHandle = "@Some_User".parse()?;
//! let decoded: XHandle = sqlx::query_scalar("SELECT ?")
//!     .bind(&handle)
//!     .fetch_one(&mut connection)
//!     .await?;
//! assert_eq!(decoded.as_str(), handle.as_str());
//!
//! let invalid = sqlx::query_scalar::<_, XHandle>("SELECT 'bad handle'")
//!     .fetch_one(&mut connection)
//!     .await;
//! assert!(matches!(invalid, Err(sqlx::Error::ColumnDecode { .. })));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # })?;
//! # }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! For SQLx's compile-time query macros (its `macros` feature), provide an
//! explicit column type override: `SELECT handle AS "handle: XHandle"` with
//! `query!`, or `SELECT handle AS "handle: _"` with `query_as!` and a struct field
//! of type `XHandle`. Runtime selection and compile-time database configuration
//! belong to the application.
//!
//! # libsql
//!
//! With `libsql`, an owned or borrowed `XHandle` converts into
//! `libsql::Value::Text`, preserving its stored spelling. Select `alloc,libsql`
//! when default features are disabled. This integration currently supplies
//! value encoding only; it does not decode database values into handles.

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::alloc_instead_of_core)]

//#[cfg(doctest)]
//#[doc = include_str!("../README.md")]
//pub struct ReadmeDoctests;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod handle;
#[cfg(feature = "alloc")]
pub use handle::*;

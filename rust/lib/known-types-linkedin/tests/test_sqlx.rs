// This is free and unencumbered software released into the public domain.

#![cfg(feature = "sqlx-sqlite")]

use known_types_linkedin::LinkedinHandle;
use sqlx::{Connection, SqliteConnection};

#[test]
fn test_sqlite_round_trip_preserves_normalized_handle() -> Result<(), sqlx::Error> {
    futures_executor::block_on(async {
        let mut connection = SqliteConnection::connect("sqlite::memory:").await?;

        for (input, stored) in [
            (" bj%C3%B6rn ", "björn"),
            ("John%2DSmith", "John-Smith"),
            ("Bj%C3%96rn", "BjÖrn"),
        ] {
            let handle = LinkedinHandle::try_from(input).expect("valid handle");
            assert_eq!(handle.to_string(), stored);

            let encoded: String = sqlx::query_scalar("SELECT ?")
                .bind(&handle)
                .fetch_one(&mut connection)
                .await?;
            assert_eq!(encoded, stored);

            // Validated stored text parses idempotently.
            let decoded: LinkedinHandle = sqlx::query_scalar("SELECT ?")
                .bind(handle.clone())
                .fetch_one(&mut connection)
                .await?;
            assert_eq!(decoded, handle);
            assert_eq!(decoded.as_str(), stored);

            let decoded: LinkedinHandle = sqlx::query_scalar("SELECT ?")
                .bind(input)
                .fetch_one(&mut connection)
                .await?;
            assert_eq!(decoded.as_str(), stored);
        }

        for input in [
            "",
            "ab",
            "%FF",
            "literal%2520handle",
            "%20handle%20",
            "abc%2Fdef",
            "abc%GG",
            "a".repeat(101).as_str(),
        ] {
            let error = sqlx::query_scalar::<_, LinkedinHandle>("SELECT ?")
                .bind(input)
                .fetch_one(&mut connection)
                .await
                .expect_err("invalid database text must not decode as a handle");
            assert!(matches!(error, sqlx::Error::ColumnDecode { .. }));
        }

        Ok(())
    })
}

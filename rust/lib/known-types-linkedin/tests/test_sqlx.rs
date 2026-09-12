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
            ("literal%2520handle", "literal%20handle"),
            ("%20handle%20", " handle "),
        ] {
            let handle = LinkedinHandle::from(input);
            assert_eq!(handle.to_string(), stored);

            let encoded: String = sqlx::query_scalar("SELECT ?")
                .bind(&handle)
                .fetch_one(&mut connection)
                .await?;
            assert_eq!(encoded, stored);

            // Database text is already normalized: parsing it again would
            // double-decode percent escapes or trim meaningful whitespace.
            let decoded: LinkedinHandle = sqlx::query_scalar("SELECT ?")
                .bind(handle.clone())
                .fetch_one(&mut connection)
                .await?;
            assert_eq!(decoded, handle);
        }

        Ok(())
    })
}

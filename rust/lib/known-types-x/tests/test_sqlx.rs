// This is free and unencumbered software released into the public domain.

#![cfg(any(
    feature = "sqlx-postgres",
    feature = "sqlx-mysql",
    feature = "sqlx-sqlite"
))]

use known_types_x::XHandle;
use sqlx::{Database, Decode, Encode, Type};

// Check the bounds used by bind(), query_scalar(), and nullable columns.
fn assert_scalar<DB, T>()
where
    DB: Database,
    T: Type<DB> + for<'q> Encode<'q, DB> + for<'r> Decode<'r, DB>,
{
}

#[test]
fn test_sqlx_traits() {
    fn assert_handle<DB: Database>()
    where
        String: Type<DB> + for<'q> Encode<'q, DB> + for<'r> Decode<'r, DB>,
    {
        assert_scalar::<DB, XHandle>();
    }

    #[cfg(feature = "sqlx-postgres")]
    {
        assert_handle::<sqlx::Postgres>();
        assert_scalar::<sqlx::Postgres, Option<XHandle>>();
        assert_scalar::<sqlx::Postgres, Vec<XHandle>>();
    }
    #[cfg(feature = "sqlx-mysql")]
    {
        assert_handle::<sqlx::MySql>();
        assert_scalar::<sqlx::MySql, Option<XHandle>>();
    }
    #[cfg(feature = "sqlx-sqlite")]
    {
        assert_handle::<sqlx::Sqlite>();
        assert_scalar::<sqlx::Sqlite, Option<XHandle>>();
    }
}

#[cfg(feature = "sqlx-postgres")]
#[test]
fn test_postgres_text_encoding() -> Result<(), sqlx::error::BoxDynError> {
    use sqlx::{Postgres, postgres::PgArgumentBuffer};

    assert_eq!(<XHandle as Type<Postgres>>::type_info().to_string(), "TEXT");
    assert!(<XHandle as Type<Postgres>>::compatible(
        &sqlx::postgres::PgTypeInfo::with_name("VARCHAR")
    ));

    let handle = XHandle::try_from("Some_User").expect("valid handle");
    let mut buffer = PgArgumentBuffer::default();
    assert!(!<XHandle as Encode<Postgres>>::encode_by_ref(&handle, &mut buffer)?.is_null());
    assert_eq!(&buffer[..], b"Some_User");

    let _query = sqlx::query::<Postgres>("SELECT $1, $2, $3")
        .bind(&handle)
        .bind(handle.clone())
        .bind(vec![handle]);
    Ok(())
}

#[cfg(feature = "sqlx-sqlite")]
#[test]
fn test_sqlite_round_trip() -> Result<(), sqlx::Error> {
    use sqlx::{Connection, SqliteConnection};

    futures_executor::block_on(async {
        let mut connection = SqliteConnection::connect("sqlite::memory:").await?;

        for text in [
            "Some_User",
            "PlayItAgainSam",
            "richardrushfield",
            "abcdefghijklmnopqrst",
            "x",
            "_user_",
        ] {
            let handle = XHandle::try_from(text).expect("valid handle");
            let decoded: XHandle = sqlx::query_scalar("SELECT ?")
                .bind(&handle)
                .fetch_one(&mut connection)
                .await?;
            assert_eq!(decoded, handle);

            let (decoded, some, none): (XHandle, Option<XHandle>, Option<XHandle>) =
                sqlx::query_as("SELECT ?, ?, ?")
                    .bind(handle.clone())
                    .bind(Some(handle.clone()))
                    .bind(None::<XHandle>)
                    .fetch_one(&mut connection)
                    .await?;
            assert_eq!(decoded, handle);
            assert_eq!(some, Some(handle));
            assert_eq!(none, None);
        }

        let normalized: XHandle = sqlx::query_scalar("SELECT '@Some_User'")
            .fetch_one(&mut connection)
            .await?;
        assert_eq!(normalized.as_str(), "Some_User");

        for input in [
            "",
            "@",
            "@@User",
            "björn",
            "literal%20handle",
            "a".repeat(21).as_str(),
        ] {
            let error = sqlx::query_scalar::<_, XHandle>("SELECT ?")
                .bind(input)
                .fetch_one(&mut connection)
                .await
                .expect_err("invalid database text must not decode as a handle");
            assert!(matches!(error, sqlx::Error::ColumnDecode { .. }));
        }

        let error = sqlx::query_scalar::<_, XHandle>("SELECT 42")
            .fetch_one(&mut connection)
            .await
            .expect_err("integer columns must not decode as handles");
        assert!(matches!(error, sqlx::Error::ColumnDecode { .. }));

        Ok(())
    })
}

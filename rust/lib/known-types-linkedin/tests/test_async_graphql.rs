// This is free and unencumbered software released into the public domain.

#![cfg(feature = "async-graphql")]

use async_graphql::{
    EmptyMutation, EmptySubscription, InputType, Object, Request, ScalarType, Schema, Value,
    Variables, connection::CursorType, value,
};
use known_types_linkedin::LinkedinHandle;

struct Query;

#[Object]
impl Query {
    async fn handle(&self, input: LinkedinHandle) -> LinkedinHandle {
        input
    }
}

#[test]
fn test_query_input_normalization() {
    futures_executor::block_on(async {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        assert_eq!(<LinkedinHandle as InputType>::type_name(), "LinkedinHandle");
        assert!(schema.sdl().contains("scalar LinkedinHandle"));

        for (input, stored) in [
            ("foobar", "foobar"),
            (" bj%C3%B6rn ", "björn"),
            ("John%2dSmith", "John-Smith"),
            ("%E6%9D%8E%E5%B0%8F%E9%BE%8D", "李小龍"),
        ] {
            assert!(<LinkedinHandle as ScalarType>::is_valid(&Value::String(
                input.into()
            )));
            let response = schema
                .execute(
                    Request::new("query($handle: LinkedinHandle!) { handle(input: $handle) }")
                        .variables(Variables::from_value(value!({"handle": input}))),
                )
                .await;
            assert!(response.errors.is_empty(), "{:?}", response.errors);
            assert_eq!(response.data, value!({"handle": stored}));
        }

        let response = schema.execute(r#"{ handle(input: " bj%C3%B6rn ") }"#).await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
        assert_eq!(response.data, value!({"handle": "björn"}));
    });
}

#[test]
fn test_invalid_inputs_return_errors() {
    futures_executor::block_on(async {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        for input in [
            Value::String("".into()),
            Value::String(" \t\n ".into()),
            Value::String("%FF".into()),
            Value::String("%C3%28".into()),
            value!("literal%2520handle"),
            value!("%20handle%20"),
            value!("abc%"),
            value!("abc%GG"),
            value!("abc%2Fdef"),
            value!("ab"),
            value!("a".repeat(101)),
            Value::Null,
            value!(42),
            value!(true),
            value!([]),
            value!({}),
        ] {
            assert!(!<LinkedinHandle as ScalarType>::is_valid(&input));
            assert!(<LinkedinHandle as InputType>::parse(Some(input.clone())).is_err());
            let response = schema
                .execute(
                    Request::new("query($handle: LinkedinHandle!) { handle(input: $handle) }")
                        .variables(Variables::from_value(value!({"handle": input}))),
                )
                .await;
            assert!(!response.errors.is_empty());
        }

        for literal in [r#""""#, r#""   ""#, r#""%FF""#, "42", "null"] {
            let response = schema
                .execute(format!("{{ handle(input: {literal}) }}"))
                .await;
            assert!(!response.errors.is_empty(), "accepted {literal}");
        }
    });
}

#[test]
fn test_cursors_preserve_normalized_handles() {
    for (input, stored) in [
        (" bj%C3%B6rn ", "björn"),
        ("John%2DSmith", "John-Smith"),
        ("Bj%C3%96rn", "BjÖrn"),
    ] {
        let handle = LinkedinHandle::try_from(input).expect("valid handle");
        assert_eq!(handle.encode_cursor(), stored);
        assert_eq!(
            LinkedinHandle::decode_cursor(&handle.encode_cursor()),
            Ok(handle)
        );
    }
    for input in [
        "",
        "ab",
        "%20",
        "%25FF",
        "literal%2520handle",
        "abc%2fdef",
        "abc%FF",
    ] {
        assert!(LinkedinHandle::decode_cursor(input).is_err());
    }
}

// This is free and unencumbered software released into the public domain.

#![cfg(feature = "async-graphql")]

use async_graphql::{
    EmptyMutation, EmptySubscription, InputObject, InputType, Name, Object, Request, ScalarType,
    Schema, SimpleObject, Value, Variables,
    connection::{self, Connection, CursorType, Edge},
    value,
};
use known_types_x::XHandle;

#[derive(InputObject)]
struct HandlesInput {
    handles: Vec<XHandle>,
    optional: Option<XHandle>,
}

#[derive(SimpleObject)]
struct HandlesOutput {
    handles: Vec<XHandle>,
    optional: Option<XHandle>,
}

struct Query;

#[Object]
impl Query {
    async fn handle(&self, input: XHandle) -> XHandle {
        input
    }

    async fn handles(&self, input: HandlesInput) -> HandlesOutput {
        HandlesOutput {
            handles: input.handles,
            optional: input.optional,
        }
    }

    async fn handle_connection(
        &self,
        after: Option<String>,
    ) -> async_graphql::Result<Connection<XHandle, XHandle>> {
        connection::query(
            after,
            None,
            None,
            None,
            |after: Option<XHandle>, _before: Option<XHandle>, _, _| async move {
                let handle = after.unwrap_or_else(|| "Some_User".parse().expect("valid handle"));
                let mut connection = Connection::new(false, false);
                connection.edges.push(Edge::new(handle.clone(), handle));
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }
}

#[test]
fn test_string_scalar_and_cursor_round_trip() {
    assert_eq!(<XHandle as InputType>::type_name(), "XHandle");

    for (text, stored) in [
        ("Some_User", "Some_User"),
        ("@User", "User"),
        ("x", "x"),
        ("PlayItAgainSam", "PlayItAgainSam"),
    ] {
        let value = Value::String(text.into());
        let handle = <XHandle as InputType>::parse(Some(value.clone())).expect("valid handle");
        assert!(<XHandle as ScalarType>::is_valid(&value));
        assert_eq!(handle.as_str(), stored);
        assert_eq!(<XHandle as ScalarType>::to_value(&handle), value!(stored));
        assert_eq!(<XHandle as InputType>::to_value(&handle), value!(stored));
        assert_eq!(handle.encode_cursor(), stored);
        assert_eq!(XHandle::decode_cursor(&handle.encode_cursor()), Ok(handle));
    }
}

#[test]
fn test_rejects_invalid_handles_and_cursors() {
    for text in [
        "",
        "@",
        "@@User",
        " bj%C3%B6rn ",
        "literal%20handle",
        "björn",
        "a".repeat(21).as_str(),
    ] {
        let value = Value::String(text.into());
        assert!(!<XHandle as ScalarType>::is_valid(&value));
        assert!(<XHandle as InputType>::parse(Some(value)).is_err());
        assert!(XHandle::decode_cursor(text).is_err());
    }
}

#[test]
fn test_rejects_non_string_scalars() {
    for value in [
        Value::Null,
        value!(42),
        value!(1.5),
        value!(true),
        Value::Enum(Name::new("Some_User")),
        value!(["Some_User"]),
        value!({"handle": "Some_User"}),
        Value::Binary(vec![1, 2, 3].into()),
    ] {
        assert!(!<XHandle as ScalarType>::is_valid(&value));
        assert!(<XHandle as ScalarType>::parse(value.clone()).is_err());
        assert!(<XHandle as InputType>::parse(Some(value)).is_err());
    }
    assert!(<XHandle as InputType>::parse(None).is_err());
    assert_eq!(
        <Option<XHandle> as InputType>::parse(None).expect("optional input"),
        None
    );
}

#[test]
fn test_query_literals_variables_and_input_objects() {
    futures_executor::block_on(async {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        let sdl = schema.sdl();
        assert!(sdl.contains("scalar XHandle"));
        assert!(sdl.contains("handle(input: XHandle!): XHandle!"));

        let response = schema
            .execute(r#"{ handle(input: "Some_User") handles(input: {handles: ["Alice", "Bob"]}) { handles optional } }"#)
            .await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
        assert_eq!(
            response.data,
            value!({
                "handle": "Some_User",
                "handles": {"handles": ["Alice", "Bob"], "optional": null},
            })
        );

        let response = schema
            .execute(Request::new(
                "query($handle: XHandle!, $input: HandlesInput!) { handle(input: $handle) handles(input: $input) { handles optional } }",
            ).variables(Variables::from_value(value!({
                "handle": "Mixed_Case",
                "input": {"handles": ["Alice", "Bob"], "optional": "Some_User"},
            }))))
            .await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
        assert_eq!(
            response.data,
            value!({
                "handle": "Mixed_Case",
                "handles": {"handles": ["Alice", "Bob"], "optional": "Some_User"},
            })
        );
    });
}

#[test]
fn test_invalid_query_inputs() {
    futures_executor::block_on(async {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        for literal in [
            "42",
            "1.5",
            "true",
            "null",
            "Some_User",
            "[]",
            "{}",
            r#""""#,
            r#""bad-handle""#,
        ] {
            let response = schema
                .execute(format!("{{ handle(input: {literal}) }}"))
                .await;
            assert!(!response.errors.is_empty(), "accepted {literal}");
        }
        for input in [
            value!(42),
            value!(true),
            Value::Null,
            value!([]),
            value!({}),
            value!(""),
            value!("@@User"),
        ] {
            let response = schema
                .execute(
                    Request::new("query($handle: XHandle!) { handle(input: $handle) }")
                        .variables(Variables::from_value(value!({"handle": input}))),
                )
                .await;
            assert!(!response.errors.is_empty());
        }
        let response = schema
            .execute("query($handle: XHandle!) { handle(input: $handle) }")
            .await;
        assert!(!response.errors.is_empty());
    });
}

#[test]
fn test_connection_query() {
    futures_executor::block_on(async {
        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        let response = schema
            .execute(r#"{ handleConnection(after: "Mixed_Case") { edges { cursor node } pageInfo { startCursor endCursor } } }"#)
            .await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
        assert_eq!(
            response.data,
            value!({
                "handleConnection": {
                    "edges": [{"cursor": "Mixed_Case", "node": "Mixed_Case"}],
                    "pageInfo": {"startCursor": "Mixed_Case", "endCursor": "Mixed_Case"},
                },
            })
        );
        let response = schema
            .execute(r#"{ handleConnection(after: "bad-handle") { edges { cursor } } }"#)
            .await;
        assert!(!response.errors.is_empty());
    });
}

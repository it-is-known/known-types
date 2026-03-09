// This is free and unencumbered software released into the public domain.

use known_types_openai::schemas::ChatCompletionRequestMessage;

#[test]
fn test_serde_deserialization1() {
    let json = r#"{
        "role": "user",
        "content": "Hello"
    }"#;

    let _message = serde_json::from_str::<ChatCompletionRequestMessage>(json).unwrap();
}

#[test]
fn test_serde_deserialization2() {
    let json = r#"[{
        "role": "user",
        "content": "Hello"
    },{
        "role": "assistant",
        "content": "Hello"
    }]"#;

    let _message = serde_json::from_str::<Vec<ChatCompletionRequestMessage>>(json).unwrap();
}

// This is free and unencumbered software released into the public domain.

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "role", rename_all = "lowercase"))]
pub enum ChatCompletionRequestMessage {
    #[cfg_attr(feature = "serde", serde(rename = "developer"))]
    Developer(ChatCompletionRequestDeveloperMessage),

    #[cfg_attr(feature = "serde", serde(rename = "system"))]
    System(ChatCompletionRequestSystemMessage),

    #[cfg_attr(feature = "serde", serde(rename = "user"))]
    User(ChatCompletionRequestUserMessage),

    #[cfg_attr(feature = "serde", serde(rename = "assistant"))]
    Assistant(ChatCompletionRequestAssistantMessage),

    #[cfg_attr(feature = "serde", serde(rename = "tool"))]
    Tool(ChatCompletionRequestToolMessage),

    #[cfg_attr(feature = "serde", serde(rename = "function"))]
    Function(ChatCompletionRequestFunctionMessage),
}

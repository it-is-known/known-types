// This is free and unencumbered software released into the public domain.

impl ChatCompletionRequestMessage {
    pub fn role(&self) -> &str {
        use ChatCompletionRequestMessage::*;
        match self {
            Developer(ChatCompletionRequestDeveloperMessage { .. }) => "developer",
            System(ChatCompletionRequestSystemMessage { .. }) => "system",
            User(ChatCompletionRequestUserMessage { .. }) => "user",
            Assistant(ChatCompletionRequestAssistantMessage { .. }) => "assistant",
            Tool(ChatCompletionRequestToolMessage { .. }) => "tool",
            Function(ChatCompletionRequestFunctionMessage { .. }) => "function",
        }
    }

    pub fn name(&self) -> Option<&str> {
        use ChatCompletionRequestMessage::*;
        match self {
            Developer(ChatCompletionRequestDeveloperMessage { name, .. }) => {
                name.as_ref().map(|s| s.as_str())
            }
            System(ChatCompletionRequestSystemMessage { name, .. }) => {
                name.as_ref().map(|s| s.as_str())
            }
            User(ChatCompletionRequestUserMessage { name, .. }) => {
                name.as_ref().map(|s| s.as_str())
            }
            Assistant(ChatCompletionRequestAssistantMessage { name, .. }) => {
                name.as_ref().map(|s| s.as_str())
            }
            Tool(..) => return None,
            Function(ChatCompletionRequestFunctionMessage { name, .. }) => {
                if name.as_str().is_empty() {
                    None
                } else {
                    Some(name.as_str())
                }
            }
        }
    }

    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestMessage::*;
        match self {
            Developer(ChatCompletionRequestDeveloperMessage { content, .. }) => {
                content.text_content()
            }
            System(ChatCompletionRequestSystemMessage { content, .. }) => content.text_content(),
            User(ChatCompletionRequestUserMessage { content, .. }) => content.text_content(),
            Assistant(ChatCompletionRequestAssistantMessage { content, .. }) => {
                content.as_ref().and_then(|c| c.text_content())
            }
            Tool(ChatCompletionRequestToolMessage { content, .. }) => content.text_content(),
            Function(ChatCompletionRequestFunctionMessage { content, .. }) => {
                content.as_ref().map(|s| s.as_str())
            }
        }
    }
}

impl ChatCompletionRequestDeveloperMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestDeveloperMessage_Content::*;
        match self {
            Text(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestSystemMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestSystemMessage_Content::*;
        match self {
            Text(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestUserMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestUserMessage_Content::*;
        match self {
            Text(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestAssistantMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestAssistantMessage_Content::*;
        match self {
            Text(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

impl ChatCompletionRequestToolMessage_Content {
    pub fn text_content(&self) -> Option<&str> {
        use ChatCompletionRequestToolMessage_Content::*;
        match self {
            Text(s) => Some(s.as_str()),
            Array(..) => None,
        }
    }
}

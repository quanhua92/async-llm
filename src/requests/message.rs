use serde::{Deserialize, Serialize};

use crate::{
    chat::{
        ChatCompletionRequestAssistantMessageBuilder, ChatCompletionRequestDeveloperMessageBuilder,
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageBuilder,
        ChatCompletionRequestToolMessageBuilder, ChatCompletionRequestUserMessageBuilder,
    },
    error::Error,
    types::{AssistantContent, Content},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChatMessage {
    Developer(MessageContent),
    System(MessageContent),
    User(MessageContent),
    Assistant {
        message_content: Option<MessageContent>,
        refusal: Option<String>,
        audio: Option<String>,
        //
    },
    Tool {
        message_content: MessageContent,
        tool_call_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageContent {
    Text(String),
    Array(Vec<String>),
}

impl ChatMessage {
    pub fn system(message: impl Into<MessageContent>) -> Self {
        Self::System(message.into())
    }

    pub fn user(message: impl Into<MessageContent>) -> Self {
        Self::User(message.into())
    }

    pub fn developer(message: impl Into<MessageContent>) -> Self {
        Self::Developer(message.into())
    }

    pub fn assistant(message: impl Into<MessageContent>) -> Self {
        Self::Assistant {
            message_content: Some(message.into()),
            refusal: None,
            audio: None,
        }
    }

    pub fn tool(message: impl Into<MessageContent>, tool_call_id: impl Into<String>) -> Self {
        Self::Tool {
            message_content: message.into(),
            tool_call_id: tool_call_id.into(),
        }
    }
}

impl From<&str> for MessageContent {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for MessageContent {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl TryInto<ChatCompletionRequestMessage> for ChatMessage {
    type Error = Error;
    fn try_into(self) -> Result<ChatCompletionRequestMessage, Self::Error> {
        Ok(match self {
            ChatMessage::Developer(message_content) => {
                ChatCompletionRequestDeveloperMessageBuilder::default()
                    .content(message_content)
                    .build()?
                    .into()
            }
            ChatMessage::System(message_content) => {
                ChatCompletionRequestSystemMessageBuilder::default()
                    .content(message_content)
                    .build()?
                    .into()
            }
            ChatMessage::User(message_content) => {
                ChatCompletionRequestUserMessageBuilder::default()
                    .content(message_content)
                    .build()?
                    .into()
            }
            ChatMessage::Tool {
                message_content,
                tool_call_id,
            } => ChatCompletionRequestToolMessageBuilder::default()
                .content(message_content)
                .tool_call_id(tool_call_id)
                .build()?
                .into(),
            ChatMessage::Assistant {
                message_content,
                refusal,
                audio,
            } => match message_content {
                Some(message_content) => {
                    let assistant_content: AssistantContent = message_content.try_into()?;
                    ChatCompletionRequestAssistantMessageBuilder::default()
                        .content(assistant_content)
                        .build()?
                        .into()
                }
                None => todo!(),
            },
        })
    }
}

impl Into<Content> for MessageContent {
    fn into(self) -> Content {
        match self {
            MessageContent::Text(content) => content.into(),
            MessageContent::Array(items) => Content::Array(items),
        }
    }
}

impl TryInto<AssistantContent> for MessageContent {
    type Error = Error;
    fn try_into(self) -> Result<AssistantContent, Self::Error> {
        Ok(match self {
            MessageContent::Text(content) => AssistantContent::Text(content),
            MessageContent::Array(items) => todo!(),
        })
    }
}

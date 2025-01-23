use serde::{Deserialize, Serialize};

use crate::{
    types::{ChatChoice, ChatChoiceStream, CompletionUsage, CompletionUsageStream},
    Error, Printable,
};

use super::Respondable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResponse {
    /// A unique identifier for the chat completion.
    pub id: Option<String>,

    /// A list of chat completion choices. Can be more than one if n is greater than 1.
    pub choices: Vec<ChatChoice>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: Option<u32>,

    /// The model used for the chat completion.
    pub model: Option<String>,

    /// The service tier used for processing the request.
    pub service_tier: Option<String>,

    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub object: Option<String>,

    /// Usage statistics for the completion request.
    pub usage: Option<CompletionUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResponseStream {
    /// A unique identifier for the chat completion. Each chunk has the same ID.
    pub id: Option<String>,

    /// A list of chat completion choices. Can contain more than one elements if n is greater than 1. Can also be empty for the last chunk if you set stream_options: {"include_usage": true}.
    pub choices: Vec<ChatChoiceStream>,

    /// The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp.
    pub created: Option<u32>,

    /// The model to generate the completion.
    pub model: Option<String>,

    /// The service tier used for processing the request.
    pub service_tier: Option<String>,

    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion.chunk`.
    pub object: Option<String>,

    /// Usage statistics for the completion request.
    pub usage: Option<CompletionUsageStream>,
}

impl Respondable for ChatResponse {
    fn is_success(&self) -> bool {
        true
    }
}

impl Printable for ChatResponse {
    fn to_string_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

impl Respondable for ChatResponseStream {
    fn is_success(&self) -> bool {
        true
    }
}

impl Printable for ChatResponseStream {
    fn to_string_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

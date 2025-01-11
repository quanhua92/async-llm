use serde::{Deserialize, Serialize};

use crate::types::{ChatChoice, CompletionUsage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionResponse {
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

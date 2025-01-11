use serde::{Deserialize, Serialize};

use crate::types::{CompletionChoice, CompletionUsage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionResponse {
    /// A unique identifier for the completion.
    pub id: Option<String>,

    /// The list of completion choices the model generated for the input prompt.
    pub choices: Vec<CompletionChoice>,

    /// The Unix timestamp (in seconds) of when the completion was created.
    pub created: Option<u32>,

    /// The model used for completion.
    pub model: Option<String>,

    /// This fingerprint represents the backend configuration that the model runs with.
    ///
    /// Can be used in conjunction with the seed request parameter to understand when backend changes have been made that might impact determinism.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `text_completion`
    pub object: Option<String>,

    /// Usage statistics for the completion request.
    pub usage: Option<CompletionUsage>,
}

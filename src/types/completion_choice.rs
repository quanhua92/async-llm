use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionChoice {
    /// The reason the model stopped generating tokens. This will be stop if the model hit a natural stop point or a provided stop sequence, length if the maximum number of tokens specified in the request was reached, or content_filter if content was omitted due to a flag from our content filters.
    pub finish_reason: Option<String>,
    pub text: Option<String>,
    pub index: Option<u32>,
    pub logprobs: Option<CompletionLogprobs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionLogprobs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<Option<f32>>,
    pub top_logprobs: Vec<serde_json::Value>,
    pub text_offset: Vec<u32>,
}

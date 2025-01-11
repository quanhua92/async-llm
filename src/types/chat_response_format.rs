use serde::{Deserialize, Serialize};

/// An object specifying the format that the model must output.
///
/// Setting to { "type": "json_schema", "json_schema": {...} } enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the Structured Outputs guide.
///
/// Setting to { "type": "json_object" } enables JSON mode, which ensures the message the model generates is valid JSON.
///
/// Important: when using JSON mode, you must also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly "stuck" request. Also note that the message content may be partially cut off if finish_reason="length", which indicates the generation exceeded max_tokens or the conversation exceeded the max context length.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatResponseFormat {
    Text,
    JsonObject,
    JsonSchema {
        json_schema: ChatResponseFormatJsonSchema,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResponseFormatJsonSchema {
    /// A description of what the response format is for, used by the model to determine how to respond in the format.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    name: String,
    /// The schema for the response format, described as a JSON Schema object.
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<serde_json::Value>,
    /// Whether to enable strict schema adherence when generating the output. If set to true, the model will always follow the exact schema defined in the `schema` field. Only a subset of JSON Schema is supported when `strict` is `true`. To learn more, read the [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

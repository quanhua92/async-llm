use serde::{Deserialize, Serialize};

use super::AssistantFunctionCall;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssistantToolCall {
    /// The ID of the tool call.
    pub id: String,

    /// The type of the tool. Currently, only function is supported.
    pub r#type: ToolType,

    /// The function that the model called.
    pub function: AssistantFunctionCall,
}

#[derive(Clone, Serialize, Default, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    Function,
}

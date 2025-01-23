use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatFunctionCall {
    #[default]
    None,
    Auto,
    #[serde(untagged)]
    Function(ChatFunctionCallFunction),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ChatFunctionCallFunction {
    pub name: String,
}

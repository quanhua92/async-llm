use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ChatFunctionCall {
    #[default]
    None,
    Auto,
    Function(ChatFunctionCallFunction),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ChatFunctionCallFunction {
    pub name: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatToolChoice {
    #[default]
    None,
    Auto,
    Required,
    #[serde(untagged)]
    Function(ChatToolChoiceNamedOption),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ChatToolChoiceNamedOption {
    Function { function: ChatToolChoiceFunction },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ChatToolChoiceFunction {
    pub name: String,
}

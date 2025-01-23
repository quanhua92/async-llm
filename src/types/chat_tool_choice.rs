use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::Error;

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

#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize, PartialEq)]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = Error))]
pub struct ChatToolChoiceFunction {
    pub name: String,
}

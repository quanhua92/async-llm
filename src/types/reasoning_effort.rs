use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum ReasoningEffort {
    Low,
    #[default]
    Medium,
    High,
}

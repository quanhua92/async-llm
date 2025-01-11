use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AssistantContent {
    Text(String),
    Array(Vec<AssistantContentPart>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum AssistantContentPart {
    Text(String),
    Refusal { refusal: String },
}

impl Default for AssistantContent {
    fn default() -> Self {
        Self::Text("".into())
    }
}

impl From<&str> for AssistantContent {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<Vec<&str>> for AssistantContent {
    fn from(value: Vec<&str>) -> Self {
        Self::Array(
            value
                .into_iter()
                .map(|v| AssistantContentPart::Text(v.to_string()))
                .collect(),
        )
    }
}

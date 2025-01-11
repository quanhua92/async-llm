use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Array(Vec<String>),
}

impl Default for Content {
    fn default() -> Self {
        Self::Text("".into())
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<Vec<&str>> for Content {
    fn from(value: Vec<&str>) -> Self {
        Self::Array(value.iter().map(|v| v.to_string()).collect())
    }
}

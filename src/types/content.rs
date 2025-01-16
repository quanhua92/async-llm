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

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<Vec<&str>> for Content {
    fn from(value: Vec<&str>) -> Self {
        Self::Array(value.iter().map(|v| v.to_string()).collect())
    }
}

impl From<Vec<String>> for Content {
    fn from(array: Vec<String>) -> Self {
        Self::Array(array)
    }
}

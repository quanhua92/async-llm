use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Stop {
    String(String),
    StringArray(Vec<String>), // minItems: 1; maxItems: 4
}

impl Default for Stop {
    fn default() -> Self {
        Self::String("".into())
    }
}

impl From<&str> for Stop {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<Vec<&str>> for Stop {
    fn from(value: Vec<&str>) -> Self {
        Self::StringArray(value.iter().map(|v| v.to_string()).collect())
    }
}

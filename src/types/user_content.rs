use serde::{Deserialize, Serialize};

use super::{ImageUrl, InputAudio};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UserContent {
    Text(String),
    Array(Vec<UserContentPart>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UserContentPart {
    Text { text: String },
    Image { image_url: ImageUrl },
    Audio { input_audio: InputAudio },
}

impl Default for UserContent {
    fn default() -> Self {
        Self::Text("".into())
    }
}

impl From<&str> for UserContent {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<Vec<&str>> for UserContent {
    fn from(value: Vec<&str>) -> Self {
        Self::Array(
            value
                .into_iter()
                .map(|v| UserContentPart::Text {
                    text: v.to_string(),
                })
                .collect(),
        )
    }
}

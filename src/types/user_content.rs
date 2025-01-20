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
    ImageUrl { image_url: ImageUrl },
    Audio { input_audio: InputAudio },
}

impl UserContentPart {
    pub fn image(image_url: impl Into<ImageUrl>) -> Self {
        Self::ImageUrl {
            image_url: image_url.into(),
        }
    }

    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }
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

impl From<&str> for UserContentPart {
    fn from(value: &str) -> Self {
        Self::text(value)
    }
}

impl From<String> for UserContentPart {
    fn from(value: String) -> Self {
        Self::text(value)
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

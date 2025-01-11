use serde::{Deserialize, Serialize};

/// Static predicted output content, such as the content of a text file that is being regenerated.
/// The type of the predicted content you want to provide. This type is currently always content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[serde(content = "content")]
pub enum PredictionContent {
    Content(PredictionContentContent),
}

/// The content that should be matched when generating a model response. If generated tokens would match this content, the entire model response can be returned much more quickly.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum PredictionContentContent {
    /// The content used for a Predicted Output. This is often the text of a file you are regenerating with minor changes.
    Text(String),
    /// An array of content parts with a defined type. Supported options differ based on the model being used to generate the response. Can contain text inputs.
    Array(Vec<PredictionContentPart>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PredictionContentPart {
    Text { text: String },
}

impl Default for PredictionContent {
    fn default() -> Self {
        Self::Content("".into())
    }
}

impl From<&str> for PredictionContent {
    fn from(value: &str) -> Self {
        Self::Content(value.into())
    }
}
impl From<&str> for PredictionContentContent {
    fn from(value: &str) -> Self {
        Self::Text(value.into())
    }
}

impl From<Vec<&str>> for PredictionContent {
    fn from(value: Vec<&str>) -> Self {
        Self::Content(value.into())
    }
}

impl From<Vec<&str>> for PredictionContentContent {
    fn from(value: Vec<&str>) -> Self {
        Self::Array(
            value
                .into_iter()
                .map(|v| PredictionContentPart::Text {
                    text: v.to_string(),
                })
                .collect(),
        )
    }
}

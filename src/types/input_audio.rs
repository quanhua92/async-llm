use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InputAudioFormat {
    Wav,
    #[default]
    Mp3,
}

#[derive(Debug, Clone, Builder, Default, Serialize, Deserialize, PartialEq)]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = Error))]
pub struct InputAudio {
    /// Base64 encoded audio data.
    pub data: String,
    /// The format of the encoded audio data. Currently supports "wav" and "mp3".
    pub format: InputAudioFormat,
}

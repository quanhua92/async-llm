use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssistantAudio {
    /// Unique identifier for a previous audio response from the model.
    pub id: String,
}

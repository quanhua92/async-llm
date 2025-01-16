#[derive(Debug, thiserror::Error)]
pub enum Error {
    // -- Argument
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    // -- Auth
    #[error("missing api key error")]
    MissingApiKey,

    // -- Config
    #[error("invalid config: {0}")]
    InvalidConfig(String),

    // -- Execution
    #[error("http client error: {0}")]
    HttpClient(String),

    #[error("stream error: {0}")]
    Stream(String),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<derive_builder::UninitializedFieldError> for Error {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        Self::InvalidArgument(value.to_string())
    }
}

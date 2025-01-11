#[derive(Debug, Clone, thiserror::Error)]
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
}

impl From<derive_builder::UninitializedFieldError> for Error {
    fn from(value: derive_builder::UninitializedFieldError) -> Self {
        Self::InvalidArgument(value.to_string())
    }
}

use std::{fmt::Debug, pin::Pin};

use async_trait::async_trait;
use futures::Stream;

use crate::{
    chat::{ChatCompletionRequest, ChatCompletionResponse, ChatCompletionResponseStream},
    completions::{CompletionRequest, CompletionResponse},
    error::Error,
    http::HttpClient,
};

pub mod config;
pub mod openai;

pub use config::{Config, OpenAIConfig};

#[async_trait]
pub trait Provider: Debug + Send + Sync {
    type Config: Config;

    fn config(&self) -> &Self::Config;

    async fn chat(
        &self,
        client: &impl HttpClient,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Error>;

    async fn chat_stream(
        &self,
        client: &impl HttpClient,
        request: ChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ChatCompletionResponseStream, Error>> + Send>>,
        Error,
    >;

    async fn completions(
        &self,
        client: &impl HttpClient,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, Error>;
}

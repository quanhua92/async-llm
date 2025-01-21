use std::{fmt::Debug, pin::Pin};

use async_trait::async_trait;
use futures::Stream;

use crate::{
    completions::{CompletionRequest, CompletionResponse},
    error::Error,
    http::HttpClient,
    ChatRequest, ChatResponse, ChatResponseStream,
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
        request: ChatRequest,
    ) -> Result<ChatResponse, Error>;

    async fn chat_stream(
        &self,
        client: &impl HttpClient,
        request: ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatResponseStream, Error>> + Send>>, Error>;

    async fn completions(
        &self,
        client: &impl HttpClient,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, Error>;
}

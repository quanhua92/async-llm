use std::{fmt::Debug, pin::Pin};

use async_trait::async_trait;
use futures::Stream;

use crate::{
    completions::{CompletionRequest, CompletionResponse},
    error::Error,
    http::HttpClient,
    request::Requestable,
    response::Respondable,
};

pub mod config;
pub mod json;
pub mod openai;

pub use config::{Config, OpenAIConfig};

#[async_trait]
pub trait Provider: Debug + Send + Sync {
    type Config: Config;
    type ChatRequest: Requestable;
    type ChatResponse: Respondable;
    type ChatResponseStream: Respondable;

    fn config(&self) -> &Self::Config;

    async fn chat(
        &self,
        client: &impl HttpClient,
        request: Self::ChatRequest,
    ) -> Result<Self::ChatResponse, Error>;

    async fn chat_stream(
        &self,
        client: &impl HttpClient,
        request: Self::ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::ChatResponseStream, Error>> + Send>>, Error>;

    async fn completions(
        &self,
        client: &impl HttpClient,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, Error>;
}

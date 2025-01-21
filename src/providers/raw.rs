use std::pin::Pin;

use async_trait::async_trait;
use futures::Stream;

use crate::{
    completions::{CompletionRequest, CompletionResponse},
    error::Error,
    http::HttpClient,
};

use super::{config::OpenAIConfig, Provider};

#[derive(Debug, Clone, Default)]
pub struct RawProvider {
    pub(crate) config: OpenAIConfig,
}

impl RawProvider {
    pub fn new(config: OpenAIConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Provider for RawProvider {
    type Config = OpenAIConfig;
    type ChatRequest = serde_json::Value;
    type ChatResponse = serde_json::Value;
    type ChatResponseStream = serde_json::Value;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn chat(
        &self,
        client: &impl HttpClient,
        request: Self::ChatRequest,
    ) -> Result<Self::ChatResponse, Error> {
        client.post("/chat/completions", request).await
    }
    async fn chat_stream(
        &self,
        client: &impl HttpClient,
        request: Self::ChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::ChatResponseStream, Error>> + Send>>, Error>
    {
        client.post_stream("/chat/completions", request).await
    }

    async fn completions(
        &self,
        client: &impl HttpClient,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, Error> {
        client.post("/completions", request).await
    }
}

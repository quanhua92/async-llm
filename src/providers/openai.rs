use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::{
    completions::{CompletionRequest, CompletionResponse},
    error::Error,
    http::HttpClient,
    ChatRequest, ChatResponse, ChatResponseStream,
};

use super::{config::OpenAIConfig, Provider};

pub const OPENAI_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone, Default)]
pub struct OpenAIProvider {
    pub(crate) config: OpenAIConfig,
}

impl OpenAIProvider {
    pub fn new(config: OpenAIConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    type Config = OpenAIConfig;
    type ChatRequest = ChatRequest;
    type ChatResponse = ChatResponse;
    type ChatResponseStream = ChatResponseStream;

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

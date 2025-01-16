use std::pin::Pin;

use async_trait::async_trait;
use derive_builder::Builder;
use futures::Stream;

use crate::{
    chat::{ChatCompletionRequest, ChatCompletionResponse, ChatCompletionResponseStream},
    completions::{CompletionRequest, CompletionResponse},
    error::Error,
    http::HttpClient,
};

use super::{config::OpenAIConfig, Provider};

pub const OPENAI_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Clone, Builder, Default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = Error))]
pub struct OpenAIProvider {
    pub(crate) config: OpenAIConfig,
}

#[async_trait]
impl Provider for OpenAIProvider {
    type Config = OpenAIConfig;
    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn chat(
        &self,
        client: &impl HttpClient,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Error> {
        client.post("/chat/completions", request).await
    }
    async fn chat_stream(
        &self,
        client: &impl HttpClient,
        request: ChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ChatCompletionResponseStream, Error>> + Send>>,
        Error,
    > {
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

use crate::{
    chat::Chat,
    completions::Completions,
    http::{HttpClient, SimpleHttpClient},
    providers::{openai::OpenAIProvider, OpenAIConfig, Provider},
};

#[derive(Debug, Clone)]
pub struct Client<P: Provider, H: HttpClient> {
    pub(crate) provider: P,
    pub(crate) http_client: H,
}

impl<P: Provider, H: HttpClient> Client<P, H> {
    pub fn new_with_args(provider: P, http_client: H) -> Self {
        Client {
            provider,
            http_client,
        }
    }
}

impl Client<OpenAIProvider, SimpleHttpClient<OpenAIConfig>> {
    pub fn new() -> Self {
        let provider = OpenAIProvider::default();
        let config = provider.config().clone();
        Self {
            provider,
            http_client: SimpleHttpClient::new(config),
        }
    }
}

impl<P: Provider, H: HttpClient> Client<P, H> {
    pub fn completions(&self) -> Completions<P, H> {
        Completions::new(self)
    }
    pub fn chat(&self) -> Chat<P, H> {
        Chat::new(self)
    }
}

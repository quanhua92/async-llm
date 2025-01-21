use secrecy::SecretString;

use crate::{
    chat::Chat,
    completions::Completions,
    http::{HttpClient, SimpleHttpClient},
    providers::{openai::OpenAIProvider, OpenAIConfig, Provider},
    RawProvider,
};

pub type DefaultHttpClient<C> = SimpleHttpClient<C>;

#[derive(Debug, Clone)]
pub struct Client<P: Provider, H: HttpClient> {
    pub(crate) provider: P,
    pub(crate) http_client: H,
}

impl<P: Provider> Client<P, DefaultHttpClient<P::Config>> {
    pub fn with_provider(provider: P) -> Self {
        let config = provider.config().clone();
        Self {
            provider,
            http_client: DefaultHttpClient::new(config),
        }
    }
}

impl<P: Provider, H: HttpClient> Client<P, H> {
    pub fn with_args(provider: P, http_client: H) -> Self {
        Client {
            provider,
            http_client,
        }
    }
}

impl Client<OpenAIProvider, DefaultHttpClient<OpenAIConfig>> {
    pub fn new() -> Self {
        let provider = OpenAIProvider::default();
        let config = provider.config().clone();
        Self {
            provider,
            http_client: DefaultHttpClient::new(config),
        }
    }

    pub fn with_auth(base_url: impl Into<String>, api_key: Option<SecretString>) -> Self {
        let config = OpenAIConfig::new(base_url, api_key);
        let provider = OpenAIProvider::new(config.clone());
        Self {
            provider,
            http_client: DefaultHttpClient::new(config),
        }
    }
}

impl Client<RawProvider, DefaultHttpClient<OpenAIConfig>> {
    pub fn raw() -> Self {
        let provider = RawProvider::default();
        let config = provider.config().clone();
        Self {
            provider,
            http_client: DefaultHttpClient::new(config),
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

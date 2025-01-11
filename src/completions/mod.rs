use crate::{error::Error, http::HttpClient, Client, Provider};

pub mod request;
pub mod response;

pub use request::*;
pub use response::*;

#[derive(Debug, Clone)]
pub struct Completions<'c, P: Provider, H: HttpClient> {
    pub(crate) client: &'c Client<P, H>,
}

impl<'c, P: Provider, H: HttpClient> Completions<'c, P, H> {
    pub fn new(client: &'c Client<P, H>) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: CompletionRequest) -> Result<CompletionResponse, Error> {
        self.client
            .provider
            .completions(&self.client.http_client, request)
            .await
    }
}

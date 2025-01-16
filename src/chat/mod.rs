use std::pin::Pin;

use crate::{error::Error, http::HttpClient, Client, Provider};

pub mod request;
pub mod response;

use futures::Stream;
pub use request::*;
pub use response::*;

#[derive(Debug, Clone)]
pub struct Chat<'c, P: Provider, H: HttpClient> {
    pub(crate) client: &'c Client<P, H>,
}

impl<'c, P: Provider, H: HttpClient> Chat<'c, P, H> {
    pub fn new(client: &'c Client<P, H>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, Error> {
        let stream = request.stream.unwrap_or(false);
        match stream {
            true => Err(Error::InvalidArgument(
                "When stream is true, use the client.create_stream function instead".into(),
            )),
            false => {
                self.client
                    .provider
                    .chat(&self.client.http_client, request)
                    .await
            }
        }
    }
    pub async fn create_stream(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ChatCompletionResponseStream, Error>> + Send>>,
        Error,
    > {
        let stream = request.stream.unwrap_or(false);
        match stream {
            false => Err(Error::InvalidArgument(
                "When stream is false, use the client.create function instead".into(),
            )),
            true => {
                self.client
                    .provider
                    .chat_stream(&self.client.http_client, request)
                    .await
            }
        }
    }
}

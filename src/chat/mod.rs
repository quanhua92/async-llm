use std::fmt::Debug;
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

    pub async fn create<T>(&self, request: T) -> Result<ChatCompletionResponse, Error>
    where
        T: TryInto<ChatCompletionRequest>,
        T::Error: Debug,
    {
        let request: ChatCompletionRequest = request.try_into().map_err(|e| {
            Error::InvalidArgument(format!(
                "Failed to convert to ChatCompletionRequest. Error = {e:?}"
            ))
        })?;
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
    pub async fn create_stream<T>(
        &self,
        request: T,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ChatCompletionResponseStream, Error>> + Send>>,
        Error,
    >
    where
        T: TryInto<ChatCompletionRequest>,
        T::Error: Debug,
    {
        let request: ChatCompletionRequest = request.try_into().map_err(|e| {
            Error::InvalidArgument(format!(
                "Failed to convert to ChatCompletionRequest. Error = {e:?}"
            ))
        })?;
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

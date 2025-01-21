use std::fmt::Debug;
use std::pin::Pin;

use crate::{
    error::Error, http::HttpClient, response::chat::ChatResponseStream, ChatRequest, ChatResponse,
    Client, Provider,
};

use futures::Stream;

#[derive(Debug, Clone)]
pub struct Chat<'c, P: Provider, H: HttpClient> {
    pub(crate) client: &'c Client<P, H>,
}

impl<'c, P: Provider, H: HttpClient> Chat<'c, P, H> {
    pub fn new(client: &'c Client<P, H>) -> Self {
        Self { client }
    }

    pub async fn create<T>(&self, request: T) -> Result<ChatResponse, Error>
    where
        T: TryInto<ChatRequest>,
        T::Error: Debug,
    {
        let request: ChatRequest = request.try_into().map_err(|e| {
            Error::InvalidArgument(format!("Failed to convert to ChatRequest. Error = {e:?}"))
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
    ) -> Result<Pin<Box<dyn Stream<Item = Result<ChatResponseStream, Error>> + Send>>, Error>
    where
        T: TryInto<ChatRequest>,
        T::Error: Debug,
    {
        let request: ChatRequest = request.try_into().map_err(|e| {
            Error::InvalidArgument(format!("Failed to convert to ChatRequest. Error = {e:?}"))
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

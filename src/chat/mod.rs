use std::fmt::Debug;
use std::pin::Pin;

use crate::{error::Error, http::HttpClient, request::Requestable, Client, Provider};

use futures::Stream;

#[derive(Debug, Clone)]
pub struct Chat<'c, P: Provider, H: HttpClient> {
    pub(crate) client: &'c Client<P, H>,
}

impl<'c, P, H> Chat<'c, P, H>
where
    P: Provider,
    H: HttpClient,
{
    pub fn new(client: &'c Client<P, H>) -> Self {
        Self { client }
    }

    pub async fn create<T>(&self, request: T) -> Result<P::ChatResponse, Error>
    where
        T: TryInto<P::ChatRequest>,
        T::Error: Debug,
    {
        let request: P::ChatRequest = request.try_into().map_err(|e| {
            Error::InvalidArgument(format!("Failed to convert to ChatRequest. Error = {e:?}"))
        })?;
        let stream = request.stream();
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
    ) -> Result<Pin<Box<dyn Stream<Item = Result<P::ChatResponseStream, Error>> + Send>>, Error>
    where
        T: TryInto<P::ChatRequest>,
        T::Error: Debug,
    {
        let request: P::ChatRequest = request.try_into().map_err(|e| {
            Error::InvalidArgument(format!("Failed to convert to ChatRequest. Error = {e:?}"))
        })?;
        let stream = request.stream();
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

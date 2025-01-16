use futures::Stream;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, pin::Pin};

use crate::error::Error;

pub mod simple;
pub mod stream;
pub use simple::SimpleHttpClient;

#[async_trait::async_trait]
pub trait HttpClient: Debug + Clone + Send + Sync {
    async fn post<I: Serialize + Send, O: DeserializeOwned>(
        &self,
        path: &str,
        request: I,
    ) -> Result<O, Error>;
    async fn post_stream<I: Serialize + Send, O: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        request: I,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<O, Error>> + Send>>, Error>;
}

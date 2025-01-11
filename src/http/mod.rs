use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

pub mod simple;
pub use simple::SimpleHttpClient;

#[async_trait::async_trait]
pub trait HttpClient: Debug + Clone + Send + Sync {
    async fn post<I: Serialize + Send, O: DeserializeOwned>(
        &self,
        path: &str,
        request: I,
    ) -> Result<O, Error>;
}

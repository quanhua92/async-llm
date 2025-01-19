use std::pin::Pin;

use futures::Stream;
use reqwest_eventsource::RequestBuilderExt;
use serde::{de::DeserializeOwned, Serialize};

use crate::{error::Error, providers::Config};

use super::{stream::stream, HttpClient};

#[derive(Debug, Clone)]
pub struct SimpleHttpClient<C: Config> {
    pub(crate) client: reqwest::Client,
    pub(crate) config: C,
}

#[async_trait::async_trait]
impl<C: Config> HttpClient for SimpleHttpClient<C> {
    async fn post<I: Serialize + Send, O: DeserializeOwned>(
        &self,
        path: &str,
        request: I,
    ) -> Result<O, Error> {
        let url = self.config.url(path);
        let headers = self.config.headers()?;
        let query = self.config.query();
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .query(&query)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                Error::HttpClient(format!(
                    "Failed to send HTTP request. Error = {}",
                    e.to_string()
                ))
            })?;
        let status_code = resp.status();
        if status_code.is_success() {
            let value: serde_json::Value = resp.json().await.map_err(|e| {
                Error::HttpClient(format!(
                    "Failed to read JSON from HTTP request. Error = {}",
                    e.to_string()
                ))
            })?;
            if let Some(_) = value.get("choices") {
                return Ok(serde_json::from_value(value.clone()).map_err(|e| {
                    tracing::debug!("value =\n {value:?}");
                    e
                })?);
            }
            if let Some(error) = value.get("error") {
                return Err(Error::HttpClient(format!(
                    "Failed to process HTTP request. Error = {error:?}",
                )));
            }
        }
        Err(Error::HttpClient(format!(
            "Failed to process HTTP request. Status Code = {status_code:?}",
        )))
    }

    async fn post_stream<I: Serialize + Send, O: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        request: I,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<O, Error>> + Send>>, Error> {
        let url = self.config.url(path);
        let headers = self.config.headers().unwrap();
        let query = self.config.query();
        let event_source = self
            .client
            .post(url)
            .headers(headers)
            .query(&query)
            .json(&request)
            .eventsource()
            .map_err(|e| {
                Error::HttpClient(format!(
                    "Failed to send HTTP request. Error = {}",
                    e.to_string()
                ))
            })?;
        stream(event_source, self.config.stream_done_message()).await
    }
}

impl<C: Config> SimpleHttpClient<C> {
    pub fn new(config: C) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }
}

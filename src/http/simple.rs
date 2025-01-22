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
            .post(&url)
            .headers(headers)
            .query(&query)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                Error::HttpClient(format!(
                    "Failed to send HTTP request. Error = {}, url = {url:?}",
                    e
                ))
            })?;
        let status_code = resp.status();
        if status_code.is_success() {
            let value: serde_json::Value = resp.json().await.map_err(|e| {
                Error::HttpClient(format!(
                    "Failed to read JSON from HTTP request. Error = {}, url = {url}",
                    e
                ))
            })?;
            if value.get("choices").is_some() {
                return Ok(serde_json::from_value(value.clone())?);
            }
            if let Some(error) = value.get("error") {
                return Err(Error::HttpClient(format!(
                    "Failed to process HTTP request. Error = {error:?}, url = {url}",
                )));
            } else {
                return Err(Error::HttpClient(format!(
                    "Failed to process HTTP request. url = {url:?}",
                )));
            }
        } else {
            let body = resp.text().await.map_err(|e| {
                Error::HttpClient(format!(
                    "Failed to read text from HTTP request. Error = {}, url = {url}",
                    e
                ))
            })?;
            Err(Error::HttpClient(format!(
            "Failed to process HTTP request. Status Code = {status_code:?}, url = {url} - body = {body}",
        )))
        }
    }

    async fn post_stream<I: Serialize + Send, O: DeserializeOwned + Send + 'static>(
        &self,
        path: &str,
        request: I,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<O, Error>> + Send>>, Error> {
        let url = self.config.url(path);
        let headers = self.config.headers()?;
        let query = self.config.query();
        let event_source = self
            .client
            .post(url)
            .headers(headers)
            .query(&query)
            .json(&request)
            .eventsource()
            .map_err(|e| {
                Error::HttpClient(format!("Failed to send HTTP request. Error = {}", e))
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

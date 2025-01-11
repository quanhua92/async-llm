use serde::{de::DeserializeOwned, Serialize};

use crate::{error::Error, providers::Config};

use super::HttpClient;

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
        let headers = self.config.headers().unwrap();
        let query = self.config.query();
        let resp = self
            .client
            .post(url)
            .headers(headers)
            .query(&query)
            .json(&request)
            .send()
            .await
            .unwrap();
        let value: serde_json::Value = resp.json().await.unwrap();
        Ok(serde_json::from_value(value).unwrap())
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

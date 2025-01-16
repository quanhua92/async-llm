use derive_builder::Builder;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use secrecy::{ExposeSecret, SecretString};
use std::fmt::Debug;

use crate::error::Error;

use super::openai::OPENAI_BASE_URL;

pub const OPENAI_ORGANIZATION: &str = "OpenAI-Organization";
pub const OPENAI_PROJECT: &str = "OpenAI-Project";
pub const OPENAI_BETA: &str = "OpenAI-Beta";

pub trait Config: Debug + Clone + Send + Sync {
    fn headers(&self) -> Result<HeaderMap, Error>;
    fn url(&self, path: &str) -> String;
    fn query(&self) -> Vec<(&str, &str)>;

    fn base_url(&self) -> &str;

    fn api_key(&self) -> Option<&SecretString>;

    fn stream_done_message(&self) -> &'static str {
        "[DONE]"
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(derive(Debug))]
#[builder(build_fn(error = Error))]
pub struct OpenAIConfig {
    pub(crate) base_url: String,
    pub(crate) api_key: Option<SecretString>,
    pub(crate) org_id: Option<String>,
    pub(crate) project_id: Option<String>,
    pub(crate) beta: Option<String>,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| OPENAI_BASE_URL.to_string()),
            api_key: std::env::var("OPENAI_API_KEY").map(|v| v.into()).ok(),
            org_id: Default::default(),
            project_id: Default::default(),
            beta: Some("assistants=v2".into()),
        }
    }
}

impl Config for OpenAIConfig {
    fn headers(&self) -> Result<reqwest::header::HeaderMap, Error> {
        let mut headers = HeaderMap::new();

        if let Some(api_key) = &self.api_key {
            let bearer = format!("Bearer {}", api_key.expose_secret());
            headers.insert(
                AUTHORIZATION,
                bearer.parse().map_err(|e| {
                    Error::InvalidConfig(format!(
                        "Failed to convert api key id to header value. {:?}",
                        e
                    ))
                })?,
            );
        }

        if let Some(org_id) = &self.org_id {
            headers.insert(
                OPENAI_ORGANIZATION,
                org_id.parse().map_err(|e| {
                    Error::InvalidConfig(format!(
                        "Failed to convert organization id to header value. {:?}",
                        e
                    ))
                })?,
            );
        }
        if let Some(project_id) = &self.project_id {
            headers.insert(
                OPENAI_PROJECT,
                project_id.parse().map_err(|e| {
                    Error::InvalidConfig(format!(
                        "Failed to convert project id to header value. {:?}",
                        e
                    ))
                })?,
            );
        }

        // See: https://github.com/64bit/async-openai/blob/bd7a87e335630d5d2f3e6cef30d15633048937b3/async-openai/src/config.rs#L111
        if let Some(beta) = &self.beta {
            headers.insert(
                OPENAI_BETA,
                beta.parse().map_err(|e| {
                    Error::InvalidConfig(format!("Failed to convert beta to heaer. {:?}", e))
                })?,
            );
        }
        Ok(headers)
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn api_key(&self) -> Option<&SecretString> {
        self.api_key.as_ref()
    }
}

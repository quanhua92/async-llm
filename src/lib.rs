pub mod chat;
pub mod client;
pub mod completions;
pub mod error;
pub mod http;
pub mod providers;
pub mod request;
pub mod response;
pub mod types;

pub use client::Client;
pub use error::Error;
pub use providers::{OpenAIProvider, Provider, RawProvider};
pub use request::{ChatMessage, ChatRequest};
pub use response::{ChatResponse, ChatResponseStream};
use serde::Serialize;

pub trait Printable: Serialize {
    fn to_string_pretty(&self) -> Result<String, Error>;
}

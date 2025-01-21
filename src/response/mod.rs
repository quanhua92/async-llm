pub mod chat;

pub use chat::{ChatResponse, ChatResponseStream};

use crate::{Error, Printable};

pub trait Respondable {
    fn is_success(&self) -> bool {
        true
    }
}

impl Respondable for serde_json::Value {}

impl Printable for serde_json::Value {
    fn to_string_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

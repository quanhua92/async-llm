pub mod chat;

pub use chat::{ChatResponse, ChatResponseStream};

pub trait Respondable {
    fn is_success(&self) -> bool {
        true
    }
}

impl Respondable for serde_json::Value {}

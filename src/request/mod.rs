pub mod chat;
pub mod message;

pub use chat::ChatRequest;
pub use message::ChatMessage;

pub trait Requestable {
    fn stream(&self) -> bool;
}

impl Requestable for serde_json::Value {
    fn stream(&self) -> bool {
        match self.get("stream") {
            Some(serde_json::Value::Bool(v)) => v.to_owned(),
            _ => false,
        }
    }
}

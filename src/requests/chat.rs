use crate::{
    chat::{
        ChatCompletionRequest, ChatCompletionRequestBuilder, ChatCompletionRequestMessage,
        ChatCompletionResponse,
    },
    error::Error,
    types::{ChatResponseFormat, ChatTool},
    Client,
};

use super::{message::MessageContent, ChatMessage};

/// ChatRequest is a wrapper struct to simplify the API for ChatCompletionRequest
#[derive(Debug, Clone, Default)]
pub struct ChatRequest {
    /// Builder
    pub(crate) builder: ChatCompletionRequestBuilder,

    /// Messages
    pub(crate) messages: Vec<ChatMessage>,
}

impl TryInto<ChatCompletionRequest> for ChatRequest {
    type Error = Error;
    fn try_into(self) -> Result<ChatCompletionRequest, Self::Error> {
        self.build()
    }
}

impl ChatRequest {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        let builder = ChatCompletionRequest::builder().model(model).to_owned();
        Self { builder, messages }
    }

    pub fn from_model(model: impl Into<String>) -> Self {
        Self::new(model, vec![])
    }

    pub fn from_system(message: impl Into<MessageContent>) -> Self {
        Self {
            builder: ChatCompletionRequest::builder(),
            messages: vec![ChatMessage::System(message.into())],
        }
    }

    pub fn iter_messages(&self) -> impl Iterator<Item = &ChatMessage> {
        self.messages.iter()
    }

    pub fn as_inner_mut(&mut self) -> &mut ChatCompletionRequestBuilder {
        &mut self.builder
    }

    pub fn build(mut self) -> Result<ChatCompletionRequest, Error> {
        let messages: Result<Vec<ChatCompletionRequestMessage>, Error> =
            self.messages.into_iter().map(|i| i.try_into()).collect();
        self.builder.messages(messages?).build()
    }

    pub async fn send(self) -> Result<ChatCompletionResponse, Error> {
        Ok(Client::new().chat().create(self).await?)
    }
}

/// Chainable setters
impl ChatRequest {
    pub fn system(mut self, message: impl Into<MessageContent>) -> Self {
        self.messages.push(ChatMessage::system(message));
        self
    }

    pub fn user(mut self, message: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::user(message));
        self
    }

    pub fn developer(mut self, message: impl Into<MessageContent>) -> Self {
        self.messages.push(ChatMessage::developer(message));
        self
    }

    pub fn assistant(mut self, message: impl Into<MessageContent>) -> Self {
        self.messages.push(ChatMessage::assistant(message));
        self
    }

    pub fn tool(
        mut self,
        message: impl Into<MessageContent>,
        tool_call_id: impl Into<String>,
    ) -> Self {
        self.messages.push(ChatMessage::tool(message, tool_call_id));
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.builder.model(model);
        self
    }

    pub fn stream(mut self) -> Self {
        self.builder.stream(true);
        self
    }

    pub fn tools(mut self, tools: Vec<impl Into<ChatTool>>) -> Self {
        self.builder
            .tools(tools.into_iter().map(Into::into).collect::<Vec<ChatTool>>());
        self
    }

    pub fn response_format(mut self, response_format: impl Into<ChatResponseFormat>) -> Self {
        self.builder.response_format(response_format.into());
        self
    }
}

impl ChatRequest {
    /// This method clones the ChatRequest, builds the ChatCompletionRequest and returns a pretty string of that request
    pub fn to_string_pretty(&self) -> Result<String, Error> {
        let request: ChatCompletionRequest = self.clone().try_into()?;
        Ok(serde_json::to_string_pretty(&request)?)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn chat_request_works() {
        let request = ChatRequest::new(
            "gpt-4o-mini",
            vec![
                ChatMessage::system("You are a helpful assistant"),
                ChatMessage::user("Who are you?"),
            ],
        )
        .user("1 + 1 =");

        let completion = request.build();
        assert!(completion.is_ok());
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatChoice {
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
    pub finish_reason: Option<String>,

    /// The index of the choice in the list of choices.
    pub index: Option<u32>,

    /// A chat completion message generated by the model.
    pub message: Option<ChatChoiceMessage>,

    /// Log probability information for the choice.
    pub logprobs: Option<ChatLogprobs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatChoiceStream {
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.
    pub finish_reason: Option<String>,

    /// The index of the choice in the list of choices.
    pub index: Option<u32>,

    /// A chat completion delta generated by streamed model responses.
    pub delta: Option<ChatChoiceMessageStream>,

    /// Log probability information for the choice.
    pub logprobs: Option<ChatLogprobs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatLogprobs {
    /// A list of message content tokens with log probability information.
    pub content: Option<Vec<ChatLogprobsMessage>>,

    /// A list of message refusal tokens with log probability information.
    pub refusal: Option<Vec<ChatLogprobsMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatLogprobsMessage {
    /// The token
    pub token: Option<String>,
    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: Option<f32>,
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be null if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
    /// List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested top_logprobs returned.
    pub top_logprobs: Option<Vec<ChatLogprobsLogProb>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatLogprobsLogProb {
    /// The token
    pub token: Option<String>,
    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: Option<f32>,
    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be null if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatChoiceMessage {
    /// The contents of the message.
    pub content: Option<String>,

    /// The refusal message generated by the model.
    pub refusal: Option<String>,

    /// The tool calls generated by the model, such as function calls.
    pub tool_calls: Option<Vec<ChatMessageToolCall>>,

    /// The role of the author of this message.
    pub role: Option<String>,

    /// Deprecated and replaced by tool_calls. The name and arguments of a function that should be called, as generated by the model.
    pub function_call: Option<ChatMessageFunctionCall>,

    /// If the audio output modality is requested, this object contains data about the audio response from the model. [Learn more](https://platform.openai.com/docs/guides/audio).
    pub audio: Option<ChatMessageAudio>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatChoiceMessageStream {
    /// The contents of the message.
    pub content: Option<String>,

    /// The refusal message generated by the model.
    pub refusal: Option<String>,

    /// The tool calls generated by the model, such as function calls.
    pub tool_calls: Option<Vec<ChatMessageToolCall>>,

    /// The role of the author of this message.
    pub role: Option<String>,

    /// Deprecated and replaced by tool_calls. The name and arguments of a function that should be called, as generated by the model.
    pub function_call: Option<ChatMessageFunctionCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessageToolCall {
    /// The ID of the tool call.
    pub id: Option<String>,
    /// The type of the tool. Currently, only function is supported.
    pub r#type: Option<String>,

    /// The function that the model called.
    pub function: Option<ChatMessageFunctionCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessageFunctionCall {
    /// The name of the function to call.
    pub name: Option<String>,

    /// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
    pub arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessageAudio {
    /// Unique identifier for this audio response.
    pub id: Option<String>,

    /// The Unix timestamp (in seconds) for when this audio response will no longer be accessible on the server for use in multi-turn conversations.
    pub expires_at: Option<u32>,

    /// Base64 encoded audio bytes generated by the model, in the format specified in the request.
    pub data: Option<String>,

    /// Transcript of the audio generated by the model.
    pub transcript: Option<String>,
}

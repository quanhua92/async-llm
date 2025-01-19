use async_llm::{
    init_tracing,
    types::{ChatResponseFormat, JsonSchema},
    ChatMessage, ChatRequest, Error,
};
use serde_json::json;

async fn example_basic() -> Result<(), Error> {
    let request = ChatRequest::new(
        "meta-llama/llama-3.2-3b-instruct:free",
        vec![ChatMessage::system("You are a helpful assistant")],
    )
    .user("1 + 1 =");
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

async fn example_assistant_prefill() -> Result<(), Error> {
    let request = ChatRequest::new(
        "mistralai/mistral-7b-instruct:free",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user("Who are you?"),
            ChatMessage::assistant("I'm not sure, but my best guess is"),
        ],
    );
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

async fn example_structured_outputs_json_object() -> Result<(), Error> {
    let request = ChatRequest::new(
        "mistralai/ministral-8b",
        // "openai/gpt-4o-mini",
        // "google/gemini-flash-1.5-8b", // error
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user(
                r#"What's the weather like in Vietnam? Reply in json as following:
                {
                    "temperature": "Temperature in Celsius",
                    "location": "City or location name"
                }"#,
            ),
        ],
    )
    .response_format(ChatResponseFormat::JsonObject);
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

async fn example_structured_outputs_json_schema() -> Result<(), Error> {
    let request = ChatRequest::new(
        "mistralai/ministral-8b",
        // "openai/gpt-4o-mini",
        // "google/gemini-flash-1.5-8b", // error
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user(r#"What's the weather like in Vietnam?"#),
        ],
    )
    .response_format(JsonSchema::new("weather").strict(true).schema(json!({
      "type": "object",
      "properties": {
          "location": {
              "type": "string",
              "description": "City or location name"
          },
          "temperature": {
              "type": "number",
              "description": "Temperature in Celsius"
          },
          "conditions": {
              "type": "string",
              "description": "Weather conditions description"
          }
      },
      "required": ["location", "temperature", "conditions"],
      "additionalProperties": false
    })));
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    init_tracing();

    // https://openrouter.ai/docs/requests
    // example_basic().await?;

    // Assitant Prefill
    // example_assistant_prefill().await?;

    // TODO: Images & Multimodel: image_url
    // TODO: Images & Multimodel: base64 image
    // TODO: Tool Calls

    // Structured outputs
    // example_structured_outputs_json_object().await?;
    // example_structured_outputs_json_schema().await?;

    // TODO: Prompt Caching with `cache_control` for Anthropic
    // TODO: Transforms: https://openrouter.ai/docs/transforms

    Ok(())
}

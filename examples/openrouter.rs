use async_llm::{
    init_tracing,
    types::{ChatResponseFormat, ChatToolFunction, JsonSchema},
    ChatMessage, ChatRequest, Error,
};
use serde_json::json;

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
async fn example_tool_calls() -> Result<(), Error> {
    let request = ChatRequest::new(
        "mistralai/ministral-8b",
        // "openai/gpt-4o-mini",
        // "google/gemini-flash-1.5-8b", // error
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user(r#"What's the weather like in Vietnam?"#),
        ],
    )
    .tools(vec![ChatToolFunction::new("get_current_weather")
        .strict(true)
        .description("Get the current weather in a given location")
        .parameters(json!({
          "type": "object",
          "properties": {
            "location": {
              "type": "string",
              "description": "The city and state, e.g. San Francisco, CA"
            },
            "unit": {
              "type": "string",
              "enum": [
                "celsius",
                "fahrenheit"
              ]
            }
          },
          "required": [
            "location"
          ],
          "additionalProperties": false
        }))]);
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

#[allow(unused)]
/// Note: SambaNova Provider returns error: `image_url` must start with 'data:image/<jpeg|jpg|png|webp>;base64,'\"
async fn example_image_url() -> Result<(), Error> {
    let request = ChatRequest::new(
        "meta-llama/llama-3.2-11b-vision-instruct:free",
        // "openai/gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user_image("https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg"),
            ChatMessage::user("What's in this image?"),
        ],
    );

    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

#[allow(unused)]
async fn example_image_base64() -> Result<(), Error> {
    let request = ChatRequest::new(
        // "meta-llama/llama-3.2-11b-vision-instruct",
        "meta-llama/llama-3.2-11b-vision-instruct:free",
        // "openai/gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user_image_with_text("What's in this image?", BASE64_IMAGE),
        ],
    );

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

    // Images & Multimodel: image_url
    // example_image_url().await?;
    // Images & Multimodel: base64 image
    // example_image_base64().await?;

    // Tool Calls
    // example_tool_calls().await?;

    // Structured outputs
    // example_structured_outputs_json_object().await?;
    // example_structured_outputs_json_schema().await?;

    // TODO: Prompt Caching with `cache_control` for Anthropic
    // TODO: Transforms: https://openrouter.ai/docs/transforms

    Ok(())
}

static BASE64_IMAGE: &str = r#"data:image/jpeg;base64,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"#;

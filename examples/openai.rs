use async_llm::{
    types::{ChatResponseFormat, ChatToolFunction, JsonSchema},
    ChatMessage, ChatRequest, Error, Printable,
};
use serde_json::json;
use tokio_stream::StreamExt;

mod utils;
use utils::tracing::init_tracing;

#[allow(unused)]
async fn example_basic() -> Result<(), Error> {
    let request = ChatRequest::new(
        "gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user("Who are you?"),
        ],
    );
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

#[allow(unused)]
async fn example_basic_stream() -> Result<(), Error> {
    let request = ChatRequest::new(
        "gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user("Who are you?"),
        ],
    )
    .stream();
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let mut response = request.send_stream().await?;
    while let Some(result) = response.next().await {
        match result {
            Ok(response) => {
                tracing::info!("response: \n{}", response.to_string_pretty()?);
            }
            Err(e) => {
                tracing::error!("error = \n {e}");
            }
        }
    }

    Ok(())
}

#[allow(unused)]
async fn example_assistant_prefill() -> Result<(), Error> {
    let request = ChatRequest::new(
        "gpt-4o-mini",
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
        "gpt-4o-mini",
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
        "gpt-4o-mini",
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
        "gpt-4o-mini",
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
async fn example_image_url() -> Result<(), Error> {
    let request = ChatRequest::new(
        "gpt-4o-mini",
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
        "gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user_image_with_text("What's in this image?", utils::BASE64_EXAMPLE_IMAGE),
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

    example_basic().await?;
    // example_basic_stream().await?;

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

    Ok(())
}

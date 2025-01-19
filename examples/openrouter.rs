use async_llm::{init_tracing, ChatMessage, ChatRequest, Error};

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    init_tracing();

    // https://openrouter.ai/docs/requests
    // example_basic().await?;
    example_assistant_prefill().await?;
    // TODO: Images & Multimodel: image_url
    // TODO: Images & Multimodel: base64 image
    // TODO: Tool Calls
    // TODO: Structured outputs
    // TODO: Prompt Caching with `cache_control` for Anthropic
    // TODO: Transforms: https://openrouter.ai/docs/transforms

    Ok(())
}

use async_llm::{
    providers::OpenAIConfig, utils::init_tracing, ChatMessage, ChatRequest, Client, Error,
    Printable, RawProvider,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    init_tracing();
    tracing::info!("This program will loop through each available providers and save outputs into JSON files for testing purpose");

    let request = ChatRequest::new(
        "mistralai/mistral-7b-instruct:free", // openrouter
        // "openai/gpt-4o-mini", // openrouter
        // "gpt-4o-mini", // openai
        vec![ChatMessage::system(
            "You are a helpful assistant. Answer in one sentence",
        )],
    )
    .user("1 + 1 = ");
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let provider = RawProvider::new(OpenAIConfig::default());
    let client = Client::with_provider(provider);

    let response = client
        .chat()
        .create(serde_json::to_value(request.clone()).unwrap())
        .await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    let request = request.stream();
    let mut response = client
        .chat()
        .create_stream(serde_json::to_value(request).unwrap())
        .await?;
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

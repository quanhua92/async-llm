use async_llm::{init_tracing, ChatMessage, ChatRequest, Error};

async fn example_basic() -> Result<(), Error> {
    let request = ChatRequest::new(
        "meta-llama/llama-3.2-3b-instruct:free",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user("Who are you?"),
        ],
    )
    .user("1 + 1 =");
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    example_basic().await.unwrap();
}

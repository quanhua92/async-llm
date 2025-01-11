use async_llm::{
    chat::{
        ChatCompletionRequest, ChatCompletionRequestDeveloperMessageBuilder,
        ChatCompletionRequestUserMessageBuilder,
    },
    client::Client,
    completions::CompletionRequest,
    error::Error,
    utils::init_tracing,
};

#[allow(unused)]
async fn create_completion() {
    let client = Client::new();
    let request = CompletionRequest::builder()
        .model("gpt-3.5-turbo-instruct")
        .prompt("who are you?")
        .build()
        .unwrap();
    tracing::debug!("request: {request:#?}");
    let response = client.completions().create(request).await.unwrap();
    tracing::debug!("response: {response:#?}");
}

async fn create_chat_completion() -> Result<(), Error> {
    let client = Client::new();
    let request = ChatCompletionRequest::builder()
        // .model("gpt-3.5-turbo-instruct")
        // .model("qwen/qwen-2-7b-instruct:free")
        .model("meta-llama/llama-3.2-3b-instruct:free")
        .messages([
            ChatCompletionRequestDeveloperMessageBuilder::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageBuilder::default()
                .content("1 + 1 =")
                .build()?
                .into(),
        ])
        .build()
        .unwrap();
    tracing::debug!("request: {request:#?}");
    let response = client.chat().create(request).await.unwrap();
    tracing::debug!("response: {response:#?}");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    // create_completion().await;
    create_chat_completion().await.unwrap();
}

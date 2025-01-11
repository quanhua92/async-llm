use async_llm::utils::init_tracing;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();
    tracing::info!("This program will loop through each available providers and save outputs into JSON files for testing purpose");
}

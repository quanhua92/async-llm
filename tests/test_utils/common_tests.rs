use async_llm::async_llm;

pub async fn common_test_chat_completions_simple() -> Result<(), String> {
    async_llm().await;
    Ok(())
}

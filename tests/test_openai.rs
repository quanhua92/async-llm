use async_llm::utils::init_tracing;

mod test_utils;
use test_utils::common_tests;

#[tokio::test]
pub async fn test_chat_completions_simple() -> Result<(), String> {
    init_tracing();
    let _ = common_tests::common_test_chat_completions_simple().await;
    assert!(false);
    Ok(())
}

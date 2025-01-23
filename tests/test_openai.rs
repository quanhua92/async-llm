mod test_utils;
use test_utils::common_tests;

#[tokio::test]
pub async fn test_chat_completions_simple() -> Result<(), String> {
    let _ = common_tests::common_test_chat_completions_simple().await;
    Ok(())
}

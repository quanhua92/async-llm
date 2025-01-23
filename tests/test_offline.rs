use std::{
    fs::{self},
    path::PathBuf,
};

use async_llm::{ChatRequest, ChatResponse, Error};
use serde::Deserialize;

use test_utils::fs::read_json;

mod test_utils;

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Info {
    model_name: String,
    provider_name: String,
    test_name: String,
}

pub async fn process_test(root_path: PathBuf) -> Result<(), Error> {
    // Read info.json
    let info_path = root_path.join("info.json");
    let info: Info = read_json(info_path).unwrap();
    println!("info = {info:?}");

    // Read request.json
    let request_path = root_path.join("request.json");
    let request: ChatRequest = read_json(request_path).unwrap();
    println!("request = {request:?}");

    // Read response.json
    let response_path = root_path.join("response.json");
    let response: ChatResponse = read_json(response_path).unwrap();
    println!("response = {response:?}");
    Ok(())
}

#[tokio::test]
pub async fn test_offline() -> Result<(), Error> {
    // Construct the data path
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("data");
    println!("data_path: {:?}", data_path);

    // Iterate over entries in the data directory
    for test_case_entry in fs::read_dir(&data_path).unwrap() {
        let test_case_path = test_case_entry.unwrap().path();

        // Skip if the entry is not a directory
        if !test_case_path.is_dir() {
            continue;
        }

        println!("test_case_path: {:?}", test_case_path);

        // Iterate over entries in the test case directory
        for provider_model_entry in fs::read_dir(&test_case_path).unwrap() {
            let provider_model_path = provider_model_entry.unwrap().path();
            println!("provider_model_path: {:?}", provider_model_path);
            process_test(provider_model_path).await?;
        }
    }

    Ok(())
}

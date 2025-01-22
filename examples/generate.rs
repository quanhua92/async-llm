use core::panic;
use std::{fs, path::PathBuf, str::FromStr};

use async_llm::{
    client::DefaultHttpClient,
    http::HttpClient,
    providers::{Config, OpenAIConfig},
    utils::init_tracing,
    ChatMessage, ChatRequest, Client, Error, Printable, Provider, RawProvider,
};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Proxy,
};
use secrecy::SecretString;
use serde_json::{json, Value};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    init_tracing();
    tracing::info!("This program will loop through each available providers and save outputs into JSON files for testing purpose");

    let openai_client = create_client("OPENAI_BASE_URL", "OPENAI_API_KEY");
    let gemini_client = create_client("GEMINI_BASE_URL", "GEMINI_API_KEY");
    let together_client = create_client("TOGETHER_BASE_URL", "TOGETHER_API_KEY");
    let openrouter_client = create_client("OPENROUTER_BASE_URL", "OPENROUTER_API_KEY");

    // generate(
    //     &gemini_client,
    //     "gemini",
    //     "gemini-2.0-flash-exp",
    //     "who_are_you",
    //     "who are you?",
    // )
    // .await?;

    generate(
        &together_client,
        "together",
        "meta-llama/Llama-3.3-70B-Instruct-Turbo-Free",
        "who_are_you",
        "who are you?",
    )
    .await?;

    generate(
        &openai_client,
        "openai",
        "gpt-4o-mini",
        "who_are_you",
        "who are you?",
    )
    .await?;

    generate(
        &openrouter_client,
        "openrouter",
        "mistralai/mistral-7b-instruct:free",
        "who_are_you",
        "who are you?",
    )
    .await?;

    Ok(())
}

fn stream() {
    // let client = Client::raw();
    // let response = client
    //     .chat()
    //     .create(serde_json::to_value(request.clone()).unwrap())
    //     .await?;
    // tracing::info!("response: \n{}", response.to_string_pretty()?);

    // let request = request.stream();
    // let mut response = client
    //     .chat()
    //     .create_stream(serde_json::to_value(request).unwrap())
    //     .await?;
    // while let Some(result) = response.next().await {
    //     match result {
    //         Ok(response) => {
    //             tracing::info!("response: \n{}", response.to_string_pretty()?);
    //         }
    //         Err(e) => {
    //             tracing::error!("error = \n {e}");
    //         }
    //     }
    // }
}

fn sanitize_folder_name(input: &str) -> String {
    let sanitized: String = input
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_', // Replace invalid characters
            c if c.is_control() => '_', // Replace control characters
            _ => c,                     // Keep valid characters
        })
        .collect();

    // Remove trailing periods or spaces (Windows-specific)
    sanitized
        .trim_end_matches(|c| c == '.' || c == ' ')
        .to_string()
}

fn save_json_to_file(value: &Value, path: &PathBuf) -> Result<(), std::io::Error> {
    let json_string = serde_json::to_string_pretty(value)?;

    fs::write(path, json_string)?;

    Ok(())
}

async fn generate<H: HttpClient>(
    client: &Option<Client<RawProvider, H>>,
    provider_name: impl Into<String>,
    model_name: impl Into<String>,
    test_name: impl Into<String>,
    prompt: impl Into<String>,
) -> Result<(), Error> {
    let test_name: String = test_name.into();
    let provider_name: String = provider_name.into();
    let model_name: String = model_name.into();
    match client {
        None => tracing::debug!(
            "Skip {}/{}/{} because client is None",
            provider_name,
            model_name,
            test_name
        ),
        Some(client) => {
            let request = ChatRequest::new(&model_name, vec![ChatMessage::user(prompt)]);
            let request = serde_json::to_value(request)?;
            tracing::debug!("Sending request: {:?}", request);
            let response = client.chat().create(request.clone()).await?;

            let provider_model_name =
                format!("{}_{}", provider_name, sanitize_folder_name(&model_name));

            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("data");
            d.push(&test_name);
            d.push(&provider_model_name);
            let output_path = d
                .to_str()
                .map(ToString::to_string)
                .ok_or(Error::InvalidArgument(format!(
                    "Failed to get path for data/{}/{}",
                    test_name, provider_model_name
                )))?;
            match fs::create_dir_all(&output_path) {
                Err(e) => tracing::error!("Failed to create folder: {:?}", e),
                Ok(_) => {
                    tracing::info!("Successfully created folder: {:?}", output_path);
                    // info.json
                    let mut info_path = PathBuf::from_str(&output_path).unwrap();
                    info_path.push("info.json");
                    let info = json!({
                        "provider_name": provider_name,
                        "test_name": test_name,
                        "model_name": model_name
                    });
                    match save_json_to_file(&info, &info_path) {
                        Ok(_) => tracing::info!("Succesfully created file: {:?}", info_path),
                        Err(e) => tracing::error!("Failed to create file: {:?}", e),
                    }

                    // request.json
                    let mut request_path = PathBuf::from_str(&output_path).unwrap();
                    request_path.push("request.json");
                    match save_json_to_file(&request, &request_path) {
                        Ok(_) => tracing::info!("Succesfully created file: {:?}", request_path),
                        Err(e) => tracing::error!("Failed to create file: {:?}", e),
                    }

                    // response.json
                    let mut response_path = PathBuf::from_str(&output_path).unwrap();
                    response_path.push("response.json");
                    match save_json_to_file(&response, &response_path) {
                        Ok(_) => tracing::info!("Succesfully created file: {:?}", response_path),
                        Err(e) => tracing::error!("Failed to create file: {:?}", e),
                    }
                }
            }
        }
    }

    Ok(())
}

fn create_client(
    env_base_url: &str,
    env_api_key: &str,
) -> Option<Client<RawProvider, DefaultHttpClient<OpenAIConfig>>> {
    match std::env::var(env_api_key) {
        Err(e) => {
            tracing::error!("Error: {e} - {env_api_key}");
            None
        }
        Ok(api_key) => match std::env::var(env_base_url) {
            Ok(base_url) => Some(Client::with_auth_raw(
                base_url,
                Some(SecretString::new(api_key.into())),
            )),
            Err(e) => {
                tracing::error!("Error: {e} - {env_base_url}");
                None
            }
        },
    }
}

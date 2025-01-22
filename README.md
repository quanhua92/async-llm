# async-llm

`async-llm` is a Rust library for working with OpenAI-compatible providers, including OpenAI, Gemini, OpenRouter, and Ollama.

**Note:** This repository is currently a **work-in-progress** and is under active development. As such, breaking changes may occur frequently. Please proceed with caution if using this code in production or for critical projects. We recommend checking the commit history and pull requests for the latest updates. Contributions, feedback, and issue reports are welcome! 🚧

## Why Choose `async-llm`?

Relying solely on OpenAI isn't ideal for every application. You can find numerous forum discussions about issues with OpenAI billing and availability. Using OpenAI-compatible providers gives you more options and flexibility. However, working with these APIs can be tricky due to differences in their specifications.

While some crates focus only on OpenAI or attempt a one-size-fits-all approach, `async-llm` takes a balanced path:
- **Follows the OpenAI Standard**: Closely aligns with the original OpenAI API design.
- **Flexible for Compatibility**: Adapts to compatible APIs by making all possible fields optional.
- **Simplifies Integration**: Provides traits to handle provider-specific tweaks without complexity.
- **Built for Performance**: Fully asynchronous, leveraging Rust's async ecosystem.

With `async-llm`, you can seamlessly work with multiple OpenAI-compatible APIs while maintaining a clean and consistent codebase.

## Installation

Add `async-llm` to your `Cargo.toml`:

```toml
[dependencies]
async-llm = "0.0.1"
```

## Examples

| Name | Description |
|------|-------------|
| [`openai`](examples/openai.rs) | OpenAI example |
| [`openrouter`](examples/openrouter.rs) | OpenRouter example |

## Usage

```rust
async fn example_basic() -> Result<(), Error> {
    let request = ChatRequest::new(
        "gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user("Who are you?"),
        ],
    );
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let response = request.send().await?;
    tracing::info!("response: \n{}", response.to_string_pretty()?);

    Ok(())
}

async fn example_basic_stream() -> Result<(), Error> {
    let request = ChatRequest::new(
        "gpt-4o-mini",
        vec![
            ChatMessage::system("You are a helpful assistant"),
            ChatMessage::user("Who are you?"),
        ],
    )
    .stream();
    tracing::info!("request: \n{}", request.to_string_pretty()?);

    let mut response = request.send_stream().await?;
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
```

## Development

Install `just`
```
  cargo install just
```

Install additional tools
```
  just init
```

Start development
```
  just dev
```

Run tests

```
  just test
```

Run selected tests with debug tracing

```
  just test-one chat
```

Generate test data

```
  just generate
```

## References
+ [OpenAI API Reference](https://platform.openai.com/docs/api-reference)
+ [OpenAI OpenAPI spec](https://github.com/openai/openai-openapi/)
+ [64bit/async-openapi](https://github.com/64bit/async-openai/)
+ [dongri/openai-api-rs](https://github.com/dongri/openai-api-rs)
+ [jeremychone/rust-genai](https://github.com/jeremychone/rust-genai)
+ [graniet/llm](https://github.com/graniet/llm)

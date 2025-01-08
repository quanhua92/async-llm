# async-llm

`async-llm` is a Rust library for working with OpenAI-compatible providers, including OpenAI, Gemini, OpenRouter, and Ollama.

## Why Choose `async-llm`?

Relying solely on OpenAI isn't ideal for every application. You can find numerous forum discussions about issues with OpenAI billing and availability. Using OpenAI-compatible providers gives you more options and flexibility. However, working with these APIs can be tricky due to differences in their specifications.

While some crates focus only on OpenAI or attempt a one-size-fits-all approach, `async-llm` takes a balanced path:
- **Follows the OpenAI Standard**: Closely aligns with the original OpenAI API design.
- **Flexible for Compatibility**: Adapts to compatible APIs by making all possible fields optional.
- **Simplifies Integration**: Provides traits to handle provider-specific tweaks without complexity.
- **Built for Performance**: Fully asynchronous, leveraging Rust's async ecosystem.

With `async-llm`, you can seamlessly work with multiple OpenAI-compatible APIs while maintaining a clean and consistent codebase.

## Getting started

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
+ [graniet/rllm](https://github.com/graniet/rllm)

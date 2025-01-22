# Data

This folder contains test data for the library. It is organized to facilitate testing across different prompts, providers, and models.

## Folder Structure

The `data` folder is structured to store test cases for various prompts. Each prompt has its own folder, which contains subfolders for different combinations of `provider` and `model`. Each subfolder includes the following files:

- **`request.json`**: The serialized request JSON sent to the Chat Completions API.
- **`response.json`**: The expected response JSON from the Chat Completions API.
- **`info.json`**: Metadata for the test runner to replicate the setup (e.g., provider, model, parameters).

### Example Folder Structure

```
data/
  - who_are_you/                     # Prompt name
    - openai_gpt-4o-mini/            # Provider and model combination
      - request.json                 # Request payload
      - response.json                # Expected response
      - info.json                    # Metadata for the test
    - openrouter_mistral-7b-instruct/
    - openrouter_gemini-flash-1.5-8b/
    - gemini_gemini-2.0-flash-exp/
  - who_are_you_stream/              # Another prompt (e.g., for streaming responses)
    - openai_gpt-4o-mini/
    - openrouter_mistral-7b-instruct/
    - openrouter_gemini-flash-1.5-8b/
    - gemini_gemini-2.0-flash-exp/
```

## How to Generate Test Data

You can create test data manually by following the folder structure and adding the required files (`request.json`, `response.json`, and `info.json`). Alternatively, you can use the `generate` script to automate the creation of multiple test cases.

### Using the `generate` Script

To generate test data automatically, run the following command:

```bash
just generate
```

This script will create the necessary folders and files for each prompt, provider, and model combination based on predefined templates or configurations.

### Manual Creation

If you prefer to create test data manually, follow these steps:

1. **Create a Prompt Folder**: Start by creating a folder for the prompt (e.g., `who_are_you`).
2. **Add Provider-Model Subfolders**: Inside the prompt folder, create subfolders for each provider and model combination (e.g., `openai_gpt-4o-mini`).
3. **Add Required Files**:
   - `request.json`: Define the request payload for the API.
   - `response.json`: Specify the expected response from the API.
   - `info.json`: Include metadata such as provider, model, and test parameters.
4. **Repeat for Other Prompts**: Follow the same structure for additional prompts (e.g., `who_are_you_stream`).

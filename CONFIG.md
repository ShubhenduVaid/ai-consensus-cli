# AI Consensus CLI - Configuration Guide

## Adding New LLMs

The AI Consensus CLI is now fully configurable! You can add new LLM tools without recompiling by editing the `config.toml` file.

## Configuration Format

```toml
[tools.your_tool_name]
name = "Display Name"
command = "cli-command"
args = ["arg1", "arg2", "{prompt}"]
description = "What this AI is good for"
```

## Parameters

- **`your_tool_name`**: The key used in CLI commands (e.g., `-s your_tool_name`)
- **`name`**: Human-readable name shown in help
- **`command`**: The actual CLI command to execute
- **`args`**: Array of arguments, use `{prompt}` where the user's question goes
- **`description`**: Brief description of the AI's strengths

## Examples

### Adding Mistral AI
```toml
[tools.mistral]
name = "Mistral AI"
command = "mistral"
args = ["chat", "{prompt}"]
description = "European AI model, strong multilingual support"
```

### Adding a Custom Ollama Model
```toml
[tools.codellama]
name = "Code Llama"
command = "ollama"
args = ["run", "codellama:7b", "{prompt}"]
description = "Specialized for code generation and programming tasks"
```

### Adding Hugging Face CLI
```toml
[tools.huggingface]
name = "Hugging Face"
command = "huggingface-cli"
args = ["chat", "--model", "microsoft/DialoGPT-medium", "{prompt}"]
description = "Access to open-source models from Hugging Face"
```

### Adding a Custom Script
```toml
[tools.custom]
name = "My Custom AI"
command = "python"
args = ["/path/to/my_ai_script.py", "{prompt}"]
description = "Custom AI implementation"
```

## Usage

After adding tools to `config.toml`, use them immediately:

```bash
# Use new tools
ai-co -s mistral,codellama -c q -p "Write a Python function"

# Mix old and new
ai-co -s q,mistral,ollama -c gemini -p "Your question"

# Check available tools
ai-co -s "" -c "" -p ""
```

## Custom Config File

Use a different config file:

```bash
ai-co --config my-config.toml -s tool1,tool2 -c tool1 -p "question"
```

## Benefits

- ✅ **No Recompilation**: Add tools instantly
- ✅ **Flexible Arguments**: Support any CLI format
- ✅ **Custom Models**: Use different Ollama models
- ✅ **Scripts**: Integrate custom AI scripts
- ✅ **Easy Maintenance**: Update tools without code changes

## Tool Requirements

For a tool to work:
1. The CLI command must be installed and in PATH
2. It should accept the prompt and return text output
3. Authentication should be configured separately

The CLI will automatically detect unavailable tools and skip them gracefully.

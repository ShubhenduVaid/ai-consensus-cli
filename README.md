# AI Consensus CLI

Orchestrate multiple AI CLI tools with consensus functionality. Get diverse perspectives on your questions by running multiple AI tools simultaneously and having another AI provide consensus on the results.

## Features

- **Multi-AI Orchestration**: Run multiple AI tools simultaneously
- **Consensus Engine**: Get synthesized insights from different AI perspectives  
- **Configurable Tools**: Add new AI tools without recompiling via TOML config
- **Async Processing**: Parallel execution for faster results
- **Error Handling**: Graceful handling of unavailable tools

## Prerequisites

### Required
- Rust 1.70+ and Cargo - [Install Rust](https://rustup.rs/)
- Git - [Install Git](https://git-scm.com/downloads)

### AI CLI Tools (Install as needed)
Before using the consensus CLI, install the AI tools you want to use:

- **Amazon Q CLI** - [Installation Guide](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-getting-started-installing.html)
- **Google Gemini CLI** - [Installation Guide](https://github.com/google-gemini/gemini-cli)
- **Anthropic Claude CLI** - [Installation Guide](https://docs.anthropic.com/en/docs/claude-code/setup#standard-installation)
- **OpenAI Codex CLI** - [Installation Guide](https://github.com/openai/openai-python)
- **Ollama** - [Installation Guide](https://ollama.ai/download)
- **OpenAI CLI** - [Installation Guide](https://developers.openai.com/codex/cli/#set-up)

## Installation

### Quick Install
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/ai-consensus-cli/main/install.sh | bash
```

### Manual Install
```bash
# Clone the repository
git clone https://github.com/yourusername/ai-consensus-cli.git
cd ai-consensus-cli

# Run install script
./install.sh
```

### From Source
```bash
cargo install --path .
```

## Building the Project

### Prerequisites
- Rust 1.70+ and Cargo
- Git

### Build Steps
```bash
# Clone the repository
git clone https://github.com/yourusername/ai-consensus-cli.git
cd ai-consensus-cli

# Build in debug mode (faster compilation)
cargo build

# Build optimized release version
cargo build --release

# Install globally from source
cargo install --path .
```

### Build Artifacts
- Debug binary: `target/debug/ai-co`
- Release binary: `target/release/ai-co`

## Running the Project

### After Installation
```bash
# Basic usage (if installed globally)
ai-co -s gemini,q -c claude -p "How do I optimize a Rust web server?"

# Multiple solvers with consensus
ai-co -s q,ollama,gemini -c claude -p "Design a microservices architecture"

# Use custom config
ai-co --config my-config.toml -s tool1,tool2 -c tool3 -p "Your question"
```

### Running from Source (without installation)
```bash
# Run debug build
cargo run -- -s q,ollama -c claude -p "Your question"

# Run release build
./target/release/ai-co -s q,ollama -c claude -p "Your question"

# Run with environment variables
RUST_LOG=info cargo run -- -s q,ollama -c claude -p "test question"
```

### Command Line Options
```bash
ai-co --help  # Show all available options

# Required arguments:
# -s, --solvers <SOLVERS>     AI tools to solve (comma-separated)
# -c, --consensus <CONSENSUS> AI tool for consensus
# -p, --prompt <PROMPT>       Question to solve

# Optional arguments:
# --config <CONFIG>           Path to config file (default: config.toml)
```

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test integration_tests

# Run tests with logging
RUST_LOG=debug cargo test

# Run tests in release mode
cargo test --release
```

### Test Coverage
```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Configuration

The CLI uses a TOML configuration file to define available AI tools. See [CONFIG.md](CONFIG.md) for detailed configuration instructions.

### Default Tools
- `q` - Amazon Q CLI - [Install Guide](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-getting-started-installing.html)
- `gemini` - Google Gemini CLI - [Install Guide](https://ai.google.dev/gemini-api/docs/quickstart?lang=python)
- `claude` - Anthropic Claude CLI - [Install Guide](https://github.com/anthropics/anthropic-cli)
- `ollama` - Local LLM via Ollama - [Install Guide](https://ollama.ai/download)
- `codex` - OpenAI Codex CLI - [Install Guide](https://developers.openai.com/codex/cli/#set-up)
- `openai` - OpenAI ChatGPT CLI - [Install Guide](https://github.com/openai/openai-python)

## Requirements

- Rust 1.70+ (for building from source)
- AI CLI tools you want to use must be installed and configured
- Unix-like system (Linux, macOS)

## Development

### Development Workflow
```bash
# Clone and setup
git clone https://github.com/yourusername/ai-consensus-cli.git
cd ai-consensus-cli

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy

# Run tests
cargo test

# Build and test everything
cargo build && cargo test && cargo clippy
```

### Debugging
```bash
# Run with debug logging
RUST_LOG=debug cargo run -- -s q,ollama -c claude -p "test question"

# Run with trace logging
RUST_LOG=trace cargo run -- -s q,ollama -c claude -p "test question"
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust and Tokio for async processing
- Uses Clap for CLI argument parsing
- Inspired by the need for diverse AI perspectives

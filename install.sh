#!/bin/bash

# Install AI Consensus CLI globally
echo "Installing AI Consensus CLI..."

# Build release version
cargo build --release

# Create config directory
mkdir -p ~/.config/ai-consensus-cli

# Copy config file
cp config.toml ~/.config/ai-consensus-cli/

# Install binary
cargo install --path .

echo "✅ Installation complete!"
echo ""
echo "Usage from anywhere:"
echo "  ai-co -s ollama -c ollama -p 'Your question'"
echo ""
echo "Config location: ~/.config/ai-consensus-cli/config.toml"

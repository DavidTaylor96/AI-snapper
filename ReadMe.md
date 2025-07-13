
## Usage Instructions

1. **Setup:**
   ```bash
   # Create new Rust project
   cargo new ai-screenshot-analyzer
   cd ai-screenshot-analyzer
   
   # Replace Cargo.toml and src/ with the code above
   ```

2. **Set API Key:**
   ```bash
   export AI_API_KEY="your-openai-api-key"
   # or
   export AI_API_KEY="your-claude-api-key"
   ```

3. **Build and Run:**
   ```bash
   # Build optimized release
   cargo build --release
   
   # Run daemon
   ./target/release/ai-screenshot-analyzer run
   
   # Or with custom settings
   ./target/release/ai-screenshot-analyzer --provider claude --prompt "Describe this screenshot" run
   ```

4. **Commands:**
   ```bash
   # Run daemon (default)
   ./target/release/ai-screenshot-analyzer run
   
   # Single screenshot
   ./target/release/ai-screenshot-analyzer capture
   
   # Test AI connection
   ./target/release/ai-screenshot-analyzer test
   
   # Show config
   ./target/release/ai-screenshot-analyzer config
   ```

## Key Features

- **Global Hotkey**: Ctrl+Shift+A to capture and analyze
- **Optimized Image Processing**: Smart format selection (PNG/JPEG)
- **Multiple AI Providers**: OpenAI GPT-4 Vision, Claude 3.5 Sonnet
- **Performance Optimized**: Binary uploads, efficient compression
- **Beautiful Terminal UI**: Progress indicators, colored output
- **Configurable**: Custom prompts, providers, quality settings
- **Error Handling**: Comprehensive error handling and logging

This implementation provides everything you need for a high-performance AI screenshot analyzer with optimal accuracy and speed!
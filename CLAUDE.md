# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based AI screenshot analyzer that captures screenshots via global hotkey (Ctrl+Shift+F12) and analyzes them using various AI providers (OpenAI GPT-4 Vision, Claude 3.5 Sonnet). The application runs as a daemon process with a beautiful terminal UI and includes optimized image processing for performance.

## Development Commands

```bash
# Build and run
cargo build --release
cargo run

# Run specific commands
cargo run -- run                    # Start daemon (default)
cargo run -- capture               # Single screenshot analysis  
cargo run -- test                  # Test AI connection
cargo run -- config                # Show current configuration

# With custom options
cargo run -- --provider claude --prompt "Custom prompt" run
cargo run -- --debug run           # Debug logging

# Testing (if tests exist)
cargo test
```

## Architecture

### Core Components

- **main.rs**: Entry point with CLI parsing, daemon management, and global hotkey handling
- **ai_client.rs**: Unified AI client supporting multiple providers (OpenAI, Claude, Gemini)
- **screenshot.rs**: Cross-platform screenshot capture with intelligent format optimization
- **image_processor.rs**: Image optimization, compression, and format selection
- **config.rs**: Configuration management with TOML serialization
- **cli.rs**: Command-line argument definitions using clap

### Key Patterns

- **Async Architecture**: Built on tokio runtime for non-blocking operations
- **State Management**: Arc<AppState> pattern for sharing state across async tasks
- **Error Handling**: anyhow crate for comprehensive error propagation
- **Configuration**: TOML-based config with sensible defaults, stored in `~/.config/ai-screenshot-analyzer/`
- **Logging**: tracing crate with configurable log levels

### AI Integration

The AIClient supports multiple providers through a unified interface:
- OpenAI GPT-4 Vision API
- Claude 3.5 Sonnet API  
- Extensible for additional providers

Images are optimized before upload using intelligent format selection (PNG for UI elements, JPEG for photos) and quality/size optimization.

## Environment Variables

```bash
AI_API_KEY="your-api-key"          # Required: API key for chosen provider
```

## Configuration

Default config location: `~/.config/ai-screenshot-analyzer/config.toml`

Key settings:
- `screenshots_dir`: Where screenshots are temporarily stored
- `image_format`: Default format (png/jpeg)
- `jpeg_quality`: Compression quality (1-100)
- `max_image_size_mb`: Maximum image size for upload
- `default_provider`: AI provider to use

## Dependencies

Key external crates:
- `screenshots`: Cross-platform screenshot capture
- `global-hotkey`: System-wide hotkey registration
- `reqwest`: HTTP client for AI APIs
- `image`: Image processing and optimization
- `clap`: CLI argument parsing
- `tokio`: Async runtime
- `serde`/`serde_json`: Serialization
- `anyhow`: Error handling
- `tracing`: Structured logging
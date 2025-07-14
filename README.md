# 🤖 AI Screenshot Analyzer

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-success.svg)]()

> **A blazingly fast, cross-platform AI screenshot analyzer that captures and analyzes screenshots with global hotkeys**

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## 🚀 Project Overview

**AI Screenshot Analyzer** is a powerful Rust-based application that captures screenshots via global hotkey (**Cmd+Shift+Space**) and analyzes them using various AI providers. The application runs as a daemon process with a beautiful terminal UI and includes optimized image processing for performance.

### ✨ Key Features

- 🔥 **Lightning Fast**: Built with Rust for maximum performance
- 🌍 **Cross-Platform**: Works on macOS, Windows, and Linux
- 🤖 **Multi-AI Support**: OpenAI GPT-4 Vision, Claude 3.5 Sonnet, and more
- ⌨️ **Global Hotkeys**: **Cmd+Shift+Space** (macOS) for instant capture
- 🎨 **Beautiful Terminal UI**: Rich, colorized output with progress indicators
- 🖼️ **Smart Image Processing**: Intelligent format optimization (PNG/JPEG)
- ⚙️ **Zero-Config**: Works out of the box with sensible defaults

---

## 🛠️ Development Commands

### 🐳 Docker Usage (Recommended)

Docker provides a consistent environment that avoids Rust installation and firewall issues:

```bash
# Build the Docker image
docker build -t ai-screenshot-analyzer .

# Run the application in container
docker run -it ai-screenshot-analyzer

# Run with environment variables
docker run -it -e AI_API_KEY="your-api-key" ai-screenshot-analyzer

# Run with volume for persistent config
docker run -it -v ~/.config:/root/.config ai-screenshot-analyzer
```

### Basic Usage (Native)
```bash
# Build and run (release mode for best performance)
cargo build --release
cargo run

# Quick development build
cargo build
cargo run -- run
```

### Command Options
```bash
# Core Commands
cargo run -- run                    # 🏃 Start daemon (default)
cargo run -- capture               # 📸 Single screenshot analysis  
cargo run -- test                  # 🧪 Test AI connection
cargo run -- config                # ⚙️  Show current configuration

# Advanced Usage
cargo run -- --provider claude --prompt "Custom prompt" run
cargo run -- --debug run           # 🐛 Debug logging with verbose output
cargo run -- --api-key "sk-..." --provider openai run
```

### Testing & Quality
```bash
# Run Rust unit tests
cargo test

# Run automation tests (Python required)
python3 run_tests.py

# Code quality checks
cargo clippy -- -D warnings       # 🔍 Linting
cargo fmt                         # 🎨 Formatting
```

---

## 🏗️ Architecture

### 📁 Core Components

| File | Purpose | Key Features |
|------|---------|--------------|
| **main.rs** | 🎯 Entry point & daemon management | CLI parsing, hotkey handling, async coordination |
| **hotkey_monitor.rs** | ⌨️ Global hotkey detection | Device-query based, cross-platform, debounced |
| **ai_client.rs** | 🤖 AI provider integration | Multi-provider support, error handling, rate limiting |
| **screenshot.rs** | 📸 Screenshot capture & processing | Cross-platform capture, format optimization |
| **config.rs** | ⚙️ Configuration management | TOML-based, auto-creation, validation |
| **ui.rs** | 🎨 Terminal user interface | Colorized output, progress bars, status messages |

### 🔧 Key Patterns

- **🔄 Async Architecture**: Built on `tokio` runtime for non-blocking operations
- **🔗 State Management**: `Arc<AppState>` pattern for sharing state across async tasks
- **⚠️ Error Handling**: `anyhow` crate for comprehensive error propagation
- **📋 Configuration**: TOML-based config with sensible defaults
- **📊 Logging**: `tracing` crate with configurable log levels
- **🔐 Security**: No secrets in logs, secure API key handling

### 🤖 AI Integration

The **AIClient** supports multiple providers through a unified interface:

| Provider | Model | Features |
|----------|-------|----------|
| **OpenAI** | GPT-4o-mini | Fast, cost-effective, excellent for UI analysis |
| **Claude** | Claude 3.5 Sonnet | Superior reasoning, detailed analysis |
| **Gemini** | *Coming Soon* | Google's multimodal AI |

**Smart Image Processing**:
- 🖼️ **PNG** for UI elements, text, and screenshots with sharp edges
- 📷 **JPEG** for photos and complex images with gradients
- 🗜️ **Auto-compression** to stay under API limits
- 📏 **Size optimization** with quality preservation

---

## ⚙️ Configuration

### 🔑 Environment Variables

```bash
# Required: API key for your chosen provider
AI_API_KEY="your-api-key-here"

# Optional: Override default provider
AI_PROVIDER="openai"  # or "claude"
```

### 📄 Configuration File

**Location**: `~/.config/ai-screenshot-analyzer/config.toml`

```toml
# Screenshot storage (temporary)
screenshots_dir = "/Users/username/.ai-screenshots"

# Image processing
image_format = "png"          # Default format (png/jpeg)
jpeg_quality = 95             # Compression quality (1-100)
max_image_size_mb = 10        # Upload size limit

# AI provider settings
default_provider = "openai"   # Default AI provider
```

### 🔧 Auto-Configuration

The app automatically:
- ✅ Creates config directory if missing
- ✅ Generates default config file
- ✅ Creates screenshots directory
- ✅ Validates settings on startup

---

## 🎯 Global Hotkey System

### 💡 How It Works

The application uses `device_query` for reliable cross-platform hotkey detection:

```rust
// Hotkey combination: Cmd+Shift+Space (macOS)
let space_pressed = keys.contains(&Keycode::Space);
let meta_pressed = keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::RMeta);
let shift_pressed = keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift);
```

### ⌨️ Platform-Specific Hotkeys

| Platform | Hotkey | Notes |
|----------|--------|-------|
| **macOS** | `Cmd+Shift+Space` | Tested and reliable |
| **Windows** | `Ctrl+Shift+Space` | Cross-platform compatible |
| **Linux** | `Ctrl+Shift+Space` | X11 and Wayland support |

### 🔧 Debouncing & Performance

- ⏱️ **1-second debounce** prevents accidental double-triggers
- 🔄 **100ms polling** for responsive detection
- 📊 **Debug logging** for troubleshooting

---

## 📦 Dependencies

### 🚀 Core Runtime Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.0+ | Async runtime and task management |
| `device_query` | 2.1 | Cross-platform hotkey detection |
| `screenshots` | 0.5 | Cross-platform screenshot capture |
| `reqwest` | 0.11 | HTTP client for AI APIs |
| `anyhow` | 1.0 | Error handling and propagation |

### 🎨 UI & Formatting

| Crate | Version | Purpose |
|-------|---------|---------|
| `crossterm` | 0.27 | Terminal manipulation and colors |
| `indicatif` | 0.17 | Progress bars and spinners |
| `clap` | 4.0 | CLI argument parsing |
| `tracing` | 0.1 | Structured logging |

### 🖼️ Image Processing

| Crate | Version | Purpose |
|-------|---------|---------|
| `image` | 0.24 | Image format conversion and optimization |
| `base64` | 0.22 | Image encoding for API upload |

### ⚙️ Configuration & Serialization

| Crate | Version | Purpose |
|-------|---------|---------|
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON handling for API requests |
| `toml` | 0.8 | Configuration file parsing |
| `dirs` | 5.0 | Cross-platform directory paths |

---

## 🔍 Testing & Quality Assurance

### 🧪 Test Suite

```bash
# Rust unit tests
cargo test

# Python integration tests (requires virtual environment)
python3 run_tests.py

# Manual testing commands
cargo run -- test              # Test AI connectivity
cargo run -- capture          # Test screenshot capture
cargo run -- config           # Verify configuration
```

### 📊 Test Coverage

- ✅ **Unit Tests**: Core functionality and error handling
- ✅ **Integration Tests**: End-to-end workflow testing
- ✅ **Permission Tests**: macOS accessibility and screen recording
- ✅ **Hotkey Tests**: Cross-platform input detection

### 🔍 Code Quality

```bash
# Linting with zero warnings
cargo clippy -- -D warnings

# Consistent formatting
cargo fmt

# Security audit (optional)
cargo audit
```

---

## 🚨 Troubleshooting

### 🔧 Common Issues

| Issue | Solution |
|-------|----------|
| **Hotkeys not working** | Check accessibility permissions in System Preferences |
| **Screenshot capture fails** | Verify screen recording permissions |
| **API errors** | Validate `AI_API_KEY` environment variable |
| **Build failures** | Run `cargo clean` and rebuild |

### 🍎 macOS Permissions

The app requires these permissions:

1. **🔐 Accessibility**: For global hotkey detection
   - `System Preferences → Security & Privacy → Privacy → Accessibility`
   - Add and enable Terminal or your app

2. **📺 Screen Recording**: For screenshot capture
   - `System Preferences → Security & Privacy → Privacy → Screen Recording`
   - Add and enable Terminal or your app

### 📝 Debug Mode

Enable detailed logging:

```bash
cargo run -- --debug run
```

This provides:
- 🔍 Detailed hotkey detection logs
- 📸 Screenshot capture debugging
- 🤖 AI API request/response details
- ⚙️ Configuration validation info

---

## 🎯 Performance Optimizations

### ⚡ Speed Optimizations

- **🦀 Rust Performance**: Zero-cost abstractions, memory safety
- **🔄 Async I/O**: Non-blocking screenshot capture and AI requests
- **🗜️ Image Compression**: Smart format selection reduces upload time
- **📊 Efficient Polling**: Optimized hotkey detection loop

### 💾 Memory Management

- **📸 Temporary Screenshots**: Auto-cleanup after analysis
- **🔄 Stream Processing**: Large images processed in chunks
- **♻️ Resource Cleanup**: Proper async task cleanup

---

## 🛡️ Security Considerations

### 🔒 API Key Security

- ✅ **Environment Variables**: Never hard-coded in source
- ✅ **No Logging**: API keys never appear in logs
- ✅ **Local Storage**: Config files use appropriate permissions

### 🖼️ Screenshot Privacy

- ✅ **Temporary Storage**: Screenshots deleted after analysis
- ✅ **Local Processing**: Images processed locally before upload
- ✅ **No Persistence**: No permanent storage of sensitive content

---

## 🚀 Getting Started

### 📋 Prerequisites

#### 🐳 Docker Method (Recommended)
- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **AI API Key**: OpenAI or Anthropic account

#### 🦀 Native Method
- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **macOS 10.15+** / **Windows 10+** / **Linux (X11/Wayland)**
- **AI API Key**: OpenAI or Anthropic account

### ⚡ Quick Start

#### 🐳 Using Docker (Recommended)

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd ai-screenshot-analyzer
   ```

2. **Build Docker image**:
   ```bash
   docker build -t ai-screenshot-analyzer .
   ```

3. **Run with API key**:
   ```bash
   docker run -it -e AI_API_KEY="your-api-key-here" ai-screenshot-analyzer
   ```

#### 🦀 Native Installation

1. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd ai-screenshot-analyzer
   cargo build --release
   ```

2. **Set up API key**:
   ```bash
   export AI_API_KEY="your-api-key-here"
   ```

3. **Run the app**:
   ```bash
   cargo run -- run
   ```

4. **Grant permissions** (macOS): Follow the prompts for Accessibility and Screen Recording

5. **Test hotkey**: Press **Cmd+Shift+Space** to capture and analyze!

---

## 📝 Important Notes for Claude Code

### 🔧 Development Guidelines

- **Always run `cargo fmt`** before committing changes
- **Use `cargo clippy`** to catch potential issues
- **Test on multiple platforms** when modifying hotkey code
- **Validate API changes** with the test command
- **Update this documentation** when adding new features

### 🚫 What NOT to do

- ❌ Never commit API keys or secrets
- ❌ Don't create files unless absolutely necessary
- ❌ Avoid adding unnecessary dependencies
- ❌ Don't modify hotkey combinations without testing
- ❌ Never skip the permission setup on macOS

### ✅ Best Practices

- ✅ Prefer editing existing files over creating new ones
- ✅ Use structured logging with appropriate levels
- ✅ Handle errors gracefully with user-friendly messages
- ✅ Maintain backward compatibility in config files
- ✅ Keep the CLI interface consistent and intuitive

---

*This documentation is maintained automatically. Last updated: $(date)*
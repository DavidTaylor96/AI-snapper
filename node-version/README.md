# 🤖 AI Screenshot Analyzer - Node.js Edition

[![Node.js](https://img.shields.io/badge/node.js-18+-brightgreen.svg)](https://nodejs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0+-blue.svg)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> **A fast, cross-platform AI screenshot analyzer built with Node.js and TypeScript - no Rust compiler needed!**

This is the Node.js/TypeScript version of the AI Screenshot Analyzer, designed to avoid firewall issues with Rust installation while maintaining all the same functionality.

---

## 🚀 Project Overview

**AI Screenshot Analyzer** is a powerful Node.js application that captures screenshots via global hotkey (**Cmd+Shift+Space**) and analyzes them using various AI providers. The application runs as a daemon process with a beautiful terminal UI and includes optimized image processing for performance.

### ✨ Key Features

- 🔥 **Fast Performance**: Built with Node.js and TypeScript for excellent performance
- 🌍 **Cross-Platform**: Works on macOS, Windows, and Linux
- 🤖 **Multi-AI Support**: OpenAI GPT-4 Vision, Claude 3.5 Sonnet, and more
- ⌨️ **Global Hotkeys**: **Cmd+Shift+Space** (macOS) / **Ctrl+Shift+Space** (Windows/Linux) for instant capture
- 🎨 **Beautiful Terminal UI**: Rich, colorized output with progress indicators
- 🖼️ **Smart Image Processing**: Intelligent format optimization using Sharp
- ⚙️ **Zero-Config**: Works out of the box with sensible defaults
- 🐳 **Docker Support**: Containerized deployment without local dependencies

---

## 🛠️ Development Commands

### 🐳 Docker Usage (Recommended)

Docker provides a consistent environment and avoids any installation issues:

```bash
# Build the Docker image
docker build -t ai-screenshot-analyzer-node .

# Run the application in container
docker run -it ai-screenshot-analyzer-node

# Run with environment variables
docker run -it -e AI_API_KEY="your-api-key" ai-screenshot-analyzer-node

# Run with volume for persistent config
docker run -it -v ~/.config:/home/nodejs/.config ai-screenshot-analyzer-node
```

### 📦 Node.js Usage (Native)

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Start the application
npm start

# Development mode with auto-reload
npm run dev
```

### Command Options

```bash
# Core Commands
npm run dev -- run                    # 🏃 Start daemon (default)
npm run dev -- capture               # 📸 Single screenshot analysis  
npm run dev -- test                  # 🧪 Test AI connection
npm run dev -- config                # ⚙️ Show current configuration

# Advanced Usage
npm run dev -- --provider openai --question "What's on screen?" run
npm run dev -- --debug run           # 🐛 Debug logging with verbose output
npm run dev -- --api-key "sk-..." --provider openai run
```

### Testing & Quality

```bash
# Run tests
npm test

# Linting
npm run lint

# Format code
npm run format

# Type checking
npx tsc --noEmit
```

---

## 🏗️ Architecture

### 📁 Core Components

| File | Purpose | Key Features |
|------|---------|--------------|
| **main.ts** | 🎯 Entry point & daemon management | CLI parsing, hotkey handling, async coordination |
| **hotkey_monitor.ts** | ⌨️ Global hotkey detection | global-hotkey based, cross-platform, debounced |
| **ai_client.ts** | 🤖 AI provider integration | Multi-provider support, error handling, rate limiting |
| **screenshot.ts** | 📸 Screenshot capture & processing | Cross-platform capture, Sharp optimization |
| **config.ts** | ⚙️ Configuration management | TOML-based, auto-creation, validation |
| **ui.ts** | 🎨 Terminal user interface | Chalk-based colors, progress indicators, status messages |

### 🔧 Key Patterns

- **🔄 Async/Await**: Built on modern async/await patterns for non-blocking operations
- **🔗 State Management**: Centralized AppState interface for sharing state
- **⚠️ Error Handling**: Comprehensive try/catch blocks with user-friendly error messages
- **📋 Configuration**: TOML-based config with sensible defaults
- **📊 Logging**: Console-based logging with configurable levels
- **🔐 Security**: No secrets in logs, secure API key handling

### 🤖 AI Integration

The **AIClient** supports multiple providers through a unified interface:

| Provider | Model | Features |
|----------|-------|----------|
| **OpenAI** | GPT-4o-mini | Fast, cost-effective, excellent for UI analysis |
| **Claude** | Claude 3.5 Sonnet | *Coming Soon* - Superior reasoning, detailed analysis |
| **Gemini** | *Coming Soon* | Google's multimodal AI |

**Smart Image Processing with Sharp**:
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

---

## 🎯 Global Hotkey System

### 💡 How It Works

The application uses `global-hotkey` for reliable cross-platform hotkey detection:

```typescript
// Hotkey combination: Cmd+Shift+Space (macOS) / Ctrl+Shift+Space (Windows/Linux)
const hotkey = process.platform === 'darwin' ? 'cmd+shift+space' : 'ctrl+shift+space';
await globalHotkey.register(hotkey, () => {
    this.handleHotkeyTrigger(state);
});
```

### ⌨️ Platform-Specific Hotkeys

| Platform | Hotkey | Notes |
|----------|--------|-------|
| **macOS** | `Cmd+Shift+Space` | Tested and reliable |
| **Windows** | `Ctrl+Shift+Space` | Cross-platform compatible |
| **Linux** | `Ctrl+Shift+Space` | X11 and Wayland support |

### 🔧 Debouncing & Performance

- ⏱️ **1-second debounce** prevents accidental double-triggers
- 🔄 **Event-driven** for responsive detection
- 📊 **Debug logging** for troubleshooting

---

## 📦 Dependencies

### 🚀 Core Runtime Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `commander` | 11.0+ | CLI argument parsing and command handling |
| `global-hotkey` | 1.0+ | Cross-platform global hotkey detection |
| `node-screenshot-desktop` | 1.1+ | Cross-platform screenshot capture |
| `axios` | 1.6+ | HTTP client for AI APIs |
| `sharp` | 0.32+ | High-performance image processing |

### 🎨 UI & Formatting

| Package | Version | Purpose |
|---------|---------|---------|
| `chalk` | 5.3+ | Terminal colors and formatting |
| `ora` | 7.0+ | Terminal spinners and progress indicators |
| `toml` | 3.0+ | Configuration file parsing |

### 🔧 Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `typescript` | 5.0+ | TypeScript compiler and type definitions |
| `@types/node` | 20.0+ | Node.js type definitions |
| `eslint` | 8.0+ | Code linting and quality checks |
| `prettier` | 3.0+ | Code formatting |
| `jest` | 29.0+ | Testing framework |

---

## 🚨 Troubleshooting

### 🔧 Common Issues

| Issue | Solution |
|-------|----------|
| **Hotkeys not working** | Check accessibility permissions in System Preferences |
| **Screenshot capture fails** | Verify screen recording permissions |
| **API errors** | Validate `AI_API_KEY` environment variable |
| **Build failures** | Run `npm ci` to clean install dependencies |
| **Native module errors** | Install build tools: `npm install -g node-gyp` |

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
npm run dev -- --debug run
```

---

## 🚀 Getting Started

### 📋 Prerequisites

#### 🐳 Docker Method (Recommended)
- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **AI API Key**: OpenAI or Anthropic account

#### 📦 Node.js Method
- **Node.js 18+**: [Install Node.js](https://nodejs.org/)
- **npm or yarn**: Package manager
- **AI API Key**: OpenAI or Anthropic account

### ⚡ Quick Start

#### 🐳 Using Docker (Recommended)

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd ai-screenshot-analyzer/node-version
   ```

2. **Build Docker image**:
   ```bash
   docker build -t ai-screenshot-analyzer-node .
   ```

3. **Run with API key**:
   ```bash
   docker run -it -e AI_API_KEY="your-api-key-here" ai-screenshot-analyzer-node
   ```

#### 📦 Native Installation

1. **Clone and install**:
   ```bash
   git clone <repository-url>
   cd ai-screenshot-analyzer/node-version
   npm install
   ```

2. **Build the project**:
   ```bash
   npm run build
   ```

3. **Set up API key**:
   ```bash
   export AI_API_KEY="your-api-key-here"
   ```

4. **Run the app**:
   ```bash
   npm start
   ```

5. **Grant permissions** (macOS): Follow the prompts for Accessibility and Screen Recording

6. **Test hotkey**: Press **Cmd+Shift+Space** to capture and analyze!

---

## 🎯 Performance Optimizations

### ⚡ Speed Optimizations

- **⚡ Node.js Performance**: Event-driven, non-blocking I/O
- **🔄 Async Processing**: Non-blocking screenshot capture and AI requests
- **🗜️ Sharp Image Processing**: High-performance image optimization
- **📊 Efficient Event Handling**: Optimized hotkey detection

### 💾 Memory Management

- **📸 Temporary Screenshots**: Auto-cleanup after analysis
- **🔄 Stream Processing**: Large images processed efficiently
- **♻️ Resource Cleanup**: Proper async resource management

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

## 🔄 Migration from Rust Version

This Node.js version provides identical functionality to the Rust version:

- ✅ **Same CLI interface**: All commands and options work identically
- ✅ **Same configuration**: Uses the same config file format
- ✅ **Same AI integration**: Compatible with OpenAI and future providers
- ✅ **Same hotkey system**: Cross-platform hotkey detection
- ✅ **Same image processing**: Smart format optimization

**Benefits of Node.js version**:
- 🚫 **No Rust compiler needed**: Avoids firewall issues
- 📦 **npm ecosystem**: Rich package ecosystem
- 🔄 **Faster development**: Easier to modify and extend
- 🐳 **Better Docker support**: Smaller containers, faster builds

---

## 📝 Important Notes

### 🔧 Development Guidelines

- **Always run `npm run lint`** before committing changes
- **Use `npm run format`** to format code consistently
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

- ✅ Use TypeScript for better type safety
- ✅ Handle errors gracefully with user-friendly messages
- ✅ Maintain backward compatibility in config files
- ✅ Keep the CLI interface consistent and intuitive
- ✅ Follow Node.js best practices for async operations

---

*This Node.js version provides all the functionality of the Rust version while avoiding compilation and firewall issues. Perfect for environments where Rust installation is blocked!*
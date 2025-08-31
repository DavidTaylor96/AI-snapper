# 🤖 AI Screenshot Analyzer

[![Node.js](https://img.shields.io/badge/node.js-18+-brightgreen.svg)](https://nodejs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0+-blue.svg)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> **Capture screenshots with a global hotkey and analyze them using Claude AI for instant insights and code solutions**

---

## 🚀 Quick Start

### 1. Install Dependencies
```bash
npm install
```

### 2. Set Your Claude API Key
```bash
export AI_API_KEY="sk-ant-api03-your-claude-api-key"
```

### 3. Build and Run
```bash
npm run build
npm start
```

### 4. Use the Application
- Press **Cmd+Shift+Space** (macOS) or **Ctrl+Shift+Space** (Windows/Linux)
- The app will capture your screen and analyze it with Claude AI
- Results appear in your terminal with formatted code solutions

---

## 🎯 How It Works

### The Process
1. **🔥 Always Running**: The app runs as a background daemon, waiting for your hotkey
2. **📸 Instant Capture**: Press the hotkey to capture your entire screen
3. **🤖 AI Analysis**: Your screenshot is sent to Claude AI for analysis
4. **💡 Smart Results**: Get formatted responses with code solutions, explanations, or insights
5. **⚡ Ready for More**: The app stays running for your next capture

### What Claude Can Do
- **📋 Analyze code problems** and provide working solutions
- **🐛 Debug errors** from screenshots of error messages
- **📚 Explain code** functionality and patterns
- **🎨 Describe UI elements** and design patterns
- **📊 Analyze data** from charts, graphs, or tables
- **📝 Read text** from any screenshot

### Example Workflow
```bash
# Start the app
npm start

# You'll see:
🤖 AI Screenshot Analyzer is running
Press Cmd+Shift+Space to capture and analyze screenshot
Press Ctrl+C to exit

# Press the hotkey while viewing a coding problem
# Get instant analysis like:

🤖 Claude Analysis
──────────────────────────────────────────────────

This appears to be a binary search algorithm implementation with a bug.

┌─ CODE SOLUTION (PYTHON) ────────────────────
```python
def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = (left + right) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1
```
└─────────────────────────────────────────────

The original code had an off-by-one error in the while condition.
──────────────────────────────────────────────────
```

---

## 📋 Available Commands

### Core Commands
```bash
npm start                     # 🏃 Start the daemon (default mode)
npm run dev                   # 🔄 Development mode with auto-reload

# Alternative commands
node dist/main.js run         # 🏃 Start daemon
node dist/main.js capture     # 📸 Single screenshot analysis
node dist/main.js test        # 🧪 Test Claude API connection
node dist/main.js config      # ⚙️ Show current configuration
node dist/main.js solve       # 🧩 Optimized for coding problems
```

### Advanced Usage
```bash
# Ask specific questions
npm run dev -- --question "What's wrong with this code?" run

# Debug mode with verbose logging
npm run dev -- --debug run

# Use custom API key
npm run dev -- --api-key "sk-ant-..." run
```

### Development Commands
```bash
npm run build                 # 🔨 Build TypeScript
npm test                      # 🧪 Run tests
npm run lint                  # 🔍 Check code quality
npm run format                # ✨ Format code
```

---

## ⚙️ Configuration

### Environment Variables
```bash
# Required: Your Claude API key
AI_API_KEY="sk-ant-api03-your-claude-api-key"
```

### Config File (Optional)
Location: `~/.config/ai-screenshot-analyzer/config.toml`

```toml
# Screenshot storage (temporary)
screenshots_dir = "/Users/username/.ai-screenshots"

# Image processing
image_format = "png"          # Default format (png/jpeg)
jpeg_quality = 95             # Compression quality (1-100)
max_image_size_mb = 10        # Upload size limit

# AI provider settings
default_provider = "claude"   # AI provider
```

---

## 🔧 Setup Requirements

### Prerequisites
- **Node.js 18+**: [Download Node.js](https://nodejs.org/)
- **Claude API Key**: [Get API key from Anthropic](https://console.anthropic.com/)

### macOS Permissions
The app needs these permissions to work:

1. **🔐 Accessibility Permission**:
   - Go to `System Preferences → Security & Privacy → Privacy → Accessibility`
   - Add and enable Terminal (or your terminal app)

2. **📺 Screen Recording Permission**:
   - Go to `System Preferences → Security & Privacy → Privacy → Screen Recording`
   - Add and enable Terminal (or your terminal app)

### Windows/Linux
- No special permissions required
- Hotkey: **Ctrl+Shift+Space**

---

## 🎨 Features

- ⚡ **Instant Analysis**: Press hotkey → get AI insights in seconds
- 🖼️ **Smart Image Processing**: Optimized compression for faster uploads
- 🎯 **Code-Focused**: Specialized prompts for programming problems
- 🌈 **Beautiful Output**: Formatted terminal display with syntax highlighting
- 🔄 **Always Ready**: Runs as daemon, no startup delay
- 💾 **Privacy-First**: Screenshots deleted after analysis
- 🛡️ **Secure**: API keys never logged or stored insecurely

---

## 🚨 Troubleshooting

| Issue | Solution |
|-------|----------|
| **Hotkey not working** | Check accessibility permissions in System Preferences |
| **Screenshot fails** | Verify screen recording permissions |
| **API errors** | Validate your `AI_API_KEY` environment variable |
| **Build failures** | Run `rm -rf node_modules package-lock.json && npm install` |

### Debug Mode
```bash
npm run dev -- --debug run
```

This shows detailed logging to help identify issues.

---

## 📖 Project Structure

```
ai-screenshot-analyzer/
├── src/
│   ├── main.ts              # Entry point & CLI commands
│   ├── ai_client.ts         # Claude API integration
│   ├── hotkey_monitor.ts    # Global hotkey detection
│   ├── screenshot.ts        # Screen capture & processing
│   ├── config.ts           # Configuration management
│   └── ui.ts               # Terminal UI formatting
├── test/                   # Test suite
├── package.json           # Dependencies & scripts
└── tsconfig.json         # TypeScript configuration
```

---

*Built with Node.js and TypeScript for excellent performance and easy deployment.*
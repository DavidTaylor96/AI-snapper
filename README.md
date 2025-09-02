# 🤖 AI Screenshot Analyzer

[![Node.js](https://img.shields.io/badge/node.js-18+-brightgreen.svg)](https://nodejs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0+-blue.svg)](https://www.typescriptlang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> **Capture screenshots and analyze them using Claude AI - no special permissions required!**

---

## 🚀 Quick Start (30 Seconds)

### 1. Install & Build
```bash
npm install
npm run build
```

### 2. Set Your Claude API Key
```bash
export AI_API_KEY="sk-ant-api03-your-claude-api-key"
```

### 3. Run (No Permissions Needed!)
```bash
npm start
```

### 4. Press Space or Enter to Capture!
That's it! Just press **Space** or **Enter** in your terminal to capture and analyze your screen.

---

## 🎯 How It Works

### The Magic
1. **🏃 Run the app** - It waits in your terminal
2. **⌨️ Press Space/Enter** - No special permissions needed!
3. **📸 Captures your screen** - Uses native screenshot tools
4. **🤖 Claude analyzes it** - Get instant insights
5. **💡 See the results** - Formatted code, explanations, solutions

### What Can It Do?
- **🐛 Debug errors** - Screenshot an error, get the fix
- **📋 Solve coding problems** - Capture leetcode, get solutions
- **📚 Explain code** - Understand what code does
- **🎨 Analyze UIs** - Get feedback on designs
- **📊 Read charts/data** - Extract info from images
- **📝 OCR text** - Convert screenshots to text

---

## 📺 Input Modes (Choose Your Style)

### 1. **Terminal Mode** (Default - No Permissions!)
```bash
npm start
# Just press Space or Enter to capture!
```

**Controls:**
- `Space` / `Enter` → Capture & Analyze
- `s` → Solve coding problem
- `e` → Explain what's on screen  
- `q` → Ask custom question
- `h` → Show help
- `Ctrl+C` → Exit

### 2. **Command Mode** (Type Commands)
```bash
npm start --mode command
# Type commands like "capture" or "solve"
```

**Commands:**
- `capture` or just `Enter` → Capture screen
- `solve` → Analyze as coding problem
- `explain` → Describe what's visible
- `ask <question>` → Ask specific question
- `repeat` → Repeat last capture
- `exit` → Quit

### 3. **Timer Mode** (Auto-Capture)
```bash
npm start --mode timer --interval 5
# Automatically captures every 5 seconds
```

**Controls:**
- `p` → Pause/Resume
- `n` → Capture now
- `Ctrl+C` → Exit

### 4. **Hotkey Mode** (Optional - Requires Permissions)
```bash
npm start --mode hotkey
# Falls back to terminal mode if permissions missing
```

If you have accessibility permissions:
- **macOS**: `Cmd+Shift+Space`
- **Windows/Linux**: `Ctrl+Shift+Space`

---

## 💻 Usage Examples

### Basic Capture
```bash
# Start and press Space when ready
npm start

# You'll see:
📸 Capturing screenshot...
🤖 Analyzing with AI...

[Claude's analysis appears here]
```

### Ask Specific Questions
```bash
# Start with a question
npm start -q "What's wrong with this code?"

# Or in interactive mode, press 'q' then type your question
```

### Solve Coding Problems
```bash
# One-shot solve
npm run solve

# Or in interactive mode, press 's'
```

### Different Analysis Modes
```bash
# Explain what's on screen
npm start --prompt "Explain this UI design"

# Debug an error
npm start -q "How do I fix this error?"

# Extract text
npm start -q "What text is in this image?"
```

---

## ⚙️ Configuration

### Environment Variables
```bash
# Required
AI_API_KEY="sk-ant-api03-your-claude-api-key"

# Optional
SCREENSHOT_MODE="terminal"  # terminal, command, timer, hotkey
AUTO_CAPTURE_INTERVAL="5"   # Seconds for timer mode
```

### Config File
Location: `~/.config/ai-screenshot-analyzer/config.toml`

```toml
# Screenshot settings
screenshots_dir = "~/.ai-screenshots"
image_format = "png"
jpeg_quality = 95
max_image_size_mb = 10

# AI settings  
default_provider = "claude"

# Input settings
default_mode = "terminal"
auto_capture_interval = 5
```

---

## 🔧 Installation

### Prerequisites
- **Node.js 18+**: [Download](https://nodejs.org/)
- **Claude API Key**: [Get one](https://console.anthropic.com/)

### Install from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/ai-screenshot-analyzer
cd ai-screenshot-analyzer

# Install dependencies
npm install

# Build
npm run build

# Set API key
export AI_API_KEY="your-api-key"

# Run!
npm start
```

### Optional: Global Installation
```bash
# Install globally
npm install -g .

# Run from anywhere
ai-screenshot-analyzer
```

### Optional: Create Desktop Shortcut
```bash
# macOS
cat > ~/Desktop/AI-Screenshot.command << 'EOF'
#!/bin/bash
cd /path/to/ai-screenshot-analyzer
npm start
EOF
chmod +x ~/Desktop/AI-Screenshot.command

# Windows (create .bat file)
echo "cd C:\path\to\ai-screenshot-analyzer && npm start" > Desktop\AI-Screenshot.bat
```

---

## 🎨 Features

### Core Features
- ⚡ **No Permissions Required** - Works in any terminal
- 🎯 **Multiple Input Modes** - Terminal, commands, timer, or hotkeys
- 🖼️ **Smart Compression** - Optimizes images for fast upload
- 🤖 **Claude 3.5 Sonnet** - Latest AI model for best results
- 🎨 **Beautiful Output** - Syntax-highlighted code in terminal
- 🔒 **Privacy-First** - Screenshots deleted after analysis
- 🌈 **Cross-Platform** - Works on macOS, Windows, Linux

### Advanced Features
- 📝 **Custom Questions** - Ask specific questions about screenshots
- 🔄 **Repeat Capture** - Quickly re-analyze with different prompts
- ⏱️ **Auto-Capture** - Timer mode for presentations/tutorials
- 🎯 **Specialized Modes** - Optimized for coding, debugging, explaining
- 📊 **Smart Detection** - Automatically identifies code vs. UI vs. text

---

## 🚨 Troubleshooting

### Common Issues

| Problem | Solution |
|---------|----------|
| **"API key required"** | Set `export AI_API_KEY="sk-ant-..."` |
| **Terminal not responding** | Make sure terminal has focus, try `--mode command` |
| **Screenshot fails** | Check if `screenshot-desktop` is installed |
| **"Cannot find module"** | Run `npm install` then `npm run build` |
| **Hotkey not working** | Use default terminal mode instead (no setup needed!) |

### Platform-Specific

#### macOS
- If screenshots are black, grant Terminal screen recording permission
- System Preferences → Security & Privacy → Privacy → Screen Recording

#### Windows  
- Run terminal as administrator if screenshot fails
- Terminal mode works without admin rights

#### Linux
- May need to install: `sudo apt-get install imagemagick`
- Wayland users: might need `XDG_SESSION_TYPE=x11`

---

## 📁 Project Structure

```
ai-screenshot-analyzer/
├── src/
│   ├── main.ts              # Entry point & CLI
│   ├── terminal_monitor.ts  # Terminal input handler (NEW!)
│   ├── ai_client.ts         # Claude API integration
│   ├── screenshot.ts        # Screen capture
│   ├── config.ts           # Configuration
│   ├── ui.ts               # Terminal UI formatting
│   └── hotkey_monitor.ts   # Optional hotkey support
├── dist/                   # Compiled JavaScript
├── package.json           # Dependencies
└── tsconfig.json         # TypeScript config
```

---

## 🤝 Contributing

Contributions are welcome! Feel free to:
- 🐛 Report bugs
- 💡 Suggest features  
- 🔧 Submit pull requests

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file

---

## 🙏 Credits

Built with:
- [Claude 3.5 Sonnet](https://www.anthropic.com/claude) by Anthropic
- [screenshot-desktop](https://github.com/bencevans/screenshot-desktop) for captures
- [chalk](https://github.com/chalk/chalk) for beautiful terminal output
- [commander](https://github.com/tj/commander.js) for CLI
- [sharp](https://sharp.pixelplumbing.com/) for image optimization

---

## 🎯 Quick Commands Reference

```bash
# Basic usage
npm start                    # Start with terminal input mode
npm start --mode command     # Use command mode
npm start --mode timer       # Auto-capture every 5 seconds

# With options
npm start -q "What is this?" # Start with a question
npm start --interval 10      # Timer mode, 10-second intervals

# One-shot commands  
npm run capture              # Single capture and exit
npm run solve                # Capture and solve coding problem
npm run test                 # Test API connection

# Development
npm run build                # Compile TypeScript
npm run dev                  # Development mode
npm test                     # Run tests
```

---

*Made with ❤️ for developers who love keyboard shortcuts but hate permission dialogs*
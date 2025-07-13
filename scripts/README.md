# Automation Tests for AI Screenshot Analyzer

This directory contains automation test scripts to verify the complete functionality of the AI Screenshot Analyzer application.

## Available Tests

### 1. Python Automation Test (`automation_test.py`)

A comprehensive Python script that:
- Builds and starts the application
- Simulates the Cmd+Shift+S hotkey
- Waits for AI response
- Validates results and provides detailed logging

**Requirements:**
```bash
pip install -r requirements.txt
```

**Usage:**
```bash
# Basic test with OpenAI
python3 scripts/automation_test.py

# Test with Claude provider and custom timeout
python3 scripts/automation_test.py --provider claude --timeout 90

# Show help
python3 scripts/automation_test.py --help
```

### 2. Bash Automation Test (`automation_test.sh`)

A simpler bash script that performs the same core functionality:

**Usage:**
```bash
# Basic test
./scripts/automation_test.sh

# Test with different provider and timeout
./scripts/automation_test.sh claude 45

# Show help
./scripts/automation_test.sh --help
```

### 3. Rust Integration Tests (`tests/test_automation.rs`)

Rust-based integration tests that test the core functionality without requiring hotkey simulation:

**Usage:**
```bash
# Run end-to-end test (requires API key)
cargo test test_end_to_end_automation --ignored

# Run all automation tests
cargo test automation --ignored

# Run performance test
cargo test test_automation_performance --ignored
```

## Prerequisites

### Environment Setup

1. **API Key**: Set your AI API key in the environment:
   ```bash
   export AI_API_KEY=your_openai_api_key_here
   ```
   Or create/update the `.env` file:
   ```
   AI_API_KEY=your_openai_api_key_here
   ```

2. **macOS Permissions**: Ensure the terminal has the required permissions:
   - **Accessibility**: For hotkey simulation
   - **Screen Recording**: For screenshot capture

3. **Dependencies**:
   - Rust and Cargo installed
   - Python 3.7+ (for Python script)
   - Required Python packages: `pip install psutil pynput`

### Permission Setup

When running for the first time, you'll need to grant permissions:

1. **Accessibility Permission**:
   - System Preferences → Security & Privacy → Privacy → Accessibility
   - Add your Terminal app and enable it

2. **Screen Recording Permission**:
   - System Preferences → Security & Privacy → Privacy → Screen Recording  
   - Add your Terminal app and enable it

## Test Features

### What Each Test Validates

- ✅ Application builds successfully
- ✅ Application starts without errors
- ✅ Hotkey simulation works (scripts only)
- ✅ Screenshot capture functions
- ✅ AI API integration works
- ✅ Response is received within timeout
- ✅ Response contains meaningful content
- ✅ Application cleanup works properly

### Test Output

All tests provide detailed logging with timestamps and status indicators:
- 🔨 Build steps
- 🚀 Application startup
- ⌨️ Hotkey simulation
- 📸 Screenshot capture
- 🤖 AI processing
- ✅ Success indicators
- ❌ Error indicators
- 📊 Test validation

## Example Test Run

```bash
$ ./scripts/automation_test.sh openai 60

[14:30:15] 🧪 Starting AI Screenshot Analyzer Automation Test
[14:30:15] ==========================================
[14:30:15] 🔍 Checking requirements...
[14:30:15] ✅ Requirements check passed
[14:30:15] 🔨 Building application...
[14:30:22] ✅ Build successful
[14:30:22] 🚀 Starting application with provider: openai
[14:30:22] Application started with PID: 12345
[14:30:27] ✅ Application started successfully
[14:30:30] ⌨️ Simulating Cmd+Shift+S hotkey...
[14:30:30] ✅ Hotkey simulation successful
[14:30:30] ⏳ Waiting for AI response (timeout: 60s)...
[14:30:45] ✅ AI response received in 15 seconds!
[14:30:45] 📊 Validating test results...
[14:30:45] ✅ Test validation successful
[14:30:45] ✅ AUTOMATION TEST PASSED!
```

## Troubleshooting

### Common Issues

1. **Permission Denied Errors**: 
   - Grant Accessibility and Screen Recording permissions
   - Restart terminal after granting permissions

2. **API Key Errors**:
   - Verify API key is set correctly
   - Check API key has sufficient credits/quota

3. **Build Failures**:
   - Ensure Rust is properly installed
   - Run `cargo clean` and try again

4. **Timeout Issues**:
   - Increase timeout parameter
   - Check network connectivity
   - Verify API service is available

### Debug Mode

For more detailed output, check the log files:
- Bash script: `/tmp/ai_screenshot_test.log`
- Python script: Console output with verbose flag

## CI/CD Integration

These tests can be integrated into CI/CD pipelines with some modifications:
- Use headless testing environments
- Mock AI API responses for unit tests
- Set up virtual displays for screenshot testing

## Contributing

When adding new automation tests:
1. Follow the existing patterns for logging and error handling
2. Include proper cleanup in all test scenarios
3. Add timeout mechanisms to prevent hanging tests
4. Document any new requirements or setup steps
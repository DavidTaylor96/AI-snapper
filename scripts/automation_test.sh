#!/bin/bash
# AI Screenshot Analyzer Automation Test Script
# 
# This script runs a simplified automation test for the AI Screenshot Analyzer
# Usage: ./scripts/automation_test.sh [provider] [timeout]

set -e

# Configuration
PROVIDER=${1:-"openai"}
TIMEOUT=${2:-60}
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_FILE="/tmp/ai_screenshot_test.log"
APP_PID=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log() {
    echo -e "${BLUE}[$(date '+%H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[$(date '+%H:%M:%S')] ✅ $1${NC}" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[$(date '+%H:%M:%S')] ❌ $1${NC}" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[$(date '+%H:%M:%S')] ⚠️  $1${NC}" | tee -a "$LOG_FILE"
}

cleanup() {
    log "🧹 Cleaning up..."
    if [ ! -z "$APP_PID" ] && kill -0 "$APP_PID" 2>/dev/null; then
        log "Terminating application (PID: $APP_PID)"
        kill "$APP_PID" 2>/dev/null || true
        sleep 2
        # Force kill if still running
        if kill -0 "$APP_PID" 2>/dev/null; then
            kill -9 "$APP_PID" 2>/dev/null || true
        fi
    fi
    
    # Clean up any remaining cargo processes
    pkill -f "ai-screenshot-analyzer" 2>/dev/null || true
    
    log_success "Cleanup completed"
}

# Set up signal handlers
trap cleanup EXIT INT TERM

check_requirements() {
    log "🔍 Checking requirements..."
    
    # Check if we're on macOS
    if [[ "$OSTYPE" != "darwin"* ]]; then
        log_error "This test is designed for macOS only"
        exit 1
    fi
    
    # Check for API key
    if [ -z "$AI_API_KEY" ]; then
        log_error "AI_API_KEY environment variable not set"
        log "Set it with: export AI_API_KEY=your_key"
        exit 1
    fi
    
    # Check for cargo
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo not found. Please install Rust"
        exit 1
    fi
    
    log_success "Requirements check passed"
}

build_application() {
    log "🔨 Building application..."
    cd "$PROJECT_DIR"
    
    if cargo build --release >> "$LOG_FILE" 2>&1; then
        log_success "Build successful"
        return 0
    else
        log_error "Build failed. Check $LOG_FILE for details"
        return 1
    fi
}

start_application() {
    log "🚀 Starting application with provider: $PROVIDER"
    cd "$PROJECT_DIR"
    
    # Start application in background
    cargo run --release -- --provider "$PROVIDER" run >> "$LOG_FILE" 2>&1 &
    APP_PID=$!
    
    log "Application started with PID: $APP_PID"
    
    # Wait for app to initialize
    log "⏳ Waiting for application to initialize..."
    sleep 5
    
    # Check if process is still running
    if ! kill -0 "$APP_PID" 2>/dev/null; then
        log_error "Application failed to start"
        return 1
    fi
    
    log_success "Application started successfully"
    return 0
}

simulate_hotkey() {
    log "⌨️ Simulating Cmd+Shift+S hotkey..."
    
    # Use AppleScript to simulate the hotkey
    osascript -e 'tell application "System Events" to key code 1 using {command down, shift down}' 2>/dev/null
    
    if [ $? -eq 0 ]; then
        log_success "Hotkey simulation successful"
        return 0
    else
        log_error "Hotkey simulation failed"
        return 1
    fi
}

wait_for_response() {
    log "⏳ Waiting for AI response (timeout: ${TIMEOUT}s)..."
    
    local start_time=$(date +%s)
    local response_found=false
    
    while [ $(($(date +%s) - start_time)) -lt "$TIMEOUT" ]; do
        # Check if application is still running
        if ! kill -0 "$APP_PID" 2>/dev/null; then
            log_error "Application process terminated unexpectedly"
            return 1
        fi
        
        # Check log for AI response indicators
        if grep -q "💡 Analysis Result:\|✅" "$LOG_FILE" 2>/dev/null; then
            local elapsed=$(($(date +%s) - start_time))
            log_success "AI response received in ${elapsed} seconds!"
            response_found=true
            break
        fi
        
        # Check for errors
        if grep -q "❌.*failed\|error" "$LOG_FILE" 2>/dev/null; then
            log_warning "Error detected in application output"
        fi
        
        sleep 2
    done
    
    if [ "$response_found" = true ]; then
        return 0
    else
        log_error "Timeout reached (${TIMEOUT}s) - no AI response"
        return 1
    fi
}

validate_results() {
    log "📊 Validating test results..."
    
    # Check for success indicators in log
    if grep -q "💡 Analysis Result:\|✅" "$LOG_FILE"; then
        log_success "Success indicators found in output"
    else
        log_error "No success indicators found in output"
        return 1
    fi
    
    # Check for critical errors
    if grep -q "❌.*failed\|fatal error" "$LOG_FILE"; then
        log_warning "Warning: Error indicators found in output"
    fi
    
    log_success "Test validation successful"
    return 0
}

print_summary() {
    echo
    log "=" | tr ' ' '='
    log "📋 TEST SUMMARY"
    log "=" | tr ' ' '='
    log "Provider: $PROVIDER"
    log "Timeout: ${TIMEOUT}s"
    log "Log File: $LOG_FILE"
    
    if grep -q "💡 Analysis Result:\|✅" "$LOG_FILE" 2>/dev/null; then
        log_success "AI Response: Received"
    else
        log_error "AI Response: Not received"
    fi
    
    echo
    log "📝 Last 10 lines of application output:"
    tail -10 "$LOG_FILE" 2>/dev/null | sed 's/^/  /' || log "No output available"
}

main() {
    # Clear log file
    > "$LOG_FILE"
    
    log "🧪 Starting AI Screenshot Analyzer Automation Test"
    log "=" | tr ' ' '='
    
    # Run test steps
    check_requirements || exit 1
    build_application || exit 1
    start_application || exit 1
    
    # Small delay before triggering hotkey
    sleep 3
    
    simulate_hotkey || exit 1
    wait_for_response || exit 1
    validate_results || exit 1
    
    log_success "AUTOMATION TEST PASSED!"
    print_summary
    
    exit 0
}

# Show usage if help requested
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Usage: $0 [provider] [timeout]"
    echo "  provider: openai, claude, or gemini (default: openai)"
    echo "  timeout:  timeout in seconds (default: 60)"
    echo
    echo "Environment variables:"
    echo "  AI_API_KEY: Required API key for the AI provider"
    echo
    echo "Example:"
    echo "  $0 openai 30"
    exit 0
fi

main "$@"
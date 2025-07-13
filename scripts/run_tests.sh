#!/bin/bash
# Test runner script for AI Screenshot Analyzer automation tests

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🧪 AI Screenshot Analyzer Test Runner${NC}"
echo "========================================"

# Check if API key is set
if [ -z "$AI_API_KEY" ]; then
    echo -e "${YELLOW}⚠️  AI_API_KEY not set. Only running unit tests...${NC}"
    RUN_INTEGRATION=false
else
    echo -e "${GREEN}✅ AI_API_KEY found. Running all tests...${NC}"
    RUN_INTEGRATION=true
fi

echo
echo -e "${BLUE}1. Running Unit Tests${NC}"
echo "--------------------"
cargo test --lib

echo
echo -e "${BLUE}2. Running Integration Tests${NC}" 
echo "----------------------------"
cargo test --test integration_tests
cargo test --test test_ai_client
cargo test --test test_config
cargo test --test test_daemon
cargo test --test test_hotkeys
cargo test --test test_main
cargo test --test test_screenshot
cargo test --test test_ui

echo
echo -e "${BLUE}3. Running Automation Helper Tests${NC}"
echo "-----------------------------------"
cargo test automation_test_helpers

if [ "$RUN_INTEGRATION" = true ]; then
    echo
    echo -e "${BLUE}4. Running Integration Tests (with API)${NC}"
    echo "---------------------------------------"
    cargo test test_configuration_validation --test test_automation
    
    echo
    echo -e "${YELLOW}⚠️  Skipping full automation tests (requires manual interaction)${NC}"
    echo "To run full automation tests manually:"
    echo "  ./scripts/automation_test.sh"
    echo "  python3 scripts/automation_test.py"
else
    echo
    echo -e "${YELLOW}4. Skipping API-dependent tests (no AI_API_KEY)${NC}"
fi

echo
echo -e "${GREEN}✅ Test run completed!${NC}"
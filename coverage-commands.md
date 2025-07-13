# Code Coverage Commands

## Quick Commands

```bash
# Generate all coverage formats
cargo tarpaulin --out Html --out Xml --out Lcov --output-dir coverage

# Run the coverage script
./scripts/coverage.sh

# View HTML report
open coverage/tarpaulin-report.html
```

## Current Coverage Summary

- **Overall Coverage**: 7.02% (20/285 lines covered)
- **Test Files**: 5 integration tests passing

### Per-Module Coverage:
- `src/ai_client.rs`: 0/105 lines (0.00%) - AI API integration
- `src/config.rs`: 5/18 lines (27.78%) - Configuration management  
- `src/daemon.rs`: 0/32 lines (0.00%) - Daemon and hotkey handling
- `src/main.rs`: 0/45 lines (0.00%) - CLI and main entry point
- `src/screenshot.rs`: 15/46 lines (32.61%) - Screenshot capture logic
- `src/ui.rs`: 0/39 lines (0.00%) - Terminal UI functions

## Coverage Files Generated

1. **HTML Report**: `coverage/tarpaulin-report.html` - Interactive web report
2. **XML Report**: `coverage/cobertura.xml` - For CI/CD integration
3. **LCOV Report**: `coverage/lcov.info` - For editors and external tools

## Improving Coverage

The low coverage is expected because:
1. Tests focus on integration testing with mocks
2. Main application code (CLI, daemon, UI) isn't directly tested
3. AI client uses external API calls that are mocked

To improve coverage, consider adding:
- Unit tests for individual modules
- Tests for CLI argument parsing
- Mock tests for daemon functionality
- UI component testing
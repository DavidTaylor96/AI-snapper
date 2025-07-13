#!/bin/bash

# AI Screenshot Analyzer - Code Coverage Script
set -e

echo "🧪 Running code coverage analysis..."

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "📦 Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

# Clean previous coverage data
echo "🧹 Cleaning previous coverage data..."
rm -rf coverage/
rm -f cobertura.xml lcov.info tarpaulin-report.html

# Run tests with coverage
echo "🔍 Generating coverage report..."
cargo tarpaulin --config tarpaulin.toml

# Display summary
echo ""
echo "✅ Coverage analysis complete!"
echo "📊 Reports generated:"
echo "   - HTML: coverage/tarpaulin-report.html"
echo "   - LCOV: lcov.info"
echo "   - XML: cobertura.xml"
echo ""
echo "💡 Open coverage/tarpaulin-report.html in your browser to view the detailed report"

# Check if we're in a CI environment
if [ "${CI}" = "true" ]; then
    echo "🤖 CI environment detected - coverage data available for upload"
fi
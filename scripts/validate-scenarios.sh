#!/bin/bash
# Scenario Validation Script
# Run this after agent completes development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "=========================================="
echo "  Quality Gate - Scenario Validation"
echo "=========================================="

cd "$PROJECT_ROOT"

# Check arguments
SCENARIOS_PATH="${1:-../scenarios-vault/flow-wasm}"
APP_ENDPOINT="${2:-http://localhost:8080}"
FEEDBACK_LEVEL="${3:-3}"

if [ ! -d "$SCENARIOS_PATH" ]; then
	echo "❌ Scenarios directory not found: $SCENARIOS_PATH"
	exit 1
fi

echo "📂 Scenarios: $SCENARIOS_PATH"
echo "🎯 App endpoint: $APP_ENDPOINT"
echo "📊 Feedback level: $FEEDBACK_LEVEL"

# Run scenario validation
cargo run --bin scenario-runner -- \
	--scenarios-path "$SCENARIOS_PATH" \
	--app-endpoint "$APP_ENDPOINT" \
	--level "$FEEDBACK_LEVEL"

echo ""
echo "✅ Scenario validation passed!"

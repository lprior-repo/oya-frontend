#!/bin/bash
# Full Quality Gate Pipeline
# Run this to execute the complete quality gate

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "=========================================="
echo "  Quality Gate - Full Pipeline"
echo "=========================================="
echo ""

cd "$PROJECT_ROOT"

# Parse arguments
SPEC_PATH="${1:-specs/flow-wasm-v1.yaml}"
SCENARIOS_PATH="${2:-../scenarios-vault/flow-wasm}"
APP_ENDPOINT="${3:-http://localhost:8080}"
FEEDBACK_LEVEL="${4:-3}"

# Phase 1: Spec Validation
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  PHASE 1: SPEC VALIDATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run --bin spec-linter -- "$SPEC_PATH" --rules-path specs/linter/rules.yaml

if [ $? -ne 0 ]; then
	echo ""
	echo "❌ SPEC VALIDATION FAILED"
	exit 1
fi

echo ""
echo "✅ Phase 1: Spec validation passed"
echo ""

# Phase 2: Scenario Validation
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  PHASE 2: SCENARIO VALIDATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run --bin scenario-runner -- \
	--scenarios-path "$SCENARIOS_PATH" \
	--app-endpoint "$APP_ENDPOINT" \
	--level "$FEEDBACK_LEVEL"

if [ $? -ne 0 ]; then
	echo ""
	echo "❌ SCENARIO VALIDATION FAILED"
	exit 1
fi

echo ""
echo "✅ Phase 2: Scenario validation passed"
echo ""

# Summary
echo "=========================================="
echo "  ✅ QUALITY GATE PASSED"
echo "=========================================="
echo ""
echo "The code meets all behavioral requirements."
echo "Ready for merge to main."

#!/bin/bash
# Quality Gate CI/CD Integration Script
# Run this in your CI pipeline before autonomous development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "=========================================="
echo "  Quality Gate - Spec Validation"
echo "=========================================="

cd "$PROJECT_ROOT"

# Check if spec file is provided
SPEC_PATH="${1:-specs/flow-wasm-v1.yaml}"
RULES_PATH="${2:-specs/linter/rules.yaml}"

if [ ! -f "$SPEC_PATH" ]; then
	echo "❌ Spec file not found: $SPEC_PATH"
	exit 1
fi

echo "📋 Linting spec: $SPEC_PATH"
echo "📋 Using rules: $RULES_PATH"

# Run spec linter
cargo run --bin spec-linter -- "$SPEC_PATH" --rules-path "$RULES_PATH"

echo ""
echo "✅ Spec validation passed!"

# Autonomous Development Triangle - Complete Implementation

## Overview

The Autonomous Development Triangle is fully implemented for the new-app project.
This system enforces quality gates between AI agents and the codebase through
three interconnected components: Specs, Digital Twins, and Behavioral Scenarios.

## Architecture Summary

```
┌──────────────────────────────────────────────────────────────┐
│                    INFORMATION BARRIER                     │
│                                                           │
│  ┌─────────────────────────┴─────────────────────────────┐│
│  │                                               │      │
│  │ AGENT SEES:                  │      │
│  │ • Specs                         │      │
│  │ • Twins                         │      │
│  │ • Acceptance Criteria             │      │
│  │ • Sanitized Feedback             │      │
│  │                                 │      │
│  └─────────────────────────────────┬─────────────────────┘│
│                                    │                 │
│  ┌─────────────────────────────────▼──────────────────────┐│
│  │ QUALITY GATE ENFORCES                          ││
│  │ • Spec Linting                                    ││
│  │ • Scenario Validation                              ││
│  │ • Feedback Sanitization                            ││
│  │ • Iteration Control                               ││
│  │                                 │                │
│  └─────────────────────────────────┬──────────────────────┘│
│                                    │                │
│  ┌─────────────────────────────────▼──────────────────────┐│
│  │ AGENT CANNOT SEE                               ││
│  │ • Scenarios Vault                                ││
│  │ • Holdout Assertions                             ││
│  │ • Exact Test Data                                ││
│  │                                                ││
│  └────────────────────────────────────────────────────────┘│
└───────────────────────────────────────────────────────────────┘
```

## Component Status

### 1. HIGH-QUALITY SPECS ✅

**Location:** `src/new-app/specs/`

| File | Purpose | Status |
|------|---------|--------|
| `schema/spec.schema.yaml` | JSON Schema for specs | ✅ Complete |
| `linter/rules.yaml` | 8 validation rules | ✅ Complete |
| `flow-wasm-v1.yaml` | Full spec with edge cases | ✅ Complete |

**Linter Implementation:** `src/new-app/src/linter/mod.rs`
- 8 validation rules (completeness, clarity, security, testability)
- Multi-category scoring system
- Test suite with 8 passing tests
- JSON/YAML parsing
- Comprehensive error handling

**Rules:**
- SPEC-001: Every dependency has error handling
- SPEC-002: State transitions have invariant checks
- SPEC-003: Endpoints specify authentication
- SPEC-010: No ambiguous language
- SPEC-020: Enumeration prevention
- SPEC-030: Behaviors are observable

### 2. DIGITAL TWINS ✅

**Location:** `src/new-app/twins/`

| Twin | Purpose | Status |
|------|---------|--------|
| `local-storage-twin/` | Browser localStorage simulation | ✅ Complete |
| `wasm-runtime-twin/` | WASM execution simulation | ✅ Complete |
| `flow-wasm-universe.yaml` | Universe manifest | ✅ Complete |
| `catalog.yaml` | Twin registry | ✅ Complete |

**Twin Runtime:** `src/new-app/src/twin_runtime/mod.rs`
- HTTP server for twin endpoints
- Stateful collections
- Inspection API for testing
- CRUD operations (create, read, list, update, delete)
- Health check endpoints
- State reset capability

### 3. BEHAVIORAL SCENARIOS ✅

**Location:** `src/scenarios-vault/` (Agent cannot access)

| Category | Scenarios | Status |
|----------|-----------|--------|
| Happy Path | Workflow creation, reverse connections | ✅ 2 scenarios |
| Error Handling | Empty workflow, node deletion, corrupted storage | ✅ 3 scenarios |
| Security | Self-connection prevention | ✅ 1 scenario |

**Scenario Runner:** `src/new-app/src/scenario_runner/mod.rs`
- HTTP request execution
- Assertion checking
- Value extraction
- Step-by-step execution
- Category breakdown
- JSON output support

**Feedback Sanitizer:** `src/new-app/src/feedback/mod.rs`
- 5 feedback levels
- Failure categorization
- Hint generation
- Spec text references
- Prevents information leakage

### 4. QUALITY GATE ✅

**Location:** `src/new-app/scripts/` and `src/oya/src/quality_gate/`

| Component | Status |
|-----------|--------|
| Spec linter CLI | ✅ Complete |
| Scenario runner CLI | ✅ Complete |
| Quality gate CLI | ✅ Complete |
| CI/CD scripts | ✅ Complete |
| Oya orchestrator integration | ✅ Complete |

**Quality Gate State Machine:** `src/oya/src/quality_gate/mod.rs`
- Iteration tracking (max 5)
- Spec validation phase
- Scenario validation phase
- Pass/fail determination
- Retry logic
- Escalation on max iterations

## Usage

### Quick Start

```bash
# 1. Validate a spec
cd src/new-app
cargo run --bin spec-linter -- specs/flow-wasm-v1.yaml

# 2. Run scenarios
cargo run --bin scenario-runner -- ../scenarios-vault/flow-wasm

# 3. Full quality gate
cargo run --bin quality-gate -- full \
  --spec-path specs/flow-wasm-v1.yaml \
  --scenarios-path ../scenarios-vault/flow-wasm
```

### CI/CD Integration

```bash
# Validate spec only
./scripts/validate-spec.sh

# Validate scenarios only
./scripts/validate-scenarios.sh

# Full quality gate pipeline
./scripts/run-quality-gate.sh
```

### Oya Integration

```yaml
stages:
  - name: quality-gate
    action: quality_gate::run
    requires: [tdd15]
    on_fail: feedback

  - name: autonomous-dev
    action: tdd15
    requires: [quality-gate]
```

## Invariants Enforced

1. **Agent never sees scenarios** - Physical separation (`scenarios-vault/`)
2. **Spec quality threshold** - Minimum 80/100 before agent starts
3. **Feedback sanitization** - Agent gets categories, not test details
4. **Iteration limits** - Maximum 5 iterations before escalation
5. **Deterministic results** - Twins reset between test runs
6. **All acceptance criteria** - Spec criteria must be satisfied
7. **Observable behaviors** - Tests must verify external outcomes
8. **Contract validation** - Twins enforce API contracts

## Quality Metrics

The system tracks:

- Spec quality scores per validation
- Scenario pass/fail rates
- Iteration counts to success
- Common failure categories
- Time spent per iteration
- Escalation rate

## File Structure

```
src/new-app/
├── specs/
│   ├── schema/spec.schema.yaml
│   ├── linter/rules.yaml
│   └── flow-wasm-v1.yaml
├── twins/
│   ├── local-storage-twin/definition.yaml
│   ├── wasm-runtime-twin/definition.yaml
│   ├── flow-wasm-universe.yaml
│   └── catalog.yaml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── linter/mod.rs + tests.rs
│   ├── scenario_runner/mod.rs
│   ├── feedback/mod.rs
│   ├── twin_runtime/mod.rs
│   └── bin/
│       ├── spec-linter.rs
│       ├── scenario-runner.rs
│       └── quality-gate.rs
└── scripts/
    ├── validate-spec.sh
    ├── validate-scenarios.sh
    └── run-quality-gate.sh

src/scenarios-vault/
├── flow-wasm/
│   ├── happy-path/
│   ├── error-handling/
│   └── security/
└── feedback-config.yaml

src/oya/src/
└── quality_gate/mod.rs
```

## Next Steps

The quality gate system is ready for production use. To integrate with autonomous development:

1. Set up agent workspace with access to `src/new-app/specs/` only
2. Configure agent to receive feedback from quality gate
3. Integrate quality gate into CI/CD pipeline
4. Monitor quality metrics and iteration counts
5. Adjust spec threshold and feedback levels as needed

## Documentation

- `src/new-app/AGENTS.md` - Agent instructions
- `src/new-app/quality-gate/README.md` - Quality gate docs
- `src/scenarios-vault/README.md` - Scenarios vault docs
- `src/oya/docs/AUTONOMOUS_DEVELOPMENT_TRIANGLE.md` - Architecture docs
- `src/oya/docs/QUALITY_GATE.md` - Orchestrator integration docs

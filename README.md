# Autonomous Development Triangle - Complete Implementation

## Overview

This directory contains the complete implementation of the Autonomous Development Triangle for the new-app project. The system enforces quality gates between AI agents and the codebase through three interconnected components: High-Quality Specs, Digital Twins, and Behavioral Scenarios.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  INFORMATION BARRIER                     │
│                                                           │
│  AGENT SEES (can access):       │  HOLDOUT (cannot access):       │
│  ┌────────────────────────┐      │  ┌──────────────────────────┐    │
│  │ • Specs               │      │  │ • Scenarios             │    │
│  │ • Twins               │      │  │ • Holdout Assertions     │    │
│  │ • Acceptance Criteria   │      │  │ • Exact Test Data        │    │
│  │ • Own Tests           │      │  └──────────────────────────┘    │
│  └────────────────────────┘      │                              │
│                                                           │
│              QUALITY GATE ENFORCES:                     │
│  ┌───────────────────────────────────────────────────────┐│
│  │ Spec Linter → Twin Manager → Scenario Runner →    ││
│  │ Feedback Sanitizer → Agent Feedback →              ││
│  └───────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. High-Quality Specs (`specs/`)
- `schema/spec.schema.yaml` - JSON Schema for specifications
- `linter/rules.yaml` - 8 validation rules (completeness, clarity, security, testability)
- `flow-wasm-v1.yaml` - Enhanced specification with edge cases
- **Implementation:** `src/linter/mod.rs`

**Quality Rules:**
- SPEC-001: Every dependency has error handling
- SPEC-002: State transitions have invariant checks
- SPEC-003: Endpoints specify authentication
- SPEC-010: No ambiguous language
- SPEC-020: Enumeration prevention
- SPEC-030: Behaviors are observable

**Usage:**
```bash
cargo run --bin spec-linter -- specs/flow-wasm-v1.yaml
cargo run --bin spec-linter -- specs/flow-wasm-v1.yaml --format json
```

### 2. Digital Twins (`twins/`)
- `local-storage-twin/definition.yaml` - Browser localStorage simulation
- `wasm-runtime-twin/definition.yaml` - WASM execution simulation
- `flow-wasm-universe.yaml` - Universe manifest
- `catalog.yaml` - Twin registry
- **Implementation:** `src/twin_runtime/mod.rs`

**Twin Features:**
- HTTP server for twin endpoints
- Stateful collections with CRUD operations
- Inspection APIs for testing
- Health check endpoints
- State reset capability

**Usage:**
```bash
cargo run --bin twin-server \
  --twin twins/local-storage-twin/definition.yaml \
  --port 9001
```

### 3. Behavioral Scenarios (`scenarios-vault/`) - Agent CANNOT ACCESS
- `flow-wasm/happy-path/` - Primary workflow tests
- `flow-wasm/error-handling/` - Error condition tests
- `flow-wasm/security/` - Security invariant tests
- `feedback-config.yaml` - Feedback level configuration
- **Implementation:** `src/scenario_runner/mod.rs`

**Scenario Categories:**
- Happy Path (2 scenarios)
- Error Handling (3 scenarios)
- Security (1 scenario)

**Usage:**
```bash
cargo run --bin scenario-runner \
  --scenarios-path ../scenarios-vault/flow-wasm \
  --app-endpoint http://localhost:8080 \
  --level 3
```

### 4. Feedback System (`src/feedback/`)
- 5 feedback levels (minimal to transparent)
- Failure categorization
- Hint generation
- Spec text references
- **Implementation:** `src/feedback/mod.rs`

**Feedback Levels:**
1. Minimal: "X of Y tests failed"
2. Categorical: + failure categories
3. Guided (default): + descriptions + hints + spec refs
4. Diagnostic: + HTTP status codes
5. Transparent: Full raw details

### 5. Quality Gate (`src/oya/src/quality_gate/` & scripts)
- Spec validation phase
- Scenario validation phase
- Iteration tracking (max 5)
- Escalation on max iterations
- State machine
- **Implementation:** `src/oya/src/quality_gate/mod.rs`

**Pipeline Phases:**
1. SPEC VALIDATION - Run spec linter, require score ≥ 80
2. UNIVERSE SETUP - Spin up twin instances
3. AGENT_DEV - Agent receives: spec + twin URLs + (feedback if retry)
4. VALIDATION - Run holdout scenarios (agent-blind)
5. FEEDBACK - Sanitize results, send to agent if failed

### 6. Metrics Collection (`src/metrics/`)
- Session tracking
- Spec quality scores
- Scenario pass/fail rates
- Iteration counts
- Common failure categories
- Performance metrics
- **Implementation:** `src/metrics/mod.rs`

**Metrics Tracked:**
- Total sessions
- Passed/failed/escalated counts
- Avg iterations to pass
- Avg duration per session
- Spec quality scores over time

**Usage:**
```bash
cargo run --bin quality-dashboard summary
cargo run --bin quality-dashboard sessions --count 10
cargo run --bin quality-dashboard export --format json
```

### 7. Coverage Analysis (`src/coverage/`)
- Spec-to-scenario coverage tracking
- Behavior coverage percentage
- Common gap identification
- **Implementation:** `src/coverage/mod.rs`

**Coverage Metrics:**
- Total behaviors per spec
- Covered behaviors
- Covered edge cases
- Missing behaviors and edge cases

**Usage:**
```bash
cargo run --bin coverage --specs-dir specs --scenarios-dir ../scenarios-vault
```

### 8. Twin Deployment (`src/deployment/`)
- Twin instance management
- Universe deployment orchestration
- Status monitoring
- **Implementation:** `src/deployment/mod.rs`

**Deployment States:**
- Stopped, Starting, Running, Error
- Port allocation (9000-9010 range)

### 9. Agent Feedback (`src/agent_feedback/`)
- Feedback template system
- Category-based feedback generation
- Priority determination
- Hint creation
- **Implementation:** `src/agent_feedback/mod.rs`

**Feedback Categories:**
- Spec Quality Issues
- Validation Failures
- Security Issues
- Integration Issues

## CLI Tools

| Tool | Description | Command |
|------|-------------|----------|
| `spec-linter` | Validate spec quality | `cargo run --bin spec-linter <spec>` |
| `scenario-runner` | Run holdout scenarios | `cargo run --bin scenario-runner --scenarios-path <path>` |
| `quality-gate` | Run full quality gate | `cargo run --bin quality-gate full` |
| `quality-dashboard` | View metrics | `cargo run --bin quality-dashboard [command]` |
| `coverage` | Analyze coverage | `cargo run --bin coverage --specs-dir specs` |
| `agent-feedback` | Generate feedback | `cargo run --bin agent-feedback generate --category <type>` |

## CI/CD Integration

### Shell Scripts (`scripts/`)
- `validate-spec.sh` - Spec validation only
- `validate-scenarios.sh` - Scenario validation only
- `run-quality-gate.sh` - Full pipeline

### Integration with Oya Orchestrator

Add quality gate as a stage before autonomous development:

```yaml
stages:
  - name: quality-gate
    action: quality_gate::run
    requires: [spec-lint]
    on_fail: feedback

  - name: autonomous-dev
    action: tdd15  # Agent implements
    requires: [quality-gate]
```

## Information Barrier Enforcement

### What Agent Sees (in `src/new-app/`):
- `specs/` - Full specification files
- `twins/` - Twin definitions
- `specs/linter/rules.yaml` - Validation rules
- `specs/flow-wasm-v1.yaml` - Acceptance criteria

### What Agent Cannot See (in `src/scenarios-vault/`):
- `flow-wasm/happy-path/` - Holdout scenarios
- `flow-wasm/error-handling/` - Holdout scenarios
- `flow-wasm/security/` - Holdout scenarios
- Scenarios with exact step sequences
- Holdout assertions and expected values

### Feedback the Agent Receives:
- Failure category (no scenario IDs)
- Natural language description
- Hints about fixing issues
- Reference to spec section
- Sanitized results only (no raw test data)

## Invariants

1. Agent never sees scenarios
2. Spec must pass linter (score ≥ 80) before agent starts
3. All validation runs use fresh twin universe
4. Feedback is always sanitized (never raw test details)
5. Max 5 iterations before escalation
6. Twins reset between test runs
7. All acceptance criteria must be satisfied

## Metrics Tracking

The system automatically tracks:

- Spec quality score trends
- Scenario pass/fail rates
- Common failure categories
- Agent iteration patterns
- Escalation rates
- Time to success metrics

## Development Workflow

```
1. Write spec → 2. Run spec linter → 3. Agent develops → 4. Run scenarios → 5. Feedback
                      ↓ if fail: go to 3  ↑              ↓ if pass: accept ✓
```

## Files Structure

```
src/new-app/
├── specs/                    # Agent CAN SEE
│   ├── schema/
│   ├── linter/
│   └── flow-wasm-v1.yaml
│
├── twins/                     # Agent CAN SEE
│   ├── local-storage-twin/
│   ├── wasm-runtime-twin/
│   ├── flow-wasm-universe.yaml
│   └── catalog.yaml
│
├── src/
│   ├── linter/mod.rs       # Quality enforcement
│   ├── scenario_runner/mod.rs  # Holdout validation
│   ├── feedback/mod.rs       # Feedback sanitization
│   ├── metrics/mod.rs        # Metrics collection
│   ├── coverage/mod.rs       # Coverage analysis
│   ├── deployment/mod.rs   # Twin deployment
│   ├── agent_feedback/mod.rs # Agent feedback gen
│   ├── dashboard/mod.rs      # Quality dashboard
│   └── bin/
│       ├── spec-linter.rs
│       ├── scenario-runner.rs
│       ├── quality-gate.rs
│       ├── quality-dashboard.rs
│       ├── coverage.rs
│       └── agent_feedback.rs
│
├── scripts/                  # CI/CD scripts
│   ├── validate-spec.sh
│   ├── validate-scenarios.sh
│   └── run-quality-gate.sh
│
└── tests/                    # Test suites
    └── canvas.spec.js   # E2E tests for the app
```

src/scenarios-vault/         # Agent CANNOT SEE (separate directory)
└── flow-wasm/
    ├── happy-path/
    ├── error-handling/
    └── security/
```

## Key Principles

1. **Spec Quality First**: Specs must meet quality threshold before agent starts
2. **Deterministic Testing**: Scenarios provide repeatable, black-box testing
3. **Observability Only**: All behaviors must have externally observable outcomes
4. **Sanitized Feedback**: Agent receives helpful hints, not test details
5. **Iterate to Quality Gate**: System guides agent toward correct behavior

## Example Usage

### Complete Quality Gate Pipeline:
```bash
# 1. Validate spec
./scripts/validate-spec.sh

# 2. Run scenarios
./scripts/validate-scenarios.sh

# 3. Full quality gate
./scripts/run-quality-gate.sh
```

### View Metrics:
```bash
cargo run --bin quality-dashboard summary
```

### Analyze Coverage:
```bash
cargo run --bin coverage --specs-dir specs --scenarios-dir ../scenarios-vault
```

### Generate Agent Feedback:
```bash
cargo run --bin agent-feedback generate --category validation --spec-ref spec-001
```

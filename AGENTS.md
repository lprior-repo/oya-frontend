# NEW-APP Quality Gate Instructions
# Agent behavior when working on new-app

## WHAT YOU CAN SEE (Agent Workspace)
- ✅ `specs/flow-wasm-v1.yaml` - The full specification
- ✅ `twins/` - Digital twin definitions
- ✅ `tests/` - Your own test files
- ✅ `src/` - The application source code
- ✅ `specs/linter/rules.yaml` - Spec quality rules
- ✅ ACCEPTANCE CRITERIA from the spec

## WHAT YOU CANNOT SEE (Quality Gate Holdout)
- ❌ `../scenarios-vault/` - Behavioral scenarios (separate repository)
- ❌ Holdout test assertions
- ❌ Exact step sequences for validation
- ❌ Raw validation results

## DEVELOPMENT WORKFLOW

1. **Read the spec** - Understand what to build from `specs/flow-wasm-v1.yaml`
2. **Check spec linter** - Run validation on specs to ensure quality
3. **Use twins** - Develop against twin endpoints (localStorage twin available)
4. **Write tests** - Create your own tests in `tests/`
5. **Build and test** - Use cargo/npm as specified in the project

## QUALITY GATE

Your code will be validated against **hidden behavioral scenarios** that you cannot see.
This prevents "teaching to the test" and ensures genuine behavioral correctness.

If validation fails, you will receive **sanitized feedback** that:
- Tells you which category failed
- References the spec
- Provides hints about the issue
- Does NOT reveal the exact test or expected values

## INVARIANTS

1. Never attempt to access ../scenarios-vault/
2. Never ask about holdout scenarios
3. Build software that genuinely implements the spec, not tests that pass
4. All acceptance criteria in the spec must be satisfied

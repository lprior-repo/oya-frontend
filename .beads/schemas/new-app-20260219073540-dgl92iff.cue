
package validation

import "list"

// Validation schema for bead: new-app-20260219073540-dgl92iff
// Title: ui: sidebar navigation and fluid animations
//
// This schema validates that implementation is complete.
// Use: cue vet new-app-20260219073540-dgl92iff.cue implementation.cue

#BeadImplementation: {
  bead_id: "new-app-20260219073540-dgl92iff"
  title: "ui: sidebar navigation and fluid animations"

  // Contract verification
  contracts_verified: {
    preconditions_checked: bool & true
    postconditions_verified: bool & true
    invariants_maintained: bool & true

    // Specific preconditions that must be verified
    precondition_checks: [
      "Multi-project logic implemented",
    ]

    // Specific postconditions that must be verified
    postcondition_checks: [
      "UI correctly reflects the selected project state",
    ]

    // Specific invariants that must be maintained
    invariant_checks: [
      "No state mutation inside render functions",
    ]
  }

  // Test verification
  tests_passing: {
    all_tests_pass: bool & true

    happy_path_tests: [...string] & list.MinItems(2)
    error_path_tests: [...string] & list.MinItems(2)

    // Note: Actual test names provided by implementer, must include all required tests

    // Required happy path tests
    required_happy_tests: [
      "Clicking a project updates the header and task list",
      "New projects appear in the sidebar immediately",
    ]

    // Required error path tests
    required_error_tests: [
      "Error message shown when switching projects fails due to data corruption",
      "Validation error shown for empty project names",
    ]
  }

  // Code completion
  code_complete: {
    implementation_exists: string  // Path to implementation file
    tests_exist: string  // Path to test file
    ci_passing: bool & true
    no_unwrap_calls: bool & true  // Rust/functional constraint
    no_panics: bool & true  // Rust constraint
  }

  // Completion criteria
  completion: {
    all_sections_complete: bool & true
    documentation_updated: bool
    beads_closed: bool
    timestamp: string  // ISO8601 completion timestamp
  }
}

// Example implementation proof - create this file to validate completion:
//
// implementation.cue:
// package validation
//
// implementation: #BeadImplementation & {
//   contracts_verified: {
//     preconditions_checked: true
//     postconditions_verified: true
//     invariants_maintained: true
//     precondition_checks: [/* documented checks */]
//     postcondition_checks: [/* documented verifications */]
//     invariant_checks: [/* documented invariants */]
//   }
//   tests_passing: {
//     all_tests_pass: true
//     happy_path_tests: ["test_version_flag_works", "test_version_format", "test_exit_code_zero"]
//     error_path_tests: ["test_invalid_flag_errors", "test_no_flags_normal_behavior"]
//   }
//   code_complete: {
//     implementation_exists: "src/main.rs"
//     tests_exist: "tests/cli_test.rs"
//     ci_passing: true
//     no_unwrap_calls: true
//     no_panics: true
//   }
//   completion: {
//     all_sections_complete: true
//     documentation_updated: true
//     beads_closed: false
//     timestamp: "2026-02-19T07:35:40Z"
//   }
// }
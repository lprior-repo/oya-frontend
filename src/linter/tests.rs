use super::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_rules() -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        "{}",
        r#"
rules:
  - id: SPEC-001
    name: test-rule
    severity: error
    description: "Test rule"
    banned_phrases: []
  - id: SPEC-010
    name: no-ambiguous-language
    severity: warning
    description: "Test ambiguous language"
    banned_phrases:
      - "obviously"
      - "simply"
"#
    )
    .unwrap();
    file
}

fn create_test_spec_minimal() -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-test
    version: 1.0.0
    status: draft
    author: test
    created: "2026-01-01T00:00:00Z"
  intent:
    problem_statement: "Test problem"
    success_criteria:
      - "Test criteria"
  context:
    system_dependencies: []
    invariants: []
  behaviors:
    - id: test-behavior
      description: "Test"
      then:
        - "Test action"
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )
    .unwrap();
    file
}

#[test]
fn test_lint_spec_minimal() {
    let rules_file = create_test_rules();
    let spec_file = create_test_spec_minimal();

    let linter = SpecLinter::new(rules_file.path()).unwrap();
    let report = linter.lint(spec_file.path()).unwrap();

    assert_eq!(report.spec_id, "spec-test");
    assert!(report.overall_score >= 80);
    assert!(report.errors.is_empty());
}

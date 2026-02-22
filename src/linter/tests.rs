#![allow(clippy::write_literal)]
use super::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_rules() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
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
    )?;
    Ok(file)
}

fn create_test_spec_minimal() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
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
    )?;
    Ok(file)
}

fn create_rules_for_completeness_underflow() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
rules:
  - id: SPEC-001
    name: dependency-failure-cases
    severity: error
    description: "Dependency failure edges"
    banned_phrases: []
"#
    )?;
    Ok(file)
}

fn create_rules_for_completeness_and_auth() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
rules:
  - id: SPEC-001
    name: dependency-failure-cases
    severity: error
    description: "Dependency failure edges"
  - id: SPEC-003
    name: explicit-auth
    severity: error
    description: "API endpoints need authentication"
"#
    )?;
    Ok(file)
}

fn create_invalid_rules(content: &str) -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", content)?;
    Ok(file)
}

fn create_spec_with_missing_dependency_error_edges() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-underflow
    version: 1.0.0
    status: draft
    author: test
    created: "2026-01-01T00:00:00Z"
  intent:
    problem_statement: "Test problem"
    success_criteria:
      - "Test criteria"
  context:
    system_dependencies:
      - service: Redis
        purpose: cache
        twin_available: true
      - service: Postgres
        purpose: db
        twin_available: true
    invariants: []
  behaviors:
    - id: behavior-1
      description: "No edge cases"
      then:
        - "System processes request"
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )?;
    Ok(file)
}

fn create_spec_with_ambiguous_language() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-clarity
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
    - id: behavior-1
      description: "Ambiguous"
      then:
        - "System should probably respond"
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )?;
    Ok(file)
}

fn create_spec_with_enumeration_risk() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-security
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
    - id: behavior-1
      description: "No enumeration checks"
      then:
        - "HTTP response is returned"
  api_contract:
    endpoints:
      - method: POST
        path: /users/email/login
        authentication: bearer
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )?;
    Ok(file)
}

fn create_spec_with_missing_auth_endpoint() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-auth
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
    - id: behavior-1
      description: "Auth path"
      then:
        - "HTTP response is returned"
  api_contract:
    endpoints:
      - method: GET
        path: /v1/private
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )?;
    Ok(file)
}

fn create_spec_with_non_observable_then_clause() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-testability
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
    - id: behavior-1
      description: "Non observable"
      then:
        - "System updates internal cache"
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )?;
    Ok(file)
}

fn create_spec_with_state_transitions_no_invariants() -> anyhow::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "{}",
        r#"
specification:
  identity:
    id: spec-data-model
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
    - id: behavior-1
      description: "Simple"
      then:
        - "HTTP response is returned"
  data_model:
    state_transitions:
      - from: draft
        to: active
  acceptance_criteria:
    - id: ac-01
      criterion: "Test criterion"
"#
    )?;
    Ok(file)
}

#[test]
fn test_lint_spec_minimal() -> anyhow::Result<()> {
    let rules_file = create_test_rules()?;
    let spec_file = create_test_spec_minimal()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert_eq!(report.spec_id, "spec-test");
    assert!(report.overall_score >= 80);
    assert!(report.errors.is_empty());

    Ok(())
}

#[test]
fn given_multiple_dependency_failures_when_checking_completeness_then_score_saturates_to_zero(
) -> anyhow::Result<()> {
    let rules_file = create_rules_for_completeness_underflow()?;
    let spec_file = create_spec_with_missing_dependency_error_edges()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert_eq!(report.errors.len(), 2);
    let completeness = report.categories.get("Completeness");
    assert!(completeness.is_some_and(|score| score.score == 0));
    Ok(())
}

#[test]
fn given_ambiguous_then_clause_when_linting_then_clarity_warning_is_reported() -> anyhow::Result<()>
{
    let rules_file = create_test_rules()?;
    let spec_file = create_spec_with_ambiguous_language()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert!(report
        .warnings
        .iter()
        .any(|warning| warning.rule_id == "SPEC-010"));
    Ok(())
}

#[test]
fn given_user_identifier_endpoint_without_enumeration_edge_case_when_linting_then_security_error_is_reported(
) -> anyhow::Result<()> {
    let rules_file = create_test_rules()?;
    let spec_file = create_spec_with_enumeration_risk()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert!(report
        .errors
        .iter()
        .any(|error| error.rule_id == "SPEC-020"));
    Ok(())
}

#[test]
fn given_unknown_rule_id_when_loading_rules_then_linter_returns_explicit_error(
) -> anyhow::Result<()> {
    let rules_file = create_invalid_rules(
        r#"
rules:
  - id: SPEC-999
    name: unknown-rule
    severity: error
    description: "Unknown rule"
"#,
    )?;

    let result = SpecLinter::new(rules_file.path());

    assert!(matches!(result, Err(LintError::UnknownRuleId { .. })));
    Ok(())
}

#[test]
fn given_invalid_rule_severity_when_loading_rules_then_linter_returns_explicit_error(
) -> anyhow::Result<()> {
    let rules_file = create_invalid_rules(
        r#"
rules:
  - id: SPEC-001
    name: invalid-severity
    severity: critical
    description: "Invalid severity"
"#,
    )?;

    let result = SpecLinter::new(rules_file.path());

    assert!(matches!(result, Err(LintError::InvalidSeverity { .. })));
    Ok(())
}

#[test]
fn given_missing_required_rule_field_when_loading_rules_then_linter_returns_explicit_error(
) -> anyhow::Result<()> {
    let rules_file = create_invalid_rules(
        r#"
rules:
  - id: SPEC-001
    name: ""
    severity: error
    description: "Missing name"
"#,
    )?;

    let result = SpecLinter::new(rules_file.path());

    assert!(matches!(
        result,
        Err(LintError::MissingRequiredField { ref field, .. }) if field == "name"
    ));
    Ok(())
}

#[test]
fn given_api_endpoint_without_auth_when_linting_then_spec_003_error_is_reported(
) -> anyhow::Result<()> {
    let rules_file = create_rules_for_completeness_and_auth()?;
    let spec_file = create_spec_with_missing_auth_endpoint()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert!(report
        .errors
        .iter()
        .any(|issue| issue.rule_id == "SPEC-003"));
    Ok(())
}

#[test]
fn given_non_observable_then_clause_when_linting_then_spec_030_warning_is_reported(
) -> anyhow::Result<()> {
    let rules_file = create_test_rules()?;
    let spec_file = create_spec_with_non_observable_then_clause()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert!(report
        .warnings
        .iter()
        .any(|issue| issue.rule_id == "SPEC-030"));
    Ok(())
}

#[test]
fn given_state_transitions_without_invariants_when_linting_then_spec_002_warning_is_reported(
) -> anyhow::Result<()> {
    let rules_file = create_test_rules()?;
    let spec_file = create_spec_with_state_transitions_no_invariants()?;

    let linter = SpecLinter::new(rules_file.path())?;
    let report = linter.lint(spec_file.path())?;

    assert!(report
        .warnings
        .iter()
        .any(|issue| issue.rule_id == "SPEC-002"));
    Ok(())
}

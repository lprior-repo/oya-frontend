use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LintError {
    #[error("Failed to read spec file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecIdentity {
    pub id: String,
    pub version: String,
    pub status: String,
    pub author: String,
    pub created: String,
    pub updated: Option<String>,
    pub supersedes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecIntent {
    pub problem_statement: String,
    pub success_criteria: Vec<String>,
    pub non_goals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDependency {
    pub service: String,
    pub purpose: String,
    #[serde(rename = "twin_available")]
    pub twin_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecContext {
    #[serde(rename = "system_dependencies")]
    pub system_dependencies: Vec<SystemDependency>,
    #[serde(rename = "existing_behaviors")]
    pub existing_behaviors: Option<Vec<String>>,
    pub constraints: Option<Vec<String>>,
    pub invariants: Vec<String>,
    pub glossary: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    pub id: String,
    pub description: String,
    pub given: Option<Vec<String>>,
    pub r#when: Option<String>,
    pub then: Vec<String>,
    #[serde(rename = "edge_cases")]
    pub edge_cases: Option<Vec<EdgeCase>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCase {
    pub id: String,
    pub r#when: String,
    pub then: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataModelEntity {
    pub name: String,
    pub fields: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataModel {
    pub entities: Option<Vec<DataModelEntity>>,
    #[serde(rename = "state_transitions")]
    pub state_transitions: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub method: String,
    pub path: String,
    #[serde(rename = "authentication")]
    pub authentication: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiContract {
    pub endpoints: Option<Vec<ApiEndpoint>>,
    #[serde(rename = "events_emitted")]
    pub events_emitted: Option<Vec<serde_json::Value>>,
    #[serde(rename = "events_consumed")]
    pub events_consumed: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceCriterion {
    pub id: String,
    #[serde(rename = "behavior_ref")]
    pub behavior_ref: Option<String>,
    pub criterion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub identity: SpecIdentity,
    pub intent: SpecIntent,
    pub context: SpecContext,
    pub behaviors: Vec<Behavior>,
    #[serde(rename = "data_model")]
    pub data_model: Option<DataModel>,
    #[serde(rename = "api_contract")]
    pub api_contract: Option<ApiContract>,
    #[serde(rename = "acceptance_criteria")]
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub specification: Specification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintRule {
    pub id: String,
    pub name: String,
    pub severity: String,
    pub description: String,
    #[serde(rename = "banned_phrases")]
    pub banned_phrases: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintRules {
    pub rules: Vec<LintRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintIssue {
    pub rule_id: String,
    pub rule_name: String,
    pub severity: String,
    pub message: String,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintReport {
    pub spec_id: String,
    pub spec_version: String,
    pub overall_score: u32,
    pub passed: bool,
    pub categories: HashMap<String, CategoryScore>,
    pub errors: Vec<LintIssue>,
    pub warnings: Vec<LintIssue>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScore {
    pub score: u32,
    pub details: String,
}

impl LintReport {
    #[must_use]
    pub fn new(spec_id: String, spec_version: String) -> Self {
        Self {
            spec_id,
            spec_version,
            overall_score: 0,
            passed: false,
            categories: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn calculate_score(&mut self) {
        let mut total = 0u32;
        let mut count = 0u32;

        for cat in self.categories.values() {
            total += cat.score;
            count += 1;
        }

        self.overall_score = if count > 0 { total / count } else { 0 };
        self.passed = self.errors.is_empty() && self.overall_score >= 80;
    }
}

pub struct SpecLinter {
    rules: LintRules,
}

impl SpecLinter {
    /// Create a new linter.
    ///
    /// # Errors
    /// Returns an error if rules file cannot be read or parsed.
    pub fn new(rules_path: &Path) -> Result<Self, LintError> {
        let rules_content = fs::read_to_string(rules_path)?;
        let rules: LintRules = serde_yaml::from_str(&rules_content)?;
        Ok(Self { rules })
    }

    /// Lint a specification file.
    ///
    /// # Errors
    /// Returns an error if spec file cannot be read or parsed.
    pub fn lint(&self, spec_path: &Path) -> Result<LintReport, LintError> {
        let spec_content = fs::read_to_string(spec_path)?;
        let spec: Spec = serde_yaml::from_str(&spec_content)?;

        let mut report = LintReport::new(
            spec.specification.identity.id.clone(),
            spec.specification.identity.version.clone(),
        );

        Self::check_completeness(&self.rules, &spec, &mut report);
        Self::check_clarity(&spec, &mut report);
        Self::check_security(&spec, &mut report);
        Self::check_testability(&spec, &mut report);
        Self::check_data_model(&spec, &mut report);

        report.calculate_score();
        Ok(report)
    }

    fn check_completeness(rules: &LintRules, spec: &Spec, report: &mut LintReport) {
        let mut errors = 0;
        let mut total = 0;

        for rule in &rules.rules {
            if !rule.id.starts_with("SPEC-00") || rule.severity == "warning" {
                continue;
            }
            total += 1;

            match rule.id.as_str() {
                "SPEC-001" => {
                    let deps = &spec.specification.context.system_dependencies;
                    for dep in deps {
                        let has_error_handling = spec.specification.behaviors.iter().any(|b| {
                            b.edge_cases.as_ref().is_some_and(|ec| {
                                ec.iter().any(|e| {
                                    e.then.iter().any(|t| {
                                        let t_low = t.to_lowercase();
                                        let dep_low = dep.service.to_lowercase();
                                        t_low.contains(&dep_low)
                                            && (t_low.contains("fail")
                                                || t_low.contains("error")
                                                || t_low.contains("unavailable"))
                                    })
                                })
                            })
                        });
                        if !has_error_handling && dep.twin_available {
                            errors += 1;
                            report.errors.push(LintIssue {
                                rule_id: rule.id.clone(),
                                rule_name: rule.name.clone(),
                                severity: rule.severity.clone(),
                                message: format!(
                                    "Dependency '{}' has no error handling edge case",
                                    dep.service
                                ),
                                line: None,
                            });
                        }
                    }
                }
                "SPEC-003" => {
                    if let Some(contract) = &spec.specification.api_contract {
                        if let Some(endpoints) = &contract.endpoints {
                            for endpoint in endpoints {
                                if endpoint.authentication.is_none() {
                                    errors += 1;
                                    report.errors.push(LintIssue {
                                        rule_id: rule.id.clone(),
                                        rule_name: rule.name.clone(),
                                        severity: rule.severity.clone(),
                                        message: format!(
                                            "Endpoint {} {} missing authentication specification",
                                            endpoint.method, endpoint.path
                                        ),
                                        line: None,
                                    });
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let score = if total > 0 {
            ((total - errors) * 100) / total
        } else {
            100
        };
        report.categories.insert(
            "Completeness".to_string(),
            CategoryScore {
                score,
                details: format!("{}/{} rules passed", total - errors, total),
            },
        );
    }

    fn check_clarity(spec: &Spec, report: &mut LintReport) {
        let mut warnings = 0;
        let banned = [
            "as appropriate",
            "if needed",
            "as necessary",
            "etc.",
            "obviously",
            "simply",
            "just",
            "should probably",
            "standard practice",
        ];

        for behavior in &spec.specification.behaviors {
            for then in &behavior.then {
                for phrase in &banned {
                    if then.to_lowercase().contains(phrase) {
                        warnings += 1;
                        report.warnings.push(LintIssue {
                            rule_id: "SPEC-010".to_string(),
                            rule_name: "no-ambiguous-language".to_string(),
                            severity: "warning".to_string(),
                            message: format!(
                                "Found ambiguous phrase: '{phrase}' in behavior {}",
                                behavior.id
                            ),
                            line: None,
                        });
                    }
                }
            }
        }

        let score = if warnings > 0 { 88 } else { 100 };
        report.categories.insert(
            "Clarity".to_string(),
            CategoryScore {
                score,
                details: format!("{warnings} ambiguous phrases found"),
            },
        );
    }

    fn check_security(spec: &Spec, report: &mut LintReport) {
        let mut score = 100;

        if let Some(contract) = &spec.specification.api_contract {
            if let Some(endpoints) = &contract.endpoints {
                let user_endpoints: Vec<_> = endpoints
                    .iter()
                    .filter(|e| {
                        e.path.contains("email")
                            || e.path.contains("username")
                            || e.path.contains("password")
                    })
                    .collect();

                for endpoint in user_endpoints {
                    let has_enumeration_check = spec.specification.behaviors.iter().any(|b| {
                        b.edge_cases.as_ref().is_some_and(|ec| {
                            ec.iter().any(|e| {
                                e.id.contains("nonexist")
                                    || e.id.contains("not-found")
                                    || e.id.contains("enumeration")
                            })
                        })
                    });

                    if !has_enumeration_check {
                        score = 95;
                        report.errors.push(LintIssue {
                            rule_id: "SPEC-020".to_string(),
                            rule_name: "enumeration-prevention".to_string(),
                            severity: "error".to_string(),
                            message: format!(
                                "Endpoint {} may be vulnerable to user enumeration",
                                endpoint.path
                            ),
                            line: None,
                        });
                    }
                }
            }
        }

        report.categories.insert(
            "Security".to_string(),
            CategoryScore {
                score,
                details: "Enumeration prevention checked".to_string(),
            },
        );
    }

    fn check_testability(spec: &Spec, report: &mut LintReport) {
        let mut errors = 0;
        let observable_terms = ["http", "response", "status", "body", "api", "event"];

        for behavior in &spec.specification.behaviors {
            for then in &behavior.then {
                let is_observable = observable_terms
                    .iter()
                    .any(|t| then.to_lowercase().contains(t));

                if !is_observable && !then.contains("audit") {
                    errors += 1;
                }
            }
        }

        let score = if errors > 0 { 90 } else { 100 };
        if errors > 0 {
            report.warnings.push(LintIssue {
                rule_id: "SPEC-030".to_string(),
                rule_name: "behaviors-are-observable".to_string(),
                severity: "warning".to_string(),
                message: format!("{errors} behaviors may not have observable outcomes"),
                line: None,
            });
        }

        report.categories.insert(
            "Testability".to_string(),
            CategoryScore {
                score,
                details: format!("{} behaviors checked", spec.specification.behaviors.len()),
            },
        );
    }

    fn check_data_model(spec: &Spec, report: &mut LintReport) {
        let mut score = 100;

        if let Some(data_model) = &spec.specification.data_model {
            if let Some(transitions) = &data_model.state_transitions {
                if !transitions.is_empty() && spec.specification.context.invariants.is_empty() {
                    score = 88;
                    report.warnings.push(LintIssue {
                        rule_id: "SPEC-002".to_string(),
                        rule_name: "every-state-transition-has-invariant-check".to_string(),
                        severity: "warning".to_string(),
                        message: "State transitions found but no invariants defined".to_string(),
                        line: None,
                    });
                }
            }
        }

        report.categories.insert(
            "Data Model".to_string(),
            CategoryScore {
                score,
                details: "Data model consistency checked".to_string(),
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_rules() -> anyhow::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            "rules:
  - id: SPEC-001
    name: test-rule
    severity: error
    description: \"Test rule\"
    banned_phrases: []"
        )?;
        Ok(file)
    }

    fn create_test_spec() -> anyhow::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            "specification:
  identity:
    id: spec-test
    version: 1.0.0
    status: draft
    author: test
    created: \"2026-01-01T00:00:00Z\"
  intent:
    problem_statement: \"Test problem\"
    success_criteria:
      - \"Test criteria\"
  context:
    system_dependencies:
      - service: test-service
        purpose: testing
        twin_available: true
    invariants:
      - \"Test invariant\"
  behaviors:
    - id: test-behavior
      description: \"Test\"
      then:
        - \"Test action\"
  acceptance_criteria:
    - id: ac-01
      criterion: \"Test criterion\""
        )?;
        Ok(file)
    }

    #[test]
    fn test_linter() -> anyhow::Result<()> {
        let rules = create_test_rules()?;
        let spec = create_test_spec()?;
        let linter = SpecLinter::new(rules.path())?;
        let _report = linter.lint(spec.path())?;
        Ok(())
    }
}

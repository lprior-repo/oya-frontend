use std::fs;
use std::path::Path;

use super::model::{CategoryScore, LintError, LintIssue, LintReport, LintRules, Spec};

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

        rules
            .rules
            .iter()
            .filter(|rule| rule.id.starts_with("SPEC-00") && rule.severity != "warning")
            .for_each(|rule| {
                total += 1;

                match rule.id.as_str() {
                    "SPEC-001" => {
                        spec.specification
                            .context
                            .system_dependencies
                            .iter()
                            .for_each(|dep| {
                                let has_error_handling =
                                    spec.specification.behaviors.iter().any(|behavior| {
                                        behavior.edge_cases.as_ref().is_some_and(|edge_cases| {
                                            edge_cases.iter().any(|edge_case| {
                                                edge_case.then.iter().any(|then_clause| {
                                                    let then_lower = then_clause.to_lowercase();
                                                    let dep_lower = dep.service.to_lowercase();
                                                    then_lower.contains(&dep_lower)
                                                        && (then_lower.contains("fail")
                                                            || then_lower.contains("error")
                                                            || then_lower.contains("unavailable"))
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
                            });
                    }
                    "SPEC-003" => {
                        if let Some(contract) = &spec.specification.api_contract {
                            if let Some(endpoints) = &contract.endpoints {
                                endpoints.iter().for_each(|endpoint| {
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
                                });
                            }
                        }
                    }
                    _ => {}
                }
            });

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

        spec.specification.behaviors.iter().for_each(|behavior| {
            behavior.then.iter().for_each(|then_clause| {
                banned.iter().for_each(|phrase| {
                    if then_clause.to_lowercase().contains(phrase) {
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
                });
            });
        });

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

                user_endpoints.iter().for_each(|endpoint| {
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
                });
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
        let observable_terms = ["http", "response", "status", "body", "api", "event"];
        let errors = spec
            .specification
            .behaviors
            .iter()
            .flat_map(|behavior| behavior.then.iter())
            .filter(|then_clause| {
                let is_observable = observable_terms
                    .iter()
                    .any(|term| then_clause.to_lowercase().contains(term));
                !is_observable && !then_clause.contains("audit")
            })
            .count();

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

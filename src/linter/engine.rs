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
        Self::validate_rules(&rules)?;
        Ok(Self { rules })
    }

    fn validate_rules(rules: &LintRules) -> Result<(), LintError> {
        const ALLOWED_RULE_IDS: [&str; 10] = [
            "SPEC-001", "SPEC-002", "SPEC-003", "SPEC-004", "SPEC-010", "SPEC-011", "SPEC-020",
            "SPEC-021", "SPEC-030", "SPEC-040",
        ];

        for rule in &rules.rules {
            let rule_id = rule.id.trim();
            if rule_id.is_empty() {
                return Err(LintError::MissingRequiredField {
                    rule_id: "<unknown>".to_string(),
                    field: "id".to_string(),
                });
            }

            if !ALLOWED_RULE_IDS.contains(&rule_id) {
                return Err(LintError::UnknownRuleId {
                    rule_id: rule_id.to_string(),
                });
            }

            if rule.name.trim().is_empty() {
                return Err(LintError::MissingRequiredField {
                    rule_id: rule.id.clone(),
                    field: "name".to_string(),
                });
            }

            if rule.description.trim().is_empty() {
                return Err(LintError::MissingRequiredField {
                    rule_id: rule.id.clone(),
                    field: "description".to_string(),
                });
            }

            let severity = rule.severity.trim();
            if severity.is_empty() {
                return Err(LintError::MissingRequiredField {
                    rule_id: rule.id.clone(),
                    field: "severity".to_string(),
                });
            }

            if severity != "error" && severity != "warning" {
                return Err(LintError::InvalidSeverity {
                    rule_id: rule.id.clone(),
                    severity: rule.severity.clone(),
                });
            }
        }

        Ok(())
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
        let (errors, total) = rules
            .rules
            .iter()
            .filter(|rule| rule.id.starts_with("SPEC-00") && rule.severity != "warning")
            .fold((0usize, 0usize), |(errors, total), rule| {
                let rule_errors = match rule.id.as_str() {
                    "SPEC-001" => spec
                        .specification
                        .context
                        .system_dependencies
                        .iter()
                        .filter(|dep| dep.twin_available)
                        .filter(|dep| {
                            !spec.specification.behaviors.iter().any(|behavior| {
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
                            })
                        })
                        .map(|dep| {
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
                            1
                        })
                        .sum::<usize>(),
                    "SPEC-003" => {
                        if let Some(contract) = &spec.specification.api_contract {
                            if let Some(endpoints) = &contract.endpoints {
                                endpoints
                                    .iter()
                                    .filter(|endpoint| endpoint.authentication.is_none())
                                    .map(|endpoint| {
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
                                        1
                                    })
                                    .sum::<usize>()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    }
                    _ => 0,
                };
                (errors + rule_errors, total + 1)
            });

        let passed = total.saturating_sub(errors);
        let score = if total > 0 {
            (passed * 100) / total
        } else {
            100
        };
        let score: u32 = score.try_into().map_or(100, |score| score);
        report.categories.insert(
            "Completeness".to_string(),
            CategoryScore {
                score,
                details: format!("{passed}/{total} rules passed"),
            },
        );
    }

    fn check_clarity(spec: &Spec, report: &mut LintReport) {
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

        let warnings: Vec<_> = spec
            .specification
            .behaviors
            .iter()
            .flat_map(|behavior| {
                behavior.then.iter().filter_map(|then_clause| {
                    banned.iter().find_map(|phrase| {
                        if then_clause.to_lowercase().contains(phrase) {
                            Some(LintIssue {
                                rule_id: "SPEC-010".to_string(),
                                rule_name: "no-ambiguous-language".to_string(),
                                severity: "warning".to_string(),
                                message: format!(
                                    "Found ambiguous phrase: '{phrase}' in behavior {}",
                                    behavior.id
                                ),
                                line: None,
                            })
                        } else {
                            None
                        }
                    })
                })
            })
            .collect();

        warnings.iter().for_each(|issue| {
            report.warnings.push(issue.clone());
        });

        let score = if warnings.is_empty() { 100 } else { 88 };
        report.categories.insert(
            "Clarity".to_string(),
            CategoryScore {
                score,
                details: format!("{} ambiguous phrases found", warnings.len()),
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

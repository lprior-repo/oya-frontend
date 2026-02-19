use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackConfig {
    pub level: u8,
    pub name: String,
    pub includes_status_codes: bool,
    pub includes_bodies: bool,
    pub includes_timing: bool,
    pub includes_scenario_ids: bool,
    pub includes_step_sequences: bool,
    pub includes_exact_assertions: bool,
}

impl FeedbackConfig {
    #[must_use]
    pub fn from_level(level: u8) -> Self {
        match level {
            1 => Self {
                level,
                name: "minimal".to_string(),
                includes_status_codes: false,
                includes_bodies: false,
                includes_timing: false,
                includes_scenario_ids: false,
                includes_step_sequences: false,
                includes_exact_assertions: false,
            },
            2 => Self {
                level,
                name: "categorical".to_string(),
                includes_status_codes: false,
                includes_bodies: false,
                includes_timing: false,
                includes_scenario_ids: false,
                includes_step_sequences: false,
                includes_exact_assertions: false,
            },
            3 => Self {
                level,
                name: "guided".to_string(),
                includes_status_codes: false,
                includes_bodies: false,
                includes_timing: false,
                includes_scenario_ids: false,
                includes_step_sequences: false,
                includes_exact_assertions: false,
            },
            4 => Self {
                level,
                name: "diagnostic".to_string(),
                includes_status_codes: true,
                includes_bodies: false,
                includes_timing: false,
                includes_scenario_ids: false,
                includes_step_sequences: false,
                includes_exact_assertions: false,
            },
            5 => Self {
                level,
                name: "transparent".to_string(),
                includes_status_codes: true,
                includes_bodies: true,
                includes_timing: true,
                includes_scenario_ids: true,
                includes_step_sequences: true,
                includes_exact_assertions: true,
            },
            _ => Self::from_level(3),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizedFailure {
    pub category: String,
    pub spec_ref: String,
    pub description: String,
    pub hint: String,
    pub spec_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizedFeedback {
    pub iteration: u32,
    pub passed_count: usize,
    pub failed_count: usize,
    pub total_count: usize,
    pub failures: Vec<SanitizedFailure>,
    pub summary: String,
}

pub struct FeedbackSanitizer {
    #[allow(dead_code)]
    config: FeedbackConfig,
}

impl FeedbackSanitizer {
    #[must_use]
    pub fn new(level: u8) -> Self {
        Self {
            config: FeedbackConfig::from_level(level),
        }
    }

    #[must_use]
    pub fn sanitize(
        &self,
        raw_results: &[super::scenario_runner::ScenarioResult],
        iteration: u32,
    ) -> SanitizedFeedback {
        let total_count = raw_results.len();
        let passed_count = raw_results.iter().filter(|r| r.passed).count();
        let failed_count = total_count - passed_count;

        let failures: Vec<SanitizedFailure> = raw_results
            .iter()
            .filter(|r| !r.passed)
            .map(Self::sanitize_failure)
            .collect();

        let summary = format!("{failed_count} of {total_count} behavioral tests failed");

        SanitizedFeedback {
            iteration,
            passed_count,
            failed_count,
            total_count,
            failures,
            summary,
        }
    }

    fn sanitize_failure(result: &super::scenario_runner::ScenarioResult) -> SanitizedFailure {
        let category = Self::categorize_failure(result);
        let description = Self::sanitize_description(&category);
        let hint = Self::generate_hint(&category);
        let spec_text = Self::get_spec_text_reference(&category);

        SanitizedFailure {
            category,
            spec_ref: result.spec_ref.clone(),
            description,
            hint,
            spec_text,
        }
    }

    fn categorize_failure(result: &super::scenario_runner::ScenarioResult) -> String {
        let failed_step = result.steps.iter().find(|s| !s.passed);

        if let Some(step) = failed_step {
            if step.error.as_ref().is_some_and(|e| e.contains("404")) {
                return "Resource Not Found".to_string();
            }
            if step.error.as_ref().is_some_and(|e| e.contains("500")) {
                return "Server Error".to_string();
            }
            if step.error.as_ref().is_some_and(|e| e.contains("timeout")) {
                return "Timeout".to_string();
            }
        }

        match result.category.as_str() {
            "security" => "Security Violation".to_string(),
            "error-handling" => "Error Handling".to_string(),
            "happy-path" => "Happy Path".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    fn sanitize_description(category: &str) -> String {
        match category {
            "Security Violation" => "The system does not properly enforce security constraints.".to_string(),
            "Error Handling" => "The system does not gracefully handle error conditions.".to_string(),
            "Happy Path" => "The primary workflow does not produce expected results.".to_string(),
            "Resource Not Found" => "The system returns incorrect HTTP status codes for missing resources.".to_string(),
            "Server Error" => "The system returns internal server errors instead of handling the request properly.".to_string(),
            "Timeout" => "The system does not complete operations within expected time limits.".to_string(),
            _ => "A behavioral requirement is not satisfied.".to_string(),
        }
    }

    fn generate_hint(category: &str) -> String {
        match category {
            "Security Violation" => {
                "Review the spec's security requirements and ensure all invariants are enforced."
                    .to_string()
            }
            "Error Handling" => {
                "Review edge cases in the spec and ensure proper error responses are returned."
                    .to_string()
            }
            "Happy Path" => "Review the spec's acceptance criteria for this behavior.".to_string(),
            "Resource Not Found" => {
                "Ensure API endpoints return correct HTTP status codes per the spec.".to_string()
            }
            "Server Error" => {
                "Check that all error conditions are handled before reaching internal logic."
                    .to_string()
            }
            "Timeout" => {
                "Review performance requirements in the spec's constraints section.".to_string()
            }
            _ => "Review the spec for the relevant behavior.".to_string(),
        }
    }

    fn get_spec_text_reference(category: &str) -> String {
        match category {
            "Security Violation" => {
                "Review context.invariants for security constraints.".to_string()
            }
            "Error Handling" => {
                "Review behaviors[].edge_cases for required error handling.".to_string()
            }
            "Happy Path" => "Review acceptance_criteria for the expected behavior.".to_string(),
            _ => "Review the relevant behavior in the spec.".to_string(),
        }
    }
}

#[must_use]
pub fn sanitize_results(
    raw_results: &[super::scenario_runner::ScenarioResult],
    iteration: u32,
    level: u8,
) -> SanitizedFeedback {
    let sanitizer = FeedbackSanitizer::new(level);
    sanitizer.sanitize(raw_results, iteration)
}

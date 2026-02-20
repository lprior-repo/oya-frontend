use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScenarioError {
    #[error("Failed to read scenario file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse YAML: {0}")]
    ParseError(#[from] serde_yaml::Error),
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),
    #[error("Setup failed: {0}")]
    SetupFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioIdentity {
    pub id: String,
    #[serde(rename = "spec_ref")]
    pub spec_ref: String,
    #[serde(rename = "spec_version")]
    pub spec_version: String,
    pub category: String,
    pub visibility: String,
    pub priority: String,
    pub description: String,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioSetup {
    pub universe: String,
    #[serde(rename = "initial_state")]
    pub initial_state: String,
    pub preconditions: Vec<Precondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precondition {
    pub description: String,
    pub check: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub method: Option<String>,
    pub url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<serde_json::Value>,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assertion {
    #[serde(rename = "type")]
    pub assertion_type: String,
    pub path: Option<String>,
    pub expected: Option<serde_json::Value>,
    pub operator: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extraction {
    pub name: String,
    pub from: String,
    pub path: Option<String>,
    pub regex: Option<String>,
    #[serde(rename = "group")]
    pub extract_group: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStep {
    pub id: String,
    pub description: String,
    pub action: StepAction,
    pub assertions: Vec<Assertion>,
    pub extractions: Vec<Extraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioTeardown {
    #[serde(rename = "reset_universe")]
    pub reset_universe: bool,
    #[serde(rename = "custom_cleanup")]
    pub custom_cleanup: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub scenario: ScenarioIdentity,
    pub setup: ScenarioSetup,
    pub steps: Vec<ScenarioStep>,
    pub teardown: ScenarioTeardown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub assertions_passed: usize,
    pub assertions_failed: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub scenario_id: String,
    pub spec_ref: String,
    pub category: String,
    pub passed: bool,
    pub steps: Vec<StepResult>,
    pub total_duration_ms: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub spec_id: String,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub results: Vec<ScenarioResult>,
    pub category_breakdown: HashMap<String, CategoryResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResult {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

pub struct ScenarioRunner<S = std::hash::RandomState> {
    http_client: reqwest::Client,
    application_endpoint: String,
    #[allow(dead_code)]
    twin_endpoints: HashMap<String, String, S>,
    extracted_values: HashMap<String, serde_json::Value>,
}

impl<S: std::hash::BuildHasher + Send + Sync> ScenarioRunner<S> {
    #[must_use]
    pub fn new(application_endpoint: &str, twins: HashMap<String, String, S>) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            application_endpoint: application_endpoint.to_string(),
            twin_endpoints: twins,
            extracted_values: HashMap::new(),
        }
    }

    pub async fn run_scenario(&mut self, scenario: &Scenario) -> ScenarioResult {
        let start = std::time::Instant::now();
        let mut step_results = Vec::new();
        let mut passed = true;

        for step in &scenario.steps {
            let step_result = self.execute_step(step).await;
            if !step_result.passed {
                passed = false;
                step_results.push(step_result);
                break;
            }
            step_results.push(step_result);
        }

        ScenarioResult {
            scenario_id: scenario.scenario.id.clone(),
            spec_ref: scenario.scenario.spec_ref.clone(),
            category: scenario.scenario.category.clone(),
            passed,
            steps: step_results,
            total_duration_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            error: None,
        }
    }

    async fn execute_step(&mut self, step: &ScenarioStep) -> StepResult {
        let start = std::time::Instant::now();
        let mut assertions_passed = 0;
        let mut assertions_failed = 0;
        let mut error = None;

        let action_result = self.execute_action(&step.action).await;

        for assertion in &step.assertions {
            match Self::check_assertion(&action_result, assertion) {
                Ok(()) => assertions_passed += 1,
                Err(e) => {
                    assertions_failed += 1;
                    error = Some(e);
                }
            }
        }

        for extraction in &step.extractions {
            self.extract_value(&action_result, extraction);
        }

        StepResult {
            step_id: step.id.clone(),
            passed: assertions_failed == 0,
            duration_ms: u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX),
            assertions_passed,
            assertions_failed,
            error,
        }
    }

    async fn execute_action(&self, action: &StepAction) -> ActionResult {
        match action.action_type.as_str() {
            "http" => {
                let client = &self.http_client;
                let url = action.url.as_ref().map_or_else(String::new, |value| {
                    value.replace("${application.endpoint}", &self.application_endpoint)
                });

                if url.is_empty() {
                    return ActionResult {
                        status: 0,
                        body: "Missing URL for http action".to_string(),
                        response_time_ms: 0,
                    };
                }

                let method = action.method.as_deref().unwrap_or("GET");

                let mut req = match method {
                    "POST" => client.post(&url),
                    "PUT" => client.put(&url),
                    "DELETE" => client.delete(&url),
                    _ => client.get(&url),
                };

                if let Some(headers) = &action.headers {
                    for (key, value) in headers {
                        req = req.header(key, value);
                    }
                }

                if let Some(body) = &action.body {
                    req = req.json(body);
                }

                match req.send().await {
                    Ok(response) => {
                        let status = response.status().as_u16();
                        let body = response.text().await.unwrap_or_default();
                        ActionResult {
                            status,
                            body,
                            response_time_ms: 0,
                        }
                    }
                    Err(e) => ActionResult {
                        status: 0,
                        body: e.to_string(),
                        response_time_ms: 0,
                    },
                }
            }
            _ => ActionResult {
                status: 0,
                body: format!("Unknown action type: {}", action.action_type),
                response_time_ms: 0,
            },
        }
    }

    fn check_assertion(result: &ActionResult, assertion: &Assertion) -> Result<(), String> {
        match assertion.assertion_type.as_str() {
            "status" => {
                let expected = match assertion.expected.as_ref() {
                    Some(value) => value,
                    None => return Err("Missing expected value for status assertion".to_string()),
                };
                let expected_status = u16::try_from(expected.as_u64().unwrap_or(0)).unwrap_or(0);
                if result.status != expected_status {
                    return Err(format!(
                        "Expected status {expected_status}, got {}",
                        result.status
                    ));
                }
            }
            "body_json" => {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&result.body) {
                    if let Some(path) = &assertion.path {
                        let value = json.pointer(path);
                        if let Some(expected) = &assertion.expected {
                            if let Some(actual) = value {
                                if actual != expected {
                                    return Err(format!(
                                        "Path {path}: expected {expected}, got {actual}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn extract_value(&mut self, result: &ActionResult, extraction: &Extraction) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&result.body) {
            if let Some(path) = &extraction.path {
                if let Some(value) = json.pointer(path) {
                    let _ = self
                        .extracted_values
                        .insert(extraction.name.clone(), value.clone());
                }
            }
        }
    }
}

pub struct ActionResult {
    pub status: u16,
    pub body: String,
    pub response_time_ms: u64,
}

/// Run validation on a directory of scenarios.
///
/// # Errors
/// Returns an error if reading directory or files fails.
pub async fn run_validation<S: std::hash::BuildHasher + Send + Sync>(
    scenario_dir: &Path,
    application_endpoint: &str,
    twins: HashMap<String, String, S>,
) -> Result<ValidationReport, ScenarioError> {
    let mut results = Vec::new();
    let mut runner = ScenarioRunner::new(application_endpoint, twins);

    let entries = fs::read_dir(scenario_dir)?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "yaml") {
            let content = fs::read_to_string(&path)?;
            let scenario: Scenario = serde_yaml::from_str(&content)?;
            let result = runner.run_scenario(&scenario).await;
            results.push(result);
        }
    }

    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = total - passed;

    let mut category_breakdown = HashMap::new();
    for result in &results {
        let entry = category_breakdown
            .entry(result.category.clone())
            .or_insert(CategoryResult {
                total: 0,
                passed: 0,
                failed: 0,
            });
        entry.total += 1;
        if result.passed {
            entry.passed += 1;
        } else {
            entry.failed += 1;
        }
    }

    Ok(ValidationReport {
        spec_id: "flow-wasm-v1".to_string(),
        total_scenarios: total,
        passed_scenarios: passed,
        failed_scenarios: failed,
        results,
        category_breakdown,
    })
}

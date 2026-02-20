use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpecValidationMetrics {
    pub timestamp: DateTime<Utc>,
    pub spec_id: String,
    pub spec_version: String,
    pub overall_score: u32,
    pub passed: bool,
    pub category_scores: HashMap<String, u32>,
    pub errors_count: usize,
    pub warnings_count: usize,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScenarioValidationMetrics {
    pub timestamp: DateTime<Utc>,
    pub spec_id: String,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub category_breakdown: HashMap<String, CategoryStats>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CategoryStats {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QualityGateIteration {
    pub iteration: u32,
    pub timestamp: DateTime<Utc>,
    pub spec_passed: bool,
    pub spec_score: u32,
    pub scenarios_passed: bool,
    pub scenarios_total: usize,
    pub scenarios_passed_count: usize,
    pub overall_passed: bool,
    pub failure_category: Option<String>,
    pub feedback_level: u8,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QualityGateSession {
    pub session_id: String,
    pub spec_id: String,
    pub spec_version: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub iterations: Vec<QualityGateIteration>,
    pub total_duration_ms: u64,
    pub status: SessionStatus,
    pub escalated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "escalated")]
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsSummary {
    pub total_sessions: usize,
    pub passed_sessions: usize,
    pub failed_sessions: usize,
    pub escalated_sessions: usize,
    pub avg_iterations_to_pass: f64,
    pub avg_duration_minutes: f64,
    pub most_common_failure_categories: Vec<(String, usize)>,
    pub avg_spec_score: f64,
}

pub struct MetricsStore {
    pub(crate) base_path: PathBuf,
    pub(crate) data: Arc<RwLock<MetricsData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct MetricsData {
    pub(crate) spec_validations: Vec<SpecValidationMetrics>,
    pub(crate) scenario_validations: Vec<ScenarioValidationMetrics>,
    pub(crate) sessions: Vec<QualityGateSession>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScenarioValidationMetrics {
    pub timestamp: DateTime<Utc>,
    pub spec_id: String,
    pub total_scenarios: usize,
    pub passed_scenarios: usize,
    pub failed_scenarios: usize,
    pub category_breakdown: HashMap<String, CategoryStats>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategoryStats {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    base_path: PathBuf,
    data: Arc<RwLock<MetricsData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct MetricsData {
    spec_validations: Vec<SpecValidationMetrics>,
    scenario_validations: Vec<ScenarioValidationMetrics>,
    sessions: Vec<QualityGateSession>,
}

impl MetricsStore {
    pub fn new(base_path: &Path) -> Self {
        let data_path = base_path.join("quality-metrics");
        std::fs::create_dir_all(&data_path).ok();

        let data = if let Ok(content) = Self::load_data(&data_path) {
            content
        } else {
            MetricsData::default()
        };

        Self {
            base_path: data_path,
            data: Arc::new(RwLock::new(data)),
        }
    }

    fn load_data(path: &Path) -> Result<MetricsData, Box<dyn std::error::Error>> {
        let metrics_file = path.join("metrics.json");
        if metrics_file.exists() {
            let content = std::fs::read_to_string(&metrics_file)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(MetricsData::default())
        }
    }

    fn save_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = self
            .data
            .read()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        let json = serde_json::to_string_pretty(&*data)?;
        let metrics_file = self.base_path.join("metrics.json");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&metrics_file)?;

        use std::io::Write;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn record_spec_validation(
        &self,
        metrics: SpecValidationMetrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut data = self
                .data
                .write()
                .map_err(|e| format!("Failed to acquire lock: {}", e))?;
            data.spec_validations.push(metrics);
        }
        self.save_data()
    }

    pub fn record_scenario_validation(
        &self,
        metrics: ScenarioValidationMetrics,
    ) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut data = self
                .data
                .write()
                .map_err(|e| format!("Failed to acquire lock: {}", e))?;
            data.scenario_validations.push(metrics);
        }
        self.save_data()
    }

    pub fn start_session(
        &self,
        spec_id: &str,
        spec_version: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let timestamp = Utc::now();

        {
            let mut data = self
                .data
                .write()
                .map_err(|e| format!("Failed to acquire lock: {}", e))?;

            let session = QualityGateSession {
                session_id: session_id.clone(),
                spec_id: spec_id.to_string(),
                spec_version: spec_version.to_string(),
                started_at: timestamp,
                completed_at: None,
                iterations: Vec::new(),
                total_duration_ms: 0,
                status: SessionStatus::InProgress,
                escalated: false,
            };

            data.sessions.push(session);
        }
        self.save_data()?;

        Ok(session_id)
    }

    pub fn record_iteration(
        &self,
        session_id: &str,
        iteration: QualityGateIteration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut data = self
                .data
                .write()
                .map_err(|e| format!("Failed to acquire lock: {}", e))?;

            if let Some(session) = data
                .sessions
                .iter_mut()
                .find(|s| s.session_id == session_id)
            {
                let passed = iteration.overall_passed;
                session.iterations.push(iteration);

                let now = Utc::now();
                if passed {
                    session.status = SessionStatus::Passed;
                    session.completed_at = Some(now);
                } else if session.iterations.len() >= 5 {
                    session.status = SessionStatus::Failed;
                    session.completed_at = Some(now);
                    session.escalated = true;
                }
            }
        }
        self.save_data()
    }

    pub fn get_session(&self, session_id: &str) -> Option<QualityGateSession> {
        let data = self.data.read().ok()?;
        data.sessions
            .iter()
            .find(|s| s.session_id == session_id)
            .cloned()
    }

    pub fn get_summary(&self) -> MetricsSummary {
        let data_guard = self.data.read();
        let empty_data = MetricsData::default();
        let data = data_guard.as_deref().unwrap_or(&empty_data);

        let total_sessions = data.sessions.len();
        let passed_sessions = data
            .sessions
            .iter()
            .filter(|s| s.status == SessionStatus::Passed)
            .count();
        let failed_sessions = data
            .sessions
            .iter()
            .filter(|s| s.status == SessionStatus::Failed)
            .count();
        let escalated_sessions = data.sessions.iter().filter(|s| s.escalated).count();

        let passed_sessions_refs: Vec<_> = data
            .sessions
            .iter()
            .filter(|s| s.status == SessionStatus::Passed)
            .collect();

        let avg_iterations = if !passed_sessions_refs.is_empty() {
            let total_iterations: usize = passed_sessions_refs
                .iter()
                .map(|s| s.iterations.len())
                .sum();
            total_iterations as f64 / passed_sessions_refs.len() as f64
        } else {
            0.0
        };

        let total_duration_ms: u64 = passed_sessions_refs
            .iter()
            .map(|s| s.total_duration_ms)
            .sum();
        let avg_duration_minutes = if !passed_sessions_refs.is_empty() {
            total_duration_ms as f64 / passed_sessions_refs.len() as f64 / 60000.0
        } else {
            0.0
        };

        let spec_scores: Vec<f64> = data
            .spec_validations
            .iter()
            .map(|v| v.overall_score as f64)
            .collect();
        let avg_spec_score = if !spec_scores.is_empty() {
            spec_scores.iter().sum::<f64>() / spec_scores.len() as f64
        } else {
            0.0
        };

        let mut failure_counts: HashMap<String, usize> = HashMap::new();
        for session in &data.sessions {
            for iteration in &session.iterations {
                if !iteration.overall_passed {
                    if let Some(category) = &iteration.failure_category {
                        *failure_counts.entry(category.clone()).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut failures: Vec<_> = failure_counts.into_iter().collect();
        failures.sort_by(|a, b| b.1.cmp(&a.1));

        MetricsSummary {
            total_sessions,
            passed_sessions,
            failed_sessions,
            escalated_sessions,
            avg_iterations_to_pass: avg_iterations,
            avg_duration_minutes,
            most_common_failure_categories: failures,
            avg_spec_score,
        }
    }

    pub fn export_report(&self, format: &str) -> Result<String, Box<dyn std::error::Error>> {
        let summary = self.get_summary();

        match format {
            "json" => Ok(serde_json::to_string_pretty(&summary)?),
            "text" => Ok(self.format_text_report(&summary)),
            _ => Err("Unsupported format. Use 'json' or 'text'".into()),
        }
    }

    fn format_text_report(&self, summary: &MetricsSummary) -> String {
        let failures_str = if summary.most_common_failure_categories.is_empty() {
            "  (none)".to_string()
        } else {
            summary
                .most_common_failure_categories
                .iter()
                .map(|(cat, count)| format!("    - {} ({} times)", cat, count))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let passed_pct = if summary.total_sessions > 0 {
            (summary.passed_sessions as f64 / summary.total_sessions as f64 * 100.0)
        } else {
            0.0
        };
        let failed_pct = if summary.total_sessions > 0 {
            (summary.failed_sessions as f64 / summary.total_sessions as f64 * 100.0)
        } else {
            0.0
        };
        let escalated_pct = if summary.total_sessions > 0 {
            (summary.escalated_sessions as f64 / summary.total_sessions as f64 * 100.0)
        } else {
            0.0
        };

        format!(
            "
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  QUALITY GATE METRICS REPORT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Sessions:
    Total: {}
    Passed: {} ({:.1}%)
    Failed: {} ({:.1}%)
    Escalated: {} ({:.1}%)

  Performance:
    Avg iterations to pass: {:.2}
    Avg duration: {:.2} minutes
    Avg spec quality score: {:.1}/100

  Common Failure Categories:
{}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
        ",
            summary.total_sessions,
            summary.passed_sessions,
            passed_pct,
            summary.failed_sessions,
            failed_pct,
            summary.escalated_sessions,
            escalated_pct,
            summary.avg_iterations_to_pass,
            summary.avg_duration_minutes,
            summary.avg_spec_score,
            failures_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_store_new() {
        let store = MetricsStore::new(Path::new("/tmp/test-metrics"));

        assert_eq!(
            store.base_path,
            PathBuf::from("/tmp/test-metrics/quality-metrics")
        );
    }

    #[test]
    fn test_spec_validation_metrics() {
        let metrics = SpecValidationMetrics {
            timestamp: Utc::now(),
            spec_id: "test-spec".to_string(),
            spec_version: "1.0.0".to_string(),
            overall_score: 90,
            passed: true,
            category_scores: HashMap::new(),
            errors_count: 0,
            warnings_count: 1,
            duration_ms: 500,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let deserialized: SpecValidationMetrics = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.spec_id, metrics.spec_id);
        assert_eq!(deserialized.overall_score, metrics.overall_score);
    }
}

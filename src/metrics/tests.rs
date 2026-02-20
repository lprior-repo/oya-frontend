use super::{MetricsStore, SpecValidationMetrics};
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[test]
fn test_metrics_store_new() {
    let store = MetricsStore::new(Path::new("/tmp/test-metrics"));

    assert_eq!(
        store.base_path,
        PathBuf::from("/tmp/test-metrics/quality-metrics")
    );
}

#[test]
fn test_spec_validation_metrics() -> anyhow::Result<()> {
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

    let json = serde_json::to_string(&metrics)?;
    let deserialized: SpecValidationMetrics = serde_json::from_str(&json)?;

    assert_eq!(deserialized.spec_id, metrics.spec_id);
    assert_eq!(deserialized.overall_score, metrics.overall_score);

    Ok(())
}

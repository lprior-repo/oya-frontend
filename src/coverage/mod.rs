use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecCoverage {
    pub spec_id: String,
    pub total_behaviors: usize,
    pub covered_behaviors: usize,
    pub total_edge_cases: usize,
    pub covered_edge_cases: usize,
    pub coverage_percentage: f64,
    pub missing_behaviors: Vec<String>,
    pub missing_edge_cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub specs: Vec<SpecCoverage>,
    pub overall_coverage: f64,
    pub total_behaviors: usize,
    pub total_edge_cases: usize,
    pub covered_behaviors: usize,
    pub covered_edge_cases: usize,
    pub common_gaps: Vec<String>,
}

pub struct CoverageAnalyzer {
    specs_dir: PathBuf,
    scenarios_dir: PathBuf,
}

impl CoverageAnalyzer {
    #[must_use]
    pub fn new(specs_dir: &Path, scenarios_dir: &Path) -> Self {
        Self {
            specs_dir: specs_dir.to_path_buf(),
            scenarios_dir: scenarios_dir.to_path_buf(),
        }
    }

    /// Analyze scenario coverage.
    ///
    /// # Errors
    /// Returns an error if finding files or reading content fails.
    pub fn analyze(&self) -> Result<CoverageReport, Box<dyn std::error::Error>> {
        let mut spec_coverage = Vec::new();

        for spec_file in self.find_spec_files()? {
            if let Ok(coverage) = self.analyze_spec(&spec_file) {
                spec_coverage.push(coverage);
            }
        }

        let (total_behaviors, covered_behaviors) = if spec_coverage.is_empty() {
            (0, 0)
        } else {
            let t: usize = spec_coverage.iter().map(|s| s.total_behaviors).sum();
            let c: usize = spec_coverage.iter().map(|s| s.covered_behaviors).sum();
            (t, c)
        };

        let total_edge_cases: usize = spec_coverage.iter().map(|s| s.total_edge_cases).sum();
        let covered_edge_cases: usize = spec_coverage.iter().map(|s| s.covered_edge_cases).sum();

        let mut gap_counts: HashMap<String, usize> = HashMap::new();
        for spec in &spec_coverage {
            for behavior in &spec.missing_behaviors {
                *gap_counts.entry(behavior.clone()).or_insert(0) += 1;
            }
            for edge_case in &spec.missing_edge_cases {
                *gap_counts.entry(edge_case.clone()).or_insert(0) += 1;
            }
        }

        let mut sorted_gaps: Vec<_> = gap_counts.into_iter().collect();
        sorted_gaps.sort_by(|a, b| b.1.cmp(&a.1));
        let common_gaps: Vec<String> = sorted_gaps.into_iter().take(10).map(|(s, _)| s).collect();

        Ok(CoverageReport {
            specs: spec_coverage,
            overall_coverage: if total_behaviors > 0 {
                #[allow(clippy::cast_precision_loss)]
                {
                    covered_behaviors as f64 / total_behaviors as f64 * 100.0
                }
            } else {
                0.0
            },
            total_behaviors,
            total_edge_cases,
            covered_behaviors,
            covered_edge_cases,
            common_gaps,
        })
    }

    fn find_spec_files(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut specs = Vec::new();
        if self.specs_dir.exists() {
            let entries = fs::read_dir(&self.specs_dir)?;
            for entry in entries {
                let path = entry?.path();
                if path.extension().is_some_and(|ext| ext == "yaml") {
                    specs.push(path);
                }
            }
        }
        specs.sort();
        Ok(specs)
    }

    fn analyze_spec(&self, spec_path: &Path) -> Result<SpecCoverage, Box<dyn std::error::Error>> {
        let spec_content = fs::read_to_string(spec_path)?;
        let yaml: Value = serde_yaml::from_str(&spec_content)?;

        let spec_id = yaml["specification"]["identity"]["id"]
            .as_str()
            .ok_or("unknown")?
            .to_string();

        let mut behavior_ids: HashSet<String> = HashSet::new();
        let mut edge_case_ids: HashSet<String> = HashSet::new();

        if let Some(behaviors) = yaml["specification"]["behaviors"].as_sequence() {
            for behavior in behaviors {
                if let Some(id) = behavior["id"].as_str() {
                    let _ = behavior_ids.insert(id.to_string());
                }
                if let Some(edge_cases) = behavior["edge_cases"].as_sequence() {
                    for edge_case in edge_cases {
                        if let Some(id) = edge_case["id"].as_str() {
                            let _ = edge_case_ids.insert(id.to_string());
                        }
                    }
                }
            }
        }

        let mut scenario_behavior_ids: HashSet<String> = HashSet::new();
        let scenario_edge_case_ids: HashSet<String> = HashSet::new();

        if let Ok(scenarios) = self.find_scenarios_for_spec(&spec_id) {
            for scenario in scenarios {
                if let Some(steps) = scenario["steps"].as_sequence() {
                    for step in steps {
                        if let Some(assertions) = step["assertions"].as_sequence() {
                            for assertion in assertions {
                                if let Some(behavior_ref) = assertion["behavior_ref"].as_str() {
                                    let _ = scenario_behavior_ids.insert(behavior_ref.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        let covered_behaviors = behavior_ids.intersection(&scenario_behavior_ids).count();
        let covered_edge_cases = edge_case_ids.intersection(&scenario_edge_case_ids).count();

        let mut missing_behaviors: Vec<String> = behavior_ids
            .difference(&scenario_behavior_ids)
            .cloned()
            .collect();
        missing_behaviors.sort();

        let mut missing_edge_cases: Vec<String> = edge_case_ids
            .difference(&scenario_edge_case_ids)
            .cloned()
            .collect();
        missing_edge_cases.sort();

        Ok(SpecCoverage {
            spec_id,
            total_behaviors: behavior_ids.len(),
            covered_behaviors,
            total_edge_cases: edge_case_ids.len(),
            covered_edge_cases,
            coverage_percentage: if behavior_ids.is_empty() {
                0.0
            } else {
                #[allow(clippy::cast_precision_loss)]
                {
                    covered_behaviors as f64 / behavior_ids.len() as f64 * 100.0
                }
            },
            missing_behaviors,
            missing_edge_cases,
        })
    }

    fn find_scenarios_for_spec(
        &self,
        spec_id: &str,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let mut scenarios = Vec::new();
        if self.scenarios_dir.exists() {
            let entries = fs::read_dir(&self.scenarios_dir)?;
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "yaml") {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(yaml) = serde_yaml::from_str::<Value>(&content) {
                        if let Some(scenario) = yaml.get("scenario") {
                            if let Some(ref_str) = scenario["spec_ref"].as_str() {
                                if ref_str == spec_id {
                                    scenarios.push(scenario.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(scenarios)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analyzer() {
        let _ = CoverageAnalyzer::new(Path::new("."), Path::new("."));
    }
}

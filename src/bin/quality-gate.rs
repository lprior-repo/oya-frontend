#[cfg(not(target_arch = "wasm32"))]
use clap::{Parser, Subcommand};
#[cfg(not(target_arch = "wasm32"))]
use oya_frontend::feedback::sanitize_results;
#[cfg(not(target_arch = "wasm32"))]
use oya_frontend::linter::{LintReport, SpecLinter};
#[cfg(not(target_arch = "wasm32"))]
use oya_frontend::scenario_runner::{run_validation, ValidationReport};
#[cfg(not(target_arch = "wasm32"))]
use serde::Serialize;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(name = "quality-gate")]
#[command(about = "Autonomous Development Quality Gate CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Subcommand)]
enum Commands {
    /// Validate a specification
    LintSpec {
        /// Path to the spec file
        spec_path: PathBuf,
        /// Path to linter rules
        #[arg(long, default_value = "specs/linter/rules.yaml")]
        rules_path: PathBuf,
    },
    /// Run holdout scenarios
    Validate {
        /// Path to scenarios directory
        scenarios_path: PathBuf,
        /// Application endpoint
        #[arg(long, default_value = "http://localhost:8081")]
        app_endpoint: String,
        /// Feedback level (1-5)
        #[arg(long, default_value = "3")]
        level: u8,
    },
    /// Generate comprehensive quality report
    Report {
        /// Output format: json or text
        #[arg(long, default_value = "text")]
        format: String,
    },
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::LintSpec {
            spec_path,
            rules_path,
        } => {
            println!("🔍 Linting spec: {}", spec_path.display());
            let linter = SpecLinter::new(&rules_path)?;
            let report = linter.lint(&spec_path)?;
            print_report(&report);
            if report.passed {
                println!("\n✅ SPEC APPROVED");
                Ok(())
            } else {
                eprintln!("\n❌ SPEC REJECTED");
                std::process::exit(1);
            }
        }

        Commands::Validate {
            scenarios_path,
            app_endpoint,
            level,
        } => {
            println!("🎭 Running holdout scenarios...");
            let twins = std::collections::HashMap::new();
            let results = run_validation(&scenarios_path, &app_endpoint, twins).await?;
            print_validation_results(&results);

            if results.failed_scenarios == 0 {
                println!("\n✅ VALIDATION PASSED");
                Ok(())
            } else {
                let feedback = sanitize_results(&results.results, 1, level);
                println!("\n❌ VALIDATION FAILED: {}", feedback.summary);
                std::process::exit(1);
            }
        }

        Commands::Report { format } => {
            let report = run_quality_gates();
            match format.as_str() {
                "json" => print_json_report(&report)?,
                _ => print_text_quality_report(&report),
            }
            if report.critical_failures > 0 {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn print_report(report: &LintReport) {
    println!(
        "Spec: {} v{} | Score: {}/100",
        report.spec_id, report.spec_version, report.overall_score
    );
    for (cat, score) in &report.categories {
        println!("  - {}: {} ({})", cat, score.score, score.details);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn print_validation_results(results: &ValidationReport) {
    println!(
        "Report: {} | Total: {} | Passed: {} | Failed: {}",
        results.spec_id,
        results.total_scenarios,
        results.passed_scenarios,
        results.failed_scenarios
    );
}

// ===========================================================================
// Comprehensive Quality Report
// ===========================================================================

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize)]
struct QualityReport {
    timestamp: String,
    gates: Vec<GateResult>,
    total_tests: Option<String>,
    critical_failures: u32,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Clone)]
struct GateResult {
    name: String,
    passed: bool,
    output: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn run_gate(name: &str, args: &[&str]) -> GateResult {
    let output = Command::new("cargo").args(args).output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let combined = if stdout.is_empty() {
                stderr
            } else if stderr.is_empty() {
                stdout
            } else {
                format!("{stdout}\n{stderr}")
            };
            let passed = out.status.success();
            if !passed {
                eprintln!("FAIL: {name}");
            }
            GateResult {
                name: name.to_string(),
                passed,
                output: combined,
            }
        }
        Err(e) => GateResult {
            name: name.to_string(),
            passed: false,
            output: format!("Failed to execute: {e}"),
        },
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn extract_test_count(output: &str) -> Option<String> {
    for line in output.lines().rev() {
        if line.contains("test result:") {
            return Some(line.to_string());
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn run_quality_gates() -> QualityReport {
    let timestamp = chrono::Utc::now().to_rfc3339();

    let check = run_gate("check", &["check"]);
    let test_output = run_gate("test", &["test", "--", "--color=never"]);
    let clippy = run_gate(
        "clippy",
        &[
            "clippy",
            "--",
            "-D",
            "warnings",
            "-D",
            "clippy::unwrap_used",
            "-D",
            "clippy::expect_used",
            "-D",
            "clippy::panic",
        ],
    );
    let fmt = run_gate("fmt", &["fmt", "--check"]);

    let total_tests = extract_test_count(&test_output.output);
    let test_result = test_output;

    let gates = vec![check, test_result, clippy, fmt];
    let critical_failures = gates.iter().filter(|g| !g.passed).count() as u32;

    QualityReport {
        timestamp,
        gates,
        total_tests,
        critical_failures,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn print_text_quality_report(report: &QualityReport) {
    println!("=== Quality Gate Report ===");
    println!("Timestamp: {}", report.timestamp);
    println!();

    for gate in &report.gates {
        let status = if gate.passed { "PASS" } else { "FAIL" };
        println!("[{status}] {}", gate.name);
    }

    if let Some(ref tests) = report.total_tests {
        println!("\n{tests}");
    }

    println!();
    if report.critical_failures == 0 {
        println!("Result: ALL GATES PASSED");
    } else {
        println!("Result: {} CRITICAL FAILURE(S)", report.critical_failures);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn print_json_report(report: &QualityReport) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(report)?;
    println!("{json}");
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

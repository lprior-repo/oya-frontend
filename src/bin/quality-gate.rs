use clap::{Parser, Subcommand};
use new_app::feedback::sanitize_results;
use new_app::linter::{LintReport, SpecLinter};
use new_app::scenario_runner::{run_validation, ValidationReport};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "quality-gate")]
#[command(about = "Autonomous Development Quality Gate CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::LintSpec {
            spec_path,
            rules_path,
        } => {
            println!("🔍 Linting spec: {:?}", spec_path);
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
    }
}

fn print_report(report: &LintReport) {
    println!("Spec: {} v{} | Score: {}/100", report.spec_id, report.spec_version, report.overall_score);
    for (cat, score) in &report.categories {
        println!("  - {}: {} ({})", cat, score.score, score.details);
    }
}

fn print_validation_results(results: &ValidationReport) {
    println!("Report: {} | Total: {} | Passed: {} | Failed: {}", 
        results.spec_id, results.total_scenarios, results.passed_scenarios, results.failed_scenarios);
}

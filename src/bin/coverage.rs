#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use oya_frontend::coverage::{CoverageAnalyzer, CoverageReport};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(name = "coverage")]
#[command(about = "Analyze scenario coverage")]
struct Args {
    #[arg(short, long, default_value = "specs")]
    specs_dir: PathBuf,

    #[arg(short, long, default_value = "../scenarios-vault")]
    scenarios_dir: PathBuf,

    #[arg(short, long, default_value = "text")]
    format: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Analyzing scenario coverage...");

    let analyzer = CoverageAnalyzer::new(&args.specs_dir, &args.scenarios_dir);
    let report = analyzer.analyze()?;

    match args.format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&report)?;
            println!("{json}");
        }
        "text" => {
            print_text_report(&report);
        }
        _ => {
            eprintln!("Unsupported format: {}", args.format);
            return Err("Use 'json' or 'text'".into());
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn print_text_report(report: &CoverageReport) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  SCENARIO COVERAGE REPORT");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("  Overall Coverage: {:.1}%", report.overall_coverage);
    println!();
    println!("  Spec Coverage:");
    for spec in &report.specs {
        println!(
            "    {}: {:.1}% ({}/{} behaviors, {}/{} edge cases)",
            spec.spec_id,
            spec.coverage_percentage,
            spec.covered_behaviors,
            spec.total_behaviors,
            spec.covered_edge_cases,
            spec.total_edge_cases
        );
    }
    println!();

    if !report.common_gaps.is_empty() {
        println!("  Common Coverage Gaps:");
        for gap in &report.common_gaps {
            println!("    - {gap}");
        }
    }

    println!(
        "\n  Totals: {} behaviors, {} edge cases",
        report.total_behaviors, report.total_edge_cases
    );
    println!(
        "  Covered: {} behaviors, {} edge cases",
        report.covered_behaviors, report.covered_edge_cases
    );
    println!(
        "  Missing: {} behaviors, {} edge cases",
        report.total_behaviors - report.covered_behaviors,
        report.total_edge_cases - report.covered_edge_cases
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {}

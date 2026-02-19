use crate::metrics::{MetricsStore, SessionStatus};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "quality-dashboard")]
#[command(about = "View quality gate metrics")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Show metrics summary")]
    Summary,

    #[command(about = "Show recent sessions")]
    Sessions {
        #[arg(short, long, default_value = "10")]
        count: usize,
    },

    #[command(about = "Export metrics report")]
    Export {
        #[arg(short, long, default_value = "text")]
        format: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let metrics_store = MetricsStore::new(&PathBuf::from("."));

    match args.command {
        Commands::Summary => {
            let summary = metrics_store.get_summary();
            print_summary(&summary);
        }

        Commands::Sessions { count } => {
            print_sessions(&metrics_store, count);
        }

        Commands::Export { format, output } => {
            let output_path = output.unwrap_or_else(|| PathBuf::from("metrics-report.txt"));
            export_metrics(&metrics_store, &format, &output_path)?;
        }
    }

    Ok(())
}

pub fn print_summary(summary: &crate::metrics::MetricsSummary) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  QUALITY GATE SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("  Sessions:");
    println!("    Total: {}", summary.total_sessions);

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

    println!(
        "    Passed: {} ({:.1}%)",
        summary.passed_sessions, passed_pct
    );
    println!(
        "    Failed: {} ({:.1}%)",
        summary.failed_sessions, failed_pct
    );
    println!(
        "    Escalated: {} ({:.1}%)",
        summary.escalated_sessions, escalated_pct
    );
    println!();
    println!("  Performance:");
    println!(
        "    Avg iterations to pass: {:.2}",
        summary.avg_iterations_to_pass
    );
}

pub fn print_sessions(_store: &MetricsStore, _count: usize) {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  RECENT SESSIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("  (Session listing currently limited to summary view)");
}

pub fn format_status(status: &SessionStatus) -> &'static str {
    match status {
        SessionStatus::Passed => "✓",
        SessionStatus::Failed => "✗",
        SessionStatus::Escalated => "!",
        SessionStatus::InProgress => "→",
    }
}

pub fn export_metrics(
    store: &MetricsStore,
    format: &str,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let summary = store.get_summary();

    let report = match format {
        "json" => serde_json::to_string_pretty(&summary)?,
        "text" => format_text_summary(&summary),
        _ => return Err("Unsupported format".into()),
    };

    std::fs::write(output, report)?;

    println!("✅ Exported metrics to {:?}", output);
    Ok(())
}

fn format_text_summary(summary: &crate::metrics::MetricsSummary) -> String {
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
        "Quality Gate Metrics
====================

Total Sessions: {}
Passed: {} ({:.1}%)
Failed: {} ({:.1}%)
Escalated: {} ({:.1}%)

Avg Iterations to Pass: {:.2}
",
        summary.total_sessions,
        summary.passed_sessions,
        passed_pct,
        summary.failed_sessions,
        failed_pct,
        summary.escalated_sessions,
        escalated_pct,
        summary.avg_iterations_to_pass
    )
}

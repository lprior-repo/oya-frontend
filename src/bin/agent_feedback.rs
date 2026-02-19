#[cfg(not(target_arch = "wasm32"))]
use clap::{Parser, Subcommand};
#[cfg(not(target_arch = "wasm32"))]
use new_app::agent_feedback::{FailureCategory, FeedbackGenerator, FeedbackRequest};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(name = "agent-feedback")]
#[command(about = "Generate feedback for AI agents")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate feedback for single failure")]
    Generate {
        #[arg(short, long)]
        category: FailureCategory,
        #[arg(long)]
        spec_ref: Option<String>,
        #[arg(long)]
        iteration: u32,
    },

    #[command(about = "Generate feedback from validation results")]
    Batch {
        #[arg(short, long)]
        spec_id: String,
        #[arg(long)]
        validation_results_path: PathBuf,
    },
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let generator = FeedbackGenerator::new();

    match args.command {
        Commands::Generate {
            category,
            spec_ref,
            iteration,
        } => {
            let request = FeedbackRequest {
                failure_category: category,
                spec_ref: spec_ref.unwrap_or_else(|| "unknown".to_string()),
                iteration,
                failure_context: format!("Implementation attempt {iteration}"),
            };

            let feedback = generator.generate(&request);
            println!("{}", feedback.message);
        }

        Commands::Batch {
            spec_id,
            validation_results_path,
        } => {
            let spec_content = std::fs::read_to_string(&validation_results_path)?;
            let validation: serde_json::Value = serde_json::from_str(&spec_content)?;

            let mut requests = Vec::new();
            if let Some(results) = validation["results"].as_array() {
                for entry in results {
                    if let Some(passed) = entry["passed"].as_bool() {
                        if !passed {
                            requests.push(FeedbackRequest {
                                failure_category: FailureCategory::Validation,
                                spec_ref: spec_id.clone(),
                                iteration: 0,
                                failure_context: format!(
                                    "Scenario failed: {}",
                                    entry["id"].as_str().unwrap_or("unknown")
                                ),
                            });
                        }
                    }
                }
            }

            let feedback_batch = generator.generate_batch(&requests);
            for feedback in &feedback_batch {
                println!("--- FEEDBACK: {} ---", feedback.category);
                println!("Priority: {}", feedback.priority);
                println!("Message: {}", feedback.message);
                println!("Hints: {:?}", feedback.hints);
            }
        }
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

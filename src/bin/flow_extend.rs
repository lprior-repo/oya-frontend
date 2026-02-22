#[cfg(not(target_arch = "wasm32"))]
use clap::{Parser, Subcommand};
#[cfg(not(target_arch = "wasm32"))]
use oya_frontend::flow_extender::{apply_extension, preview_extension, suggest_extensions};
#[cfg(not(target_arch = "wasm32"))]
use oya_frontend::Workflow;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(name = "flow-extend")]
#[command(about = "Suggest and apply workflow extensions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Subcommand)]
enum Command {
    /// Print extension suggestions for a workflow JSON file.
    Suggest {
        /// Path to the workflow JSON file.
        workflow_path: PathBuf,
    },
    /// Preview one extension without mutating workflow.
    Preview {
        /// Path to the workflow JSON file.
        workflow_path: PathBuf,
        /// Extension key to preview.
        extension_key: String,
    },
    /// Apply one extension to a workflow JSON file.
    Apply {
        /// Path to the input workflow JSON file.
        workflow_path: PathBuf,
        /// Extension key to apply.
        extension_key: String,
        /// Optional output path. Defaults to in-place write.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_workflow(path: &PathBuf) -> Result<Workflow, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let workflow = serde_json::from_str::<Workflow>(&content)?;
    Ok(workflow)
}

#[cfg(not(target_arch = "wasm32"))]
fn write_workflow(path: &PathBuf, workflow: &Workflow) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(workflow)?;
    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Suggest { workflow_path } => {
            let workflow = parse_workflow(&workflow_path)?;
            let suggestions = suggest_extensions(&workflow);

            if suggestions.is_empty() {
                println!("No extension suggestions. Workflow already covers current principles.");
            } else {
                for item in &suggestions {
                    println!("- {} ({:?}): {}", item.key, item.priority, item.rationale);
                }
            }
        }
        Command::Preview {
            workflow_path,
            extension_key,
        } => {
            let workflow = parse_workflow(&workflow_path)?;
            let preview = preview_extension(&workflow, &extension_key).map_err(|message| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
            })?;

            match preview {
                Some(patch) => {
                    println!("{}", serde_json::to_string_pretty(&patch)?);
                }
                None => {
                    println!("No preview changes for extension '{}'.", extension_key);
                }
            }
        }
        Command::Apply {
            workflow_path,
            extension_key,
            output,
        } => {
            let mut workflow = parse_workflow(&workflow_path)?;
            let applied = apply_extension(&mut workflow, &extension_key).map_err(|message| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
            })?;

            let target_path = output.as_ref().unwrap_or(&workflow_path);
            write_workflow(target_path, &workflow)?;

            println!(
                "Applied {} and created {} node(s). Saved to {}",
                applied.key,
                applied.created_nodes.len(),
                target_path.display()
            );
        }
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

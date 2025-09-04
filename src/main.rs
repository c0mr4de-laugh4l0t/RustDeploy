use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "RustDeploy", about = "Deploy in one command")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deploy an app to the cloud
    Deploy {
        /// Path to the app folder
        path: String,
    },
    /// View logs of the deployed app
    Logs,
    /// Check status of the deployed app
    Status,
}

fn docker_build_and_push(path: &str, image_name: &str) -> std::io::Result<()> {
    // Build docker image
    let build_status = Command::new("docker")
        .args(["build", "-t", image_name, path])
        .status()?;

    if !build_status.success() {
        eprintln!("Failed to build Docker image");
        return Ok(());
    }

    // Push docker image
    let push_status = Command::new("docker")
        .args(["push", image_name])
        .status()?;

    if !push_status.success() {
        eprintln!("Failed to push Docker image");
        return Ok(());
    }

    println!("Docker image {} built and pushed", image_name);
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Deploy { path } => {
            println!("Deploying app from path: {}", path);
            let image_name = "yourdockerhubusername/rustdeploy-demo:latest";
            if let Err(e) = docker_build_and_push(&path, image_name) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Logs => {
            println!("Fetching logs...");
            // Day 6: implement API call
        }
        Commands::Status => {
            println!("Checking deployment status...");
            // Day 6: implement API call
        }
    }
}
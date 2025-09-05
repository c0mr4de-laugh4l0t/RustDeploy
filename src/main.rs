use clap::{Parser, Subcommand};
use std::process::Command;
use std::path::Path;

/// RustDeploy - Simple Rust-based deployment CLI
#[derive(Parser)]
#[command(name = "rustdeploy")]
#[command(about = "RustDeploy - Simple Docker build & push tool written in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a Docker image from a given path
    Build {
        /// Path to the app directory containing Dockerfile
        path: String,
        /// Image name (e.g. user/repo:tag)
        image: String,
    },

    /// Push an already built Docker image
    Push {
        /// Image name (e.g. user/repo:tag)
        image: String,
    },

    /// Build and push in one step
    Deploy {
        /// Path to the app directory containing Dockerfile
        path: String,
        /// Image name (e.g. user/repo:tag)
        image: String,
    },
}

fn run_command(cmd: &mut Command, desc: &str) {
    let status = cmd.status().unwrap_or_else(|_| {
        eprintln!("Failed to start {}", desc);
        std::process::exit(1);
    });

    if !status.success() {
        eprintln!("{} failed", desc);
        std::process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { path, image } => {
            if !Path::new(path).exists() {
                eprintln!("Error: Path '{}' not found", path);
                std::process::exit(1);
            }
            println!("Building image {} from {}", image, path);
            run_command(Command::new("docker").args(["build", "-t", image, path]), "docker build");
        }

        Commands::Push { image } => {
            println!("Pushing image {}", image);
            run_command(Command::new("docker").args(["push", image]), "docker push");
        }

        Commands::Deploy { path, image } => {
            if !Path::new(path).exists() {
                eprintln!("Error: Path '{}' not found", path);
                std::process::exit(1);
            }
            println!("Deploying image {} from {}", image, path);
            run_command(Command::new("docker").args(["build", "-t", image, path]), "docker build");
            run_command(Command::new("docker").args(["push", image]), "docker push");
        }
    }
}

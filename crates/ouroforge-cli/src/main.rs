use anyhow::Result;
use clap::{Parser, Subcommand};
use ouroforge_core::{create_run, Seed};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "ouroforge")]
#[command(about = "Ouroforge harness CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Seed {
        #[command(subcommand)]
        command: SeedCommand,
    },
    Run {
        seed_path: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum SeedCommand {
    Validate { seed_path: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Seed {
            command: SeedCommand::Validate { seed_path },
        } => {
            let seed = Seed::from_path(seed_path)?;
            println!("Seed valid: {}", seed.id);
        }
        Commands::Run { seed_path } => {
            let artifacts = create_run(seed_path, "runs")?;
            println!("Run created: {}", artifacts.run_dir.display());
        }
    }

    Ok(())
}

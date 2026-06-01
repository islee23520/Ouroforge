use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use ouroforge_core::{
    add_evidence_artifact, append_ledger_event, create_mutation_proposal, create_run, evaluate_run,
    list_evidence_artifacts, list_mutation_proposals, read_ledger_events, run_browser_smoke,
    run_browser_smoke_pool, run_scenarios, show_journal, update_journal, BrowserSmokeConfig,
    BrowserSmokePoolConfig, MutationProposalInput, ScenarioRunConfig, Seed, WorkerId,
};
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
    Ledger {
        #[command(subcommand)]
        command: LedgerCommand,
    },
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommand,
    },
    Browser {
        #[command(subcommand)]
        command: BrowserCommand,
    },
    Scenario {
        #[command(subcommand)]
        command: ScenarioCommand,
    },
    Evaluate {
        run_dir: PathBuf,
    },
    Journal {
        #[command(subcommand)]
        command: JournalCommand,
    },
    Mutation {
        #[command(subcommand)]
        command: MutationCommand,
    },
}

#[derive(Debug, Subcommand)]
enum SeedCommand {
    Validate { seed_path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum LedgerCommand {
    Append {
        run_dir: PathBuf,
        #[arg(long)]
        kind: String,
        #[arg(long)]
        actor: String,
        #[arg(long, value_name = "JSON")]
        json: String,
    },
    List {
        run_dir: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum BrowserCommand {
    Smoke {
        run_dir: PathBuf,
        #[arg(long)]
        url: String,
        #[arg(long, default_value = "http://127.0.0.1:9222")]
        cdp: String,
        #[arg(long, default_value = "worker-1")]
        worker_id: String,
        #[arg(long, default_value_t = 1)]
        workers: usize,
    },
}

#[derive(Debug, Subcommand)]
enum MutationCommand {
    Create {
        run_dir: PathBuf,
        #[arg(long)]
        reason: String,
        #[arg(long)]
        evidence: String,
        #[arg(long)]
        target: String,
        #[arg(long)]
        path: String,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
    },
    List {
        run_dir: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum JournalCommand {
    Update { run_dir: PathBuf },
    Show { run_dir: PathBuf },
}

#[derive(Debug, Subcommand)]
enum ScenarioCommand {
    Run {
        run_dir: PathBuf,
        #[arg(long)]
        url: String,
        #[arg(long, default_value = "http://127.0.0.1:9222")]
        cdp: String,
    },
}

#[derive(Debug, Subcommand)]
enum EvidenceCommand {
    Add {
        run_dir: PathBuf,
        #[arg(long)]
        id: String,
        #[arg(long)]
        kind: String,
        #[arg(long, value_name = "PATH")]
        path: String,
        #[arg(long, value_name = "JSON", default_value = "{}")]
        json: String,
    },
    List {
        run_dir: PathBuf,
    },
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
        Commands::Ledger {
            command:
                LedgerCommand::Append {
                    run_dir,
                    kind,
                    actor,
                    json,
                },
        } => {
            let payload = parse_json_arg(&json)?;
            let event = append_ledger_event(run_dir, &kind, &actor, payload)?;
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
        Commands::Ledger {
            command: LedgerCommand::List { run_dir },
        } => {
            let events = read_ledger_events(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&events)?);
        }
        Commands::Evidence {
            command:
                EvidenceCommand::Add {
                    run_dir,
                    id,
                    kind,
                    path,
                    json,
                },
        } => {
            let metadata = parse_json_arg(&json)?;
            let artifact = add_evidence_artifact(run_dir, &id, &kind, &path, metadata)?;
            println!("{}", serde_json::to_string_pretty(&artifact)?);
        }
        Commands::Evidence {
            command: EvidenceCommand::List { run_dir },
        } => {
            let artifacts = list_evidence_artifacts(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&artifacts)?);
        }
        Commands::Browser {
            command:
                BrowserCommand::Smoke {
                    run_dir,
                    url,
                    cdp,
                    worker_id,
                    workers,
                },
        } => {
            let mut config = BrowserSmokeConfig::new(run_dir, url)?;
            config.debugging_http_url = cdp;
            config.worker_id = WorkerId::new(worker_id)?;
            if workers == 1 {
                let result = run_browser_smoke(&config)?;
                println!(
                    "Browser smoke captured: {}",
                    result.screenshot_path.display()
                );
            } else {
                let pool_config = BrowserSmokePoolConfig::new(config, workers)?;
                let result = run_browser_smoke_pool(&pool_config);
                println!("{}", serde_json::to_string_pretty(&result)?);
                if result.has_failures() {
                    return Err(anyhow!(
                        "browser smoke failed for {} of {} worker(s)",
                        result.failed,
                        result.workers
                    ));
                }
            }
        }
        Commands::Scenario {
            command: ScenarioCommand::Run { run_dir, url, cdp },
        } => {
            let mut config = ScenarioRunConfig::new(run_dir, url)?;
            config.debugging_http_url = cdp;
            let summary = run_scenarios(&config)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
            if summary.has_failures() {
                return Err(anyhow!(
                    "scenario run failed for {} of {} scenario(s)",
                    summary.failed,
                    summary.scenarios
                ));
            }
        }
        Commands::Evaluate { run_dir } => {
            let verdict = evaluate_run(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&verdict)?);
        }
        Commands::Journal {
            command: JournalCommand::Update { run_dir },
        } => {
            let journal = update_journal(run_dir)?;
            print!("{journal}");
        }
        Commands::Journal {
            command: JournalCommand::Show { run_dir },
        } => {
            let journal = show_journal(run_dir)?;
            print!("{journal}");
        }
        Commands::Mutation {
            command:
                MutationCommand::Create {
                    run_dir,
                    reason,
                    evidence,
                    target,
                    path,
                    from,
                    to,
                },
        } => {
            let proposal = create_mutation_proposal(
                run_dir,
                MutationProposalInput {
                    reason,
                    evidence_id: evidence,
                    target,
                    path,
                    from,
                    to,
                },
            )?;
            println!("{}", serde_json::to_string_pretty(&proposal)?);
        }
        Commands::Mutation {
            command: MutationCommand::List { run_dir },
        } => {
            let proposals = list_mutation_proposals(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&proposals)?);
        }
    }

    Ok(())
}

fn parse_json_arg(input: &str) -> Result<serde_json::Value> {
    serde_json::from_str(input).with_context(|| format!("failed to parse JSON argument: {input}"))
}

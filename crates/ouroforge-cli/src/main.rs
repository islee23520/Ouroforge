use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use ouroforge_core::{
    add_evidence_artifact, append_ledger_event, create_mutation_proposal, create_run, evaluate_run,
    evolve_run, list_evidence_artifacts, list_mutation_proposals, read_cdp_targets,
    read_ledger_events, run_browser_smoke, run_browser_smoke_pool, run_scenarios, show_journal,
    update_journal, BrowserSmokeConfig, BrowserSmokePoolConfig, MutationProposalInput,
    ScenarioRunConfig, Seed, WorkerId,
};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
        #[arg(long, default_value_t = 1)]
        workers: usize,
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
    Evolve {
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
        Commands::Run { seed_path, workers } => {
            let artifacts = create_run(seed_path, "runs")?;
            println!("Run created: {}", artifacts.run_dir.display());
            if workers > 1 {
                let summary = run_private_mvp(&artifacts.run_dir, workers)?;
                println!("{}", serde_json::to_string_pretty(&summary)?);
            }
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
        Commands::Evolve { run_dir } => {
            let summary = evolve_run(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
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

fn run_private_mvp(run_dir: &Path, workers: usize) -> Result<serde_json::Value> {
    let runtime_dir = PathBuf::from("examples/game-runtime");
    if !runtime_dir.join("index.html").is_file() {
        return Err(anyhow!(
            "game runtime page is missing: {}",
            runtime_dir.display()
        ));
    }
    let http_port = reserve_loopback_port()?;
    let cdp_port = reserve_loopback_port()?;
    let profile_dir = std::env::temp_dir().join(format!(
        "ouroforge-mvp-chrome-{}-{}",
        std::process::id(),
        monotonic_millis()?
    ));

    let mut server = Command::new("python3")
        .args([
            "-m",
            "http.server",
            &http_port.to_string(),
            "--bind",
            "127.0.0.1",
            "--directory",
            runtime_dir.to_str().unwrap_or("examples/game-runtime"),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("failed to start local runtime server")?;
    let chrome_path = find_chrome()?;
    let mut chrome = Command::new(&chrome_path)
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--remote-debugging-address=127.0.0.1")
        .arg(format!("--remote-debugging-port={cdp_port}"))
        .arg(format!("--user-data-dir={}", profile_dir.display()))
        .arg("about:blank")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| format!("failed to start Chrome at {}", chrome_path.display()))?;

    let result = (|| {
        let runtime_url = format!("http://127.0.0.1:{http_port}/");
        let cdp_url = format!("http://127.0.0.1:{cdp_port}");
        wait_for_cdp(&cdp_url)?;

        let mut smoke_config = BrowserSmokeConfig::new(run_dir, runtime_url.clone())?;
        smoke_config.debugging_http_url = cdp_url.clone();
        let smoke = run_browser_smoke_pool(&BrowserSmokePoolConfig::new(smoke_config, workers)?);
        if smoke.has_failures() {
            return Err(anyhow!(
                "private MVP browser smoke failed for {} of {} worker(s)",
                smoke.failed,
                smoke.workers
            ));
        }

        let mut scenario_config = ScenarioRunConfig::new(run_dir, runtime_url)?;
        scenario_config.debugging_http_url = cdp_url;
        let scenarios = run_scenarios(&scenario_config)?;
        let verdict = evaluate_run(run_dir)?;
        let evolve = if verdict.status == "failed" {
            Some(evolve_run(run_dir)?)
        } else {
            let _journal = update_journal(run_dir)?;
            None
        };
        Ok(json_mvp_summary(
            workers,
            &smoke,
            &scenarios,
            &verdict,
            evolve.as_ref(),
        ))
    })();

    terminate_child(&mut chrome);
    terminate_child(&mut server);
    let _ = std::fs::remove_dir_all(profile_dir);
    result
}

fn json_mvp_summary(
    workers: usize,
    smoke: &ouroforge_core::BrowserSmokePoolResult,
    scenarios: &ouroforge_core::ScenarioRunSummary,
    verdict: &ouroforge_core::EvaluationVerdict,
    evolve: Option<&ouroforge_core::EvolveSummary>,
) -> serde_json::Value {
    serde_json::json!({
        "mvp": "private-local",
        "status": verdict.status,
        "workers": workers,
        "browser_smoke": smoke,
        "scenarios": scenarios,
        "verdict": verdict,
        "journal_updated": true,
        "evolve": evolve
    })
}

fn reserve_loopback_port() -> Result<u16> {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").context("failed to reserve loopback port")?;
    Ok(listener.local_addr()?.port())
}

fn wait_for_cdp(cdp_url: &str) -> Result<()> {
    for _ in 0..100 {
        if read_cdp_targets(cdp_url).is_ok() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err(anyhow!(
        "Chrome CDP endpoint did not become ready: {cdp_url}"
    ))
}

fn find_chrome() -> Result<PathBuf> {
    if let Ok(path) = std::env::var("OUROFORGE_CHROME") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }
    }
    for candidate in [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
    ] {
        let path = PathBuf::from(candidate);
        if path.is_file() {
            return Ok(path);
        }
    }
    Err(anyhow!("Chrome executable not found; set OUROFORGE_CHROME"))
}

fn terminate_child(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn monotonic_millis() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before UNIX_EPOCH")?
        .as_millis())
}

fn parse_json_arg(input: &str) -> Result<serde_json::Value> {
    serde_json::from_str(input).with_context(|| format!("failed to parse JSON argument: {input}"))
}

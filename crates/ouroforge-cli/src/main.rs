use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use ouroforge_core::{
    add_evidence_artifact, append_ledger_event,
    append_mutation_review_decision_for_proposal_from_path, apply_patch_sandbox_from_path,
    apply_scene_only_mutation_operation, bind_run_command_context, bind_run_project_metadata,
    bind_run_transaction_provenance, build_authoring_loop_dry_run_summary_from_path,
    build_authoring_loop_resume_preflight_from_path, build_authoring_loop_status_from_path,
    build_regression_promotion_draft_from_run, build_regression_run_matrix,
    build_studio_loop_cockpit_read_model, create_minimal_2d_project_scaffold,
    create_mutation_proposal, create_run, edit_scene, evaluate_run, evolve_run,
    execute_authoring_loop_step_from_path, hash_project_manifest_file, hash_scene_document,
    list_agent_handoff_contracts, list_authoring_loop_evidence_bundles, list_dashboard_runs,
    list_evidence_artifacts, list_mutation_proposals, orchestrate_evolve_rerun_from_path,
    preview_scene_edit_transaction, project_run_metadata_from_manifest,
    promote_regression_draft_to_scenario_pack, read_cdp_targets, read_dashboard_run,
    read_ledger_events, read_scene, reject_generated_artifact_source_collision,
    reject_transaction_output_target_collision, run_browser_smoke, run_browser_smoke_pool,
    run_command_context_for_run, run_evolve_demo_lifecycle_from_path, run_scenarios, show_journal,
    update_journal, validate_scene_reload, write_agent_handoff_contract_from_path,
    write_regression_promotion_draft, write_run_comparison_artifact,
    write_scene_edit_transaction_artifact, BrowserSmokeConfig, BrowserSmokePoolConfig,
    MutationProposalInput, MutationReviewReviewerType, MutationReviewState, ProjectManifest,
    ProjectSceneMutationContext, ScenarioRunConfig, SceneEdit, SceneOnlyMutationOperation, Seed,
    WorkerId,
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
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    Run {
        seed_path: PathBuf,
        #[arg(long, default_value_t = 1)]
        workers: usize,
        #[arg(long, value_name = "PATH")]
        transaction: Option<PathBuf>,
        #[arg(long, value_name = "PATH")]
        project: Option<PathBuf>,
        #[arg(long, value_name = "ID")]
        scenario_pack: Option<String>,
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
    Compare {
        before_run_dir: PathBuf,
        after_run_dir: PathBuf,
        #[arg(long, default_value = "runs/comparisons")]
        output_dir: PathBuf,
    },
    Journal {
        #[command(subcommand)]
        command: JournalCommand,
    },
    Mutation {
        #[command(subcommand)]
        command: MutationCommand,
    },
    Dashboard {
        #[command(subcommand)]
        command: DashboardCommand,
    },
    Scene {
        #[command(subcommand)]
        command: SceneCommand,
    },
    Loop {
        #[command(subcommand)]
        command: LoopCommand,
    },
}

#[derive(Debug, Subcommand)]
enum LoopCommand {
    DryRun {
        plan_path: PathBuf,
    },
    Status {
        plan_path: PathBuf,
    },
    Resume {
        plan_path: PathBuf,
        #[arg(long)]
        step: String,
    },
    Step {
        plan_path: PathBuf,
        #[arg(long)]
        step: String,
    },
    Handoff {
        plan_path: PathBuf,
        #[arg(long, value_name = "PATH")]
        output: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum SeedCommand {
    Validate { seed_path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum ProjectCommand {
    Validate {
        project_root_or_manifest: PathBuf,
    },
    Init {
        destination: PathBuf,
        #[arg(long, default_value = "minimal-2d")]
        template: String,
    },
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
enum SceneCommand {
    Validate {
        scene_path: PathBuf,
    },
    Show {
        scene_path: PathBuf,
    },
    ReloadValidate {
        scene_path: PathBuf,
    },
    Edit {
        scene_path: PathBuf,
        #[arg(long)]
        entity: String,
        #[arg(
            long,
            help = "Supported paths: sprite.color, components.transform.x, components.transform.y, components.velocity.x, components.velocity.y, components.size.width, components.size.height, components.controllable"
        )]
        path: String,
        #[arg(long, value_name = "JSON")]
        value: String,
        #[arg(long, value_name = "PATH")]
        transaction_output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum DashboardCommand {
    Export {
        #[arg(long, default_value = "runs")]
        runs_root: PathBuf,
        #[arg(
            long,
            default_value = "examples/evidence-dashboard/dashboard-data.json"
        )]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum CliMutationReviewDecision {
    Accepted,
    Rejected,
    Deferred,
}

impl From<CliMutationReviewDecision> for MutationReviewState {
    fn from(value: CliMutationReviewDecision) -> Self {
        match value {
            CliMutationReviewDecision::Accepted => MutationReviewState::Accepted,
            CliMutationReviewDecision::Rejected => MutationReviewState::Rejected,
            CliMutationReviewDecision::Deferred => MutationReviewState::Deferred,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum CliMutationReviewReviewerType {
    Human,
    Agent,
    System,
}

impl From<CliMutationReviewReviewerType> for MutationReviewReviewerType {
    fn from(value: CliMutationReviewReviewerType) -> Self {
        match value {
            CliMutationReviewReviewerType::Human => MutationReviewReviewerType::Human,
            CliMutationReviewReviewerType::Agent => MutationReviewReviewerType::Agent,
            CliMutationReviewReviewerType::System => MutationReviewReviewerType::System,
        }
    }
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
    ApplyScene {
        run_dir: PathBuf,
        #[arg(long, value_name = "PATH")]
        project: Option<PathBuf>,
        #[arg(long, value_name = "PATH")]
        operation: PathBuf,
        #[arg(long, value_name = "ID")]
        decision: Option<String>,
        #[arg(long, value_name = "PATH")]
        transaction_output: PathBuf,
    },
    Review {
        run_or_draft_path: PathBuf,
        #[arg(long)]
        proposal: Option<String>,
        #[arg(long, value_enum)]
        decision: Option<CliMutationReviewDecision>,
        #[arg(long)]
        accept: bool,
        #[arg(long)]
        reject: bool,
        #[arg(long)]
        defer: bool,
        #[arg(long)]
        reason: String,
        #[arg(long = "evidence")]
        evidence_refs: Vec<String>,
        #[arg(long, default_value = "mutation-review-cli")]
        reviewer: String,
        #[arg(long, value_enum, default_value = "human")]
        reviewer_type: CliMutationReviewReviewerType,
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
    PromoteDraft {
        run_dir: PathBuf,
        #[arg(long, value_name = "PATH")]
        project: PathBuf,
        #[arg(long, value_name = "ID")]
        scenario: String,
        #[arg(long, value_name = "PATH")]
        output: PathBuf,
    },
    Promote {
        draft_path: PathBuf,
        #[arg(long, value_name = "PATH")]
        project: PathBuf,
        #[arg(long = "scenario-pack", value_name = "ID")]
        scenario_pack: String,
        #[arg(long)]
        dry_run: bool,
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
    if try_handle_evolve_sandbox_command()? {
        return Ok(());
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Loop {
            command: LoopCommand::DryRun { plan_path },
        } => {
            let summary = build_authoring_loop_dry_run_summary_from_path(&plan_path)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Loop {
            command: LoopCommand::Status { plan_path },
        } => {
            let summary = build_authoring_loop_status_from_path(&plan_path)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Loop {
            command: LoopCommand::Resume { plan_path, step },
        } => {
            let summary = build_authoring_loop_resume_preflight_from_path(&plan_path, &step)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Loop {
            command: LoopCommand::Step { plan_path, step },
        } => {
            let summary = execute_authoring_loop_step_from_path(&plan_path, &step)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Loop {
            command: LoopCommand::Handoff { plan_path, output },
        } => {
            reject_generated_artifact_source_collision(&output, "agent handoff")?;
            let handoff = write_agent_handoff_contract_from_path(&plan_path, &output)?;
            println!("Agent handoff written: {}", output.display());
            println!("{}", serde_json::to_string_pretty(&handoff)?);
        }
        Commands::Seed {
            command: SeedCommand::Validate { seed_path },
        } => {
            let seed = Seed::from_path(seed_path)?;
            println!("Seed valid: {}", seed.id);
        }
        Commands::Project {
            command:
                ProjectCommand::Validate {
                    project_root_or_manifest,
                },
        } => {
            let manifest_path = resolve_project_manifest_path(&project_root_or_manifest);
            let manifest = ProjectManifest::from_path(&manifest_path)?;
            let base_dir = match manifest_path.parent() {
                Some(parent) if !parent.as_os_str().is_empty() => parent,
                _ => Path::new("."),
            };
            let report = manifest.validate_references(base_dir)?;
            println!("Project manifest valid: {}", report.project_id);
            println!("Manifest: {}", manifest_path.display());
            println!("Source refs: {}", report.source_refs);
            println!("Asset roots: {}", report.asset_roots);
            println!("Scenario packs: {}", report.scenario_packs);
            println!("Runs root: {}", report.runs_root);
            println!("Generated roots: {}", report.generated_roots.join(","));
        }
        Commands::Project {
            command:
                ProjectCommand::Init {
                    destination,
                    template,
                },
        } => {
            if template != "minimal-2d" {
                return Err(anyhow!(
                    "unsupported project template {template}; supported templates: minimal-2d"
                ));
            }
            let report = create_minimal_2d_project_scaffold(&destination)?;
            println!("Project scaffold created: {}", destination.display());
            println!("Template: minimal-2d");
            println!("Project manifest valid: {}", report.project_id);
            println!("Source refs: {}", report.source_refs);
            println!("Runs root: {}", report.runs_root);
        }
        Commands::Run {
            seed_path,
            workers,
            transaction,
            project,
            scenario_pack,
        } => {
            if workers == 0 {
                return Err(anyhow!("--workers must be at least 1"));
            }
            let project_metadata = if let Some(project_path) = project {
                let manifest_path = resolve_project_manifest_path(&project_path);
                Some(project_run_metadata_from_manifest(
                    &manifest_path,
                    &seed_path,
                    scenario_pack.as_deref(),
                )?)
            } else {
                if scenario_pack.is_some() {
                    return Err(anyhow!("--scenario-pack requires --project"));
                }
                None
            };
            let transaction_path_for_context = transaction.clone();
            let artifacts = create_run(&seed_path, "runs")?;
            let mut transaction_id = None;
            if let Some(transaction_path) = transaction {
                // A stale/malformed transaction (or scene-hash mismatch) must not
                // leave an orphaned run behind for `ledger list`/dashboard exports
                // to pick up, so remove the freshly created run on bind failure.
                match bind_run_transaction_provenance(&artifacts.run_dir, transaction_path) {
                    Ok(provenance) => {
                        transaction_id = Some(provenance.transaction_id.clone());
                        println!("Run transaction bound: {}", provenance.transaction_id);
                    }
                    Err(error) => {
                        let _ = std::fs::remove_dir_all(&artifacts.run_dir);
                        return Err(error);
                    }
                }
            }
            let mut bound_project_metadata = None;
            if let Some(mut metadata) = project_metadata {
                metadata.transaction_id = transaction_id.clone();
                let project_id = metadata.id.clone();
                let metadata = bind_run_project_metadata(&artifacts.run_dir, metadata)?;
                bound_project_metadata = Some(metadata);
                println!("Run project bound: {project_id}");
            }
            let command_context = run_command_context_for_run(
                &seed_path,
                Path::new("runs"),
                workers,
                bound_project_metadata.as_ref(),
                transaction_path_for_context.as_deref(),
            );
            bind_run_command_context(&artifacts.run_dir, command_context)?;
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
        Commands::Scenario {
            command:
                ScenarioCommand::PromoteDraft {
                    run_dir,
                    project,
                    scenario,
                    output,
                },
        } => {
            reject_generated_artifact_source_collision(&output, "regression promotion draft")?;
            let manifest_path = resolve_project_manifest_path(&project);
            let draft =
                build_regression_promotion_draft_from_run(&run_dir, &manifest_path, &scenario)?;
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "failed to create regression promotion draft output directory {}",
                        parent.display()
                    )
                })?;
            }
            write_regression_promotion_draft(&output, &draft)?;
            println!("Regression promotion draft: {}", output.display());
            println!("Source run: {}", draft.source_run.run_id);
            println!("Scenario: {}", draft.source_evidence.scenario_id);
            println!(
                "Scenario result: {}",
                draft.source_evidence.scenario_result_path
            );
            if let Some(replay) = &draft.source_evidence.replay_artifact_path {
                println!("Replay artifact: {replay}");
            }
            println!("Target scenario pack: {}", draft.target.scenario_pack_id);
        }
        Commands::Scenario {
            command:
                ScenarioCommand::Promote {
                    draft_path,
                    project,
                    scenario_pack,
                    dry_run,
                },
        } => {
            let manifest_path = resolve_project_manifest_path(&project);
            let result = promote_regression_draft_to_scenario_pack(
                &draft_path,
                &manifest_path,
                &scenario_pack,
                dry_run,
            )?;
            if dry_run {
                println!("Regression promotion dry-run: {}", result.scenario_id);
            } else {
                println!("Regression promoted: {}", result.scenario_id);
            }
            println!("Target scenario pack: {}", result.target.scenario_pack_id);
            println!("Before hash: {}", result.before_hash.value);
            println!("After hash: {}", result.after_hash.value);
            if let Some(record_path) = &result.record_path {
                println!("Promotion record: {record_path}");
            }
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::Evaluate { run_dir } => {
            let verdict = evaluate_run(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&verdict)?);
        }
        Commands::Evolve { run_dir } => {
            let summary = evolve_run(run_dir)?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::Compare {
            before_run_dir,
            after_run_dir,
            output_dir,
        } => {
            reject_generated_artifact_source_collision(&output_dir, "comparison")?;
            let path = write_run_comparison_artifact(before_run_dir, after_run_dir, output_dir)?;
            let comparison = std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read comparison {}", path.display()))?;
            println!("Comparison written: {}", path.display());
            print_semantic_compare_summary(&comparison)?;
            println!("{comparison}");
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
        Commands::Mutation {
            command:
                MutationCommand::ApplyScene {
                    run_dir,
                    project,
                    operation,
                    decision,
                    transaction_output,
                },
        } => {
            let input = std::fs::read_to_string(&operation)
                .with_context(|| format!("failed to read operation {}", operation.display()))?;
            let mut operation_model: SceneOnlyMutationOperation = serde_json::from_str(&input)
                .with_context(|| format!("failed to parse operation {}", operation.display()))?;
            if let Some(project_path) = project {
                operation_model = bind_project_context_to_scene_mutation_operation(
                    operation_model,
                    &project_path,
                )?;
            }
            if let Some(decision_id) = decision {
                operation_model.review_decision_id = Some(decision_id);
            }
            let transaction = apply_scene_only_mutation_operation(
                &run_dir,
                &operation_model,
                &transaction_output,
            )?;
            println!("Scene-only mutation applied: {}", transaction.id);
            println!("Transaction artifact: {}", transaction_output.display());
            if let Some(decision_id) = &operation_model.review_decision_id {
                println!("Review decision: {decision_id}");
            }
            println!("Before scene hash: {}", transaction.before_scene_hash.value);
            if let Some(after_hash) = &transaction.after_scene_hash {
                println!("After scene hash: {}", after_hash.value);
            }
            println!(
                "Next QA command: cargo run -p ouroforge-cli -- run <seed> --transaction {}",
                transaction_output.display()
            );
            println!("{}", serde_json::to_string_pretty(&transaction)?);
        }
        Commands::Mutation {
            command:
                MutationCommand::Review {
                    run_or_draft_path,
                    proposal,
                    decision,
                    accept,
                    reject,
                    defer,
                    reason,
                    evidence_refs,
                    reviewer,
                    reviewer_type,
                },
        } => {
            let legacy_flags = [accept, reject, defer]
                .iter()
                .filter(|selected| **selected)
                .count();
            let state = match (decision, accept, reject, defer) {
                (Some(decision), false, false, false) => MutationReviewState::from(decision),
                (None, true, false, false) => MutationReviewState::Accepted,
                (None, false, true, false) => MutationReviewState::Rejected,
                (None, false, false, true) => MutationReviewState::Deferred,
                _ => {
                    let _ = legacy_flags;
                    return Err(anyhow!(
                        "mutation review requires exactly one of --decision, --accept, --reject, or --defer"
                    ));
                }
            };
            let decision = append_mutation_review_decision_for_proposal_from_path(
                run_or_draft_path,
                proposal,
                state,
                reason,
                evidence_refs,
                reviewer,
                Some(MutationReviewReviewerType::from(reviewer_type)),
            )?;
            println!("{}", serde_json::to_string_pretty(&decision)?);
        }
        Commands::Dashboard {
            command: DashboardCommand::Export { runs_root, output },
        } => {
            reject_generated_artifact_source_collision(&output, "dashboard export")?;
            let summaries = list_dashboard_runs(&runs_root)?;
            let runs = summaries
                .iter()
                .map(|summary| read_dashboard_run(&summary.run_dir))
                .collect::<Result<Vec<_>>>()?;
            let regression_matrix = build_regression_run_matrix(&runs_root)?;
            let loop_evidence_bundles = list_authoring_loop_evidence_bundles(&runs_root)?;
            let agent_handoffs = list_agent_handoff_contracts(&runs_root)?;
            let loop_cockpit =
                build_studio_loop_cockpit_read_model(&loop_evidence_bundles, &agent_handoffs);
            let payload = serde_json::json!({
                "schema": "ouroforge-dashboard-v1",
                "runs_root": runs_root,
                "runs": runs,
                "regression_matrix": regression_matrix,
                "loop_evidence_bundles": loop_evidence_bundles,
                "agent_handoffs": agent_handoffs,
                "loop_cockpit": loop_cockpit
            });
            if let Some(parent) = output.parent() {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "failed to create dashboard output directory {}",
                        parent.display()
                    )
                })?;
            }
            std::fs::write(&output, serde_json::to_string_pretty(&payload)?)
                .with_context(|| format!("failed to write dashboard data {}", output.display()))?;
            println!("Dashboard data exported: {}", output.display());
        }
        Commands::Scene {
            command: SceneCommand::Validate { scene_path },
        } => {
            let scene = read_scene(&scene_path)?;
            println!("Scene valid: {}", scene.id);
            if let Some(manifest) = scene.asset_manifest {
                // read_scene only runs structural manifest validation; verify the
                // referenced files actually exist relative to the scene directory
                // before reporting success, matching the runtime's load contract.
                let base_dir = match scene_path.parent() {
                    Some(parent) if !parent.as_os_str().is_empty() => parent,
                    _ => Path::new("."),
                };
                manifest.validate_files(base_dir).with_context(|| {
                    format!(
                        "scene asset manifest {} references invalid files",
                        manifest.id
                    )
                })?;
                println!(
                    "Asset manifest valid: {} ({} asset(s))",
                    manifest.id,
                    manifest.assets.len()
                );
            }
        }
        Commands::Scene {
            command: SceneCommand::ReloadValidate { scene_path },
        } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&validate_scene_reload(scene_path)?)?
            );
        }
        Commands::Scene {
            command: SceneCommand::Show { scene_path },
        } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&read_scene(scene_path)?)?
            );
        }
        Commands::Scene {
            command:
                SceneCommand::Edit {
                    scene_path,
                    entity,
                    path,
                    value,
                    transaction_output,
                },
        } => {
            let value = parse_json_arg(&value)?;
            let edit = SceneEdit {
                entity_id: entity,
                path,
                value,
            };
            if let Some(output_path) = transaction_output {
                reject_transaction_output_target_collision(&output_path, &scene_path)?;
                let transaction = preview_scene_edit_transaction(&scene_path, edit.clone())?;
                write_scene_edit_transaction_artifact(&output_path, &transaction)?;
                if transaction.validation_result.status != "passed" {
                    println!("{}", serde_json::to_string_pretty(&transaction)?);
                    return Err(anyhow!(
                        "scene edit transaction failed validation; artifact written to {}",
                        output_path.display()
                    ));
                }
                let _scene = edit_scene(&scene_path, edit)?;
                println!("{}", serde_json::to_string_pretty(&transaction)?);
            } else {
                let scene = edit_scene(&scene_path, edit)?;
                println!("{}", serde_json::to_string_pretty(&scene)?);
            }
        }
    }

    Ok(())
}

fn resolve_project_manifest_path(project_root_or_manifest: &Path) -> PathBuf {
    if project_root_or_manifest.is_dir() {
        project_root_or_manifest.join("ouroforge.project.json")
    } else {
        project_root_or_manifest.to_path_buf()
    }
}

fn bind_project_context_to_scene_mutation_operation(
    mut operation: SceneOnlyMutationOperation,
    project_root_or_manifest: &Path,
) -> Result<SceneOnlyMutationOperation> {
    let manifest_path = resolve_project_manifest_path(project_root_or_manifest);
    let manifest = ProjectManifest::from_path(&manifest_path)?;
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let target_scene_path = Path::new(&operation.target_scene_path);
    let target_scene_canonical = target_scene_path.canonicalize().with_context(|| {
        format!(
            "failed to resolve scene-only mutation targetScenePath {}",
            target_scene_path.display()
        )
    })?;
    let mut matched_scene_path = None;
    for scene_ref in &manifest.scenes {
        let candidate = project_root.join(&scene_ref.path);
        let candidate_canonical = candidate.canonicalize().with_context(|| {
            format!(
                "failed to resolve project scene {} from manifest {}",
                scene_ref.path,
                manifest_path.display()
            )
        })?;
        if candidate_canonical == target_scene_canonical {
            matched_scene_path = Some(scene_ref.path.clone());
            break;
        }
    }
    let scene_path = matched_scene_path.ok_or_else(|| {
        anyhow!(
            "scene-only mutation targetScenePath {} is not declared in project manifest scenes",
            operation.target_scene_path
        )
    })?;
    let scene_hash = hash_scene_document(&read_scene(&operation.target_scene_path)?)?;
    let context = ProjectSceneMutationContext {
        project_id: manifest.project.id.clone(),
        manifest_path: manifest_path.to_string_lossy().to_string(),
        manifest_hash: hash_project_manifest_file(&manifest_path)?,
        scene_path,
        scene_hash,
    };
    if let Some(existing) = &operation.project {
        if existing != &context {
            return Err(anyhow!(
                "scene-only mutation --project context does not match operation project context"
            ));
        }
    } else {
        operation.project = Some(context);
    }
    Ok(operation)
}

fn try_handle_evolve_sandbox_command() -> Result<bool> {
    let args = std::env::args_os().collect::<Vec<_>>();
    if args.len() == 4 && args[1] == "evolve" && args[2] == "sandbox" {
        let result = apply_patch_sandbox_from_path(PathBuf::from(&args[3]))?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(true);
    }
    if args.len() == 4 && args[1] == "evolve" && args[2] == "compare" {
        let result = orchestrate_evolve_rerun_from_path(PathBuf::from(&args[3]))?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(true);
    }
    if args.len() == 4 && args[1] == "evolve" && args[2] == "demo" {
        let result = run_evolve_demo_lifecycle_from_path(PathBuf::from(&args[3]))?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(true);
    }
    Ok(false)
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

    // Resolve Chrome before starting the static server so a missing/invalid
    // browser fails closed without leaking an orphaned `python3 -m http.server`.
    let chrome_path = find_chrome()?;
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
    let mut chrome = match Command::new(&chrome_path)
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--remote-debugging-address=127.0.0.1")
        .arg(format!("--remote-debugging-port={cdp_port}"))
        .arg(format!("--user-data-dir={}", profile_dir.display()))
        .arg("about:blank")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            // Tear down the already-started static server before surfacing the error.
            terminate_child(&mut server);
            return Err(anyhow::Error::new(error).context(format!(
                "failed to start Chrome at {}",
                chrome_path.display()
            )));
        }
    };

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
    for _ in 0..300 {
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

fn print_semantic_compare_summary(comparison_json: &str) -> Result<()> {
    let comparison: serde_json::Value =
        serde_json::from_str(comparison_json).context("failed to parse comparison JSON")?;
    let reasons = comparison
        .pointer("/semantic/reasons")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    println!("Semantic reasons:");
    if reasons.is_empty() {
        println!("- none");
    } else {
        for reason in reasons {
            let kind = reason
                .get("kind")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown");
            let severity = reason
                .get("severity")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown");
            let summary = reason
                .get("summary")
                .and_then(|value| value.as_str())
                .unwrap_or("no summary");
            println!("- [{severity}] {kind}: {summary}");
        }
    }
    let warnings = comparison
        .pointer("/semantic/warnings")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    if !warnings.is_empty() {
        println!("Semantic warnings:");
        for warning in warnings {
            println!("- {}", warning.as_str().unwrap_or("unknown warning"));
        }
    }
    if let Some(project) = comparison.pointer("/semantic/project") {
        let relation = project
            .get("relation")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown");
        let changed = project
            .get("changed")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        println!("Project comparison:");
        println!("- relation: {relation}");
        println!("- changed: {changed}");
        if let Some(changes) = project.get("changes").and_then(|value| value.as_array()) {
            for change in changes {
                let kind = change
                    .get("kind")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown");
                let summary = change
                    .get("summary")
                    .and_then(|value| value.as_str())
                    .unwrap_or("no summary");
                let before = change
                    .get("before")
                    .and_then(|value| value.as_str())
                    .unwrap_or("none");
                let after = change
                    .get("after")
                    .and_then(|value| value.as_str())
                    .unwrap_or("none");
                println!("- [{kind}] {summary}: {before} -> {after}");
            }
        }
        if let Some(project_warnings) = project.get("warnings").and_then(|value| value.as_array()) {
            for warning in project_warnings {
                println!(
                    "- [warning] {}",
                    warning.as_str().unwrap_or("unknown project warning")
                );
            }
        }
    }
    Ok(())
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

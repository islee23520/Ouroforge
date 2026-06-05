use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use ouroforge_core::behavior_runtime::{
    BehaviorApplyTransactionArtifact, BehaviorApplyTransactionStatus, BehaviorDraftArtifact,
};
use ouroforge_core::internal_sprite_audit::{
    audit_internal_sprite_reference, InternalSpriteAuditProfile, InternalSpriteAuditReport,
};
use ouroforge_core::{
    add_evidence_artifact, append_ledger_event,
    append_mutation_review_decision_for_proposal_from_path, append_visual_edit_draft_application,
    apply_patch_sandbox_from_path, apply_scene_only_mutation_operation, bind_run_command_context,
    bind_run_project_metadata, bind_run_transaction_provenance,
    build_authoring_loop_dry_run_summary_from_path,
    build_authoring_loop_resume_preflight_from_path, build_authoring_loop_status_from_path,
    build_regression_promotion_draft_from_run, build_regression_run_matrix,
    build_studio_loop_cockpit_read_model, create_minimal_2d_project_scaffold,
    create_mutation_proposal, create_run, edit_scene, evaluate_run, evolve_run,
    execute_authoring_loop_step_from_path, hash_project_manifest_file, hash_scene_document,
    list_agent_handoff_contracts, list_agent_handoff_v2_contracts,
    list_authoring_loop_evidence_bundles, list_dashboard_runs, list_evidence_artifacts,
    list_mutation_proposals, orchestrate_evolve_rerun_from_path, preview_scene_edit_transaction,
    project_run_metadata_from_manifest, promote_regression_draft_to_scenario_pack,
    read_cdp_targets, read_dashboard_run, read_ledger_events, read_runtime_frame_budget,
    read_scene, reject_already_applied_visual_edit_draft_decision,
    reject_generated_artifact_source_collision, reject_transaction_output_target_collision,
    run_browser_smoke, run_browser_smoke_pool, run_command_context_for_run,
    run_evolve_demo_lifecycle_from_path, run_scenarios, show_journal,
    source_patch_preview_read_model, update_journal, validate_scene_reload,
    validate_source_patch_preview_artifact, validate_visual_edit_draft_review_preflight,
    write_agent_handoff_contract_from_path, write_regression_promotion_draft,
    write_run_comparison_artifact, write_scene_edit_transaction_artifact, BrowserSmokeConfig,
    BrowserSmokePoolConfig, MutationProposalInput, MutationReviewReviewerType, MutationReviewState,
    PatchDiffIntegrityLimits, ProjectAssetManifest, ProjectAssetType, ProjectManifest,
    ProjectSceneMutationContext, RuntimeFrameBudgetStatus, ScenarioRunConfig, SceneEdit,
    SceneOnlyMutationOperation, Seed, SourcePatchPreviewArtifact,
    VisualEditDraftApplyCommandContext, VisualEditDraftArtifact, VisualEditDraftTargetType,
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
    Asset {
        #[command(subcommand)]
        command: AssetCommand,
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
    RuntimeDebug {
        #[command(subcommand)]
        command: RuntimeDebugCommand,
    },
    Edit {
        #[command(subcommand)]
        command: EditCommand,
    },
    Loop {
        #[command(subcommand)]
        command: LoopCommand,
    },
    PatchPreview {
        #[command(subcommand)]
        command: PatchPreviewCommand,
    },
    Behavior {
        #[command(subcommand)]
        command: BehaviorCommand,
    },
    Plugin {
        #[command(subcommand)]
        command: PluginCommand,
    },
}

/// Read-only local plugin registry inspection (#752). No install, update, run,
/// enable, disable, delete, publish, or marketplace behavior is provided.
#[derive(Debug, Subcommand)]
enum PluginCommand {
    /// Discover and list local plugins with status and descriptors (JSON).
    List {
        #[arg(default_value = "plugins")]
        dir: PathBuf,
    },
    /// Validate local plugins; exits non-zero if any plugin or conflict fails.
    Validate {
        #[arg(default_value = "plugins")]
        dir: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum BehaviorCommand {
    Draft {
        #[command(subcommand)]
        command: BehaviorDraftCommand,
    },
    Apply {
        #[command(subcommand)]
        command: BehaviorApplyCommand,
    },
}

#[derive(Debug, Subcommand)]
enum BehaviorDraftCommand {
    Validate {
        draft_path: PathBuf,
        #[arg(long, value_name = "PATH")]
        project_root: Option<PathBuf>,
    },
    Preview {
        draft_path: PathBuf,
        #[arg(long, value_name = "PATH")]
        project_root: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum BehaviorApplyCommand {
    Transaction {
        #[command(subcommand)]
        command: BehaviorApplyTransactionCommand,
    },
}

#[derive(Debug, Subcommand)]
enum BehaviorApplyTransactionCommand {
    Validate { transaction_path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum PatchPreviewCommand {
    Validate {
        preview_path: PathBuf,
        #[arg(long, default_value_t = 32)]
        max_files: usize,
        #[arg(long, default_value_t = 5_000)]
        max_changed_lines: usize,
    },
    Show {
        preview_path: PathBuf,
        #[arg(long, default_value_t = 32)]
        max_files: usize,
        #[arg(long, default_value_t = 5_000)]
        max_changed_lines: usize,
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
enum AssetCommand {
    Validate {
        project_root_or_manifest: PathBuf,
    },
    AuditInternalSprites {
        reference_root: PathBuf,
        #[arg(long, default_value = "ro-vibe-v1")]
        profile: String,
        #[arg(long)]
        json: bool,
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
enum RuntimeDebugCommand {
    FrameBudget {
        #[command(subcommand)]
        command: RuntimeFrameBudgetCommand,
    },
}

#[derive(Debug, Subcommand)]
enum RuntimeFrameBudgetCommand {
    Validate { budget_path: PathBuf },
    Show { budget_path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum EditCommand {
    DraftPreview {
        draft_path: PathBuf,
        #[arg(long, value_name = "PATH")]
        project: PathBuf,
        #[arg(long, value_name = "PATH")]
        transaction_output: Option<PathBuf>,
    },
    DraftApply {
        draft_path: PathBuf,
        #[arg(long, value_name = "PATH")]
        project: PathBuf,
        #[arg(long, value_name = "PATH")]
        run_dir: PathBuf,
        #[arg(long, value_name = "ID")]
        proposal: String,
        #[arg(long, value_name = "ID")]
        decision: String,
        #[arg(long, value_name = "PATH")]
        transaction_output: PathBuf,
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
        Commands::Behavior {
            command:
                BehaviorCommand::Draft {
                    command:
                        BehaviorDraftCommand::Validate {
                            draft_path,
                            project_root,
                        },
                },
        } => {
            let draft = read_behavior_draft(&draft_path)?;
            let target_check = behavior_draft_target_check(&draft, project_root.as_deref())?;
            let valid = target_check.get("stale").and_then(|value| value.as_bool()) != Some(true);
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "status": if valid { "valid" } else { "stale" },
                    "draftId": draft.draft_id,
                    "validationStatus": draft.validation_status,
                    "target": draft.target,
                    "targetCheck": target_check,
                    "guardrail": "untrusted draft validation only; does not apply trusted files or execute scripts"
                }))?
            );
            if !valid {
                return Err(anyhow!("behavior draft target hash is stale"));
            }
        }
        Commands::Behavior {
            command:
                BehaviorCommand::Draft {
                    command:
                        BehaviorDraftCommand::Preview {
                            draft_path,
                            project_root,
                        },
                },
        } => {
            let draft = read_behavior_draft(&draft_path)?;
            let runtime = draft.proposed_behavior.runtime_state();
            let target_check = behavior_draft_target_check(&draft, project_root.as_deref())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "schemaVersion": "ouroforge.behavior-draft-preview.v1",
                    "draftId": draft.draft_id,
                    "target": draft.target,
                    "validationStatus": draft.validation_status,
                    "linkedEvidenceCount": draft.linked_evidence.len(),
                    "expectedScenarioImpactCount": draft.expected_scenario_impact.len(),
                    "behaviorCount": runtime.counts.behavior_count,
                    "diagnostics": runtime.diagnostics,
                    "targetCheck": target_check,
                    "guardrail": "read-only untrusted preview; does not apply trusted files, execute scripts, open command bridges, or grant browser writes"
                }))?
            );
        }
        Commands::Behavior {
            command:
                BehaviorCommand::Apply {
                    command:
                        BehaviorApplyCommand::Transaction {
                            command: BehaviorApplyTransactionCommand::Validate { transaction_path },
                        },
                },
        } => {
            let transaction = read_behavior_apply_transaction(&transaction_path)?;
            let review_accepted = transaction.review_decision.status
                == ouroforge_core::behavior_runtime::BehaviorApplyReviewDecisionStatus::Accepted;
            let target_hash_fresh = transaction.target_hashes.expected_before_hash
                == transaction.target_hashes.observed_before_hash;
            let self_approval = transaction.review_decision.reviewer_id
                == transaction.review_decision.draft_author_id;
            let trusted_apply_ready = transaction.status
                == BehaviorApplyTransactionStatus::ReadyForTrustedApply
                && review_accepted
                && target_hash_fresh
                && !self_approval;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "schemaVersion": "ouroforge.behavior-apply-transaction-validation.v1",
                    "status": if trusted_apply_ready { "readyForTrustedApply" } else { "blocked" },
                    "trustedApplyReady": trusted_apply_ready,
                    "transactionStatus": transaction.status,
                    "transactionId": transaction.transaction_id,
                    "draftId": transaction.draft_id,
                    "reviewDecision": transaction.review_decision,
                    "reviewAccepted": review_accepted,
                    "selfApproval": self_approval,
                    "target": transaction.target,
                    "targetHashes": transaction.target_hashes,
                    "targetHashFresh": target_hash_fresh,
                    "transactionOutputRef": transaction.transaction_output_ref,
                    "rollbackMetadata": transaction.rollback_metadata,
                    "rerunCommand": transaction.rerun_command,
                    "evidenceRefCount": transaction.evidence_refs.len(),
                    "blockedReasons": transaction.blocked_reasons,
                    "guardrail": "read-only local validation only; does not apply trusted files, execute scripts, auto-apply, self-approve, open command bridges, or grant browser writes"
                }))?
            );
            if !trusted_apply_ready {
                return Err(anyhow!(
                    "behavior apply transaction is not ready for trusted apply"
                ));
            }
        }
        Commands::PatchPreview {
            command:
                PatchPreviewCommand::Validate {
                    preview_path,
                    max_files,
                    max_changed_lines,
                },
        } => {
            let artifact: SourcePatchPreviewArtifact = serde_json::from_str(
                &std::fs::read_to_string(&preview_path).with_context(|| {
                    format!("failed to read patch preview {}", preview_path.display())
                })?,
            )
            .with_context(|| format!("failed to parse patch preview {}", preview_path.display()))?;
            let validation = validate_source_patch_preview_artifact(
                &artifact,
                PatchDiffIntegrityLimits {
                    max_files,
                    max_changed_lines,
                },
            )?;
            println!("{}", serde_json::to_string_pretty(&validation)?);
        }
        Commands::PatchPreview {
            command:
                PatchPreviewCommand::Show {
                    preview_path,
                    max_files,
                    max_changed_lines,
                },
        } => {
            let artifact: SourcePatchPreviewArtifact = serde_json::from_str(
                &std::fs::read_to_string(&preview_path).with_context(|| {
                    format!("failed to read patch preview {}", preview_path.display())
                })?,
            )
            .with_context(|| format!("failed to parse patch preview {}", preview_path.display()))?;
            let validation = ouroforge_core::inspect_source_patch_preview_artifact(
                &artifact,
                PatchDiffIntegrityLimits {
                    max_files,
                    max_changed_lines,
                },
            );
            let read_model = source_patch_preview_read_model(&validation);
            println!("{}", serde_json::to_string_pretty(&read_model)?);
        }
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
        Commands::Asset {
            command:
                AssetCommand::Validate {
                    project_root_or_manifest,
                },
        } => {
            let manifest_path = resolve_project_asset_manifest_path(&project_root_or_manifest);
            let manifest = ProjectAssetManifest::from_path(&manifest_path)?;
            let base_dir = match manifest_path.parent() {
                Some(parent) if !parent.as_os_str().is_empty() => parent,
                _ => Path::new("."),
            };
            let report = manifest.validate_assets(base_dir)?;
            println!("Asset manifest valid: {}", report.manifest_id);
            println!("Manifest: {}", manifest_path.display());
            println!("Assets: {}", report.assets);
            println!("Source-like assets: {}", report.source_like_assets);
            println!("Generated assets: {}", report.generated_assets);
            println!("Sprite atlases: {}", report.sprite_atlases);
            println!("Sprite atlas frames: {}", report.sprite_atlas_frames);
            println!(
                "Sprite atlas animations: {}",
                report.sprite_atlas_animations
            );
            println!(
                "Asset types: {}",
                format_asset_type_counts(&report.asset_types)
            );
        }
        Commands::Asset {
            command:
                AssetCommand::AuditInternalSprites {
                    reference_root,
                    profile,
                    json,
                },
        } => {
            let profile = InternalSpriteAuditProfile::parse(&profile)?;
            let report = audit_internal_sprite_reference(&reference_root, profile)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print_internal_sprite_audit_report(&report);
            }
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
            let agent_handoff_v2s = list_agent_handoff_v2_contracts(&runs_root)?;
            let loop_cockpit = build_studio_loop_cockpit_read_model(
                &loop_evidence_bundles,
                &agent_handoffs,
                &agent_handoff_v2s,
            );
            let payload = serde_json::json!({
                "schema": "ouroforge-dashboard-v1",
                "runs_root": runs_root,
                "runs": runs,
                "regression_matrix": regression_matrix,
                "loop_evidence_bundles": loop_evidence_bundles,
                "agent_handoffs": agent_handoffs,
                "agent_handoff_v2s": agent_handoff_v2s,
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
        Commands::RuntimeDebug {
            command:
                RuntimeDebugCommand::FrameBudget {
                    command: RuntimeFrameBudgetCommand::Validate { budget_path },
                },
        } => {
            let budget = read_runtime_frame_budget(&budget_path)?;
            let counts = budget.debug_counts();
            let status = match budget.status() {
                RuntimeFrameBudgetStatus::WithinBudget => "within-budget",
                RuntimeFrameBudgetStatus::Violated => "violated",
            };
            println!("Runtime frame budget valid: {}", budget.frame_id);
            println!("Scene: {}", budget.scene_id);
            println!("Status: {status}");
            println!("Draw calls: {}", counts.draw_call_count);
            println!("Layers: {}", counts.layer_count);
            println!("Collision pairs: {}", counts.collision_pair_count);
            println!("Violation(s): {}", budget.computed_violations().len());
        }
        Commands::RuntimeDebug {
            command:
                RuntimeDebugCommand::FrameBudget {
                    command: RuntimeFrameBudgetCommand::Show { budget_path },
                },
        } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&read_runtime_frame_budget(budget_path)?)?
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
        Commands::Edit {
            command:
                EditCommand::DraftPreview {
                    draft_path,
                    project,
                    transaction_output,
                },
        } => {
            let preview = preview_visual_edit_draft_cli(
                &draft_path,
                &project,
                transaction_output.as_deref(),
            )?;
            println!("{}", serde_json::to_string_pretty(&preview)?);
        }
        Commands::Edit {
            command:
                EditCommand::DraftApply {
                    draft_path,
                    project,
                    run_dir,
                    proposal,
                    decision,
                    transaction_output,
                },
        } => {
            let result = apply_visual_edit_draft_cli(
                &draft_path,
                &project,
                &run_dir,
                &proposal,
                &decision,
                &transaction_output,
            )?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::Plugin { command } => {
            handle_plugin_command(command)?;
        }
    }

    Ok(())
}

fn resolve_project_asset_manifest_path(project_root_or_manifest: &Path) -> PathBuf {
    if project_root_or_manifest.is_dir() {
        project_root_or_manifest.join("asset-manifest.json")
    } else if project_root_or_manifest
        .file_name()
        .and_then(|name| name.to_str())
        == Some("ouroforge.project.json")
    {
        project_root_or_manifest
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("asset-manifest.json")
    } else {
        project_root_or_manifest.to_path_buf()
    }
}

fn format_asset_type_counts(
    counts: &std::collections::BTreeMap<ProjectAssetType, usize>,
) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }
    counts
        .iter()
        .map(|(asset_type, count)| format!("{}={count}", project_asset_type_label(*asset_type)))
        .collect::<Vec<_>>()
        .join(",")
}

fn print_internal_sprite_audit_report(report: &InternalSpriteAuditReport) {
    println!("Internal sprite audit: {}", report.profile);
    println!("Reference root: {}", report.reference_root);
    println!("Render readiness: {}", report.render_readiness.status);
    println!("PNG frames: {}", report.inventory.png_frames);
    println!("Required files: {}", report.inventory.required_files);
    println!(
        "Present required files: {}",
        report.inventory.present_required_files
    );
    println!(
        "Missing required files: {}",
        report.missing_required_files.len()
    );
    println!(
        "License scope: {}",
        report.distribution_policy.license_scope
    );
    println!(
        "Git commit allowed: {}",
        yes_no(report.distribution_policy.git_commit_allowed)
    );
    println!(
        "Screenshot capture allowed: {}",
        yes_no(report.distribution_policy.screenshot_allowed)
    );
    println!(
        "Upload allowed: {}",
        yes_no(report.distribution_policy.upload_allowed)
    );
    println!(
        "Copied private files: {}",
        report.distribution_policy.copied_private_files
    );
    for note in &report.issue_notes {
        println!("Issue note: {note}");
    }
}

const fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn project_asset_type_label(asset_type: ProjectAssetType) -> &'static str {
    match asset_type {
        ProjectAssetType::Image => "image",
        ProjectAssetType::SpriteAtlas => "sprite_atlas",
        ProjectAssetType::Tileset => "tileset",
        ProjectAssetType::Tilemap => "tilemap",
        ProjectAssetType::Audio => "audio",
        ProjectAssetType::Font => "font",
        ProjectAssetType::Mesh => "mesh",
        ProjectAssetType::Material => "material",
    }
}

fn resolve_project_manifest_path(project_root_or_manifest: &Path) -> PathBuf {
    if project_root_or_manifest.is_dir() {
        project_root_or_manifest.join("ouroforge.project.json")
    } else {
        project_root_or_manifest.to_path_buf()
    }
}

fn read_behavior_draft(path: &Path) -> Result<BehaviorDraftArtifact> {
    BehaviorDraftArtifact::from_json_str(
        &std::fs::read_to_string(path)
            .with_context(|| format!("failed to read behavior draft {}", path.display()))?,
    )
    .with_context(|| format!("failed to validate behavior draft {}", path.display()))
}

fn read_behavior_apply_transaction(path: &Path) -> Result<BehaviorApplyTransactionArtifact> {
    BehaviorApplyTransactionArtifact::from_json_str(&std::fs::read_to_string(path).with_context(
        || {
            format!(
                "failed to read behavior apply transaction {}",
                path.display()
            )
        },
    )?)
    .with_context(|| {
        format!(
            "failed to validate behavior apply transaction {}",
            path.display()
        )
    })
}

fn behavior_draft_target_check(
    draft: &BehaviorDraftArtifact,
    project_root: Option<&Path>,
) -> Result<serde_json::Value> {
    let Some(project_root) = project_root else {
        return Ok(serde_json::json!({
            "checked": false,
            "reason": "project-root not provided"
        }));
    };
    let scene_path = project_root.join(&draft.target.scene_path);
    let current = hash_scene_document(&read_scene(&scene_path).with_context(|| {
        format!(
            "failed to read behavior draft target scene {}",
            scene_path.display()
        )
    })?)?;
    let current_hash = format!("{}:{}", current.algorithm, current.value);
    Ok(serde_json::json!({
        "checked": true,
        "scenePath": scene_path,
        "expectedHash": draft.target.scene_hash,
        "currentHash": current_hash,
        "stale": current_hash != draft.target.scene_hash
    }))
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

fn preview_visual_edit_draft_cli(
    draft_path: &Path,
    project_root_or_manifest: &Path,
    transaction_output: Option<&Path>,
) -> Result<serde_json::Value> {
    let draft_input = std::fs::read_to_string(draft_path)
        .with_context(|| format!("failed to read visual edit draft {}", draft_path.display()))?;
    let draft: VisualEditDraftArtifact = serde_json::from_str(&draft_input)
        .with_context(|| format!("failed to parse visual edit draft {}", draft_path.display()))?;
    let manifest_path = resolve_project_manifest_path(project_root_or_manifest);
    let manifest = ProjectManifest::from_path(&manifest_path)?;
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    match draft.target.target_type {
        VisualEditDraftTargetType::Scene => {
            let scene_ref = resolve_visual_edit_scene_target(&draft, &manifest)?;
            let scene_path = project_root.join(&scene_ref.path);
            let previews = draft
                .preview_scene_edit_transactions(&scene_path)
                .with_context(|| {
                    format!(
                        "failed to preview scene visual edit draft {} against {}",
                        draft_path.display(),
                        scene_path.display()
                    )
                })?;
            let transaction_output_preflight = match transaction_output {
                Some(output_path) => Some(preflight_visual_edit_transaction_output(
                    output_path,
                    &scene_path,
                    previews.len(),
                )?),
                None => None,
            };
            Ok(serde_json::json!({
                "schemaVersion": "visual-edit-draft-preview-cli-v1",
                "draftId": draft.draft_id,
                "projectId": manifest.project.id,
                "manifestPath": manifest_path.to_string_lossy(),
                "target": {
                    "type": "scene",
                    "path": scene_ref.path,
                    "id": scene_ref.id,
                },
                "previewKind": "scene_edit_transactions",
                "previews": previews,
                "projectPreflight": {
                    "status": "passed",
                    "manifestHash": hash_project_manifest_file(&manifest_path)?,
                    "scenePath": scene_ref.path,
                    "sceneDeclaredInManifest": true,
                    "hashValidation": "passed",
                },
                "transactionOutputPreflight": transaction_output_preflight,
                "guardrail": "preview only; no scene writes, transaction output writes, browser trusted writes, or apply behavior",
            }))
        }
        VisualEditDraftTargetType::AssetReference => {
            if transaction_output.is_some() {
                return Err(anyhow!(
                    "edit draft-preview --transaction-output is only valid for scene transaction previews"
                ));
            }
            let asset_manifest_path = resolve_project_asset_manifest_path(project_root_or_manifest);
            let asset_manifest = ProjectAssetManifest::from_path(&asset_manifest_path)?;
            let asset_manifest_base = asset_manifest_path
                .parent()
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or_else(|| Path::new("."));
            let previews = draft
                .preview_asset_reference_drafts(&asset_manifest, asset_manifest_base)
                .with_context(|| {
                    format!(
                        "failed to preview asset-reference visual edit draft {} against {}",
                        draft_path.display(),
                        asset_manifest_path.display()
                    )
                })?;
            Ok(serde_json::json!({
                "schemaVersion": "visual-edit-draft-preview-cli-v1",
                "draftId": draft.draft_id,
                "projectId": manifest.project.id,
                "manifestPath": manifest_path.to_string_lossy(),
                "target": {
                    "type": "asset-reference",
                    "path": draft.target.path,
                    "id": draft.target.id,
                },
                "previewKind": "asset_reference_drafts",
                "previews": previews,
                "projectPreflight": {
                    "status": "passed",
                    "manifestHash": hash_project_manifest_file(&manifest_path)?,
                    "assetManifestPath": asset_manifest_path.to_string_lossy(),
                    "assetManifestId": asset_manifest.id,
                    "assetReferencesValidated": true,
                },
                "transactionOutputPreflight": null,
                "guardrail": "preview only; no asset file writes, transaction output writes, browser trusted writes, remote fetches, or apply behavior",
            }))
        }
        VisualEditDraftTargetType::Tilemap => Err(anyhow!(
            "edit draft-preview reserves {:?} draft targets for later #348 PR units",
            draft.target.target_type
        )),
    }
}

fn preflight_visual_edit_transaction_output(
    output_path: &Path,
    scene_path: &Path,
    preview_count: usize,
) -> Result<serde_json::Value> {
    if preview_count != 1 {
        return Err(anyhow!(
            "edit draft-preview --transaction-output requires exactly one scene transaction preview; found {preview_count}"
        ));
    }
    reject_transaction_output_target_collision(output_path, scene_path)?;
    Ok(serde_json::json!({
        "status": "passed",
        "path": output_path.to_string_lossy(),
        "checked": [
            "single-preview-output",
            "not-target-scene-path-or-alias"
        ],
        "guardrail": "preflight only; transaction output is not written by draft-preview",
    }))
}

fn apply_visual_edit_draft_cli(
    draft_path: &Path,
    project_root_or_manifest: &Path,
    run_dir: &Path,
    proposal_id: &str,
    decision_id: &str,
    transaction_output: &Path,
) -> Result<serde_json::Value> {
    if decision_id.trim().is_empty() {
        return Err(anyhow!("edit draft-apply requires --decision"));
    }
    let draft_input = std::fs::read_to_string(draft_path)
        .with_context(|| format!("failed to read visual edit draft {}", draft_path.display()))?;
    let draft: VisualEditDraftArtifact = serde_json::from_str(&draft_input)
        .with_context(|| format!("failed to parse visual edit draft {}", draft_path.display()))?;
    let review_gate = draft.review_gate.as_ref().ok_or_else(|| {
        anyhow!("edit draft-apply requires draft reviewGate linkage from VA1.8.1")
    })?;
    if review_gate.proposal_id != proposal_id {
        return Err(anyhow!(
            "edit draft-apply --proposal {} does not match draft reviewGate proposalId {}",
            proposal_id,
            review_gate.proposal_id
        ));
    }
    if review_gate.review_decision_id != decision_id {
        return Err(anyhow!(
            "edit draft-apply --decision {} does not match draft reviewGate reviewDecisionId {}",
            decision_id,
            review_gate.review_decision_id
        ));
    }
    let review_preflight = validate_visual_edit_draft_review_preflight(run_dir, &draft)?;
    if draft.target.target_type != VisualEditDraftTargetType::Scene {
        return Err(anyhow!(
            "edit draft-apply supports scene drafts only in #348 VA1.6.3"
        ));
    }
    if draft.proposed_operations.len() != 1 {
        return Err(anyhow!(
            "edit draft-apply requires exactly one scene operation; found {}",
            draft.proposed_operations.len()
        ));
    }

    let manifest_path = resolve_project_manifest_path(project_root_or_manifest);
    let manifest = ProjectManifest::from_path(&manifest_path)?;
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let scene_ref = resolve_visual_edit_scene_target(&draft, &manifest)?;
    let scene_path = project_root.join(&scene_ref.path);
    let mut previews = draft
        .preview_scene_edit_transactions(&scene_path)
        .with_context(|| {
            format!(
                "failed to preflight scene visual edit draft {} against {}",
                draft_path.display(),
                scene_path.display()
            )
        })?;
    if previews.len() != 1 {
        return Err(anyhow!(
            "edit draft-apply requires exactly one scene transaction preview; found {}",
            previews.len()
        ));
    }
    reject_generated_artifact_source_collision(
        transaction_output,
        "visual edit draft apply transaction",
    )?;
    reject_transaction_output_target_collision(transaction_output, &scene_path)?;
    let preview = previews.remove(0);
    let operation = SceneOnlyMutationOperation {
        schema_version: "scene-only-mutation-v1".to_string(),
        proposal_id: proposal_id.to_string(),
        target_scene_path: scene_path.to_string_lossy().to_string(),
        project: Some(ProjectSceneMutationContext {
            project_id: manifest.project.id.clone(),
            manifest_path: manifest_path.to_string_lossy().to_string(),
            manifest_hash: hash_project_manifest_file(&manifest_path)?,
            scene_path: scene_ref.path.clone(),
            scene_hash: preview.before_scene_hash.clone(),
        }),
        edit: preview.edit.clone(),
        review_decision_id: Some(decision_id.to_string()),
        expected_before_scene_hash: preview.before_scene_hash.clone(),
        validation_required: true,
    };
    // Fail closed before any trusted scene mutation: reject a duplicate apply of
    // an already-applied review decision so a rerun cannot mutate the scene and
    // only then be rejected by the application append.
    reject_already_applied_visual_edit_draft_decision(run_dir, decision_id)?;
    let transaction = apply_scene_only_mutation_operation(run_dir, &operation, transaction_output)?;
    let command_context = visual_edit_draft_apply_command_context(
        draft_path,
        project_root_or_manifest,
        run_dir,
        proposal_id,
        decision_id,
        transaction_output,
    );
    let visual_application = append_visual_edit_draft_application(
        run_dir,
        &draft,
        &transaction,
        transaction_output,
        &scene_path,
        command_context,
    )?;
    Ok(serde_json::json!({
        "schemaVersion": "visual-edit-draft-apply-cli-v1",
        "draftId": draft.draft_id,
        "projectId": manifest.project.id,
        "proposalId": proposal_id,
        "patchDraftId": review_preflight.patch_draft_id,
        "reviewDecisionId": decision_id,
        "visualEditApplicationId": visual_application.id,
        "transactionId": transaction.id,
        "transactionOutput": transaction_output.to_string_lossy(),
        "target": {
            "type": "scene",
            "path": scene_ref.path,
            "id": scene_ref.id,
        },
        "beforeSceneHash": visual_application.before_scene_hash,
        "afterSceneHash": visual_application.after_scene_hash,
        "rollback": transaction.rollback,
        "commandContext": visual_application.command_context,
        "guardrail": "review-gated manual CLI apply only; no browser apply, auto-apply, source mutation, or hidden trusted writes",
    }))
}

fn shell_quote_cli_arg(arg: &str) -> String {
    if arg
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-' | ':' | '='))
    {
        return arg.to_string();
    }
    format!("'{}'", arg.replace('\'', "'\\''"))
}

fn visual_edit_draft_apply_command_context(
    draft_path: &Path,
    project_root_or_manifest: &Path,
    run_dir: &Path,
    proposal_id: &str,
    decision_id: &str,
    transaction_output: &Path,
) -> VisualEditDraftApplyCommandContext {
    let argv = vec![
        "cargo".to_string(),
        "run".to_string(),
        "-p".to_string(),
        "ouroforge-cli".to_string(),
        "--".to_string(),
        "edit".to_string(),
        "draft-apply".to_string(),
        draft_path.to_string_lossy().to_string(),
        "--project".to_string(),
        project_root_or_manifest.to_string_lossy().to_string(),
        "--run-dir".to_string(),
        run_dir.to_string_lossy().to_string(),
        "--proposal".to_string(),
        proposal_id.to_string(),
        "--decision".to_string(),
        decision_id.to_string(),
        "--transaction-output".to_string(),
        transaction_output.to_string_lossy().to_string(),
    ];
    let command = argv
        .iter()
        .map(|arg| shell_quote_cli_arg(arg))
        .collect::<Vec<_>>()
        .join(" ");
    VisualEditDraftApplyCommandContext {
        schema_version: "visual-edit-draft-apply-command-context-v1".to_string(),
        command,
        argv,
        draft_path: draft_path.to_string_lossy().to_string(),
        project_path: project_root_or_manifest.to_string_lossy().to_string(),
        run_dir: run_dir.to_string_lossy().to_string(),
        transaction_output: transaction_output.to_string_lossy().to_string(),
        guardrail: "reproducible CLI context only; dashboards may display but must not execute it"
            .to_string(),
    }
}

fn resolve_visual_edit_scene_target<'a>(
    draft: &VisualEditDraftArtifact,
    manifest: &'a ProjectManifest,
) -> Result<&'a ouroforge_core::ProjectManifestPathRef> {
    let by_path = manifest
        .scenes
        .iter()
        .find(|scene_ref| scene_ref.path == draft.target.path);
    let by_id = draft.target.id.as_ref().and_then(|target_id| {
        manifest
            .scenes
            .iter()
            .find(|scene_ref| &scene_ref.id == target_id)
    });
    match (by_path, by_id) {
        (Some(path_match), Some(id_match)) if path_match.id == id_match.id => Ok(path_match),
        (Some(path_match), None) if draft.target.id.is_none() => Ok(path_match),
        (Some(_), Some(_)) => Err(anyhow!(
            "visual edit draft scene target path {} and id {:?} refer to different project scenes",
            draft.target.path,
            draft.target.id
        )),
        (Some(path_match), None) => Err(anyhow!(
            "visual edit draft scene target id {:?} is not declared in project manifest",
            draft.target.id
        ))
        .with_context(|| format!("matched scene path {}", path_match.path)),
        (None, Some(_)) => Err(anyhow!(
            "visual edit draft scene target path {} is not declared in project manifest",
            draft.target.path
        )),
        (None, None) => Err(anyhow!(
            "visual edit draft scene target {} is not declared in project manifest",
            draft.target.path
        )),
    }
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

/// Read-only plugin registry inspection (#752). Discovers and validates local
/// plugins; never installs, updates, runs, enables, or mutates anything.
fn handle_plugin_command(command: PluginCommand) -> Result<()> {
    match command {
        PluginCommand::List { dir } => {
            reject_generated_discovery_root(&dir)?;
            let registry = ouroforge_core::plugin_registry::discover_plugins_in_dir(&dir)?;
            let read_model = registry.read_model();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "schemaVersion": "ouroforge.plugin-cli-list.v1",
                    "root": registry.root,
                    "summary": read_model,
                    "plugins": registry.entries,
                    "guardrail": "read-only plugin inspection; no install, update, run, enable, disable, delete, publish, or marketplace behavior"
                }))?
            );
        }
        PluginCommand::Validate { dir } => {
            reject_generated_discovery_root(&dir)?;
            let registry = ouroforge_core::plugin_registry::discover_plugins_in_dir(&dir)?;
            let read_model = registry.read_model();
            let conflicts = ouroforge_core::plugin_conflicts::detect_conflicts(&registry);
            let ok = read_model.invalid_count == 0
                && read_model.blocked_count == 0
                && read_model.incompatible_count == 0
                && !conflicts.has_failures();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "schemaVersion": "ouroforge.plugin-cli-validate.v1",
                    "status": if ok { "ok" } else { "failed" },
                    "root": registry.root,
                    "summary": read_model,
                    "plugins": registry.entries,
                    "conflicts": conflicts,
                    "guardrail": "read-only plugin validation; no install, update, run, enable, disable, delete, publish, or marketplace behavior"
                }))?
            );
            if !ok {
                return Err(anyhow!(
                    "plugin validation failed: invalid/blocked/incompatible plugins or conflicts present"
                ));
            }
        }
    }
    Ok(())
}

/// Reject a user-supplied plugin discovery directory that is, or lies within, a
/// generated/evidence root. Discovery treats the supplied directory as its base,
/// so relative manifest paths never carry the generated-root prefix and the
/// registry's generated-root guard cannot fire; rejecting up front preserves the
/// contract that generated/evidence roots never host discovered plugin manifests.
fn reject_generated_discovery_root(dir: &Path) -> Result<()> {
    if ouroforge_core::plugin_registry::is_generated_discovery_root(dir) {
        return Err(anyhow!(
            "refusing to scan plugins under a generated/evidence root `{}`: generated roots (runs, evidence, dashboard-data, .omx) must never host discovered plugin manifests",
            dir.display()
        ));
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

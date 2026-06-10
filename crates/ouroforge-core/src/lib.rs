use anyhow::{anyhow, Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use tungstenite::client::IntoClientRequest;

pub use ouroforge_core_types::*;

pub mod behavior_runtime;
pub mod complexity_ladder;
pub mod evolve_iteration_journal;
pub mod live_failure_classifier;
pub mod physics_2d;
pub mod product_backlog_handoff;
pub mod product_gap_taxonomy;
pub mod production_journal;
pub mod production_decision_log;
pub use physics_2d::{
    simulate_scene_physics_step, Physics2dBlockedMovement, Physics2dCollisionEvent,
    Physics2dStepEvidence, Physics2dVector,
};
pub mod diagnosis_correction;
pub mod engine_growth_justification;
pub use ouroforge_gdd::gdd_asset_placeholder_plan;
pub use ouroforge_gdd::gdd_design_brief;
pub use ouroforge_gdd::gdd_feasibility_gate;
pub use ouroforge_gdd::gdd_gameplay_behavior_plan;
pub use ouroforge_gdd::gdd_mechanics_mapping;
pub use ouroforge_gdd::gdd_project_scaffold_plan;
pub use ouroforge_gdd::gdd_prototype_apply;
pub use ouroforge_gdd::gdd_prototype_draft_bundle;
pub use ouroforge_gdd::gdd_prototype_evidence;
pub use ouroforge_gdd::gdd_prototype_evidence_bundle;
pub use ouroforge_gdd::gdd_prototype_task_graph;
pub use ouroforge_gdd::gdd_requirement_extraction;
pub use ouroforge_gdd::gdd_scenario_acceptance_plan;
pub use ouroforge_gdd::gdd_scene_level_plan;
pub use ouroforge_source_apply::source_apply_controlled_failure_flow;
pub use ouroforge_source_apply::source_apply_review_enforcement;
pub use source_apply_review_enforcement::*;
pub mod human_artifact_intake;
pub mod optional_human_channel_contract;
pub mod optional_human_channel_demo;
pub mod proposal_amendment;
pub mod safe_source_apply_demo;
pub mod self_audit_acceptance_evaluator;
pub mod self_audit_attribution_contract;
pub mod self_audit_bottleneck_attribution;
pub mod self_audit_demo;
pub mod self_diagnosis_fix_proposal_contract;
pub mod self_improvement_loop_contract;
use behavior_runtime::{
    BehaviorRuntimeEvidenceBundle, BehaviorScenarioAssertionStatus, BehaviorScenarioAssertionSuite,
};
pub use diagnosis_correction::*;
pub use human_artifact_intake::*;
pub use optional_human_channel_contract::*;
pub use optional_human_channel_demo::*;
pub use ouroforge_source_apply::source_apply_sandbox_promotion;
pub use proposal_amendment::*;
pub use self_audit_acceptance_evaluator::*;
pub use self_audit_attribution_contract::*;
pub use self_audit_bottleneck_attribution::*;
pub use self_audit_demo::*;
pub use self_diagnosis_fix_proposal_contract::*;
pub use self_improvement_loop_contract::*;
pub use source_apply_sandbox_promotion::*;
pub mod behavior_draft;
pub use ouroforge_source_apply::source_apply_rollback_snapshot;
pub use source_apply_rollback_snapshot::*;
mod behavior_draft_validation;
pub use ouroforge_source_apply::source_apply_verification_runner;
pub use source_apply_verification_runner::*;
pub mod behavior_evidence;
pub mod design_regression_harness;
pub mod dogfood_campaign_harness;
pub use dogfood_campaign_harness::*;
pub mod evidence_marketplace_proof;
pub mod evidence_marketplace_registry;
pub mod export_asset_manifest;
pub mod export_bundle;
pub mod export_capability_report;
pub mod export_evidence;
pub mod export_fingerprint;
pub mod export_hash;
pub mod export_package_metadata;
pub mod export_plan;
pub mod export_probe_check;
pub mod export_profile;
pub mod export_release_blocker;
pub mod export_staging;
pub mod export_verification;
pub use ouroforge_source_apply::source_apply_post_apply_rerun;
pub use source_apply_post_apply_rerun::*;
pub mod deterministic_reexpression;
pub mod differential_verification;
pub mod internal_sprite_audit;
pub mod legacy_logic_ingestion;
pub mod loop_coverage_attribution;
pub mod semantic_port_coverage;
pub mod tacit_oracle_capture;
pub mod tilemap_authoring;
pub mod trust_gradient_auto_apply;
pub mod unity_2d_adapter_ir;
pub use loop_coverage_attribution::*;
pub mod loop_coverage_metric;
pub use loop_coverage_metric::*;
pub mod localization;
pub use localization::{
    generate_locale_catalog, validate_locale_catalog, LocaleCatalog, LocalizationValidationReport,
    LocalizationValidationStatus, StringCatalog, StringCatalogEntry, LOCALIZATION_BOUNDARY,
    LOCALIZATION_CATALOG_SCHEMA_VERSION, LOCALIZATION_GENERATOR,
    LOCALIZATION_LOCALE_SCHEMA_VERSION,
};
pub mod asset_generation_proposal;
pub mod asset_import;
pub mod audio_generation;
pub mod audio_hooks;
pub mod audio_qa;
pub mod balance_combo_detector;
pub mod balance_copilot;
pub mod balance_dominant_build;
pub mod balance_fairness;
pub mod candidate_generation;
pub mod card_roguelite_substrate;
pub mod content_difficulty_curve;
pub mod content_novelty;
pub mod content_scale_generation;
pub mod curation_surface;
pub mod evolve_campaign;
pub mod generative_accessibility;
pub mod generative_intake;
pub mod generative_promotion_guard;
pub mod gltf_25d_import;
pub mod godot_2d_adapter_ir;
pub mod import_verification_report;
pub mod ir_mapping_fidelity_classifier;
pub mod logic_touchpoint_handoff;
pub use evolve_campaign::*;
pub mod economy_system;
pub mod funfeel_gate;
pub mod meta_progression;
pub mod narrative_candidate;
pub mod narrative_integration;
pub mod narrative_system;
pub mod playtest_capture;
pub mod plugin_asset_metadata;
pub mod plugin_compatibility;
pub mod plugin_conflicts;
pub mod save_migration;
pub mod save_profile_scale;
pub use ouroforge_source_apply::source_apply_highrisk_blocker;
pub mod trust_gradient_risk_tier;
pub mod uiux_flow;
pub use source_apply_highrisk_blocker::*;
pub mod grid_puzzle_dsl_ingest;
pub mod patch_reverify;
pub mod performance_soak;
pub mod plugin_evidence;
pub mod plugin_extension_catalog;
pub mod plugin_manifest;
pub mod plugin_permission;
pub mod plugin_registry;
pub mod plugin_threat_model;
pub mod producer_budget_gates;
pub mod producer_orchestration;
pub mod producer_plan;
pub mod production_handoff;
pub mod production_qa_matrix;
pub mod production_qa_verdict;
pub mod production_review_gates;
pub mod production_roles;
pub mod provenance_bundle;
pub mod provenance_replay;
pub mod puzzle_difficulty_metric;
pub mod puzzle_oversolution;
pub mod puzzle_solver;
pub mod release_auto_apply;
pub mod release_compliance_gate;
pub mod release_provenance_bundle;
pub mod release_readiness;
pub mod seeded_rng;
pub mod steam_export_build;
pub mod steam_store_assets;
pub mod steamworks_integration;
pub use ouroforge_source_apply::source_apply_audit_ledger;
pub use seeded_rng::SEEDED_RNG_ALGORITHM;
pub mod trust_gradient_audit;
pub use source_apply_audit_ledger::*;
pub mod qa_error_classifier;
pub mod qa_evidence_bundle;
pub mod qa_failure_backlog;
pub mod qa_flake_rerun_policy;
pub mod qa_performance_budget;
pub mod qa_playtest_demo;
pub mod qa_regression_coverage;
pub mod qa_run_matrix;
pub mod responsiveness;
pub mod runtime_frame_budget;
pub mod score_cascade_feedback;
pub mod visual_regression_scale;
pub use card_roguelite_substrate::{
    analyze_card_roguelite_score_composition, card_roguelite_probe_state,
    card_roguelite_seed_algorithm, deck_roguelike_spec_to_substrate_config,
    default_deck_roguelike_substrate_config, default_engine_builder_deckbuilder_substrate_config,
    digest_card_roguelite_state, resolve_card_roguelite_run_ante,
    resolve_card_roguelite_score_resolution, resolve_card_roguelite_shop_economy,
    resolve_card_roguelite_state, validate_card_roguelite_config, CardRogueliteAnteStep,
    CardRogueliteCard, CardRogueliteCardScoreTrace, CardRogueliteCompositionFinding,
    CardRogueliteConfig, CardRogueliteDigest, CardRogueliteEffect, CardRogueliteMetaConfig,
    CardRogueliteModifier, CardRogueliteModifierEffect, CardRogueliteModifierEffectOperation,
    CardRogueliteModifierEffectScope, CardRogueliteProbeState, CardRogueliteReadOnlyInspection,
    CardRogueliteRunAnteReport, CardRogueliteRunAnteRound, CardRogueliteRunConfig,
    CardRogueliteScoreComposition, CardRogueliteScoreResolution, CardRogueliteScoreStep,
    CardRogueliteShopCommand, CardRogueliteShopConfig, CardRogueliteShopEconomyReport,
    CardRogueliteShopOffer, CardRogueliteShopTransaction, CardRogueliteState, CardRogueliteStatus,
    CardRogueliteUnlock, CARD_ROGUELITE_RUN_ANTE_SCHEMA_VERSION,
    CARD_ROGUELITE_SCORE_COMPOSITION_SCHEMA_VERSION,
    CARD_ROGUELITE_SCORE_RESOLUTION_SCHEMA_VERSION, CARD_ROGUELITE_SHOP_ECONOMY_SCHEMA_VERSION,
    CARD_ROGUELITE_SUBSTRATE_CONFIG_SCHEMA_VERSION, CARD_ROGUELITE_SUBSTRATE_DIGEST_ALGORITHM,
    CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION, CARD_ROGUELITE_SUBSTRATE_STATE_SCHEMA_VERSION,
};
pub use ouroforge_evaluator::*;
pub use ouroforge_evidence::{
    add_evidence_artifact, list_evidence_artifacts, read_evidence_index,
    validate_evidence_artifact_path, write_evidence_index, EvidenceArtifact, EvidenceIndex,
};
pub use ouroforge_ledger::{append_ledger_event, read_ledger_events, write_ledger_created};
pub use ouroforge_source_apply::source_apply_emergency_hold;
pub use ouroforge_source_apply::source_apply_evidence_bundle;
pub use responsiveness::{
    verify_responsiveness, ResponsivenessEvent, ResponsivenessEventKind, ResponsivenessEvidence,
    ResponsivenessMeasurement, ResponsivenessReadOnlyInspection, ResponsivenessReport,
    ResponsivenessStatus, DEFAULT_RESPONSIVENESS_BUDGET_MS, RESPONSIVENESS_EVENT_SCHEMA_VERSION,
    RESPONSIVENESS_REPORT_SCHEMA_VERSION,
};
pub use runtime_frame_budget::{read_runtime_frame_budget, RuntimeFrameBudgetStatus};
pub use score_cascade_feedback::{
    score_cascade_feedback_trace, ScoreCascadeFeedbackEvent, ScoreCascadeFeedbackTrace,
    ScoreCascadeReadOnlyInspection, SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION,
    SCORE_CASCADE_FEEDBACK_SCHEMA_VERSION,
};
pub use source_apply_emergency_hold::*;
pub use source_apply_evidence_bundle::*;

static MUTATION_INDEX_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub id: String,
    pub title: String,
    pub goal: String,
    pub constraints: Constraints,
    pub acceptance: Vec<String>,
    pub scenarios: Vec<Scenario>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluator: Option<EvaluatorConfig>,
}

pub const SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION: &str =
    "ouroforge.scenario-input-replay.v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ScenarioInputReplayArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "workerId", default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(rename = "stepIndex")]
    pub step_index: usize,
    pub action: ScenarioInputReplayAction,
    pub frame: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tick: Option<u64>,
    pub input: InputStep,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe: Option<ScenarioInputReplayProbeCorrelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<ScenarioInputReplayResultCorrelation>,
}

impl ScenarioInputReplayArtifact {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "scenario input replay artifact schemaVersion must be {}",
                SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION
            ));
        }
        validate_path_component("scenario input replay scenarioId", &self.scenario_id)?;
        if let Some(worker_id) = &self.worker_id {
            validate_path_component("scenario input replay workerId", worker_id)?;
        }
        require_text("scenario input replay action kind", &self.action.kind)?;
        if self.frame > MAX_INPUT_REPLAY_FRAME {
            return Err(anyhow!(
                "scenario input replay frame must be <= {}",
                MAX_INPUT_REPLAY_FRAME
            ));
        }
        if self.input.left.is_none()
            && self.input.right.is_none()
            && self.input.up.is_none()
            && self.input.down.is_none()
        {
            return Err(anyhow!(
                "scenario input replay input must set at least one direction"
            ));
        }
        if let Some(probe) = &self.probe {
            if probe.contract_version != RUNTIME_PROBE_CONTRACT_VERSION {
                return Err(anyhow!(
                    "scenario input replay probe contractVersion must be {}",
                    RUNTIME_PROBE_CONTRACT_VERSION
                ));
            }
            if let Some(path) = &probe.world_state_path {
                validate_evidence_artifact_path(path)
                    .with_context(|| "scenario input replay worldStatePath is invalid")?;
            }
            if let Some(path) = &probe.frame_stats_path {
                validate_evidence_artifact_path(path)
                    .with_context(|| "scenario input replay frameStatsPath is invalid")?;
            }
        }
        if let Some(result) = &self.result {
            validate_relative_artifact_path(
                "scenario input replay scenarioResultPath",
                &result.scenario_result_path,
            )?;
            if let Some(path) = &result.verdict_path {
                validate_relative_artifact_path("scenario input replay verdictPath", path)?;
            }
        }
        Ok(())
    }
}

impl Seed {
    pub fn from_yaml_str(input: &str) -> Result<Self> {
        // Scenario steps and assertions are untagged enums, so serde silently
        // accepts the first matching variant and drops any extra action/target
        // keys in the same map. Reject multi-key entries up front so a malformed
        // DSL like `{ wait: ..., input: ... }` fails validation instead of
        // quietly ignoring one of the actions.
        validate_single_key_scenario_entries(input)?;
        let seed: Seed = serde_yaml::from_str(input).context("failed to parse Seed YAML")?;
        seed.validate()?;
        Ok(seed)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = fs::read_to_string(path)
            .with_context(|| format!("failed to read Seed file {}", path.display()))?;
        let seed = Self::from_yaml_str(&input)?;
        let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
        seed.validate_replay_references(base_dir)?;
        Ok(seed)
    }

    pub fn validate(&self) -> Result<()> {
        require_text("id", &self.id)?;
        require_text("title", &self.title)?;
        require_text("goal", &self.goal)?;
        require_text("constraints.target", &self.constraints.target)?;

        if self.acceptance.is_empty() {
            return Err(anyhow!("acceptance must contain at least one item"));
        }
        for (index, item) in self.acceptance.iter().enumerate() {
            require_text(&format!("acceptance[{index}]"), item)?;
        }

        if let Some(evaluator) = &self.evaluator {
            evaluator.validate()?;
        }

        if self.scenarios.is_empty() {
            return Err(anyhow!("scenarios must contain at least one item"));
        }
        for (index, scenario) in self.scenarios.iter().enumerate() {
            scenario.validate(index)?;
        }

        Ok(())
    }

    fn validate_replay_references(&self, base_dir: &Path) -> Result<()> {
        for (scenario_index, scenario) in self.scenarios.iter().enumerate() {
            for (step_index, step) in scenario.steps.iter().enumerate() {
                if let ScenarioStep::ReplayRef { replay_ref } = step {
                    replay_ref
                        .load_from_base(base_dir)
                        .with_context(|| {
                            format!(
                                "scenarios[{scenario_index}].steps[{step_index}].replayRef could not be loaded"
                            )
                        })?;
                }
            }
        }
        Ok(())
    }

    fn replay_references(&self) -> Vec<&ReplayReference> {
        self.scenarios
            .iter()
            .flat_map(|scenario| scenario.steps.iter())
            .filter_map(|step| match step {
                ScenarioStep::ReplayRef { replay_ref } => Some(replay_ref),
                _ => None,
            })
            .collect()
    }
}

const REGRESSION_PROMOTION_DRAFT_SCHEMA_VERSION: &str = "regression-promotion-draft-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RegressionPromotionDraft {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub id: String,
    #[serde(rename = "sourceRun")]
    pub source_run: RegressionPromotionSourceRun,
    #[serde(rename = "sourceEvidence")]
    pub source_evidence: RegressionPromotionSourceEvidence,
    pub target: RegressionPromotionTarget,
    #[serde(rename = "proposedScenario")]
    pub proposed_scenario: Scenario,
    #[serde(rename = "createdAtUnixMs")]
    pub created_at_unix_ms: u128,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RegressionPromotionSourceEvidence {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "scenarioResultPath")]
    pub scenario_result_path: String,
    #[serde(
        rename = "replayArtifactPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replay_artifact_path: Option<String>,
    #[serde(rename = "evidenceIds")]
    pub evidence_ids: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

impl RegressionPromotionDraft {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let draft: RegressionPromotionDraft = serde_json::from_str(input)
            .context("failed to parse regression promotion draft JSON")?;
        draft.validate()?;
        Ok(draft)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let input = fs::read_to_string(path).with_context(|| {
            format!(
                "failed to read regression promotion draft {}",
                path.display()
            )
        })?;
        Self::from_json_str(&input).with_context(|| {
            format!(
                "failed to parse regression promotion draft {}",
                path.display()
            )
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != REGRESSION_PROMOTION_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "regression promotion draft schemaVersion must be {REGRESSION_PROMOTION_DRAFT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("regression promotion draft id", &self.id)?;
        self.source_run.validate()?;
        self.source_evidence.validate()?;
        self.target.validate()?;
        self.proposed_scenario
            .validate(0)
            .context("regression promotion proposedScenario is invalid")?;
        if self.proposed_scenario.id != self.source_evidence.scenario_id {
            return Err(anyhow!(
                "regression promotion proposedScenario id must match sourceEvidence scenarioId"
            ));
        }
        Ok(())
    }
}

impl RegressionPromotionSourceEvidence {
    fn validate(&self) -> Result<()> {
        validate_path_component(
            "regression promotion sourceEvidence scenarioId",
            &self.scenario_id,
        )?;
        validate_scenario_result_ref(&self.scenario_result_path)?;
        if let Some(replay_artifact_path) = &self.replay_artifact_path {
            validate_mutation_review_ref(replay_artifact_path)?;
        }
        if self.evidence_ids.is_empty() {
            return Err(anyhow!(
                "regression promotion sourceEvidence evidenceIds must not be empty"
            ));
        }
        let mut ids = BTreeSet::new();
        for evidence_id in &self.evidence_ids {
            validate_path_component(
                "regression promotion sourceEvidence evidenceId",
                evidence_id,
            )?;
            if !ids.insert(evidence_id.clone()) {
                return Err(anyhow!(
                    "duplicate regression promotion evidence id: {evidence_id}"
                ));
            }
        }
        if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "regression promotion sourceEvidence evidenceRefs must not be empty"
            ));
        }
        let mut refs = BTreeSet::new();
        for evidence_ref in &self.evidence_refs {
            validate_mutation_review_ref(evidence_ref)?;
            if !refs.insert(evidence_ref.clone()) {
                return Err(anyhow!(
                    "duplicate regression promotion evidence ref: {evidence_ref}"
                ));
            }
        }
        Ok(())
    }
}

pub fn write_regression_promotion_draft(
    path: impl AsRef<Path>,
    draft: &RegressionPromotionDraft,
) -> Result<()> {
    draft.validate()?;
    write_json_atomic(path.as_ref(), &json!(draft))
}

pub fn build_regression_promotion_draft_from_run(
    run_dir: impl AsRef<Path>,
    project_manifest_path: impl AsRef<Path>,
    scenario_id: &str,
) -> Result<RegressionPromotionDraft> {
    validate_path_component("regression promotion scenario id", scenario_id)?;
    let run_dir = run_dir.as_ref();
    let manifest_path = project_manifest_path.as_ref();
    let manifest = ProjectManifest::from_path(manifest_path)?;
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    manifest.validate_references(project_root)?;

    let run = read_json_value(run_dir.join("run.json"))?;
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    if verdict.get("status").and_then(|value| value.as_str()) != Some("failed") {
        return Err(anyhow!(
            "regression promotion requires a failed source run verdict"
        ));
    }
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    require_text("regression promotion source run id", &run_id)?;

    let project = run
        .get("project")
        .ok_or_else(|| anyhow!("regression promotion source run must be project-bound"))?;
    if project.get("id").and_then(|value| value.as_str()) != Some(manifest.project.id.as_str()) {
        return Err(anyhow!(
            "regression promotion project does not match source run project metadata"
        ));
    }
    let source_pack = project
        .get("scenarioPack")
        .ok_or_else(|| anyhow!("regression promotion source run must record a scenario pack"))?;
    let scenario_pack_id = source_pack
        .get("id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| anyhow!("regression promotion source run scenario pack id is missing"))?;
    let scenario_pack_path = source_pack
        .get("path")
        .and_then(|value| value.as_str())
        .ok_or_else(|| anyhow!("regression promotion source run scenario pack path is missing"))?;
    let _pack_ref = manifest
        .scenario_packs
        .iter()
        .find(|reference| reference.id == scenario_pack_id && reference.path == scenario_pack_path)
        .ok_or_else(|| {
            anyhow!(
                "regression promotion scenario pack {scenario_pack_id} is not authorized by project manifest"
            )
        })?;

    let evidence = read_evidence_index(run_dir)?;
    let scenario_result_path =
        select_failed_scenario_result_path(run_dir, &verdict, &evidence, scenario_id)?;
    let scenario_result =
        read_json_value(run_dir.join(&scenario_result_path)).with_context(|| {
            format!(
                "failed to read regression promotion scenario result {}",
                scenario_result_path
            )
        })?;
    if scenario_result
        .get("scenario_id")
        .and_then(|value| value.as_str())
        != Some(scenario_id)
    {
        return Err(anyhow!(
            "regression promotion scenario result scenario_id must match requested scenario"
        ));
    }
    if scenario_result
        .get("status")
        .and_then(|value| value.as_str())
        != Some("failed")
    {
        return Err(anyhow!(
            "regression promotion scenario result must be failed"
        ));
    }

    let replay_artifact_paths =
        select_regression_replay_paths(run_dir, &scenario_result, &evidence, scenario_id)?;
    let steps = replay_artifact_paths
        .iter()
        .map(|path| scenario_step_from_replay_artifact(run_dir, path, &evidence, scenario_id))
        .collect::<Result<Vec<_>>>()?;
    let assertions = scenario_assertions_from_failed_result(&scenario_result)?;
    let evidence_ids = evidence
        .artifacts
        .iter()
        .filter(|artifact| {
            artifact.path == scenario_result_path
                || replay_artifact_paths
                    .iter()
                    .any(|replay_path| replay_path == &artifact.path)
        })
        .map(|artifact| artifact.id.clone())
        .collect::<Vec<_>>();
    if evidence_ids.is_empty() {
        return Err(anyhow!(
            "regression promotion source evidence must be indexed"
        ));
    }
    let mut evidence_refs = Vec::new();
    evidence_refs.push(scenario_result_path.clone());
    evidence_refs.extend(replay_artifact_paths.clone());
    evidence_refs.sort();
    evidence_refs.dedup();

    let draft = RegressionPromotionDraft {
        schema_version: REGRESSION_PROMOTION_DRAFT_SCHEMA_VERSION.to_string(),
        id: format!("regression-draft-{scenario_id}-{run_id}"),
        source_run: RegressionPromotionSourceRun {
            run_id: run_id.clone(),
            run_dir: regression_promotion_run_dir_ref(run_dir)?,
            verdict_path: "verdict.json".to_string(),
        },
        source_evidence: RegressionPromotionSourceEvidence {
            scenario_id: scenario_id.to_string(),
            scenario_result_path: scenario_result_path.clone(),
            replay_artifact_path: replay_artifact_paths.first().cloned(),
            evidence_ids,
            evidence_refs,
        },
        target: RegressionPromotionTarget {
            project_manifest_path: project_relative_path(project_root, manifest_path)?,
            scenario_pack_id: scenario_pack_id.to_string(),
            scenario_pack_path: scenario_pack_path.to_string(),
            scenario_group_id: "promoted-regressions".to_string(),
        },
        proposed_scenario: Scenario {
            id: scenario_id.to_string(),
            description: format!("Promoted regression from failed run {run_id}."),
            steps,
            assertions,
        },
        created_at_unix_ms: unix_millis()?,
    };
    draft.validate()?;
    Ok(draft)
}

fn select_failed_scenario_result_path(
    run_dir: &Path,
    verdict: &serde_json::Value,
    evidence: &EvidenceIndex,
    scenario_id: &str,
) -> Result<String> {
    let mut candidates = Vec::new();
    if let Some(failures) = verdict.get("failures").and_then(|value| value.as_array()) {
        for failure in failures {
            if failure.get("scenario_id").and_then(|value| value.as_str()) == Some(scenario_id) {
                for field in ["path", "evidence_ref"] {
                    if let Some(path) = failure.get(field).and_then(|value| value.as_str()) {
                        candidates.push(path.to_string());
                    }
                }
            }
        }
    }
    for artifact in &evidence.artifacts {
        if artifact
            .metadata
            .get("scenario_id")
            .and_then(|value| value.as_str())
            == Some(scenario_id)
            && is_scenario_result_artifact_path(&artifact.path)
            && (artifact.id.contains("scenario-result")
                || artifact
                    .metadata
                    .get("artifact")
                    .and_then(|value| value.as_str())
                    == Some("scenario_result"))
        {
            candidates.push(artifact.path.clone());
        }
    }
    candidates.sort();
    candidates.dedup();
    for candidate in candidates {
        if validate_evidence_artifact_path(&candidate).is_err()
            || !is_scenario_result_artifact_path(&candidate)
        {
            continue;
        }
        let value = read_json_value(run_dir.join(&candidate))?;
        if value.get("scenario_id").and_then(|value| value.as_str()) == Some(scenario_id)
            && value.get("status").and_then(|value| value.as_str()) == Some("failed")
        {
            return Ok(candidate);
        }
    }
    Err(anyhow!(
        "regression promotion could not find failed scenario result evidence for {scenario_id}"
    ))
}

fn select_regression_replay_paths(
    run_dir: &Path,
    scenario_result: &serde_json::Value,
    evidence: &EvidenceIndex,
    scenario_id: &str,
) -> Result<Vec<String>> {
    let mut paths = Vec::new();
    for pointer in [
        "/evidence/scenario_input_replays",
        "/evidence/input_replays",
        "/evidence/replays",
    ] {
        if let Some(values) = scenario_result
            .pointer(pointer)
            .and_then(|value| value.as_array())
        {
            for value in values {
                if let Some(path) = value.as_str() {
                    paths.push(path.to_string());
                }
            }
        }
    }
    for artifact in &evidence.artifacts {
        if artifact
            .metadata
            .get("scenario_id")
            .and_then(|value| value.as_str())
            == Some(scenario_id)
            && (artifact.id.contains("input-replay")
                || artifact.path.contains("input-replay")
                || artifact
                    .metadata
                    .get("artifact")
                    .and_then(|value| value.as_str())
                    == Some("scenario_input_replay"))
        {
            paths.push(artifact.path.clone());
        }
    }
    paths.sort();
    paths.dedup();
    if paths.is_empty() {
        return Err(anyhow!(
            "regression promotion requires at least one replay evidence artifact"
        ));
    }
    for path in &paths {
        validate_evidence_artifact_path(path)?;
        if !run_dir.join(path).is_file() {
            return Err(anyhow!(
                "regression promotion replay evidence missing file: {path}"
            ));
        }
    }
    Ok(paths)
}

fn scenario_step_from_replay_artifact(
    run_dir: &Path,
    path: &str,
    evidence: &EvidenceIndex,
    scenario_id: &str,
) -> Result<ScenarioStep> {
    let value = read_json_value(run_dir.join(path))?;
    if value.get("schemaVersion").and_then(|value| value.as_str())
        == Some(SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION)
    {
        let artifact: ScenarioInputReplayArtifact = serde_json::from_value(value)
            .with_context(|| format!("failed to parse scenario input replay artifact {path}"))?;
        artifact.validate()?;
        if artifact.scenario_id != scenario_id {
            return Err(anyhow!(
                "regression promotion replay artifact {path} belongs to scenario {}, not {scenario_id}",
                artifact.scenario_id
            ));
        }
        return Ok(ScenarioStep::Input {
            input: artifact.input,
        });
    }
    if !evidence.artifacts.iter().any(|artifact| {
        artifact.path == path
            && artifact
                .metadata
                .get("scenario_id")
                .and_then(|value| value.as_str())
                == Some(scenario_id)
    }) {
        return Err(anyhow!(
            "regression promotion legacy replay artifact {path} must be indexed with matching scenario_id {scenario_id}"
        ));
    }
    let replay: InputReplay = serde_json::from_value(value)
        .with_context(|| format!("failed to parse input replay artifact {path}"))?;
    replay.validate()?;
    Ok(ScenarioStep::Replay { replay })
}

pub fn promote_regression_draft_to_scenario_pack(
    draft_path: impl AsRef<Path>,
    project_manifest_path: impl AsRef<Path>,
    scenario_pack_id: &str,
    dry_run: bool,
) -> Result<RegressionPromotionPackResult> {
    validate_path_component("regression promotion scenario pack id", scenario_pack_id)?;
    let draft = RegressionPromotionDraft::from_path(&draft_path)?;
    if draft.target.scenario_pack_id != scenario_pack_id {
        return Err(anyhow!(
            "regression promotion draft targets scenario pack {}, not {}",
            draft.target.scenario_pack_id,
            scenario_pack_id
        ));
    }
    let manifest_path = project_manifest_path.as_ref();
    let manifest = ProjectManifest::from_path(manifest_path)?;
    let project_root = manifest_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    manifest.validate_references(project_root)?;
    if draft.target.project_manifest_path != project_relative_path(project_root, manifest_path)? {
        return Err(anyhow!(
            "regression promotion draft project manifest does not match requested project"
        ));
    }
    let pack_ref = manifest
        .scenario_packs
        .iter()
        .find(|reference| reference.id == scenario_pack_id)
        .ok_or_else(|| {
            anyhow!(
                "regression promotion scenario pack {scenario_pack_id} is not authorized by project manifest"
            )
        })?;
    if pack_ref.path != draft.target.scenario_pack_path {
        return Err(anyhow!(
            "regression promotion draft scenario pack path does not match project manifest"
        ));
    }
    let pack_path = project_root.join(&pack_ref.path);
    let pack_input = fs::read(&pack_path)
        .with_context(|| format!("failed to read scenario pack {}", pack_path.display()))?;
    let before_hash = artifact_hash_from_bytes(&pack_input);
    let mut pack = ScenarioPack::from_json_str(
        std::str::from_utf8(&pack_input).context("scenario pack JSON is not UTF-8")?,
    )?;
    if pack.id != scenario_pack_id {
        return Err(anyhow!(
            "regression promotion scenario pack id mismatch: expected {}, found {}",
            scenario_pack_id,
            pack.id
        ));
    }
    if pack
        .ordered_scenario_ids()
        .iter()
        .any(|id| id == &draft.proposed_scenario.id)
    {
        return Err(anyhow!(
            "regression promotion scenario id already exists in target scenario pack: {}",
            draft.proposed_scenario.id
        ));
    }
    let mut created_group = false;
    match pack
        .scenario_groups
        .iter_mut()
        .find(|group| group.id == draft.target.scenario_group_id)
    {
        Some(group) => group.scenarios.push(draft.proposed_scenario.clone()),
        None => {
            created_group = true;
            pack.scenario_groups.push(ScenarioPackGroup {
                id: draft.target.scenario_group_id.clone(),
                description: "Promoted regression scenarios.".to_string(),
                scenarios: vec![draft.proposed_scenario.clone()],
            });
        }
    }
    pack.validate()?;
    let pack_value = json!(pack);
    let mut pack_json = serde_json::to_vec_pretty(&pack_value)
        .context("failed to serialize promoted scenario pack candidate")?;
    pack_json.push(b'\n');
    let after_hash = artifact_hash_from_bytes(&pack_json);
    let mut changes = Vec::new();
    if created_group {
        changes.push(format!("created_group:{}", draft.target.scenario_group_id));
    }
    changes.push(format!("added_scenario:{}", draft.proposed_scenario.id));
    let id = format!(
        "regression-promotion-{}-{}",
        draft.proposed_scenario.id,
        unix_millis()?
    );
    let mut result = RegressionPromotionPackResult {
        schema_version: "regression-promotion-result-v1".to_string(),
        id,
        draft_id: draft.id.clone(),
        scenario_id: draft.proposed_scenario.id.clone(),
        source_run: draft.source_run.clone(),
        target: draft.target.clone(),
        dry_run,
        created_group,
        before_hash,
        after_hash,
        changes,
        record_path: None,
    };
    if dry_run {
        return Ok(result);
    }
    let record_path = format!("regression-promotions/{}.json", result.id);
    // Resolve the source run directory and verify it is actually this draft's
    // source run before writing anything. Prefer the project root that owns the
    // scenario pack (the manifest declares runsRoot relative to itself); fall
    // back to the process working directory only when it holds the *same* run.
    // Selecting a runs/<id> directory merely because it exists could write the
    // record into an unrelated project's run, corrupting its
    // regression-promotions lifecycle while leaving the real source run without
    // a record.
    let source_run_dir = resolve_regression_promotion_source_run_dir(
        project_root,
        &draft.source_run.run_dir,
        &draft.source_run.run_id,
    )?;
    let record_abs_path = source_run_dir.join(&record_path);
    if let Some(parent) = record_abs_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create regression promotion record directory {}",
                parent.display()
            )
        })?;
    }
    // Write the run-local promotion record before replacing the scenario pack so
    // the pack can never be committed without its corresponding audit record.
    result.record_path = Some(record_path.clone());
    write_json_atomic(&record_abs_path, &json!(result))?;
    write_json_atomic(&pack_path, &pack_value)?;
    Ok(result)
}

pub fn write_agent_handoff_contract_from_path(
    plan_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<AgentHandoffContract> {
    let plan_path = plan_path.as_ref();
    let output_path = output_path.as_ref();
    let input = fs::read_to_string(plan_path)
        .with_context(|| format!("failed to read authoring loop plan {}", plan_path.display()))?;
    let plan = AuthoringLoopPlan::from_json_str(&input).with_context(|| {
        format!(
            "failed to parse authoring loop plan {}",
            plan_path.display()
        )
    })?;
    let base_dir = plan_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let relative_output_path = relative_generated_output_path(base_dir, output_path);
    validate_authoring_loop_generated_artifact_path(
        "agent handoff output path",
        &relative_output_path,
        &plan.generated_state.roots,
    )?;
    let handoff = build_agent_handoff_contract(&plan, base_dir, plan_path, output_path)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create agent handoff output directory {}",
                parent.display()
            )
        })?;
    }
    write_json_atomic(output_path, &json!(handoff))?;
    let ledger_dir = ensure_authoring_loop_ledger_dir(base_dir, &plan.loop_id)?;
    append_ledger_event(
        &ledger_dir,
        "authoring_loop.handoff.generated",
        "loop-handoff",
        json!({
            "loop_id": handoff.loop_id,
            "status": handoff.status,
            "handoff_path": relative_output_path.to_string_lossy(),
            "next_safe_action": handoff.next_safe_action,
            "blockers": handoff.blockers,
            "allowed_commands": handoff.allowed_commands,
            "boundary": "Agent handoff generation writes advisory JSON and a ledger summary only; it does not execute displayed commands or grant authority."
        }),
    )?;
    Ok(handoff)
}

pub fn execute_authoring_loop_step_from_path(
    plan_path: impl AsRef<Path>,
    step_id: &str,
) -> Result<AuthoringLoopStepExecutionSummary> {
    let plan_path = plan_path.as_ref();
    let input = fs::read_to_string(plan_path)
        .with_context(|| format!("failed to read authoring loop plan {}", plan_path.display()))?;
    let plan = AuthoringLoopPlan::from_json_str(&input).with_context(|| {
        format!(
            "failed to parse authoring loop plan {}",
            plan_path.display()
        )
    })?;
    let base_dir = plan_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let (updated, summary) = execute_authoring_loop_step(&plan, base_dir, plan_path, step_id)?;
    write_json_atomic(plan_path, &json!(updated))?;
    write_authoring_loop_evidence_bundle(&updated, base_dir, plan_path)?;
    Ok(summary)
}

pub fn execute_authoring_loop_step(
    plan: &AuthoringLoopPlan,
    base_dir: impl AsRef<Path>,
    plan_path: impl AsRef<Path>,
    step_id: &str,
) -> Result<(AuthoringLoopPlan, AuthoringLoopStepExecutionSummary)> {
    plan.validate_schema()?;
    validate_path_component("authoring loop step id", step_id)?;
    let base_dir = base_dir.as_ref();
    let plan_path = plan_path.as_ref();
    let step = plan
        .steps
        .iter()
        .find(|step| step.id == step_id)
        .ok_or_else(|| anyhow!("authoring loop plan step not found: {step_id}"))?
        .clone();
    let ledger_dir = ensure_authoring_loop_ledger_dir(base_dir, &plan.loop_id)?;
    let ledger_path = ledger_dir.join("ledger.jsonl");

    let mut blocked_reasons = dry_run_global_missing_prerequisites(plan, base_dir);
    blocked_reasons.extend(dry_run_missing_prerequisites(&step, base_dir));
    if !matches!(
        step.kind,
        AuthoringLoopStepKind::RunScenarioPack
            | AuthoringLoopStepKind::CompareRuns
            | AuthoringLoopStepKind::GenerateProposal
            | AuthoringLoopStepKind::ApplyAcceptedSceneMutation
            | AuthoringLoopStepKind::Rerun
            | AuthoringLoopStepKind::PromoteRegression
    ) {
        blocked_reasons.push(format!(
            "step kind {} is reserved for a later #306 PR unit",
            step.kind.as_str()
        ));
    }
    if !blocked_reasons.is_empty() {
        let (blocked, _) = plan.update_step_status(step_id, AuthoringLoopStepStatus::Blocked)?;
        append_ledger_event(
            &ledger_dir,
            "authoring_loop.step.blocked",
            "loop-step-runner",
            json!({
                "loop_id": plan.loop_id,
                "step_id": step.id,
                "kind": step.kind.as_str(),
                "blocked_reasons": blocked_reasons,
                "boundary": "no command executed because preflight or scope gate blocked the step"
            }),
        )?;
        let summary = authoring_loop_step_execution_summary(
            &blocked,
            &step,
            plan_path,
            &ledger_path,
            "blocked",
            Vec::new(),
            blocked_reasons,
        );
        return Ok((blocked, summary));
    }

    let (running, _) = plan.update_step_status(step_id, AuthoringLoopStepStatus::Running)?;
    append_ledger_event(
        &ledger_dir,
        "authoring_loop.step.started",
        "loop-step-runner",
        json!({
            "loop_id": running.loop_id,
            "step_id": step.id,
            "kind": step.kind.as_str(),
            "boundary": "CLI-only Rust trusted step execution"
        }),
    )?;

    // Everything from here on runs after the step has started (the started
    // ledger event above is a side effect), so any failure must be persisted as
    // a failed plan status rather than left on disk as "pending".
    let finalize = (|| -> Result<(AuthoringLoopPlan, Vec<AuthoringLoopGeneratedArtifact>)> {
        let generated_artifacts = match step.kind {
            AuthoringLoopStepKind::RunScenarioPack => {
                execute_authoring_loop_run_step(&running, &step, base_dir)?
            }
            AuthoringLoopStepKind::CompareRuns => {
                execute_authoring_loop_compare_step(&step, base_dir)?
            }
            AuthoringLoopStepKind::GenerateProposal => {
                execute_authoring_loop_proposal_step(&step, base_dir)?
            }
            AuthoringLoopStepKind::ApplyAcceptedSceneMutation => {
                execute_authoring_loop_apply_step(&step, base_dir)?
            }
            AuthoringLoopStepKind::Rerun => {
                execute_authoring_loop_rerun_step(&running, &step, base_dir)?
            }
            AuthoringLoopStepKind::PromoteRegression => {
                execute_authoring_loop_promotion_step(&running, &step, base_dir)?
            }
            AuthoringLoopStepKind::RecordReviewDecision | AuthoringLoopStepKind::Summarize => {
                unreachable!("unsupported kinds are blocked before execution")
            }
        };
        let remapped = remap_authoring_loop_artifact_paths(running.clone(), &generated_artifacts)?;
        let (completed, _) =
            remapped.update_step_status(step_id, AuthoringLoopStepStatus::Completed)?;
        append_ledger_event(
            &ledger_dir,
            "authoring_loop.step.completed",
            "loop-step-runner",
            json!({
                "loop_id": completed.loop_id,
                "step_id": step.id,
                "kind": step.kind.as_str(),
                "generated_artifacts": generated_artifacts,
                "boundary": "bounded run/compare/proposal step completed without browser execution or source-code mutation"
            }),
        )?;
        Ok((completed, generated_artifacts))
    })();

    let (completed, generated_artifacts) = match finalize {
        Ok(value) => value,
        Err(error) => {
            // Persist the failed status so the next invocation sees the failure
            // instead of rerunning or duplicating work. Best-effort: persistence
            // failures never mask the original execution error.
            if let Ok((failed, _)) =
                running.update_step_status(step_id, AuthoringLoopStepStatus::Failed)
            {
                let _ = write_json_atomic(plan_path, &json!(failed));
                let _ = append_ledger_event(
                    &ledger_dir,
                    "authoring_loop.step.failed",
                    "loop-step-runner",
                    json!({
                        "loop_id": failed.loop_id,
                        "step_id": step.id,
                        "kind": step.kind.as_str(),
                        "error": error.to_string(),
                        "boundary": "step execution failed after start; persisted failed plan state"
                    }),
                );
            }
            return Err(error);
        }
    };
    let summary = authoring_loop_step_execution_summary(
        &completed,
        &step,
        plan_path,
        &ledger_path,
        "completed",
        generated_artifacts,
        Vec::new(),
    );
    Ok((completed, summary))
}

fn execute_authoring_loop_run_step(
    plan: &AuthoringLoopPlan,
    step: &AuthoringLoopStep,
    base_dir: &Path,
) -> Result<Vec<AuthoringLoopGeneratedArtifact>> {
    let seed_path = base_dir.join(&plan.seed.path);
    let runs_root = base_dir.join("runs");
    reject_generated_artifact_source_collision(&runs_root, "authoring loop run step")?;
    let artifacts = create_run(&seed_path, &runs_root)?;
    let manifest_path = base_dir.join(&plan.project.manifest_path);
    let mut project_metadata = project_run_metadata_from_manifest(
        &manifest_path,
        &seed_path,
        Some(plan.scenario_pack.id.as_str()),
    )?;
    project_metadata.transaction_id = None;
    let bound_project = bind_run_project_metadata(&artifacts.run_dir, project_metadata)?;
    let command_context =
        run_command_context_for_run(&seed_path, &runs_root, 1, Some(&bound_project), None);
    bind_run_command_context(&artifacts.run_dir, command_context)?;
    append_ledger_event(
        &artifacts.run_dir,
        "authoring_loop.run_step",
        "loop-step-runner",
        json!({
            "loop_step_id": step.id,
            "loop_step_kind": step.kind.as_str(),
            "scenario_pack": plan.scenario_pack.id,
            "boundary": "created run artifacts only; scenario/browser execution is not invoked by this step"
        }),
    )?;
    Ok(vec![AuthoringLoopGeneratedArtifact {
        id: step
            .expected_artifacts
            .first()
            .map(|artifact| artifact.id.clone())
            .unwrap_or_else(|| "run".to_string()),
        path: relative_to_base(base_dir, &artifacts.run_dir.join("run.json"))?,
        kind: "run".to_string(),
    }])
}

fn execute_authoring_loop_compare_step(
    step: &AuthoringLoopStep,
    base_dir: &Path,
) -> Result<Vec<AuthoringLoopGeneratedArtifact>> {
    let run_inputs = step
        .inputs
        .iter()
        .filter(|input| input.path.ends_with("/run.json") || input.path == "run.json")
        .map(|input| base_dir.join(&input.path))
        .collect::<Vec<_>>();
    if run_inputs.len() < 2 {
        return Err(anyhow!(
            "compare-runs step {} requires at least two run.json input artifacts",
            step.id
        ));
    }
    let before_run_dir = run_inputs[0]
        .parent()
        .ok_or_else(|| anyhow!("compare-runs before input has no run directory"))?;
    let after_run_dir = run_inputs[1]
        .parent()
        .ok_or_else(|| anyhow!("compare-runs after input has no run directory"))?;
    let output_dir = step
        .expected_artifacts
        .first()
        .and_then(|artifact| {
            Path::new(&artifact.path)
                .parent()
                .map(|path| base_dir.join(path))
        })
        .unwrap_or_else(|| base_dir.join("runs/comparisons"));
    reject_generated_artifact_source_collision(&output_dir, "authoring loop comparison")?;
    let comparison_path = write_run_comparison_artifact(before_run_dir, after_run_dir, output_dir)?;
    Ok(vec![AuthoringLoopGeneratedArtifact {
        id: step
            .expected_artifacts
            .first()
            .map(|artifact| artifact.id.clone())
            .unwrap_or_else(|| "comparison".to_string()),
        path: relative_to_base(base_dir, &comparison_path)?,
        kind: "run-comparison".to_string(),
    }])
}

fn execute_authoring_loop_proposal_step(
    step: &AuthoringLoopStep,
    base_dir: &Path,
) -> Result<Vec<AuthoringLoopGeneratedArtifact>> {
    let run_json = step
        .inputs
        .iter()
        .find(|input| input.path.ends_with("/run.json") || input.path == "run.json")
        .ok_or_else(|| {
            anyhow!(
                "generate-proposal step {} requires a run.json input artifact",
                step.id
            )
        })?;
    let run_json_path = base_dir.join(&run_json.path);
    let run_dir = run_json_path
        .parent()
        .ok_or_else(|| anyhow!("generate-proposal input has no run directory"))?;
    let summary = evolve_run(run_dir)?;
    if summary.status != "proposed" {
        return Err(anyhow!(
            "generate-proposal step {} did not create a proposal: {}",
            step.id,
            summary.reason
        ));
    }
    let mut artifacts = Vec::new();
    for (id, path, kind) in [
        ("proposal", "mutation/proposals.json", "mutation-proposals"),
        (
            "classification",
            "mutation/classifications.json",
            "mutation-classifications",
        ),
        ("patch-draft", "mutation/patch-drafts.json", "patch-drafts"),
    ] {
        let absolute = run_dir.join(path);
        if absolute.is_file() {
            artifacts.push(AuthoringLoopGeneratedArtifact {
                id: step
                    .expected_artifacts
                    .iter()
                    .find(|artifact| artifact.id == id || artifact.path.ends_with(path))
                    .map(|artifact| artifact.id.clone())
                    .unwrap_or_else(|| id.to_string()),
                path: relative_to_base(base_dir, &absolute)?,
                kind: kind.to_string(),
            });
        }
    }
    if artifacts.is_empty() {
        return Err(anyhow!(
            "generate-proposal step {} completed without proposal artifacts",
            step.id
        ));
    }
    Ok(artifacts)
}

fn execute_authoring_loop_apply_step(
    step: &AuthoringLoopStep,
    base_dir: &Path,
) -> Result<Vec<AuthoringLoopGeneratedArtifact>> {
    let operation_path = find_input_parseable_as::<SceneOnlyMutationOperation>(step, base_dir)
        .ok_or_else(|| {
            anyhow!(
                "apply-accepted-scene-mutation step {} requires a scene mutation operation input",
                step.id
            )
        })?;
    let operation_input = fs::read_to_string(&operation_path)
        .with_context(|| format!("failed to read operation {}", operation_path.display()))?;
    let operation: SceneOnlyMutationOperation = serde_json::from_str(&operation_input)
        .with_context(|| format!("failed to parse operation {}", operation_path.display()))?;
    if operation.review_decision_id.is_none() {
        return Err(anyhow!(
            "apply-accepted-scene-mutation step {} requires reviewDecisionId before writes",
            step.id
        ));
    }
    let run_dir = find_run_dir_for_step(step, base_dir)?;
    let transaction_output = step
        .expected_artifacts
        .first()
        .map(|artifact| base_dir.join(&artifact.path))
        .ok_or_else(|| {
            anyhow!(
                "apply-accepted-scene-mutation step {} requires a transaction expected artifact",
                step.id
            )
        })?;
    reject_transaction_output_target_collision(
        &transaction_output,
        &base_dir.join(&operation.target_scene_path),
    )?;
    let transaction =
        apply_scene_only_mutation_operation(&run_dir, &operation, &transaction_output)?;
    Ok(vec![AuthoringLoopGeneratedArtifact {
        id: step
            .expected_artifacts
            .first()
            .map(|artifact| artifact.id.clone())
            .unwrap_or_else(|| "transaction".to_string()),
        path: relative_to_base(base_dir, &transaction_output)?,
        kind: format!("scene-edit-transaction:{}", transaction.id),
    }])
}

fn execute_authoring_loop_rerun_step(
    plan: &AuthoringLoopPlan,
    step: &AuthoringLoopStep,
    base_dir: &Path,
) -> Result<Vec<AuthoringLoopGeneratedArtifact>> {
    let transaction_path = step
        .inputs
        .iter()
        .map(|input| base_dir.join(&input.path))
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.contains("transaction") && name.ends_with(".json"))
        })
        .ok_or_else(|| anyhow!("rerun step {} requires a transaction input", step.id))?;
    let seed_path = base_dir.join(&plan.seed.path);
    let runs_root = base_dir.join("runs");
    reject_generated_artifact_source_collision(&runs_root, "authoring loop rerun step")?;
    let artifacts = create_run(&seed_path, &runs_root)?;
    let provenance = bind_run_transaction_provenance(&artifacts.run_dir, &transaction_path)
        .with_context(|| "rerun step could not bind transaction provenance")?;
    let manifest_path = base_dir.join(&plan.project.manifest_path);
    let mut project_metadata = project_run_metadata_from_manifest(
        &manifest_path,
        &seed_path,
        Some(plan.scenario_pack.id.as_str()),
    )?;
    project_metadata.transaction_id = Some(provenance.transaction_id.clone());
    let bound_project = bind_run_project_metadata(&artifacts.run_dir, project_metadata)?;
    let command_context = run_command_context_for_run(
        &seed_path,
        &runs_root,
        1,
        Some(&bound_project),
        Some(&transaction_path),
    );
    bind_run_command_context(&artifacts.run_dir, command_context)?;
    append_ledger_event(
        &artifacts.run_dir,
        "authoring_loop.rerun_step",
        "loop-step-runner",
        json!({
            "loop_step_id": step.id,
            "transaction_id": provenance.transaction_id,
            "boundary": "created transaction-bound rerun artifacts only; browser execution is not invoked by this step"
        }),
    )?;
    Ok(vec![AuthoringLoopGeneratedArtifact {
        id: step
            .expected_artifacts
            .first()
            .map(|artifact| artifact.id.clone())
            .unwrap_or_else(|| "after-run".to_string()),
        path: relative_to_base(base_dir, &artifacts.run_dir.join("run.json"))?,
        kind: "rerun".to_string(),
    }])
}

fn execute_authoring_loop_promotion_step(
    plan: &AuthoringLoopPlan,
    step: &AuthoringLoopStep,
    base_dir: &Path,
) -> Result<Vec<AuthoringLoopGeneratedArtifact>> {
    if !step_has_accepted_decision(
        step,
        base_dir,
        AuthoringLoopDecisionKind::RegressionPromotion,
    )? {
        return Err(anyhow!(
            "promote-regression step {} requires an accepted regression promotion decision before writes",
            step.id
        ));
    }
    let draft_path = find_input_parseable_as::<RegressionPromotionDraft>(step, base_dir)
        .ok_or_else(|| {
            anyhow!(
                "promote-regression step {} requires a regression promotion draft input",
                step.id
            )
        })?;
    let draft = RegressionPromotionDraft::from_path(&draft_path)?;
    let project_root = dry_run_project_artifact_root(plan, base_dir);
    let result = promote_regression_draft_to_scenario_pack(
        &draft_path,
        project_root.join(&draft.target.project_manifest_path),
        &draft.target.scenario_pack_id,
        false,
    )?;
    let record_path = result.record_path.clone().ok_or_else(|| {
        anyhow!(
            "promote-regression step {} completed without a promotion record",
            step.id
        )
    })?;
    Ok(vec![AuthoringLoopGeneratedArtifact {
        id: step
            .expected_artifacts
            .first()
            .map(|artifact| artifact.id.clone())
            .unwrap_or_else(|| "promotion".to_string()),
        path: format!("{}/{}", draft.source_run.run_dir, record_path),
        kind: "regression-promotion".to_string(),
    }])
}

fn ensure_authoring_loop_ledger_dir(base_dir: &Path, loop_id: &str) -> Result<PathBuf> {
    validate_path_component("authoring loop id", loop_id)?;
    let ledger_dir = base_dir.join("runs/authoring-loop-ledgers").join(loop_id);
    fs::create_dir_all(&ledger_dir).with_context(|| {
        format!(
            "failed to create authoring loop ledger dir {}",
            ledger_dir.display()
        )
    })?;
    let ledger_path = ledger_dir.join("ledger.jsonl");
    if !ledger_path.exists() {
        write_ledger_created(&ledger_path, unix_millis()?)?;
    }
    Ok(ledger_dir)
}

pub fn create_minimal_2d_project_scaffold(
    destination: impl AsRef<Path>,
) -> Result<ProjectManifestValidationReport> {
    let destination = destination.as_ref();
    validate_project_scaffold_destination_path(destination)?;
    if destination.exists() {
        if !destination.is_dir() {
            return Err(anyhow!(
                "project scaffold destination exists and is not a directory: {}",
                destination.display()
            ));
        }
        if destination
            .read_dir()
            .with_context(|| format!("failed to read destination {}", destination.display()))?
            .next()
            .is_some()
        {
            return Err(anyhow!(
                "project scaffold destination must be empty: {}",
                destination.display()
            ));
        }
    } else {
        fs::create_dir_all(destination).with_context(|| {
            format!(
                "failed to create project scaffold destination {}",
                destination.display()
            )
        })?;
    }

    for file in minimal_2d_project_scaffold_files() {
        let relative = Path::new(file.path);
        let path = destination.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create scaffold directory {}", parent.display())
            })?;
        }
        fs::write(&path, file.contents)
            .with_context(|| format!("failed to write scaffold file {}", path.display()))?;
    }

    let manifest_path = destination.join(PROJECT_MANIFEST_FILE_NAME);
    let manifest = ProjectManifest::from_path(&manifest_path)?;
    let report = manifest.validate_references(destination)?;
    Seed::from_path(destination.join("seeds/platformer.yaml"))?;
    read_scene(destination.join("scenes/main.scene.json"))?;
    Ok(report)
}

const ROUTE_ATTEMPT_EVIDENCE_SCHEMA_VERSION: &str = "route-attempt-evidence-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RouteAttemptEvidenceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "attemptId")]
    pub attempt_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "objectiveId")]
    pub objective_id: String,
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "startState")]
    pub start_state: RouteAttemptStartState,
    #[serde(rename = "strategyId")]
    pub strategy_id: String,
    #[serde(rename = "strategyKind")]
    pub strategy_kind: RouteAttemptStrategyKind,
    #[serde(rename = "actionSequence")]
    pub action_sequence: Vec<RouteAttemptAction>,
    pub route: Vec<RouteAttemptRouteNode>,
    pub outcome: RouteAttemptOutcome,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blockers: Vec<RouteAttemptBlocker>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "budgetUsed")]
    pub budget_used: RouteAttemptBudgetUsed,
    #[serde(rename = "unsupportedReason", skip_serializing_if = "Option::is_none")]
    pub unsupported_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RouteAttemptStartState {
    #[serde(rename = "stateId")]
    pub state_id: String,
    #[serde(rename = "worldStateRef")]
    pub world_state_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RouteAttemptRouteNode {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub x: i32,
    pub y: i32,
    #[serde(rename = "evidenceRef", skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RouteAttemptBlocker {
    #[serde(rename = "blockerId")]
    pub blocker_id: String,
    pub reason: String,
    #[serde(rename = "evidenceRef", skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

impl RouteAttemptEvidenceArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: RouteAttemptEvidenceArtifact =
            serde_json::from_str(input).context("failed to parse Route Attempt Evidence JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ROUTE_ATTEMPT_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "route attempt evidence schemaVersion must be {ROUTE_ATTEMPT_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("route attempt attemptId", &self.attempt_id)?;
        validate_path_component("route attempt runId", &self.run_id)?;
        validate_path_component("route attempt objectiveId", &self.objective_id)?;
        validate_path_component("route attempt scenarioId", &self.scenario_id)?;
        self.start_state.validate()?;
        validate_path_component("route attempt strategyId", &self.strategy_id)?;
        if self.action_sequence.is_empty() {
            return Err(anyhow!("route attempt actionSequence must not be empty"));
        }
        let mut action_ids = BTreeSet::new();
        for action in &self.action_sequence {
            action.validate()?;
            if !action_ids.insert(action.action_id.as_str()) {
                return Err(anyhow!(
                    "duplicate route attempt actionId: {}",
                    action.action_id
                ));
            }
        }
        if self.route.is_empty() {
            return Err(anyhow!("route attempt route must not be empty"));
        }
        let mut node_ids = BTreeSet::new();
        for node in &self.route {
            node.validate()?;
            if !node_ids.insert(node.node_id.as_str()) {
                return Err(anyhow!(
                    "duplicate route attempt route nodeId: {}",
                    node.node_id
                ));
            }
        }
        if self.evidence_refs.is_empty() {
            return Err(anyhow!("route attempt evidenceRefs must not be empty"));
        }
        for reference in &self.evidence_refs {
            validate_evidence_artifact_path(reference)?;
        }
        self.budget_used.validate()?;
        self.validate_strategy_action_support()?;
        self.validate_action_sequence_order()?;
        if self.budget_used.actions_used != self.action_sequence.len() as u32 {
            return Err(anyhow!(
                "route attempt budgetUsed.actionsUsed must match actionSequence length"
            ));
        }
        if self.budget_used.route_nodes_used != self.route.len() as u32 {
            return Err(anyhow!(
                "route attempt budgetUsed.routeNodesUsed must match route length"
            ));
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        match self.outcome {
            RouteAttemptOutcome::Blocked if self.blockers.is_empty() => {
                Err(anyhow!("route attempt blocked outcome requires blockers"))
            }
            RouteAttemptOutcome::Unsupported if self.unsupported_reason.is_none() => Err(anyhow!(
                "route attempt unsupported outcome requires unsupportedReason"
            )),
            RouteAttemptOutcome::Passed
                if !self.blockers.is_empty() || self.unsupported_reason.is_some() =>
            {
                Err(anyhow!(
                    "route attempt passed outcome must not include blockers or unsupportedReason"
                ))
            }
            _ => {
                if let Some(reason) = &self.unsupported_reason {
                    require_bounded_display_text("route attempt unsupportedReason", reason)?;
                }
                for guardrail in &self.guardrails {
                    require_bounded_display_text("route attempt guardrail", guardrail)?;
                }
                Ok(())
            }
        }
    }

    fn validate_strategy_action_support(&self) -> Result<()> {
        if self.strategy_kind == RouteAttemptStrategyKind::GraphSearch
            && self
                .action_sequence
                .iter()
                .any(|action| matches!(action.kind, RouteAttemptActionKind::Interact))
            && self.outcome != RouteAttemptOutcome::Unsupported
        {
            return Err(anyhow!(
                "route attempt graph_search strategy must mark unsupported mechanics with unsupported outcome"
            ));
        }
        Ok(())
    }

    fn validate_action_sequence_order(&self) -> Result<()> {
        let mut last_frame = None;
        for action in &self.action_sequence {
            if let Some(frame) = action.frame {
                if let Some(previous) = last_frame {
                    if frame < previous {
                        return Err(anyhow!(
                            "route attempt actionSequence frames must be nondecreasing"
                        ));
                    }
                }
                last_frame = Some(frame);
            }
        }
        Ok(())
    }
}

pub fn validate_route_attempt_evidence_refs(
    run_dir: impl AsRef<Path>,
    attempt: &RouteAttemptEvidenceArtifact,
) -> Result<()> {
    let run_dir = run_dir.as_ref();
    attempt.validate()?;
    let index = read_evidence_index(run_dir)?;
    let indexed_paths = index
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    let mut references = Vec::new();
    references.push(attempt.start_state.world_state_ref.as_str());
    references.extend(attempt.evidence_refs.iter().map(String::as_str));
    references.extend(
        attempt
            .route
            .iter()
            .filter_map(|node| node.evidence_ref.as_deref()),
    );
    references.extend(
        attempt
            .blockers
            .iter()
            .filter_map(|blocker| blocker.evidence_ref.as_deref()),
    );
    references.sort();
    references.dedup();

    let mut objective_seen = false;
    for reference in references {
        if !indexed_paths.contains(reference) {
            return Err(anyhow!(
                "route attempt reference is missing from evidence index: {reference}"
            ));
        }
        let value = read_json_value(run_dir.join(reference))
            .with_context(|| format!("route attempt reference is unreadable: {reference}"))?;
        validate_route_attempt_linked_reference_freshness(attempt, reference, &value)?;
        if json_string(&value, "objectiveId")
            .or_else(|| json_string(&value, "objective_id"))
            .as_deref()
            == Some(attempt.objective_id.as_str())
        {
            objective_seen = true;
        }
    }
    if !objective_seen {
        return Err(anyhow!(
            "route attempt objectiveId must be backed by indexed evidence: {}",
            attempt.objective_id
        ));
    }
    Ok(())
}

fn validate_route_attempt_linked_reference_freshness(
    attempt: &RouteAttemptEvidenceArtifact,
    reference: &str,
    value: &serde_json::Value,
) -> Result<()> {
    match json_string(value, "runId").or_else(|| json_string(value, "run_id")) {
        Some(run_id) if run_id == attempt.run_id => {}
        Some(run_id) => {
            return Err(anyhow!(
                "route attempt reference is stale for runId {run_id}; expected {} at {reference}",
                attempt.run_id
            ));
        }
        None => {
            return Err(anyhow!(
                "route attempt reference is missing a runId/run_id: {reference}"
            ));
        }
    }

    if let Some(scenario_id) =
        json_string(value, "scenarioId").or_else(|| json_string(value, "scenario_id"))
    {
        if scenario_id != attempt.scenario_id {
            return Err(anyhow!(
                "route attempt reference scenarioId drift at {reference}: {scenario_id} != {}",
                attempt.scenario_id
            ));
        }
    }

    if reference == attempt.start_state.world_state_ref {
        match json_string(value, "stateId").or_else(|| json_string(value, "state_id")) {
            Some(state_id) if state_id == attempt.start_state.state_id => {}
            Some(state_id) => {
                return Err(anyhow!(
                    "route attempt startState is stale at {reference}: {state_id} != {}",
                    attempt.start_state.state_id
                ));
            }
            None => {
                return Err(anyhow!(
                    "route attempt startState reference is missing a stateId/state_id: {reference}"
                ));
            }
        }
    }
    Ok(())
}

impl RouteAttemptStartState {
    fn validate(&self) -> Result<()> {
        validate_path_component("route attempt startState.stateId", &self.state_id)?;
        validate_evidence_artifact_path(&self.world_state_ref)
    }
}

impl RouteAttemptRouteNode {
    fn validate(&self) -> Result<()> {
        validate_path_component("route attempt route.nodeId", &self.node_id)?;
        if let Some(reference) = &self.evidence_ref {
            validate_evidence_artifact_path(reference)?;
        }
        Ok(())
    }
}

impl RouteAttemptBlocker {
    fn validate(&self) -> Result<()> {
        validate_path_component("route attempt blockers.blockerId", &self.blocker_id)?;
        require_bounded_display_text("route attempt blockers.reason", &self.reason)?;
        if let Some(reference) = &self.evidence_ref {
            validate_evidence_artifact_path(reference)?;
        }
        Ok(())
    }
}

const QA_SCENARIO_CANDIDATE_SCHEMA_VERSION: &str = "qa-scenario-candidate-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaScenarioCandidateArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "candidateId")]
    pub candidate_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "sourceRisk")]
    pub source_risk: QaScenarioSourceRisk,
    #[serde(rename = "targetObjective")]
    pub target_objective: QaScenarioTargetObjective,
    #[serde(rename = "sourceRefs")]
    pub source_refs: QaScenarioSourceRefs,
    #[serde(rename = "inputStrategy")]
    pub input_strategy: QaScenarioInputStrategy,
    pub assertions: Vec<QaScenarioCandidateAssertion>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<QaScenarioExpectedEvidence>,
    pub budget: QaScenarioCandidateBudget,
    pub priority: QaScenarioCandidatePriority,
    pub status: QaScenarioCandidateStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaScenarioSourceRisk {
    #[serde(rename = "riskId")]
    pub risk_id: String,
    pub description: String,
    #[serde(
        rename = "evidenceRefs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaScenarioExpectedEvidence {
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "artifactKind")]
    pub artifact_kind: QaScenarioExpectedEvidenceKind,
    #[serde(rename = "pathHint")]
    pub path_hint: String,
}

impl QaScenarioCandidateArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: QaScenarioCandidateArtifact =
            serde_json::from_str(input).context("failed to parse QA Scenario Candidate JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_SCENARIO_CANDIDATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "qa scenario candidate schemaVersion must be {QA_SCENARIO_CANDIDATE_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("qa scenario candidate candidateId", &self.candidate_id)?;
        validate_path_component("qa scenario candidate runId", &self.run_id)?;
        self.source_risk.validate()?;
        self.target_objective.validate()?;
        self.source_refs.validate()?;
        self.input_strategy.validate()?;
        if self.assertions.is_empty() {
            return Err(anyhow!(
                "qa scenario candidate assertions must not be empty"
            ));
        }
        let mut assertion_ids = BTreeSet::new();
        for assertion in &self.assertions {
            assertion.validate()?;
            if !assertion_ids.insert(assertion.assertion_id.as_str()) {
                return Err(anyhow!(
                    "duplicate qa scenario candidate assertionId: {}",
                    assertion.assertion_id
                ));
            }
        }
        if self.expected_evidence.is_empty() {
            return Err(anyhow!(
                "qa scenario candidate expectedEvidence must not be empty"
            ));
        }
        let mut evidence_ids = BTreeSet::new();
        let mut evidence_paths = BTreeSet::new();
        for evidence in &self.expected_evidence {
            evidence.validate(&self.candidate_id)?;
            if !evidence_ids.insert(evidence.evidence_id.as_str()) {
                return Err(anyhow!(
                    "duplicate qa scenario candidate evidenceId: {}",
                    evidence.evidence_id
                ));
            }
            if !evidence_paths.insert(evidence.path_hint.as_str()) {
                return Err(anyhow!(
                    "duplicate qa scenario candidate expectedEvidence.pathHint: {}",
                    evidence.path_hint
                ));
            }
        }
        self.validate_assertion_evidence_coverage()?;
        self.budget.validate()?;
        if self.priority == QaScenarioCandidatePriority::High
            && self.budget.max_runs == 1
            && self.input_strategy.kind == QaScenarioInputStrategyKind::ManualReview
        {
            return Err(anyhow!(
                "qa scenario candidate high priority manual-review candidates require more than one bounded run or a non-manual strategy"
            ));
        }
        for reason in &self.blocked_reasons {
            require_bounded_display_text("qa scenario candidate blockedReasons", reason)?;
        }
        match self.status {
            QaScenarioCandidateStatus::Blocked if self.blocked_reasons.is_empty() => Err(anyhow!(
                "qa scenario candidate blocked status requires blockedReasons"
            )),
            QaScenarioCandidateStatus::Proposed if !self.blocked_reasons.is_empty() => Err(
                anyhow!("qa scenario candidate proposed status must not include blockedReasons"),
            ),
            _ => {
                for guardrail in &self.guardrails {
                    require_bounded_display_text("qa scenario candidate guardrail", guardrail)?;
                }
                Ok(())
            }
        }
    }

    fn validate_assertion_evidence_coverage(&self) -> Result<()> {
        let evidence_kinds = self
            .expected_evidence
            .iter()
            .map(|evidence| evidence.artifact_kind)
            .collect::<BTreeSet<_>>();
        for assertion in &self.assertions {
            let required = match assertion.kind {
                QaScenarioAssertionKind::WorldState => QaScenarioExpectedEvidenceKind::WorldState,
                QaScenarioAssertionKind::FrameStats => QaScenarioExpectedEvidenceKind::FrameStats,
                QaScenarioAssertionKind::RuntimeEvents => {
                    QaScenarioExpectedEvidenceKind::RuntimeProbe
                }
                QaScenarioAssertionKind::ConsoleErrors => {
                    QaScenarioExpectedEvidenceKind::ConsoleLog
                }
                QaScenarioAssertionKind::PerformanceMetrics => {
                    QaScenarioExpectedEvidenceKind::FrameStats
                }
                QaScenarioAssertionKind::CollisionEvidence => {
                    QaScenarioExpectedEvidenceKind::WorldState
                }
                QaScenarioAssertionKind::VisualCheckpoint => {
                    QaScenarioExpectedEvidenceKind::CandidateSummary
                }
            };
            if !evidence_kinds.contains(&required) {
                return Err(anyhow!(
                    "qa scenario candidate assertion {} requires expectedEvidence artifactKind {:?}",
                    assertion.assertion_id,
                    required
                ));
            }
        }
        Ok(())
    }
}

pub fn validate_qa_scenario_candidate_refs(
    repo_root: impl AsRef<Path>,
    run_dir: impl AsRef<Path>,
    candidate: &QaScenarioCandidateArtifact,
) -> Result<()> {
    let repo_root = repo_root.as_ref();
    let run_dir = run_dir.as_ref();
    candidate.validate()?;
    let seed_path = repo_root.join(&candidate.source_refs.seed_ref);
    if !seed_path.is_file() {
        return Err(anyhow!(
            "qa scenario candidate sourceRefs.seedRef is missing: {}",
            candidate.source_refs.seed_ref
        ));
    }
    let pack_path = repo_root.join(&candidate.source_refs.scenario_pack_ref);
    let pack = ScenarioPack::from_path(&pack_path)?;
    if pack.seed != candidate.source_refs.seed_ref {
        return Err(anyhow!(
            "qa scenario candidate sourceRefs.seedRef is stale for scenarioPackRef: {} != {}",
            candidate.source_refs.seed_ref,
            pack.seed
        ));
    }
    let scenario_ids = pack
        .ordered_scenario_ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    for scenario_id in &candidate.source_refs.source_scenario_ids {
        if !scenario_ids.contains(scenario_id) {
            return Err(anyhow!(
                "qa scenario candidate sourceScenarioIds contains stale scenario ref: {scenario_id}"
            ));
        }
    }
    let evidence_index = read_evidence_index(run_dir)?;
    let indexed_paths = evidence_index
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    for reference in &candidate.source_risk.evidence_refs {
        if !indexed_paths.contains(reference.as_str()) {
            return Err(anyhow!(
                "qa scenario candidate sourceRisk.evidenceRefs is missing from evidence index: {reference}"
            ));
        }
        let value = read_json_value(run_dir.join(reference)).with_context(|| {
            format!("qa scenario candidate sourceRisk.evidenceRef is unreadable: {reference}")
        })?;
        if let Some(run_id) = json_string(&value, "runId").or_else(|| json_string(&value, "run_id"))
        {
            if run_id != candidate.run_id {
                return Err(anyhow!(
                    "qa scenario candidate sourceRisk.evidenceRefs has stale runId {run_id}; expected {}",
                    candidate.run_id
                ));
            }
        } else {
            return Err(anyhow!(
                "qa scenario candidate sourceRisk.evidenceRefs is missing a runId/run_id: {reference}"
            ));
        }
    }
    Ok(())
}

impl QaScenarioSourceRisk {
    fn validate(&self) -> Result<()> {
        validate_path_component("qa scenario candidate sourceRisk.riskId", &self.risk_id)?;
        require_bounded_display_text(
            "qa scenario candidate sourceRisk.description",
            &self.description,
        )?;
        for reference in &self.evidence_refs {
            validate_evidence_artifact_path(reference)?;
        }
        Ok(())
    }
}

impl QaScenarioExpectedEvidence {
    fn validate(&self, candidate_id: &str) -> Result<()> {
        validate_path_component(
            "qa scenario candidate expectedEvidence.evidenceId",
            &self.evidence_id,
        )?;
        validate_evidence_artifact_path(&self.path_hint)?;
        let expected_prefix = format!("evidence/scenarios/{candidate_id}/");
        if !self.path_hint.starts_with(&expected_prefix) {
            return Err(anyhow!(
                "qa scenario candidate expectedEvidence.pathHint must stay under {expected_prefix}"
            ));
        }
        if !self.path_hint.ends_with(".json") {
            return Err(anyhow!(
                "qa scenario candidate expectedEvidence.pathHint must be JSON evidence"
            ));
        }
        let expected_fragment = match self.artifact_kind {
            QaScenarioExpectedEvidenceKind::ScenarioResult => "scenario-result",
            QaScenarioExpectedEvidenceKind::ScenarioInputReplay => "input-replay",
            QaScenarioExpectedEvidenceKind::WorldState => "world-state",
            QaScenarioExpectedEvidenceKind::FrameStats => "frame-stats",
            QaScenarioExpectedEvidenceKind::RuntimeProbe => "runtime-probe",
            QaScenarioExpectedEvidenceKind::ConsoleLog => "console-log",
            QaScenarioExpectedEvidenceKind::CandidateSummary => "candidate-summary",
        };
        if !self.path_hint.contains(expected_fragment) {
            return Err(anyhow!(
                "qa scenario candidate expectedEvidence.pathHint must match artifactKind {expected_fragment}"
            ));
        }
        Ok(())
    }
}

const SCENE_GENERATION_PLAN_SCHEMA_VERSION: &str = "scene-generation-plan-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SceneGenerationPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "targetTilemapRef", skip_serializing_if = "Option::is_none")]
    pub target_tilemap_ref: Option<String>,
    #[serde(rename = "previewSummary")]
    pub preview_summary: String,
    #[serde(rename = "proposedZones")]
    pub proposed_zones: Vec<SceneGenerationPlanZone>,
    #[serde(rename = "placementStrategy")]
    pub placement_strategy: SceneGenerationPlacementStrategy,
    #[serde(rename = "requiredAssets")]
    pub required_assets: Vec<LevelIntentRef>,
    #[serde(rename = "requiredEntities")]
    pub required_entities: Vec<LevelIntentRef>,
    #[serde(rename = "scenarioChecksToGenerate")]
    pub scenario_checks_to_generate: Vec<SceneGenerationScenarioCheck>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<SceneGenerationExpectedEvidence>,
    #[serde(rename = "targetHashes")]
    pub target_hashes: Vec<SceneGenerationTargetHash>,
    pub status: SceneGenerationPlanStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl SceneGenerationPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: SceneGenerationPlanArtifact =
            serde_json::from_str(input).context("failed to parse Scene Generation Plan JSON")?;
        artifact.validate_shape()?;
        Ok(artifact)
    }

    pub fn validate_against_intent(&self, intent: &LevelIntentArtifact) -> Result<()> {
        self.validate_shape()?;
        intent.validate()?;
        if self.intent_id != intent.intent_id {
            return Err(anyhow!(
                "scene generation plan intentId does not match level intent: {} != {}",
                self.intent_id,
                intent.intent_id
            ));
        }
        validate_scene_generation_refs_against_intent(
            "scene generation plan requiredAssets",
            &self.required_assets,
            &intent.allowed_assets,
        )?;
        validate_scene_generation_refs_against_intent(
            "scene generation plan requiredEntities",
            &self.required_entities,
            &intent.allowed_entities,
        )?;
        let objective_ids = intent
            .objectives
            .iter()
            .map(|objective| objective.objective_id.as_str())
            .collect::<BTreeSet<_>>();
        for zone in &self.proposed_zones {
            for objective_ref in &zone.objective_refs {
                if !objective_ids.contains(objective_ref.as_str()) {
                    return Err(anyhow!(
                        "scene generation plan zone {} references missing intent objective: {}",
                        zone.zone_id,
                        objective_ref
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<()> {
        if self.schema_version != SCENE_GENERATION_PLAN_SCHEMA_VERSION {
            return Err(anyhow!(
                "scene generation plan schemaVersion must be {SCENE_GENERATION_PLAN_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("scene generation plan planId", &self.plan_id)?;
        validate_path_component("scene generation plan intentId", &self.intent_id)?;
        validate_repo_relative_source_ref(
            "scene generation plan targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "scene generation plan targetSceneRef must point to a .scene.json source fixture"
            ));
        }
        if let Some(tilemap_ref) = &self.target_tilemap_ref {
            validate_repo_relative_source_ref(
                "scene generation plan targetTilemapRef",
                tilemap_ref,
            )?;
        }
        require_bounded_display_text(
            "scene generation plan previewSummary",
            &self.preview_summary,
        )?;
        validate_scene_generation_zones(&self.proposed_zones)?;
        self.placement_strategy.validate(&self.proposed_zones)?;
        validate_level_intent_refs(
            "scene generation plan requiredAssets",
            &self.required_assets,
            false,
        )?;
        validate_level_intent_refs(
            "scene generation plan requiredEntities",
            &self.required_entities,
            false,
        )?;
        validate_scene_generation_checks(&self.scenario_checks_to_generate)?;
        validate_scene_generation_expected_evidence(&self.plan_id, &self.expected_evidence)?;
        validate_scene_generation_target_hashes(
            &self.target_scene_ref,
            self.target_tilemap_ref.as_deref(),
            &self.target_hashes,
        )?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("scene generation plan blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("scene generation plan guardrails", guardrail)?;
        }
        match self.status {
            SceneGenerationPlanStatus::Blocked | SceneGenerationPlanStatus::Stale
                if self.blocked_reasons.is_empty() =>
            {
                Err(anyhow!(
                    "scene generation plan blocked or stale status requires blockedReasons"
                ))
            }
            SceneGenerationPlanStatus::Planned if !self.blocked_reasons.is_empty() => Err(anyhow!(
                "scene generation plan planned status must not include blockedReasons"
            )),
            _ => Ok(()),
        }
    }
}

pub fn scene_generation_plan_read_model_from_json_str(
    input: &str,
) -> Result<SceneGenerationPlanReadModel> {
    let plan = SceneGenerationPlanArtifact::from_json_str(input)?;
    Ok(scene_generation_plan_read_model(&plan))
}

pub fn scene_generation_plan_read_model(
    plan: &SceneGenerationPlanArtifact,
) -> SceneGenerationPlanReadModel {
    SceneGenerationPlanReadModel {
        schema_version: "scene-generation-plan-read-model-v1".to_string(),
        plan_id: plan.plan_id.clone(),
        intent_id: plan.intent_id.clone(),
        target_scene_ref: plan.target_scene_ref.clone(),
        target_tilemap_ref: plan.target_tilemap_ref.clone(),
        status: match plan.status {
            SceneGenerationPlanStatus::Planned => "planned",
            SceneGenerationPlanStatus::Stale => "stale",
            SceneGenerationPlanStatus::Blocked => "blocked",
        }
        .to_string(),
        zone_count: plan.proposed_zones.len(),
        scenario_check_count: plan.scenario_checks_to_generate.len(),
        expected_evidence_refs: plan
            .expected_evidence
            .iter()
            .map(|evidence| evidence.path_hint.clone())
            .collect(),
        target_hash_refs: plan
            .target_hashes
            .iter()
            .map(|target| target.target_ref.clone())
            .collect(),
        target_hash_count: plan.target_hashes.len(),
        blocked_reasons: plan.blocked_reasons.clone(),
        preview_summary: plan.preview_summary.clone(),
        boundary: "Read-only scene generation plan preview; advisory and untrusted, does not generate drafts, does not write trusted files, and grants no browser command bridge, auto-apply, auto-merge, or quality guarantee authority.".to_string(),
    }
}

fn validate_scene_generation_expected_evidence(
    plan_id: &str,
    values: &[SceneGenerationExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!(
            "scene generation plan expectedEvidence must not be empty"
        ));
    }
    let mut evidence_ids = BTreeSet::new();
    let mut path_hints = BTreeSet::new();
    let expected_prefix = format!("evidence/scene-generation-plans/{plan_id}/");
    for evidence in values {
        validate_path_component(
            "scene generation plan expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "scene generation plan expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !evidence_ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate scene generation plan expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !path_hints.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate scene generation plan expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const SPATIAL_LAYOUT_CONSTRAINT_SOLVER_SCHEMA_VERSION: &str = "spatial-layout-constraint-solver-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SpatialLayoutConstraintSolverArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "solverId")]
    pub solver_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "targetTilemapRef", skip_serializing_if = "Option::is_none")]
    pub target_tilemap_ref: Option<String>,
    pub grid: SpatialLayoutGrid,
    pub placements: Vec<SpatialLayoutPlacement>,
    pub constraints: Vec<SpatialLayoutConstraint>,
    #[serde(rename = "constraintResults")]
    pub constraint_results: Vec<SpatialLayoutConstraintResult>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<SpatialLayoutExpectedEvidence>,
    pub status: SpatialLayoutSolverStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl SpatialLayoutConstraintSolverArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: SpatialLayoutConstraintSolverArtifact = serde_json::from_str(input)
            .context("failed to parse Spatial Layout Constraint Solver JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != SPATIAL_LAYOUT_CONSTRAINT_SOLVER_SCHEMA_VERSION {
            return Err(anyhow!(
                "spatial layout constraint solver schemaVersion must be {SPATIAL_LAYOUT_CONSTRAINT_SOLVER_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("spatial layout solver solverId", &self.solver_id)?;
        validate_path_component("spatial layout solver planId", &self.plan_id)?;
        validate_repo_relative_source_ref(
            "spatial layout solver targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "spatial layout solver targetSceneRef must point to a .scene.json fixture"
            ));
        }
        if let Some(tilemap_ref) = &self.target_tilemap_ref {
            validate_repo_relative_source_ref(
                "spatial layout solver targetTilemapRef",
                tilemap_ref,
            )?;
        }
        self.grid.validate()?;
        validate_spatial_layout_placements(&self.grid, &self.placements)?;
        validate_spatial_layout_constraints(&self.placements, &self.constraints)?;
        validate_spatial_layout_expected_evidence(&self.solver_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("spatial layout solver blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("spatial layout solver guardrails", guardrail)?;
        }
        let evaluated =
            evaluate_spatial_layout_constraints(&self.grid, &self.placements, &self.constraints)?;
        validate_spatial_layout_results(&self.constraint_results, &evaluated)?;
        validate_spatial_layout_status(self.status, &self.constraint_results, &self.blocked_reasons)
    }
}

pub fn spatial_layout_constraint_solver_read_model_from_json_str(
    input: &str,
) -> Result<SpatialLayoutConstraintSolverReadModel> {
    let artifact = SpatialLayoutConstraintSolverArtifact::from_json_str(input)?;
    Ok(spatial_layout_constraint_solver_read_model(&artifact))
}

pub fn spatial_layout_constraint_solver_read_model(
    artifact: &SpatialLayoutConstraintSolverArtifact,
) -> SpatialLayoutConstraintSolverReadModel {
    let count_status = |status| {
        artifact
            .constraint_results
            .iter()
            .filter(|result| result.status == status)
            .count()
    };
    SpatialLayoutConstraintSolverReadModel {
        schema_version: "spatial-layout-constraint-solver-read-model-v1".to_string(),
        solver_id: artifact.solver_id.clone(),
        plan_id: artifact.plan_id.clone(),
        status: spatial_layout_solver_status_label(artifact.status).to_string(),
        grid_width: artifact.grid.width,
        grid_height: artifact.grid.height,
        placement_count: artifact.placements.len(),
        constraint_count: artifact.constraints.len(),
        satisfied_count: count_status(SpatialLayoutConstraintStatus::Satisfied),
        violated_count: count_status(SpatialLayoutConstraintStatus::Violated),
        unsupported_count: count_status(SpatialLayoutConstraintStatus::Unsupported),
        skipped_count: count_status(SpatialLayoutConstraintStatus::Skipped),
        expected_evidence_refs: artifact
            .expected_evidence
            .iter()
            .map(|evidence| evidence.path_hint.clone())
            .collect(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only spatial layout constraint evidence; deterministic local validation only, no hidden AI judgment, no scene generation, no trusted writes, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn validate_spatial_layout_expected_evidence(
    solver_id: &str,
    values: &[SpatialLayoutExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("spatial layout expectedEvidence must not be empty"));
    }
    let expected_prefix = format!("evidence/spatial-layout-constraints/{solver_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "spatial layout expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "spatial layout expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate spatial layout expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate spatial layout expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const TILEMAP_TERRAIN_GENERATION_DRAFT_SCHEMA_VERSION: &str = "tilemap-terrain-generation-draft-v1";
const TILEMAP_TERRAIN_GENERATION_DRAFT_READ_MODEL_SCHEMA_VERSION: &str =
    "tilemap-terrain-generation-draft-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TilemapTerrainGenerationDraftArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "solverId")]
    pub solver_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "targetTilemapRef")]
    pub target_tilemap_ref: String,
    pub grid: TilemapTerrainDraftGrid,
    pub layers: Vec<TilemapTerrainDraftLayer>,
    #[serde(rename = "tilePlacements")]
    pub tile_placements: Vec<TilemapTerrainDraftTilePlacement>,
    #[serde(
        rename = "terrainRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terrain_regions: Vec<TilemapTerrainDraftRegion>,
    #[serde(
        rename = "collisionRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub collision_regions: Vec<TilemapTerrainDraftRegion>,
    #[serde(
        rename = "triggerRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trigger_regions: Vec<TilemapTerrainDraftTriggerRegion>,
    #[serde(rename = "beforeHash")]
    pub before_hash: String,
    #[serde(rename = "expectedAfterSummary")]
    pub expected_after_summary: String,
    #[serde(rename = "linkedConstraints")]
    pub linked_constraints: Vec<TilemapTerrainDraftConstraintLink>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<TilemapTerrainDraftExpectedEvidence>,
    pub status: TilemapTerrainDraftStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl TilemapTerrainGenerationDraftArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: TilemapTerrainGenerationDraftArtifact = serde_json::from_str(input)
            .context("failed to parse Tilemap Terrain Generation Draft JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != TILEMAP_TERRAIN_GENERATION_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "tilemap terrain generation draft schemaVersion must be {TILEMAP_TERRAIN_GENERATION_DRAFT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("tilemap terrain draft draftId", &self.draft_id)?;
        validate_path_component("tilemap terrain draft intentId", &self.intent_id)?;
        validate_path_component("tilemap terrain draft planId", &self.plan_id)?;
        validate_path_component("tilemap terrain draft solverId", &self.solver_id)?;
        validate_repo_relative_source_ref(
            "tilemap terrain draft targetSceneRef",
            &self.target_scene_ref,
        )?;
        validate_repo_relative_source_ref(
            "tilemap terrain draft targetTilemapRef",
            &self.target_tilemap_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "tilemap terrain draft targetSceneRef must point to a .scene.json fixture"
            ));
        }
        if !self.target_tilemap_ref.ends_with(".json") {
            return Err(anyhow!(
                "tilemap terrain draft targetTilemapRef must point to a JSON tilemap fixture"
            ));
        }
        self.grid.validate()?;
        validate_tilemap_terrain_layers(&self.layers)?;
        let layer_ids = self
            .layers
            .iter()
            .map(|layer| layer.layer_id.as_str())
            .collect::<BTreeSet<_>>();
        validate_tilemap_terrain_tile_placements(&self.grid, &layer_ids, &self.tile_placements)?;
        validate_tilemap_terrain_regions(
            "terrainRegions",
            &self.grid,
            &layer_ids,
            &self.terrain_regions,
        )?;
        validate_tilemap_terrain_regions(
            "collisionRegions",
            &self.grid,
            &layer_ids,
            &self.collision_regions,
        )?;
        validate_tilemap_terrain_trigger_regions(&self.grid, &layer_ids, &self.trigger_regions)?;
        parse_visual_edit_draft_hash("tilemap terrain draft beforeHash", &self.before_hash)?;
        require_bounded_display_text(
            "tilemap terrain draft expectedAfterSummary",
            &self.expected_after_summary,
        )?;
        validate_tilemap_terrain_constraint_links(&self.linked_constraints)?;
        validate_tilemap_terrain_expected_evidence(&self.draft_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("tilemap terrain draft blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("tilemap terrain draft guardrails", guardrail)?;
        }
        validate_tilemap_terrain_draft_status(
            self.status,
            &self.linked_constraints,
            &self.blocked_reasons,
        )
    }
}

pub fn tilemap_terrain_generation_draft_read_model_from_json_str(
    input: &str,
) -> Result<TilemapTerrainGenerationDraftReadModel> {
    let artifact = TilemapTerrainGenerationDraftArtifact::from_json_str(input)?;
    Ok(tilemap_terrain_generation_draft_read_model(&artifact))
}

pub fn tilemap_terrain_generation_draft_read_model(
    artifact: &TilemapTerrainGenerationDraftArtifact,
) -> TilemapTerrainGenerationDraftReadModel {
    let mut constraint_status_summary = BTreeMap::new();
    for constraint in &artifact.linked_constraints {
        let label = tilemap_terrain_constraint_status_label(constraint.status).to_string();
        *constraint_status_summary.entry(label).or_insert(0) += 1;
    }
    TilemapTerrainGenerationDraftReadModel {
        schema_version: TILEMAP_TERRAIN_GENERATION_DRAFT_READ_MODEL_SCHEMA_VERSION.to_string(),
        draft_id: artifact.draft_id.clone(),
        intent_id: artifact.intent_id.clone(),
        plan_id: artifact.plan_id.clone(),
        solver_id: artifact.solver_id.clone(),
        status: tilemap_terrain_draft_status_label(artifact.status).to_string(),
        target_tilemap_ref: artifact.target_tilemap_ref.clone(),
        grid_width: artifact.grid.width,
        grid_height: artifact.grid.height,
        layer_count: artifact.layers.len(),
        tile_placement_count: artifact.tile_placements.len(),
        terrain_region_count: artifact.terrain_regions.len(),
        collision_region_count: artifact.collision_regions.len(),
        trigger_region_count: artifact.trigger_regions.len(),
        constraint_status_summary,
        expected_evidence_refs: artifact
            .expected_evidence
            .iter()
            .map(|evidence| evidence.path_hint.clone())
            .collect(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only tilemap terrain generation draft preview; no tilemap file writes, no trusted apply, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn validate_tilemap_terrain_expected_evidence(
    draft_id: &str,
    values: &[TilemapTerrainDraftExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!(
            "tilemap terrain draft expectedEvidence must not be empty"
        ));
    }
    let expected_prefix = format!("evidence/tilemap-terrain-drafts/{draft_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "tilemap terrain draft expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "tilemap terrain draft expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate tilemap terrain draft expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate tilemap terrain draft expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const ENTITY_OBJECTIVE_PLACEMENT_DRAFT_SCHEMA_VERSION: &str =
    "entity-objective-encounter-placement-draft-v1";
const ENTITY_OBJECTIVE_PLACEMENT_DRAFT_READ_MODEL_SCHEMA_VERSION: &str =
    "entity-objective-encounter-placement-draft-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EntityObjectiveEncounterPlacementDraftArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "solverId")]
    pub solver_id: String,
    #[serde(rename = "tilemapDraftId")]
    pub tilemap_draft_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    pub grid: EntityObjectivePlacementGrid,
    pub placements: Vec<EntityObjectivePlacementDraftPlacement>,
    pub objectives: Vec<EntityObjectivePlacementDraftObjective>,
    #[serde(rename = "beforeHash")]
    pub before_hash: String,
    #[serde(rename = "expectedAfterSummary")]
    pub expected_after_summary: String,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<EntityObjectivePlacementDraftExpectedEvidence>,
    pub status: EntityObjectivePlacementDraftStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl EntityObjectiveEncounterPlacementDraftArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: EntityObjectiveEncounterPlacementDraftArtifact = serde_json::from_str(input)
            .context("failed to parse Entity Objective Encounter Placement Draft JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ENTITY_OBJECTIVE_PLACEMENT_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "entity objective placement draft schemaVersion must be {ENTITY_OBJECTIVE_PLACEMENT_DRAFT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("entity objective placement draft draftId", &self.draft_id)?;
        validate_path_component("entity objective placement draft intentId", &self.intent_id)?;
        validate_path_component("entity objective placement draft planId", &self.plan_id)?;
        validate_path_component("entity objective placement draft solverId", &self.solver_id)?;
        validate_path_component(
            "entity objective placement draft tilemapDraftId",
            &self.tilemap_draft_id,
        )?;
        validate_repo_relative_source_ref(
            "entity objective placement draft targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "entity objective placement draft targetSceneRef must point to a .scene.json fixture"
            ));
        }
        self.grid.validate()?;
        let placement_ids = validate_entity_objective_placements(&self.grid, &self.placements)?;
        validate_entity_objective_objectives(&placement_ids, &self.objectives)?;
        parse_visual_edit_draft_hash(
            "entity objective placement draft beforeHash",
            &self.before_hash,
        )?;
        require_bounded_display_text(
            "entity objective placement draft expectedAfterSummary",
            &self.expected_after_summary,
        )?;
        validate_entity_objective_expected_evidence(&self.draft_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text(
                "entity objective placement draft blockedReasons",
                reason,
            )?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("entity objective placement draft guardrails", guardrail)?;
        }
        validate_entity_objective_status(self.status, &self.placements, &self.blocked_reasons)
    }
}

pub fn entity_objective_encounter_placement_draft_read_model_from_json_str(
    input: &str,
) -> Result<EntityObjectiveEncounterPlacementDraftReadModel> {
    let artifact = EntityObjectiveEncounterPlacementDraftArtifact::from_json_str(input)?;
    Ok(entity_objective_encounter_placement_draft_read_model(
        &artifact,
    ))
}

pub fn entity_objective_encounter_placement_draft_read_model(
    artifact: &EntityObjectiveEncounterPlacementDraftArtifact,
) -> EntityObjectiveEncounterPlacementDraftReadModel {
    let encounter_groups = artifact
        .placements
        .iter()
        .filter_map(|placement| placement.encounter_group_id.as_deref())
        .collect::<BTreeSet<_>>();
    let behavior_link_count = artifact
        .placements
        .iter()
        .map(|placement| placement.behavior_refs.len())
        .sum();
    EntityObjectiveEncounterPlacementDraftReadModel {
        schema_version: ENTITY_OBJECTIVE_PLACEMENT_DRAFT_READ_MODEL_SCHEMA_VERSION.to_string(),
        draft_id: artifact.draft_id.clone(),
        status: entity_objective_status_label(artifact.status).to_string(),
        target_scene_ref: artifact.target_scene_ref.clone(),
        placement_count: artifact.placements.len(),
        objective_count: artifact.objectives.len(),
        encounter_group_count: encounter_groups.len(),
        behavior_link_count,
        expected_evidence_refs: artifact
            .expected_evidence
            .iter()
            .map(|evidence| evidence.path_hint.clone())
            .collect(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only entity objective encounter placement draft preview; no scene writes, no trusted apply, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn validate_entity_objective_expected_evidence(
    draft_id: &str,
    values: &[EntityObjectivePlacementDraftExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!(
            "entity objective placement draft expectedEvidence must not be empty"
        ));
    }
    let expected_prefix = format!("evidence/entity-objective-placement-drafts/{draft_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "entity objective placement draft expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "entity objective placement draft expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate entity objective placement draft expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate entity objective placement draft expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const REACHABILITY_PATHING_EVIDENCE_SCHEMA_VERSION: &str = "reachability-pathing-evidence-v1";
const REACHABILITY_PATHING_READ_MODEL_SCHEMA_VERSION: &str =
    "reachability-pathing-evidence-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReachabilityPathingEvidenceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "tilemapDraftId")]
    pub tilemap_draft_id: String,
    #[serde(rename = "placementDraftId")]
    pub placement_draft_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    pub grid: ReachabilityGrid,
    pub cells: Vec<ReachabilityCell>,
    pub queries: Vec<ReachabilityQuery>,
    pub results: Vec<ReachabilityQueryResult>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<ReachabilityExpectedEvidence>,
    pub status: ReachabilityEvidenceStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl ReachabilityPathingEvidenceArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: ReachabilityPathingEvidenceArtifact = serde_json::from_str(input)
            .context("failed to parse Reachability Pathing Evidence JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != REACHABILITY_PATHING_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "reachability pathing evidence schemaVersion must be {REACHABILITY_PATHING_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("reachability evidence evidenceId", &self.evidence_id)?;
        validate_path_component("reachability evidence intentId", &self.intent_id)?;
        validate_path_component("reachability evidence planId", &self.plan_id)?;
        validate_path_component(
            "reachability evidence tilemapDraftId",
            &self.tilemap_draft_id,
        )?;
        validate_path_component(
            "reachability evidence placementDraftId",
            &self.placement_draft_id,
        )?;
        validate_repo_relative_source_ref(
            "reachability evidence targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "reachability evidence targetSceneRef must point to a .scene.json fixture"
            ));
        }
        self.grid.validate()?;
        let cell_map = validate_reachability_cells(&self.grid, &self.cells)?;
        validate_reachability_queries(&self.grid, &self.queries)?;
        validate_reachability_expected_evidence(&self.evidence_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("reachability evidence blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("reachability evidence guardrails", guardrail)?;
        }
        let expected = evaluate_reachability_queries(&self.grid, &cell_map, &self.queries)?;
        validate_reachability_results(&self.results, &expected)?;
        validate_reachability_status(self.status, &self.results, &self.blocked_reasons)
    }
}

pub fn reachability_pathing_evidence_read_model_from_json_str(
    input: &str,
) -> Result<ReachabilityPathingEvidenceReadModel> {
    let artifact = ReachabilityPathingEvidenceArtifact::from_json_str(input)?;
    Ok(reachability_pathing_evidence_read_model(&artifact))
}

pub fn reachability_pathing_evidence_read_model(
    artifact: &ReachabilityPathingEvidenceArtifact,
) -> ReachabilityPathingEvidenceReadModel {
    let count_status = |status| {
        artifact
            .results
            .iter()
            .filter(|result| result.status == status)
            .count()
    };
    ReachabilityPathingEvidenceReadModel {
        schema_version: REACHABILITY_PATHING_READ_MODEL_SCHEMA_VERSION.to_string(),
        evidence_id: artifact.evidence_id.clone(),
        status: reachability_evidence_status_label(artifact.status).to_string(),
        query_count: artifact.queries.len(),
        reachable_count: count_status(ReachabilityQueryStatus::Reachable),
        unreachable_count: count_status(ReachabilityQueryStatus::Unreachable),
        unsupported_count: count_status(ReachabilityQueryStatus::Unsupported),
        blocked_count: count_status(ReachabilityQueryStatus::Blocked),
        expected_evidence_refs: artifact
            .expected_evidence
            .iter()
            .map(|evidence| evidence.path_hint.clone())
            .collect(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only reachability/pathing evidence; bounded local graph analysis only, unsupported movement is explicit, no gameplay quality guarantee, no scene writes, no trusted apply, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn validate_reachability_expected_evidence(
    evidence_id: &str,
    values: &[ReachabilityExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("reachability expectedEvidence must not be empty"));
    }
    let expected_prefix = format!("evidence/reachability-pathing/{evidence_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "reachability expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "reachability expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate reachability expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate reachability expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const OBJECTIVE_COMPLETION_PROOF_SCHEMA_VERSION: &str = "objective-completion-proof-v1";
const OBJECTIVE_COMPLETION_PROOF_READ_MODEL_SCHEMA_VERSION: &str =
    "objective-completion-proof-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ObjectiveCompletionProofArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "proofId")]
    pub proof_id: String,
    #[serde(rename = "objectiveId")]
    pub objective_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "placementDraftId")]
    pub placement_draft_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "reachabilityEvidenceRef")]
    pub reachability_evidence_ref: String,
    #[serde(rename = "scenarioRef")]
    pub scenario_ref: String,
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "verdictRef")]
    pub verdict_ref: String,
    #[serde(rename = "behaviorEvidenceRefs")]
    pub behavior_evidence_refs: Vec<String>,
    pub route: Vec<ObjectiveProofRouteStep>,
    #[serde(rename = "requiredActions")]
    pub required_actions: Vec<ObjectiveProofAction>,
    #[serde(rename = "requiredEvents")]
    pub required_events: Vec<String>,
    #[serde(rename = "expectedFlags")]
    pub expected_flags: Vec<ObjectiveProofFlag>,
    #[serde(rename = "expectedStateTransitions")]
    pub expected_state_transitions: Vec<ObjectiveProofStateTransition>,
    #[serde(rename = "observedEvents")]
    pub observed_events: Vec<String>,
    #[serde(rename = "observedFlags")]
    pub observed_flags: Vec<ObjectiveProofFlag>,
    #[serde(rename = "observedStateTransitions")]
    pub observed_state_transitions: Vec<ObjectiveProofStateTransition>,
    pub result: ObjectiveCompletionProofResult,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<ObjectiveCompletionExpectedEvidence>,
    pub status: ObjectiveCompletionProofStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl ObjectiveCompletionProofArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: ObjectiveCompletionProofArtifact = serde_json::from_str(input)
            .context("failed to parse Objective Completion Proof JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != OBJECTIVE_COMPLETION_PROOF_SCHEMA_VERSION {
            return Err(anyhow!(
                "objective completion proof schemaVersion must be {OBJECTIVE_COMPLETION_PROOF_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("objective completion proof proofId", &self.proof_id)?;
        validate_path_component("objective completion proof objectiveId", &self.objective_id)?;
        validate_path_component("objective completion proof intentId", &self.intent_id)?;
        validate_path_component("objective completion proof planId", &self.plan_id)?;
        validate_path_component(
            "objective completion proof placementDraftId",
            &self.placement_draft_id,
        )?;
        validate_repo_relative_source_ref(
            "objective completion proof targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "objective completion proof targetSceneRef must point to a .scene.json fixture"
            ));
        }
        validate_evidence_artifact_path(&self.reachability_evidence_ref)?;
        validate_repo_relative_source_ref(
            "objective completion proof scenarioRef",
            &self.scenario_ref,
        )?;
        validate_path_component("objective completion proof scenarioId", &self.scenario_id)?;
        validate_evidence_artifact_path(&self.verdict_ref)?;
        validate_objective_behavior_refs(&self.behavior_evidence_refs)?;
        validate_objective_route(&self.route)?;
        validate_objective_actions(&self.required_actions)?;
        validate_objective_text_refs("objective completion requiredEvents", &self.required_events)?;
        validate_objective_flags("objective completion expectedFlags", &self.expected_flags)?;
        validate_objective_flags("objective completion observedFlags", &self.observed_flags)?;
        validate_objective_transitions(
            "objective completion expectedStateTransitions",
            &self.expected_state_transitions,
        )?;
        validate_objective_transitions(
            "objective completion observedStateTransitions",
            &self.observed_state_transitions,
        )?;
        validate_objective_text_refs("objective completion observedEvents", &self.observed_events)?;
        validate_objective_expected_evidence(&self.proof_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("objective completion blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("objective completion guardrails", guardrail)?;
        }
        let expected = evaluate_objective_completion(self)?;
        if self.result != expected {
            return Err(anyhow!("objective completion result drift"));
        }
        validate_objective_completion_status(self.status, &self.result, &self.blocked_reasons)
    }
}

pub fn objective_completion_proof_read_model_from_json_str(
    input: &str,
) -> Result<ObjectiveCompletionProofReadModel> {
    let artifact = ObjectiveCompletionProofArtifact::from_json_str(input)?;
    Ok(objective_completion_proof_read_model(&artifact))
}

pub fn objective_completion_proof_read_model(
    artifact: &ObjectiveCompletionProofArtifact,
) -> ObjectiveCompletionProofReadModel {
    let linked_evidence_refs = std::iter::once(artifact.reachability_evidence_ref.clone())
        .chain(std::iter::once(artifact.verdict_ref.clone()))
        .chain(artifact.behavior_evidence_refs.iter().cloned())
        .chain(
            artifact
                .expected_evidence
                .iter()
                .map(|evidence| evidence.path_hint.clone()),
        )
        .collect();
    ObjectiveCompletionProofReadModel {
        schema_version: OBJECTIVE_COMPLETION_PROOF_READ_MODEL_SCHEMA_VERSION.to_string(),
        proof_id: artifact.proof_id.clone(),
        objective_id: artifact.objective_id.clone(),
        status: objective_completion_proof_status_label(artifact.status).to_string(),
        result_status: objective_completion_result_status_label(artifact.result.status).to_string(),
        objective_complete: artifact.result.objective_complete,
        win_condition_met: artifact.result.win_condition_met,
        loss_condition_triggered: artifact.result.loss_condition_triggered,
        required_action_count: artifact.required_actions.len(),
        required_event_count: artifact.required_events.len(),
        missing_evidence_count: artifact.result.missing_events.len()
            + artifact.result.missing_transitions.len(),
        failed_flag_count: artifact.result.failed_flags.len(),
        linked_evidence_refs,
        journal_summary: format!(
            "Objective {} proof {} is {}; result {}.",
            artifact.objective_id,
            artifact.proof_id,
            objective_completion_proof_status_label(artifact.status),
            objective_completion_result_status_label(artifact.result.status)
        ),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only objective completion and win/loss proof evidence; local scenario, verdict, reachability, and behavior evidence only, unsupported mechanics are explicit, no subjective quality guarantee, no scene writes, no trusted apply, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn validate_objective_behavior_refs(refs: &[String]) -> Result<()> {
    if refs.is_empty() || refs.len() > MAX_OBJECTIVE_PROOF_ITEMS {
        return Err(anyhow!(
            "objective completion behaviorEvidenceRefs must contain between 1 and {MAX_OBJECTIVE_PROOF_ITEMS} refs"
        ));
    }
    let mut seen = BTreeSet::new();
    for evidence_ref in refs {
        validate_evidence_artifact_path(evidence_ref)?;
        if !seen.insert(evidence_ref.as_str()) {
            return Err(anyhow!(
                "duplicate objective completion behaviorEvidenceRefs: {evidence_ref}"
            ));
        }
    }
    Ok(())
}

fn validate_objective_expected_evidence(
    proof_id: &str,
    values: &[ObjectiveCompletionExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!(
            "objective completion expectedEvidence must not be empty"
        ));
    }
    let expected_prefix = format!("evidence/objective-completion/{proof_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "objective completion expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "objective completion expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate objective completion expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate objective completion expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

fn evaluate_objective_completion(
    artifact: &ObjectiveCompletionProofArtifact,
) -> Result<ObjectiveCompletionProofResult> {
    if artifact.status == ObjectiveCompletionProofStatus::Blocked {
        return Ok(objective_completion_result(
            ObjectiveCompletionProofResultStatus::Blocked,
            false,
            false,
            false,
            "objective proof is blocked and cannot certify completion",
        ));
    }

    let unsupported_reasons = artifact
        .required_actions
        .iter()
        .filter(|action| action.kind == ObjectiveProofActionKind::Unsupported)
        .map(|action| {
            action
                .unsupported_reason
                .clone()
                .unwrap_or_else(|| "objective mechanic is unsupported by proof v1".to_string())
        })
        .collect::<Vec<_>>();
    if !unsupported_reasons.is_empty() {
        let mut result = objective_completion_result(
            ObjectiveCompletionProofResultStatus::Unsupported,
            false,
            false,
            false,
            "objective proof contains unsupported mechanics",
        );
        result.unsupported_reasons = unsupported_reasons;
        return Ok(result);
    }

    let observed_events = artifact.observed_events.iter().collect::<BTreeSet<_>>();
    let missing_events = artifact
        .required_events
        .iter()
        .filter(|event| !observed_events.contains(event))
        .cloned()
        .collect::<Vec<_>>();
    let observed_flags = artifact
        .observed_flags
        .iter()
        .map(|flag| (flag.flag.as_str(), flag.value))
        .collect::<BTreeMap<_, _>>();
    let failed_flags = artifact
        .expected_flags
        .iter()
        .filter(|flag| observed_flags.get(flag.flag.as_str()) != Some(&flag.value))
        .map(|flag| flag.flag.clone())
        .collect::<Vec<_>>();
    let observed_transitions = artifact
        .observed_state_transitions
        .iter()
        .collect::<BTreeSet<_>>();
    let missing_transitions = artifact
        .expected_state_transitions
        .iter()
        .filter(|transition| !observed_transitions.contains(transition))
        .map(|transition| {
            format!(
                "{}:{}->{}",
                transition.state, transition.from, transition.to
            )
        })
        .collect::<Vec<_>>();

    if !missing_events.is_empty() || !missing_transitions.is_empty() {
        let mut result = objective_completion_result(
            ObjectiveCompletionProofResultStatus::MissingEvidence,
            false,
            false,
            false,
            "objective proof is missing required event or transition evidence",
        );
        result.missing_events = missing_events;
        result.failed_flags = failed_flags;
        result.missing_transitions = missing_transitions;
        return Ok(result);
    }
    if !failed_flags.is_empty() {
        let mut result = objective_completion_result(
            ObjectiveCompletionProofResultStatus::Failed,
            false,
            false,
            true,
            "objective proof observed a failed flag or loss condition",
        );
        result.failed_flags = failed_flags;
        return Ok(result);
    }
    Ok(objective_completion_result(
        ObjectiveCompletionProofResultStatus::Complete,
        true,
        true,
        false,
        "objective proof completed under scoped local evidence",
    ))
}

const DIFFICULTY_PACING_HEURISTIC_SCHEMA_VERSION: &str = "difficulty-pacing-heuristic-evidence-v1";
const DIFFICULTY_PACING_HEURISTIC_READ_MODEL_SCHEMA_VERSION: &str =
    "difficulty-pacing-heuristic-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DifficultyPacingHeuristicEvidenceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "placementDraftId")]
    pub placement_draft_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "reachabilityEvidenceRef")]
    pub reachability_evidence_ref: String,
    #[serde(rename = "objectiveProofRef")]
    pub objective_proof_ref: String,
    pub metrics: Vec<DifficultyHeuristicMetric>,
    pub warnings: Vec<DifficultyHeuristicWarning>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<DifficultyHeuristicExpectedEvidence>,
    pub status: DifficultyHeuristicEvidenceStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl DifficultyPacingHeuristicEvidenceArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: DifficultyPacingHeuristicEvidenceArtifact = serde_json::from_str(input)
            .context("failed to parse Difficulty Pacing Heuristic Evidence JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != DIFFICULTY_PACING_HEURISTIC_SCHEMA_VERSION {
            return Err(anyhow!(
                "difficulty pacing heuristic schemaVersion must be {DIFFICULTY_PACING_HEURISTIC_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("difficulty heuristic evidenceId", &self.evidence_id)?;
        validate_path_component("difficulty heuristic intentId", &self.intent_id)?;
        validate_path_component("difficulty heuristic planId", &self.plan_id)?;
        validate_path_component(
            "difficulty heuristic placementDraftId",
            &self.placement_draft_id,
        )?;
        validate_repo_relative_source_ref(
            "difficulty heuristic targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "difficulty heuristic targetSceneRef must point to a .scene.json fixture"
            ));
        }
        validate_evidence_artifact_path(&self.reachability_evidence_ref)?;
        validate_evidence_artifact_path(&self.objective_proof_ref)?;
        validate_difficulty_heuristic_metrics(&self.metrics)?;
        validate_difficulty_expected_evidence(&self.evidence_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("difficulty heuristic blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("difficulty heuristic guardrails", guardrail)?;
        }
        let expected_warnings = evaluate_difficulty_heuristics(&self.metrics)?;
        if self.warnings != expected_warnings {
            return Err(anyhow!("difficulty heuristic warning drift"));
        }
        validate_difficulty_status(self.status, &self.warnings, &self.blocked_reasons)
    }
}

pub fn difficulty_pacing_heuristic_read_model_from_json_str(
    input: &str,
) -> Result<DifficultyPacingHeuristicReadModel> {
    let artifact = DifficultyPacingHeuristicEvidenceArtifact::from_json_str(input)?;
    Ok(difficulty_pacing_heuristic_read_model(&artifact))
}

pub fn difficulty_pacing_heuristic_read_model(
    artifact: &DifficultyPacingHeuristicEvidenceArtifact,
) -> DifficultyPacingHeuristicReadModel {
    let count_warnings = |kind| {
        artifact
            .warnings
            .iter()
            .filter(|warning| warning.kind == kind)
            .count()
    };
    DifficultyPacingHeuristicReadModel {
        schema_version: DIFFICULTY_PACING_HEURISTIC_READ_MODEL_SCHEMA_VERSION.to_string(),
        evidence_id: artifact.evidence_id.clone(),
        status: difficulty_status_label(artifact.status).to_string(),
        metric_count: artifact.metrics.len(),
        warning_count: artifact.warnings.len(),
        missing_input_count: count_warnings(DifficultyHeuristicWarningKind::MissingInput),
        unsupported_count: count_warnings(DifficultyHeuristicWarningKind::Unsupported),
        malformed_input_count: count_warnings(DifficultyHeuristicWarningKind::MalformedInput),
        linked_evidence_refs: std::iter::once(artifact.reachability_evidence_ref.clone())
            .chain(std::iter::once(artifact.objective_proof_ref.clone()))
            .chain(
                artifact
                    .expected_evidence
                    .iter()
                    .map(|evidence| evidence.path_hint.clone()),
            )
            .collect(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only difficulty, pacing, and balance heuristic evidence; transparent local metrics only, not a fun score, not a quality guarantee, no scene writes, no trusted apply, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn validate_difficulty_expected_evidence(
    evidence_id: &str,
    values: &[DifficultyHeuristicExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!(
            "difficulty heuristic expectedEvidence must not be empty"
        ));
    }
    let expected_prefix = format!("evidence/difficulty-pacing/{evidence_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "difficulty heuristic expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "difficulty heuristic expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate difficulty heuristic expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate difficulty heuristic expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const LEVEL_VISUAL_SEMANTIC_DIFF_SCHEMA_VERSION: &str = "level-visual-semantic-diff-v1";
const LEVEL_VISUAL_SEMANTIC_DIFF_READ_MODEL_SCHEMA_VERSION: &str =
    "level-visual-semantic-diff-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LevelVisualSemanticDiffArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "diffId")]
    pub diff_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "beforeDraftRef")]
    pub before_draft_ref: String,
    #[serde(rename = "afterDraftRef")]
    pub after_draft_ref: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "reachabilityEvidenceRef")]
    pub reachability_evidence_ref: String,
    #[serde(rename = "objectiveProofRef")]
    pub objective_proof_ref: String,
    #[serde(rename = "heuristicEvidenceRef")]
    pub heuristic_evidence_ref: String,
    #[serde(
        rename = "transactionRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_ref: Option<String>,
    pub changes: Vec<LevelSemanticChange>,
    #[serde(rename = "expectedScenarioImpacts")]
    pub expected_scenario_impacts: Vec<LevelScenarioImpact>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<LevelDiffExpectedEvidence>,
    pub status: LevelDiffStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl LevelVisualSemanticDiffArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: LevelVisualSemanticDiffArtifact = serde_json::from_str(input)
            .context("failed to parse Level Visual Semantic Diff JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != LEVEL_VISUAL_SEMANTIC_DIFF_SCHEMA_VERSION {
            return Err(anyhow!(
                "level visual semantic diff schemaVersion must be {LEVEL_VISUAL_SEMANTIC_DIFF_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("level diff diffId", &self.diff_id)?;
        validate_path_component("level diff intentId", &self.intent_id)?;
        validate_path_component("level diff planId", &self.plan_id)?;
        validate_evidence_artifact_path(&self.before_draft_ref)?;
        validate_evidence_artifact_path(&self.after_draft_ref)?;
        validate_repo_relative_source_ref("level diff targetSceneRef", &self.target_scene_ref)?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "level diff targetSceneRef must point to a .scene.json fixture"
            ));
        }
        validate_evidence_artifact_path(&self.reachability_evidence_ref)?;
        validate_evidence_artifact_path(&self.objective_proof_ref)?;
        validate_evidence_artifact_path(&self.heuristic_evidence_ref)?;
        if let Some(transaction_ref) = &self.transaction_ref {
            validate_evidence_artifact_path(transaction_ref)?;
        }
        validate_level_diff_changes(&self.changes)?;
        validate_level_scenario_impacts(&self.expected_scenario_impacts)?;
        validate_level_diff_expected_evidence(&self.diff_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("level diff blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("level diff guardrails", guardrail)?;
        }
        validate_level_diff_status(self.status, &self.changes, &self.blocked_reasons)
    }
}

pub fn level_visual_semantic_diff_read_model_from_json_str(
    input: &str,
) -> Result<LevelVisualSemanticDiffReadModel> {
    let artifact = LevelVisualSemanticDiffArtifact::from_json_str(input)?;
    Ok(level_visual_semantic_diff_read_model(&artifact))
}

pub fn level_visual_semantic_diff_read_model(
    artifact: &LevelVisualSemanticDiffArtifact,
) -> LevelVisualSemanticDiffReadModel {
    let count_category = |category| {
        artifact
            .changes
            .iter()
            .filter(|change| change.category == category)
            .count()
    };
    LevelVisualSemanticDiffReadModel {
        schema_version: LEVEL_VISUAL_SEMANTIC_DIFF_READ_MODEL_SCHEMA_VERSION.to_string(),
        diff_id: artifact.diff_id.clone(),
        status: level_diff_status_label(artifact.status).to_string(),
        change_count: artifact.changes.len(),
        semantic_change_count: artifact
            .changes
            .iter()
            .filter(|change| change.category != LevelSemanticChangeCategory::Unchanged)
            .count(),
        missing_evidence_count: count_category(LevelSemanticChangeCategory::MissingEvidence),
        partial_count: count_category(LevelSemanticChangeCategory::Partial),
        scenario_impact_count: artifact.expected_scenario_impacts.len(),
        linked_evidence_refs: level_diff_linked_refs(artifact),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only level visual and semantic diff evidence; trusted diffs are local Rust-validated artifacts, browser and Studio display only, no scene writes, no trusted apply, no browser command bridge, no auto-apply, and no auto-merge.".to_string(),
    }
}

fn level_diff_linked_refs(artifact: &LevelVisualSemanticDiffArtifact) -> Vec<String> {
    std::iter::once(artifact.before_draft_ref.clone())
        .chain(std::iter::once(artifact.after_draft_ref.clone()))
        .chain(std::iter::once(artifact.reachability_evidence_ref.clone()))
        .chain(std::iter::once(artifact.objective_proof_ref.clone()))
        .chain(std::iter::once(artifact.heuristic_evidence_ref.clone()))
        .chain(artifact.transaction_ref.iter().cloned())
        .chain(
            artifact
                .expected_evidence
                .iter()
                .map(|evidence| evidence.path_hint.clone()),
        )
        .collect()
}

fn validate_level_diff_changes(changes: &[LevelSemanticChange]) -> Result<()> {
    if changes.is_empty() || changes.len() > MAX_LEVEL_DIFF_CHANGES {
        return Err(anyhow!(
            "level diff changes must contain between 1 and {MAX_LEVEL_DIFF_CHANGES} entries"
        ));
    }
    let mut ids = BTreeSet::new();
    for change in changes {
        validate_path_component("level diff changes.changeId", &change.change_id)?;
        require_bounded_display_text("level diff changes.summary", &change.summary)?;
        if let Some(before_ref) = &change.before_ref {
            validate_evidence_artifact_path(before_ref)?;
        }
        if let Some(after_ref) = &change.after_ref {
            validate_evidence_artifact_path(after_ref)?;
        }
        if !ids.insert(change.change_id.as_str()) {
            return Err(anyhow!(
                "duplicate level diff changes.changeId: {}",
                change.change_id
            ));
        }
    }
    Ok(())
}

fn validate_level_diff_expected_evidence(
    diff_id: &str,
    values: &[LevelDiffExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!("level diff expectedEvidence must not be empty"));
    }
    let expected_prefix = format!("evidence/level-diff/{diff_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "level diff expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "level diff expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate level diff expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate level diff expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
    }
    Ok(())
}

const AGENT_GENERATED_LEVEL_DRAFT_SCHEMA_VERSION: &str = "agent-generated-level-draft-v1";
const AGENT_GENERATED_LEVEL_DRAFT_READ_MODEL_SCHEMA_VERSION: &str =
    "agent-generated-level-draft-read-model-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AgentGeneratedLevelDraftArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "draftKind")]
    pub draft_kind: AgentGeneratedLevelDraftKind,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "tilemapDraftRef")]
    pub tilemap_draft_ref: String,
    #[serde(rename = "placementDraftRef")]
    pub placement_draft_ref: String,
    #[serde(rename = "reachabilityEvidenceRef")]
    pub reachability_evidence_ref: String,
    #[serde(rename = "objectiveProofRef")]
    pub objective_proof_ref: String,
    #[serde(rename = "heuristicEvidenceRef")]
    pub heuristic_evidence_ref: String,
    #[serde(rename = "diffEvidenceRef")]
    pub diff_evidence_ref: String,
    #[serde(rename = "targetHashes")]
    pub target_hashes: Vec<AgentGeneratedLevelDraftTargetHash>,
    pub author: AgentGeneratedLevelDraftAuthor,
    pub sections: Vec<AgentGeneratedLevelDraftSection>,
    pub operations: Vec<AgentGeneratedLevelDraftOperation>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<AgentGeneratedLevelDraftExpectedEvidence>,
    pub status: AgentGeneratedLevelDraftStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

impl AgentGeneratedLevelDraftArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: AgentGeneratedLevelDraftArtifact = serde_json::from_str(input)
            .context("failed to parse Agent Generated Level Draft JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != AGENT_GENERATED_LEVEL_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "agent generated level draft schemaVersion must be {AGENT_GENERATED_LEVEL_DRAFT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("agent level draft draftId", &self.draft_id)?;
        validate_path_component("agent level draft intentId", &self.intent_id)?;
        validate_path_component("agent level draft planId", &self.plan_id)?;
        validate_repo_relative_source_ref(
            "agent level draft targetSceneRef",
            &self.target_scene_ref,
        )?;
        if !self.target_scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "agent level draft targetSceneRef must point to a .scene.json fixture"
            ));
        }
        validate_evidence_artifact_path(&self.tilemap_draft_ref)?;
        validate_evidence_artifact_path(&self.placement_draft_ref)?;
        validate_evidence_artifact_path(&self.reachability_evidence_ref)?;
        validate_evidence_artifact_path(&self.objective_proof_ref)?;
        validate_evidence_artifact_path(&self.heuristic_evidence_ref)?;
        validate_evidence_artifact_path(&self.diff_evidence_ref)?;
        validate_agent_level_draft_target_hashes(&self.target_scene_ref, &self.target_hashes)?;
        self.author.validate()?;
        validate_agent_level_draft_sections(&self.sections)?;
        validate_agent_level_draft_operations(&self.sections, &self.operations)?;
        validate_agent_level_draft_expected_evidence(&self.draft_id, &self.expected_evidence)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("agent level draft blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("agent level draft guardrails", guardrail)?;
        }
        validate_agent_level_draft_status(
            self.status,
            self.draft_kind,
            &self.sections,
            &self.operations,
            &self.blocked_reasons,
        )
    }
}

pub fn agent_generated_level_draft_read_model_from_json_str(
    input: &str,
) -> Result<AgentGeneratedLevelDraftReadModel> {
    let artifact = AgentGeneratedLevelDraftArtifact::from_json_str(input)?;
    Ok(agent_generated_level_draft_read_model(&artifact))
}

pub fn agent_generated_level_draft_read_model(
    artifact: &AgentGeneratedLevelDraftArtifact,
) -> AgentGeneratedLevelDraftReadModel {
    let partial_count = artifact
        .sections
        .iter()
        .filter(|section| {
            section.completeness == AgentGeneratedLevelDraftSectionCompleteness::Partial
        })
        .count()
        + artifact
            .operations
            .iter()
            .filter(|operation| {
                operation.status == AgentGeneratedLevelDraftOperationStatus::Partial
            })
            .count();
    let missing_evidence_count = artifact
        .sections
        .iter()
        .filter(|section| {
            section.completeness == AgentGeneratedLevelDraftSectionCompleteness::MissingEvidence
        })
        .count()
        + artifact
            .operations
            .iter()
            .filter(|operation| {
                operation.status == AgentGeneratedLevelDraftOperationStatus::MissingEvidence
            })
            .count();
    let unsupported_count = artifact
        .sections
        .iter()
        .filter(|section| {
            section.completeness == AgentGeneratedLevelDraftSectionCompleteness::Unsupported
        })
        .count()
        + artifact
            .operations
            .iter()
            .filter(|operation| {
                operation.status == AgentGeneratedLevelDraftOperationStatus::Unsupported
            })
            .count();
    AgentGeneratedLevelDraftReadModel {
        schema_version: AGENT_GENERATED_LEVEL_DRAFT_READ_MODEL_SCHEMA_VERSION.to_string(),
        draft_id: artifact.draft_id.clone(),
        intent_id: artifact.intent_id.clone(),
        plan_id: artifact.plan_id.clone(),
        status: agent_level_draft_status_label(artifact.status).to_string(),
        section_count: artifact.sections.len(),
        operation_count: artifact.operations.len(),
        partial_count,
        missing_evidence_count,
        unsupported_count,
        linked_evidence_refs: agent_level_draft_linked_refs(artifact),
        target_hash_refs: artifact
            .target_hashes
            .iter()
            .map(|target| target.target_ref.clone())
            .collect(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Read-only untrusted generated level draft; review-gated apply required, no scene writes, no trusted apply, no browser command bridge, no auto-apply, no auto-merge, no autonomous full game generation, and no production editor claim.".to_string(),
    }
}

fn agent_level_draft_linked_refs(artifact: &AgentGeneratedLevelDraftArtifact) -> Vec<String> {
    std::iter::once(artifact.tilemap_draft_ref.clone())
        .chain(std::iter::once(artifact.placement_draft_ref.clone()))
        .chain(std::iter::once(artifact.reachability_evidence_ref.clone()))
        .chain(std::iter::once(artifact.objective_proof_ref.clone()))
        .chain(std::iter::once(artifact.heuristic_evidence_ref.clone()))
        .chain(std::iter::once(artifact.diff_evidence_ref.clone()))
        .chain(
            artifact
                .expected_evidence
                .iter()
                .map(|evidence| evidence.path_hint.clone()),
        )
        .collect()
}

fn validate_agent_level_draft_expected_evidence(
    draft_id: &str,
    values: &[AgentGeneratedLevelDraftExpectedEvidence],
) -> Result<()> {
    if values.is_empty() {
        return Err(anyhow!(
            "agent level draft expectedEvidence must not be empty"
        ));
    }
    let expected_prefix = format!("evidence/agent-level-drafts/{draft_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for evidence in values {
        validate_path_component(
            "agent level draft expectedEvidence.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "agent level draft expectedEvidence.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate agent level draft expectedEvidence.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate agent level draft expectedEvidence.pathHint: {}",
                evidence.path_hint
            ));
        }
        kinds.insert(evidence.kind);
    }
    for required in [
        AgentGeneratedLevelDraftExpectedEvidenceKind::Constraint,
        AgentGeneratedLevelDraftExpectedEvidenceKind::Reachability,
        AgentGeneratedLevelDraftExpectedEvidenceKind::ObjectiveProof,
    ] {
        if !kinds.contains(&required) {
            return Err(anyhow!(
                "agent level draft expectedEvidence must include {} evidence",
                agent_level_draft_expected_evidence_kind_label(required)
            ));
        }
    }
    Ok(())
}

const REVIEW_GATED_LEVEL_APPLY_SCHEMA_VERSION: &str = "review-gated-level-apply-v1";
const REVIEW_GATED_LEVEL_APPLY_READ_MODEL_SCHEMA_VERSION: &str =
    "review-gated-level-apply-read-model-v1";
const MAX_REVIEW_GATED_LEVEL_APPLY_TARGETS: usize = 16;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewGatedLevelApplyArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "reviewDecision")]
    pub review_decision: ReviewGatedLevelApplyDecision,
    #[serde(rename = "targetHashes")]
    pub target_hashes: Vec<ReviewGatedLevelApplyTargetHash>,
    pub targets: Vec<ReviewGatedLevelApplyTarget>,
    #[serde(rename = "rollbackMetadata")]
    pub rollback_metadata: ReviewGatedLevelApplyRollbackMetadata,
    #[serde(rename = "rerunCommands")]
    pub rerun_commands: Vec<ReviewGatedLevelApplyRerunCommand>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<ReviewGatedLevelApplyEvidenceRef>,
    pub status: ReviewGatedLevelApplyStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewGatedLevelApplyDecision {
    #[serde(rename = "reviewDecisionId")]
    pub review_decision_id: String,
    pub status: ReviewGatedLevelApplyDecisionStatus,
    #[serde(rename = "reviewerId")]
    pub reviewer_id: String,
    #[serde(rename = "draftAuthorId")]
    pub draft_author_id: String,
    #[serde(rename = "decisionRef")]
    pub decision_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReviewGatedLevelApplyRollbackMetadata {
    #[serde(rename = "rollbackPlanRef")]
    pub rollback_plan_ref: String,
    #[serde(rename = "preApplyBranch")]
    pub pre_apply_branch: String,
    #[serde(rename = "preApplyCommit")]
    pub pre_apply_commit: String,
    #[serde(rename = "targetBeforeHashes")]
    pub target_before_hashes: Vec<ReviewGatedLevelApplyRollbackTarget>,
}

impl ReviewGatedLevelApplyArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: ReviewGatedLevelApplyArtifact =
            serde_json::from_str(input).context("failed to parse Review-Gated Level Apply JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != REVIEW_GATED_LEVEL_APPLY_SCHEMA_VERSION {
            return Err(anyhow!(
                "review-gated level apply schemaVersion must be {REVIEW_GATED_LEVEL_APPLY_SCHEMA_VERSION}"
            ));
        }
        validate_path_component(
            "review-gated level apply transactionId",
            &self.transaction_id,
        )?;
        validate_path_component("review-gated level apply draftId", &self.draft_id)?;
        validate_path_component("review-gated level apply intentId", &self.intent_id)?;
        validate_path_component("review-gated level apply planId", &self.plan_id)?;
        self.review_decision.validate()?;
        validate_review_gated_level_apply_targets(&self.transaction_id, &self.targets)?;
        validate_review_gated_level_apply_target_hashes(&self.targets, &self.target_hashes)?;
        self.rollback_metadata.validate(&self.targets)?;
        validate_review_gated_level_apply_rerun_commands(&self.rerun_commands)?;
        validate_review_gated_level_apply_evidence(
            &self.transaction_id,
            self.status,
            &self.evidence_refs,
        )?;
        validate_review_gated_level_apply_guardrails(&self.guardrails)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("review-gated level apply blockedReasons", reason)?;
        }
        validate_review_gated_level_apply_status(
            self.status,
            &self.review_decision,
            &self.target_hashes,
            &self.blocked_reasons,
        )
    }
}

impl ReviewGatedLevelApplyDecision {
    fn validate(&self) -> Result<()> {
        validate_path_component(
            "review-gated level apply reviewDecision.reviewDecisionId",
            &self.review_decision_id,
        )?;
        validate_path_component(
            "review-gated level apply reviewDecision.reviewerId",
            &self.reviewer_id,
        )?;
        validate_path_component(
            "review-gated level apply reviewDecision.draftAuthorId",
            &self.draft_author_id,
        )?;
        validate_evidence_artifact_path(&self.decision_ref)?;
        Ok(())
    }
}

impl ReviewGatedLevelApplyRollbackMetadata {
    fn validate(&self, targets: &[ReviewGatedLevelApplyTarget]) -> Result<()> {
        validate_evidence_artifact_path(&self.rollback_plan_ref)?;
        validate_path_component(
            "review-gated level apply rollbackMetadata.preApplyBranch",
            &self.pre_apply_branch,
        )?;
        validate_snapshot_hash(
            "review-gated level apply rollbackMetadata.preApplyCommit",
            &self.pre_apply_commit,
        )?;
        if self.target_before_hashes.is_empty() {
            return Err(anyhow!(
                "review-gated level apply rollbackMetadata.targetBeforeHashes must not be empty"
            ));
        }
        let target_refs = targets
            .iter()
            .map(|target| target.target_ref.as_str())
            .collect::<BTreeSet<_>>();
        let mut rollback_refs = BTreeSet::new();
        for target in &self.target_before_hashes {
            validate_repo_relative_source_ref(
                "review-gated level apply rollbackMetadata.targetBeforeHashes.targetRef",
                &target.target_ref,
            )?;
            validate_snapshot_hash(
                "review-gated level apply rollbackMetadata.targetBeforeHashes.beforeHash",
                &target.before_hash,
            )?;
            if !target_refs.contains(target.target_ref.as_str()) {
                return Err(anyhow!(
                    "review-gated level apply rollbackMetadata targetBeforeHashes targetRef is not an apply target: {}",
                    target.target_ref
                ));
            }
            if !rollback_refs.insert(target.target_ref.as_str()) {
                return Err(anyhow!(
                    "duplicate review-gated level apply rollback target: {}",
                    target.target_ref
                ));
            }
        }
        for target in targets {
            if !rollback_refs.contains(target.target_ref.as_str()) {
                return Err(anyhow!(
                    "review-gated level apply rollbackMetadata must include target: {}",
                    target.target_ref
                ));
            }
        }
        Ok(())
    }
}

pub fn review_gated_level_apply_read_model_from_json_str(
    input: &str,
) -> Result<ReviewGatedLevelApplyReadModel> {
    let artifact = ReviewGatedLevelApplyArtifact::from_json_str(input)?;
    Ok(review_gated_level_apply_read_model(&artifact))
}

pub fn review_gated_level_apply_read_model(
    artifact: &ReviewGatedLevelApplyArtifact,
) -> ReviewGatedLevelApplyReadModel {
    ReviewGatedLevelApplyReadModel {
        schema_version: REVIEW_GATED_LEVEL_APPLY_READ_MODEL_SCHEMA_VERSION.to_string(),
        transaction_id: artifact.transaction_id.clone(),
        draft_id: artifact.draft_id.clone(),
        review_decision_id: artifact.review_decision.review_decision_id.clone(),
        status: review_gated_level_apply_status_label(artifact.status).to_string(),
        target_count: artifact.targets.len(),
        rerun_command_count: artifact.rerun_commands.len(),
        evidence_count: artifact.evidence_refs.len(),
        blocked_reasons: artifact.blocked_reasons.clone(),
        boundary: "Review-gated level apply contract; ready state requires accepted non-self review, fresh target hashes, rollback metadata, rerun evidence, and safe transaction outputs; no browser command bridge, no auto-apply, no auto-merge, no self-approval, no arbitrary script execution, and no autonomous full game generation.".to_string(),
    }
}

fn validate_review_gated_level_apply_targets(
    transaction_id: &str,
    targets: &[ReviewGatedLevelApplyTarget],
) -> Result<()> {
    if targets.is_empty() || targets.len() > MAX_REVIEW_GATED_LEVEL_APPLY_TARGETS {
        return Err(anyhow!(
            "review-gated level apply targets must contain between 1 and {MAX_REVIEW_GATED_LEVEL_APPLY_TARGETS} entries"
        ));
    }
    let mut target_refs = BTreeSet::new();
    let mut output_refs = BTreeSet::new();
    let expected_prefix = format!("evidence/level-apply/{transaction_id}/");
    for target in targets {
        validate_repo_relative_source_ref(
            "review-gated level apply targets.targetRef",
            &target.target_ref,
        )?;
        if matches!(target.kind, ReviewGatedLevelApplyTargetKind::Scene)
            && !target.target_ref.ends_with(".scene.json")
        {
            return Err(anyhow!(
                "review-gated level apply scene targets must point to .scene.json fixtures"
            ));
        }
        validate_evidence_artifact_path(&target.transaction_output_ref)?;
        if !target.transaction_output_ref.starts_with(&expected_prefix)
            || !target.transaction_output_ref.ends_with(".json")
        {
            return Err(anyhow!(
                "review-gated level apply transactionOutputRef must be JSON evidence under {expected_prefix}"
            ));
        }
        if target.transaction_output_ref == target.target_ref {
            return Err(anyhow!(
                "review-gated level apply transactionOutputRef must not collide with targetRef"
            ));
        }
        validate_snapshot_hash(
            "review-gated level apply targets.expectedAfterHash",
            &target.expected_after_hash,
        )?;
        if !target_refs.insert(target.target_ref.as_str()) {
            return Err(anyhow!(
                "duplicate review-gated level apply targets.targetRef: {}",
                target.target_ref
            ));
        }
        if !output_refs.insert(target.transaction_output_ref.as_str()) {
            return Err(anyhow!(
                "duplicate review-gated level apply targets.transactionOutputRef: {}",
                target.transaction_output_ref
            ));
        }
    }
    Ok(())
}

fn validate_review_gated_level_apply_evidence(
    transaction_id: &str,
    status: ReviewGatedLevelApplyStatus,
    refs: &[ReviewGatedLevelApplyEvidenceRef],
) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!(
            "review-gated level apply evidenceRefs must not be empty"
        ));
    }
    let expected_prefix = format!("evidence/level-apply/{transaction_id}/");
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for evidence in refs {
        validate_path_component(
            "review-gated level apply evidenceRefs.evidenceId",
            &evidence.evidence_id,
        )?;
        validate_evidence_artifact_path(&evidence.path_hint)?;
        if !evidence.path_hint.starts_with(&expected_prefix)
            || !evidence.path_hint.ends_with(".json")
        {
            return Err(anyhow!(
                "review-gated level apply evidenceRefs.pathHint must be JSON evidence under {expected_prefix}"
            ));
        }
        if !ids.insert(evidence.evidence_id.as_str()) {
            return Err(anyhow!(
                "duplicate review-gated level apply evidenceRefs.evidenceId: {}",
                evidence.evidence_id
            ));
        }
        if !paths.insert(evidence.path_hint.as_str()) {
            return Err(anyhow!(
                "duplicate review-gated level apply evidenceRefs.pathHint: {}",
                evidence.path_hint
            ));
        }
        kinds.insert(evidence.kind);
    }
    for required in [
        ReviewGatedLevelApplyEvidenceKind::AgentDraft,
        ReviewGatedLevelApplyEvidenceKind::ReviewDecision,
        ReviewGatedLevelApplyEvidenceKind::RollbackPlan,
        ReviewGatedLevelApplyEvidenceKind::RerunPlan,
        ReviewGatedLevelApplyEvidenceKind::GeneratedStateAudit,
    ] {
        if !kinds.contains(&required) {
            return Err(anyhow!(
                "review-gated level apply evidenceRefs must include {} evidence",
                review_gated_level_apply_evidence_kind_label(required)
            ));
        }
    }
    if status == ReviewGatedLevelApplyStatus::ReadyForTrustedApply
        && !kinds.contains(&ReviewGatedLevelApplyEvidenceKind::LevelDiff)
    {
        return Err(anyhow!(
            "review-gated level apply ready_for_trusted_apply artifacts must include {} evidence",
            review_gated_level_apply_evidence_kind_label(
                ReviewGatedLevelApplyEvidenceKind::LevelDiff
            )
        ));
    }
    Ok(())
}

fn validate_review_gated_level_apply_status(
    status: ReviewGatedLevelApplyStatus,
    decision: &ReviewGatedLevelApplyDecision,
    target_hashes: &[ReviewGatedLevelApplyTargetHash],
    blocked_reasons: &[String],
) -> Result<()> {
    let is_self_approval = decision.reviewer_id == decision.draft_author_id;
    let has_stale_target = target_hashes
        .iter()
        .any(|hash| hash.expected_before_hash != hash.observed_before_hash);
    match status {
        ReviewGatedLevelApplyStatus::ReadyForTrustedApply
            if decision.status == ReviewGatedLevelApplyDecisionStatus::Accepted
                && !is_self_approval
                && !has_stale_target
                && blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        ReviewGatedLevelApplyStatus::MissingReview
            if decision.status == ReviewGatedLevelApplyDecisionStatus::Missing
                && !blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        ReviewGatedLevelApplyStatus::Rejected
            if matches!(
                decision.status,
                ReviewGatedLevelApplyDecisionStatus::Rejected
                    | ReviewGatedLevelApplyDecisionStatus::Deferred
            ) && !blocked_reasons.is_empty() =>
        {
            Ok(())
        }
        ReviewGatedLevelApplyStatus::Stale if !blocked_reasons.is_empty() => Ok(()),
        ReviewGatedLevelApplyStatus::Blocked if !blocked_reasons.is_empty() => Ok(()),
        ReviewGatedLevelApplyStatus::ReadyForTrustedApply => Err(anyhow!(
            "review-gated level apply ready status requires accepted non-self review, fresh target hashes, and no blockedReasons"
        )),
        ReviewGatedLevelApplyStatus::MissingReview => Err(anyhow!(
            "review-gated level apply missing_review status requires missing review decision and blockedReasons"
        )),
        ReviewGatedLevelApplyStatus::Rejected => Err(anyhow!(
            "review-gated level apply rejected status requires rejected/deferred review decision and blockedReasons"
        )),
        ReviewGatedLevelApplyStatus::Stale => Err(anyhow!(
            "review-gated level apply stale status requires blockedReasons"
        )),
        ReviewGatedLevelApplyStatus::Blocked => Err(anyhow!(
            "review-gated level apply blocked status requires blockedReasons"
        )),
    }
}

const ADVERSARIAL_INPUT_FUZZING_PLAN_SCHEMA_VERSION: &str = "adversarial-input-fuzzing-plan-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AdversarialInputFuzzingPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "inputDomain")]
    pub input_domain: FuzzInputDomain,
    #[serde(rename = "deterministicSeed")]
    pub deterministic_seed: u64,
    pub budget: FuzzBudget,
    #[serde(rename = "actionSet")]
    pub action_set: Vec<FuzzAction>,
    pub constraints: Vec<FuzzConstraint>,
    #[serde(rename = "stopCondition")]
    pub stop_condition: FuzzStopCondition,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<FuzzExpectedEvidence>,
    #[serde(rename = "outputRoot")]
    pub output_root: String,
    #[serde(rename = "cleanupPolicy")]
    pub cleanup_policy: FuzzCleanupPolicy,
    pub status: FuzzPlanStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FuzzInputDomain {
    #[serde(rename = "domainId")]
    pub domain_id: String,
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(
        rename = "scenarioCandidateRef",
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_candidate_ref: Option<String>,
    #[serde(rename = "replayEvidenceRef", skip_serializing_if = "Option::is_none")]
    pub replay_evidence_ref: Option<String>,
    #[serde(rename = "allowedKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_keys: Vec<ReplayKey>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FuzzExpectedEvidence {
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "artifactKind")]
    pub artifact_kind: FuzzExpectedEvidenceKind,
    #[serde(rename = "pathHint")]
    pub path_hint: String,
}

impl AdversarialInputFuzzingPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: AdversarialInputFuzzingPlanArtifact = serde_json::from_str(input)
            .context("failed to parse Adversarial Input Fuzzing Plan JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ADVERSARIAL_INPUT_FUZZING_PLAN_SCHEMA_VERSION {
            return Err(anyhow!(
                "adversarial input fuzzing plan schemaVersion must be {ADVERSARIAL_INPUT_FUZZING_PLAN_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("adversarial input fuzzing plan planId", &self.plan_id)?;
        validate_path_component("adversarial input fuzzing plan runId", &self.run_id)?;
        self.input_domain.validate()?;
        self.budget.validate()?;
        if self.action_set.is_empty() {
            return Err(anyhow!(
                "adversarial input fuzzing plan actionSet must not be empty"
            ));
        }
        let mut actions = BTreeSet::new();
        for action in &self.action_set {
            if !actions.insert(*action) {
                return Err(anyhow!(
                    "adversarial input fuzzing plan actionSet must not contain duplicates"
                ));
            }
        }
        if actions.contains(&FuzzAction::ReplayStep)
            && self.input_domain.replay_evidence_ref.is_none()
        {
            return Err(anyhow!(
                "adversarial input fuzzing plan actionSet replay_step requires inputDomain.replayEvidenceRef"
            ));
        }
        if actions.contains(&FuzzAction::SnapshotProbe)
            && !self.expected_evidence.iter().any(|evidence| {
                matches!(
                    evidence.artifact_kind,
                    FuzzExpectedEvidenceKind::RuntimeProbe | FuzzExpectedEvidenceKind::WorldState
                )
            })
        {
            return Err(anyhow!(
                "adversarial input fuzzing plan actionSet snapshot_probe requires runtime_probe or world_state expectedEvidence"
            ));
        }
        if self.constraints.is_empty() {
            return Err(anyhow!(
                "adversarial input fuzzing plan constraints must not be empty"
            ));
        }
        let mut constraint_ids = BTreeSet::new();
        for constraint in &self.constraints {
            constraint.validate()?;
            if !constraint_ids.insert(constraint.constraint_id.as_str()) {
                return Err(anyhow!(
                    "duplicate adversarial input fuzzing plan constraintId: {}",
                    constraint.constraint_id
                ));
            }
        }
        self.stop_condition.validate()?;
        if self.expected_evidence.is_empty() {
            return Err(anyhow!(
                "adversarial input fuzzing plan expectedEvidence must not be empty"
            ));
        }
        let mut evidence_ids = BTreeSet::new();
        let mut path_hints = BTreeSet::new();
        for evidence in &self.expected_evidence {
            evidence.validate(&self.output_root)?;
            if !evidence_ids.insert(evidence.evidence_id.as_str()) {
                return Err(anyhow!(
                    "duplicate adversarial input fuzzing plan evidenceId: {}",
                    evidence.evidence_id
                ));
            }
            if !path_hints.insert(evidence.path_hint.as_str()) {
                return Err(anyhow!(
                    "duplicate adversarial input fuzzing plan expectedEvidence.pathHint: {}",
                    evidence.path_hint
                ));
            }
        }
        validate_fuzz_output_root(&self.output_root, &self.plan_id)?;
        self.cleanup_policy.validate()?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text("adversarial input fuzzing plan blockedReasons", reason)?;
        }
        match self.status {
            FuzzPlanStatus::Blocked if self.blocked_reasons.is_empty() => Err(anyhow!(
                "adversarial input fuzzing plan blocked status requires blockedReasons"
            )),
            FuzzPlanStatus::Planned if !self.blocked_reasons.is_empty() => Err(anyhow!(
                "adversarial input fuzzing plan planned status must not include blockedReasons"
            )),
            _ => {
                for guardrail in &self.guardrails {
                    require_bounded_display_text(
                        "adversarial input fuzzing plan guardrail",
                        guardrail,
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl FuzzInputDomain {
    fn validate(&self) -> Result<()> {
        validate_path_component(
            "adversarial input fuzzing plan inputDomain.domainId",
            &self.domain_id,
        )?;
        validate_path_component(
            "adversarial input fuzzing plan inputDomain.scenarioId",
            &self.scenario_id,
        )?;
        if self.scenario_candidate_ref.is_none() && self.replay_evidence_ref.is_none() {
            return Err(anyhow!(
                "adversarial input fuzzing plan inputDomain requires scenarioCandidateRef or replayEvidenceRef"
            ));
        }
        if let Some(reference) = &self.scenario_candidate_ref {
            validate_evidence_artifact_path(reference)?;
            if !reference.starts_with("evidence/scenarios/") || !reference.ends_with(".json") {
                return Err(anyhow!(
                    "adversarial input fuzzing plan scenarioCandidateRef must be scenario JSON evidence"
                ));
            }
        }
        if let Some(reference) = &self.replay_evidence_ref {
            validate_evidence_artifact_path(reference)?;
            if !reference.starts_with("evidence/scenarios/") || !reference.ends_with(".json") {
                return Err(anyhow!(
                    "adversarial input fuzzing plan replayEvidenceRef must be scenario JSON evidence"
                ));
            }
        }
        if self.allowed_keys.is_empty() {
            return Err(anyhow!(
                "adversarial input fuzzing plan inputDomain.allowedKeys must not be empty"
            ));
        }
        let mut keys = BTreeSet::new();
        for key in &self.allowed_keys {
            if !keys.insert(*key) {
                return Err(anyhow!(
                    "adversarial input fuzzing plan inputDomain.allowedKeys must not contain duplicates"
                ));
            }
        }
        Ok(())
    }
}

impl FuzzExpectedEvidence {
    fn validate(&self, output_root: &str) -> Result<()> {
        validate_path_component(
            "adversarial input fuzzing plan expectedEvidence.evidenceId",
            &self.evidence_id,
        )?;
        validate_evidence_artifact_path(&self.path_hint)?;
        match self.artifact_kind {
            FuzzExpectedEvidenceKind::FuzzInput | FuzzExpectedEvidenceKind::FuzzSummary => {
                if !self.path_hint.starts_with(output_root) {
                    return Err(anyhow!(
                        "adversarial input fuzzing plan expectedEvidence.pathHint for fuzz outputs must stay under outputRoot"
                    ));
                }
            }
            FuzzExpectedEvidenceKind::ScenarioInputReplay
            | FuzzExpectedEvidenceKind::ScenarioResult => {
                if !self.path_hint.starts_with("evidence/scenarios/")
                    || !self.path_hint.ends_with(".json")
                {
                    return Err(anyhow!(
                        "adversarial input fuzzing plan expectedEvidence.pathHint for scenario evidence must be scenario JSON evidence"
                    ));
                }
            }
            FuzzExpectedEvidenceKind::WorldState | FuzzExpectedEvidenceKind::RuntimeProbe => {
                if !self.path_hint.starts_with("evidence/scenarios/")
                    && !self.path_hint.starts_with(output_root)
                {
                    return Err(anyhow!(
                        "adversarial input fuzzing plan expectedEvidence.pathHint for probe evidence must stay under outputRoot or evidence/scenarios"
                    ));
                }
            }
        }
        Ok(())
    }
}

fn validate_fuzz_output_root(output_root: &str, plan_id: &str) -> Result<()> {
    validate_evidence_artifact_path(output_root)?;
    let expected_prefix = format!("evidence/fuzz/{plan_id}/");
    if !output_root.starts_with(&expected_prefix) {
        return Err(anyhow!(
            "adversarial input fuzzing plan outputRoot must stay under {expected_prefix}"
        ));
    }
    Ok(())
}

pub fn validate_adversarial_input_fuzzing_plan_refs(
    run_dir: impl AsRef<Path>,
    plan: &AdversarialInputFuzzingPlanArtifact,
) -> Result<()> {
    let run_dir = run_dir.as_ref();
    plan.validate()?;
    let index = read_evidence_index(run_dir)?;
    let indexed_paths = index
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    let expected_refs = [
        plan.input_domain.scenario_candidate_ref.as_deref(),
        plan.input_domain.replay_evidence_ref.as_deref(),
    ];
    for reference in expected_refs.into_iter().flatten() {
        if !indexed_paths.contains(reference) {
            return Err(anyhow!(
                "adversarial input fuzzing plan reference is missing from evidence index: {reference}"
            ));
        }
        let value = read_json_value(run_dir.join(reference)).with_context(|| {
            format!("adversarial input fuzzing plan reference is unreadable: {reference}")
        })?;
        validate_fuzz_linked_reference_freshness(plan, reference, &value)?;
    }
    Ok(())
}

fn validate_fuzz_linked_reference_freshness(
    plan: &AdversarialInputFuzzingPlanArtifact,
    reference: &str,
    value: &serde_json::Value,
) -> Result<()> {
    let run_id = json_string(value, "runId").or_else(|| json_string(value, "run_id"));
    match run_id {
        Some(run_id) if run_id == plan.run_id => {}
        Some(run_id) => {
            return Err(anyhow!(
                "adversarial input fuzzing plan reference is stale for runId {run_id}; expected {} at {reference}",
                plan.run_id
            ));
        }
        None => {
            return Err(anyhow!(
                "adversarial input fuzzing plan reference is missing a runId/run_id: {reference}"
            ));
        }
    }

    let scenario_id =
        json_string(value, "scenarioId").or_else(|| json_string(value, "scenario_id"));
    if let Some(scenario_id) = scenario_id {
        if scenario_id != plan.input_domain.scenario_id {
            return Err(anyhow!(
                "adversarial input fuzzing plan reference scenarioId drift at {reference}: {scenario_id} != {}",
                plan.input_domain.scenario_id
            ));
        }
    }

    let target_id = json_string(value, "targetId").or_else(|| json_string(value, "target_id"));
    if let Some(target_id) = target_id {
        if target_id != plan.input_domain.domain_id && target_id != plan.input_domain.scenario_id {
            return Err(anyhow!(
                "adversarial input fuzzing plan reference target identity drift at {reference}: {target_id}"
            ));
        }
    }
    Ok(())
}

const QA_WORKER_ASSIGNMENT_SCHEMA_VERSION: &str = "qa-worker-assignment-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaWorkerAssignmentArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    pub assignments: Vec<QaWorkerAssignment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaWorkerAssignment {
    #[serde(rename = "assignmentId")]
    pub assignment_id: String,
    #[serde(rename = "workerId")]
    pub worker_id: String,
    #[serde(rename = "assignedLane")]
    pub assigned_lane: String,
    pub target: QaWorkerAssignmentTarget,
    pub budget: QaWorkerAssignmentBudget,
    #[serde(rename = "timeoutMs")]
    pub timeout_ms: u64,
    #[serde(rename = "runCount")]
    pub run_count: u32,
    #[serde(rename = "outputRoot")]
    pub output_root: String,
    #[serde(rename = "cleanupPolicy")]
    pub cleanup_policy: QaWorkerCleanupPolicy,
    pub status: QaWorkerAssignmentStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QaWorkerAssignmentTarget {
    #[serde(rename = "targetType")]
    pub target_type: QaWorkerAssignmentTargetType,
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
}

impl QaWorkerAssignmentArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: QaWorkerAssignmentArtifact =
            serde_json::from_str(input).context("failed to parse QA Worker Assignment JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != QA_WORKER_ASSIGNMENT_SCHEMA_VERSION {
            return Err(anyhow!(
                "qa worker assignment schemaVersion must be {QA_WORKER_ASSIGNMENT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("qa worker assignment planId", &self.plan_id)?;
        validate_path_component("qa worker assignment runId", &self.run_id)?;
        if self.assignments.is_empty() {
            return Err(anyhow!(
                "qa worker assignment assignments must not be empty"
            ));
        }
        let mut assignment_ids = BTreeSet::new();
        let mut output_roots = BTreeSet::new();
        for (index, assignment) in self.assignments.iter().enumerate() {
            assignment.validate(index)?;
            if !assignment_ids.insert(assignment.assignment_id.as_str()) {
                return Err(anyhow!(
                    "duplicate qa worker assignment assignmentId: {}",
                    assignment.assignment_id
                ));
            }
            if !output_roots.insert(assignment.output_root.as_str()) {
                return Err(anyhow!(
                    "duplicate qa worker assignment outputRoot: {}",
                    assignment.output_root
                ));
            }
        }
        for (left_index, left) in self.assignments.iter().enumerate() {
            for (right_index, right) in self.assignments.iter().enumerate().skip(left_index + 1) {
                if qa_worker_output_roots_overlap(&left.output_root, &right.output_root) {
                    return Err(anyhow!(
                        "qa worker assignment outputRoot overlap between assignments[{left_index}] and assignments[{right_index}]"
                    ));
                }
            }
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("qa worker assignment guardrail", guardrail)?;
        }
        Ok(())
    }
}

impl QaWorkerAssignment {
    fn validate(&self, index: usize) -> Result<()> {
        validate_path_component(
            &format!("qa worker assignments[{index}].assignmentId"),
            &self.assignment_id,
        )?;
        validate_path_component(
            &format!("qa worker assignments[{index}].workerId"),
            &self.worker_id,
        )?;
        validate_path_component(
            &format!("qa worker assignments[{index}].assignedLane"),
            &self.assigned_lane,
        )?;
        self.target.validate(index)?;
        self.budget.validate(index)?;
        if self.timeout_ms == 0 {
            return Err(anyhow!(
                "qa worker assignments[{index}].timeoutMs must be bounded and greater than zero"
            ));
        }
        if self.run_count > self.budget.max_runs {
            return Err(anyhow!(
                "qa worker assignments[{index}].runCount must not exceed budget.maxRuns"
            ));
        }
        validate_qa_worker_output_root(&self.output_root, &self.worker_id)
            .with_context(|| format!("qa worker assignments[{index}].outputRoot is invalid"))?;
        self.cleanup_policy.validate(index)?;
        for reason in &self.blocked_reasons {
            require_bounded_display_text(
                &format!("qa worker assignments[{index}].blockedReasons"),
                reason,
            )?;
        }
        match self.status {
            QaWorkerAssignmentStatus::Blocked if self.blocked_reasons.is_empty() => Err(anyhow!(
                "qa worker assignments[{index}] blocked status requires blockedReasons"
            )),
            QaWorkerAssignmentStatus::Assigned | QaWorkerAssignmentStatus::Passed
                if !self.blocked_reasons.is_empty() =>
            {
                Err(anyhow!(
                    "qa worker assignments[{index}] assigned/passed status must not include blockedReasons"
                ))
            }
            _ => Ok(()),
        }
    }
}

impl QaWorkerAssignmentTarget {
    fn validate(&self, index: usize) -> Result<()> {
        validate_path_component(
            &format!("qa worker assignments[{index}].target.targetId"),
            &self.target_id,
        )?;
        validate_qa_worker_target_ref(&self.evidence_ref, self.target_type).with_context(|| {
            format!("qa worker assignments[{index}].target.evidenceRef is invalid")
        })
    }
}

fn validate_qa_worker_target_ref(
    reference: &str,
    target_type: QaWorkerAssignmentTargetType,
) -> Result<()> {
    validate_evidence_artifact_path(reference)?;
    if !reference.ends_with(".json") {
        return Err(anyhow!(
            "qa worker assignment target evidenceRef must be JSON"
        ));
    }
    let allowed_prefix = match target_type {
        QaWorkerAssignmentTargetType::ScenarioCandidate => "evidence/scenarios/",
        QaWorkerAssignmentTargetType::FuzzTarget => "evidence/fuzz/",
    };
    if !reference.starts_with(allowed_prefix) {
        return Err(anyhow!(
            "qa worker assignment target evidenceRef must match targetType output root {allowed_prefix}"
        ));
    }
    Ok(())
}

fn validate_qa_worker_output_root(output_root: &str, worker_id: &str) -> Result<()> {
    validate_evidence_artifact_path(output_root)?;
    let expected_prefix = format!("evidence/qa-workers/{worker_id}/");
    if !output_root.starts_with(&expected_prefix) {
        return Err(anyhow!(
            "qa worker assignment outputRoot must stay under {expected_prefix}"
        ));
    }
    Ok(())
}

pub fn validate_qa_worker_assignment_refs(
    run_dir: impl AsRef<Path>,
    artifact: &QaWorkerAssignmentArtifact,
) -> Result<()> {
    let run_dir = run_dir.as_ref();
    artifact.validate()?;
    let index = read_evidence_index(run_dir)?;
    let indexed_paths = index
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    for (index, assignment) in artifact.assignments.iter().enumerate() {
        if !indexed_paths.contains(assignment.target.evidence_ref.as_str()) {
            return Err(anyhow!(
                "qa worker assignments[{index}].target.evidenceRef is missing from evidence index: {}",
                assignment.target.evidence_ref
            ));
        }
        let value =
            read_json_value(run_dir.join(&assignment.target.evidence_ref)).with_context(|| {
                format!(
                    "failed to read qa worker assignments[{index}].target.evidenceRef {}",
                    assignment.target.evidence_ref
                )
            })?;
        // Fail closed: target evidence must carry a run identity so freshness can
        // actually be proven. Accepting an artifact with no runId/run_id would let
        // an unversioned or stale target pass as current-run planning evidence.
        let run_id = json_string(&value, "runId")
            .or_else(|| json_string(&value, "run_id"))
            .ok_or_else(|| {
                anyhow!(
                    "qa worker assignments[{index}].target.evidenceRef is missing a runId/run_id needed to prove freshness"
                )
            })?;
        if run_id != artifact.run_id {
            return Err(anyhow!(
                "qa worker assignments[{index}].target.evidenceRef is stale for runId {} (expected {})",
                run_id,
                artifact.run_id
            ));
        }
        // Fail closed: target evidence must carry a target identity so target-id
        // drift can be rejected. Accepting an artifact with no recognized identity
        // field would let it validate against any assignment targetId.
        let target_id = json_string(&value, "targetId")
            .or_else(|| json_string(&value, "target_id"))
            .or_else(|| json_string(&value, "scenarioId"))
            .or_else(|| json_string(&value, "scenario_id"))
            .or_else(|| json_string(&value, "id"))
            .ok_or_else(|| {
                anyhow!(
                    "qa worker assignments[{index}].target.evidenceRef is missing a target identity field needed to reject target-id drift"
                )
            })?;
        if target_id != assignment.target.target_id {
            return Err(anyhow!(
                "qa worker assignments[{index}].target.evidenceRef target id {target_id} does not match assignment targetId {}",
                assignment.target.target_id
            ));
        }
    }
    Ok(())
}

const RUNTIME_SAVE_ARTIFACT_SCHEMA_VERSION: &str = "runtime-save-artifact-v1";
const RUNTIME_REPLAY_DIGEST_SCHEMA_VERSION: &str = "runtime-replay-digest-v1";
const RUNTIME_REPLAY_DIVERGENCE_SCHEMA_VERSION: &str = "runtime-replay-divergence-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeSaveArtifactV1 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "saveId")]
    pub save_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "slotId")]
    pub slot_id: String,
    #[serde(rename = "createdAtUnixMs")]
    pub created_at_unix_ms: u128,
    pub state: RuntimeStateV1,
    pub policy: RuntimeSaveGeneratedStatePolicy,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeSaveGeneratedStatePolicy {
    #[serde(rename = "artifactPath")]
    pub artifact_path: String,
    #[serde(rename = "rootKind")]
    pub root_kind: RuntimeSaveRootKind,
    #[serde(rename = "trustedWriter")]
    pub trusted_writer: String,
    #[serde(rename = "browserWriteAccess")]
    pub browser_write_access: RuntimeSaveBrowserWriteAccess,
    #[serde(rename = "retention")]
    pub retention: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeReplayDigestEvidenceV1 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "frameId")]
    pub frame_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    pub tick: u64,
    #[serde(rename = "stateId")]
    pub state_id: String,
    pub digest: RuntimeStateDigest,
    pub policy: RuntimeReplayGeneratedStatePolicy,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeReplayDivergenceEvidenceV1 {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub status: RuntimeReplayDivergenceStatus,
    #[serde(rename = "frameId")]
    pub frame_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    pub tick: u64,
    pub expected: RuntimeStateDigest,
    pub actual: RuntimeStateDigest,
    #[serde(rename = "firstDivergence", skip_serializing_if = "Option::is_none")]
    pub first_divergence: Option<RuntimeReplayFirstDivergence>,
    pub policy: RuntimeReplayGeneratedStatePolicy,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeReplayGeneratedStatePolicy {
    #[serde(rename = "artifactPath")]
    pub artifact_path: String,
    #[serde(rename = "rootKind")]
    pub root_kind: RuntimeSaveRootKind,
    #[serde(rename = "trustedWriter")]
    pub trusted_writer: String,
    #[serde(rename = "browserWriteAccess")]
    pub browser_write_access: RuntimeSaveBrowserWriteAccess,
    #[serde(rename = "retention")]
    pub retention: String,
}

impl RuntimeSaveArtifactV1 {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: RuntimeSaveArtifactV1 =
            serde_json::from_str(input).context("failed to parse Runtime Save Artifact JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_SAVE_ARTIFACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "runtime save artifact schemaVersion must be {RUNTIME_SAVE_ARTIFACT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("runtime save artifact saveId", &self.save_id)?;
        validate_path_component("runtime save artifact runId", &self.run_id)?;
        validate_path_component("runtime save artifact slotId", &self.slot_id)?;
        self.state
            .validate()
            .context("runtime save artifact state is invalid")?;
        if self.state.run_id != self.run_id {
            return Err(anyhow!(
                "runtime save artifact runId must match state runId"
            ));
        }
        self.policy.validate()?;
        Ok(())
    }
}

impl RuntimeSaveGeneratedStatePolicy {
    fn validate(&self) -> Result<()> {
        match self.root_kind {
            RuntimeSaveRootKind::GeneratedEvidence => {
                validate_evidence_artifact_path(&self.artifact_path)?;
                if !self
                    .artifact_path
                    .starts_with("evidence/runtime-state/saves/")
                    || !self.artifact_path.ends_with(".save.json")
                {
                    return Err(anyhow!(
                        "runtime save policy artifactPath must be evidence/runtime-state/saves/<slot>.save.json for generated evidence"
                    ));
                }
            }
            RuntimeSaveRootKind::LocalGeneratedState => {
                validate_runtime_save_local_generated_path(&self.artifact_path)?;
            }
        }
        if self.trusted_writer != "rust-local-runtime-save-v1" {
            return Err(anyhow!(
                "runtime save policy trustedWriter must be rust-local-runtime-save-v1"
            ));
        }
        if self.browser_write_access != RuntimeSaveBrowserWriteAccess::None {
            return Err(anyhow!(
                "runtime save policy browserWriteAccess must be none for trusted save artifacts"
            ));
        }
        require_text("runtime save policy retention", &self.retention)?;
        Ok(())
    }
}

impl RuntimeReplayDigestEvidenceV1 {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let evidence: RuntimeReplayDigestEvidenceV1 =
            serde_json::from_str(input).context("failed to parse Runtime Replay Digest JSON")?;
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_REPLAY_DIGEST_SCHEMA_VERSION {
            return Err(anyhow!(
                "runtime replay digest schemaVersion must be {RUNTIME_REPLAY_DIGEST_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("runtime replay digest frameId", &self.frame_id)?;
        validate_path_component("runtime replay digest sceneId", &self.scene_id)?;
        validate_path_component("runtime replay digest stateId", &self.state_id)?;
        self.digest.validate("runtime replay digest")?;
        self.policy.validate("digest")?;
        Ok(())
    }
}

impl RuntimeReplayDivergenceEvidenceV1 {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let evidence: RuntimeReplayDivergenceEvidenceV1 = serde_json::from_str(input)
            .context("failed to parse Runtime Replay Divergence JSON")?;
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_REPLAY_DIVERGENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "runtime replay divergence schemaVersion must be {RUNTIME_REPLAY_DIVERGENCE_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("runtime replay divergence frameId", &self.frame_id)?;
        validate_path_component("runtime replay divergence sceneId", &self.scene_id)?;
        self.expected.validate("runtime replay expected digest")?;
        self.actual.validate("runtime replay actual digest")?;
        match self.status {
            RuntimeReplayDivergenceStatus::Matched => {
                if self.first_divergence.is_some() {
                    return Err(anyhow!(
                        "runtime replay matched evidence must not include firstDivergence"
                    ));
                }
                if self.expected != self.actual {
                    return Err(anyhow!(
                        "runtime replay matched evidence requires expected and actual digests to match"
                    ));
                }
            }
            RuntimeReplayDivergenceStatus::Diverged => {
                if self.first_divergence.is_none() {
                    return Err(anyhow!(
                        "runtime replay diverged evidence requires firstDivergence"
                    ));
                }
                if self.expected == self.actual {
                    return Err(anyhow!(
                        "runtime replay diverged evidence requires expected and actual digests to differ"
                    ));
                }
            }
        }
        if let Some(first_divergence) = &self.first_divergence {
            first_divergence.validate()?;
        }
        self.policy.validate("divergence")?;
        Ok(())
    }
}

impl RuntimeReplayGeneratedStatePolicy {
    fn validate(&self, artifact_kind: &str) -> Result<()> {
        match self.root_kind {
            RuntimeSaveRootKind::GeneratedEvidence => {
                validate_evidence_artifact_path(&self.artifact_path)?;
                let expected_suffix = match artifact_kind {
                    "digest" => ".digest.json",
                    "divergence" => ".divergence.json",
                    _ => ".json",
                };
                if !self
                    .artifact_path
                    .starts_with("evidence/runtime-state/replay/")
                    || !self.artifact_path.ends_with(expected_suffix)
                {
                    return Err(anyhow!(
                        "runtime replay policy artifactPath must be evidence/runtime-state/replay/<frame>{expected_suffix} for generated evidence"
                    ));
                }
            }
            RuntimeSaveRootKind::LocalGeneratedState => {
                validate_runtime_replay_local_generated_path(&self.artifact_path)?;
            }
        }
        if self.trusted_writer != "rust-local-scenario-runner-v1" {
            return Err(anyhow!(
                "runtime replay policy trustedWriter must be rust-local-scenario-runner-v1"
            ));
        }
        if self.browser_write_access != RuntimeSaveBrowserWriteAccess::None {
            return Err(anyhow!(
                "runtime replay policy browserWriteAccess must be none for trusted replay artifacts"
            ));
        }
        require_text("runtime replay policy retention", &self.retention)?;
        Ok(())
    }
}

pub trait CdpTransport {
    fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value>;
}

pub struct CdpClient<T> {
    transport: T,
}

impl<T: CdpTransport> CdpClient<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn into_transport(self) -> T {
        self.transport
    }

    pub fn navigate(&mut self, url: &str) -> Result<CdpNavigateResult> {
        require_text("navigation URL", url)?;
        let result = self
            .transport
            .send_command("Page.navigate", json!({ "url": url }))?;
        if let Some(error_text) = result.get("errorText").and_then(|value| value.as_str()) {
            return Err(anyhow!("CDP navigation failed: {error_text}"));
        }
        Ok(CdpNavigateResult {
            frame_id: result
                .get("frameId")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            loader_id: result
                .get("loaderId")
                .and_then(|value| value.as_str())
                .map(str::to_string),
        })
    }

    pub fn enable_page(&mut self) -> Result<()> {
        self.transport.send_command("Page.enable", json!({}))?;
        Ok(())
    }

    pub fn add_script_to_evaluate_on_new_document(&mut self, source: &str) -> Result<()> {
        require_text("CDP preload script", source)?;
        self.transport.send_command(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({ "source": source }),
        )?;
        Ok(())
    }

    pub fn bring_page_to_front(&mut self) -> Result<()> {
        self.transport
            .send_command("Page.bringToFront", json!({}))?;
        Ok(())
    }

    pub fn capture_screenshot_png(&mut self) -> Result<Vec<u8>> {
        let result = self
            .transport
            .send_command("Page.captureScreenshot", json!({ "format": "png" }))?;
        let data = result
            .get("data")
            .and_then(|value| value.as_str())
            .ok_or_else(|| anyhow!("CDP screenshot response missing data"))?;
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .context("failed to decode CDP screenshot data")
    }

    pub fn enable_performance(&mut self) -> Result<()> {
        self.transport
            .send_command("Performance.enable", json!({}))?;
        Ok(())
    }

    pub fn performance_metrics(&mut self) -> Result<serde_json::Value> {
        self.transport
            .send_command("Performance.getMetrics", json!({}))
    }

    pub fn evaluate_json(&mut self, expression: &str) -> Result<serde_json::Value> {
        self.evaluate_json_with_await(expression, false)
    }

    pub fn evaluate_json_await(&mut self, expression: &str) -> Result<serde_json::Value> {
        self.evaluate_json_with_await(expression, true)
    }

    fn evaluate_json_with_await(
        &mut self,
        expression: &str,
        await_promise: bool,
    ) -> Result<serde_json::Value> {
        require_text("CDP Runtime.evaluate expression", expression)?;
        let result = self.transport.send_command(
            "Runtime.evaluate",
            json!({
                "expression": expression,
                "returnByValue": true,
                "awaitPromise": await_promise
            }),
        )?;
        if let Some(exception) = result.get("exceptionDetails") {
            return Err(anyhow!("CDP runtime evaluation failed: {exception}"));
        }
        Ok(result
            .get("result")
            .and_then(|remote_object| remote_object.get("value"))
            .cloned()
            .unwrap_or(serde_json::Value::Null))
    }
}

pub struct WebSocketCdpTransport {
    socket: tungstenite::WebSocket<std::net::TcpStream>,
    next_id: u64,
}

impl WebSocketCdpTransport {
    pub fn connect(config: &CdpConnectionConfig) -> Result<Self> {
        let request = config
            .target_ws_url
            .as_str()
            .into_client_request()
            .context("failed to build CDP WebSocket request")?;
        let endpoint = CdpWebSocketEndpoint::parse(&config.target_ws_url)?;
        let stream = endpoint.connect(config.io_timeout)?;
        let (mut socket, _) = tungstenite::client(request, stream)
            .with_context(|| format!("failed to connect to CDP target {}", config.target_ws_url))?;
        set_tcp_stream_timeouts(socket.get_mut(), config.io_timeout)?;
        Ok(Self { socket, next_id: 1 })
    }
}

impl CdpTransport for WebSocketCdpTransport {
    fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        require_text("CDP method", method)?;
        let id = self.next_id;
        self.next_id += 1;
        let request = json!({
            "id": id,
            "method": method,
            "params": params,
        });
        let request_body =
            serde_json::to_string(&request).context("failed to serialize CDP request")?;
        self.socket
            .send(tungstenite::Message::Text(request_body))
            .context("failed to send CDP request")?;

        loop {
            let message = self.socket.read().context("failed to read CDP response")?;
            let tungstenite::Message::Text(body) = message else {
                continue;
            };
            let response: serde_json::Value =
                serde_json::from_str(&body).context("failed to parse CDP response")?;
            if response.get("id").and_then(|value| value.as_u64()) != Some(id) {
                continue;
            }
            if let Some(error) = response.get("error") {
                return Err(anyhow!("CDP command {method} failed: {error}"));
            }
            return Ok(response.get("result").cloned().unwrap_or_else(|| json!({})));
        }
    }
}

pub fn run_browser_smoke_pool(config: &BrowserSmokePoolConfig) -> BrowserSmokePoolResult {
    let worker_configs = match config.worker_configs() {
        Ok(worker_configs) => worker_configs,
        Err(error) => {
            return BrowserSmokePoolResult {
                workers: config.workers,
                succeeded: 0,
                failed: 1,
                outcomes: vec![BrowserSmokeWorkerOutcome {
                    worker_id: "pool".to_string(),
                    ok: false,
                    screenshot_path: None,
                    error: Some(error.to_string()),
                }],
            };
        }
    };

    let mut setup_failures = Vec::new();
    let worker_configs: Vec<_> = worker_configs
        .into_iter()
        .filter_map(|mut worker_config| {
            if config.workers > 1 {
                match create_cdp_page_target(&worker_config.debugging_http_url, "about:blank") {
                    Ok(connection) => {
                        worker_config.target_ws_url = Some(connection.target_ws_url);
                    }
                    Err(error) => {
                        let error_message = error.to_string();
                        let failure_path = write_browser_worker_failure_artifact(
                            &worker_config,
                            "target_setup",
                            &error_message,
                        )
                        .ok();
                        let _ = append_ledger_event(
                            &worker_config.run_dir,
                            "browser.worker.failed",
                            "browser-smoke",
                            json!({
                                "worker_id": worker_config.worker_id.as_str(),
                                "error": error_message,
                                "phase": "target_setup",
                                "failure_path": failure_path
                            }),
                        );
                        setup_failures.push(BrowserSmokeWorkerOutcome {
                            worker_id: worker_config.worker_id.as_str().to_string(),
                            ok: false,
                            screenshot_path: None,
                            error: Some(error_message),
                        });
                        return None;
                    }
                }
            }
            Some(worker_config)
        })
        .collect();

    let handles: Vec<_> = worker_configs
        .into_iter()
        .map(|worker_config| {
            thread::spawn(move || {
                let worker_id = worker_config.worker_id.as_str().to_string();
                match run_browser_smoke(&worker_config) {
                    Ok(result) => BrowserSmokeWorkerOutcome {
                        worker_id,
                        ok: true,
                        screenshot_path: Some(result.screenshot_path),
                        error: None,
                    },
                    Err(error) => BrowserSmokeWorkerOutcome {
                        worker_id,
                        ok: false,
                        screenshot_path: None,
                        error: Some(error.to_string()),
                    },
                }
            })
        })
        .collect();

    let mut outcomes = setup_failures;
    outcomes.reserve(handles.len());
    for handle in handles {
        match handle.join() {
            Ok(outcome) => outcomes.push(outcome),
            Err(_) => outcomes.push(BrowserSmokeWorkerOutcome {
                worker_id: "unknown".to_string(),
                ok: false,
                screenshot_path: None,
                error: Some("browser smoke worker panicked".to_string()),
            }),
        }
    }
    outcomes.sort_by(|left, right| left.worker_id.cmp(&right.worker_id));
    let succeeded = outcomes.iter().filter(|outcome| outcome.ok).count();
    let failed = outcomes.len().saturating_sub(succeeded);
    BrowserSmokePoolResult {
        workers: config.workers,
        succeeded,
        failed,
        outcomes,
    }
}

pub fn run_browser_smoke(config: &BrowserSmokeConfig) -> Result<BrowserSmokeResult> {
    append_ledger_event(
        &config.run_dir,
        "browser.worker.started",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "debugging_http_url": config.debugging_http_url
        }),
    )?;

    let result = run_browser_smoke_inner(config);
    match &result {
        Ok(smoke) => {
            append_ledger_event(
                &config.run_dir,
                "browser.worker.completed",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "screenshot_path": smoke.screenshot_path.to_string_lossy()
                }),
            )?;
        }
        Err(error) => {
            let error_message = error.to_string();
            let failure_path =
                write_browser_worker_failure_artifact(config, "worker_run", &error_message).ok();
            let _ = append_ledger_event(
                &config.run_dir,
                "browser.worker.failed",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "error": error_message,
                    "phase": "worker_run",
                    "failure_path": failure_path
                }),
            );
        }
    }
    result
}

const RUNTIME_PROBE_CONTRACT_NAME: &str = "ouroforge-runtime-probe";
const RUNTIME_PROBE_MAX_ATTEMPTS: u32 = 20;
const RUNTIME_PROBE_RETRY_INTERVAL: Duration = Duration::from_millis(50);

fn write_browser_worker_failure_artifact(
    config: &BrowserSmokeConfig,
    phase: &str,
    error: &str,
) -> Result<String> {
    validate_path_component("browser worker failure phase", phase)?;
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.failure_path(suffix);
    fs::create_dir_all(config.run_dir.join(config.worker_id.evidence_dir())).with_context(
        || {
            format!(
                "failed to create worker evidence directory {}",
                config
                    .run_dir
                    .join(config.worker_id.evidence_dir())
                    .display()
            )
        },
    )?;
    let target_binding = browser_worker_effective_target_binding(config);
    let failure = json!({
        "artifact": "browser_worker_failure",
        "worker_id": config.worker_id.as_str(),
        "run_id": run_id_from_run_dir(&config.run_dir),
        "worker_session_id": format!("{}:{}", run_id_from_run_dir(&config.run_dir), config.worker_id.as_str()),
        "phase": phase,
        "error": error,
        "url": config.url,
        "debugging_http_url": config.debugging_http_url,
        "execution_boundary": "openchrome_cdp",
        "cdp_transport": "chrome_devtools_protocol",
        "target_binding": target_binding.clone(),
        "target_selection": target_binding,
        "configured_target_selection": config.target_selection.label(),
        "target_ws_url_bound": config.target_ws_url.is_some(),
        "bounded": true
    });
    write_json(&config.run_dir.join(&rel_path), &failure)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-worker-failure-{}-{phase}-{suffix}",
            config.worker_id.as_str()
        ),
        "application/json",
        &rel_path,
        browser_worker_evidence_metadata(
            config,
            "browser_worker_failure",
            json!({
                "phase": phase,
                "bounded": true
            }),
        ),
    )?;
    Ok(rel_path)
}

fn capture_runtime_probe<T: CdpTransport>(
    config: &BrowserSmokeConfig,
    client: &mut CdpClient<T>,
) -> Result<()> {
    // The probe API (`window.__OUROFORGE__`) only appears once the page's bundle
    // has loaded, which can take longer than the fixed post-navigation settle on
    // slow local/CI loads. Retry the availability check until it appears (or the
    // budget is exhausted) instead of skipping after a single attempt, so we
    // don't record `browser.probe.skipped` for a page that was merely still
    // loading.
    let mut available = json!(false);
    for attempt in 0..RUNTIME_PROBE_MAX_ATTEMPTS {
        available = client.evaluate_json(
            "Boolean(window.__OUROFORGE__ && typeof window.__OUROFORGE__.getWorldState === 'function' && typeof window.__OUROFORGE__.getFrameStats === 'function')",
        )?;
        if available == json!(true) {
            break;
        }
        if attempt + 1 < RUNTIME_PROBE_MAX_ATTEMPTS {
            std::thread::sleep(RUNTIME_PROBE_RETRY_INTERVAL);
        }
    }
    if available != json!(true) {
        append_ledger_event(
            &config.run_dir,
            "browser.probe.skipped",
            "browser-smoke",
            json!({
                "worker_id": config.worker_id.as_str(),
                "url": config.url,
                "reason": "window.__OUROFORGE__ probe API not found",
                "optional": true
            }),
        )?;
        return Ok(());
    }

    let world_state = capture_runtime_probe_value(
        config,
        client,
        "world-state",
        "getWorldState",
        "window.__OUROFORGE__.getWorldState()",
    )?;
    capture_scene3d_probe_artifacts(config, &world_state)?;
    capture_runtime_probe_value(
        config,
        client,
        "frame-stats",
        "getFrameStats",
        "window.__OUROFORGE__.getFrameStats()",
    )?;
    Ok(())
}

fn capture_runtime_probe_value<T: CdpTransport>(
    config: &BrowserSmokeConfig,
    client: &mut CdpClient<T>,
    artifact_name: &str,
    call_name: &str,
    expression: &str,
) -> Result<serde_json::Value> {
    let value = client.evaluate_json(expression)?;
    write_runtime_probe_artifact(config, artifact_name, call_name, &value)
}

fn write_runtime_probe_artifact(
    config: &BrowserSmokeConfig,
    artifact_name: &str,
    call_name: &str,
    value: &serde_json::Value,
) -> Result<serde_json::Value> {
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.probe_json_path(artifact_name, suffix);
    fs::create_dir_all(config.run_dir.join(config.worker_id.evidence_dir())).with_context(
        || {
            format!(
                "failed to create worker evidence directory {}",
                config
                    .run_dir
                    .join(config.worker_id.evidence_dir())
                    .display()
            )
        },
    )?;
    write_json(&config.run_dir.join(&rel_path), value)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-probe-{artifact_name}-{}-{suffix}",
            config.worker_id.as_str()
        ),
        "application/json",
        &rel_path,
        browser_worker_evidence_metadata(
            config,
            artifact_name,
            json!({
                "probe_call": call_name,
                "probe_contract": {
                    "name": RUNTIME_PROBE_CONTRACT_NAME,
                    "version": RUNTIME_PROBE_CONTRACT_VERSION
                }
            }),
        ),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.probe.captured",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "probe_call": call_name,
            "path": rel_path,
            "probe_contract": {
                "name": RUNTIME_PROBE_CONTRACT_NAME,
                "version": RUNTIME_PROBE_CONTRACT_VERSION
            }
        }),
    )?;
    Ok(value.clone())
}

fn capture_scene3d_probe_artifacts(
    config: &BrowserSmokeConfig,
    world_state: &serde_json::Value,
) -> Result<()> {
    let scene_kind_3d = world_state
        .get("sceneKind")
        .or_else(|| world_state.get("scene_kind"))
        .and_then(|value| value.as_str())
        == Some("3d");
    for (field, artifact_name) in [
        ("scene3dProbe", "scene3d-probe"),
        ("scene3dTransforms", "scene3d-transforms"),
    ] {
        match world_state.get(field) {
            Some(value) if value.is_object() => {
                write_runtime_probe_artifact(config, artifact_name, field, value)?;
            }
            Some(value) if scene_kind_3d => {
                let malformed = json!({
                    "schemaVersion": format!("ouroforge.{artifact_name}.capture.v1"),
                    "present": false,
                    "status": "malformed",
                    "field": field,
                    "observedType": json_value_type(value),
                    "boundedObserved": value,
                    "readOnlyInspection": {
                        "trustedEmitter": "browser-smoke-runtime-probe-capture",
                        "browserStudioMode": "read-only 3D runtime probe capture",
                        "disallowedActions": ["trusted writes", "command bridge", "scene mutation", "trusted persistence"]
                    }
                });
                write_runtime_probe_artifact(config, artifact_name, field, &malformed)?;
            }
            None if scene_kind_3d => {
                let missing = json!({
                    "schemaVersion": format!("ouroforge.{artifact_name}.capture.v1"),
                    "present": false,
                    "status": "missing",
                    "field": field,
                    "emptyState": format!("{field} is missing from 3D world-state probe capture."),
                    "readOnlyInspection": {
                        "trustedEmitter": "browser-smoke-runtime-probe-capture",
                        "browserStudioMode": "read-only 3D runtime probe capture",
                        "disallowedActions": ["trusted writes", "command bridge", "scene mutation", "trusted persistence"]
                    }
                });
                write_runtime_probe_artifact(config, artifact_name, field, &missing)?;
            }
            _ => {}
        }
    }
    Ok(())
}

const CONSOLE_CAPTURE_SCRIPT: &str = r#"
(() => {
  if (window.__OUROFORGE_CONSOLE_INSTALLED__) return;
  window.__OUROFORGE_CONSOLE_INSTALLED__ = true;
  window.__OUROFORGE_CONSOLE__ = [];
  const levels = ['debug', 'info', 'log', 'warn', 'error'];
  for (const level of levels) {
    const original = console[level] && console[level].bind(console);
    console[level] = (...args) => {
      try {
        var MAX_TEXT = 2048;
        var rendered = args.map((arg) => {
          if (typeof arg === 'string') return arg;
          try { return JSON.stringify(arg); } catch (_) { return String(arg); }
        }).join(' ');
        var truncated = rendered.length > MAX_TEXT;
        if (truncated) rendered = rendered.slice(0, MAX_TEXT);
        window.__OUROFORGE_CONSOLE__.push({
          level,
          text: rendered,
          truncated,
          argCount: args.length,
          timestampMs: Math.round(performance.now())
        });
        if (window.__OUROFORGE_CONSOLE__.length > 100) window.__OUROFORGE_CONSOLE__.shift();
      } catch (_) {}
      if (original) original(...args);
    };
  }
})();
"#;

fn install_console_capture<T: CdpTransport>(client: &mut CdpClient<T>) -> Result<()> {
    client.add_script_to_evaluate_on_new_document(CONSOLE_CAPTURE_SCRIPT)
}

fn capture_console_log<T: CdpTransport>(
    config: &BrowserSmokeConfig,
    client: &mut CdpClient<T>,
) -> Result<Option<String>> {
    let logs = client.evaluate_json("window.__OUROFORGE_CONSOLE__ || []")?;
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.console_log_path(suffix);
    fs::create_dir_all(config.run_dir.join(config.worker_id.evidence_dir())).with_context(
        || {
            format!(
                "failed to create worker evidence directory {}",
                config
                    .run_dir
                    .join(config.worker_id.evidence_dir())
                    .display()
            )
        },
    )?;
    write_json(&config.run_dir.join(&rel_path), &logs)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("browser-console-{}-{suffix}", config.worker_id.as_str()),
        "application/json",
        &rel_path,
        browser_worker_evidence_metadata(
            config,
            "console_log",
            json!({
                "bounded": true,
                "limit": 100
            }),
        ),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.capture.console",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "path": rel_path,
            "bounded": true,
            "limit": 100
        }),
    )?;
    Ok(Some(rel_path))
}

fn write_worker_cdp_trace_summary(
    config: &BrowserSmokeConfig,
    navigation: &CdpNavigateResult,
    performance_metric_count: Option<usize>,
) -> Result<String> {
    let suffix = unix_millis()?;
    let rel_path = config.worker_id.cdp_trace_summary_path(suffix);
    let mut events = vec![json!({
        "name": "Page.navigate",
        "frameIdPresent": navigation.frame_id.is_some(),
        "loaderIdPresent": navigation.loader_id.is_some()
    })];
    if let Some(metric_count) = performance_metric_count {
        events.push(json!({
            "name": "Performance.getMetrics",
            "metricCount": metric_count
        }));
    }
    write_json(
        &config.run_dir.join(&rel_path),
        &json!({
            "bounded": true,
            "limit": 32,
            "source": "cdp-summary",
            "workerId": config.worker_id.as_str(),
            "events": events
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-cdp-trace-summary-{}-{suffix}",
            config.worker_id.as_str()
        ),
        "application/json",
        &rel_path,
        browser_worker_evidence_metadata(
            config,
            "cdp_trace_summary",
            json!({
                "bounded": true,
                "limit": 32
            }),
        ),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.capture.cdp_trace_summary",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "path": rel_path,
            "bounded": true,
            "limit": 32
        }),
    )?;
    Ok(rel_path)
}

fn run_browser_smoke_inner(config: &BrowserSmokeConfig) -> Result<BrowserSmokeResult> {
    let connection = if let Some(target_ws_url) = &config.target_ws_url {
        CdpConnectionConfig::new(target_ws_url.clone())?
    } else {
        let targets = read_cdp_targets(&config.debugging_http_url)?;
        select_page_target(&targets, &config.target_selection)?
    };
    let transport = WebSocketCdpTransport::connect(&connection)?;
    let mut client = CdpClient::new(transport);

    client.enable_page()?;
    install_console_capture(&mut client)?;
    let _ = client.bring_page_to_front();
    let navigation = client.navigate(&config.url)?;
    append_ledger_event(
        &config.run_dir,
        "browser.navigation.completed",
        "browser-smoke",
        json!({
            "worker_id": config.worker_id.as_str(),
            "url": config.url,
            "frame_id": navigation.frame_id,
            "loader_id": navigation.loader_id
        }),
    )?;

    std::thread::sleep(Duration::from_millis(300));
    capture_console_log(config, &mut client)?;
    capture_runtime_probe(config, &mut client)?;
    let _ = client.bring_page_to_front();
    let screenshot = client.capture_screenshot_png()?;
    let artifact_id_suffix = unix_millis()?;
    let worker_evidence_dir = config.worker_id.evidence_dir();
    fs::create_dir_all(config.run_dir.join(&worker_evidence_dir)).with_context(|| {
        format!(
            "failed to create worker evidence directory {}",
            config.run_dir.join(&worker_evidence_dir).display()
        )
    })?;
    let screenshot_rel_path =
        format!("{worker_evidence_dir}/browser-smoke-{artifact_id_suffix}.png");
    let screenshot_path = config.run_dir.join(&screenshot_rel_path);
    fs::write(&screenshot_path, screenshot)
        .with_context(|| format!("failed to write screenshot {}", screenshot_path.display()))?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "browser-smoke-screenshot-{}-{artifact_id_suffix}",
            config.worker_id.as_str()
        ),
        "image/png",
        &screenshot_rel_path,
        browser_worker_evidence_metadata(config, "screenshot", json!({})),
    )?;
    append_ledger_event(
        &config.run_dir,
        "browser.capture.screenshot",
        "browser-smoke",
        json!({ "worker_id": config.worker_id.as_str(), "path": screenshot_rel_path }),
    )?;

    let mut performance_metric_count = None;
    match client
        .enable_performance()
        .and_then(|_| client.performance_metrics())
    {
        Ok(metrics) => {
            performance_metric_count = Some(count_cdp_metrics(&metrics));
            let metrics_rel_path = config.worker_id.performance_metrics_path(unix_millis()?);
            let metrics_path = config.run_dir.join(&metrics_rel_path);
            write_json(&metrics_path, &metrics)?;
            let _ = add_evidence_artifact(
                &config.run_dir,
                &format!(
                    "browser-smoke-performance-{}-{}",
                    config.worker_id.as_str(),
                    unix_millis()?
                ),
                "application/json",
                &metrics_rel_path,
                browser_worker_evidence_metadata(
                    config,
                    "performance_metrics",
                    json!({
                        "optional": true,
                        "bounded": true
                    }),
                ),
            );
            append_ledger_event(
                &config.run_dir,
                "browser.capture.performance",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "path": metrics_rel_path,
                    "optional": true
                }),
            )?;
        }
        Err(error) => {
            append_ledger_event(
                &config.run_dir,
                "browser.capture.performance.skipped",
                "browser-smoke",
                json!({
                    "worker_id": config.worker_id.as_str(),
                    "error": error.to_string(),
                    "optional": true
                }),
            )?;
        }
    }

    write_worker_cdp_trace_summary(config, &navigation, performance_metric_count)?;

    Ok(BrowserSmokeResult { screenshot_path })
}

pub fn evolve_run(run_dir: impl AsRef<Path>) -> Result<EvolveSummary> {
    let run_dir = run_dir.as_ref();
    append_ledger_event(run_dir, "evolve.started", "evolve-cli", json!({}))?;

    let verdict_input = fs::read_to_string(run_dir.join("verdict.json"))
        .context("failed to read verdict for evolve")?;
    let verdict: serde_json::Value =
        serde_json::from_str(&verdict_input).context("failed to parse verdict for evolve")?;
    let verdict_status = verdict["status"].as_str().unwrap_or("unknown");

    if verdict_status != "failed" {
        let summary = EvolveSummary {
            status: "noop".to_string(),
            proposals_created: 0,
            proposal_ids: Vec::new(),
            classification_ids: Vec::new(),
            patch_draft_ids: Vec::new(),
            reason: format!("verdict status is {verdict_status}; evolve v0 only proposes mutations for failed runs"),
        };
        append_ledger_event(
            run_dir,
            "evolve.completed",
            "evolve-cli",
            json!({ "status": summary.status, "proposals_created": 0 }),
        )?;
        update_journal(run_dir)?;
        return Ok(summary);
    }

    let evidence = read_evidence_index(run_dir)?;
    let failures = verdict["failures"].as_array().cloned().unwrap_or_default();
    let mut proposal_ids = Vec::new();
    // Classify every failure first, then choose the primary classification by backlog
    // severity across ALL classifications (not just the first). Selecting the first
    // classification before consulting the backlog starves a later classification's
    // higher-severity backlog item. #1293
    let classification_artifact = classify_mutation_failures(run_dir, &[])?;
    let Some(classification) =
        select_primary_classification(run_dir, &classification_artifact.classifications)
    else {
        return complete_evolve_without_proposal(
            run_dir,
            "missing-classification",
            "failed verdict did not produce a mutation classification; no mutation proposal was fabricated".to_string(),
        );
    };
    // Use the failure that produced the selected classification (classification-{N}
    // maps to failures[N-1]) so the proposal's evidence and rationale stay coherent
    // with the chosen backlog item.
    let failure = failures
        .get(classification_failure_index(&classification.id))
        .or_else(|| failures.first())
        .cloned()
        .unwrap_or_else(|| json!({ "kind": "failed_verdict" }));
    let evidence_refs = collect_classification_evidence_refs(&failure, &verdict);
    let (category, _) = classify_failure_category(&failure, &verdict, "", &evidence_refs);
    let gate_category = infer_mutation_proposal_gate_category(&failure, &verdict, &category);
    let evidence_state = infer_mutation_proposal_evidence_state(&evidence_refs, &evidence);
    if matches!(gate_category, MutationProposalGateCategory::Unsupported)
        && matches!(evidence_state, MutationProposalEvidenceState::Linked)
    {
        return complete_evolve_without_proposal(
            run_dir,
            mutation_proposal_gate_category_label(&gate_category),
            "failed verdict uses an unsupported mutation proposal gate; no mutation proposal was fabricated".to_string(),
        );
    }
    let Some(evidence_id) = select_evidence_id_for_failure(&evidence, &failure, &verdict) else {
        return complete_evolve_without_proposal(
            run_dir,
            mutation_proposal_evidence_state_label(&evidence_state),
            format!(
                "failed verdict has {} justifying evidence; no mutation proposal was fabricated",
                mutation_proposal_evidence_state_label(&evidence_state)
            ),
        );
    };
    let selection = select_mutation_proposal_strategy(run_dir, classification, &evidence)?;
    let Some(selection) = selection else {
        return complete_evolve_without_proposal(
            run_dir,
            "backlog-only",
            format!(
                "failure classification `{}` is backlog-only; no mutation proposal was fabricated",
                mutation_classification_category_label(&classification.category)
            ),
        );
    };
    let proposal = create_mutation_proposal(
        run_dir,
        MutationProposalInput {
            reason: format!(
                "Deterministic evolve v0 placeholder for verdict failure `{}`",
                failure["kind"].as_str().unwrap_or("failed_verdict")
            ),
            evidence_id,
            target: mutation_selection_target_path(&selection.bounded_mutation_type).to_string(),
            path: mutation_selection_path(&selection.bounded_mutation_type).to_string(),
            from: "current evidence-linked failing criteria".to_string(),
            to: "review evidence and adjust the next explicit implementation issue".to_string(),
        },
    )?;
    let proposal_id = proposal.id.clone();
    let existing_journal = fs::read_to_string(run_dir.join("journal.md")).unwrap_or_default();
    let rationale = build_mutation_proposal_rationale(
        &proposal,
        &failure,
        &verdict,
        &evidence,
        &existing_journal,
        &selection,
    )?;
    attach_mutation_proposal_rationale(run_dir, &proposal_id, rationale)?;
    proposal_ids.push(proposal_id);
    update_journal(run_dir)?;
    let classification_artifact = classify_mutation_failures(run_dir, &proposal_ids)?;
    let classification_ids = classification_artifact
        .classifications
        .iter()
        .map(|classification| classification.id.clone())
        .collect::<Vec<_>>();
    let patch_draft_artifact = generate_patch_drafts(run_dir)?;
    let patch_draft_ids = patch_draft_artifact
        .drafts
        .iter()
        .map(|draft| draft.id.clone())
        .collect::<Vec<_>>();

    let summary = EvolveSummary {
        status: "proposed".to_string(),
        proposals_created: proposal_ids.len(),
        proposal_ids,
        classification_ids,
        patch_draft_ids,
        reason: "failed verdict produced deterministic placeholder mutation proposal".to_string(),
    };
    append_ledger_event(
        run_dir,
        "evolve.completed",
        "evolve-cli",
        json!({
            "status": summary.status,
            "proposals_created": summary.proposals_created,
            "proposal_ids": summary.proposal_ids,
            "classification_ids": summary.classification_ids,
            "patch_draft_ids": summary.patch_draft_ids
        }),
    )?;
    Ok(summary)
}

fn select_evidence_id_for_failure(
    evidence: &EvidenceIndex,
    failure: &serde_json::Value,
    verdict: &serde_json::Value,
) -> Option<String> {
    for key in [
        "path",
        "evidence_path",
        "evidence_ref",
        "model_ref",
        "world_state_ref",
        "comparison_ref",
    ] {
        if let Some(path) = failure.get(key).and_then(|value| value.as_str()) {
            if let Some(artifact) = evidence
                .artifacts
                .iter()
                .find(|artifact| artifact.path == path || artifact.id == path)
            {
                return Some(artifact.id.clone());
            }
        }
    }
    if let Some(paths) = failure
        .get("evidence_refs")
        .and_then(|value| value.as_array())
    {
        for path in paths.iter().filter_map(|value| value.as_str()) {
            if let Some(artifact) = evidence
                .artifacts
                .iter()
                .find(|artifact| artifact.path == path || artifact.id == path)
            {
                return Some(artifact.id.clone());
            }
        }
    }
    verdict
        .get("evidence_refs")
        .and_then(|value| value.as_array())
        .and_then(|refs| {
            refs.iter()
                .filter_map(|value| value.as_str())
                .find_map(|path| {
                    evidence
                        .artifacts
                        .iter()
                        .find(|artifact| artifact.path == path || artifact.id == path)
                        .map(|artifact| artifact.id.clone())
                })
        })
}

fn select_mutation_proposal_strategy(
    run_dir: &Path,
    classification: &MutationClassification,
    evidence: &EvidenceIndex,
) -> Result<Option<MutationProposalSelection>> {
    let Some(mapped_type) = bounded_mutation_type_for_classification(&classification.category)
    else {
        return Ok(None);
    };
    let backlog_path = run_dir.join("mutation/backlog.json");
    if !backlog_path.exists() {
        return Ok(Some(MutationProposalSelection {
            bounded_mutation_type: mapped_type,
            backlog_item_id: None,
            source: "classification-only".to_string(),
            reason: format!(
                "classified failure `{}` mapped directly because no mutation backlog artifact exists",
                mutation_classification_category_label(&classification.category)
            ),
            backlog_read_only: true,
        }));
    }

    let backlog = read_mutation_backlog_artifact(run_dir)?;
    let mut candidates = backlog
        .items
        .iter()
        .filter(|item| {
            item.classification_id == classification.id
                || item.failure_class == classification.category
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Err(anyhow!(
            "missing-backlog-ref: mutation backlog has no item for classification {}",
            classification.id
        ));
    }
    candidates.sort_by(|left, right| {
        severity_rank(&right.severity)
            .cmp(&severity_rank(&left.severity))
            .then_with(|| right.evidence_refs.len().cmp(&left.evidence_refs.len()))
            .then_with(|| left.id.cmp(&right.id))
    });
    let selected = candidates[0];
    if selected.classification_id != classification.id {
        return Err(anyhow!(
            "missing-classification: selected backlog item {} references {} instead of {}",
            selected.id,
            selected.classification_id,
            classification.id
        ));
    }
    if selected.bounded_mutation_type != mapped_type {
        return Err(anyhow!(
            "bounded-type-violation: classification {} maps to {} but backlog item {} requested {}",
            mutation_classification_category_label(&classification.category),
            mutation_proposal_bounded_type_label(&mapped_type),
            selected.id,
            mutation_proposal_bounded_type_label(&selected.bounded_mutation_type)
        ));
    }
    let stale_ref = selected.evidence_refs.iter().find(|evidence_ref| {
        !evidence
            .artifacts
            .iter()
            .any(|artifact| artifact.path == **evidence_ref || artifact.id == **evidence_ref)
    });
    if let Some(stale_ref) = stale_ref {
        return Err(anyhow!(
            "stale-ref: mutation backlog item {} references missing evidence {}",
            selected.id,
            stale_ref
        ));
    }
    Ok(Some(MutationProposalSelection {
        bounded_mutation_type: selected.bounded_mutation_type.clone(),
        backlog_item_id: Some(selected.id.clone()),
        source: "mutation/backlog.json".to_string(),
        reason: format!(
            "selected backlog item {} by severity {:?} and reproduction context `{}` without mutating backlog state",
            selected.id, selected.severity, selected.reproduction_context
        ),
        backlog_read_only: true,
    }))
}

fn build_mutation_proposal_rationale(
    proposal: &MutationProposal,
    failure: &serde_json::Value,
    verdict: &serde_json::Value,
    evidence: &EvidenceIndex,
    journal: &str,
    selection: &MutationProposalSelection,
) -> Result<MutationProposalRationale> {
    let evidence_refs = collect_classification_evidence_refs(failure, verdict);
    let (category, reason) = classify_failure_category(failure, verdict, journal, &evidence_refs);
    let failing_gate_category = infer_mutation_proposal_gate_category(failure, verdict, &category);
    let justifying_evidence_ref = select_justifying_evidence_ref(&evidence_refs, evidence);
    let evidence_state = infer_mutation_proposal_evidence_state(&evidence_refs, evidence);
    let (confidence, confidence_basis) =
        derive_mutation_proposal_confidence(&evidence_state, &failing_gate_category);
    let mut evidence_artifact_ids = Vec::new();
    push_unique_ref(&mut evidence_artifact_ids, &proposal.evidence_id);
    for evidence_ref in &evidence_refs {
        if let Some(artifact) = evidence
            .artifacts
            .iter()
            .find(|artifact| artifact.path == *evidence_ref || artifact.id == *evidence_ref)
        {
            push_unique_ref(&mut evidence_artifact_ids, &artifact.id);
        }
    }
    let mut scenario_result_refs = collect_scenario_result_refs(&evidence_refs, evidence);
    if scenario_result_refs.is_empty() {
        let verdict_evidence_refs = verdict
            .get("evidence_refs")
            .and_then(|value| value.as_array())
            .map(|refs| {
                refs.iter()
                    .filter_map(|value| value.as_str())
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        scenario_result_refs = collect_scenario_result_refs(&verdict_evidence_refs, evidence);
    }
    if scenario_result_refs.is_empty() {
        scenario_result_refs = evidence
            .artifacts
            .iter()
            .filter(|artifact| {
                artifact.path.contains("scenario-result")
                    || artifact
                        .metadata
                        .get("artifact")
                        .and_then(|value| value.as_str())
                        == Some("scenario_result")
            })
            .map(|artifact| artifact.path.clone())
            .collect();
    }
    let rationale = MutationProposalRationale {
        schema_version: "1".to_string(),
        failure_classification: mutation_classification_category_label(&category).to_string(),
        evidence_artifact_ids,
        scenario_result_refs,
        verdict_refs: vec!["verdict.json".to_string()],
        expected_effect: format!(
            "Review evidence-linked failure `{}` and adjust `{}` at `{}` only if review accepts the proposal.",
            failure["kind"].as_str().unwrap_or("failed_verdict"),
            proposal.target,
            proposal.path
        ),
        confidence,
        reasoning_summary: reason,
        allowed_mutation_type: mutation_allowed_type_for_selection(&selection.bounded_mutation_type),
        failing_gate_category: Some(failing_gate_category.clone()),
        justifying_evidence_ref,
        evidence_state: Some(evidence_state),
        confidence_basis: Some(confidence_basis),
        bounded_mutation_type: Some(selection.bounded_mutation_type.clone()),
        selection_backlog_item_id: selection.backlog_item_id.clone(),
        selection_source: Some(selection.source.clone()),
        selection_reason: Some(selection.reason.clone()),
        backlog_read_only: Some(selection.backlog_read_only),
    };
    rationale.validate(&proposal.evidence_id)?;
    Ok(rationale)
}

fn complete_evolve_without_proposal(
    run_dir: &Path,
    status: &str,
    reason: String,
) -> Result<EvolveSummary> {
    let classification_artifact = classify_mutation_failures(run_dir, &[])?;
    let classification_ids = classification_artifact
        .classifications
        .iter()
        .map(|classification| classification.id.clone())
        .collect::<Vec<_>>();
    let summary = EvolveSummary {
        status: status.to_string(),
        proposals_created: 0,
        proposal_ids: Vec::new(),
        classification_ids,
        patch_draft_ids: Vec::new(),
        reason,
    };
    append_ledger_event(
        run_dir,
        "evolve.completed",
        "evolve-cli",
        json!({
            "status": summary.status,
            "proposals_created": summary.proposals_created,
            "classification_ids": summary.classification_ids,
            "reason": summary.reason
        }),
    )?;
    update_journal(run_dir)?;
    Ok(summary)
}

fn select_justifying_evidence_ref(
    evidence_refs: &[String],
    evidence: &EvidenceIndex,
) -> Option<String> {
    evidence_refs
        .iter()
        .find(|evidence_ref| {
            evidence
                .artifacts
                .iter()
                .any(|artifact| artifact.path == **evidence_ref || artifact.id == **evidence_ref)
        })
        .cloned()
}

fn infer_mutation_proposal_evidence_state(
    evidence_refs: &[String],
    evidence: &EvidenceIndex,
) -> MutationProposalEvidenceState {
    if select_justifying_evidence_ref(evidence_refs, evidence).is_some() {
        MutationProposalEvidenceState::Linked
    } else if evidence_refs.is_empty()
        || evidence_refs
            .iter()
            .any(|evidence_ref| evidence_ref.contains("missing"))
    {
        MutationProposalEvidenceState::MissingEvidence
    } else {
        MutationProposalEvidenceState::StaleRef
    }
}

fn collect_scenario_result_refs(evidence_refs: &[String], evidence: &EvidenceIndex) -> Vec<String> {
    evidence_refs
        .iter()
        .filter(|path| {
            path.contains("scenario-result")
                || evidence.artifacts.iter().any(|artifact| {
                    artifact.path == **path
                        && artifact
                            .metadata
                            .get("artifact")
                            .and_then(|value| value.as_str())
                            == Some("scenario_result")
                })
        })
        .cloned()
        .collect()
}

pub fn classify_mutation_failures(
    run_dir: impl AsRef<Path>,
    proposal_ids: &[String],
) -> Result<MutationClassificationArtifact> {
    let run_dir = run_dir.as_ref();
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    let journal = fs::read_to_string(run_dir.join("journal.md"))
        .context("failed to read journal for mutation classification")?;
    let evidence = read_evidence_index(run_dir)?;
    let failures = verdict
        .get("failures")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let failures = if failures.is_empty() {
        vec![json!({
            "kind": "unknown",
            "summary": "failed verdict did not include structured failures"
        })]
    } else {
        failures
    };

    let mut classifications = Vec::new();
    for (index, failure) in failures.iter().enumerate() {
        let evidence_refs = collect_classification_evidence_refs(failure, &verdict);
        let scenario_result_refs = collect_scenario_result_refs(&evidence_refs, &evidence);
        let (category, reason) =
            classify_failure_category(failure, &verdict, &journal, &evidence_refs);
        let proposal_id = proposal_ids.get(index).cloned().or_else(|| {
            if proposal_ids.len() == 1 {
                proposal_ids.first().cloned()
            } else {
                None
            }
        });
        classifications.push(MutationClassification {
            id: format!("classification-{}", index + 1),
            proposal_id,
            category,
            lifecycle_state: MutationClassificationState::Classified,
            reason,
            evidence_refs,
            verdict_ref: "verdict.json".to_string(),
            journal_ref: "journal.md".to_string(),
            scenario_result_refs,
        });
    }

    let artifact = MutationClassificationArtifact {
        schema_version: "1".to_string(),
        run_id,
        classifications,
    };
    let path = write_mutation_classification_artifact(run_dir, &artifact)?;
    append_ledger_event(
        run_dir,
        "mutation.classified",
        "evolve-cli",
        json!({
            "path": path
                .strip_prefix(run_dir)
                .ok()
                .and_then(|path| path.to_str())
                .unwrap_or("mutation/classifications.json"),
            "classification_ids": artifact
                .classifications
                .iter()
                .map(|classification| classification.id.clone())
                .collect::<Vec<_>>()
        }),
    )?;
    Ok(artifact)
}

pub fn append_mutation_review_decision(
    run_dir: impl AsRef<Path>,
    input: MutationReviewDecisionInput,
) -> Result<MutationReviewDecision> {
    let run_dir = run_dir.as_ref();
    let mut artifact = read_mutation_review_artifact(run_dir)?;
    let next_index = artifact.decisions.len() + 1;
    let drafts = read_patch_draft_artifact(run_dir)?;
    let draft = drafts
        .drafts
        .iter()
        .find(|draft| draft.id == input.patch_draft_id)
        .ok_or_else(|| {
            anyhow!(
                "mutation review patch draft id not found: {}",
                input.patch_draft_id
            )
        })?;
    let proposals = read_mutation_proposals(run_dir)?;
    let proposal_ids = proposals
        .proposals
        .iter()
        .map(|proposal| proposal.id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let proposal_id = match input.proposal_id {
        Some(proposal_id) => {
            if !proposal_ids.contains(proposal_id.as_str()) {
                return Err(anyhow!(
                    "mutation review proposal id not found: {}",
                    proposal_id
                ));
            }
            if proposal_id.as_str() != draft.proposal_id.as_str() {
                return Err(anyhow!(
                    "mutation review proposal_id {} does not match patch draft {} proposal_id {}",
                    proposal_id,
                    draft.id,
                    draft.proposal_id
                ));
            }
            Some(proposal_id)
        }
        None => Some(draft.proposal_id.clone()),
    };
    let state = input.state;
    let decision = MutationReviewDecision {
        id: format!("review-decision-{next_index}"),
        patch_draft_id: input.patch_draft_id,
        proposal_id,
        state: state.clone(),
        decision_status: Some(state),
        reviewer_type: input
            .reviewer_type
            .or(Some(MutationReviewReviewerType::Human)),
        reason: input.reason,
        evidence_refs: input.evidence_refs,
        reviewer: input.reviewer,
        expected_hashes: input.expected_hashes,
        guardrail_checklist: input.guardrail_checklist.or(Some(Default::default())),
        source_patch_review: input.source_patch_review,
        decided_at_unix_ms: unix_millis()?,
    };
    let draft_ids = drafts
        .drafts
        .iter()
        .map(|draft| draft.id.as_str())
        .collect::<std::collections::HashSet<_>>();
    decision.validate(&draft_ids, &proposal_ids)?;
    artifact.decisions.push(decision.clone());
    write_mutation_review_artifact(run_dir, &artifact)?;
    append_ledger_event(
        run_dir,
        "mutation.review_decision",
        "mutation-review",
        json!({
            "decision_id": decision.id,
            "patch_draft_id": decision.patch_draft_id,
            "proposal_id": decision.proposal_id,
            "state": decision.state,
            "decision_status": decision.decision_status,
            "reviewer_type": decision.reviewer_type,
            "evidence_refs": decision.evidence_refs,
            "reviewer": decision.reviewer,
        }),
    )?;
    update_journal(run_dir)?;
    Ok(decision)
}

pub fn append_mutation_review_decision_from_path(
    run_or_draft_path: impl AsRef<Path>,
    state: MutationReviewState,
    reason: String,
    evidence_refs: Vec<String>,
    reviewer: String,
) -> Result<MutationReviewDecision> {
    append_mutation_review_decision_for_proposal_from_path(
        run_or_draft_path,
        None,
        state,
        reason,
        evidence_refs,
        reviewer,
        Some(MutationReviewReviewerType::Human),
    )
}

pub fn append_mutation_review_decision_for_proposal_from_path(
    run_or_draft_path: impl AsRef<Path>,
    proposal_id: Option<String>,
    state: MutationReviewState,
    reason: String,
    evidence_refs: Vec<String>,
    reviewer: String,
    reviewer_type: Option<MutationReviewReviewerType>,
) -> Result<MutationReviewDecision> {
    let run_dir = resolve_patch_sandbox_run_dir(run_or_draft_path.as_ref())?;
    let drafts = read_patch_draft_artifact(&run_dir)?;
    let patch_draft = match proposal_id.as_deref() {
        Some(proposal_id) => drafts
            .drafts
            .iter()
            .find(|draft| draft.proposal_id == proposal_id)
            .ok_or_else(|| anyhow!("mutation review proposal id not found: {proposal_id}"))?,
        None => drafts
            .drafts
            .first()
            .ok_or_else(|| anyhow!("mutation review requires at least one patch draft"))?,
    };
    let patch_draft_id = patch_draft.id.clone();
    let evidence_refs = if evidence_refs.is_empty() {
        default_mutation_review_evidence_refs(&run_dir, &patch_draft_id)?
    } else {
        evidence_refs
    };
    append_mutation_review_decision(
        run_dir,
        MutationReviewDecisionInput {
            patch_draft_id,
            proposal_id,
            state,
            reviewer_type,
            reason,
            evidence_refs,
            reviewer,
            expected_hashes: None,
            guardrail_checklist: Some(Default::default()),
            source_patch_review: None,
        },
    )
}

/// Validate that a regression promotion `scenarioResultPath` actually anchors a
/// scenario result artifact. Scenario results are written under
/// `evidence/scenarios/<scenario-id>/scenario-result.json` by legacy fixtures
/// and `scenario-result-*.json` by current scenario runs, so accepting any
/// `evidence/`, `mutation/`, or `sandbox/` ref (as the generic mutation review
/// ref validator does) would let a draft pass while pointing at an unrelated
/// artifact that downstream promotion/preview code cannot rely on.
fn validate_scenario_result_ref(reference: &str) -> Result<()> {
    validate_evidence_artifact_path(reference)?;
    if !is_scenario_result_artifact_path(reference) {
        return Err(anyhow!(
            "regression promotion scenarioResultPath must reference a scenario result under evidence/scenarios/ (scenario-result.json or scenario-result-*.json)"
        ));
    }
    Ok(())
}

pub fn generate_patch_drafts(run_dir: impl AsRef<Path>) -> Result<PatchDraftArtifact> {
    let run_dir = run_dir.as_ref();
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let proposals = read_mutation_proposals(run_dir)?.proposals;
    if proposals.is_empty() {
        return Err(anyhow!(
            "patch draft generation requires a mutation proposal"
        ));
    }
    let classifications = read_mutation_classification_artifact(run_dir)?;
    let mut drafts = Vec::new();
    for (index, proposal) in proposals.iter().enumerate() {
        validate_patch_draft_target_path(&proposal.target)
            .with_context(|| format!("unsupported mutation proposal target for {}", proposal.id))?;
        let classification = classifications
            .classifications
            .iter()
            .find(|classification| {
                classification.proposal_id.as_deref() == Some(proposal.id.as_str())
            })
            // Only fall back to positional matching for legacy classifications
            // that carry no proposal_id; otherwise an index hit could pair this
            // proposal with a classification that explicitly belongs to another.
            .or_else(|| {
                classifications
                    .classifications
                    .get(index)
                    .filter(|candidate| candidate.proposal_id.is_none())
            })
            .ok_or_else(|| {
                anyhow!(
                    "patch draft generation requires classification for proposal {}",
                    proposal.id
                )
            })?;
        let mut evidence_refs = classification.evidence_refs.clone();
        push_unique_ref(&mut evidence_refs, &proposal.evidence_id);
        let draft = PatchDraft {
            id: format!("patch-draft-{}", index + 1),
            proposal_id: proposal.id.clone(),
            classification_id: classification.id.clone(),
            lifecycle_state: PatchDraftState::Drafted,
            target_path: proposal.target.clone(),
            rationale: format!(
                "Draft derived from proposal {} and classification {} ({:?}).",
                proposal.id, classification.id, classification.category
            ),
            evidence_refs,
            draft_text: render_patch_draft_text(proposal, classification),
        };
        draft.validate()?;
        drafts.push(draft);
    }

    let artifact = PatchDraftArtifact {
        schema_version: "1".to_string(),
        run_id,
        drafts,
    };
    let path = write_patch_draft_artifact(run_dir, &artifact)?;
    append_ledger_event(
        run_dir,
        "mutation.drafted",
        "evolve-cli",
        json!({
            "path": path
                .strip_prefix(run_dir)
                .ok()
                .and_then(|path| path.to_str())
                .unwrap_or("mutation/patch-drafts.json"),
            "patch_draft_ids": artifact
                .drafts
                .iter()
                .map(|draft| draft.id.clone())
                .collect::<Vec<_>>()
        }),
    )?;
    Ok(artifact)
}

pub fn apply_patch_sandbox_from_path(
    run_or_draft_path: impl AsRef<Path>,
) -> Result<PatchSandboxApplicationResult> {
    let run_dir = resolve_patch_sandbox_run_dir(run_or_draft_path.as_ref())?;
    let drafts = read_patch_draft_artifact(&run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("patch sandbox requires at least one patch draft"))?
        .id
        .clone();
    let repo_root = git_repo_root()?;
    apply_patch_sandbox(&run_dir, &patch_draft_id, &repo_root, true)
}

pub fn orchestrate_evolve_rerun_from_path(
    run_or_draft_path: impl AsRef<Path>,
) -> Result<EvolveRerunOrchestration> {
    let run_dir = resolve_patch_sandbox_run_dir(run_or_draft_path.as_ref())?;
    let drafts = read_patch_draft_artifact(&run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("evolve rerun comparison requires at least one patch draft"))?
        .id
        .clone();
    let repo_root = git_repo_root()?;
    orchestrate_evolve_rerun(&run_dir, &patch_draft_id, &repo_root, true)
}

pub fn run_evolve_demo_lifecycle_from_path(
    run_or_demo_path: impl AsRef<Path>,
) -> Result<EvolveDemoLifecycleSummary> {
    let run_dir = run_or_demo_path.as_ref();
    if !run_dir.is_dir() {
        return Err(anyhow!(
            "evolve demo expects a run directory produced by the demo seed"
        ));
    }
    let repo_root = git_repo_root()?;
    run_evolve_demo_lifecycle(run_dir, &repo_root, true)
}

pub fn run_evolve_demo_lifecycle(
    run_dir: impl AsRef<Path>,
    repo_root: impl AsRef<Path>,
    run_verification: bool,
) -> Result<EvolveDemoLifecycleSummary> {
    let run_dir = run_dir.as_ref();
    if !run_dir.join("mutation/patch-drafts.json").is_file() {
        evolve_run(run_dir)?;
    }
    let classifications = read_mutation_classification_artifact(run_dir)?;
    let drafts = read_patch_draft_artifact(run_dir)?;
    let patch_draft_id = drafts
        .drafts
        .first()
        .ok_or_else(|| anyhow!("evolve demo requires at least one patch draft"))?
        .id
        .clone();
    let rerun = orchestrate_evolve_rerun(run_dir, &patch_draft_id, repo_root, run_verification)?;
    let reviews = read_mutation_review_artifact(run_dir)?;
    let manual_review_state = reviews
        .decisions
        .last()
        .map(|decision| decision.state.clone())
        .unwrap_or(MutationReviewState::PendingReview);
    let lifecycle_summary_path = "mutation/evolve-v1-demo-summary.json".to_string();
    let review_decision_artifact_path = if reviews.decisions.is_empty() {
        None
    } else {
        Some("mutation/review-decisions.json".to_string())
    };
    let run = read_json_value(run_dir.join("run.json"))?;
    let run_id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    let summary = EvolveDemoLifecycleSummary {
        schema_version: "1".to_string(),
        run_id,
        status: "lifecycle_evidence_ready".to_string(),
        classification_artifact_path: "mutation/classifications.json".to_string(),
        classification_ids: classifications
            .classifications
            .iter()
            .map(|classification| classification.id.clone())
            .collect(),
        patch_draft_artifact_path: "mutation/patch-drafts.json".to_string(),
        patch_draft_ids: drafts.drafts.iter().map(|draft| draft.id.clone()).collect(),
        sandbox_result_path: rerun.after.sandbox_result_path,
        rerun_evidence_path: rerun.evolve_evidence_path,
        comparison_artifact_path: rerun
            .comparison_artifact_path
            .ok_or_else(|| anyhow!("evolve demo requires comparison artifact"))?,
        manual_review_state,
        review_decision_artifact_path,
        lifecycle_summary_path,
        primary_git_status_before: rerun.primary_git_status_before,
        primary_git_status_after: rerun.primary_git_status_after,
        omitted_features: vec![
            "Manual review remains a separate explicit mutation review command.".to_string(),
            "No patch is applied or merged into the primary working tree.".to_string(),
        ],
    };
    write_json_atomic(
        &run_dir.join(&summary.lifecycle_summary_path),
        &json!(summary),
    )?;
    append_ledger_event(
        run_dir,
        "evolve.demo_lifecycle",
        "evolve-cli",
        json!({
            "lifecycle_summary_path": summary.lifecycle_summary_path,
            "classification_artifact_path": summary.classification_artifact_path,
            "patch_draft_artifact_path": summary.patch_draft_artifact_path,
            "sandbox_result_path": summary.sandbox_result_path,
            "comparison_artifact_path": summary.comparison_artifact_path,
            "manual_review_state": summary.manual_review_state,
        }),
    )?;
    Ok(summary)
}

pub fn orchestrate_evolve_rerun(
    run_dir: impl AsRef<Path>,
    patch_draft_id: &str,
    repo_root: impl AsRef<Path>,
    run_verification: bool,
) -> Result<EvolveRerunOrchestration> {
    let run_dir = run_dir.as_ref();
    let drafts = read_patch_draft_artifact(run_dir)?;
    let draft = drafts
        .drafts
        .iter()
        .find(|draft| draft.id == patch_draft_id)
        .ok_or_else(|| {
            anyhow!("patch draft id not found for rerun comparison: {patch_draft_id}")
        })?;
    let sandbox = apply_patch_sandbox(run_dir, patch_draft_id, repo_root, run_verification)?;
    let before = evolve_run_reference(run_dir)?;
    let after_run_id = format!("{}--sandbox-{}", before.run_id, patch_draft_id);
    let verification_evidence_refs = sandbox
        .verification
        .iter()
        .flat_map(|output| [output.stdout_path.clone(), output.stderr_path.clone()])
        .collect::<Vec<_>>();
    let after_run_dir = write_sandbox_after_run_reference(run_dir, &sandbox, &after_run_id)?;
    let comparison_path =
        write_run_comparison_artifact(run_dir, &after_run_dir, run_dir.join("mutation"))?;
    let comparison = compare_runs(run_dir, &after_run_dir)?;
    let comparison_artifact_path = run_relative_path(run_dir, &comparison_path)?;
    let final_classification =
        normalize_evolve_comparison_classification(&comparison.classification).to_string();
    let evolve_evidence_path = "mutation/rerun-orchestration.json".to_string();
    let result = EvolveRerunOrchestration {
        schema_version: "1".to_string(),
        source_run_id: before.run_id.clone(),
        patch_draft_id: draft.id.clone(),
        mutation_proposal_id: draft.proposal_id.clone(),
        mutation_classification_id: draft.classification_id.clone(),
        before,
        after: EvolveSandboxRunReference {
            run_id: after_run_id,
            sandbox_root: sandbox.sandbox_root,
            sandbox_result_path: sandbox.result_path,
            applied_target_path: sandbox.applied_target_path,
            verification_evidence_refs,
        },
        comparison_artifact_path: Some(comparison_artifact_path),
        final_classification: Some(final_classification),
        evolve_evidence_path,
        primary_git_status_before: sandbox.primary_git_status_before,
        primary_git_status_after: sandbox.primary_git_status_after,
    };
    let evidence_path = run_dir.join(&result.evolve_evidence_path);
    write_json_atomic(&evidence_path, &json!(result))?;
    append_ledger_event(
        run_dir,
        "mutation.rerun_orchestrated",
        "evolve-cli",
        json!({
            "patch_draft_id": result.patch_draft_id,
            "before_run_id": result.before.run_id,
            "after_run_id": result.after.run_id,
            "evolve_evidence_path": result.evolve_evidence_path,
            "comparison_artifact_path": result.comparison_artifact_path,
            "final_classification": result.final_classification,
        }),
    )?;
    Ok(result)
}

fn write_sandbox_after_run_reference(
    source_run_dir: &Path,
    sandbox: &PatchSandboxApplicationResult,
    after_run_id: &str,
) -> Result<PathBuf> {
    let after_run_dir = source_run_dir.join(&sandbox.sandbox_root).join("after-run");
    let evidence_dir = after_run_dir.join("evidence");
    fs::create_dir_all(&evidence_dir)
        .with_context(|| format!("failed to create {}", evidence_dir.display()))?;
    write_json(
        &after_run_dir.join("run.json"),
        &json!({
            "id": after_run_id,
            "source_run_id": sandbox.run_id,
            "sandbox_result_path": sandbox.result_path,
            "status": "sandbox_verified",
        }),
    )?;
    let verdict_status = if sandbox.lifecycle_state == PatchSandboxState::Verified {
        "passed"
    } else {
        "failed"
    };
    write_json(
        &after_run_dir.join("verdict.json"),
        &json!({
            "status": verdict_status,
            "summary": "Sandbox verification result used as evolve after-run reference.",
            "failures": [],
            "evidence_refs": [
                sandbox.result_path,
                sandbox.applied_draft_path,
            ],
            "metadata": {
                "evaluator": "ouroforge-evolve-rerun-v1",
                "sandbox_id": sandbox.sandbox_id,
                "patch_draft_id": sandbox.patch_draft_id,
            }
        }),
    )?;
    let mut artifacts = vec![
        EvidenceArtifact {
            id: "sandbox-result".to_string(),
            kind: "application/json".to_string(),
            path: "evidence/sandbox-result.json".to_string(),
            metadata: json!({ "artifact": "sandbox_result" }),
            added_at_unix_ms: unix_millis()?,
        },
        EvidenceArtifact {
            id: "applied-draft".to_string(),
            kind: "text/plain".to_string(),
            path: "evidence/applied-draft.txt".to_string(),
            metadata: json!({ "artifact": "applied_patch_draft" }),
            added_at_unix_ms: unix_millis()?,
        },
    ];
    for (index, output) in sandbox.verification.iter().enumerate() {
        artifacts.push(EvidenceArtifact {
            id: format!("sandbox-verification-{}", index + 1),
            kind: "text/plain".to_string(),
            path: format!("evidence/sandbox-verification-{}.txt", index + 1),
            metadata: json!({
                "artifact": "sandbox_verification",
                "command": output.command,
                "status": output.status,
                "stdout_path": output.stdout_path,
                "stderr_path": output.stderr_path,
            }),
            added_at_unix_ms: unix_millis()?,
        });
    }
    write_evidence_index(&after_run_dir, &EvidenceIndex { artifacts })?;
    Ok(after_run_dir)
}

pub fn apply_patch_sandbox(
    run_dir: impl AsRef<Path>,
    patch_draft_id: &str,
    repo_root: impl AsRef<Path>,
    run_verification: bool,
) -> Result<PatchSandboxApplicationResult> {
    let run_dir = run_dir.as_ref();
    let repo_root = repo_root.as_ref();
    let primary_git_status_before = git_status_short(repo_root)?;
    let plan = create_patch_sandbox_layout(run_dir, patch_draft_id)?;
    let drafts = read_patch_draft_artifact(run_dir)?;
    let draft = drafts
        .drafts
        .iter()
        .find(|draft| draft.id == patch_draft_id)
        .ok_or_else(|| anyhow!("patch draft id not found for sandbox: {patch_draft_id}"))?;

    let worktree = run_dir.join(&plan.layout.worktree_path);
    let evidence = run_dir.join(&plan.layout.evidence_path);
    copy_repo_tracked_files(repo_root, &worktree)?;

    let applied_target = worktree.join(&draft.target_path);
    ensure_path_inside(&worktree, &applied_target)?;
    if !applied_target.exists() {
        return Err(anyhow!(
            "patch sandbox target does not exist in sandbox worktree: {}",
            draft.target_path
        ));
    }
    fs::write(&applied_target, &draft.draft_text).with_context(|| {
        format!(
            "failed to apply patch draft {} to sandbox target {}",
            draft.id,
            applied_target.display()
        )
    })?;

    let applied_draft_path = evidence.join("applied-draft.txt");
    fs::write(&applied_draft_path, &draft.draft_text)
        .with_context(|| format!("failed to write {}", applied_draft_path.display()))?;

    let mut verification = Vec::new();
    if run_verification {
        for (index, command) in plan.verification_commands.iter().enumerate() {
            verification.push(run_sandbox_verification_command(
                &worktree,
                &evidence,
                index + 1,
                command,
            )?);
        }
    }

    let primary_git_status_after = git_status_short(repo_root)?;
    let lifecycle_state = if verification.iter().all(|output| output.status == 0) {
        PatchSandboxState::Verified
    } else {
        PatchSandboxState::Failed
    };
    let result = PatchSandboxApplicationResult {
        schema_version: "1".to_string(),
        run_id: plan.run_id,
        sandbox_id: plan.layout.sandbox_id,
        patch_draft_id: draft.id.clone(),
        lifecycle_state,
        sandbox_root: plan.layout.sandbox_root,
        worktree_path: plan.layout.worktree_path,
        evidence_path: plan.layout.evidence_path,
        applied_target_path: run_relative_path(run_dir, &applied_target)?,
        applied_draft_path: run_relative_path(run_dir, &applied_draft_path)?,
        result_path: format!("sandbox/{patch_draft_id}/evidence/result.json"),
        verification,
        primary_git_status_before,
        primary_git_status_after,
    };
    let result_path = run_dir.join(&result.result_path);
    write_json_atomic(&result_path, &json!(result))?;
    if result.primary_git_status_before != result.primary_git_status_after {
        return Err(anyhow!(
            "primary repo git status changed during sandbox application"
        ));
    }
    if result.lifecycle_state == PatchSandboxState::Failed {
        return Err(anyhow!(
            "patch sandbox verification failed; result written to {}",
            result_path.display()
        ));
    }
    append_ledger_event(
        run_dir,
        "mutation.sandboxed",
        "evolve-cli",
        json!({
            "patch_draft_id": result.patch_draft_id,
            "sandbox_root": result.sandbox_root,
            "result_path": result.result_path,
        }),
    )?;
    Ok(result)
}

pub fn create_mutation_proposal(
    run_dir: impl AsRef<Path>,
    input: MutationProposalInput,
) -> Result<MutationProposal> {
    let run_dir = run_dir.as_ref();
    require_text("mutation reason", &input.reason)?;
    require_text("mutation evidence", &input.evidence_id)?;
    require_text("mutation target", &input.target)?;
    require_text("mutation path", &input.path)?;
    require_text("mutation from", &input.from)?;
    require_text("mutation to", &input.to)?;
    let evidence = read_evidence_index(run_dir)?;
    if !evidence
        .artifacts
        .iter()
        .any(|artifact| artifact.id == input.evidence_id)
    {
        return Err(anyhow!(
            "mutation evidence id not found: {}",
            input.evidence_id
        ));
    }
    let verdict_status = fs::read_to_string(run_dir.join("verdict.json"))
        .ok()
        .and_then(|input| serde_json::from_str::<serde_json::Value>(&input).ok())
        .and_then(|value| {
            value
                .get("status")
                .and_then(|status| status.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "unknown".to_string());
    // Serialize the read-modify-write of the proposals index so concurrent
    // proposal creation for the same run cannot lose a proposal, mirroring how
    // add_evidence_artifact guards the evidence index.
    let _guard = MUTATION_INDEX_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .map_err(|_| anyhow!("mutation index lock poisoned"))?;
    let mut index = read_mutation_proposals(run_dir)?;
    let created_at_unix_ms = unix_millis()?;
    let proposal = MutationProposal {
        id: format!(
            "mutation-{created_at_unix_ms}-{}",
            index.proposals.len() + 1
        ),
        reason: input.reason,
        evidence_id: input.evidence_id,
        target: input.target,
        path: input.path,
        from: input.from,
        to: input.to,
        confidence: "medium".to_string(),
        status: "proposed".to_string(),
        verdict_status,
        created_at_unix_ms,
        rationale: None,
    };
    index.proposals.push(proposal.clone());
    write_mutation_proposals(run_dir, &index)?;
    append_ledger_event(
        run_dir,
        "mutation.proposed",
        "mutation-cli",
        json!({
            "proposal_id": proposal.id,
            "evidence_id": proposal.evidence_id,
            "target": proposal.target,
            "path": proposal.path,
            "status": proposal.status
        }),
    )?;
    Ok(proposal)
}

fn attach_mutation_proposal_rationale(
    run_dir: impl AsRef<Path>,
    proposal_id: &str,
    rationale: MutationProposalRationale,
) -> Result<()> {
    let run_dir = run_dir.as_ref();
    let mut index = read_mutation_proposals(run_dir)?;
    let proposal = index
        .proposals
        .iter_mut()
        .find(|proposal| proposal.id == proposal_id)
        .ok_or_else(|| anyhow!("mutation proposal id not found: {proposal_id}"))?;
    proposal.rationale = Some(rationale);
    write_mutation_proposals(run_dir, &index)?;
    append_ledger_event(
        run_dir,
        "mutation.proposal_rationale.recorded",
        "evolve-cli",
        json!({
            "proposal_id": proposal_id,
            "path": "mutation/proposals.json"
        }),
    )?;
    Ok(())
}

pub fn update_journal(run_dir: impl AsRef<Path>) -> Result<String> {
    let run_dir = run_dir.as_ref();
    let seed = Seed::from_path(run_dir.join("seed.snapshot.yaml"))?;
    let evidence = read_evidence_index(run_dir)?;
    let ledger = read_ledger_events(run_dir)?;
    let verdict_input = fs::read_to_string(run_dir.join("verdict.json"))
        .context("failed to read verdict for journal")?;
    let verdict: serde_json::Value =
        serde_json::from_str(&verdict_input).context("failed to parse verdict for journal")?;
    let run = read_json_value(run_dir.join("run.json"))?;
    let proposals = read_mutation_proposals(run_dir)?.proposals;
    let reviews = read_mutation_review_artifact(run_dir)?.decisions;
    let (applications, application_read_error) =
        match read_scene_only_mutation_applications(run_dir) {
            Ok(index) => (index.applications, None),
            Err(error) => (Vec::new(), Some(error.to_string())),
        };
    let (visual_applications, visual_application_read_error) =
        match read_visual_edit_draft_applications(run_dir) {
            Ok(index) => (index.applications, None),
            Err(error) => (Vec::new(), Some(error.to_string())),
        };
    let comparison = read_dashboard_comparison(run_dir);
    let regression_promotions = read_regression_promotion_records(run_dir);
    let mut journal = render_journal(
        run_dir, &seed, &evidence, &ledger, &verdict, &proposals, &run,
    );
    journal
        .push_str(&behavior_evidence::render_behavior_evidence_journal_section(run_dir, &evidence));
    journal.push_str(&render_authoring_governance_journal_section(
        &proposals,
        &reviews,
        &applications,
        application_read_error.as_deref(),
        (
            &visual_applications,
            visual_application_read_error.as_deref(),
        ),
        &comparison,
        &regression_promotions,
    ));
    journal.push_str(&render_review_decision_journal_section(&reviews));
    journal.push_str(&render_regression_promotion_journal_section(
        &regression_promotions,
    ));
    fs::write(run_dir.join("journal.md"), &journal).context("failed to write journal")?;
    Ok(journal)
}

fn render_journal(
    run_dir: &Path,
    seed: &Seed,
    evidence: &EvidenceIndex,
    ledger: &[serde_json::Value],
    verdict: &serde_json::Value,
    proposals: &[MutationProposal],
    run: &serde_json::Value,
) -> String {
    let mut out = String::new();
    out.push_str("# Ouroforge Run Journal\n\n");
    out.push_str("## Seed Summary\n\n");
    out.push_str(&format!("- Seed: `{}` — {}\n", seed.id, seed.title));
    out.push_str(&format!("- Goal: {}\n", seed.goal));
    out.push_str(&format!("- Target: `{}`\n\n", seed.constraints.target));

    out.push_str("## Expected Criteria\n\n");
    for item in &seed.acceptance {
        out.push_str(&format!("- {}\n", item));
    }
    out.push('\n');

    if run.get("project").is_some() {
        out.push_str("## Project Context\n\n");
        match read_dashboard_project_context(run) {
            Some(project) => {
                out.push_str(&format!("- Project: `{}` — {}\n", project.id, project.name));
                out.push_str(&format!("- Project root: `{}`\n", project.project_root));
                out.push_str(&format!("- Manifest: `{}`\n", project.manifest_path));
                out.push_str(&format!(
                    "- Manifest hash: `{}:{}`\n",
                    project.manifest_hash.algorithm, project.manifest_hash.value
                ));
                out.push_str(&format!("- Seed path: `{}`\n", project.seed_path));
                if let Some(pack) = &project.scenario_pack {
                    out.push_str(&format!(
                        "- Scenario pack: `{}` (`{}`) — {} scenario(s)\n",
                        pack.id,
                        pack.path,
                        pack.scenario_ids.len()
                    ));
                }
                if let Some(transaction_id) = &project.transaction_id {
                    out.push_str(&format!("- Linked transaction: `{transaction_id}`\n"));
                }
                if project.scenes.is_empty() {
                    out.push_str("- Scenes: none recorded\n");
                } else {
                    out.push_str("- Scenes:\n");
                    for scene in &project.scenes {
                        out.push_str(&format!(
                            "  - `{}` (`{}:{}`)\n",
                            scene.path, scene.hash.algorithm, scene.hash.value
                        ));
                    }
                }
            }
            None => {
                out.push_str("- Project metadata is present but malformed; dashboard export keeps it unavailable instead of inferring context.\n");
            }
        }
        out.push('\n');
    }

    if run.get("run_command_context").is_some() {
        out.push_str("## Reproducible Command Context\n\n");
        match read_dashboard_command_context(run) {
            Some(context) => {
                out.push_str("- Display-only: copy manually if needed; Ouroforge does not auto-rerun this command from browser surfaces.\n");
                out.push_str(&format!("- Command: `{}`\n", context.command));
                out.push_str(&format!("- Seed path: `{}`\n", context.seed_path));
                out.push_str(&format!("- Workers: `{}`\n", context.workers));
                out.push_str(&format!("- Runs root: `{}`\n", context.runs_root));
                if let Some(project_root) = &context.project_root {
                    out.push_str(&format!("- Project root: `{project_root}`\n"));
                }
                if let Some(manifest_path) = &context.manifest_path {
                    out.push_str(&format!("- Manifest: `{manifest_path}`\n"));
                }
                if let Some(scenario_pack_id) = &context.scenario_pack_id {
                    out.push_str(&format!("- Scenario pack: `{scenario_pack_id}`\n"));
                }
                if let Some(transaction_path) = &context.transaction_path {
                    out.push_str(&format!("- Transaction: `{transaction_path}`\n"));
                }
                out.push_str(&format!(
                    "- Target assumption: `{}`\n",
                    context.runtime_target
                ));
                out.push_str(&format!(
                    "- Browser boundary: `{}` / `{}`\n",
                    context.browser_boundary, context.cdp_transport
                ));
            }
            None => {
                out.push_str("- Command context is present but malformed; no reproduction command was inferred.\n");
            }
        }
        out.push('\n');
    }

    if let Some(provenance) = run.get("transaction_provenance") {
        out.push_str("## Scene Edit Transaction\n\n");
        if let Some(id) = provenance
            .get("transactionId")
            .and_then(|value| value.as_str())
        {
            out.push_str(&format!("- Transaction: `{}`\n", id));
        }
        if let Some(path) = provenance
            .get("transactionArtifactPath")
            .and_then(|value| value.as_str())
        {
            out.push_str(&format!("- Artifact: `{}`\n", path));
        }
        if let Some(scene_path) = provenance.get("scenePath").and_then(|value| value.as_str()) {
            out.push_str(&format!("- Scene: `{}`\n", scene_path));
        }
        out.push('\n');
    }

    out.push_str("## Executed Scenarios\n\n");
    for scenario in &seed.scenarios {
        let started = ledger.iter().any(|event| {
            event["event"] == "scenario.started"
                && event["payload"]["scenario_id"] == scenario.id.as_str()
        });
        let completed = ledger.iter().any(|event| {
            event["event"] == "scenario.completed"
                && event["payload"]["scenario_id"] == scenario.id.as_str()
        });
        out.push_str(&format!(
            "- `{}`: {} (started: {}, completed: {})\n",
            scenario.id, scenario.description, started, completed
        ));
    }
    out.push('\n');

    out.push_str("## Observations\n\n");
    out.push_str(&format!("- Ledger events recorded: {}\n", ledger.len()));
    out.push_str(&format!(
        "- Evidence artifacts indexed: {}\n",
        evidence.artifacts.len()
    ));
    let legacy_replay_count = evidence
        .artifacts
        .iter()
        .filter(|artifact| {
            artifact
                .metadata
                .get("artifact")
                .and_then(|value| value.as_str())
                == Some("input_replay")
        })
        .count();
    let scenario_replay_count = evidence
        .artifacts
        .iter()
        .filter(|artifact| {
            artifact
                .metadata
                .get("artifact")
                .and_then(|value| value.as_str())
                == Some("scenario_input_replay")
        })
        .count();
    out.push_str(&format!(
        "- Input replay evidence: {} legacy replay artifact(s), {} scenario input replay artifact(s)\n\n",
        legacy_replay_count, scenario_replay_count
    ));

    out.push_str(&render_asset_reference_integrity_journal_section(
        run_dir, evidence,
    ));

    out.push_str("## Gameplay Trigger/Flag Evidence\n\n");
    match journal_gameplay_summary(run_dir, evidence) {
        Some((source, gameplay)) => {
            out.push_str(&format!("- Source world-state: `{source}`\n"));
            out.push_str(&format!(
                "- Declared flags: `{}`; observed world flags: `{}` (`{}` true, `{}` false)\n",
                gameplay["declaredFlagCount"].as_u64().unwrap_or(0),
                gameplay["worldFlagCount"].as_u64().unwrap_or(0),
                gameplay["trueFlagCount"].as_u64().unwrap_or(0),
                gameplay["falseFlagCount"].as_u64().unwrap_or(0)
            ));
            out.push_str(&format!(
                "- Trigger components: `{}`; goalFlag components: `{}`; HUD value components: `{}`; trigger collision events: `{}`\n",
                gameplay["triggerEntityCount"].as_u64().unwrap_or(0),
                gameplay["goalFlagEntityCount"].as_u64().unwrap_or(0),
                gameplay["hudValueEntityCount"].as_u64().unwrap_or(0),
                gameplay["triggerCollisionEventCount"].as_u64().unwrap_or(0)
            ));
            let true_flags = gameplay["trueFlags"]
                .as_array()
                .map(|flags| {
                    flags
                        .iter()
                        .filter_map(|flag| flag.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            out.push_str(&format!(
                "- True flags: {}\n\n",
                if true_flags.is_empty() {
                    "none".to_string()
                } else {
                    format!("`{true_flags}`")
                }
            ));
        }
        None => {
            out.push_str("- No readable world-state trigger/flag evidence was available.\n\n");
        }
    }

    out.push_str(&render_scene3d_scenario_assertions_journal_section(
        run_dir, evidence,
    ));
    out.push_str(&render_behavior_assertions_journal_section(
        run_dir, evidence, verdict,
    ));
    out.push_str(&render_behavior_lifecycle_journal_section(
        run_dir, evidence,
    ));

    out.push_str("## Evidence\n\n");
    if evidence.artifacts.is_empty() {
        out.push_str("- No evidence artifacts indexed.\n");
    } else {
        for artifact in &evidence.artifacts {
            out.push_str(&format!(
                "- `{}` ({}) → `{}`\n",
                artifact.id, artifact.kind, artifact.path
            ));
        }
    }
    out.push('\n');

    out.push_str("## Verdict Summary\n\n");
    out.push_str(&format!(
        "- Status: `{}`\n",
        verdict["status"].as_str().unwrap_or("unknown")
    ));
    out.push_str(&format!(
        "- Summary: {}\n\n",
        verdict["summary"]
            .as_str()
            .unwrap_or("No summary available.")
    ));
    out.push_str(&render_visual_semantic_gate_journal_section(verdict));

    out.push_str("## Failed Criteria\n\n");
    let failures = verdict["failures"].as_array().cloned().unwrap_or_default();
    if failures.is_empty() {
        out.push_str("- None recorded.\n");
    } else {
        for failure in failures {
            out.push_str(&format!(
                "- `{}`: {}\n",
                failure["kind"].as_str().unwrap_or("failure"),
                failure
            ));
        }
    }
    out.push('\n');

    out.push_str("## Open Questions\n\n");
    out.push_str("- None recorded by deterministic artifacts.\n\n");
    out.push_str("## Next Mutation\n\n");
    if proposals.is_empty() {
        out.push_str("- No mutation proposals recorded.\n");
    } else {
        for proposal in proposals {
            out.push_str(&format!(
                "- `{}`: {} (target `{}` path `{}` evidence `{}` status `{}`)\n",
                proposal.id,
                proposal.reason,
                proposal.target,
                proposal.path,
                proposal.evidence_id,
                proposal.status
            ));
            if let Some(rationale) = &proposal.rationale {
                out.push_str(&format!(
                    "  - Rationale: `{}`; expected effect: {}; evidence ids: {}; allowed mutation: `{}`; confidence: `{}`\n",
                    rationale.failure_classification,
                    rationale.expected_effect,
                    rationale.evidence_artifact_ids.join(", "),
                    mutation_allowed_type_label(&rationale.allowed_mutation_type),
                    mutation_rationale_confidence_label(&rationale.confidence)
                ));
                if let (Some(gate), Some(evidence_state), Some(bounded_type)) = (
                    &rationale.failing_gate_category,
                    &rationale.evidence_state,
                    &rationale.bounded_mutation_type,
                ) {
                    out.push_str(&format!(
                        "  - Evidence-linked gate: `{}`; justifying evidence: `{}`; evidence state: `{}`; bounded type: `{}`\n",
                        mutation_proposal_gate_category_label(gate),
                        rationale
                            .justifying_evidence_ref
                            .as_deref()
                            .unwrap_or("missing"),
                        mutation_proposal_evidence_state_label(evidence_state),
                        mutation_proposal_bounded_type_label(bounded_type)
                    ));
                }
            }
        }
    }
    out.push_str(&render_evolve_rerun_delta_journal_section(
        run_dir, proposals,
    ));
    out
}

fn render_scene3d_scenario_assertions_journal_section(
    run_dir: &Path,
    evidence: &EvidenceIndex,
) -> String {
    let mut out = String::new();
    out.push_str("## 3D Scenario Assertion Evidence\n\n");
    let scenario_results = match select_dashboard_artifacts(
        run_dir,
        &evidence.artifacts,
        dashboard_artifact_is_scenario_result,
    ) {
        Ok(results) => results,
        Err(error) => {
            out.push_str(&format!(
                "- Scenario assertion evidence could not be read: {error}\n\n"
            ));
            return out;
        }
    };
    let assertions = read_dashboard_scenario_assertions(&scenario_results);
    if assertions.scene3d_count == 0 {
        out.push_str("- No `scene3d_*` scenario assertion results were recorded.\n\n");
        return out;
    }
    out.push_str("- Boundary: bounded local 3D scenario QA evidence only; no production 3D engine or Godot replacement claim.\n");
    out.push_str(&format!(
        "- Scene3D assertions: `{}` total (`{}` passed, `{}` failed).\n",
        assertions.scene3d_count,
        assertions.scene3d_count - assertions.scene3d_failed_count,
        assertions.scene3d_failed_count
    ));
    out.push_str("- Targets:\n");
    for target in assertions
        .targets
        .iter()
        .filter(|target| target.id.starts_with("scene3d_"))
    {
        out.push_str(&format!(
            "  - `{}`: `{}` total (`{}` passed, `{}` failed); evidence refs: {}\n",
            target.id,
            target.total_count,
            target.passed_count,
            target.failed_count,
            join_or_none(&target.evidence_refs)
        ));
    }
    out.push_str(&format!(
        "- 3D assertion evidence refs: {}\n\n",
        join_or_none(&assertions.evidence_refs)
    ));
    out
}

fn render_behavior_assertions_journal_section(
    run_dir: &Path,
    evidence: &EvidenceIndex,
    verdict: &serde_json::Value,
) -> String {
    let mut out = String::new();
    out.push_str("## Behavior Assertion Evidence\n\n");
    let results = match select_dashboard_artifacts(
        run_dir,
        &evidence.artifacts,
        dashboard_artifact_is_behavior_assertion_result,
    ) {
        Ok(results) => results,
        Err(error) => {
            out.push_str(&format!(
                "- Behavior assertion evidence could not be read: {error}\n\n"
            ));
            return out;
        }
    };
    let summary = read_dashboard_behavior_assertions(&results);
    if !summary.present {
        out.push_str("- No behavior assertion result artifacts were recorded.\n\n");
        return out;
    }

    out.push_str(&format!("- Boundary: {}\n", summary.boundary));
    out.push_str(&format!(
        "- Behavior assertion result artifacts: `{}` (`{}` malformed/missing).\n",
        summary.result_count, summary.malformed_count
    ));
    out.push_str(&format!(
        "- Behavior assertions: `{}` total (`{}` passed, `{}` failed).\n",
        summary.total_count, summary.passed_count, summary.failed_count
    ));
    out.push_str(&format!(
        "- Result refs: {}; evidence refs: {}\n",
        join_or_none(&summary.result_refs),
        join_or_none(&summary.evidence_refs)
    ));
    out.push_str("- Suites:\n");
    for suite in &summary.suites {
        out.push_str(&format!(
            "  - Suite `{}` status `{}` scenario `{}` result `{}` evidence `{}` (`{}` passed, `{}` failed)\n",
            suite.suite_id,
            suite.status,
            suite.scenario_id.as_deref().unwrap_or("none"),
            suite.result_ref,
            suite.evidence_ref.as_deref().unwrap_or("none"),
            suite.passed_count,
            suite.failed_count
        ));
        for failure in &suite.failures {
            out.push_str(&format!(
                "    - Failed behavior assertion `{}`: {}\n",
                failure.assertion_id, failure.message
            ));
        }
    }

    let behavior_failures = verdict
        .get("failures")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter(|failure| {
            matches!(
                failure.get("kind").and_then(|value| value.as_str()),
                Some("behavior_assertion_failed")
                    | Some("missing_behavior_evidence")
                    | Some("malformed_behavior_evidence")
                    | Some("malformed_behavior_assertion_suite")
            )
        })
        .collect::<Vec<_>>();
    if behavior_failures.is_empty() {
        out.push_str("- Verdict-linked behavior failures: none.\n\n");
    } else {
        out.push_str("- Verdict-linked behavior failures:\n");
        for failure in behavior_failures {
            out.push_str(&format!(
                "  - `{}`: result `{}` evidence `{}` message `{}`\n",
                failure
                    .get("kind")
                    .and_then(|value| value.as_str())
                    .unwrap_or("behavior_failure"),
                failure
                    .get("result_ref")
                    .and_then(|value| value.as_str())
                    .unwrap_or("none"),
                failure
                    .get("evidence_ref")
                    .or_else(|| failure.get("path"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("none"),
                failure
                    .get("message")
                    .or_else(|| failure.get("reason"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("none")
            ));
        }
        out.push('\n');
    }
    out
}

fn render_behavior_lifecycle_journal_section(run_dir: &Path, evidence: &EvidenceIndex) -> String {
    let mut out = String::new();
    out.push_str("## Behavior Lifecycle Evidence\n\n");
    out.push_str("- Boundary: read-only structured behavior lifecycle summary; no arbitrary script execution, eval, dynamic import, plugin loader, command bridge, local server bridge, browser trusted writes, auto-apply, auto-merge, or production-stable scripting API.\n");

    let bundles = match select_dashboard_artifacts(
        run_dir,
        &evidence.artifacts,
        dashboard_artifact_is_behavior_evidence_bundle,
    ) {
        Ok(bundles) => bundles,
        Err(error) => {
            out.push_str(&format!(
                "- Behavior lifecycle evidence could not be read: {error}\n\n"
            ));
            return out;
        }
    };

    if bundles.is_empty() {
        out.push_str("- Status: `missing`; no behavior evidence bundle artifacts were recorded, so behavior definitions, runtime events, scenario outcomes, drafts, reviews, applies, rollback metadata, and rerun comparisons remain unavailable instead of inferred.\n\n");
        return out;
    }

    out.push_str(&format!(
        "- Behavior evidence bundle artifacts: `{}`.\n",
        bundles.len()
    ));
    for artifact in &bundles {
        out.push_str(&format!(
            "- Bundle artifact `{}` path `{}`:\n",
            artifact.id, artifact.path
        ));
        if let Some(error) = &artifact.read_error {
            out.push_str(&format!(
                "  - Status: `malformed`; artifact is present but unreadable: {error}\n"
            ));
            continue;
        }
        let Some(value) = artifact.value.as_ref() else {
            out.push_str(
                "  - Status: `malformed`; artifact is present but no JSON value was available.\n",
            );
            continue;
        };
        let bundle_json = match serde_json::to_string(value) {
            Ok(bundle_json) => bundle_json,
            Err(error) => {
                out.push_str(&format!(
                    "  - Status: `malformed`; artifact could not be serialized for validation: {error}\n"
                ));
                continue;
            }
        };
        let bundle =
            match behavior_evidence::BehaviorEvidenceBundleArtifact::from_json_str(&bundle_json) {
                Ok(bundle) => bundle,
                Err(error) => {
                    out.push_str(&format!(
                    "  - Status: `malformed`; behavior evidence bundle validation failed: {error}\n"
                ));
                    continue;
                }
            };
        let validation = bundle.inspect();
        out.push_str(&format!(
            "  - Bundle `{}` status `{}`; lifecycle refs `{}`; validation status `{}`.\n",
            bundle.bundle_id,
            behavior_bundle_status_label(bundle.status),
            validation.lifecycle_ref_count,
            validation.status
        ));
        out.push_str(&format!(
            "  - Behavior definitions: {}; runtime events: {}; scenario outcomes: {}\n",
            behavior_ref_paths(&bundle.behavior_definition_refs),
            behavior_ref_paths(&bundle.runtime_event_refs),
            behavior_ref_paths(&bundle.scenario_outcome_refs)
        ));
        out.push_str(&format!(
            "  - Drafts: {}; reviews: {}; applies: {}; rollback: {}; rerun comparisons: {}\n",
            behavior_ref_paths(&bundle.draft_refs),
            behavior_ref_paths(&bundle.review_decision_refs),
            behavior_ref_paths(&bundle.apply_transaction_refs),
            behavior_ref_paths(&bundle.rollback_metadata_refs),
            behavior_ref_paths(&bundle.rerun_comparison_refs)
        ));
        if bundle.observed_failures.is_empty() {
            out.push_str("  - Observed failures: none recorded.\n");
        } else {
            out.push_str("  - Observed failures:\n");
            for failure in &bundle.observed_failures {
                out.push_str(&format!(
                    "    - Scenario `{}`: {} (evidence `{}`)\n",
                    failure.scenario_id, failure.summary, failure.evidence_ref.path
                ));
            }
        }
        if bundle.next_step_hypotheses.is_empty() {
            out.push_str("  - Next-step hypotheses: none recorded.\n");
        } else {
            out.push_str("  - Next-step hypotheses:\n");
            for hypothesis in &bundle.next_step_hypotheses {
                out.push_str(&format!(
                    "    - `{}`: {}\n",
                    hypothesis.id, hypothesis.summary
                ));
            }
        }
        if bundle.blocked_reasons.is_empty() {
            out.push_str("  - Blocked/stale reasons: none recorded.\n");
        } else {
            out.push_str(&format!(
                "  - Blocked/stale reasons: {}\n",
                join_or_none(&bundle.blocked_reasons)
            ));
        }
        out.push_str(&format!(
            "  - Linked evidence refs: {}\n",
            behavior_ref_paths(&bundle.linked_evidence)
        ));
        out.push_str(&format!(
            "  - Guardrails: {}\n",
            join_or_none(&bundle.guardrails)
        ));
    }
    out.push('\n');
    out
}

fn behavior_bundle_status_label(
    status: behavior_evidence::BehaviorEvidenceBundleStatus,
) -> &'static str {
    match status {
        behavior_evidence::BehaviorEvidenceBundleStatus::Complete => "complete",
        behavior_evidence::BehaviorEvidenceBundleStatus::Partial => "partial",
        behavior_evidence::BehaviorEvidenceBundleStatus::Blocked => "blocked",
        behavior_evidence::BehaviorEvidenceBundleStatus::Stale => "stale",
    }
}

fn behavior_ref_paths(refs: &[behavior_evidence::BehaviorEvidenceRef]) -> String {
    if refs.is_empty() {
        "none".to_string()
    } else {
        refs.iter()
            .map(|reference| format!("`{}`", reference.path))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn journal_gameplay_summary(
    run_dir: &Path,
    evidence: &EvidenceIndex,
) -> Option<(String, serde_json::Value)> {
    let world_states = select_dashboard_artifacts(
        run_dir,
        &evidence.artifacts,
        dashboard_artifact_is_world_state,
    )
    .ok()?;
    let artifact = world_states.iter().find(|artifact| {
        artifact.exists && artifact.read_error.is_none() && artifact.value.is_some()
    })?;
    let world_state = artifact.value.as_ref()?;
    Some((
        artifact.path.clone(),
        dashboard_gameplay_summary(world_state),
    ))
}

fn render_asset_reference_integrity_journal_section(
    run_dir: &Path,
    evidence: &EvidenceIndex,
) -> String {
    let mut out = String::new();
    out.push_str("## Asset Reference Integrity\n\n");
    match read_dashboard_asset_integrity(run_dir, &evidence.artifacts) {
        Ok(report) if report.present => {
            out.push_str(&format!(
                "- Warnings: `{}` total (`{}` stale hash, `{}` missing ref/file, `{}` invalid type)\n",
                report.warning_count,
                report.stale_hash_count,
                report.missing_ref_count,
                report.invalid_type_count
            ));
            out.push_str(&format!(
                "- Evidence refs: {}\n",
                join_or_none(&report.evidence_refs)
            ));
            if report.warnings.is_empty() {
                out.push_str("- No missing, stale, invalid-type, or unresolved asset reference warnings recorded.\n");
            } else {
                for warning in report.warnings.iter().take(8) {
                    out.push_str(&format!(
                        "- `{}` `{}`: {}{}\n",
                        warning.kind,
                        warning.asset_id,
                        warning.message,
                        warning
                            .path
                            .as_ref()
                            .map(|path| format!(" (`{path}`)"))
                            .unwrap_or_default()
                    ));
                }
                if report.warnings.len() > 8 {
                    out.push_str(&format!(
                        "- ... {} more warning(s) in asset reference integrity evidence.\n",
                        report.warnings.len() - 8
                    ));
                }
            }
        }
        Ok(report) => {
            out.push_str(&format!("- {}\n", report.empty_state));
        }
        Err(error) => {
            out.push_str(&format!(
                "- Asset reference integrity evidence could not be read: {}\n",
                error
            ));
        }
    }
    out.push('\n');
    out
}

fn evaluate_behavior_assertion_suite_artifact(
    run_dir: &Path,
    suite_path: &str,
    failures: &mut Vec<serde_json::Value>,
    evidence_refs: &mut Vec<String>,
) -> Result<bool> {
    validate_relative_artifact_path("behavior assertion suite path", suite_path)?;
    let suite_full_path = run_dir.join(suite_path);
    let suite = match fs::read_to_string(&suite_full_path)
        .with_context(|| format!("failed to read behavior assertion suite {suite_path}"))
        .and_then(|input| {
            serde_json::from_str::<BehaviorScenarioAssertionSuite>(&input)
                .with_context(|| format!("failed to parse behavior assertion suite {suite_path}"))
        })
        .and_then(|suite| {
            suite.validate().with_context(|| {
                format!("failed to validate behavior assertion suite {suite_path}")
            })?;
            Ok(suite)
        }) {
        Ok(suite) => suite,
        Err(error) => {
            failures.push(json!({
                "kind": "malformed_behavior_assertion_suite",
                "path": suite_path,
                "reason": error.to_string()
            }));
            return Ok(false);
        }
    };

    validate_relative_artifact_path("behavior assertion suite evidenceRef", &suite.evidence_ref)?;
    let evidence_full_path = run_dir.join(&suite.evidence_ref);
    let evidence = match fs::read_to_string(&evidence_full_path) {
        Ok(input) => match serde_json::from_str::<BehaviorRuntimeEvidenceBundle>(&input) {
            Ok(evidence) => evidence,
            Err(error) => {
                failures.push(json!({
                    "kind": "malformed_behavior_evidence",
                    "suite_id": suite.suite_id,
                    "path": suite.evidence_ref,
                    "reason": error.to_string()
                }));
                return Ok(false);
            }
        },
        Err(error) if error.kind() == ErrorKind::NotFound => {
            failures.push(json!({
                "kind": "missing_behavior_evidence",
                "suite_id": suite.suite_id,
                "path": suite.evidence_ref
            }));
            return Ok(false);
        }
        Err(error) => {
            failures.push(json!({
                "kind": "malformed_behavior_evidence",
                "suite_id": suite.suite_id,
                "path": suite.evidence_ref,
                "reason": error.to_string()
            }));
            return Ok(false);
        }
    };

    let result = match suite.evaluate(&evidence) {
        Ok(result) => result,
        Err(error) => {
            failures.push(json!({
                "kind": "malformed_behavior_assertion_suite",
                "path": suite_path,
                "reason": error.to_string()
            }));
            return Ok(false);
        }
    };
    let result_path = format!(
        "evidence/behavior-assertions/{}-result.json",
        suite.suite_id
    );
    fs::create_dir_all(run_dir.join("evidence/behavior-assertions"))
        .context("failed to create behavior assertion evidence directory")?;
    write_json(&run_dir.join(&result_path), &json!(result))?;
    add_evidence_artifact(
        run_dir,
        &format!("behavior-assertion-result-{}", suite.suite_id),
        "application/json",
        &result_path,
        json!({
            "artifact": "behavior_assertion_result",
            "suite_id": suite.suite_id,
            "scenario_id": suite.scenario_id,
            "evidence_ref": suite.evidence_ref
        }),
    )?;
    evidence_refs.push(result_path.clone());

    for assertion in result
        .assertions
        .iter()
        .filter(|assertion| assertion.status == BehaviorScenarioAssertionStatus::Failed)
    {
        failures.push(json!({
            "kind": "behavior_assertion_failed",
            "suite_id": result.suite_id,
            "scenario_id": result.scenario_id,
            "path": suite_path,
            "assertion_id": assertion.assertion_id,
            "evidence_ref": result.evidence_ref,
            "result_ref": result_path,
            "message": assertion.message
        }));
    }
    Ok(result.status == BehaviorScenarioAssertionStatus::Passed)
}

pub fn evaluate_run(run_dir: impl AsRef<Path>) -> Result<EvaluationVerdict> {
    let run_dir = run_dir.as_ref();
    let evaluator_config = Seed::from_path(run_dir.join("seed.snapshot.yaml"))
        .ok()
        .and_then(|seed| seed.evaluator);
    ouroforge_evaluator::evaluate_run_with_behavior_evaluator(
        run_dir,
        evaluator_config,
        evaluate_behavior_assertion_suite_artifact,
    )
}

pub fn compare_runs(
    before_run_dir: impl AsRef<Path>,
    after_run_dir: impl AsRef<Path>,
) -> Result<RunComparison> {
    let before_run_dir = before_run_dir.as_ref();
    let after_run_dir = after_run_dir.as_ref();
    let before_details = load_run_comparison_details(before_run_dir, "before")?;
    let after_details = load_run_comparison_details(after_run_dir, "after")?;
    let before = before_details.snapshot.clone();
    let after = after_details.snapshot.clone();
    let classification = classify_run_comparison(&before, &after).to_string();
    let semantic = build_run_semantic_diff(&before_details, &after_details, &classification);
    let four_gate_deltas = build_four_gate_deltas(&before_details, &after_details);
    let comparability = run_comparison_comparability(&four_gate_deltas);
    let four_gate = run_four_gate_comparison_value(&four_gate_deltas);
    let evidence_refs = vec![
        before_run_dir.join("run.json").display().to_string(),
        before_run_dir.join("verdict.json").display().to_string(),
        before_run_dir
            .join("evidence/index.json")
            .display()
            .to_string(),
        after_run_dir.join("run.json").display().to_string(),
        after_run_dir.join("verdict.json").display().to_string(),
        after_run_dir
            .join("evidence/index.json")
            .display()
            .to_string(),
    ];
    Ok(RunComparison {
        before_run_id: before.run_id.clone(),
        after_run_id: after.run_id.clone(),
        classification,
        deltas: json!({
            "scenario_results": after.scenario_results as i64 - before.scenario_results as i64,
            "failed_scenarios": after.failed_scenarios as i64 - before.failed_scenarios as i64,
            "assertion_failures": after.assertion_failures as i64 - before.assertion_failures as i64,
            "performance_artifacts": after.performance_artifacts as i64 - before.performance_artifacts as i64,
            "evidence_artifacts": after.evidence_artifacts as i64 - before.evidence_artifacts as i64,
            "input_replay_artifacts": after.input_replay_artifacts as i64 - before.input_replay_artifacts as i64,
            "mutation_proposals": after.mutation_proposals as i64 - before.mutation_proposals as i64
        }),
        four_gate,
        four_gate_deltas,
        comparability,
        semantic,
        before,
        after,
        evidence_refs,
        unsupported: vec![
            "semantic gameplay quality is not inferred beyond verdict/scenario/evidence deltas"
                .to_string(),
        ],
    })
}

pub fn write_run_comparison_artifact(
    before_run_dir: impl AsRef<Path>,
    after_run_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<PathBuf> {
    let comparison = compare_runs(before_run_dir, after_run_dir)?;
    validate_path_component("before run id", &comparison.before_run_id)?;
    validate_path_component("after run id", &comparison.after_run_id)?;
    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "failed to create comparison output dir {}",
            output_dir.display()
        )
    })?;
    let path = output_dir.join(format!(
        "run-comparison-{}--{}.json",
        comparison.before_run_id, comparison.after_run_id
    ));
    write_json(&path, &json!(comparison))?;
    Ok(path)
}

fn collect_run_gate_observations(
    verdict: &serde_json::Value,
    evidence: &EvidenceIndex,
    run_id: &str,
    warnings: &[String],
) -> BTreeMap<String, RunGateObservation> {
    let mut observations = BTreeMap::new();
    for gate in ["mechanical", "runtime", "visual", "semantic"] {
        let category = verdict
            .get("gateCategories")
            .and_then(|value| value.get(gate));
        let declared = category
            .and_then(|value| value.get("declared"))
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let status = category
            .and_then(|value| value.get("status"))
            .and_then(|value| value.as_str())
            .map(str::to_string)
            .unwrap_or_else(|| {
                default_gate_observation(gate, verdict["status"].as_str().unwrap_or("unknown"))
                    .status
            });
        let mut refs = Vec::new();
        if let Some(category) = category {
            collect_gate_json_evidence_refs(category, &mut refs);
        }
        if gate == "mechanical" {
            collect_top_level_verdict_evidence_refs(verdict, &mut refs);
        }
        collect_gate_refs_from_verdict_arrays(verdict, gate, &mut refs);
        collect_gate_refs_from_failures(verdict, gate, &mut refs);
        refs.sort();
        refs.dedup();
        let mut reasons = Vec::new();
        if (declared || status != "pass") && refs.is_empty() {
            reasons.push(format!("{gate} gate has no before/after evidence refs"));
        }
        for reference in &refs {
            match evidence
                .artifacts
                .iter()
                .find(|artifact| artifact.path == *reference || artifact.id == *reference)
            {
                Some(artifact) => {
                    if evidence_artifact_is_stale_for_run(artifact, run_id) {
                        reasons.push(format!(
                            "stale evidence ref `{reference}` for run `{run_id}`"
                        ));
                    }
                }
                None => reasons.push(format!("missing evidence ref `{reference}`")),
            }
        }
        reasons.extend(warnings.iter().cloned());
        reasons.sort();
        reasons.dedup();
        observations.insert(
            gate.to_string(),
            RunGateObservation {
                declared,
                status,
                evidence_refs: refs,
                non_comparable_reasons: reasons,
            },
        );
    }
    observations
}

fn evidence_artifact_is_stale_for_run(artifact: &EvidenceArtifact, run_id: &str) -> bool {
    for key in ["run_id", "runId"] {
        if let Some(value) = artifact.metadata.get(key).and_then(|value| value.as_str()) {
            return value != run_id;
        }
    }
    false
}

fn load_run_comparison_details(run_dir: &Path, label: &str) -> Result<RunComparisonDetails> {
    let run_path = run_dir.join("run.json");
    let verdict_path = run_dir.join("verdict.json");
    let evidence_path = run_dir.join("evidence/index.json");
    for path in [&run_path, &verdict_path, &evidence_path] {
        if !path.is_file() {
            return Err(anyhow!(
                "{label} run is missing required artifact {}",
                path.display()
            ));
        }
    }
    let run = read_json_value(&run_path)?;
    let verdict = read_json_value(&verdict_path)?;
    let evidence = read_evidence_index(run_dir)?;
    let transaction_provenance = run
        .get("transaction_provenance")
        .cloned()
        .and_then(|value| serde_json::from_value(value).ok());
    let project = match run.get("project") {
        Some(value) => match serde_json::from_value(value.clone()) {
            Ok(project) => Some(project),
            Err(error) => {
                let warnings = vec![format!(
                    "{label} run project metadata could not be parsed: {error}"
                )];
                // Keep the rest of comparison readable; malformed project
                // metadata is surfaced as a warning rather than trusted.
                let mut details = load_run_comparison_details_without_project(
                    run_dir,
                    label,
                    run,
                    verdict,
                    evidence,
                    transaction_provenance,
                    warnings,
                )?;
                details.project = None;
                return Ok(details);
            }
        },
        None => None,
    };
    load_run_comparison_details_without_project(
        run_dir,
        label,
        run,
        verdict,
        evidence,
        transaction_provenance,
        Vec::new(),
    )
    .map(|mut details| {
        details.project = project;
        details
    })
}

fn load_run_comparison_details_without_project(
    run_dir: &Path,
    label: &str,
    run: serde_json::Value,
    verdict: serde_json::Value,
    evidence: EvidenceIndex,
    transaction_provenance: Option<RunTransactionProvenance>,
    warnings: Vec<String>,
) -> Result<RunComparisonDetails> {
    let mut failed_scenarios = 0usize;
    let mut assertion_failures = 0usize;
    let mut scenario_results = 0usize;
    let mut performance_artifacts = 0usize;
    let mut scenario_statuses = BTreeMap::new();
    let mut world_state = BTreeMap::new();
    let mut events = BTreeSet::new();
    let mut performance = BTreeMap::new();
    let mut evidence_keys = BTreeSet::new();
    let mut input_replay_keys = BTreeSet::new();
    let mut input_replay_artifacts = 0usize;
    let mut warnings = warnings;
    for artifact in &evidence.artifacts {
        evidence_keys.insert(format!(
            "{}|{}|{}",
            artifact.id, artifact.kind, artifact.path
        ));
        match artifact
            .metadata
            .get("artifact")
            .and_then(|value| value.as_str())
        {
            Some("scenario_result") => {
                scenario_results += 1;
                // Scenario results are required comparison inputs: classification
                // and deltas are derived from these counters, so an indexed but
                // unreadable/corrupt scenario_result must fail the comparison
                // rather than silently contributing zero failures (which could be
                // reported as no_change/improved on incomplete data).
                let result = read_json_value(run_dir.join(&artifact.path)).with_context(|| {
                    format!(
                        "{label} scenario_result artifact could not be read: {}",
                        artifact.path
                    )
                })?;
                let scenario_id = result
                    .get("scenario_id")
                    .and_then(|value| value.as_str())
                    .unwrap_or(&artifact.id)
                    .to_string();
                let status = result
                    .get("status")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                scenario_statuses.insert(scenario_id, status.clone());
                if result.get("status").and_then(|value| value.as_str()) != Some("passed") {
                    failed_scenarios += 1;
                }
                assertion_failures += result
                    .get("assertions")
                    .and_then(|value| value.as_array())
                    .map(|assertions| {
                        assertions
                            .iter()
                            .filter(|assertion| {
                                assertion.get("passed").and_then(|value| value.as_bool())
                                    == Some(false)
                            })
                            .count()
                    })
                    .unwrap_or(0);
            }
            Some("world_state") => match read_json_value(run_dir.join(&artifact.path)) {
                // Namespace per artifact (like performance/frame_stats) so multiple
                // world_state artifacts from different scenarios don't overwrite
                // each other under a shared "world/..." key and drop a change.
                Ok(value) => collect_semantic_scalars(
                    &format!("world/{}", artifact.id),
                    &value,
                    &mut world_state,
                    64,
                ),
                Err(_) => warnings.push(format!(
                    "{label} world_state artifact could not be read: {}",
                    artifact.path
                )),
            },
            Some("console_log") => match read_json_value(run_dir.join(&artifact.path)) {
                Ok(value) => collect_semantic_events("console", &value, &mut events, 64),
                Err(_) => warnings.push(format!(
                    "{label} console_log artifact could not be read: {}",
                    artifact.path
                )),
            },
            Some("performance_metrics") => {
                performance_artifacts += 1;
                match read_json_value(run_dir.join(&artifact.path)) {
                    Ok(value) => collect_semantic_scalars(
                        &format!("performance/{}", artifact.id),
                        &value,
                        &mut performance,
                        32,
                    ),
                    Err(_) => warnings.push(format!(
                        "{label} performance_metrics artifact could not be read: {}",
                        artifact.path
                    )),
                }
            }
            Some("frame_stats") => match read_json_value(run_dir.join(&artifact.path)) {
                Ok(value) => collect_semantic_scalars(
                    &format!("frame_stats/{}", artifact.id),
                    &value,
                    &mut performance,
                    32,
                ),
                Err(_) => warnings.push(format!(
                    "{label} frame_stats artifact could not be read: {}",
                    artifact.path
                )),
            },
            Some("input_replay") | Some("scenario_input_replay") => {
                input_replay_artifacts += 1;
                input_replay_keys.insert(input_replay_semantic_key(
                    run_dir,
                    label,
                    artifact,
                    &mut warnings,
                ));
            }
            _ => {}
        }
    }
    // Proposals live in mutation/proposals.json (not the evidence index), so
    // count them there; the evidence index never carries mutation_proposal
    // artifacts, which previously made this count always zero.
    let mutation_proposals = list_mutation_proposals(run_dir)
        .map(|proposals| proposals.len())
        .unwrap_or(0);
    let snapshot = RunComparisonSnapshot {
        run_id: run
            .get("id")
            .and_then(|value| value.as_str())
            .ok_or_else(|| anyhow!("{label} run.json missing id"))?
            .to_string(),
        verdict_status: verdict
            .get("status")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string(),
        scenario_results,
        failed_scenarios,
        assertion_failures,
        performance_artifacts,
        evidence_artifacts: evidence.artifacts.len(),
        input_replay_artifacts,
        mutation_proposals,
    };
    let gate_observations =
        collect_run_gate_observations(&verdict, &evidence, &snapshot.run_id, &warnings);
    Ok(RunComparisonDetails {
        snapshot,
        gate_observations,
        scenario_statuses,
        world_state,
        events,
        performance,
        evidence_keys,
        input_replay_keys,
        project: None,
        transaction_provenance,
        warnings,
    })
}

fn input_replay_semantic_key(
    run_dir: &Path,
    label: &str,
    artifact: &EvidenceArtifact,
    warnings: &mut Vec<String>,
) -> String {
    match read_json_value(run_dir.join(&artifact.path)) {
        Ok(value) => {
            let scenario_id = value
                .get("scenarioId")
                .or_else(|| value.get("scenario_id"))
                .and_then(|value| value.as_str())
                .or_else(|| {
                    artifact
                        .metadata
                        .get("scenario_id")
                        .and_then(|value| value.as_str())
                })
                .unwrap_or("unknown-scenario");
            let action = value
                .get("action")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .or_else(|| {
                    value
                        .get("replay")
                        .and_then(|replay| replay.get("id"))
                        .and_then(|value| value.as_str())
                })
                .or_else(|| value.get("id").and_then(|value| value.as_str()))
                .unwrap_or("input_replay");
            let frame = value
                .get("frame")
                .and_then(|value| value.as_u64())
                .map(|frame| frame.to_string())
                .unwrap_or_else(|| "sequence".to_string());
            format!("{}|{}|{}|{}", artifact.path, scenario_id, action, frame)
        }
        Err(error) => {
            warnings.push(format!(
                "{label} input replay artifact could not be read: {} ({error})",
                artifact.path
            ));
            artifact.path.clone()
        }
    }
}

pub fn run_scenarios(config: &ScenarioRunConfig) -> Result<ScenarioRunSummary> {
    let seed = Seed::from_path(config.run_dir.join("seed.snapshot.yaml"))?;
    let connection = create_cdp_page_target(&config.debugging_http_url, "about:blank")?;
    let transport = WebSocketCdpTransport::connect(&connection)?;
    let mut client = CdpClient::new(transport);

    client.enable_page()?;
    install_console_capture(&mut client)?;
    let _ = client.bring_page_to_front();
    client.navigate(&config.url)?;
    std::thread::sleep(Duration::from_millis(300));

    run_scenarios_with_client(config, &seed, &mut client)
}

fn run_scenarios_with_client<T: CdpTransport>(
    config: &ScenarioRunConfig,
    seed: &Seed,
    client: &mut CdpClient<T>,
) -> Result<ScenarioRunSummary> {
    let mut evidence_paths = Vec::new();
    let mut result_paths = Vec::new();
    let mut scenario_order = Vec::new();
    let mut scenario_summaries = Vec::new();
    let mut passed = 0;
    let mut failed = 0;
    for scenario in &seed.scenarios {
        scenario_order.push(scenario.id.clone());
        let result = run_scenario(config, client, scenario)?;
        evidence_paths.extend(result.evidence_paths);
        result_paths.push(result.result_path.clone());
        if result.passed {
            passed += 1;
        } else {
            failed += 1;
        }
        scenario_summaries.push(json!({
            "scenario_id": scenario.id,
            "status": if result.passed { "passed" } else { "failed" },
            "result_path": result.result_path
        }));
    }
    let suite_summary_path = format!("evidence/suite-summary-{}.json", unix_millis()?);
    write_json(
        &config.run_dir.join(&suite_summary_path),
        &json!({
            "artifact": "suite_summary",
            "status": if failed == 0 { "passed" } else { "failed" },
            "scenarios": seed.scenarios.len(),
            "completed": result_paths.len(),
            "passed": passed,
            "failed": failed,
            "scenario_order": scenario_order.clone(),
            "scenario_results": scenario_summaries,
            "result_paths": result_paths.clone(),
            "evidence_paths": evidence_paths.clone()
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("suite-summary-{}", unix_millis()?),
        "application/json",
        &suite_summary_path,
        json!({
            "artifact": "suite_summary",
            "scenarios": seed.scenarios.len(),
            "passed": passed,
            "failed": failed
        }),
    )?;
    evidence_paths.push(suite_summary_path.clone());

    Ok(ScenarioRunSummary {
        scenarios: seed.scenarios.len(),
        completed: result_paths.len(),
        passed,
        failed,
        scenario_order,
        suite_summary_path,
        evidence_paths,
        result_paths,
    })
}

fn run_scenario<T: CdpTransport>(
    config: &ScenarioRunConfig,
    client: &mut CdpClient<T>,
    scenario: &Scenario,
) -> Result<ScenarioExecutionResult> {
    validate_path_component("scenario id", &scenario.id)?;
    append_ledger_event(
        &config.run_dir,
        "scenario.started",
        "scenario-runner",
        json!({ "scenario_id": scenario.id, "url": config.url }),
    )?;

    let suffix = unix_millis()?;
    let scenario_dir = format!("evidence/scenarios/{}", scenario.id);
    fs::create_dir_all(config.run_dir.join(&scenario_dir)).with_context(|| {
        format!(
            "failed to create scenario evidence directory {}",
            config.run_dir.join(&scenario_dir).display()
        )
    })?;

    let mut replay_paths = Vec::new();
    let mut snapshot_paths = Vec::new();
    let mut visual_checkpoint_paths = Vec::new();
    let mut visual_checkpoint_screenshot_paths = Vec::new();
    let mut visual_checkpoint_summaries = Vec::new();
    let mut snapshot_ids = std::collections::BTreeMap::new();
    let mut pending_replay_artifacts = Vec::new();
    let mut scenario_frame = 0u32;
    for (step_index, step) in scenario.steps.iter().enumerate() {
        match step {
            ScenarioStep::Replay { replay } => {
                let replay_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    "input-replay",
                    "input_replay",
                    unix_millis()?,
                    &json!(replay),
                )?;
                replay_paths.push(replay_path.clone());
                pending_replay_artifacts.extend(pending_replay_artifacts_from_replay(
                    scenario,
                    step_index,
                    scenario_frame,
                    replay,
                    "inline_replay_event",
                )?);
                scenario_frame = scenario_frame
                    .checked_add(input_replay_last_frame(replay))
                    .ok_or_else(|| anyhow!("scenario replay frame overflow"))?;
                append_ledger_event(
                    &config.run_dir,
                    "scenario.input_replay",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "replay_id": replay.id,
                        "events": replay.events.len(),
                        "path": replay_path,
                        "source": "inline"
                    }),
                )?;
                execute_scenario_step(client, step)?;
            }
            ScenarioStep::ReplayRef { replay_ref } => {
                let replay = replay_ref.load_from_base(&config.run_dir)?;
                let replay_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    "input-replay",
                    "input_replay",
                    unix_millis()?,
                    &json!({
                        "reference": replay_ref,
                        "replay": replay
                    }),
                )?;
                replay_paths.push(replay_path.clone());
                pending_replay_artifacts.extend(pending_replay_artifacts_from_replay(
                    scenario,
                    step_index,
                    scenario_frame,
                    &replay,
                    "referenced_replay_event",
                )?);
                scenario_frame = scenario_frame
                    .checked_add(input_replay_last_frame(&replay))
                    .ok_or_else(|| anyhow!("scenario replay frame overflow"))?;
                append_ledger_event(
                    &config.run_dir,
                    "scenario.input_replay",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "replay_id": replay.id,
                        "events": replay.events.len(),
                        "path": replay_path,
                        "source": "replayRef",
                        "reference_path": replay_ref.path
                    }),
                )?;
                execute_input_replay(client, &replay)?;
            }
            ScenarioStep::Snapshot { snapshot } => {
                let snapshot_result = client.evaluate_json("window.__OUROFORGE__.snapshot()")?;
                let snapshot_id = snapshot_result
                    .get("snapshotId")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| anyhow!("snapshot probe did not return snapshotId"))?
                    .to_string();
                snapshot_ids.insert(snapshot.id.clone(), snapshot_id.clone());
                let world_state = client.evaluate_json("window.__OUROFORGE__.getWorldState()")?;
                let snapshot_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    &format!("snapshot-{}", snapshot.id),
                    "snapshot",
                    unix_millis()?,
                    &json!({
                        "step_id": snapshot.id,
                        "snapshot_id": snapshot_id,
                        "snapshot": snapshot_result,
                        "world_state": world_state
                    }),
                )?;
                snapshot_paths.push(snapshot_path.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.snapshot",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "step_id": snapshot.id,
                        "snapshot_id": snapshot_id,
                        "path": snapshot_path
                    }),
                )?;
            }
            ScenarioStep::Restore { restore } => {
                let snapshot_id = snapshot_ids
                    .get(&restore.id)
                    .ok_or_else(|| anyhow!("snapshot id not found for restore: {}", restore.id))?;
                let snapshot_json = serde_json::to_string(snapshot_id)
                    .context("failed to serialize snapshot id")?;
                let restored_world_state = client
                    .evaluate_json(&format!("window.__OUROFORGE__.restore({snapshot_json})"))?;
                let restore_path = write_scenario_json_artifact(
                    config,
                    scenario,
                    &scenario_dir,
                    &format!("restore-{}", restore.id),
                    "snapshot_restore",
                    unix_millis()?,
                    &json!({
                        "step_id": restore.id,
                        "snapshot_id": snapshot_id,
                        "world_state": restored_world_state
                    }),
                )?;
                snapshot_paths.push(restore_path.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.restore",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "step_id": restore.id,
                        "snapshot_id": snapshot_id,
                        "path": restore_path
                    }),
                )?;
            }
            ScenarioStep::VisualCheckpoint { visual_checkpoint } => {
                let capture = capture_visual_checkpoint(
                    config,
                    scenario,
                    &scenario_dir,
                    visual_checkpoint,
                    client,
                )?;
                visual_checkpoint_paths.push(capture.metadata_path.clone());
                visual_checkpoint_screenshot_paths.push(capture.screenshot_path.clone());
                visual_checkpoint_summaries.push(capture.summary.clone());
                append_ledger_event(
                    &config.run_dir,
                    "scenario.visual_checkpoint",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "checkpoint_id": visual_checkpoint.id,
                        "screenshot_path": capture.screenshot_path,
                        "metadata_path": capture.metadata_path,
                        "advisory": true
                    }),
                )?;
            }
            ScenarioStep::Wait { wait } => {
                execute_scenario_step(client, step)?;
                scenario_frame = scenario_frame
                    .checked_add(wait.frames)
                    .ok_or_else(|| anyhow!("scenario wait frame overflow"))?;
            }
            ScenarioStep::Input { input } => {
                pending_replay_artifacts.push(ScenarioInputReplayArtifact {
                    schema_version: SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION.to_string(),
                    scenario_id: scenario.id.clone(),
                    worker_id: None,
                    step_index,
                    action: ScenarioInputReplayAction {
                        kind: "scenario_input_step".to_string(),
                        key: None,
                        pressed: None,
                    },
                    frame: scenario_frame,
                    tick: Some(u64::from(scenario_frame)),
                    input: input.clone(),
                    probe: None,
                    result: None,
                });
                execute_scenario_step(client, step)?;
            }
            ScenarioStep::Transition { transition } => {
                execute_scenario_step(client, step)?;
                append_ledger_event(
                    &config.run_dir,
                    "scenario.transition",
                    "scenario-runner",
                    json!({
                        "scenario_id": scenario.id,
                        "transition_id": transition.id,
                        "frame": scenario_frame
                    }),
                )?;
            }
        }
    }

    let mut prior_evidence_paths = replay_paths.clone();
    prior_evidence_paths.extend(snapshot_paths.clone());
    prior_evidence_paths.extend(visual_checkpoint_paths.clone());
    prior_evidence_paths.extend(visual_checkpoint_screenshot_paths.clone());

    let world_state = match evaluate_runtime_probe_object(
        client,
        "getWorldState",
        "window.__OUROFORGE__.getWorldState()",
    ) {
        Ok(value) => value,
        Err(error) => {
            return write_runtime_probe_contract_failure_result(
                config,
                scenario,
                &scenario_dir,
                "getWorldState",
                "JSON object world state",
                error,
                prior_evidence_paths,
            );
        }
    };
    let world_state_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "world-state",
        "world_state",
        suffix,
        &world_state,
    )?;
    prior_evidence_paths.push(world_state_path.clone());
    let mut asset_load_paths = Vec::new();
    if let Some(asset_load_evidence) = runtime_asset_load_evidence_from_world_state(
        config,
        scenario,
        &world_state,
        unix_millis()?,
    )? {
        let path = write_scenario_json_artifact(
            config,
            scenario,
            &scenario_dir,
            "asset-load-evidence",
            "runtime_asset_load_evidence",
            unix_millis()?,
            &json!(asset_load_evidence),
        )?;
        prior_evidence_paths.push(path.clone());
        asset_load_paths.push(path);
    }
    let frame_stats = match evaluate_runtime_probe_object(
        client,
        "getFrameStats",
        "window.__OUROFORGE__.getFrameStats()",
    ) {
        Ok(value) => value,
        Err(error) => {
            return write_runtime_probe_contract_failure_result(
                config,
                scenario,
                &scenario_dir,
                "getFrameStats",
                "JSON object frame stats",
                error,
                prior_evidence_paths,
            );
        }
    };
    let frame_stats_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "frame-stats",
        "frame_stats",
        unix_millis()?,
        &frame_stats,
    )?;
    let mut runtime_event_paths = Vec::new();
    let runtime_event_source = match evaluate_runtime_probe_object(
        client,
        "getEvents",
        "({ events: window.__OUROFORGE__.getEvents() })",
    ) {
        Ok(value) => {
            let path = write_scenario_json_artifact(
                config,
                scenario,
                &scenario_dir,
                "runtime-events",
                "runtime_events",
                unix_millis()?,
                &value,
            )?;
            runtime_event_paths.push(path);
            value
        }
        Err(_) => json!({ "events": [] }),
    };
    let mut console_paths = Vec::new();
    let mut performance_paths = Vec::new();
    let mut trace_paths = Vec::new();
    let mut console_source = json!({ "logs": [], "count": 0 });
    if let Ok(console_logs) = client.evaluate_json("window.__OUROFORGE_CONSOLE__ || []") {
        console_source = json!({
            "logs": console_entries(&console_logs)
                .into_iter()
                .filter(|entry| entry.get("level").and_then(|value| value.as_str()) == Some("error"))
                .cloned()
                .collect::<Vec<_>>(),
            "count": console_entries(&console_logs)
                .into_iter()
                .filter(|entry| entry.get("level").and_then(|value| value.as_str()) == Some("error"))
                .count()
        });
        let console_path = write_scenario_json_artifact(
            config,
            scenario,
            &scenario_dir,
            "console-log",
            "console_log",
            unix_millis()?,
            &json!({
                "bounded": true,
                "limit": 100,
                "logs": console_logs
            }),
        )?;
        console_paths.push(console_path);
    }
    let mut scenario_performance_metric_count = None;
    let mut performance_source = json!({});
    if let Ok(performance_metrics) = client
        .enable_performance()
        .and_then(|_| client.performance_metrics())
    {
        scenario_performance_metric_count = Some(count_cdp_metrics(&performance_metrics));
        performance_source = performance_metrics_by_name(&performance_metrics);
        let performance_path = write_scenario_json_artifact(
            config,
            scenario,
            &scenario_dir,
            "performance-metrics",
            "performance_metrics",
            unix_millis()?,
            &json!({
                "bounded": true,
                "metrics": performance_metrics
            }),
        )?;
        performance_paths.push(performance_path);
    }
    let trace_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "cdp-trace-summary",
        "cdp_trace_summary",
        unix_millis()?,
        &json!({
            "bounded": true,
            "limit": 32,
            "source": "scenario-cdp-summary",
            "scenarioId": scenario.id,
            "events": [
                { "name": "Scenario.steps", "count": scenario.steps.len() },
                { "name": "Scenario.assertions", "count": scenario.assertions.len() },
                { "name": "Runtime.getWorldState", "captured": true },
                { "name": "Runtime.getFrameStats", "captured": true },
                { "name": "Performance.getMetrics", "metricCount": scenario_performance_metric_count.unwrap_or(0) }
            ]
        }),
    )?;
    trace_paths.push(trace_path.clone());

    let runtime_events = json!({
        "steps": &scenario.steps,
        "stepCount": scenario.steps.len(),
        "inputReplays": &replay_paths,
        "snapshots": &snapshot_paths,
        "events": runtime_event_source
            .get("events")
            .cloned()
            .unwrap_or_else(|| json!([]))
    });
    let collision_source = world_state
        .get("collisions")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let physics_source = world_state
        .get("physics")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let world_flags_source = world_state
        .get("goalFlags")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    // The runtime exposes audio under `audioEvents` and animation per-entity
    // under `entities[*].components.animation` (examples/game-runtime/runtime.js),
    // so point the audio/animation assertion sources at the data that actually
    // exists instead of nonexistent top-level `audio`/`animation` keys (which
    // would make every audio_evidence/animation_evidence assertion read Null).
    let audio_source = world_state
        .get("audioEvents")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let audio_warnings_source = world_state
        .get("audioWarnings")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let audio_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "audio-evidence",
        "audio_evidence",
        unix_millis()?,
        &json!({
            "bounded": true,
            "audioEvents": audio_source.clone(),
            "audioEventCount": audio_source.as_array().map_or(0, Vec::len),
            "audioWarnings": audio_warnings_source.clone(),
            "audioWarningCount": audio_warnings_source.as_array().map_or(0, Vec::len),
            "browserAudioAuthority": "intent_evidence_only"
        }),
    )?;
    let animation_source = world_state
        .get("entities")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let vfx_source = world_state
        .get("vfxEvents")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let vfx_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "vfx-evidence",
        "vfx_evidence",
        unix_millis()?,
        &json!({
            "bounded": true,
            "vfxEvents": vfx_source.clone(),
            "vfxEventCount": vfx_source.as_array().map_or(0, Vec::len)
        }),
    )?;
    let transition_source = world_state
        .get("transitionEvents")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let transition_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "transition-evidence",
        "transition_evidence",
        unix_millis()?,
        &json!({
            "bounded": true,
            "currentSceneId": world_state.get("sceneId").cloned().unwrap_or(serde_json::Value::Null),
            "declaredTransitions": world_state.get("sceneTransitions").cloned().unwrap_or_else(|| json!([])),
            "transitionEvents": transition_source.clone()
        }),
    )?;
    let scene3d_camera_source = scene3d_camera_evidence_from_world_state(&world_state);
    let scene3d_camera_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "scene3d-camera-evidence",
        "scene3d_camera",
        unix_millis()?,
        &scene3d_camera_source,
    )?;
    let scene3d_animation_source = scene3d_animation_evidence_from_world_state(&world_state);
    let scene3d_animation_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "scene3d-animation-evidence",
        "scene3d_animation",
        unix_millis()?,
        &scene3d_animation_source,
    )?;
    let scene3d_probe_source = scene3d_probe_evidence_from_world_state(&world_state);
    let scene3d_probe_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "scene3d-probe-evidence",
        "scene3d_probe",
        unix_millis()?,
        &scene3d_probe_source,
    )?;
    let scene3d_transform_source = scene3d_transform_evidence_from_world_state(&world_state);
    let scene3d_transform_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "scene3d-transform-evidence",
        "scene3d_transform",
        unix_millis()?,
        &scene3d_transform_source,
    )?;
    let scene3d_render_source = scene3d_render_evidence_from_world_state(&world_state);
    let scene3d_render_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "scene3d-render-evidence",
        "scene3d_render",
        unix_millis()?,
        &scene3d_render_source,
    )?;
    let scene3d_collision_source = scene3d_collision_evidence_from_world_state(&world_state);
    let scene3d_collision_path = write_scenario_json_artifact(
        config,
        scenario,
        &scenario_dir,
        "scene3d-collision-evidence",
        "scene3d_collision",
        unix_millis()?,
        &scene3d_collision_source,
    )?;
    let assertion_sources = ScenarioAssertionSources {
        world_state: AssertionSource {
            value: &world_state,
            evidence_ref: &world_state_path,
        },
        frame_stats: AssertionSource {
            value: &frame_stats,
            evidence_ref: &frame_stats_path,
        },
        runtime_events: AssertionSource {
            value: &runtime_events,
            evidence_ref: runtime_event_paths
                .first()
                .map_or(trace_path.as_str(), String::as_str),
        },
        world_flags: AssertionSource {
            value: &world_flags_source,
            evidence_ref: &world_state_path,
        },
        physics_evidence: AssertionSource {
            value: &physics_source,
            evidence_ref: &world_state_path,
        },
        performance_metrics: AssertionSource {
            value: &performance_source,
            evidence_ref: performance_paths
                .first()
                .map_or(frame_stats_path.as_str(), String::as_str),
        },
        console_errors: AssertionSource {
            value: &console_source,
            evidence_ref: console_paths
                .first()
                .map_or(trace_path.as_str(), String::as_str),
        },
        collision_evidence: AssertionSource {
            value: &collision_source,
            evidence_ref: &world_state_path,
        },
        audio_evidence: AssertionSource {
            value: &audio_source,
            evidence_ref: &audio_path,
        },
        animation_evidence: AssertionSource {
            value: &animation_source,
            evidence_ref: &world_state_path,
        },
        vfx_evidence: AssertionSource {
            value: &vfx_source,
            evidence_ref: &vfx_path,
        },
        transition_evidence: AssertionSource {
            value: &transition_source,
            evidence_ref: &transition_path,
        },
        scene3d_camera: AssertionSource {
            value: &scene3d_camera_source,
            evidence_ref: &scene3d_camera_path,
        },
        scene3d_animation: AssertionSource {
            value: &scene3d_animation_source,
            evidence_ref: &scene3d_animation_path,
        },
        scene3d_probe: AssertionSource {
            value: &scene3d_probe_source,
            evidence_ref: &scene3d_probe_path,
        },
        scene3d_transform: AssertionSource {
            value: &scene3d_transform_source,
            evidence_ref: &scene3d_transform_path,
        },
        scene3d_render: AssertionSource {
            value: &scene3d_render_source,
            evidence_ref: &scene3d_render_path,
        },
        scene3d_collision: AssertionSource {
            value: &scene3d_collision_source,
            evidence_ref: &scene3d_collision_path,
        },
    };
    let assertions = evaluate_scenario_assertions(scenario, &assertion_sources);
    for assertion in &assertions {
        append_ledger_event(
            &config.run_dir,
            "scenario.assertion",
            "scenario-runner",
            json!({
                "scenario_id": scenario.id,
                "target": assertion["target"],
                "path": assertion["path"],
                "passed": assertion["passed"],
                "evidence_path": assertion["evidence_ref"]
            }),
        )?;
    }
    let passed = assertions
        .iter()
        .all(|assertion| assertion["passed"].as_bool() == Some(true))
        && visual_checkpoint_summaries
            .iter()
            .all(|summary| summary["passed"].as_bool() != Some(false));
    let status = if passed { "passed" } else { "failed" };
    let result_path = format!("{scenario_dir}/scenario-result-{}.json", unix_millis()?);
    let deterministic_replay_paths = write_pending_replay_artifacts(
        config,
        scenario,
        &scenario_dir,
        pending_replay_artifacts,
        &world_state_path,
        &frame_stats_path,
        &result_path,
    )?;
    replay_paths.extend(deterministic_replay_paths.clone());
    write_json(
        &config.run_dir.join(&result_path),
        &json!({
            "scenario_id": scenario.id,
            "status": status,
            "evidence": {
                "world_state": world_state_path,
                "frame_stats": frame_stats_path,
                "runtime_events": runtime_event_paths.clone(),
                "input_replays": replay_paths.clone(),
                "scenario_input_replays": deterministic_replay_paths.clone(),
                "snapshots": snapshot_paths.clone(),
                "visual_checkpoints": visual_checkpoint_paths.clone(),
                "visual_checkpoint_screenshots": visual_checkpoint_screenshot_paths.clone(),
                "transition_evidence": transition_path.clone(),
                "scene3d_camera": scene3d_camera_path.clone(),
                "scene3d_animation": scene3d_animation_path.clone(),
                "scene3d_probe": scene3d_probe_path.clone(),
                "scene3d_transform": scene3d_transform_path.clone(),
                "scene3d_render": scene3d_render_path.clone(),
                "scene3d_collision": scene3d_collision_path.clone(),
                "audio_evidence": audio_path.clone(),
                "vfx_evidence": vfx_path.clone(),
                "asset_load_evidence": asset_load_paths.clone(),
                "console_logs": console_paths.clone(),
                "performance_metrics": performance_paths.clone(),
                "cdp_trace_summaries": trace_paths.clone()
            },
            "assertions": assertions,
            "visual_checkpoints": visual_checkpoint_summaries
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("scenario-result-{}-{}", scenario.id, unix_millis()?),
        "application/json",
        &result_path,
        json!({ "scenario_id": scenario.id, "url": config.url, "artifact": "scenario_result", "status": status }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "scenario.completed",
        "scenario-runner",
        json!({
            "scenario_id": scenario.id,
            "status": status,
            "world_state_path": world_state_path,
            "frame_stats_path": frame_stats_path,
            "input_replay_paths": replay_paths.clone(),
            "scenario_input_replay_paths": deterministic_replay_paths.clone(),
            "snapshot_paths": snapshot_paths.clone(),
            "visual_checkpoint_paths": visual_checkpoint_paths.clone(),
            "visual_checkpoint_screenshot_paths": visual_checkpoint_screenshot_paths.clone(),
            "transition_evidence_path": transition_path.clone(),
            "scene3d_camera_path": scene3d_camera_path.clone(),
            "scene3d_animation_path": scene3d_animation_path.clone(),
            "scene3d_probe_path": scene3d_probe_path.clone(),
            "scene3d_transform_path": scene3d_transform_path.clone(),
            "scene3d_render_path": scene3d_render_path.clone(),
            "scene3d_collision_path": scene3d_collision_path.clone(),
            "audio_evidence_path": audio_path.clone(),
            "vfx_evidence_path": vfx_path.clone(),
            "asset_load_evidence_paths": asset_load_paths.clone(),
            "console_log_paths": console_paths.clone(),
            "performance_metric_paths": performance_paths.clone(),
            "cdp_trace_summary_paths": trace_paths.clone(),
            "result_path": result_path
        }),
    )?;
    let mut evidence_paths = replay_paths;
    evidence_paths.extend(snapshot_paths);
    evidence_paths.extend(visual_checkpoint_paths);
    evidence_paths.extend(visual_checkpoint_screenshot_paths);
    evidence_paths.push(world_state_path);
    evidence_paths.push(transition_path);
    evidence_paths.push(scene3d_camera_path);
    evidence_paths.push(scene3d_animation_path);
    evidence_paths.push(scene3d_probe_path);
    evidence_paths.push(scene3d_transform_path);
    evidence_paths.push(scene3d_render_path);
    evidence_paths.push(scene3d_collision_path);
    evidence_paths.push(audio_path);
    evidence_paths.push(vfx_path);
    evidence_paths.extend(asset_load_paths);
    evidence_paths.push(frame_stats_path);
    evidence_paths.extend(console_paths);
    evidence_paths.extend(performance_paths);
    evidence_paths.extend(trace_paths);
    Ok(ScenarioExecutionResult {
        passed,
        evidence_paths,
        result_path,
    })
}

fn pending_replay_artifacts_from_replay(
    scenario: &Scenario,
    step_index: usize,
    base_frame: u32,
    replay: &InputReplay,
    action_kind: &str,
) -> Result<Vec<ScenarioInputReplayArtifact>> {
    replay.validate()?;
    let mut artifacts = Vec::new();
    let mut index = 0;
    while index < replay.events.len() {
        let frame = replay.events[index].frame;
        let absolute_frame = base_frame
            .checked_add(frame)
            .ok_or_else(|| anyhow!("scenario replay frame overflow"))?;
        let mut input = InputStep::default();
        let first_event = replay.events[index].clone();
        let mut event_count = 0usize;
        while index < replay.events.len() && replay.events[index].frame == frame {
            let event = &replay.events[index];
            match event.key {
                ReplayKey::Left => input.left = Some(event.pressed),
                ReplayKey::Right => input.right = Some(event.pressed),
                ReplayKey::Up => input.up = Some(event.pressed),
                ReplayKey::Down => input.down = Some(event.pressed),
            }
            event_count += 1;
            index += 1;
        }
        artifacts.push(ScenarioInputReplayArtifact {
            schema_version: SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION.to_string(),
            scenario_id: scenario.id.clone(),
            worker_id: None,
            step_index,
            action: ScenarioInputReplayAction {
                kind: action_kind.to_string(),
                key: (event_count == 1).then_some(first_event.key),
                pressed: (event_count == 1).then_some(first_event.pressed),
            },
            frame: absolute_frame,
            tick: Some(u64::from(absolute_frame)),
            input,
            probe: None,
            result: None,
        });
    }
    Ok(artifacts)
}

fn write_pending_replay_artifacts(
    config: &ScenarioRunConfig,
    scenario: &Scenario,
    scenario_dir: &str,
    artifacts: Vec<ScenarioInputReplayArtifact>,
    world_state_path: &str,
    frame_stats_path: &str,
    result_path: &str,
) -> Result<Vec<String>> {
    let mut paths = Vec::new();
    for (artifact_index, mut artifact) in artifacts.into_iter().enumerate() {
        artifact.probe = Some(ScenarioInputReplayProbeCorrelation {
            contract_version: RUNTIME_PROBE_CONTRACT_VERSION.to_string(),
            world_state_path: Some(world_state_path.to_string()),
            frame_stats_path: Some(frame_stats_path.to_string()),
        });
        artifact.result = Some(ScenarioInputReplayResultCorrelation {
            scenario_result_path: result_path.to_string(),
            verdict_path: None,
        });
        artifact.validate()?;
        let replay_path = write_scenario_json_artifact(
            config,
            scenario,
            scenario_dir,
            &format!(
                "scenario-input-replay-step-{}-frame-{}",
                artifact.step_index, artifact.frame
            ),
            "scenario_input_replay",
            unix_millis()? * 1000 + artifact_index as u128,
            &json!(artifact),
        )?;
        append_ledger_event(
            &config.run_dir,
            "scenario.input_replay_artifact",
            "scenario-runner",
            json!({
                "scenario_id": scenario.id,
                "step_index": artifact.step_index,
                "frame": artifact.frame,
                "path": replay_path,
                "artifact": "scenario_input_replay"
            }),
        )?;
        paths.push(replay_path);
    }
    Ok(paths)
}

fn evaluate_runtime_probe_object<T: CdpTransport>(
    client: &mut CdpClient<T>,
    method: &str,
    expression: &str,
) -> Result<serde_json::Value> {
    let value = client
        .evaluate_json(expression)
        .with_context(|| format!("runtime probe {method} failed"))?;
    if !value.is_object() {
        return Err(anyhow!(
            "runtime probe {method} returned malformed response; expected JSON object, found {}",
            json_type_name(&value)
        ));
    }
    Ok(value)
}

fn write_runtime_probe_contract_failure_result(
    config: &ScenarioRunConfig,
    scenario: &Scenario,
    scenario_dir: &str,
    method: &str,
    expected: &str,
    error: anyhow::Error,
    mut evidence_paths: Vec<String>,
) -> Result<ScenarioExecutionResult> {
    let suffix = unix_millis()?;
    let failure_path = format!("{scenario_dir}/runtime-probe-contract-failure-{suffix}.json");
    let failure = json!({
        "artifact": "runtime_probe_contract_failure",
        "scenario_id": scenario.id,
        "status": "failed",
        "method": method,
        "expected": expected,
        "error": error.to_string(),
        "probe_contract": {
            "name": RUNTIME_PROBE_CONTRACT_NAME,
            "version": RUNTIME_PROBE_CONTRACT_VERSION
        }
    });
    write_json(&config.run_dir.join(&failure_path), &failure)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "scenario-runtime-probe-contract-failure-{}-{suffix}",
            scenario.id
        ),
        "application/json",
        &failure_path,
        json!({
            "scenario_id": scenario.id,
            "url": config.url,
            "artifact": "runtime_probe_contract_failure",
            "status": "failed",
            "probe_call": method,
            "probe_contract": {
                "name": RUNTIME_PROBE_CONTRACT_NAME,
                "version": RUNTIME_PROBE_CONTRACT_VERSION
            }
        }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "scenario.probe_contract_failed",
        "scenario-runner",
        json!({
            "scenario_id": scenario.id,
            "status": "failed",
            "probe_call": method,
            "expected": expected,
            "error": failure["error"],
            "path": failure_path,
            "probe_contract": failure["probe_contract"]
        }),
    )?;

    let result_path = format!("{scenario_dir}/scenario-result-{}.json", unix_millis()?);
    write_json(
        &config.run_dir.join(&result_path),
        &json!({
            "scenario_id": scenario.id,
            "status": "failed",
            "evidence": {
                "runtime_probe_contract_failure": failure_path
            },
            "assertions": [],
            "probe_contract_failure": failure
        }),
    )?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("scenario-result-{}-{}", scenario.id, unix_millis()?),
        "application/json",
        &result_path,
        json!({
            "scenario_id": scenario.id,
            "url": config.url,
            "artifact": "scenario_result",
            "status": "failed",
            "probe_contract_failure": true
        }),
    )?;
    append_ledger_event(
        &config.run_dir,
        "scenario.completed",
        "scenario-runner",
        json!({
            "scenario_id": scenario.id,
            "status": "failed",
            "runtime_probe_contract_failure_path": failure_path,
            "result_path": result_path
        }),
    )?;

    evidence_paths.push(failure_path);
    Ok(ScenarioExecutionResult {
        passed: false,
        evidence_paths,
        result_path,
    })
}

fn write_scenario_json_artifact(
    config: &ScenarioRunConfig,
    scenario: &Scenario,
    scenario_dir: &str,
    file_prefix: &str,
    artifact_name: &str,
    suffix: u128,
    value: &serde_json::Value,
) -> Result<String> {
    let rel_path = format!("{scenario_dir}/{file_prefix}-{suffix}.json");
    write_json(&config.run_dir.join(&rel_path), value)?;
    add_evidence_artifact(
        &config.run_dir,
        &format!("scenario-{artifact_name}-{}-{suffix}", scenario.id),
        "application/json",
        &rel_path,
        json!({ "scenario_id": scenario.id, "url": config.url, "artifact": artifact_name }),
    )?;
    Ok(rel_path)
}

fn capture_visual_checkpoint<T: CdpTransport>(
    config: &ScenarioRunConfig,
    scenario: &Scenario,
    scenario_dir: &str,
    checkpoint: &VisualCheckpointStep,
    client: &mut CdpClient<T>,
) -> Result<VisualCheckpointCapture> {
    let suffix = unix_millis()?;
    let screenshot = client.capture_screenshot_png()?;
    let dimensions = png_dimensions(&screenshot);
    let comparison = visual_comparison_summary(checkpoint, dimensions);
    let passed = comparison
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(true);
    let screenshot_path = format!(
        "{scenario_dir}/visual-checkpoint-{}-{suffix}.png",
        checkpoint.id
    );
    let full_screenshot_path = config.run_dir.join(&screenshot_path);
    fs::write(&full_screenshot_path, screenshot).with_context(|| {
        format!(
            "failed to write visual checkpoint screenshot {}",
            full_screenshot_path.display()
        )
    })?;
    add_evidence_artifact(
        &config.run_dir,
        &format!(
            "scenario-visual-checkpoint-screenshot-{}-{}-{suffix}",
            scenario.id, checkpoint.id
        ),
        "image/png",
        &screenshot_path,
        json!({
            "scenario_id": scenario.id,
            "checkpoint_id": checkpoint.id,
            "url": config.url,
            "artifact": "visual_checkpoint_screenshot",
            "advisory": checkpoint.threshold.is_none()
        }),
    )?;
    let summary = json!({
        "checkpoint_id": checkpoint.id,
        "screenshot_path": screenshot_path,
        "metadata_path": format!("{scenario_dir}/visual-checkpoint-{}-{suffix}.json", checkpoint.id),
        "advisory": checkpoint.threshold.is_none(),
        "passed": passed,
        "evidence_ref": screenshot_path,
        "comparison": comparison
    });

    let metadata_path = write_scenario_json_artifact(
        config,
        scenario,
        scenario_dir,
        &format!("visual-checkpoint-{}", checkpoint.id),
        "visual_checkpoint",
        suffix,
        &json!({
            "checkpoint_id": checkpoint.id,
            "screenshot_path": screenshot_path,
            "advisory": checkpoint.threshold.is_none(),
            "passed": passed,
            "dimensions": dimensions.map(|(width, height)| json!({ "width": width, "height": height })),
            "comparison": comparison,
            "baseline": checkpoint.baseline
        }),
    )?;
    Ok(VisualCheckpointCapture {
        screenshot_path,
        metadata_path,
        summary,
    })
}

fn wait_for_runtime_step_api<T: CdpTransport>(client: &mut CdpClient<T>) -> Result<()> {
    let expression = "Boolean(window.__OUROFORGE__ && typeof window.__OUROFORGE__.step === 'function' && typeof window.__OUROFORGE__.setInput === 'function')";
    let mut last_value = serde_json::Value::Null;
    for _ in 0..30 {
        last_value = client.evaluate_json(expression)?;
        if last_value == json!(true) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err(anyhow!(
        "window.__OUROFORGE__ step/input API not ready for scenario step; last readiness value: {}",
        last_value
    ))
}

fn wait_for_runtime_transition_api<T: CdpTransport>(client: &mut CdpClient<T>) -> Result<()> {
    let expression =
        "Boolean(window.__OUROFORGE__ && typeof window.__OUROFORGE__.transition === 'function')";
    let mut last_value = serde_json::Value::Null;
    for _ in 0..30 {
        last_value = client.evaluate_json(expression)?;
        if last_value == json!(true) {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err(anyhow!(
        "window.__OUROFORGE__ transition API not ready for scenario step; last readiness value: {}",
        last_value
    ))
}

fn execute_scenario_step<T: CdpTransport>(
    client: &mut CdpClient<T>,
    step: &ScenarioStep,
) -> Result<()> {
    match step {
        ScenarioStep::Wait { wait } => {
            wait_for_runtime_step_api(client)?;
            client.evaluate_json(&format!("window.__OUROFORGE__.step({})", wait.frames))?;
        }
        ScenarioStep::Input { input } => {
            wait_for_runtime_step_api(client)?;
            let input_json =
                serde_json::to_string(input).context("failed to serialize input step")?;
            client.evaluate_json(&format!("window.__OUROFORGE__.setInput({input_json})"))?;
        }
        ScenarioStep::Transition { transition } => {
            wait_for_runtime_transition_api(client)?;
            validate_path_component("scenario transition id", &transition.id)?;
            let transition_json = serde_json::to_string(&transition.id)
                .context("failed to serialize transition id")?;
            client.evaluate_json_await(&format!(
                "window.__OUROFORGE__.transition({transition_json})"
            ))?;
        }
        ScenarioStep::Replay { replay } => {
            execute_input_replay(client, replay)?;
        }
        ScenarioStep::ReplayRef { .. } => {
            return Err(anyhow!(
                "replayRef execution is implemented by scenario evidence context"
            ));
        }
        ScenarioStep::Snapshot { .. }
        | ScenarioStep::Restore { .. }
        | ScenarioStep::VisualCheckpoint { .. } => {
            return Err(anyhow!(
                "snapshot/restore/visual checkpoint steps require scenario evidence context"
            ));
        }
    }
    Ok(())
}

fn execute_input_replay<T: CdpTransport>(
    client: &mut CdpClient<T>,
    replay: &InputReplay,
) -> Result<()> {
    replay.validate()?;
    let mut current_frame = 0;
    let mut index = 0;
    while index < replay.events.len() {
        let frame = replay.events[index].frame;
        if frame > current_frame {
            wait_for_runtime_step_api(client)?;
            client.evaluate_json(&format!(
                "window.__OUROFORGE__.step({})",
                frame - current_frame
            ))?;
            current_frame = frame;
        }

        let mut patch = InputStep::default();
        while index < replay.events.len() && replay.events[index].frame == frame {
            let event = &replay.events[index];
            match event.key {
                ReplayKey::Left => patch.left = Some(event.pressed),
                ReplayKey::Right => patch.right = Some(event.pressed),
                ReplayKey::Up => patch.up = Some(event.pressed),
                ReplayKey::Down => patch.down = Some(event.pressed),
            }
            index += 1;
        }
        let input_json =
            serde_json::to_string(&patch).context("failed to serialize replay input patch")?;
        wait_for_runtime_step_api(client)?;
        client.evaluate_json(&format!("window.__OUROFORGE__.setInput({input_json})"))?;
    }
    Ok(())
}

pub fn validate_visual_edit_draft_review_preflight(
    run_dir: impl AsRef<Path>,
    draft: &VisualEditDraftArtifact,
) -> Result<VisualEditDraftReviewPreflight> {
    let run_dir = run_dir.as_ref();
    draft.validate()?;
    let review_gate = draft
        .review_gate
        .as_ref()
        .ok_or_else(|| anyhow!("visual edit draft review preflight requires reviewGate linkage"))?;
    review_gate.validate()?;

    let proposals = read_mutation_proposals(run_dir)?;
    let proposal = proposals
        .proposals
        .iter()
        .find(|proposal| proposal.id == review_gate.proposal_id)
        .ok_or_else(|| {
            anyhow!(
                "visual edit draft review preflight proposal id not found: {}",
                review_gate.proposal_id
            )
        })?;
    if proposal.status != "proposed" {
        return Err(anyhow!(
            "visual edit draft review preflight proposal {} must remain proposed; found {}",
            proposal.id,
            proposal.status
        ));
    }

    let patch_drafts = read_patch_draft_artifact(run_dir)?;
    let patch_draft = patch_drafts
        .drafts
        .iter()
        .find(|patch_draft| patch_draft.id == review_gate.patch_draft_id)
        .ok_or_else(|| {
            anyhow!(
                "visual edit draft review preflight patch draft id not found: {}",
                review_gate.patch_draft_id
            )
        })?;
    if patch_draft.proposal_id != review_gate.proposal_id {
        return Err(anyhow!(
            "visual edit draft review preflight patch draft {} is linked to proposal {}, not {}",
            patch_draft.id,
            patch_draft.proposal_id,
            review_gate.proposal_id
        ));
    }

    let review = read_mutation_review_artifact(run_dir)?;
    let decision = review
        .decisions
        .iter()
        .find(|decision| decision.id == review_gate.review_decision_id)
        .ok_or_else(|| {
            anyhow!(
                "visual edit draft review preflight decision id not found: {}",
                review_gate.review_decision_id
            )
        })?;
    validate_accepted_mutation_review_decision(
        run_dir,
        decision,
        &review_gate.proposal_id,
        Some(&review_gate.patch_draft_id),
        true,
    )?;

    Ok(VisualEditDraftReviewPreflight {
        draft_id: draft.draft_id.clone(),
        proposal_id: review_gate.proposal_id.clone(),
        patch_draft_id: review_gate.patch_draft_id.clone(),
        review_decision_id: review_gate.review_decision_id.clone(),
        status: "review_preflight_passed".to_string(),
    })
}

fn validate_accepted_mutation_review_decision(
    run_dir: &Path,
    decision: &MutationReviewDecision,
    expected_proposal_id: &str,
    expected_patch_draft_id: Option<&str>,
    require_expected_hashes: bool,
) -> Result<()> {
    if decision.state != MutationReviewState::Accepted
        || decision.decision_status.as_ref() != Some(&MutationReviewState::Accepted)
    {
        return Err(anyhow!(
            "review-gated preflight requires an accepted decision; found {:?}",
            decision.decision_status.as_ref().unwrap_or(&decision.state)
        ));
    }
    if decision.proposal_id.as_deref() != Some(expected_proposal_id) {
        return Err(anyhow!(
            "review-gated preflight decision {} is not linked to proposal {}",
            decision.id,
            expected_proposal_id
        ));
    }
    if let Some(expected_patch_draft_id) = expected_patch_draft_id {
        if decision.patch_draft_id != expected_patch_draft_id {
            return Err(anyhow!(
                "review-gated preflight decision {} is linked to patch draft {}, not {}",
                decision.id,
                decision.patch_draft_id,
                expected_patch_draft_id
            ));
        }
    }
    if decision.evidence_refs.is_empty() {
        return Err(anyhow!(
            "review-gated preflight decision {} requires evidence refs",
            decision.id
        ));
    }
    let checklist = decision.guardrail_checklist.as_ref().ok_or_else(|| {
        anyhow!(
            "review-gated preflight decision {} requires a guardrail checklist",
            decision.id
        )
    })?;
    checklist.validate()?;
    let Some(expected_hashes) = decision.expected_hashes.as_ref() else {
        if require_expected_hashes {
            return Err(anyhow!(
                "review-gated preflight decision {} requires expected proposal, patch draft, and evidence hashes",
                decision.id
            ));
        }
        return Ok(());
    };
    if let Some(expected) = expected_hashes.proposal_index_hash.as_deref() {
        let actual = canonical_json_digest(json!(read_mutation_proposals(run_dir)?))?;
        if expected != actual {
            return Err(anyhow!(
                "review-gated preflight decision {} proposal index hash mismatch; expected {}, found {}",
                decision.id, expected, actual
            ));
        }
    } else if require_expected_hashes {
        return Err(anyhow!(
            "review-gated preflight decision {} requires proposal index hash",
            decision.id
        ));
    }
    if let Some(expected) = expected_hashes.patch_draft_hash.as_deref() {
        let actual = canonical_json_digest(json!(read_patch_draft_artifact(run_dir)?))?;
        if expected != actual {
            return Err(anyhow!(
                "review-gated preflight decision {} patch draft hash mismatch; expected {}, found {}",
                decision.id, expected, actual
            ));
        }
    } else if require_expected_hashes {
        return Err(anyhow!(
            "review-gated preflight decision {} requires patch draft hash",
            decision.id
        ));
    }
    if let Some(expected) = expected_hashes.evidence_index_hash.as_deref() {
        let actual = canonical_json_digest(json!(read_evidence_index(run_dir)?))?;
        if expected != actual {
            return Err(anyhow!(
                "review-gated preflight decision {} evidence index hash mismatch; expected {}, found {}",
                decision.id, expected, actual
            ));
        }
    } else if require_expected_hashes {
        return Err(anyhow!(
            "review-gated preflight decision {} requires evidence index hash",
            decision.id
        ));
    }
    Ok(())
}

fn validate_review_gated_scene_apply_decision(
    run_dir: &Path,
    operation: &SceneOnlyMutationOperation,
) -> Result<Option<String>> {
    let Some(decision_id) = operation.review_decision_id.as_deref() else {
        return Ok(None);
    };
    require_text("scene-only mutation reviewDecisionId", decision_id)?;
    let review = read_mutation_review_artifact(run_dir)?;
    let decision = review
        .decisions
        .iter()
        .find(|decision| decision.id == decision_id)
        .ok_or_else(|| {
            anyhow!(
                "review-gated scene apply decision id not found: {}",
                decision_id
            )
        })?;
    if decision.state != MutationReviewState::Accepted
        || decision.decision_status.as_ref() != Some(&MutationReviewState::Accepted)
    {
        return Err(anyhow!(
            "review-gated scene apply requires an accepted decision; found {:?}",
            decision.decision_status.as_ref().unwrap_or(&decision.state)
        ));
    }
    if decision.proposal_id.as_deref() != Some(operation.proposal_id.as_str()) {
        return Err(anyhow!(
            "review-gated scene apply decision {} is not linked to proposal {}",
            decision_id,
            operation.proposal_id
        ));
    }
    if decision.evidence_refs.is_empty() {
        return Err(anyhow!(
            "review-gated scene apply decision {} requires evidence refs",
            decision_id
        ));
    }
    let checklist = decision.guardrail_checklist.as_ref().ok_or_else(|| {
        anyhow!(
            "review-gated scene apply decision {} requires a guardrail checklist",
            decision_id
        )
    })?;
    checklist.validate()?;
    if let Some(expected_hashes) = &decision.expected_hashes {
        if let Some(expected) = expected_hashes.proposal_index_hash.as_deref() {
            let actual = canonical_json_digest(json!(read_mutation_proposals(run_dir)?))?;
            if expected != actual {
                return Err(anyhow!(
                    "review-gated scene apply decision {} proposal index hash mismatch; expected {}, found {}",
                    decision_id, expected, actual
                ));
            }
        }
        if let Some(expected) = expected_hashes.patch_draft_hash.as_deref() {
            let actual = canonical_json_digest(json!(read_patch_draft_artifact(run_dir)?))?;
            if expected != actual {
                return Err(anyhow!(
                    "review-gated scene apply decision {} patch draft hash mismatch; expected {}, found {}",
                    decision_id, expected, actual
                ));
            }
        }
        if let Some(expected) = expected_hashes.evidence_index_hash.as_deref() {
            let actual = canonical_json_digest(json!(read_evidence_index(run_dir)?))?;
            if expected != actual {
                return Err(anyhow!(
                    "review-gated scene apply decision {} evidence index hash mismatch; expected {}, found {}",
                    decision_id, expected, actual
                ));
            }
        }
    }
    let applications = read_scene_only_mutation_applications(run_dir)?;
    if applications
        .applications
        .iter()
        .any(|application| application.review_decision_id.as_deref() == Some(decision_id))
    {
        return Err(anyhow!(
            "review-gated scene apply decision {} was already used",
            decision_id
        ));
    }
    if applications.applications.iter().any(|application| {
        application.proposal_id == operation.proposal_id && application.status == "applied"
    }) {
        return Err(anyhow!(
            "review-gated scene apply proposal {} already has an applied scene mutation",
            operation.proposal_id
        ));
    }
    Ok(Some(decision_id.to_string()))
}

pub fn validate_scene_only_mutation_operation(
    run_dir: impl AsRef<Path>,
    operation: &SceneOnlyMutationOperation,
) -> Result<SceneOnlyMutationValidation> {
    let run_dir = run_dir.as_ref();
    if operation.schema_version != "scene-only-mutation-v1" {
        return Err(anyhow!(
            "scene-only mutation schemaVersion must be scene-only-mutation-v1"
        ));
    }
    if !operation.validation_required {
        return Err(anyhow!(
            "scene-only mutation requires validationRequired=true"
        ));
    }
    require_text("scene-only mutation proposalId", &operation.proposal_id)?;
    require_text(
        "scene-only mutation targetScenePath",
        &operation.target_scene_path,
    )?;
    if !supported_scene_edit_paths().contains(&operation.edit.path.as_str()) {
        return Err(anyhow!(
            "scene-only mutation edit path is not allowed: {}",
            operation.edit.path
        ));
    }
    let proposals = read_mutation_proposals(run_dir)?;
    let proposal = proposals
        .proposals
        .iter()
        .find(|proposal| proposal.id == operation.proposal_id)
        .ok_or_else(|| {
            anyhow!(
                "scene-only mutation proposal id not found: {}",
                operation.proposal_id
            )
        })?;
    if proposal.status != "proposed" {
        return Err(anyhow!(
            "scene-only mutation requires a proposed mutation; found status {}",
            proposal.status
        ));
    }
    let scene = read_scene(&operation.target_scene_path)?;
    let before_scene_hash = hash_scene_document(&scene)?;
    let project = validate_project_scene_mutation_context(operation, &before_scene_hash)?;
    let review_decision_id = validate_review_gated_scene_apply_decision(run_dir, operation)?;
    if before_scene_hash != operation.expected_before_scene_hash {
        return Err(anyhow!(
            "scene-only mutation before hash mismatch; expected {}, found {}",
            operation.expected_before_scene_hash.value,
            before_scene_hash.value
        ));
    }
    let mut candidate_scene = scene.clone();
    apply_scene_edit(&mut candidate_scene, operation.edit.clone())?;
    validate_scene(&candidate_scene).context("scene-only mutation candidate validation failed")?;
    Ok(SceneOnlyMutationValidation {
        status: "passed".to_string(),
        proposal_id: operation.proposal_id.clone(),
        target_scene_path: operation.target_scene_path.clone(),
        before_scene_hash,
        allowed_path: true,
        review_decision_id,
        project,
    })
}

pub fn write_source_apply_worktree_context_evidence(
    run_dir: impl AsRef<Path>,
    input: &SourceApplyWorktreeContextInput,
) -> Result<SourceApplyWorktreeContextReport> {
    let run_dir = run_dir.as_ref();
    let report = inspect_source_apply_worktree_context(input);
    let artifact_path = "evidence/source-apply/worktree-context.json";
    if let Some(parent) = run_dir.join(artifact_path).parent() {
        fs::create_dir_all(parent)?;
    }
    write_json(&run_dir.join(artifact_path), &json!(report))?;
    add_evidence_artifact(
        run_dir,
        "source-apply-worktree-context",
        "application/json",
        artifact_path,
        json!({
            "artifact": "source_apply_worktree_context",
            "schemaVersion": report.schema_version,
            "status": report.status,
            "policyId": report.policy_id,
            "boundary": "read-only context evidence; no trusted source apply was performed"
        }),
    )?;
    Ok(report)
}

pub fn apply_scene_only_mutation_operation(
    run_dir: impl AsRef<Path>,
    operation: &SceneOnlyMutationOperation,
    transaction_output: impl AsRef<Path>,
) -> Result<SceneEditTransaction> {
    let run_dir = run_dir.as_ref();
    let transaction_output = transaction_output.as_ref();
    if operation.schema_version != "scene-only-mutation-v1" {
        return Err(anyhow!(
            "scene-only mutation schemaVersion must be scene-only-mutation-v1"
        ));
    }
    if !operation.validation_required {
        return Err(anyhow!(
            "scene-only mutation requires validationRequired=true"
        ));
    }
    if !supported_scene_edit_paths().contains(&operation.edit.path.as_str()) {
        return Err(anyhow!(
            "scene-only mutation edit path is not allowed: {}",
            operation.edit.path
        ));
    }
    let proposals = read_mutation_proposals(run_dir)?;
    let proposal = proposals
        .proposals
        .iter()
        .find(|proposal| proposal.id == operation.proposal_id)
        .ok_or_else(|| {
            anyhow!(
                "scene-only mutation proposal id not found: {}",
                operation.proposal_id
            )
        })?;
    if proposal.status != "proposed" {
        return Err(anyhow!(
            "scene-only mutation requires a proposed mutation; found status {}",
            proposal.status
        ));
    }
    let scene = read_scene(&operation.target_scene_path)?;
    let before_scene_hash = hash_scene_document(&scene)?;
    let project = validate_project_scene_mutation_context(operation, &before_scene_hash)?;
    let review_decision_id = validate_review_gated_scene_apply_decision(run_dir, operation)?;
    if before_scene_hash != operation.expected_before_scene_hash {
        return Err(anyhow!(
            "scene-only mutation before hash mismatch; expected {}, found {}",
            operation.expected_before_scene_hash.value,
            before_scene_hash.value
        ));
    }
    reject_scene_mutation_transaction_output_collision(
        transaction_output,
        Path::new(&operation.target_scene_path),
    )?;
    let transaction =
        preview_scene_edit_transaction(&operation.target_scene_path, operation.edit.clone())?;
    write_scene_edit_transaction_artifact(transaction_output, &transaction)?;
    if transaction.validation_result.status != "passed" {
        return Err(anyhow!(
            "scene-only mutation transaction failed validation; artifact written to {}",
            transaction_output.display()
        ));
    }
    edit_scene(&operation.target_scene_path, operation.edit.clone())?;
    let after_scene_hash = transaction
        .after_scene_hash
        .clone()
        .ok_or_else(|| anyhow!("passed scene-only transaction missing afterSceneHash"))?;
    let application = append_scene_only_mutation_application(
        run_dir,
        operation,
        &transaction,
        transaction_output,
        &after_scene_hash,
    )?;
    append_ledger_event(
        run_dir,
        "mutation.scene_applied",
        "mutation-cli",
        json!({
            "proposal_id": operation.proposal_id,
            "decision_id": review_decision_id,
            "application_id": application.id,
            "transaction_id": transaction.id,
            "transaction_artifact_path": transaction_output.to_string_lossy(),
            "target_scene_path": operation.target_scene_path,
            "before_scene_hash": transaction.before_scene_hash,
            "after_scene_hash": after_scene_hash,
            "project": project
        }),
    )?;
    Ok(transaction)
}

pub fn append_visual_edit_draft_application(
    run_dir: impl AsRef<Path>,
    draft: &VisualEditDraftArtifact,
    transaction: &SceneEditTransaction,
    transaction_output: impl AsRef<Path>,
    target_scene_path: impl AsRef<Path>,
    command_context: VisualEditDraftApplyCommandContext,
) -> Result<VisualEditDraftApplicationRecord> {
    let run_dir = run_dir.as_ref();
    let preflight = validate_visual_edit_draft_review_preflight(run_dir, draft)?;
    let after_scene_hash = transaction
        .after_scene_hash
        .clone()
        .ok_or_else(|| anyhow!("visual edit draft application requires afterSceneHash"))?;
    reject_already_applied_visual_edit_draft_decision(run_dir, &preflight.review_decision_id)?;
    let mut index = read_visual_edit_draft_applications(run_dir)?;
    let record = VisualEditDraftApplicationRecord {
        id: format!(
            "visual-edit-application-{}-{}",
            unix_millis()?,
            index.applications.len() + 1
        ),
        draft_id: preflight.draft_id,
        proposal_id: preflight.proposal_id,
        patch_draft_id: preflight.patch_draft_id,
        review_decision_id: preflight.review_decision_id,
        transaction_id: transaction.id.clone(),
        transaction_artifact_path: transaction_output.as_ref().to_string_lossy().to_string(),
        target_scene_path: target_scene_path.as_ref().to_string_lossy().to_string(),
        before_scene_hash: transaction.before_scene_hash.clone(),
        after_scene_hash,
        rollback: transaction.rollback.clone(),
        command_context,
        status: "applied".to_string(),
        created_at_unix_ms: unix_millis()?,
    };
    index.applications.push(record.clone());
    write_visual_edit_draft_applications(run_dir, &index)?;
    append_ledger_event(
        run_dir,
        "visual_edit_draft.applied",
        "mutation-cli",
        json!({
            "application_id": record.id,
            "draft_id": record.draft_id,
            "proposal_id": record.proposal_id,
            "patch_draft_id": record.patch_draft_id,
            "decision_id": record.review_decision_id,
            "transaction_id": record.transaction_id,
            "transaction_artifact_path": record.transaction_artifact_path,
            "target_scene_path": record.target_scene_path,
            "before_scene_hash": record.before_scene_hash,
            "after_scene_hash": record.after_scene_hash,
        }),
    )?;
    Ok(record)
}

pub fn bind_run_transaction_provenance(
    run_dir: impl AsRef<Path>,
    transaction_path: impl AsRef<Path>,
) -> Result<RunTransactionProvenance> {
    let run_dir = run_dir.as_ref();
    let provenance = run_transaction_provenance_from_artifact(&transaction_path)?;
    let scene = read_scene(&provenance.scene_path).with_context(|| {
        format!(
            "failed to read transaction source scene {}",
            provenance.scene_path
        )
    })?;
    let current_hash = hash_scene_document(&scene)?;
    if current_hash != provenance.after_scene_hash {
        return Err(anyhow!(
            "transaction source scene hash mismatch for {}; expected afterSceneHash {}, found {}",
            provenance.transaction_id,
            provenance.after_scene_hash.value,
            current_hash.value
        ));
    }
    let run_path = run_dir.join("run.json");
    let mut run = read_json_value(&run_path)?;
    let run_object = run
        .as_object_mut()
        .ok_or_else(|| anyhow!("run.json must be a JSON object"))?;
    run_object.insert(
        "transaction_provenance".to_string(),
        json!(provenance.clone()),
    );
    write_json_atomic(&run_path, &run)?;
    append_ledger_event(
        run_dir,
        "run.transaction_bound",
        "run-cli",
        json!({
            "transaction_id": provenance.transaction_id,
            "transaction_artifact_path": provenance.transaction_artifact_path,
            "scene_path": provenance.scene_path,
            "before_scene_hash": provenance.before_scene_hash,
            "after_scene_hash": provenance.after_scene_hash
        }),
    )?;
    Ok(provenance)
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardReadModel {
    pub summary: RunDashboardSummary,
    pub run: serde_json::Value,
    pub project: Option<ProjectRunMetadata>,
    pub command_context: Option<RunCommandContext>,
    pub verdict: serde_json::Value,
    pub journal: String,
    pub journal_view: RunDashboardJournal,
    pub evidence: Vec<EvidenceArtifact>,
    pub screenshots: Vec<RunDashboardArtifact>,
    pub console_logs: Vec<RunDashboardArtifact>,
    pub cdp_trace_summaries: Vec<RunDashboardArtifact>,
    pub world_states: Vec<RunDashboardArtifact>,
    pub frame_metrics: Vec<RunDashboardArtifact>,
    pub performance_metrics: Vec<RunDashboardArtifact>,
    pub scenario_results: Vec<RunDashboardArtifact>,
    pub scenario_assertions: RunDashboardScenarioAssertions,
    pub behavior_assertions: RunDashboardBehaviorAssertions,
    pub behavior_evidence: behavior_evidence::RunDashboardBehaviorEvidence,
    pub mutation_artifacts: Vec<RunDashboardArtifact>,
    pub mutation_lifecycle: RunDashboardMutationLifecycle,
    pub review_cockpit: RunDashboardReviewCockpit,
    pub regression_promotions: Vec<RegressionPromotionPackResult>,
    pub replay: RunDashboardReplay,
    pub comparison: RunDashboardComparison,
    pub transaction_provenance: Option<RunTransactionProvenance>,
    pub engine_summaries: RunDashboardEngineSummaries,
    pub asset_integrity: RunDashboardAssetIntegrity,
    pub asset_loading: RunDashboardAssetLoading,
    pub asset_preview: RunDashboardAssetPreview,
    pub asset_inspector: RunDashboardAssetInspector,
    pub plugin_registry: RunDashboardPluginRegistry,
    pub source_apply_worktree_context: RunDashboardSourceApplyWorktreeContext,
    pub runtime_invariants: RunDashboardRuntimeInvariants,
    pub qa_worker_assignments: RunDashboardQaWorkerAssignments,
    pub qa_agent_work_queues: RunDashboardQaAgentWorkQueues,
    pub performance_regression_lanes: RunDashboardPerformanceRegressionLanes,
    pub fuzzing_plans: RunDashboardFuzzingPlans,
    pub qa_scenario_candidates: RunDashboardQaScenarioCandidates,
    pub route_attempts: RunDashboardRouteAttempts,
    pub visual_comparisons: RunDashboardVisualComparisons,
    pub qa_swarm_inspection: RunDashboardQaSwarmInspection,
    pub evidence_categories: Vec<RunDashboardCategorySummary>,
    pub probe_contract_status: RunDashboardProbeContractStatus,
    pub evidence_fidelity: RunDashboardEvidenceFidelity,
    pub mutations: Vec<MutationProposal>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardRuntimeInvariants {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub check_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub unsupported_count: usize,
    pub missing_count: usize,
    pub malformed_count: usize,
    pub stale_count: usize,
    pub evidence_refs: Vec<String>,
    pub summaries: Vec<RuntimeInvariantEvidenceSummary>,
    pub evidence: Vec<RuntimeInvariantEvidence>,
    pub artifacts: Vec<RunDashboardArtifact>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardQaScenarioCandidates {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub candidate_count: usize,
    pub blocked_count: usize,
    pub deferred_count: usize,
    pub high_priority_count: usize,
    pub malformed_count: usize,
    pub evidence_refs: Vec<String>,
    pub candidates: Vec<QaScenarioCandidateArtifact>,
    pub artifacts: Vec<RunDashboardArtifact>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardFuzzingPlans {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub plan_count: usize,
    pub blocked_count: usize,
    pub exhausted_count: usize,
    pub malformed_count: usize,
    pub evidence_refs: Vec<String>,
    pub plans: Vec<AdversarialInputFuzzingPlanArtifact>,
    pub artifacts: Vec<RunDashboardArtifact>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardQaWorkerAssignments {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub assignment_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub deferred_count: usize,
    pub blocked_count: usize,
    pub exhausted_count: usize,
    pub malformed_count: usize,
    pub evidence_refs: Vec<String>,
    pub plans: Vec<QaWorkerAssignmentArtifact>,
    pub artifacts: Vec<RunDashboardArtifact>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardRouteAttempts {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub attempt_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub blocked_count: usize,
    pub inconclusive_count: usize,
    pub unsupported_count: usize,
    pub malformed_count: usize,
    pub evidence_refs: Vec<String>,
    pub attempts: Vec<RouteAttemptEvidenceArtifact>,
    pub artifacts: Vec<RunDashboardArtifact>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardVisualComparisons {
    pub present: bool,
    pub empty_state: String,
    pub status: String,
    pub comparison_count: usize,
    pub unchanged_count: usize,
    pub changed_count: usize,
    pub missing_screenshot_count: usize,
    pub malformed_screenshot_count: usize,
    pub mismatched_dimensions_count: usize,
    pub unsupported_count: usize,
    pub blocked_count: usize,
    pub malformed_count: usize,
    pub evidence_refs: Vec<String>,
    pub summaries: Vec<RunDashboardVisualComparisonSummary>,
    pub comparisons: Vec<VisualComparisonEvidenceArtifact>,
    pub artifacts: Vec<RunDashboardArtifact>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RunDashboardVisualComparisonSummary {
    pub comparison_id: String,
    pub run_id: String,
    pub scenario_id: String,
    pub checkpoint_id: String,
    pub outcome: VisualComparisonOutcome,
    pub failure_classification: String,
    pub changed_pixels: Option<u64>,
    pub changed_percent_x1000: Option<u32>,
    pub changed_region_count: usize,
    pub before_screenshot_ref: Option<String>,
    pub after_screenshot_ref: Option<String>,
    pub evidence_refs: Vec<String>,
}

pub fn list_dashboard_runs(runs_root: impl AsRef<Path>) -> Result<Vec<RunDashboardSummary>> {
    let runs_root = runs_root.as_ref();
    if !runs_root.exists() {
        return Ok(Vec::new());
    }
    let mut runs = Vec::new();
    for entry in fs::read_dir(runs_root)
        .with_context(|| format!("failed to read runs root {}", runs_root.display()))?
    {
        let entry = entry.context("failed to read runs root entry")?;
        let path = entry.path();
        if !path.is_dir() || !path.join("run.json").is_file() {
            continue;
        }
        runs.push(read_dashboard_run_summary(&path)?);
    }
    runs.sort_by(|left, right| {
        right
            .created_at_unix_ms
            .cmp(&left.created_at_unix_ms)
            .then_with(|| left.id.cmp(&right.id))
    });
    Ok(runs)
}

pub fn build_regression_run_matrix(runs_root: impl AsRef<Path>) -> Result<RegressionRunMatrix> {
    let mut summaries = list_dashboard_runs(runs_root)?;
    summaries.sort_by(|left, right| {
        left.created_at_unix_ms
            .cmp(&right.created_at_unix_ms)
            .then_with(|| left.id.cmp(&right.id))
    });

    let mut skipped_runs = Vec::new();
    let mut scenarios: BTreeMap<(String, String, String), RegressionMatrixScenarioBuilder> =
        BTreeMap::new();
    let mut projects: BTreeMap<String, String> = BTreeMap::new();
    let mut packs: BTreeMap<(String, String), String> = BTreeMap::new();

    for summary in summaries {
        let Some(project) = summary.project.clone() else {
            skipped_runs.push(RegressionRunMatrixSkippedRun {
                run_id: summary.id.clone(),
                run_dir: summary.run_dir.to_string_lossy().to_string(),
                reason: "missing_or_malformed_project_context".to_string(),
            });
            continue;
        };
        let Some(pack) = project.scenario_pack.clone() else {
            skipped_runs.push(RegressionRunMatrixSkippedRun {
                run_id: summary.id.clone(),
                run_dir: summary.run_dir.to_string_lossy().to_string(),
                reason: "missing_scenario_pack_context".to_string(),
            });
            continue;
        };

        projects.insert(project.id.clone(), project.name.clone());
        packs.insert((project.id.clone(), pack.id.clone()), pack.path.clone());

        let run = read_dashboard_run(&summary.run_dir)?;
        let result_observations = dashboard_scenario_result_observations(&run);
        let expected_ids = pack
            .scenario_ids
            .iter()
            .cloned()
            .chain(result_observations.keys().cloned())
            .collect::<BTreeSet<_>>();

        for scenario_id in expected_ids {
            let key = (project.id.clone(), pack.id.clone(), scenario_id.clone());
            let builder = scenarios.entry(key).or_insert_with(|| {
                RegressionMatrixScenarioBuilder::new(
                    project.id.clone(),
                    pack.id.clone(),
                    scenario_id.clone(),
                )
            });
            builder.context.mutation_ids.extend(
                run.mutations
                    .iter()
                    .map(|mutation| mutation.id.clone())
                    .collect::<Vec<_>>(),
            );
            builder.context.review_decision_ids.extend(
                read_mutation_review_artifact(&summary.run_dir)
                    .map(|artifact| {
                        artifact
                            .decisions
                            .into_iter()
                            .map(|decision| decision.id)
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
            );
            builder.context.promotion_ids.extend(
                run.regression_promotions
                    .iter()
                    .filter(|promotion| promotion.scenario_id == scenario_id)
                    .map(|promotion| promotion.id.clone())
                    .collect::<Vec<_>>(),
            );

            let observation = result_observations
                .get(&scenario_id)
                .cloned()
                .unwrap_or_else(|| RegressionRunMatrixObservation {
                    run_id: summary.id.clone(),
                    run_dir: summary.run_dir.to_string_lossy().to_string(),
                    created_at_unix_ms: summary.created_at_unix_ms,
                    status: "pending".to_string(),
                    scenario_result_path: None,
                    verdict_status: summary.verdict_status.clone(),
                    evidence_refs: Vec::new(),
                });
            builder.runs.push(observation);
        }
    }

    let mut project_builders: BTreeMap<String, BTreeMap<String, Vec<RegressionRunMatrixScenario>>> =
        BTreeMap::new();
    for ((project_id, pack_id, _scenario_id), builder) in scenarios {
        project_builders
            .entry(project_id)
            .or_default()
            .entry(pack_id)
            .or_default()
            .push(builder.finish());
    }

    let projects = project_builders
        .into_iter()
        .map(|(project_id, pack_builders)| {
            let scenario_packs = pack_builders
                .into_iter()
                .map(|(pack_id, mut scenarios)| {
                    scenarios.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
                    RegressionRunMatrixScenarioPack {
                        scenario_pack_path: packs
                            .get(&(project_id.clone(), pack_id.clone()))
                            .cloned()
                            .unwrap_or_default(),
                        scenario_pack_id: pack_id,
                        scenarios,
                    }
                })
                .collect();
            RegressionRunMatrixProject {
                project_name: projects.get(&project_id).cloned().unwrap_or_default(),
                project_id,
                scenario_packs,
            }
        })
        .collect();

    Ok(RegressionRunMatrix {
        schema_version: "ouroforge-regression-run-matrix-v1".to_string(),
        projects,
        skipped_runs,
    })
}

fn dashboard_scenario_result_observations(
    run: &RunDashboardReadModel,
) -> BTreeMap<String, RegressionRunMatrixObservation> {
    let mut observations = BTreeMap::new();
    for artifact in &run.scenario_results {
        let Some(value) = &artifact.value else {
            // An indexed scenario result that cannot be read or parsed must still
            // be surfaced (as unknown) rather than dropped; otherwise the matrix
            // falls back to a misleading "pending" with no evidence even though a
            // broken result artifact exists. Derive the scenario id from the
            // artifact path (evidence/scenarios/<id>/scenario-result*.json).
            if let Some(scenario_id) = scenario_id_from_scenario_result_path(&artifact.path) {
                observations.insert(
                    scenario_id,
                    RegressionRunMatrixObservation {
                        run_id: run.summary.id.clone(),
                        run_dir: run.summary.run_dir.to_string_lossy().to_string(),
                        created_at_unix_ms: run.summary.created_at_unix_ms,
                        status: "unknown".to_string(),
                        scenario_result_path: Some(artifact.path.clone()),
                        verdict_status: run.summary.verdict_status.clone(),
                        evidence_refs: vec![artifact.path.clone()],
                    },
                );
            }
            continue;
        };
        let Some(scenario_id) = value
            .get("scenario_id")
            .or_else(|| value.get("scenarioId"))
            .and_then(|value| value.as_str())
        else {
            continue;
        };
        let status = value
            .get("status")
            .and_then(|value| value.as_str())
            .filter(|status| matches!(*status, "passed" | "failed" | "pending"))
            .unwrap_or("failed")
            .to_string();
        let mut evidence_refs = vec![artifact.path.clone()];
        collect_dashboard_evidence_refs(value.get("evidence"), &mut evidence_refs);
        evidence_refs.sort();
        evidence_refs.dedup();
        observations.insert(
            scenario_id.to_string(),
            RegressionRunMatrixObservation {
                run_id: run.summary.id.clone(),
                run_dir: run.summary.run_dir.to_string_lossy().to_string(),
                created_at_unix_ms: run.summary.created_at_unix_ms,
                status,
                scenario_result_path: Some(artifact.path.clone()),
                verdict_status: run.summary.verdict_status.clone(),
                evidence_refs,
            },
        );
    }
    observations
}

pub fn read_dashboard_run(run_dir: impl AsRef<Path>) -> Result<RunDashboardReadModel> {
    let run_dir = run_dir.as_ref();
    let summary = read_dashboard_run_summary(run_dir)?;
    let run = read_json_value(run_dir.join("run.json"))?;
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    let evidence = read_evidence_index(run_dir)?.artifacts;
    let mutations = list_mutation_proposals(run_dir)?;
    let journal_view = read_dashboard_journal(run_dir, &evidence, &mutations);
    let journal = if journal_view.exists {
        fs::read_to_string(run_dir.join("journal.md")).unwrap_or_default()
    } else {
        String::new()
    };
    let screenshots =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_screenshot)?;
    let console_logs =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_console_log)?;
    let cdp_trace_summaries =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_cdp_trace_summary)?;
    let world_states =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_world_state)?;
    let frame_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_frame_metric)?;
    let performance_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_performance_metric)?;
    let scenario_results =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_scenario_result)?;
    let scenario_assertions = read_dashboard_scenario_assertions(&scenario_results);
    let behavior_assertion_results = select_dashboard_artifacts(
        run_dir,
        &evidence,
        dashboard_artifact_is_behavior_assertion_result,
    )?;
    let behavior_assertions = read_dashboard_behavior_assertions(&behavior_assertion_results);
    let behavior_evidence =
        behavior_evidence::read_dashboard_behavior_evidence(run_dir, &evidence)?;
    let mutation_artifacts = select_dashboard_mutation_artifacts(run_dir)?;
    let mutation_lifecycle = read_dashboard_mutation_lifecycle(run_dir, &mutations);
    let regression_promotions = read_regression_promotion_records(run_dir);
    let review_cockpit = dashboard_review_cockpit(&mutation_lifecycle, &regression_promotions);
    let replay = read_dashboard_replay(run_dir, &evidence)?;
    let comparison = read_dashboard_comparison(run_dir);
    let transaction_provenance = read_dashboard_transaction_provenance(&run);
    let project = read_dashboard_project_context(&run);
    let command_context = read_dashboard_command_context(&run);
    let engine_summaries = read_dashboard_engine_summaries(&world_states);
    let asset_integrity = read_dashboard_asset_integrity(run_dir, &evidence)?;
    let asset_loading = read_dashboard_asset_loading(run_dir, &evidence)?;
    let asset_preview = read_dashboard_asset_preview(run_dir, &evidence)?;
    let asset_inspector =
        dashboard_asset_inspector(&asset_integrity, &asset_loading, &asset_preview);
    let plugin_registry = read_dashboard_plugin_registry(run_dir, &evidence)?;
    let source_apply_worktree_context =
        read_dashboard_source_apply_worktree_context(run_dir, &evidence)?;
    let runtime_invariants = read_dashboard_runtime_invariants(run_dir, &evidence, &run)?;
    let qa_worker_assignments = read_dashboard_qa_worker_assignments(run_dir, &evidence)?;
    let qa_agent_work_queues = read_dashboard_qa_agent_work_queues(run_dir, &evidence)?;
    let performance_regression_lanes =
        read_dashboard_performance_regression_lanes(run_dir, &evidence)?;
    let fuzzing_plans = read_dashboard_fuzzing_plans(run_dir, &evidence)?;
    let qa_scenario_candidates = read_dashboard_qa_scenario_candidates(run_dir, &evidence)?;
    let route_attempts = read_dashboard_route_attempts(run_dir, &evidence)?;
    let evidence_categories = dashboard_category_summaries(DashboardCategoryArtifacts {
        screenshots: &screenshots,
        world_states: &world_states,
        frame_metrics: &frame_metrics,
        performance_metrics: &performance_metrics,
        console_logs: &console_logs,
        cdp_trace_summaries: &cdp_trace_summaries,
        scenario_results: &scenario_results,
        mutation_artifacts: &mutation_artifacts,
    });
    let visual_comparisons = read_dashboard_visual_comparisons(run_dir, &evidence)?;
    let qa_swarm_inspection = dashboard_qa_swarm_inspection(DashboardQaSwarmInspectionInputs {
        qa_scenario_candidates: &qa_scenario_candidates,
        fuzzing_plans: &fuzzing_plans,
        qa_worker_assignments: &qa_worker_assignments,
        qa_agent_work_queues: &qa_agent_work_queues,
        runtime_invariants: &runtime_invariants,
        route_attempts: &route_attempts,
        visual_comparisons: &visual_comparisons,
        performance_metrics: &performance_metrics,
        console_logs: &console_logs,
        scenario_results: &scenario_results,
        evidence_categories: &evidence_categories,
        mutations: &mutations,
    });
    let probe_contract_status = dashboard_probe_contract_status(
        &evidence,
        &world_states,
        &frame_metrics,
        &scenario_results,
    );
    let evidence_fidelity = dashboard_evidence_fidelity(DashboardEvidenceFidelityInputs {
        transaction_provenance: transaction_provenance.as_ref(),
        probe_contract_status: &probe_contract_status,
        replay: &replay,
        screenshots: &screenshots,
        console_logs: &console_logs,
        performance_metrics: &performance_metrics,
        cdp_trace_summaries: &cdp_trace_summaries,
        command_context: command_context.as_ref(),
    });
    Ok(RunDashboardReadModel {
        summary,
        run,
        project,
        command_context,
        verdict,
        journal,
        journal_view,
        evidence,
        screenshots,
        console_logs,
        cdp_trace_summaries,
        world_states,
        frame_metrics,
        performance_metrics,
        scenario_results,
        scenario_assertions,
        behavior_assertions,
        behavior_evidence,
        mutation_artifacts,
        mutation_lifecycle,
        review_cockpit,
        regression_promotions,
        replay,
        comparison,
        transaction_provenance,
        engine_summaries,
        asset_integrity,
        asset_loading,
        asset_preview,
        asset_inspector,
        plugin_registry,
        source_apply_worktree_context,
        runtime_invariants,
        qa_worker_assignments,
        qa_agent_work_queues,
        performance_regression_lanes,
        fuzzing_plans,
        qa_scenario_candidates,
        route_attempts,
        visual_comparisons,
        qa_swarm_inspection,
        evidence_categories,
        probe_contract_status,
        evidence_fidelity,
        mutations,
    })
}

fn read_dashboard_run_summary(run_dir: &Path) -> Result<RunDashboardSummary> {
    let run = read_json_value(run_dir.join("run.json"))?;
    let evidence = read_evidence_index(run_dir)?.artifacts;
    let evidence_count = evidence.len();
    let mutations = list_mutation_proposals(run_dir)?;
    let verdict = read_json_value(run_dir.join("verdict.json"))?;
    let mutation_artifacts = select_dashboard_mutation_artifacts(run_dir)?;
    let screenshots =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_screenshot)?;
    let console_logs =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_console_log)?;
    let cdp_trace_summaries =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_cdp_trace_summary)?;
    let world_states =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_world_state)?;
    let frame_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_frame_metric)?;
    let performance_metrics =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_performance_metric)?;
    let scenario_results =
        select_dashboard_artifacts(run_dir, &evidence, dashboard_artifact_is_scenario_result)?;
    let evidence_categories = dashboard_category_summaries(DashboardCategoryArtifacts {
        screenshots: &screenshots,
        world_states: &world_states,
        frame_metrics: &frame_metrics,
        performance_metrics: &performance_metrics,
        console_logs: &console_logs,
        cdp_trace_summaries: &cdp_trace_summaries,
        scenario_results: &scenario_results,
        mutation_artifacts: &mutation_artifacts,
    });
    let probe_contract_status = dashboard_probe_contract_status(
        &evidence,
        &world_states,
        &frame_metrics,
        &scenario_results,
    );
    let transaction_provenance = read_dashboard_transaction_provenance(&run);
    let command_context = read_dashboard_command_context(&run);
    let replay = read_dashboard_replay(run_dir, &evidence).unwrap_or_else(|_| RunDashboardReplay {
        present: false,
        empty_state: "Replay read model unavailable.".to_string(),
        sequences: Vec::new(),
    });
    let evidence_fidelity = dashboard_evidence_fidelity(DashboardEvidenceFidelityInputs {
        transaction_provenance: transaction_provenance.as_ref(),
        probe_contract_status: &probe_contract_status,
        replay: &replay,
        screenshots: &screenshots,
        console_logs: &console_logs,
        performance_metrics: &performance_metrics,
        cdp_trace_summaries: &cdp_trace_summaries,
        command_context: command_context.as_ref(),
    });
    let id = json_string(&run, "id").unwrap_or_else(|| {
        run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown-run")
            .to_string()
    });
    Ok(RunDashboardSummary {
        id,
        run_dir: run_dir.to_path_buf(),
        seed_id: json_string(&run, "seed_id").unwrap_or_else(|| "unknown-seed".to_string()),
        seed_title: json_string(&run, "seed_title").unwrap_or_else(|| "Untitled Seed".to_string()),
        project: read_dashboard_project_context(&run),
        command_context,
        run_status: json_string(&run, "status").unwrap_or_else(|| "unknown".to_string()),
        verdict_status: json_string(&verdict, "status").unwrap_or_else(|| "unknown".to_string()),
        scenario_status: dashboard_scenario_status(&verdict),
        created_at_unix_ms: run
            .get("created_at_unix_ms")
            .and_then(|value| value.as_u64())
            .map(u128::from)
            .unwrap_or(0),
        evidence_count,
        mutation_count: mutations.len(),
        worker_count: dashboard_worker_count(&evidence),
        evidence_categories,
        probe_contract_status,
        evidence_fidelity,
        journal_path: run_dir.join("journal.md"),
    })
}

fn select_dashboard_artifacts(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
    predicate: fn(&EvidenceArtifact) -> bool,
) -> Result<Vec<RunDashboardArtifact>> {
    evidence
        .iter()
        .filter(|artifact| predicate(artifact))
        .map(|artifact| dashboard_artifact(run_dir, artifact))
        .collect()
}

fn dashboard_artifact(run_dir: &Path, artifact: &EvidenceArtifact) -> Result<RunDashboardArtifact> {
    dashboard_artifact_from_parts(
        run_dir,
        artifact.id.clone(),
        artifact.kind.clone(),
        artifact.path.clone(),
        artifact.metadata.clone(),
    )
}

fn dashboard_artifact_is_screenshot(artifact: &EvidenceArtifact) -> bool {
    artifact.kind == "image/png" || artifact.path.ends_with(".png")
}

fn dashboard_artifact_is_console_log(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("console_log")
        || artifact.id.contains("console")
        || artifact.path.contains("console")
}

fn dashboard_artifact_is_cdp_trace_summary(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("cdp_trace_summary")
        || artifact.id.contains("cdp-trace")
        || artifact.path.contains("cdp-trace")
}

fn dashboard_probe_contract_status(
    evidence: &[EvidenceArtifact],
    world_states: &[RunDashboardArtifact],
    frame_metrics: &[RunDashboardArtifact],
    scenario_results: &[RunDashboardArtifact],
) -> RunDashboardProbeContractStatus {
    let mut evidence_refs = Vec::new();
    let mut observed_count = 0usize;
    for artifact in evidence {
        if artifact
            .metadata
            .get("probe_contract")
            .and_then(|value| value.get("version"))
            .and_then(|value| value.as_str())
            == Some(RUNTIME_PROBE_CONTRACT_VERSION)
        {
            observed_count += 1;
            evidence_refs.push(artifact.path.clone());
        }
    }
    let indexed_malformed_count = evidence
        .iter()
        .filter(|artifact| {
            artifact
                .metadata
                .get("artifact")
                .and_then(|value| value.as_str())
                == Some("runtime_probe_contract_failure")
                || artifact
                    .metadata
                    .get("probe_contract_failure")
                    .and_then(|value| value.as_bool())
                    == Some(true)
        })
        .count()
        + scenario_results
            .iter()
            .filter(|artifact| {
                artifact
                    .value
                    .as_ref()
                    .is_some_and(|value| value.get("probe_contract_failure").is_some())
            })
            .count();
    let malformed_artifact_count = world_states
        .iter()
        .chain(frame_metrics.iter())
        .chain(scenario_results.iter())
        .filter(|artifact| artifact.exists && artifact.read_error.is_some())
        .count();
    let malformed_count = indexed_malformed_count + malformed_artifact_count;
    let has_world_v2 = world_states
        .iter()
        .any(dashboard_artifact_has_readable_probe_contract_v2);
    let has_frame_v2 = frame_metrics
        .iter()
        .any(dashboard_artifact_has_readable_probe_contract_v2);
    let missing_artifact_count = world_states
        .iter()
        .chain(frame_metrics.iter())
        .chain(scenario_results.iter())
        .filter(|artifact| !artifact.exists)
        .count();
    let missing_count =
        usize::from(!has_world_v2) + usize::from(!has_frame_v2) + missing_artifact_count;
    let status = if malformed_count > 0 {
        "malformed"
    } else if has_world_v2 && has_frame_v2 && missing_artifact_count == 0 {
        "present"
    } else if observed_count > 0 {
        "partial"
    } else {
        "legacy"
    };
    RunDashboardProbeContractStatus {
        status: status.to_string(),
        contract_name: RUNTIME_PROBE_CONTRACT_NAME.to_string(),
        version: RUNTIME_PROBE_CONTRACT_VERSION.to_string(),
        observed_count,
        missing_count,
        malformed_count,
        evidence_refs,
    }
}

fn dashboard_artifact_is_world_state(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("world_state")
        || artifact
            .metadata
            .get("probe_call")
            .and_then(|value| value.as_str())
            == Some("getWorldState")
        || artifact.id.contains("world-state")
        || artifact.path.contains("world-state")
}

fn dashboard_artifact_is_asset_reference_integrity(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("asset_reference_integrity")
        || artifact.id.contains("asset-reference-integrity")
        || artifact.path.contains("asset-reference-integrity")
}

fn dashboard_artifact_is_runtime_asset_load_evidence(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("runtime_asset_load_evidence")
        || artifact.id.contains("asset-load-evidence")
        || artifact.path.contains("asset-load-evidence")
}

fn dashboard_artifact_is_asset_preview_evidence(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("asset_preview_evidence")
        || artifact.id.contains("asset-preview-evidence")
        || artifact.path.contains("asset-preview-evidence")
}

fn dashboard_artifact_is_qa_scenario_candidate(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("qa_scenario_candidate")
        || artifact.id.contains("qa-scenario-candidate")
        || artifact.id.contains("scenario-candidate")
        || artifact.path.contains("qa-scenario-candidate")
        || artifact.path.contains("scenario-candidate")
}

fn read_dashboard_qa_scenario_candidates(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardQaScenarioCandidates> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_qa_scenario_candidate,
    )?;
    let boundary = "Read-only QA scenario candidates; dashboard/Studio surfaces must not run candidates, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardQaScenarioCandidates {
            present: false,
            empty_state: "No QA scenario candidates are indexed for this run.".to_string(),
            status: "missing".to_string(),
            candidate_count: 0,
            blocked_count: 0,
            deferred_count: 0,
            high_priority_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            candidates: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut candidates = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<QaScenarioCandidateArtifact>(value.clone()) {
            Ok(candidate) if candidate.validate().is_ok() => candidates.push(candidate),
            _ => malformed_count += 1,
        }
    }
    candidates.sort_by(|left, right| {
        (left.run_id.as_str(), left.candidate_id.as_str())
            .cmp(&(right.run_id.as_str(), right.candidate_id.as_str()))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let candidate_count = candidates.len();
    let blocked_count = candidates
        .iter()
        .filter(|candidate| candidate.status == QaScenarioCandidateStatus::Blocked)
        .count();
    let deferred_count = candidates
        .iter()
        .filter(|candidate| candidate.status == QaScenarioCandidateStatus::Deferred)
        .count();
    let high_priority_count = candidates
        .iter()
        .filter(|candidate| candidate.priority == QaScenarioCandidatePriority::High)
        .count();
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if deferred_count > 0 {
        "deferred"
    } else if candidate_count > 0 {
        "proposed"
    } else {
        "missing"
    };
    Ok(RunDashboardQaScenarioCandidates {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        candidate_count,
        blocked_count,
        deferred_count,
        high_priority_count,
        malformed_count,
        evidence_refs,
        candidates,
        artifacts,
        boundary,
    })
}

fn dashboard_artifact_is_fuzzing_plan(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("adversarial_input_fuzzing_plan")
        || artifact.id.contains("fuzzing-plan")
        || artifact.id.contains("fuzz-plan")
        || artifact.path.contains("adversarial-input-fuzzing")
        || artifact.path.contains("fuzzing-plan")
}

fn read_dashboard_fuzzing_plans(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardFuzzingPlans> {
    let artifacts =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_fuzzing_plan)?;
    let boundary = "Read-only adversarial input fuzzing plans; dashboard/Studio surfaces must not run fuzzers, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardFuzzingPlans {
            present: false,
            empty_state: "No adversarial input fuzzing plans are indexed for this run.".to_string(),
            status: "missing".to_string(),
            plan_count: 0,
            blocked_count: 0,
            exhausted_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            plans: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut plans = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<AdversarialInputFuzzingPlanArtifact>(value.clone()) {
            Ok(plan) if plan.validate().is_ok() => plans.push(plan),
            _ => malformed_count += 1,
        }
    }
    plans.sort_by(|left, right| {
        (left.run_id.as_str(), left.plan_id.as_str())
            .cmp(&(right.run_id.as_str(), right.plan_id.as_str()))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let plan_count = plans.len();
    let blocked_count = plans
        .iter()
        .filter(|plan| plan.status == FuzzPlanStatus::Blocked)
        .count();
    let exhausted_count = plans
        .iter()
        .filter(|plan| plan.status == FuzzPlanStatus::Exhausted)
        .count();
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if exhausted_count > 0 {
        "exhausted"
    } else if plan_count > 0 {
        "planned"
    } else {
        "missing"
    };
    Ok(RunDashboardFuzzingPlans {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        plan_count,
        blocked_count,
        exhausted_count,
        malformed_count,
        evidence_refs,
        plans,
        artifacts,
        boundary,
    })
}

fn dashboard_artifact_is_plugin_registry_evidence(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("plugin_registry_evidence")
        || artifact.id.contains("plugin-registry")
        || artifact.path.contains("evidence/plugins/")
}

fn read_dashboard_plugin_registry(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardPluginRegistry> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_plugin_registry_evidence,
    )?;
    let boundary = "Read-only plugin registry browser data; dashboard/Studio surfaces must not install, update, delete, enable executable plugins, run commands, load arbitrary JavaScript or native extensions, access credentials, mutate source or trusted files, publish, deploy, sign, upload, or write generated registry state.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardPluginRegistry {
            present: false,
            empty_state: "No plugin registry evidence is indexed for this run.".to_string(),
            status: "missing".to_string(),
            registry_count: 0,
            plugin_count: 0,
            blocked_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            registries: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut registries = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        let Ok(registry) = serde_json::from_value::<plugin_evidence::PluginRegistryEvidenceArtifact>(
            value.clone(),
        ) else {
            malformed_count += 1;
            continue;
        };
        if registry.validate().is_err() {
            malformed_count += 1;
            continue;
        }
        let read_model = registry.read_model();
        evidence_refs.push(registry.ledger_ref.clone());
        let mut plugin_rows = registry
            .plugins
            .iter()
            .map(|plugin| {
                evidence_refs.extend(
                    plugin
                        .evidence_refs
                        .iter()
                        .map(|evidence| evidence.path.clone()),
                );
                RunDashboardPluginDescriptorRow {
                    plugin_id: plugin.plugin_id.clone(),
                    manifest_path: plugin.manifest_path.clone(),
                    manifest_hash: plugin.manifest_hash.clone(),
                    manifest_version: plugin.manifest_version.clone(),
                    validation_status: format!("{:?}", plugin.validation_status)
                        .to_ascii_lowercase(),
                    compatibility_status: format!("{:?}", plugin.compatibility_status)
                        .to_ascii_lowercase(),
                    declared_capabilities: plugin.declared_capabilities.clone(),
                    extension_points: plugin.extension_points.clone(),
                    evidence_refs: plugin
                        .evidence_refs
                        .iter()
                        .map(|evidence| evidence.path.clone())
                        .collect(),
                    dashboard_panels: plugin
                        .dashboard_panels
                        .iter()
                        .map(|panel| RunDashboardPluginPanelDescriptorRow {
                            panel_id: panel.panel_id.clone(),
                            title: panel.title.clone(),
                            data_source_key: panel.data_source_key.clone(),
                            template_ref: panel.template_ref.clone(),
                            layout_hint: panel.layout_hint.clone(),
                            display_hints: panel.display_hints.clone(),
                            boundary: panel.boundary.clone(),
                        })
                        .collect(),
                    scenario_templates: plugin
                        .scenario_templates
                        .iter()
                        .map(|template| RunDashboardPluginScenarioTemplateDescriptorRow {
                            template_id: template.template_id.clone(),
                            description: template.description.clone(),
                            parameters: template
                                .parameters
                                .iter()
                                .map(|parameter| RunDashboardPluginScenarioTemplateParameterRow {
                                    name: parameter.name.clone(),
                                    parameter_type: parameter.parameter_type.clone(),
                                    description: parameter.description.clone(),
                                    required: parameter.required,
                                    allowed_values: parameter.allowed_values.clone(),
                                })
                                .collect(),
                            supported_game_types: template.supported_game_types.clone(),
                            tags: template.tags.clone(),
                            expected_evidence_type: template.expected_evidence_type.clone(),
                            validation_hints: template.validation_hints.clone(),
                            boundary: template.boundary.clone(),
                        })
                        .collect(),
                    blocked_reasons: plugin.blocked_reasons.clone(),
                }
            })
            .collect::<Vec<_>>();
        plugin_rows.sort_by(|left, right| left.plugin_id.cmp(&right.plugin_id));
        registries.push(RunDashboardPluginRegistryRecord {
            registry_id: registry.registry_id,
            project_id: registry.project_id,
            run_id: registry.run_id,
            ledger_ref: registry.ledger_ref,
            generated_state: serde_json::to_value(registry.generated_state)
                .unwrap_or_else(|_| json!({})),
            status: read_model.status,
            plugin_count: read_model.plugin_count,
            blocked_count: read_model.blocked_count,
            blocked_reasons: read_model.blocked_reasons,
            plugins: plugin_rows,
        });
    }
    registries.sort_by(|left, right| left.registry_id.cmp(&right.registry_id));
    evidence_refs.sort();
    evidence_refs.dedup();
    let registry_count = registries.len();
    let plugin_count = registries
        .iter()
        .map(|registry| registry.plugin_count)
        .sum();
    let blocked_count = registries
        .iter()
        .map(|registry| registry.blocked_count)
        .sum();
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if registry_count > 0 {
        "ready"
    } else {
        "missing"
    };
    Ok(RunDashboardPluginRegistry {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        registry_count,
        plugin_count,
        blocked_count,
        malformed_count,
        evidence_refs,
        registries,
        artifacts,
        boundary,
    })
}

fn dashboard_artifact_is_qa_worker_assignment(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("qa_worker_assignment")
        || artifact.id.contains("qa-worker-assignment")
        || artifact.id.contains("worker-assignment")
        || artifact.path.contains("qa-worker-assignment")
        || artifact.path.contains("worker-assignment")
}

fn dashboard_artifact_is_qa_agent_work_queue(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("qa_agent_work_queue")
        || artifact.id.contains("qa-agent-work-queue")
        || artifact.id.contains("qa-work-queue")
        || artifact.path.contains("qa-agent-work-queue")
        || artifact.path.contains("qa-work-queue")
}

fn dashboard_artifact_is_performance_regression_lane(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("performance_regression_lane")
        || artifact.id.contains("performance-regression-lane")
        || artifact.id.contains("regression-lane")
        || artifact.path.contains("performance-regression-lane")
        || artifact.path.contains("regression-lane")
}

fn dashboard_artifact_is_route_attempt(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("route_attempt_evidence")
        || artifact.id.contains("route-attempt")
        || artifact.id.contains("route_attempt")
        || artifact.path.contains("route-attempt")
        || artifact.path.contains("route_attempt")
}

fn dashboard_artifact_is_visual_comparison(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("visual_comparison_evidence")
        || artifact.id.contains("visual-comparison")
        || artifact.id.contains("visual_comparison")
        || artifact.path.contains("visual-comparison")
        || artifact.path.contains("visual_comparison")
}

fn read_dashboard_route_attempts(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardRouteAttempts> {
    let artifacts =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_route_attempt)?;
    let boundary = "Read-only route attempt evidence; dashboard/Studio surfaces must not run solvers, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim objective solvability or quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardRouteAttempts {
            present: false,
            empty_state: "No route attempt evidence is indexed for this run.".to_string(),
            status: "missing".to_string(),
            attempt_count: 0,
            passed_count: 0,
            failed_count: 0,
            blocked_count: 0,
            inconclusive_count: 0,
            unsupported_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            attempts: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut attempts = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<RouteAttemptEvidenceArtifact>(value.clone()) {
            Ok(attempt) if validate_route_attempt_evidence_refs(run_dir, &attempt).is_ok() => {
                evidence_refs.push(attempt.start_state.world_state_ref.clone());
                evidence_refs.extend(attempt.evidence_refs.clone());
                evidence_refs.extend(
                    attempt
                        .route
                        .iter()
                        .filter_map(|node| node.evidence_ref.clone()),
                );
                evidence_refs.extend(
                    attempt
                        .blockers
                        .iter()
                        .filter_map(|blocker| blocker.evidence_ref.clone()),
                );
                attempts.push(attempt);
            }
            _ => malformed_count += 1,
        }
    }
    attempts.sort_by(|left, right| {
        (left.run_id.as_str(), left.attempt_id.as_str())
            .cmp(&(right.run_id.as_str(), right.attempt_id.as_str()))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let attempt_count = attempts.len();
    let passed_count = attempts
        .iter()
        .filter(|attempt| attempt.outcome == RouteAttemptOutcome::Passed)
        .count();
    let failed_count = attempts
        .iter()
        .filter(|attempt| attempt.outcome == RouteAttemptOutcome::Failed)
        .count();
    let blocked_count = attempts
        .iter()
        .filter(|attempt| attempt.outcome == RouteAttemptOutcome::Blocked)
        .count();
    let inconclusive_count = attempts
        .iter()
        .filter(|attempt| attempt.outcome == RouteAttemptOutcome::Inconclusive)
        .count();
    let unsupported_count = attempts
        .iter()
        .filter(|attempt| attempt.outcome == RouteAttemptOutcome::Unsupported)
        .count();
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if unsupported_count > 0 {
        "unsupported"
    } else if failed_count > 0 {
        "failed"
    } else if inconclusive_count > 0 {
        "inconclusive"
    } else if attempt_count > 0 && passed_count == attempt_count {
        "passed"
    } else if attempt_count > 0 {
        "attempted"
    } else {
        "missing"
    };
    Ok(RunDashboardRouteAttempts {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        attempt_count,
        passed_count,
        failed_count,
        blocked_count,
        inconclusive_count,
        unsupported_count,
        malformed_count,
        evidence_refs,
        attempts,
        artifacts,
        boundary,
    })
}

fn read_dashboard_visual_comparisons(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardVisualComparisons> {
    let artifacts =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_visual_comparison)?;
    let boundary = "Read-only visual comparison evidence; dashboard/Studio surfaces must not compute trusted diffs, run browser-side visual judgment, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim aesthetic quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardVisualComparisons {
            present: false,
            empty_state: "No visual comparison evidence is indexed for this run.".to_string(),
            status: "missing".to_string(),
            comparison_count: 0,
            unchanged_count: 0,
            changed_count: 0,
            missing_screenshot_count: 0,
            malformed_screenshot_count: 0,
            mismatched_dimensions_count: 0,
            unsupported_count: 0,
            blocked_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            summaries: Vec::new(),
            comparisons: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut comparisons = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<VisualComparisonEvidenceArtifact>(value.clone()) {
            Ok(comparison)
                if validate_visual_comparison_evidence_refs(run_dir, &comparison).is_ok() =>
            {
                evidence_refs.extend(visual_comparison_evidence_refs(&comparison));
                comparisons.push(comparison);
            }
            _ => malformed_count += 1,
        }
    }
    comparisons.sort_by(|left, right| {
        (
            left.run_id.as_str(),
            left.scenario_id.as_str(),
            left.checkpoint_id.as_str(),
            left.comparison_id.as_str(),
        )
            .cmp(&(
                right.run_id.as_str(),
                right.scenario_id.as_str(),
                right.checkpoint_id.as_str(),
                right.comparison_id.as_str(),
            ))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let summaries = comparisons
        .iter()
        .map(dashboard_visual_comparison_summary)
        .collect::<Vec<_>>();
    let comparison_count = comparisons.len();
    let unchanged_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::Unchanged)
        .count();
    let changed_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::Changed)
        .count();
    let missing_screenshot_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::MissingScreenshot)
        .count();
    let malformed_screenshot_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::MalformedScreenshot)
        .count();
    let mismatched_dimensions_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::MismatchedDimensions)
        .count();
    let unsupported_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::Unsupported)
        .count();
    let blocked_count = comparisons
        .iter()
        .filter(|comparison| comparison.outcome == VisualComparisonOutcome::Blocked)
        .count();
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if unsupported_count > 0 {
        "unsupported"
    } else if malformed_screenshot_count > 0 {
        "malformed_screenshot"
    } else if missing_screenshot_count > 0 {
        "missing_screenshot"
    } else if mismatched_dimensions_count > 0 {
        "mismatched_dimensions"
    } else if changed_count > 0 {
        "changed"
    } else if comparison_count > 0 && unchanged_count == comparison_count {
        "unchanged"
    } else if comparison_count > 0 {
        "mixed"
    } else {
        "missing"
    };
    Ok(RunDashboardVisualComparisons {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        comparison_count,
        unchanged_count,
        changed_count,
        missing_screenshot_count,
        malformed_screenshot_count,
        mismatched_dimensions_count,
        unsupported_count,
        blocked_count,
        malformed_count,
        evidence_refs,
        summaries,
        comparisons,
        artifacts,
        boundary,
    })
}

struct DashboardQaSwarmInspectionInputs<'a> {
    qa_scenario_candidates: &'a RunDashboardQaScenarioCandidates,
    fuzzing_plans: &'a RunDashboardFuzzingPlans,
    qa_worker_assignments: &'a RunDashboardQaWorkerAssignments,
    qa_agent_work_queues: &'a RunDashboardQaAgentWorkQueues,
    runtime_invariants: &'a RunDashboardRuntimeInvariants,
    route_attempts: &'a RunDashboardRouteAttempts,
    visual_comparisons: &'a RunDashboardVisualComparisons,
    performance_metrics: &'a [RunDashboardArtifact],
    console_logs: &'a [RunDashboardArtifact],
    scenario_results: &'a [RunDashboardArtifact],
    evidence_categories: &'a [RunDashboardCategorySummary],
    mutations: &'a [MutationProposal],
}

fn dashboard_qa_swarm_inspection(
    inputs: DashboardQaSwarmInspectionInputs<'_>,
) -> RunDashboardQaSwarmInspection {
    let mut panels = vec![
        dashboard_qa_swarm_panel(
            "scenario-candidates",
            "Scenario candidate panel",
            &inputs.qa_scenario_candidates.status,
            inputs.qa_scenario_candidates.candidate_count,
            inputs.qa_scenario_candidates.malformed_count,
            inputs.qa_scenario_candidates.evidence_refs.clone(),
            &inputs.qa_scenario_candidates.boundary,
        ),
        dashboard_qa_swarm_panel(
            "fuzzing-plans",
            "Fuzzing plan panel",
            &inputs.fuzzing_plans.status,
            inputs.fuzzing_plans.plan_count,
            inputs.fuzzing_plans.malformed_count,
            inputs.fuzzing_plans.evidence_refs.clone(),
            &inputs.fuzzing_plans.boundary,
        ),
        dashboard_qa_swarm_panel(
            "worker-assignments",
            "Worker budget/assignment panel",
            &inputs.qa_worker_assignments.status,
            inputs.qa_worker_assignments.assignment_count,
            inputs.qa_worker_assignments.malformed_count,
            inputs.qa_worker_assignments.evidence_refs.clone(),
            &inputs.qa_worker_assignments.boundary,
        ),
        dashboard_qa_swarm_panel(
            "qa-run-matrix",
            "QA run matrix panel",
            dashboard_artifact_panel_status(inputs.scenario_results),
            inputs.scenario_results.len(),
            dashboard_artifact_malformed_count(inputs.scenario_results),
            dashboard_artifact_refs(inputs.scenario_results),
            "Read-only QA run matrix summary; dashboard/Studio surfaces must not rerun scenarios, execute commands, spawn workers, or write trusted state.",
        ),
        dashboard_qa_swarm_panel(
            "invariant-checker",
            "Invariant checker panel",
            &inputs.runtime_invariants.status,
            inputs.runtime_invariants.check_count,
            inputs.runtime_invariants.malformed_count,
            inputs.runtime_invariants.evidence_refs.clone(),
            &inputs.runtime_invariants.boundary,
        ),
        dashboard_qa_swarm_panel(
            "objective-route-attempts",
            "Objective route attempt panel",
            &inputs.route_attempts.status,
            inputs.route_attempts.attempt_count,
            inputs.route_attempts.malformed_count,
            inputs.route_attempts.evidence_refs.clone(),
            &inputs.route_attempts.boundary,
        ),
        dashboard_qa_swarm_panel(
            "visual-performance-error-evidence",
            "Visual/performance/error evidence panel",
            dashboard_visual_performance_error_status(
                inputs.visual_comparisons,
                inputs.performance_metrics,
                inputs.console_logs,
            ),
            inputs.visual_comparisons.comparison_count
                + inputs.performance_metrics.len()
                + inputs.console_logs.len(),
            inputs.visual_comparisons.malformed_count
                + dashboard_artifact_malformed_count(inputs.performance_metrics)
                + dashboard_artifact_malformed_count(inputs.console_logs),
            dashboard_visual_performance_error_refs(
                inputs.visual_comparisons,
                inputs.performance_metrics,
                inputs.console_logs,
            ),
            "Read-only visual, performance, and error evidence summary; Studio may inspect escaped evidence only and must not compute trusted diffs, run probes, execute commands, or claim quality guarantees.",
        ),
        dashboard_qa_swarm_panel(
            "flaky-rerun-failure-backlog",
            "Flaky rerun policy/results and failure backlog panel",
            dashboard_flaky_backlog_status(inputs.qa_agent_work_queues, inputs.mutations),
            inputs.qa_agent_work_queues.flaky_count
                + inputs.qa_agent_work_queues.needs_rerun_count
                + inputs.qa_agent_work_queues.failed_count
                + inputs.qa_agent_work_queues.blocked_count
                + inputs.mutations.len(),
            inputs.qa_agent_work_queues.malformed_count,
            inputs.qa_agent_work_queues.evidence_refs.clone(),
            "Read-only flaky/rerun/failure backlog summary; QA outputs remain evidence and backlog inputs only until reviewed and must not auto-fix, auto-apply, auto-merge, or self-approve.",
        ),
        dashboard_qa_swarm_panel(
            "qa-evidence-bundle",
            "QA evidence bundle panel",
            if inputs.evidence_categories.is_empty() { "missing" } else { "present" },
            inputs.evidence_categories.iter().map(|category| category.count).sum(),
            0,
            Vec::new(),
            "Read-only QA evidence bundle category summary; generated runs, fuzz inputs, screenshots, videos, traces, and local tool state remain generated/ignored unless fixture-scoped.",
        ),
    ];
    panels.sort_by(|left, right| left.panel_id.cmp(&right.panel_id));

    let panel_count = panels.len();
    let missing_panel_count = panels
        .iter()
        .filter(|panel| panel.status == "missing")
        .count();
    let malformed_panel_count: usize = panels.iter().map(|panel| panel.malformed_count).sum();
    let item_count: usize = panels.iter().map(|panel| panel.item_count).sum();
    let mut evidence_refs: Vec<String> = panels
        .iter()
        .flat_map(|panel| panel.evidence_refs.iter().cloned())
        .collect();
    evidence_refs.sort();
    evidence_refs.dedup();
    let present = item_count > 0 || malformed_panel_count > 0;
    let status = if malformed_panel_count > 0 {
        "malformed"
    } else if panels.iter().any(|panel| {
        matches!(
            panel.status.as_str(),
            "blocked" | "failed" | "flaky" | "needs-rerun" | "regressed" | "changed"
        )
    }) {
        "needs-review"
    } else if present {
        "present"
    } else {
        "missing"
    };
    RunDashboardQaSwarmInspection {
        present,
        empty_state: if present {
            String::new()
        } else {
            "No QA/playtest swarm inspection evidence is indexed for this run.".to_string()
        },
        status: status.to_string(),
        panel_count,
        missing_panel_count,
        malformed_panel_count,
        item_count,
        evidence_refs,
        panels,
        boundary: "Read-only QA/playtest swarm inspection summary; Studio and dashboard surfaces must not spawn workers, execute commands, bridge to local/cloud runners, write trusted state, auto-fix, auto-apply, auto-merge, self-approve, or claim production quality guarantees.".to_string(),
    }
}

fn dashboard_visual_performance_error_status(
    visual_comparisons: &RunDashboardVisualComparisons,
    performance_metrics: &[RunDashboardArtifact],
    console_logs: &[RunDashboardArtifact],
) -> &'static str {
    if visual_comparisons.malformed_count > 0
        || dashboard_artifact_malformed_count(performance_metrics) > 0
        || dashboard_artifact_malformed_count(console_logs) > 0
    {
        "malformed"
    } else if visual_comparisons.changed_count > 0
        || visual_comparisons.missing_screenshot_count > 0
        || visual_comparisons.mismatched_dimensions_count > 0
        || visual_comparisons.blocked_count > 0
        || !console_logs.is_empty()
    {
        "needs-review"
    } else if visual_comparisons.comparison_count > 0 || !performance_metrics.is_empty() {
        "present"
    } else {
        "missing"
    }
}

fn dashboard_visual_performance_error_refs(
    visual_comparisons: &RunDashboardVisualComparisons,
    performance_metrics: &[RunDashboardArtifact],
    console_logs: &[RunDashboardArtifact],
) -> Vec<String> {
    let mut refs = visual_comparisons.evidence_refs.clone();
    refs.extend(dashboard_artifact_refs(performance_metrics));
    refs.extend(dashboard_artifact_refs(console_logs));
    refs.sort();
    refs.dedup();
    refs
}

fn visual_comparison_evidence_refs(comparison: &VisualComparisonEvidenceArtifact) -> Vec<String> {
    let mut refs = Vec::new();
    refs.extend(comparison.evidence_refs.clone());
    refs.extend(comparison.metadata_refs.clone());
    refs.extend(comparison.before.screenshot_ref.clone());
    refs.extend(comparison.after.screenshot_ref.clone());
    refs.extend(comparison.before.metadata_ref.clone());
    refs.extend(comparison.after.metadata_ref.clone());
    refs.sort();
    refs.dedup();
    refs
}

fn dashboard_visual_comparison_summary(
    comparison: &VisualComparisonEvidenceArtifact,
) -> RunDashboardVisualComparisonSummary {
    let summary = comparison.pixel_diff_summary.as_ref();
    RunDashboardVisualComparisonSummary {
        comparison_id: comparison.comparison_id.clone(),
        run_id: comparison.run_id.clone(),
        scenario_id: comparison.scenario_id.clone(),
        checkpoint_id: comparison.checkpoint_id.clone(),
        outcome: comparison.outcome,
        failure_classification: visual_comparison_failure_classification(comparison.outcome)
            .to_string(),
        changed_pixels: summary.map(|summary| summary.changed_pixels),
        changed_percent_x1000: summary.map(|summary| summary.changed_percent_x1000),
        changed_region_count: comparison.changed_regions.len(),
        before_screenshot_ref: comparison.before.screenshot_ref.clone(),
        after_screenshot_ref: comparison.after.screenshot_ref.clone(),
        evidence_refs: visual_comparison_evidence_refs(comparison),
    }
}

fn visual_comparison_failure_classification(outcome: VisualComparisonOutcome) -> &'static str {
    match outcome {
        VisualComparisonOutcome::Unchanged => "visual_unchanged",
        VisualComparisonOutcome::Changed => "visual_regression_candidate",
        VisualComparisonOutcome::MissingScreenshot => "visual_missing_screenshot",
        VisualComparisonOutcome::MalformedScreenshot => "visual_malformed_screenshot",
        VisualComparisonOutcome::MismatchedDimensions => "visual_mismatched_dimensions",
        VisualComparisonOutcome::Unsupported => "visual_unsupported",
        VisualComparisonOutcome::Blocked => "visual_blocked",
    }
}

fn read_dashboard_qa_agent_work_queues(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardQaAgentWorkQueues> {
    let artifacts =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_qa_agent_work_queue)?;
    let boundary = "Read-only QA agent work queues; dashboard/Studio surfaces may display linked scenario, evaluator, run, task, work-package, review-gate, stale-ref, and expected-evidence refs but must not execute queue commands, spawn agents, write trusted state, auto-fix, auto-apply, auto-merge, self-approve, or claim quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardQaAgentWorkQueues {
            present: false,
            empty_state: "No QA agent work queues are indexed for this run.".to_string(),
            status: "missing".to_string(),
            queue_count: 0,
            item_count: 0,
            passed_count: 0,
            failed_count: 0,
            deferred_count: 0,
            blocked_count: 0,
            flaky_count: 0,
            needs_rerun_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            queues: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut queues = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<QaAgentWorkQueueArtifact>(value.clone()) {
            Ok(queue) if queue.validate().is_ok() => {
                evidence_refs.extend(qa_agent_work_queue_evidence_refs(&queue));
                queues.push(queue);
            }
            _ => malformed_count += 1,
        }
    }
    queues.sort_by(|left, right| left.queue_id.cmp(&right.queue_id));
    evidence_refs.sort();
    evidence_refs.dedup();

    let queue_count = queues.len();
    let mut item_count = 0usize;
    let mut passed_count = 0usize;
    let mut failed_count = 0usize;
    let mut deferred_count = 0usize;
    let mut blocked_count = 0usize;
    let mut flaky_count = 0usize;
    let mut needs_rerun_count = 0usize;
    for queue in &queues {
        item_count += queue.items.len();
        for item in &queue.items {
            match item.status {
                QaAgentQueueStatus::Pass => passed_count += 1,
                QaAgentQueueStatus::Fail => failed_count += 1,
                QaAgentQueueStatus::Deferred => deferred_count += 1,
                QaAgentQueueStatus::Blocked => blocked_count += 1,
                QaAgentQueueStatus::Flaky => flaky_count += 1,
                QaAgentQueueStatus::NeedsRerun => needs_rerun_count += 1,
            }
        }
    }
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if needs_rerun_count > 0 {
        "needs-rerun"
    } else if flaky_count > 0 {
        "flaky"
    } else if failed_count > 0 {
        "failed"
    } else if deferred_count > 0 {
        "deferred"
    } else if item_count > 0 && passed_count == item_count {
        "passed"
    } else if item_count > 0 {
        "queued"
    } else {
        "missing"
    };
    Ok(RunDashboardQaAgentWorkQueues {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        queue_count,
        item_count,
        passed_count,
        failed_count,
        deferred_count,
        blocked_count,
        flaky_count,
        needs_rerun_count,
        malformed_count,
        evidence_refs,
        queues,
        artifacts,
        boundary,
    })
}

fn read_dashboard_performance_regression_lanes(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardPerformanceRegressionLanes> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_performance_regression_lane,
    )?;
    let boundary = "Read-only performance/regression lanes; dashboard/Studio surfaces may display linked run comparison, frame budget, scenario matrix, QA queue, review-gate, stale-ref, and browser-warning refs but must not execute commands, spawn agents, write trusted state, promote regressions, auto-apply, auto-merge, self-approve, or claim production guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardPerformanceRegressionLanes {
            present: false,
            empty_state: "No performance/regression lanes are indexed for this run.".to_string(),
            status: "missing".to_string(),
            lane_count: 0,
            improved_count: 0,
            unchanged_count: 0,
            regressed_count: 0,
            inconclusive_count: 0,
            missing_baseline_count: 0,
            unsupported_count: 0,
            stale_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            lanes: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut lanes = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<PerformanceRegressionLaneArtifact>(value.clone()) {
            Ok(lane) if lane.validate().is_ok() => {
                evidence_refs.extend(performance_regression_lane_evidence_refs(&lane));
                lanes.push(lane);
            }
            _ => malformed_count += 1,
        }
    }
    lanes.sort_by(|left, right| left.lane_id.cmp(&right.lane_id));
    evidence_refs.sort();
    evidence_refs.dedup();

    let lane_count = lanes.len();
    let mut improved_count = 0usize;
    let mut unchanged_count = 0usize;
    let mut regressed_count = 0usize;
    let mut inconclusive_count = 0usize;
    let mut missing_baseline_count = 0usize;
    let mut unsupported_count = 0usize;
    let mut stale_count = 0usize;
    for lane in &lanes {
        match lane.classification {
            PerformanceRegressionClassification::Improved => improved_count += 1,
            PerformanceRegressionClassification::Unchanged => unchanged_count += 1,
            PerformanceRegressionClassification::Regressed => regressed_count += 1,
            PerformanceRegressionClassification::Inconclusive => inconclusive_count += 1,
            PerformanceRegressionClassification::MissingBaseline => missing_baseline_count += 1,
            PerformanceRegressionClassification::Unsupported => unsupported_count += 1,
            PerformanceRegressionClassification::Stale => stale_count += 1,
        }
    }
    let status = if malformed_count > 0 {
        "malformed"
    } else if regressed_count > 0 {
        "regressed"
    } else if missing_baseline_count > 0 {
        "missing-baseline"
    } else if stale_count > 0 {
        "stale"
    } else if unsupported_count > 0 {
        "unsupported"
    } else if inconclusive_count > 0 {
        "inconclusive"
    } else if lane_count > 0 && improved_count + unchanged_count == lane_count {
        "passed"
    } else if lane_count > 0 {
        "mixed"
    } else {
        "missing"
    };
    Ok(RunDashboardPerformanceRegressionLanes {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        lane_count,
        improved_count,
        unchanged_count,
        regressed_count,
        inconclusive_count,
        missing_baseline_count,
        unsupported_count,
        stale_count,
        malformed_count,
        evidence_refs,
        lanes,
        artifacts,
        boundary,
    })
}

fn read_dashboard_qa_worker_assignments(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardQaWorkerAssignments> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_qa_worker_assignment,
    )?;
    let boundary = "Read-only QA worker assignment evidence; dashboard/Studio surfaces must not spawn workers, execute commands, write trusted state, auto-fix, auto-apply, auto-merge, or claim quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardQaWorkerAssignments {
            present: false,
            empty_state: "No QA worker assignment evidence is indexed for this run.".to_string(),
            status: "missing".to_string(),
            assignment_count: 0,
            passed_count: 0,
            failed_count: 0,
            deferred_count: 0,
            blocked_count: 0,
            exhausted_count: 0,
            malformed_count: 0,
            evidence_refs: Vec::new(),
            plans: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut plans = Vec::new();
    let mut malformed_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_count += 1;
            continue;
        };
        match serde_json::from_value::<QaWorkerAssignmentArtifact>(value.clone()) {
            Ok(plan) if plan.validate().is_ok() => plans.push(plan),
            _ => malformed_count += 1,
        }
    }
    plans.sort_by(|left, right| {
        (left.run_id.as_str(), left.plan_id.as_str())
            .cmp(&(right.run_id.as_str(), right.plan_id.as_str()))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let mut assignment_count = 0usize;
    let mut passed_count = 0usize;
    let mut failed_count = 0usize;
    let mut deferred_count = 0usize;
    let mut blocked_count = 0usize;
    let mut exhausted_count = 0usize;
    for plan in &plans {
        assignment_count += plan.assignments.len();
        for assignment in &plan.assignments {
            match assignment.status {
                QaWorkerAssignmentStatus::Passed => passed_count += 1,
                QaWorkerAssignmentStatus::Failed => failed_count += 1,
                QaWorkerAssignmentStatus::Deferred => deferred_count += 1,
                QaWorkerAssignmentStatus::Blocked => blocked_count += 1,
                QaWorkerAssignmentStatus::Exhausted => exhausted_count += 1,
                QaWorkerAssignmentStatus::Assigned => {}
            }
        }
    }
    let status = if malformed_count > 0 {
        "malformed"
    } else if blocked_count > 0 {
        "blocked"
    } else if exhausted_count > 0 {
        "exhausted"
    } else if failed_count > 0 {
        "failed"
    } else if deferred_count > 0 {
        "deferred"
    } else if assignment_count > 0 && passed_count == assignment_count {
        "passed"
    } else if assignment_count > 0 {
        "assigned"
    } else {
        "missing"
    };
    Ok(RunDashboardQaWorkerAssignments {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        assignment_count,
        passed_count,
        failed_count,
        deferred_count,
        blocked_count,
        exhausted_count,
        malformed_count,
        evidence_refs,
        plans,
        artifacts,
        boundary,
    })
}

fn dashboard_artifact_is_runtime_invariant_evidence(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("runtime_invariant_evidence")
        || artifact.id.contains("runtime-invariant")
        || artifact.id.contains("invariant-evidence")
        || artifact.path.contains("runtime-invariant")
        || artifact.path.contains("invariant-evidence")
}

fn read_dashboard_runtime_invariants(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
    run: &serde_json::Value,
) -> Result<RunDashboardRuntimeInvariants> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_runtime_invariant_evidence,
    )?;
    let boundary = "Read-only runtime invariant evidence linked from Rust validation; dashboard/Studio surfaces must not mutate source, execute commands, launch workers, auto-fix, auto-apply, auto-merge, or claim gameplay quality guarantees.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardRuntimeInvariants {
            present: false,
            empty_state: "No runtime invariant evidence is indexed for this run.".to_string(),
            status: "missing".to_string(),
            check_count: 0,
            passed_count: 0,
            failed_count: 0,
            unsupported_count: 0,
            missing_count: 0,
            malformed_count: 0,
            stale_count: 0,
            evidence_refs: Vec::new(),
            summaries: Vec::new(),
            evidence: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let current_run_id = json_string(run, "id").unwrap_or_else(|| run_id_from_run_dir(run_dir));
    let mut evidence_refs = Vec::new();
    let mut parsed_evidence = Vec::new();
    let mut summaries = Vec::new();
    let mut malformed_artifact_count = 0usize;
    let mut stale_artifact_count = 0usize;
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if artifact.read_error.is_some() {
            malformed_artifact_count += 1;
            continue;
        }
        let Some(value) = &artifact.value else {
            malformed_artifact_count += 1;
            continue;
        };
        match serde_json::from_value::<RuntimeInvariantEvidence>(value.clone()) {
            Ok(runtime_evidence) if runtime_evidence.validate().is_ok() => {
                let summary = runtime_evidence.summary();
                if runtime_evidence.run_id != current_run_id {
                    stale_artifact_count += 1;
                }
                summaries.push(summary);
                parsed_evidence.push(runtime_evidence);
            }
            _ => malformed_artifact_count += 1,
        }
    }
    summaries.sort_by(|left, right| {
        (
            left.run_id.as_str(),
            left.scenario_id.as_deref().unwrap_or_default(),
            left.model_id.as_str(),
        )
            .cmp(&(
                right.run_id.as_str(),
                right.scenario_id.as_deref().unwrap_or_default(),
                right.model_id.as_str(),
            ))
    });
    parsed_evidence.sort_by(|left, right| {
        (
            left.run_id.as_str(),
            left.scenario_id.as_deref().unwrap_or_default(),
            left.model_id.as_str(),
        )
            .cmp(&(
                right.run_id.as_str(),
                right.scenario_id.as_deref().unwrap_or_default(),
                right.model_id.as_str(),
            ))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let check_count = summaries.iter().map(|summary| summary.check_count).sum();
    let passed_count = summaries.iter().map(|summary| summary.passed_count).sum();
    let failed_count = summaries.iter().map(|summary| summary.failed_count).sum();
    let unsupported_count = summaries
        .iter()
        .map(|summary| summary.unsupported_count)
        .sum();
    let missing_count = summaries.iter().map(|summary| summary.missing_count).sum();
    let malformed_count = summaries
        .iter()
        .map(|summary| summary.malformed_count)
        .sum::<usize>()
        + malformed_artifact_count;
    let stale_count = summaries
        .iter()
        .map(|summary| summary.stale_count)
        .sum::<usize>()
        + stale_artifact_count;
    let status = if malformed_count > 0 {
        "malformed"
    } else if stale_count > 0 {
        "stale"
    } else if failed_count > 0 {
        "failed"
    } else if unsupported_count > 0 || missing_count > 0 {
        "attention"
    } else if check_count > 0 {
        "passed"
    } else {
        "missing"
    };

    Ok(RunDashboardRuntimeInvariants {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        check_count,
        passed_count,
        failed_count,
        unsupported_count,
        missing_count,
        malformed_count,
        stale_count,
        evidence_refs,
        summaries,
        evidence: parsed_evidence,
        artifacts,
        boundary,
    })
}

fn dashboard_artifact_is_source_apply_worktree_context(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("source_apply_worktree_context")
        || artifact.id.contains("source-apply-worktree-context")
        || artifact.path.contains("source-apply-worktree-context")
}

fn read_dashboard_source_apply_worktree_context(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardSourceApplyWorktreeContext> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_source_apply_worktree_context,
    )?;
    let boundary = "Read-only source apply worktree context evidence from Rust validation; browser/dashboard/Studio surfaces must not apply patches, execute commands, write trusted files, merge branches, or bypass review gates.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardSourceApplyWorktreeContext {
            present: false,
            empty_state: "No source apply worktree context evidence is indexed for this run."
                .to_string(),
            status: "missing".to_string(),
            target_count: 0,
            blocked_count: 0,
            evidence_refs: Vec::new(),
            reports: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut reports = Vec::new();
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if let Some(value) = &artifact.value {
            if let Ok(report) =
                serde_json::from_value::<SourceApplyWorktreeContextReport>(value.clone())
            {
                reports.push(report);
            }
        }
    }
    reports.sort_by(|left, right| {
        (
            left.policy_id.as_str(),
            left.branch.as_str(),
            left.head_commit.as_str(),
        )
            .cmp(&(
                right.policy_id.as_str(),
                right.branch.as_str(),
                right.head_commit.as_str(),
            ))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let target_count = reports.iter().map(|report| report.targets.len()).sum();
    let blocked_count = reports
        .iter()
        .map(|report| report.blocked_reasons.len())
        .sum();
    let status = if artifacts
        .iter()
        .any(|artifact| artifact.read_error.is_some())
    {
        "malformed"
    } else if reports
        .iter()
        .any(SourceApplyWorktreeContextReport::is_blocked)
    {
        "blocked"
    } else if reports.is_empty() {
        "malformed"
    } else {
        "passed"
    };
    Ok(RunDashboardSourceApplyWorktreeContext {
        present: true,
        empty_state: String::new(),
        status: status.to_string(),
        target_count,
        blocked_count,
        evidence_refs,
        reports,
        artifacts,
        boundary,
    })
}

fn read_dashboard_asset_preview(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardAssetPreview> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_asset_preview_evidence,
    )?;
    let boundary = "Read-only asset preview evidence from Rust-exported artifacts; browser surfaces display escaped local metadata only and never fetch remote assets, upload files, write trusted state, or execute commands.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardAssetPreview {
            present: false,
            empty_state: "No asset preview evidence is indexed for this run.".to_string(),
            preview_count: 0,
            warning_count: 0,
            image_count: 0,
            atlas_frame_count: 0,
            tilemap_count: 0,
            audio_count: 0,
            font_count: 0,
            evidence_refs: Vec::new(),
            records: Vec::new(),
            warnings: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut records = Vec::new();
    let mut warnings = Vec::new();
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if let Some(value) = &artifact.value {
            if let Ok(evidence) = serde_json::from_value::<AssetPreviewEvidence>(value.clone()) {
                records.extend(evidence.previews);
                warnings.extend(evidence.warnings);
            }
        }
    }
    records.sort_by(|left, right| left.asset_id.cmp(&right.asset_id));
    warnings.sort_by(|left, right| {
        (
            left.asset_id.as_deref(),
            left.kind.as_str(),
            left.message.as_str(),
        )
            .cmp(&(
                right.asset_id.as_deref(),
                right.kind.as_str(),
                right.message.as_str(),
            ))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let image_count = records
        .iter()
        .filter(|record| record.image.is_some())
        .count();
    let atlas_frame_count = records.iter().map(|record| record.atlas_frames.len()).sum();
    let tilemap_count = records
        .iter()
        .filter(|record| record.tilemap.is_some())
        .count();
    let audio_count = records
        .iter()
        .filter(|record| record.audio.is_some())
        .count();
    let font_count = records
        .iter()
        .filter(|record| record.font.is_some())
        .count();
    Ok(RunDashboardAssetPreview {
        present: true,
        empty_state: String::new(),
        preview_count: records.len(),
        warning_count: warnings.len(),
        image_count,
        atlas_frame_count,
        tilemap_count,
        audio_count,
        font_count,
        evidence_refs,
        records,
        warnings,
        artifacts,
        boundary,
    })
}

fn read_dashboard_asset_loading(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardAssetLoading> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_runtime_asset_load_evidence,
    )?;
    let boundary = "Read-only runtime loading evidence from Rust-exported artifacts; browser surfaces display escaped local evidence only and never fetch remote assets, upload files, write trusted state, or execute commands.".to_string();
    if artifacts.is_empty() {
        return Ok(RunDashboardAssetLoading {
            present: false,
            empty_state: "No runtime asset loading evidence is indexed for this run.".to_string(),
            attempt_count: 0,
            loaded_count: 0,
            failed_count: 0,
            rejected_count: 0,
            fallback_count: 0,
            evidence_refs: Vec::new(),
            records: Vec::new(),
            artifacts,
            boundary,
        });
    }

    let mut evidence_refs = Vec::new();
    let mut records = Vec::new();
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if let Some(value) = &artifact.value {
            if let Ok(evidence) = serde_json::from_value::<RuntimeAssetLoadEvidence>(value.clone())
            {
                records.extend(evidence.loads);
            }
        }
    }
    records.sort_by(|left, right| {
        (
            left.asset_id.as_str(),
            left.attempt_id.as_str(),
            left.started_at_unix_ms,
        )
            .cmp(&(
                right.asset_id.as_str(),
                right.attempt_id.as_str(),
                right.started_at_unix_ms,
            ))
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let loaded_count = records
        .iter()
        .filter(|record| record.status == RuntimeAssetLoadStatus::Loaded)
        .count();
    let failed_count = records
        .iter()
        .filter(|record| record.status == RuntimeAssetLoadStatus::Failed)
        .count();
    let rejected_count = records
        .iter()
        .filter(|record| record.status == RuntimeAssetLoadStatus::Rejected)
        .count();
    let fallback_count = records
        .iter()
        .filter(|record| record.status == RuntimeAssetLoadStatus::Fallback)
        .count();
    Ok(RunDashboardAssetLoading {
        present: true,
        empty_state: String::new(),
        attempt_count: records.len(),
        loaded_count,
        failed_count,
        rejected_count,
        fallback_count,
        evidence_refs,
        records,
        artifacts,
        boundary,
    })
}

fn read_dashboard_asset_integrity(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardAssetIntegrity> {
    let artifacts = select_dashboard_artifacts(
        run_dir,
        evidence,
        dashboard_artifact_is_asset_reference_integrity,
    )?;
    if artifacts.is_empty() {
        return Ok(RunDashboardAssetIntegrity {
            present: false,
            empty_state: "No asset reference integrity evidence is indexed for this run."
                .to_string(),
            warning_count: 0,
            stale_hash_count: 0,
            missing_ref_count: 0,
            invalid_type_count: 0,
            evidence_refs: Vec::new(),
            warnings: Vec::new(),
            artifacts,
        });
    }

    let mut warnings = Vec::new();
    let mut evidence_refs = Vec::new();
    for artifact in &artifacts {
        evidence_refs.push(artifact.path.clone());
        if let Some(value) = &artifact.value {
            if let Some(items) = value.get("warnings").and_then(|value| value.as_array()) {
                for item in items {
                    if let Ok(warning) = serde_json::from_value::<
                        ProjectAssetReferenceIntegrityWarning,
                    >(item.clone())
                    {
                        warnings.push(warning);
                    }
                }
            }
        }
    }
    warnings.sort_by(|left, right| {
        (
            left.kind.as_str(),
            left.field.as_str(),
            left.asset_id.as_str(),
        )
            .cmp(&(
                right.kind.as_str(),
                right.field.as_str(),
                right.asset_id.as_str(),
            ))
    });
    warnings.dedup_by(|left, right| {
        left.kind == right.kind && left.field == right.field && left.asset_id == right.asset_id
    });
    evidence_refs.sort();
    evidence_refs.dedup();
    let warning_count = warnings.len();
    let stale_hash_count = warnings
        .iter()
        .filter(|warning| warning.kind == "stale_asset_hash")
        .count();
    let missing_ref_count = warnings
        .iter()
        .filter(|warning| {
            matches!(
                warning.kind.as_str(),
                "missing_asset_ref" | "missing_asset_file"
            )
        })
        .count();
    let invalid_type_count = warnings
        .iter()
        .filter(|warning| warning.kind == "invalid_asset_type")
        .count();
    Ok(RunDashboardAssetIntegrity {
        present: true,
        empty_state: String::new(),
        warning_count,
        stale_hash_count,
        missing_ref_count,
        invalid_type_count,
        evidence_refs,
        warnings,
        artifacts,
    })
}

fn dashboard_artifact_is_frame_metric(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("probe_call")
        .and_then(|value| value.as_str())
        == Some("getFrameStats")
        || artifact.id.contains("frame-stats")
        || artifact.path.contains("frame-stats")
}

fn dashboard_artifact_is_performance_metric(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("performance_metrics")
        || artifact.id.contains("performance")
        || artifact.path.contains("metrics")
}

fn dashboard_artifact_is_scenario_result(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("scenario_result")
        || artifact.id.contains("scenario-result")
        || artifact.path.contains("scenario-result")
}

fn dashboard_artifact_is_behavior_assertion_result(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("behavior_assertion_result")
        || artifact.id.contains("behavior-assertion-result")
        || artifact.path.contains("behavior-assertions")
}

fn dashboard_artifact_is_behavior_evidence_bundle(artifact: &EvidenceArtifact) -> bool {
    artifact
        .metadata
        .get("artifact")
        .and_then(|value| value.as_str())
        == Some("behavior_evidence_bundle")
        || artifact.id.contains("behavior-evidence-bundle")
        || artifact.path.contains("behavior-evidence-bundle")
}

fn dashboard_artifact_is_input_replay(artifact: &EvidenceArtifact) -> bool {
    matches!(
        artifact
            .metadata
            .get("artifact")
            .and_then(|value| value.as_str()),
        Some("input_replay") | Some("scenario_input_replay")
    ) || artifact.id.contains("input-replay")
        || artifact.path.contains("input-replay")
        || artifact.id.contains("scenario-input-replay")
        || artifact.path.contains("scenario-input-replay")
}

fn read_dashboard_replay(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
) -> Result<RunDashboardReplay> {
    let replay_artifacts =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_input_replay)?;
    if replay_artifacts.is_empty() {
        return Ok(RunDashboardReplay {
            present: false,
            empty_state: "No replay evidence artifacts were found for this run.".to_string(),
            sequences: Vec::new(),
        });
    }
    let scenario_results =
        select_dashboard_artifacts(run_dir, evidence, dashboard_artifact_is_scenario_result)?;
    let mut sequences = Vec::new();
    for artifact in replay_artifacts {
        let scenario_id = dashboard_artifact_scenario_id(&artifact);
        let replay_value = artifact
            .value
            .as_ref()
            .and_then(dashboard_replay_payload_value);
        let frames = replay_value.map_or_else(Vec::new, dashboard_replay_frames);
        let replay_id = replay_value
            .and_then(|value| json_string(value, "id"))
            .unwrap_or_else(|| artifact.id.clone());
        let event_count = replay_value
            .and_then(|value| value.get("events"))
            .and_then(|value| value.as_array())
            .map_or_else(
                || {
                    usize::from(
                        replay_value
                            .and_then(|value| value.get("schemaVersion"))
                            .and_then(|value| value.as_str())
                            == Some(SCENARIO_INPUT_REPLAY_ARTIFACT_SCHEMA_VERSION),
                    )
                },
                Vec::len,
            );
        let source = dashboard_replay_source(&artifact);
        let evidence_refs = std::iter::once(artifact.path.clone())
            .chain(
                scenario_results
                    .iter()
                    .filter(|result| {
                        dashboard_artifact_matches_scenario(result, scenario_id.as_deref())
                    })
                    .map(|result| result.path.clone()),
            )
            .collect::<Vec<_>>();
        sequences.push(RunDashboardReplaySequence {
            id: replay_id,
            source,
            scenario_id: scenario_id.clone(),
            replay_path: artifact.path.clone(),
            event_count,
            first_frame: frames.first().copied(),
            last_frame: frames.last().copied(),
            frames,
            evidence_refs,
            checkpoints: dashboard_replay_checkpoints(
                run_dir,
                scenario_id.as_deref(),
                &scenario_results,
            ),
            read_error: artifact.read_error.clone(),
        });
    }
    Ok(RunDashboardReplay {
        present: sequences
            .iter()
            .any(|sequence| sequence.read_error.is_none()),
        empty_state: if sequences
            .iter()
            .any(|sequence| sequence.read_error.is_none())
        {
            String::new()
        } else {
            "Replay evidence artifacts were indexed, but no readable replay files are available."
                .to_string()
        },
        sequences,
    })
}

fn dashboard_replay_checkpoints(
    run_dir: &Path,
    scenario_id: Option<&str>,
    scenario_results: &[RunDashboardArtifact],
) -> Vec<RunDashboardReplayCheckpoint> {
    scenario_results
        .iter()
        .filter(|result| dashboard_artifact_matches_scenario(result, scenario_id))
        .filter_map(|result| {
            let value = result.value.as_ref()?;
            let evidence = value.get("evidence")?;
            let world_state_path = evidence
                .get("world_state")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            let frame_stats_path = evidence
                .get("frame_stats")
                .and_then(|value| value.as_str())
                .map(str::to_string);
            // These refs come from scenario-result JSON, which may be external or
            // malformed; unlike evidence-index paths they are otherwise unchecked.
            // Constrain them to the run's evidence tree before reading so a value
            // like "../../secret.json" cannot disclose arbitrary files.
            let world_state = world_state_path
                .as_ref()
                .filter(|path| validate_evidence_artifact_path(path.as_str()).is_ok())
                .and_then(|path| read_json_value(run_dir.join(path)).ok());
            let frame_stats = frame_stats_path
                .as_ref()
                .filter(|path| validate_evidence_artifact_path(path.as_str()).is_ok())
                .and_then(|path| read_json_value(run_dir.join(path)).ok());
            let tick = world_state
                .as_ref()
                .and_then(|value| value.get("tick"))
                .and_then(|value| value.as_u64())
                .or_else(|| {
                    frame_stats
                        .as_ref()
                        .and_then(|value| value.get("tick"))
                        .and_then(|value| value.as_u64())
                });
            let frame = frame_stats
                .as_ref()
                .and_then(|value| value.get("frame"))
                .and_then(|value| value.as_u64())
                .or(tick);
            let checkpoint_id = format!(
                "{}-checkpoint",
                json_string(value, "scenario_id")
                    .or_else(|| dashboard_artifact_scenario_id(result))
                    .unwrap_or_else(|| result.id.clone())
            );
            Some(RunDashboardReplayCheckpoint {
                id: checkpoint_id,
                label: "Post-replay world state".to_string(),
                frame,
                tick,
                world_state_path,
                frame_stats_path,
                world_state,
            })
        })
        .collect()
}

fn dashboard_worker_count(evidence: &[EvidenceArtifact]) -> usize {
    let mut worker_ids = BTreeSet::new();
    for artifact in evidence {
        if let Some(worker_id) = artifact
            .metadata
            .get("worker_id")
            .and_then(|value| value.as_str())
        {
            worker_ids.insert(worker_id.to_string());
        }
    }
    worker_ids.len()
}

fn read_dashboard_journal(
    run_dir: &Path,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) -> RunDashboardJournal {
    let path = "journal.md".to_string();
    let absolute_path = run_dir.join(&path);
    match fs::read_to_string(&absolute_path) {
        Ok(body) => parse_dashboard_journal(&path, true, None, &body, evidence, mutations),
        Err(error) if error.kind() == ErrorKind::NotFound => parse_dashboard_journal(
            &path,
            false,
            Some("missing journal artifact".to_string()),
            "",
            evidence,
            mutations,
        ),
        // The journal file is present but unreadable (e.g. invalid UTF-8 or a
        // permission error). Report it as existing with a read_error so it is
        // distinguishable from a genuinely missing journal.
        Err(error) => parse_dashboard_journal(
            &path,
            true,
            Some(format!("failed to read journal artifact: {error}")),
            "",
            evidence,
            mutations,
        ),
    }
}

fn parse_dashboard_journal(
    path: &str,
    exists: bool,
    read_error: Option<String>,
    journal: &str,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) -> RunDashboardJournal {
    let entries = dashboard_journal_entries(journal, evidence, mutations);
    let evidence_refs = collect_entry_refs(&entries, |entry| &entry.evidence_refs);
    let verdict_refs = collect_entry_refs(&entries, |entry| &entry.verdict_refs);
    let mutation_refs = collect_entry_refs(&entries, |entry| &entry.mutation_refs);
    RunDashboardJournal {
        path: path.to_string(),
        exists,
        read_error,
        summary: dashboard_journal_summary(journal, &entries),
        entries,
        evidence_refs,
        verdict_refs,
        mutation_refs,
    }
}

fn dashboard_journal_entries(
    journal: &str,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) -> Vec<RunDashboardJournalEntry> {
    let mut entries = Vec::new();
    let mut current_heading = String::new();
    let mut current_level = 0usize;
    let mut current_body = String::new();

    for line in journal.lines() {
        if let Some((level, heading)) = markdown_heading(line) {
            push_dashboard_journal_entry(
                &mut entries,
                &current_heading,
                current_level,
                &current_body,
                evidence,
                mutations,
            );
            current_heading = heading;
            current_level = level;
            current_body.clear();
        } else {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }
    push_dashboard_journal_entry(
        &mut entries,
        &current_heading,
        current_level,
        &current_body,
        evidence,
        mutations,
    );
    entries
}

fn push_dashboard_journal_entry(
    entries: &mut Vec<RunDashboardJournalEntry>,
    heading: &str,
    level: usize,
    body: &str,
    evidence: &[EvidenceArtifact],
    mutations: &[MutationProposal],
) {
    let body = body.trim();
    if heading.trim().is_empty() && body.is_empty() {
        return;
    }
    let fallback_heading = if heading.trim().is_empty() {
        "Journal"
    } else {
        heading.trim()
    };
    let category = dashboard_journal_category(fallback_heading);
    let entry_text = format!("{fallback_heading}\n{body}");
    let index = entries.len() + 1;
    entries.push(RunDashboardJournalEntry {
        id: format!("journal-entry-{index}-{}", slug_for_id(fallback_heading)),
        heading: fallback_heading.to_string(),
        level,
        category,
        body: body.to_string(),
        evidence_refs: extract_journal_evidence_refs(&entry_text, evidence),
        verdict_refs: extract_journal_verdict_refs(&entry_text),
        mutation_refs: extract_journal_mutation_refs(&entry_text, mutations),
    });
}

fn extract_journal_evidence_refs(text: &str, evidence: &[EvidenceArtifact]) -> Vec<String> {
    let mut refs = BTreeSet::new();
    for artifact in evidence {
        if text.contains(&artifact.path) || text.contains(&artifact.id) {
            refs.insert(artifact.path.clone());
        }
    }
    refs.into_iter().collect()
}

pub fn bind_run_command_context(
    run_dir: impl AsRef<Path>,
    context: RunCommandContext,
) -> Result<RunCommandContext> {
    let run_dir = run_dir.as_ref();
    let run_path = run_dir.join("run.json");
    let mut run = read_json_value(&run_path)?;
    let run_object = run
        .as_object_mut()
        .ok_or_else(|| anyhow!("run.json must be a JSON object"))?;
    run_object.insert("run_command_context".to_string(), json!(context.clone()));
    write_json_atomic(&run_path, &run)?;
    append_ledger_event(
        run_dir,
        "run.command_context_recorded",
        "run-core",
        json!({
            "schema_version": context.schema_version,
            "seed_path": context.seed_path,
            "workers": context.workers,
            "runs_root": context.runs_root,
            "project_root": context.project_root,
            "manifest_path": context.manifest_path,
            "scenario_pack_id": context.scenario_pack_id,
            "transaction_path": context.transaction_path,
            "runtime_target": context.runtime_target,
            "browser_boundary": context.browser_boundary,
            "cdp_transport": context.cdp_transport
        }),
    )?;
    Ok(context)
}

pub fn bind_run_project_metadata(
    run_dir: impl AsRef<Path>,
    metadata: ProjectRunMetadata,
) -> Result<ProjectRunMetadata> {
    let run_dir = run_dir.as_ref();
    let run_path = run_dir.join("run.json");
    let mut run = read_json_value(&run_path)?;
    let run_object = run
        .as_object_mut()
        .ok_or_else(|| anyhow!("run.json must be a JSON object"))?;
    run_object.insert("project".to_string(), json!(metadata.clone()));
    write_json_atomic(&run_path, &run)?;
    append_ledger_event(
        run_dir,
        "run.project_bound",
        "run-core",
        json!({
            "project_id": metadata.id,
            "manifest_path": metadata.manifest_path,
            "manifest_hash": metadata.manifest_hash,
            "seed_path": metadata.seed_path,
            "scene_paths": metadata.scenes.iter().map(|scene| scene.path.clone()).collect::<Vec<_>>(),
            "scenario_pack_id": metadata.scenario_pack.as_ref().map(|pack| pack.id.clone()),
            "transaction_id": metadata.transaction_id
        }),
    )?;
    Ok(metadata)
}

pub fn create_run(
    seed_path: impl AsRef<Path>,
    runs_root: impl AsRef<Path>,
) -> Result<RunArtifacts> {
    let seed_path = seed_path.as_ref();
    let runs_root = runs_root.as_ref();
    let seed_yaml = fs::read_to_string(seed_path)
        .with_context(|| format!("failed to read Seed file {}", seed_path.display()))?;
    let seed = Seed::from_path(seed_path)?;
    let seed_base_dir = seed_path.parent().unwrap_or_else(|| Path::new("."));

    fs::create_dir_all(runs_root)
        .with_context(|| format!("failed to create runs root {}", runs_root.display()))?;

    let created_at_unix_ms = unix_millis()?;
    let run_id = format!("run-{created_at_unix_ms}-{}", std::process::id());
    let run_dir = runs_root.join(&run_id);
    fs::create_dir(&run_dir)
        .with_context(|| format!("failed to create run directory {}", run_dir.display()))?;
    fs::create_dir(run_dir.join("evidence")).context("failed to create evidence directory")?;

    let command_context = run_command_context_for_run(seed_path, runs_root, 1, None, None);
    write_json(
        &run_dir.join("run.json"),
        &json!({
            "id": run_id,
            "seed_id": seed.id,
            "seed_title": seed.title,
            "status": "created",
            "created_at_unix_ms": created_at_unix_ms,
            "run_command_context": command_context,
        }),
    )?;
    fs::write(run_dir.join("seed.snapshot.yaml"), seed_yaml)
        .context("failed to write seed snapshot")?;
    copy_replay_references_to_run(&seed, seed_base_dir, &run_dir)?;
    write_ledger_created(&run_dir.join("ledger.jsonl"), created_at_unix_ms)?;
    fs::write(run_dir.join("journal.md"), initial_journal()).context("failed to write journal")?;
    write_json(
        &run_dir.join("verdict.json"),
        &json!({ "status": "pending" }),
    )?;
    write_evidence_index(
        &run_dir,
        &EvidenceIndex {
            artifacts: Vec::new(),
        },
    )?;
    if seed.id == "evolve.v1.demo" {
        materialize_evolve_demo_controlled_failure(&run_dir)?;
    }

    Ok(RunArtifacts { run_dir })
}

fn materialize_evolve_demo_controlled_failure(run_dir: &Path) -> Result<()> {
    let scenario_result_rel = "evidence/scenarios/evolve-controlled-failure/scenario-result.json";
    let scenario_result_path = run_dir.join(scenario_result_rel);
    if let Some(parent) = scenario_result_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create scenario evidence directory {}",
                parent.display()
            )
        })?;
    }
    write_json(
        &scenario_result_path,
        &json!({
            "schema_version": "1",
            "artifact": "scenario_result",
            "scenario_id": "evolve-controlled-failure",
            "status": "failed",
            "summary": "controlled assertion failure for evolve demo",
            "failures": [{
                "kind": "scenario_assertion_failure",
                "assertion": "world_state.object.x equals 999",
                "actual": 0,
                "expected": 999,
                "evidence_ref": scenario_result_rel
            }]
        }),
    )?;
    add_evidence_artifact(
        run_dir,
        "scenario-result-evolve-controlled-failure",
        "application/json",
        scenario_result_rel,
        json!({
            "artifact": "scenario_result",
            "scenario_id": "evolve-controlled-failure",
            "status": "failed",
            "controlled_demo": true
        }),
    )?;
    write_json(
        &run_dir.join("verdict.json"),
        &json!({
            "status": "failed",
            "summary": "controlled evolve demo scenario assertion failure",
            "failures": [{
                "kind": "scenario_assertion_failure",
                "path": scenario_result_rel,
                "evidence_ref": scenario_result_rel,
                "scenario_id": "evolve-controlled-failure"
            }],
            "evidence_refs": [scenario_result_rel],
            "metadata": {
                "evaluator": "ouroforge-evolve-demo-smoke-v1",
                "controlled_demo": true
            }
        }),
    )?;
    append_ledger_event(
        run_dir,
        "scenario.completed",
        "run-cli",
        json!({
            "scenario_id": "evolve-controlled-failure",
            "status": "failed",
            "path": scenario_result_rel,
            "controlled_demo": true
        }),
    )?;
    update_journal(run_dir)?;
    Ok(())
}

fn copy_replay_references_to_run(seed: &Seed, seed_base_dir: &Path, run_dir: &Path) -> Result<()> {
    let mut copied = std::collections::BTreeSet::new();
    for replay_ref in seed.replay_references() {
        if !copied.insert(replay_ref.path.clone()) {
            continue;
        }
        let source = seed_base_dir.join(&replay_ref.path);
        let target = run_dir.join(&replay_ref.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create replay directory {}", parent.display())
            })?;
        }
        fs::copy(&source, &target).with_context(|| {
            format!(
                "failed to copy replay reference {} to {}",
                source.display(),
                target.display()
            )
        })?;
    }
    Ok(())
}

fn initial_journal() -> &'static str {
    "# Ouroforge Run Journal\n\n## Seed\n\n## Hypothesis\n\n## Observations\n\n## Evidence\n\n## Verdict\n\n## Next Mutation\n"
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;

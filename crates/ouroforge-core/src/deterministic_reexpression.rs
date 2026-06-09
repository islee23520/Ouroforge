use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::behavior_runtime::{
    BehaviorAction, BehaviorArtifact, BehaviorDefinition, BehaviorDraftArtifact,
    BehaviorDraftAuthor, BehaviorDraftEvidenceRef, BehaviorDraftScenarioImpact,
    BehaviorDraftTarget, BehaviorDraftValidationStatus, BehaviorTrigger,
    BehaviorValidationAuthority, BEHAVIOR_ARTIFACT_SCHEMA_VERSION,
};
use crate::legacy_logic_ingestion::{
    BehavioralUnitRecord, EraRHandoffState, FidelityGrade, ReDerivationTask,
};
use crate::tacit_oracle_capture::{CapturedOracleStatus, OracleSpec};

pub const DETERMINISTIC_REEXPRESSION_SCHEMA_VERSION: &str = "deterministic.reexpression.v1";
pub const DETERMINISTIC_REEXPRESSION_BOUNDARY: &str = "Rust data-plane deterministic re-expression from captured oracle evidence; one-way on-ramp; source-project/open-text clean-room inputs only; no source translation; no decompiled source copying; no engine runtime bridge; source-apply/review gates required for writes; no ported claim without downstream differential verification";
pub const DETERMINISTIC_REEXPRESSION_DIGEST_ALGORITHM: &str = "fnv1a64";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReExpressionTargetDimensionality {
    TwoD,
    TwoPointFiveD,
    ThreeD,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReExpressionRequest {
    pub project_id: String,
    pub scene_path: String,
    pub scene_hash: String,
    pub target_dimensionality: ReExpressionTargetDimensionality,
    pub units: Vec<BehavioralUnitRecord>,
    pub oracle_specs: Vec<OracleSpec>,
    #[serde(default)]
    pub skeleton_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeterministicReExpressionReport {
    pub schema_version: String,
    pub boundary: String,
    pub deterministic_digest_algorithm: String,
    pub deterministic_digest: String,
    pub target_dimensionality: ReExpressionTargetDimensionality,
    pub plans: Vec<ReExpressionPlan>,
    pub behavior_drafts: Vec<BehaviorDraftArtifact>,
    pub fidelity_report: ReExpressionFidelityReport,
    pub re_derivation_tasks: Vec<ReDerivationTask>,
    pub verification_handoffs: Vec<VerificationHandoff>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReExpressionPlan {
    pub id: String,
    pub unit_id: String,
    pub target_runtime: String,
    pub source_ir_refs: Vec<String>,
    pub skeleton_refs: Vec<String>,
    pub oracle_ref: Option<String>,
    pub deterministic_constraints: Vec<String>,
    pub unsupported_gaps: Vec<String>,
    pub gate_handoff: ReExpressionGateHandoff,
    pub fidelity_grade: FidelityGrade,
    pub handoff_state: EraRHandoffState,
    pub ported_claim_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReExpressionGateHandoff {
    pub source_apply_required: bool,
    pub review_gate_required: bool,
    pub rollback_required: bool,
    pub writes_artifacts_directly: bool,
    pub trusted_write_authority: bool,
    pub provenance_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationHandoff {
    pub unit_id: String,
    pub oracle_ref: String,
    pub primary_state_hash: String,
    pub secondary_render_digest: Option<String>,
    pub verification_rule: String,
    pub downstream_milestone: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReExpressionFidelityReport {
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub no_oracle_not_ported: bool,
    pub clean_room_source_only: bool,
    pub deterministic_reexpression: bool,
    pub source_apply_gate_required: bool,
    pub studio_trusted_write_authority: bool,
    pub blocked_or_unsupported: Vec<String>,
    pub gap_summary: Vec<String>,
}

pub fn reexpress_deterministic_behaviors(
    request: &ReExpressionRequest,
) -> Result<DeterministicReExpressionReport> {
    validate_request(request)?;

    let mut units = request.units.clone();
    units.sort_by(|a, b| a.id.cmp(&b.id));
    let mut oracles = request.oracle_specs.clone();
    oracles.sort_by(|a, b| a.unit_id.cmp(&b.unit_id).then(a.id.cmp(&b.id)));
    let oracle_by_unit = oracles.iter().fold(
        BTreeMap::<String, Vec<&OracleSpec>>::new(),
        |mut acc, oracle| {
            acc.entry(oracle.unit_id.clone()).or_default().push(oracle);
            acc
        },
    );

    let mut plans = Vec::new();
    let mut behavior_drafts = Vec::new();
    let mut verification_handoffs = Vec::new();
    let mut re_derivation_tasks = Vec::new();
    let mut blocked_or_unsupported = BTreeSet::new();
    let mut gap_summary = BTreeSet::new();

    for unit in &units {
        let oracle = oracle_by_unit.get(&unit.id).and_then(|matches| {
            matches
                .iter()
                .copied()
                .find(|oracle| oracle_is_usable(oracle))
        });
        let mut gaps = unit.gaps.clone();
        let (grade, handoff_state) = if unit.fidelity_grade == FidelityGrade::Red
            || unit.handoff_state == EraRHandoffState::RejectOrDefer
        {
            gaps.push("source unit is blocked or rejected before re-expression".to_string());
            (FidelityGrade::Red, EraRHandoffState::RejectOrDefer)
        } else if oracle.is_none() {
            gaps.push("captured oracle is required before deterministic re-expression".to_string());
            (FidelityGrade::Yellow, EraRHandoffState::CaptureOracle)
        } else if unit.ported_claim_allowed {
            gaps.push("upstream unit attempted a ported claim before verification".to_string());
            (FidelityGrade::Red, EraRHandoffState::RejectOrDefer)
        } else {
            (FidelityGrade::Green, EraRHandoffState::Verify)
        };

        let provenance_refs = oracle
            .map(|oracle| oracle.provenance_refs.clone())
            .unwrap_or_default();
        let plan = ReExpressionPlan {
            id: format!("reexpr.{}", stable_id(&unit.id)),
            unit_id: unit.id.clone(),
            target_runtime: "behavior_runtime".to_string(),
            source_ir_refs: unit.provenance_node_ids.clone(),
            skeleton_refs: request.skeleton_refs.clone(),
            oracle_ref: oracle.map(|oracle| oracle.id.clone()),
            deterministic_constraints: deterministic_constraints(request.target_dimensionality),
            unsupported_gaps: sorted_unique(gaps.clone()),
            gate_handoff: ReExpressionGateHandoff {
                source_apply_required: true,
                review_gate_required: true,
                rollback_required: true,
                writes_artifacts_directly: false,
                trusted_write_authority: false,
                provenance_refs,
            },
            fidelity_grade: grade,
            handoff_state,
            ported_claim_allowed: false,
        };

        if grade == FidelityGrade::Green {
            let oracle = oracle.expect("green requires oracle");
            behavior_drafts.push(build_behavior_draft(request, unit, oracle));
            verification_handoffs.push(VerificationHandoff {
                unit_id: unit.id.clone(),
                oracle_ref: oracle.id.clone(),
                primary_state_hash: oracle.primary_state_hash.clone(),
                secondary_render_digest: oracle.secondary_render_digest.clone(),
                verification_rule: verification_rule(request.target_dimensionality),
                downstream_milestone: "Era R M111 differential verification A/B".to_string(),
            });
        } else {
            for gap in &plan.unsupported_gaps {
                gap_summary.insert(format!("{}: {gap}", unit.id));
            }
            if grade == FidelityGrade::Red {
                blocked_or_unsupported.insert(format!("{} blocked: {}", unit.id, gaps.join("; ")));
            }
            re_derivation_tasks.push(ReDerivationTask {
                unit_id: unit.id.clone(),
                task: if grade == FidelityGrade::Red {
                    "reject_or_defer_reexpression".to_string()
                } else {
                    "capture_or_repair_oracle_before_reexpression".to_string()
                },
                reason: gaps.join("; "),
                handoff_state,
            });
        }

        plans.push(plan);
    }

    let green_count = plans
        .iter()
        .filter(|plan| plan.fidelity_grade == FidelityGrade::Green)
        .count();
    let yellow_count = plans
        .iter()
        .filter(|plan| plan.fidelity_grade == FidelityGrade::Yellow)
        .count();
    let red_count = plans
        .iter()
        .filter(|plan| plan.fidelity_grade == FidelityGrade::Red)
        .count();

    let digest_payload = serde_json::json!({
        "targetDimensionality": request.target_dimensionality,
        "plans": plans,
        "behaviorDrafts": behavior_drafts,
        "verificationHandoffs": verification_handoffs,
        "tasks": re_derivation_tasks,
    });

    Ok(DeterministicReExpressionReport {
        schema_version: DETERMINISTIC_REEXPRESSION_SCHEMA_VERSION.to_string(),
        boundary: DETERMINISTIC_REEXPRESSION_BOUNDARY.to_string(),
        deterministic_digest_algorithm: DETERMINISTIC_REEXPRESSION_DIGEST_ALGORITHM.to_string(),
        deterministic_digest: stable_digest(&digest_payload.to_string()),
        target_dimensionality: request.target_dimensionality,
        plans,
        behavior_drafts,
        fidelity_report: ReExpressionFidelityReport {
            green_count,
            yellow_count,
            red_count,
            no_oracle_not_ported: true,
            clean_room_source_only: true,
            deterministic_reexpression: true,
            source_apply_gate_required: true,
            studio_trusted_write_authority: false,
            blocked_or_unsupported: blocked_or_unsupported.into_iter().collect(),
            gap_summary: gap_summary.into_iter().collect(),
        },
        re_derivation_tasks,
        verification_handoffs,
    })
}

fn validate_request(request: &ReExpressionRequest) -> Result<()> {
    if request.project_id.trim().is_empty() {
        return Err(anyhow!("re-expression request requires a project id"));
    }
    if request.scene_path.trim().is_empty() || request.scene_path.contains("..") {
        return Err(anyhow!("re-expression request requires a safe scene path"));
    }
    if !is_hash_like(&request.scene_hash) {
        return Err(anyhow!(
            "re-expression request requires a deterministic scene hash"
        ));
    }
    if request.units.is_empty() {
        return Err(anyhow!("re-expression request requires behavioral units"));
    }
    for skeleton_ref in &request.skeleton_refs {
        let lower = skeleton_ref.to_ascii_lowercase();
        if skeleton_ref.contains("..")
            || lower.contains("decompiled")
            || lower.contains("shipped-build")
            || lower.contains("vendored_unity_runtime")
            || lower.contains("vendored_unreal_runtime")
        {
            return Err(anyhow!(
                "skeleton refs must be source-project/open-text and cannot name blocked runtime or decompiled inputs"
            ));
        }
    }
    Ok(())
}

fn oracle_is_usable(oracle: &OracleSpec) -> bool {
    matches!(
        oracle.status,
        CapturedOracleStatus::Captured | CapturedOracleStatus::Passing
    ) && !oracle.ported_claim_allowed
        && is_state_hash(&oracle.primary_state_hash)
        && !oracle.expected_events.is_empty()
        && oracle
            .provenance_refs
            .iter()
            .all(|reference| !is_blocked_reference(reference))
}

fn build_behavior_draft(
    request: &ReExpressionRequest,
    unit: &BehavioralUnitRecord,
    oracle: &OracleSpec,
) -> BehaviorDraftArtifact {
    let behavior_id = format!("behavior.{}", stable_id(&unit.id));
    let trigger_kind = trigger_kind_for(&unit.stimuli);
    let event_name = format!("oracle.{}", stable_id(&oracle.id));
    let behavior = BehaviorDefinition {
        id: behavior_id.clone(),
        entity_id: unit.name.clone(),
        triggers: vec![BehaviorTrigger {
            id: format!("trigger.{}", stable_id(&oracle.stimulus)),
            kind: trigger_kind,
            event: Some(event_name.clone()),
        }],
        conditions: Vec::new(),
        actions: oracle
            .expected_events
            .iter()
            .enumerate()
            .map(|(idx, event)| BehaviorAction {
                id: format!("action.{:02}.{}", idx + 1, stable_id(event)),
                kind: "emitEvent".to_string(),
                effect_kind: "emitEvent".to_string(),
                target_entity_id: Some(unit.name.clone()),
                value: Some(event.clone()),
                flag: None,
                state: None,
                event: Some(event.clone()),
                item: None,
                amount: None,
                dx: None,
                dy: None,
            })
            .collect(),
        state_machine: None,
        abilities: Vec::new(),
    };

    BehaviorDraftArtifact {
        schema_version: crate::behavior_runtime::BEHAVIOR_DRAFT_SCHEMA_VERSION.to_string(),
        draft_id: format!("draft.reexpr.{}", stable_id(&unit.id)),
        target: BehaviorDraftTarget {
            project_id: request.project_id.clone(),
            scene_path: request.scene_path.clone(),
            scene_hash: request.scene_hash.clone(),
        },
        proposed_behavior: BehaviorArtifact {
            schema_version: BEHAVIOR_ARTIFACT_SCHEMA_VERSION.to_string(),
            artifact_id: format!("artifact.reexpr.{}", stable_id(&unit.id)),
            scene_id: request.scene_path.clone(),
            validated_by: BehaviorValidationAuthority {
                authority: "deterministic_reexpression".to_string(),
                validation_status: "candidate_requires_source_apply_review".to_string(),
                review_decision_id: None,
            },
            behaviors: vec![behavior],
        },
        rationale: "Clean-room deterministic re-expression candidate generated from captured oracle evidence; not a ported claim.".to_string(),
        linked_evidence: vec![BehaviorDraftEvidenceRef {
            id: oracle.id.clone(),
            kind: "captured_oracle".to_string(),
            path: "generated/era-r/oracles.json".to_string(),
            summary: format!(
                "Primary state hash {}; source-apply review required before writes",
                oracle.primary_state_hash
            ),
        }],
        expected_scenario_impact: vec![BehaviorDraftScenarioImpact {
            scenario_id: format!("verify.{}", stable_id(&unit.id)),
            summary: verification_rule(request.target_dimensionality),
            expected_verdict: "must_match_oracle_state_hash".to_string(),
        }],
        author: BehaviorDraftAuthor {
            source: "agent_clean_room_reexpression".to_string(),
            actor: Some("ouroforge-core".to_string()),
        },
        validation_status: BehaviorDraftValidationStatus::Drafted,
        blocked_reasons: Vec::new(),
        untrusted_boundary: "candidate only; all writes require source-apply review gate and rollback evidence".to_string(),
    }
}

fn deterministic_constraints(target: ReExpressionTargetDimensionality) -> Vec<String> {
    let mut constraints = vec![
        "fixed-tick evaluation".to_string(),
        "stable ordered events".to_string(),
        "source-apply review gate before writes".to_string(),
        "ported claim forbidden before M111 verification".to_string(),
    ];
    constraints.push(verification_rule(target));
    constraints
}

fn verification_rule(target: ReExpressionTargetDimensionality) -> String {
    match target {
        ReExpressionTargetDimensionality::TwoD => {
            "2D bit-exact state hash must match captured oracle".to_string()
        }
        ReExpressionTargetDimensionality::TwoPointFiveD => {
            "2.5D deterministic state-hash primary with perceptual render corroboration only"
                .to_string()
        }
        ReExpressionTargetDimensionality::ThreeD => {
            "3D deterministic state-hash primary with SSIM/pixel-diff secondary only".to_string()
        }
    }
}

fn trigger_kind_for(stimuli: &[String]) -> String {
    let joined = stimuli.join(" ").to_ascii_lowercase();
    if joined.contains("input") || joined.contains("button") || joined.contains("key") {
        "onInputAction".to_string()
    } else if joined.contains("collision") || joined.contains("trigger") {
        "onCollision".to_string()
    } else if joined.contains("tick") || joined.contains("frame") {
        "onTick".to_string()
    } else {
        "onEvent".to_string()
    }
}

fn sorted_unique(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn is_hash_like(value: &str) -> bool {
    is_state_hash(value) || value.starts_with("sha256:") || value.starts_with("scenehash:")
}

fn is_state_hash(value: &str) -> bool {
    value.starts_with("fnv64:") && value.len() == "fnv64:".len() + 16
}

fn is_blocked_reference(reference: &str) -> bool {
    let lower = reference.to_ascii_lowercase();
    lower.contains("decompiled")
        || lower.contains("ilspy")
        || lower.contains("dnspy")
        || lower.contains("ripped")
        || lower.contains("shipped-build")
        || lower.contains("foreign-runtime")
        || lower.contains("live-bridge")
}

fn stable_id(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn stable_digest(text: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::legacy_logic_ingestion::{EngineCouplingKind, OracleStatus};

    fn unit() -> BehavioralUnitRecord {
        BehavioralUnitRecord {
            id: "unit.player-open-door".to_string(),
            name: "PlayerOpenDoor".to_string(),
            source_path: "Assets/Scripts/DoorController.cs".to_string(),
            provenance_node_ids: vec!["ir.method.door-controller.update".to_string()],
            stimuli: vec!["input action Open on fixed tick".to_string()],
            observed_outcomes: vec!["door_opened event emitted once".to_string()],
            engine_couplings: vec![EngineCouplingKind::Input],
            oracle_status: OracleStatus::Captured,
            fidelity_grade: FidelityGrade::Green,
            handoff_state: EraRHandoffState::Reexpress,
            ported_claim_allowed: false,
            gaps: Vec::new(),
        }
    }

    fn oracle(unit_id: &str, hash: &str) -> OracleSpec {
        OracleSpec {
            id: "oracle.player-open-door".to_string(),
            unit_id: unit_id.to_string(),
            stimulus: "frame 12 input Open".to_string(),
            expected_events: vec!["door_opened".to_string()],
            primary_state_hash: hash.to_string(),
            secondary_render_digest: None,
            tolerance: "bit-exact state hash".to_string(),
            provenance_refs: vec!["source-notes/intent.md".to_string()],
            status: CapturedOracleStatus::Captured,
            ported_claim_allowed: false,
        }
    }

    fn request(unit: BehavioralUnitRecord, oracle: Option<OracleSpec>) -> ReExpressionRequest {
        ReExpressionRequest {
            project_id: "fixture-project".to_string(),
            scene_path: "scenes/door.scene.json".to_string(),
            scene_hash: "sha256:0123456789abcdef".to_string(),
            target_dimensionality: ReExpressionTargetDimensionality::TwoD,
            units: vec![unit],
            oracle_specs: oracle.into_iter().collect(),
            skeleton_refs: vec!["Assets/Scenes/Door.unity".to_string()],
        }
    }

    #[test]
    fn captured_oracle_reexpresses_to_gated_behavior_draft_not_port_claim() {
        let unit = unit();
        let oracle = oracle(&unit.id, "fnv64:aaaaaaaaaaaaaaaa");
        let report = reexpress_deterministic_behaviors(&request(unit, Some(oracle))).unwrap();

        assert_eq!(
            report.schema_version,
            DETERMINISTIC_REEXPRESSION_SCHEMA_VERSION
        );
        assert_eq!(report.fidelity_report.green_count, 1);
        assert_eq!(report.fidelity_report.yellow_count, 0);
        assert_eq!(report.fidelity_report.red_count, 0);
        assert_eq!(report.behavior_drafts.len(), 1);
        assert_eq!(report.verification_handoffs.len(), 1);
        assert!(!report.plans[0].ported_claim_allowed);
        assert!(report.plans[0].gate_handoff.source_apply_required);
        assert!(report.plans[0].gate_handoff.rollback_required);
        assert!(!report.plans[0].gate_handoff.writes_artifacts_directly);
        assert_eq!(
            report.behavior_drafts[0].validation_status,
            BehaviorDraftValidationStatus::Drafted
        );
        assert!(report.behavior_drafts[0]
            .untrusted_boundary
            .contains("source-apply"));
    }

    #[test]
    fn missing_oracle_stays_yellow_and_emits_re_derivation_task() {
        let report = reexpress_deterministic_behaviors(&request(unit(), None)).unwrap();

        assert_eq!(report.fidelity_report.green_count, 0);
        assert_eq!(report.fidelity_report.yellow_count, 1);
        assert!(report.behavior_drafts.is_empty());
        assert!(report.verification_handoffs.is_empty());
        assert_eq!(
            report.re_derivation_tasks[0].task,
            "capture_or_repair_oracle_before_reexpression"
        );
        assert!(!report.plans[0].ported_claim_allowed);
    }

    #[test]
    fn blocked_or_port_claim_inputs_fail_red_without_laundering() {
        let mut unit = unit();
        unit.ported_claim_allowed = true;
        unit.oracle_status = OracleStatus::Passing;
        unit.fidelity_grade = FidelityGrade::Green;
        unit.handoff_state = EraRHandoffState::Reexpress;
        unit.engine_couplings.push(EngineCouplingKind::Physics);
        let oracle = oracle(&unit.id, "fnv64:bbbbbbbbbbbbbbbb");
        let mut req = request(unit, Some(oracle));
        req.target_dimensionality = ReExpressionTargetDimensionality::ThreeD;
        let report = reexpress_deterministic_behaviors(&req).unwrap();

        assert_eq!(report.fidelity_report.red_count, 1);
        assert!(report.behavior_drafts.is_empty());
        assert!(report.fidelity_report.blocked_or_unsupported[0].contains("ported claim"));
        assert!(!report.plans[0].ported_claim_allowed);
    }

    #[test]
    fn deterministic_digest_is_stable_and_changes_with_state_hash() {
        let first_unit = unit();
        let first = reexpress_deterministic_behaviors(&request(
            first_unit.clone(),
            Some(oracle(&first_unit.id, "fnv64:cccccccccccccccc")),
        ))
        .unwrap();
        let same_unit = unit();
        let same = reexpress_deterministic_behaviors(&request(
            same_unit.clone(),
            Some(oracle(&same_unit.id, "fnv64:cccccccccccccccc")),
        ))
        .unwrap();
        let changed_unit = unit();
        let changed = reexpress_deterministic_behaviors(&request(
            changed_unit.clone(),
            Some(oracle(&changed_unit.id, "fnv64:dddddddddddddddd")),
        ))
        .unwrap();

        assert_eq!(first.deterministic_digest, same.deterministic_digest);
        assert_ne!(first.deterministic_digest, changed.deterministic_digest);
    }

    #[test]
    fn blocked_skeleton_refs_are_rejected_before_generation() {
        let unit = unit();
        let oracle = oracle(&unit.id, "fnv64:eeeeeeeeeeeeeeee");
        let mut req = request(unit, Some(oracle));
        req.skeleton_refs = vec!["vendored_unity_runtime/PlayerLoop.dll".to_string()];
        let err = reexpress_deterministic_behaviors(&req)
            .unwrap_err()
            .to_string();
        assert!(err.contains("source-project/open-text"));
    }
}

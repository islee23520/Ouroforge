use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::deterministic_reexpression::ReExpressionTargetDimensionality;
use crate::legacy_logic_ingestion::{EraRHandoffState, FidelityGrade, ReDerivationTask};
use crate::loop_coverage_attribution::{
    validate_loop_coverage_attribution, LoopCoverageAttributionArtifact,
    LoopCoverageAttributionStatus, LoopCoverageProvenanceClass,
};

/// Era R M112 semantic-port coverage data shapes live in this module. The
/// request binds behavioral-unit records by ref, oracle refs, M111 differential
/// evidence refs, Rust-owned fidelity grades, residual re-derivation tasks, and
/// optional loop-coverage attribution artifacts. The report is an honest
/// progress/readiness ledger: it may say semantic coverage is complete, pending,
/// blocked, or human-feel escalated, but it never emits a finished-game
/// auto-port or fully-ported product claim.
pub const SEMANTIC_PORT_COVERAGE_SCHEMA_VERSION: &str = "semantic-port-coverage.v1";
pub const SEMANTIC_PORT_COVERAGE_BOUNDARY: &str = "Rust data-plane semantic re-derivation coverage over M111 verified evidence; one-way on-ramp; source-project/open-text clean-room refs only; no decompiled source copying; no source translation; no foreign runtime bridge; no finished-game auto-port or fully-ported claim; Ring 2 human-feel escalation remains human-owned; Studio has no trusted write authority";
pub const SEMANTIC_PORT_COVERAGE_DIGEST_ALGORITHM: &str = "fnv1a64";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticPortCoverageRequest {
    pub project_id: String,
    pub source_project_ref: String,
    pub target_dimensionality: ReExpressionTargetDimensionality,
    pub units: Vec<SemanticPortCoverageUnit>,
    pub convergence_policy: ConvergencePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticPortCoverageUnit {
    pub unit_id: String,
    pub behavioral_unit_ref: String,
    pub status: SemanticPortUnitStatus,
    pub fidelity_grade: FidelityGrade,
    #[serde(default)]
    pub oracle_ref: Option<String>,
    #[serde(default)]
    pub primary_state_hash: Option<String>,
    #[serde(default)]
    pub secondary_render_digest: Option<String>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub gap_summary: Vec<String>,
    #[serde(default)]
    pub re_derivation_tasks: Vec<ReDerivationTask>,
    #[serde(default)]
    pub loop_attribution: Option<LoopCoverageAttributionArtifact>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticPortUnitStatus {
    Verified,
    Pending,
    HumanFeelEscalated,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvergencePolicy {
    pub current_iteration: u32,
    pub max_iterations: u32,
    pub allow_ring2_human_escalation: bool,
    #[serde(default)]
    pub stop_when_all_verified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticPortConvergenceStatus {
    Passed,
    Continue,
    HumanEscalated,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticPortCoverageSummary {
    pub total_units: usize,
    pub verified_units: usize,
    pub pending_units: usize,
    pub human_feel_escalated_units: usize,
    pub blocked_units: usize,
    pub verified_basis_points: u16,
    pub semantic_coverage_complete: bool,
    pub fully_ported_claim_allowed: bool,
    pub ported_claim_allowed: bool,
    pub source_apply_gate_required: bool,
    pub studio_trusted_write_authority: bool,
    pub deterministic_state_hash_primary: bool,
    pub perceptual_render_secondary_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResidualBacklogItem {
    pub unit_id: String,
    pub status: SemanticPortUnitStatus,
    pub fidelity_grade: FidelityGrade,
    pub reasons: Vec<String>,
    pub next_handoff_state: EraRHandoffState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticPortCoverageReport {
    pub schema_version: String,
    pub boundary: String,
    pub deterministic_digest_algorithm: String,
    pub deterministic_digest: String,
    pub project_id: String,
    pub target_dimensionality: ReExpressionTargetDimensionality,
    pub summary: SemanticPortCoverageSummary,
    pub convergence_status: SemanticPortConvergenceStatus,
    pub convergence_terminated: bool,
    pub residual_backlog: Vec<ResidualBacklogItem>,
    pub re_derivation_tasks: Vec<ReDerivationTask>,
}

pub fn evaluate_semantic_port_coverage(
    request: &SemanticPortCoverageRequest,
) -> Result<SemanticPortCoverageReport> {
    validate_request(request)?;

    let total_units = request.units.len();
    let verified_units = request
        .units
        .iter()
        .filter(|unit| unit.status == SemanticPortUnitStatus::Verified)
        .count();
    let pending_units = request
        .units
        .iter()
        .filter(|unit| unit.status == SemanticPortUnitStatus::Pending)
        .count();
    let human_feel_escalated_units = request
        .units
        .iter()
        .filter(|unit| unit.status == SemanticPortUnitStatus::HumanFeelEscalated)
        .count();
    let blocked_units = request
        .units
        .iter()
        .filter(|unit| unit.status == SemanticPortUnitStatus::Blocked)
        .count();

    let mut residual_backlog = Vec::new();
    let mut re_derivation_tasks = Vec::new();
    for unit in &request.units {
        if unit.status != SemanticPortUnitStatus::Verified {
            let reasons = residual_reasons(unit);
            residual_backlog.push(ResidualBacklogItem {
                unit_id: unit.unit_id.clone(),
                status: unit.status,
                fidelity_grade: unit.fidelity_grade,
                reasons: reasons.clone(),
                next_handoff_state: next_handoff_state(unit.status),
            });
            if unit.re_derivation_tasks.is_empty() {
                re_derivation_tasks.push(ReDerivationTask {
                    unit_id: unit.unit_id.clone(),
                    task: residual_task(unit.status),
                    reason: reasons.join("; "),
                    handoff_state: next_handoff_state(unit.status),
                });
            } else {
                re_derivation_tasks.extend(unit.re_derivation_tasks.clone());
            }
        }
    }

    let semantic_coverage_complete = total_units > 0
        && verified_units == total_units
        && pending_units == 0
        && human_feel_escalated_units == 0
        && blocked_units == 0;
    let convergence_status = if blocked_units > 0 {
        SemanticPortConvergenceStatus::Blocked
    } else if human_feel_escalated_units > 0
        && request.convergence_policy.allow_ring2_human_escalation
    {
        SemanticPortConvergenceStatus::HumanEscalated
    } else if semantic_coverage_complete && request.convergence_policy.stop_when_all_verified {
        SemanticPortConvergenceStatus::Passed
    } else if request.convergence_policy.current_iteration
        >= request.convergence_policy.max_iterations
    {
        SemanticPortConvergenceStatus::Blocked
    } else {
        SemanticPortConvergenceStatus::Continue
    };
    let convergence_terminated = matches!(
        convergence_status,
        SemanticPortConvergenceStatus::Passed
            | SemanticPortConvergenceStatus::HumanEscalated
            | SemanticPortConvergenceStatus::Blocked
    );

    let summary = SemanticPortCoverageSummary {
        total_units,
        verified_units,
        pending_units,
        human_feel_escalated_units,
        blocked_units,
        verified_basis_points: ((verified_units * 10_000) / total_units) as u16,
        semantic_coverage_complete,
        fully_ported_claim_allowed: false,
        ported_claim_allowed: false,
        source_apply_gate_required: true,
        studio_trusted_write_authority: false,
        deterministic_state_hash_primary: true,
        perceptual_render_secondary_only: true,
    };

    let digest_payload = serde_json::json!({
        "projectId": request.project_id,
        "targetDimensionality": request.target_dimensionality,
        "unitEvidence": request
            .units
            .iter()
            .map(unit_digest_payload)
            .collect::<Vec<_>>(),
        "summary": summary,
        "convergenceStatus": convergence_status,
        "convergenceTerminated": convergence_terminated,
        "residualBacklog": residual_backlog,
        "tasks": re_derivation_tasks,
    });

    Ok(SemanticPortCoverageReport {
        schema_version: SEMANTIC_PORT_COVERAGE_SCHEMA_VERSION.to_string(),
        boundary: SEMANTIC_PORT_COVERAGE_BOUNDARY.to_string(),
        deterministic_digest_algorithm: SEMANTIC_PORT_COVERAGE_DIGEST_ALGORITHM.to_string(),
        deterministic_digest: stable_digest(&digest_payload.to_string()),
        project_id: request.project_id.clone(),
        target_dimensionality: request.target_dimensionality,
        summary,
        convergence_status,
        convergence_terminated,
        residual_backlog,
        re_derivation_tasks,
    })
}

fn validate_request(request: &SemanticPortCoverageRequest) -> Result<()> {
    if request.project_id.trim().is_empty() || request.units.is_empty() {
        return Err(anyhow!(
            "semantic-port coverage requires a project id and at least one unit"
        ));
    }
    validate_ref("sourceProjectRef", &request.source_project_ref)?;
    if request.convergence_policy.max_iterations == 0
        || request.convergence_policy.current_iteration > request.convergence_policy.max_iterations
    {
        return Err(anyhow!(
            "convergence policy requires 0 <= currentIteration <= maxIterations and maxIterations > 0"
        ));
    }
    let mut unit_ids = BTreeSet::new();
    for unit in &request.units {
        validate_unit(unit, request.target_dimensionality)?;
        if !unit_ids.insert(&unit.unit_id) {
            return Err(anyhow!("duplicate semantic-port unit `{}`", unit.unit_id));
        }
    }
    Ok(())
}

fn unit_digest_payload(unit: &SemanticPortCoverageUnit) -> serde_json::Value {
    serde_json::json!({
        "unitId": unit.unit_id,
        "behavioralUnitRef": unit.behavioral_unit_ref,
        "status": unit.status,
        "fidelityGrade": unit.fidelity_grade,
        "oracleRef": unit.oracle_ref,
        "primaryStateHash": unit.primary_state_hash,
        "secondaryRenderDigest": unit.secondary_render_digest,
        "evidenceRefs": unit.evidence_refs,
        "gapSummary": unit.gap_summary,
    })
}

fn validate_unit(
    unit: &SemanticPortCoverageUnit,
    dimensionality: ReExpressionTargetDimensionality,
) -> Result<()> {
    if unit.unit_id.trim().is_empty() {
        return Err(anyhow!("semantic-port unit id is required"));
    }
    validate_ref("behavioralUnitRef", &unit.behavioral_unit_ref)?;
    validate_refs("evidenceRefs", &unit.evidence_refs)?;
    if let Some(oracle_ref) = &unit.oracle_ref {
        validate_ref("oracleRef", oracle_ref)?;
    }
    for task in &unit.re_derivation_tasks {
        if task.unit_id != unit.unit_id
            || task.task.trim().is_empty()
            || task.reason.trim().is_empty()
        {
            return Err(anyhow!(
                "re-derivation tasks must target their semantic-port unit with task and reason"
            ));
        }
    }
    if let Some(attribution) = &unit.loop_attribution {
        let read_model = validate_loop_coverage_attribution(attribution)?;
        if read_model.status != LoopCoverageAttributionStatus::Classified
            || !matches!(
                read_model.provenance_class,
                Some(
                    LoopCoverageProvenanceClass::LoopProduced
                        | LoopCoverageProvenanceClass::LoopVerified
                )
            )
        {
            return Err(anyhow!(
                "semantic-port coverage requires loop-produced or loop-verified attribution when supplied"
            ));
        }
    }
    match unit.status {
        SemanticPortUnitStatus::Verified => {
            if unit.fidelity_grade != FidelityGrade::Green
                || unit.oracle_ref.is_none()
                || unit.evidence_refs.is_empty()
                || unit
                    .re_derivation_tasks
                    .iter()
                    .any(|task| task.handoff_state != EraRHandoffState::Verify)
                || !unit.gap_summary.is_empty()
                || !unit
                    .primary_state_hash
                    .as_deref()
                    .is_some_and(is_state_hash)
            {
                return Err(anyhow!(
                    "verified semantic-port units require Green fidelity, captured oracle ref, evidence refs, primary state hash, and no residual gaps"
                ));
            }
            if requires_render_corrob(dimensionality) && unit.secondary_render_digest.is_none() {
                return Err(anyhow!(
                    "2.5D/3D verified semantic-port units require secondary render corroboration"
                ));
            }
        }
        SemanticPortUnitStatus::Pending
        | SemanticPortUnitStatus::HumanFeelEscalated
        | SemanticPortUnitStatus::Blocked => {
            if unit.fidelity_grade == FidelityGrade::Green
                && unit.gap_summary.is_empty()
                && unit.re_derivation_tasks.is_empty()
            {
                return Err(anyhow!(
                    "non-verified semantic-port units require visible gaps or re-derivation tasks"
                ));
            }
        }
    }
    Ok(())
}

fn residual_reasons(unit: &SemanticPortCoverageUnit) -> Vec<String> {
    let mut reasons = unit.gap_summary.clone();
    for task in &unit.re_derivation_tasks {
        reasons.push(format!("{}: {}", task.task, task.reason));
    }
    if reasons.is_empty() {
        reasons.push(format!(
            "{:?} unit requires residual re-derivation",
            unit.status
        ));
    }
    sorted_unique(reasons)
}

fn residual_task(status: SemanticPortUnitStatus) -> String {
    match status {
        SemanticPortUnitStatus::Verified => "none".to_string(),
        SemanticPortUnitStatus::Pending => "continue_semantic_rederivation".to_string(),
        SemanticPortUnitStatus::HumanFeelEscalated => {
            "escalate_ring2_human_feel_review".to_string()
        }
        SemanticPortUnitStatus::Blocked => "block_semantic_port_convergence".to_string(),
    }
}

fn next_handoff_state(status: SemanticPortUnitStatus) -> EraRHandoffState {
    match status {
        SemanticPortUnitStatus::Verified => EraRHandoffState::Verify,
        SemanticPortUnitStatus::Pending => EraRHandoffState::Reexpress,
        SemanticPortUnitStatus::HumanFeelEscalated => EraRHandoffState::RejectOrDefer,
        SemanticPortUnitStatus::Blocked => EraRHandoffState::RejectOrDefer,
    }
}

fn requires_render_corrob(target: ReExpressionTargetDimensionality) -> bool {
    matches!(
        target,
        ReExpressionTargetDimensionality::TwoPointFiveD | ReExpressionTargetDimensionality::ThreeD
    )
}

fn is_state_hash(value: &str) -> bool {
    value.starts_with("fnv64:") && value.len() == "fnv64:".len() + 16
}

fn validate_refs(label: &str, refs: &[String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in refs {
        validate_ref(label, value)?;
        if !seen.insert(value) {
            return Err(anyhow!("{label} contains duplicate ref `{value}`"));
        }
    }
    Ok(())
}

fn validate_ref(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.starts_with('/')
        || value.contains("..")
        || value.contains('\\')
    {
        return Err(anyhow!("{label} must be a safe repo-relative ref"));
    }
    let lower = value.to_ascii_lowercase();
    if lower.contains("decompiled")
        || lower.contains("ilspy")
        || lower.contains("dnspy")
        || lower.contains("ripped")
        || lower.contains("shipped-build")
        || lower.contains("vendored_unity_runtime")
        || lower.contains("vendored_unreal_runtime")
        || lower.contains("foreign-runtime")
        || lower.contains("live-bridge")
    {
        return Err(anyhow!(
            "{label} must stay source-project/open-text clean-room and cannot reference decompiled/runtime bridge material"
        ));
    }
    Ok(())
}

fn sorted_unique(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
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
    use crate::loop_coverage_attribution::{
        LoopCoverageAttributionSignal, LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION,
    };

    fn attribution(unit: &str) -> LoopCoverageAttributionArtifact {
        LoopCoverageAttributionArtifact {
            schema_version: LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION.to_string(),
            artifact_ref: format!("evidence/m112/{unit}.json"),
            artifact_kind: "evidence-artifact".to_string(),
            status: LoopCoverageAttributionStatus::Classified,
            provenance_class: Some(LoopCoverageProvenanceClass::LoopVerified),
            source_signals: vec![LoopCoverageAttributionSignal {
                signal_kind: "m111-differential-verification".to_string(),
                source_ref: format!("evidence/m111/{unit}.json"),
                class_hint: Some(LoopCoverageProvenanceClass::LoopVerified),
                stale: false,
            }],
            evidence_refs: vec![format!("evidence/m111/{unit}.json")],
            verdict_refs: vec![format!("verdicts/m111/{unit}.json")],
            transaction_refs: Vec::new(),
            blocked_reasons: Vec::new(),
            boundary: "descriptive coverage, not quality, no auto-apply, read-only".to_string(),
        }
    }

    fn verified(unit: &str, hash: &str) -> SemanticPortCoverageUnit {
        SemanticPortCoverageUnit {
            unit_id: unit.to_string(),
            behavioral_unit_ref: format!("ir/m108/{unit}.json"),
            status: SemanticPortUnitStatus::Verified,
            fidelity_grade: FidelityGrade::Green,
            oracle_ref: Some(format!("oracles/m109/{unit}.json")),
            primary_state_hash: Some(hash.to_string()),
            secondary_render_digest: None,
            evidence_refs: vec![format!("evidence/m111/{unit}.json")],
            gap_summary: Vec::new(),
            re_derivation_tasks: Vec::new(),
            loop_attribution: Some(attribution(unit)),
        }
    }

    fn pending(unit: &str) -> SemanticPortCoverageUnit {
        SemanticPortCoverageUnit {
            unit_id: unit.to_string(),
            behavioral_unit_ref: format!("ir/m108/{unit}.json"),
            status: SemanticPortUnitStatus::Pending,
            fidelity_grade: FidelityGrade::Yellow,
            oracle_ref: None,
            primary_state_hash: None,
            secondary_render_digest: None,
            evidence_refs: vec![format!("evidence/m111/{unit}-gap.json")],
            gap_summary: vec!["oracle capture incomplete; outcome mismatch remains".to_string()],
            re_derivation_tasks: Vec::new(),
            loop_attribution: None,
        }
    }

    fn human(unit: &str) -> SemanticPortCoverageUnit {
        SemanticPortCoverageUnit {
            unit_id: unit.to_string(),
            behavioral_unit_ref: format!("ir/m108/{unit}.json"),
            status: SemanticPortUnitStatus::HumanFeelEscalated,
            fidelity_grade: FidelityGrade::Yellow,
            oracle_ref: Some(format!("oracles/m109/{unit}.json")),
            primary_state_hash: Some("fnv64:abababababababab".to_string()),
            secondary_render_digest: Some("ssim:0.990;pixel-diff:0.002".to_string()),
            evidence_refs: vec![format!("evidence/m111/{unit}.json")],
            gap_summary: vec!["particle feel requires Ring 2 human review".to_string()],
            re_derivation_tasks: Vec::new(),
            loop_attribution: None,
        }
    }

    fn request(units: Vec<SemanticPortCoverageUnit>) -> SemanticPortCoverageRequest {
        SemanticPortCoverageRequest {
            project_id: "semantic-port-m112".to_string(),
            source_project_ref: "source-projects/m112/source.project.json".to_string(),
            target_dimensionality: ReExpressionTargetDimensionality::TwoD,
            units,
            convergence_policy: ConvergencePolicy {
                current_iteration: 2,
                max_iterations: 4,
                allow_ring2_human_escalation: true,
                stop_when_all_verified: true,
            },
        }
    }

    #[test]
    fn all_verified_converges_without_port_claim() {
        let report = evaluate_semantic_port_coverage(&request(vec![verified(
            "unit.m112-door",
            "fnv64:aaaaaaaaaaaaaaaa",
        )]))
        .unwrap();
        assert_eq!(report.schema_version, SEMANTIC_PORT_COVERAGE_SCHEMA_VERSION);
        assert_eq!(
            report.convergence_status,
            SemanticPortConvergenceStatus::Passed
        );
        assert!(report.convergence_terminated);
        assert!(report.summary.semantic_coverage_complete);
        assert_eq!(report.summary.verified_basis_points, 10_000);
        assert!(!report.summary.fully_ported_claim_allowed);
        assert!(!report.summary.ported_claim_allowed);
        assert!(!report.summary.studio_trusted_write_authority);
        assert!(report.residual_backlog.is_empty());
    }

    #[test]
    fn pending_units_keep_residual_backlog_and_continue() {
        let report = evaluate_semantic_port_coverage(&request(vec![
            verified("unit.m112-door", "fnv64:bbbbbbbbbbbbbbbb"),
            pending("unit.m112-hazard"),
        ]))
        .unwrap();
        assert_eq!(
            report.convergence_status,
            SemanticPortConvergenceStatus::Continue
        );
        assert!(!report.summary.semantic_coverage_complete);
        assert_eq!(report.summary.verified_units, 1);
        assert_eq!(report.summary.pending_units, 1);
        assert_eq!(report.residual_backlog.len(), 1);
        assert_eq!(
            report.residual_backlog[0].next_handoff_state,
            EraRHandoffState::Reexpress
        );
        assert!(report.re_derivation_tasks[0]
            .reason
            .contains("outcome mismatch"));
    }

    #[test]
    fn human_feel_escalation_terminates_without_fully_ported_claim() {
        let mut door = verified("unit.m112-door", "fnv64:cccccccccccccccc");
        door.secondary_render_digest = Some("ssim:0.996;pixel-diff:0.001".to_string());
        let mut req = request(vec![door, human("unit.m112-particles")]);
        req.target_dimensionality = ReExpressionTargetDimensionality::ThreeD;
        let report = evaluate_semantic_port_coverage(&req).unwrap();
        assert_eq!(
            report.convergence_status,
            SemanticPortConvergenceStatus::HumanEscalated
        );
        assert!(report.convergence_terminated);
        assert_eq!(report.summary.human_feel_escalated_units, 1);
        assert!(!report.summary.fully_ported_claim_allowed);
        assert_eq!(
            report.residual_backlog[0].next_handoff_state,
            EraRHandoffState::RejectOrDefer
        );
    }

    #[test]
    fn verified_units_require_oracle_evidence_and_state_hash() {
        let mut unit = verified("unit.m112-door", "fnv64:dddddddddddddddd");
        unit.oracle_ref = None;
        let err = evaluate_semantic_port_coverage(&request(vec![unit]))
            .unwrap_err()
            .to_string();
        assert!(err.contains("captured oracle ref"));

        let unit = verified("unit.m112-door", "sha256:not-a-state-hash");
        let err = evaluate_semantic_port_coverage(&request(vec![unit]))
            .unwrap_err()
            .to_string();
        assert!(err.contains("primary state hash"));
    }

    #[test]
    fn blocked_refs_and_manual_attribution_fail_closed() {
        let mut unit = pending("unit.m112-hazard");
        unit.behavioral_unit_ref = "decompiled/Assembly-CSharp/Hazard.cs".to_string();
        let err = evaluate_semantic_port_coverage(&request(vec![unit]))
            .unwrap_err()
            .to_string();
        assert!(err.contains("source-project/open-text"));

        let mut unit = verified("unit.m112-door", "fnv64:eeeeeeeeeeeeeeee");
        let mut attr = attribution("unit.m112-door");
        attr.provenance_class = Some(LoopCoverageProvenanceClass::Manual);
        attr.source_signals[0].class_hint = Some(LoopCoverageProvenanceClass::Manual);
        unit.loop_attribution = Some(attr);
        let err = evaluate_semantic_port_coverage(&request(vec![unit]))
            .unwrap_err()
            .to_string();
        assert!(err.contains("loop-produced or loop-verified"));
    }

    #[test]
    fn digest_changes_when_residual_backlog_changes() {
        let first = evaluate_semantic_port_coverage(&request(vec![
            verified("unit.m112-door", "fnv64:ffffffffffffffff"),
            pending("unit.m112-hazard"),
        ]))
        .unwrap();
        let second = evaluate_semantic_port_coverage(&request(vec![verified(
            "unit.m112-door",
            "fnv64:ffffffffffffffff",
        )]))
        .unwrap();
        assert_ne!(first.deterministic_digest, second.deterministic_digest);
        assert_eq!(
            second.convergence_status,
            SemanticPortConvergenceStatus::Passed
        );
    }
}

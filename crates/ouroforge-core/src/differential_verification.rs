use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::deterministic_reexpression::{
    ReExpressionGateHandoff, ReExpressionTargetDimensionality, VerificationHandoff,
};
use crate::legacy_logic_ingestion::{EraRHandoffState, FidelityGrade, ReDerivationTask};
use crate::tacit_oracle_capture::{CapturedOracleStatus, OracleSpec};

/// Era R M111 behavioral A/B verification data shapes live in this module:
/// `DifferentialVerificationRequest` binds captured oracle records from
/// `tacit_oracle_capture`, deterministic re-expression handoffs from
/// `deterministic_reexpression`, native observations, and source-project
/// evidence refs; `DifferentialVerificationReport` emits the comparison result,
/// fidelity verdict, rollback/re-derivation tasks, and an optional downstream
/// M112 handoff. The comparison is outcome-level and clean-room: it never
/// accepts decompiled-source refs, foreign runtime bridges, or trusted Studio
/// writes, and it never allows a "ported" claim.
pub const DIFFERENTIAL_VERIFICATION_SCHEMA_VERSION: &str = "differential.verification.v1";
pub const DIFFERENTIAL_VERIFICATION_BOUNDARY: &str = "Rust data-plane behavioral A/B verification over captured oracle evidence and Ouroforge-native re-expression candidates; one-way on-ramp; source-project/open-text clean-room inputs only; no decompiled source copying; no source translation; no foreign runtime bridge; source-apply/review gates remain required for writes; no finished-game auto-port claim";
pub const DIFFERENTIAL_VERIFICATION_DIGEST_ALGORITHM: &str = "fnv1a64";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferentialVerificationRequest {
    pub unit_id: String,
    pub candidate_id: String,
    pub target_dimensionality: ReExpressionTargetDimensionality,
    pub oracle: OracleSpec,
    pub verification_handoff: VerificationHandoff,
    pub gate_handoff: ReExpressionGateHandoff,
    pub native_observation: NativeBehaviorObservation,
    #[serde(default)]
    pub baseline_evidence_refs: Vec<String>,
    #[serde(default)]
    pub native_evidence_refs: Vec<String>,
    #[serde(default)]
    pub source_ir_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeBehaviorObservation {
    pub stimulus: String,
    pub observed_events: Vec<String>,
    pub primary_state_hash: String,
    pub secondary_render_digest: Option<String>,
    #[serde(default)]
    pub nondeterminism_notes: Vec<String>,
    #[serde(default)]
    pub rollback_evidence_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DifferentialVerificationStatus {
    Passed,
    NeedsRepair,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonStatus {
    Match,
    Mismatch,
    Inconclusive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BehavioralAbResult {
    pub unit_id: String,
    pub stimulus: String,
    pub expected_events: Vec<String>,
    pub observed_events: Vec<String>,
    pub event_status: ComparisonStatus,
    pub expected_primary_state_hash: String,
    pub observed_primary_state_hash: String,
    pub state_hash_status: ComparisonStatus,
    pub expected_secondary_render_digest: Option<String>,
    pub observed_secondary_render_digest: Option<String>,
    pub render_status: ComparisonStatus,
    pub rollback_required: bool,
    pub rollback_evidence_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferentialFidelityReport {
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub fidelity_grade: FidelityGrade,
    pub oracle_status: CapturedOracleStatus,
    pub ported_claim_allowed: bool,
    pub source_apply_gate_required: bool,
    pub studio_trusted_write_authority: bool,
    pub deterministic_state_hash_primary: bool,
    pub perceptual_render_secondary_only: bool,
    pub gap_summary: Vec<String>,
    pub blocked_or_failed: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticPortHandoff {
    pub unit_id: String,
    pub candidate_id: String,
    pub downstream_milestone: String,
    pub evidence_refs: Vec<String>,
    pub primary_state_hash: String,
    pub secondary_render_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferentialVerificationReport {
    pub schema_version: String,
    pub boundary: String,
    pub deterministic_digest_algorithm: String,
    pub deterministic_digest: String,
    pub status: DifferentialVerificationStatus,
    pub unit_id: String,
    pub candidate_id: String,
    pub target_dimensionality: ReExpressionTargetDimensionality,
    pub ab_result: BehavioralAbResult,
    pub fidelity_report: DifferentialFidelityReport,
    pub semantic_port_handoff: Option<SemanticPortHandoff>,
    pub re_derivation_tasks: Vec<ReDerivationTask>,
}

pub fn verify_behavioral_ab(
    request: &DifferentialVerificationRequest,
) -> Result<DifferentialVerificationReport> {
    validate_request(request)?;

    let event_status = compare_events(
        &request.oracle.expected_events,
        &request.native_observation.observed_events,
    );
    let state_hash_status =
        if request.oracle.primary_state_hash == request.native_observation.primary_state_hash {
            ComparisonStatus::Match
        } else {
            ComparisonStatus::Mismatch
        };
    let render_status = compare_render(request);

    let mut gaps = BTreeSet::new();
    let mut blocked = BTreeSet::new();
    if event_status != ComparisonStatus::Match {
        gaps.insert("observed native events do not match captured oracle events".to_string());
    }
    if state_hash_status != ComparisonStatus::Match {
        gaps.insert("primary deterministic state hash differs from captured oracle".to_string());
    }
    if render_status == ComparisonStatus::Mismatch {
        gaps.insert("secondary render digest differs from captured oracle tolerance".to_string());
    }
    if render_status == ComparisonStatus::Inconclusive {
        gaps.insert(
            "secondary render corroboration is missing for 2.5D/3D verification".to_string(),
        );
    }
    for note in &request.native_observation.nondeterminism_notes {
        gaps.insert(format!("native nondeterminism note: {note}"));
    }

    let status = if event_status == ComparisonStatus::Match
        && state_hash_status == ComparisonStatus::Match
        && render_status != ComparisonStatus::Mismatch
        && !(requires_render_corrob(request.target_dimensionality)
            && render_status == ComparisonStatus::Inconclusive)
    {
        DifferentialVerificationStatus::Passed
    } else if state_hash_status == ComparisonStatus::Mismatch {
        blocked.insert("state-hash mismatch blocks differential verification".to_string());
        DifferentialVerificationStatus::Failed
    } else {
        DifferentialVerificationStatus::NeedsRepair
    };

    let fidelity_grade = match status {
        DifferentialVerificationStatus::Passed => FidelityGrade::Green,
        DifferentialVerificationStatus::NeedsRepair => FidelityGrade::Yellow,
        DifferentialVerificationStatus::Failed => FidelityGrade::Red,
    };

    let rollback_required = status != DifferentialVerificationStatus::Passed;
    if rollback_required && request.native_observation.rollback_evidence_ref.is_none() {
        gaps.insert("rollback evidence is required for mismatched candidate".to_string());
    }

    let ab_result = BehavioralAbResult {
        unit_id: request.unit_id.clone(),
        stimulus: request.oracle.stimulus.clone(),
        expected_events: request.oracle.expected_events.clone(),
        observed_events: request.native_observation.observed_events.clone(),
        event_status,
        expected_primary_state_hash: request.oracle.primary_state_hash.clone(),
        observed_primary_state_hash: request.native_observation.primary_state_hash.clone(),
        state_hash_status,
        expected_secondary_render_digest: request.oracle.secondary_render_digest.clone(),
        observed_secondary_render_digest: request
            .native_observation
            .secondary_render_digest
            .clone(),
        render_status,
        rollback_required,
        rollback_evidence_ref: request.native_observation.rollback_evidence_ref.clone(),
    };

    let evidence_refs = sorted_unique(
        request
            .baseline_evidence_refs
            .iter()
            .chain(request.native_evidence_refs.iter())
            .cloned()
            .collect(),
    );
    let semantic_port_handoff = if status == DifferentialVerificationStatus::Passed {
        Some(SemanticPortHandoff {
            unit_id: request.unit_id.clone(),
            candidate_id: request.candidate_id.clone(),
            downstream_milestone: "Era R M112 semantic-port coverage + convergence".to_string(),
            evidence_refs: evidence_refs.clone(),
            primary_state_hash: request.native_observation.primary_state_hash.clone(),
            secondary_render_digest: request.native_observation.secondary_render_digest.clone(),
        })
    } else {
        None
    };

    let re_derivation_tasks = if status == DifferentialVerificationStatus::Passed {
        Vec::new()
    } else {
        vec![ReDerivationTask {
            unit_id: request.unit_id.clone(),
            task: if status == DifferentialVerificationStatus::Failed {
                "rollback_and_reject_differential_mismatch".to_string()
            } else {
                "repair_reexpression_or_oracle_before_differential_verification".to_string()
            },
            reason: sorted_unique(gaps.iter().cloned().collect()).join("; "),
            handoff_state: if status == DifferentialVerificationStatus::Failed {
                EraRHandoffState::RejectOrDefer
            } else {
                EraRHandoffState::Reexpress
            },
        }]
    };

    let fidelity_report = DifferentialFidelityReport {
        green_count: usize::from(fidelity_grade == FidelityGrade::Green),
        yellow_count: usize::from(fidelity_grade == FidelityGrade::Yellow),
        red_count: usize::from(fidelity_grade == FidelityGrade::Red),
        fidelity_grade,
        oracle_status: request.oracle.status,
        ported_claim_allowed: false,
        source_apply_gate_required: true,
        studio_trusted_write_authority: false,
        deterministic_state_hash_primary: true,
        perceptual_render_secondary_only: true,
        gap_summary: sorted_unique(gaps.into_iter().collect()),
        blocked_or_failed: sorted_unique(blocked.into_iter().collect()),
    };

    let digest_payload = serde_json::json!({
        "unitId": request.unit_id,
        "candidateId": request.candidate_id,
        "dimensionality": request.target_dimensionality,
        "abResult": ab_result,
        "fidelity": fidelity_report,
        "handoff": semantic_port_handoff,
        "tasks": re_derivation_tasks,
    });

    Ok(DifferentialVerificationReport {
        schema_version: DIFFERENTIAL_VERIFICATION_SCHEMA_VERSION.to_string(),
        boundary: DIFFERENTIAL_VERIFICATION_BOUNDARY.to_string(),
        deterministic_digest_algorithm: DIFFERENTIAL_VERIFICATION_DIGEST_ALGORITHM.to_string(),
        deterministic_digest: stable_digest(&digest_payload.to_string()),
        status,
        unit_id: request.unit_id.clone(),
        candidate_id: request.candidate_id.clone(),
        target_dimensionality: request.target_dimensionality,
        ab_result,
        fidelity_report,
        semantic_port_handoff,
        re_derivation_tasks,
    })
}

fn validate_request(request: &DifferentialVerificationRequest) -> Result<()> {
    if request.unit_id.trim().is_empty() || request.candidate_id.trim().is_empty() {
        return Err(anyhow!(
            "differential verification requires unit and candidate ids"
        ));
    }
    if request.oracle.unit_id != request.unit_id
        || request.verification_handoff.unit_id != request.unit_id
    {
        return Err(anyhow!("oracle and handoff must target the verified unit"));
    }
    if !matches!(
        request.oracle.status,
        CapturedOracleStatus::Captured | CapturedOracleStatus::Passing
    ) || request.oracle.ported_claim_allowed
    {
        return Err(anyhow!(
            "captured non-port oracle is required before differential verification"
        ));
    }
    if !is_state_hash(&request.oracle.primary_state_hash)
        || !is_state_hash(&request.native_observation.primary_state_hash)
        || request.verification_handoff.primary_state_hash != request.oracle.primary_state_hash
    {
        return Err(anyhow!(
            "primary deterministic state hashes are required and must bind to the oracle handoff"
        ));
    }
    if !request.gate_handoff.source_apply_required
        || !request.gate_handoff.review_gate_required
        || !request.gate_handoff.rollback_required
        || request.gate_handoff.writes_artifacts_directly
        || request.gate_handoff.trusted_write_authority
    {
        return Err(anyhow!(
            "source-apply/review/rollback gates are required and trusted writes are forbidden"
        ));
    }
    let refs = request
        .baseline_evidence_refs
        .iter()
        .chain(request.native_evidence_refs.iter())
        .chain(request.source_ir_refs.iter())
        .chain(request.oracle.provenance_refs.iter());
    for reference in refs {
        if !is_safe_ref(reference) {
            return Err(anyhow!("differential verification refs must be repo-relative source-project/open-text clean-room refs"));
        }
    }
    Ok(())
}

fn compare_events(expected: &[String], observed: &[String]) -> ComparisonStatus {
    if expected.is_empty() || observed.is_empty() {
        return ComparisonStatus::Inconclusive;
    }
    let expected_set = expected.iter().collect::<BTreeSet<_>>();
    let observed_set = observed.iter().collect::<BTreeSet<_>>();
    if expected_set == observed_set {
        ComparisonStatus::Match
    } else {
        ComparisonStatus::Mismatch
    }
}

fn compare_render(request: &DifferentialVerificationRequest) -> ComparisonStatus {
    match (
        request.oracle.secondary_render_digest.as_deref(),
        request
            .native_observation
            .secondary_render_digest
            .as_deref(),
    ) {
        (Some(expected), Some(observed)) if expected == observed => ComparisonStatus::Match,
        (Some(_), Some(_)) => ComparisonStatus::Mismatch,
        (None, None) if requires_render_corrob(request.target_dimensionality) => {
            ComparisonStatus::Inconclusive
        }
        (None, None) => ComparisonStatus::Match,
        _ => ComparisonStatus::Inconclusive,
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

fn is_safe_ref(reference: &str) -> bool {
    if reference.trim().is_empty()
        || reference.starts_with('/')
        || reference.contains("..")
        || reference.contains('\\')
    {
        return false;
    }
    let lower = reference.to_ascii_lowercase();
    !(lower.contains("decompiled")
        || lower.contains("ilspy")
        || lower.contains("dnspy")
        || lower.contains("ripped")
        || lower.contains("shipped-build")
        || lower.contains("vendored_unity_runtime")
        || lower.contains("vendored_unreal_runtime")
        || lower.contains("foreign-runtime")
        || lower.contains("live-bridge"))
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
    use crate::deterministic_reexpression::ReExpressionGateHandoff;

    fn oracle(hash: &str) -> OracleSpec {
        OracleSpec {
            id: "oracle.unit.v111".to_string(),
            unit_id: "unit.v111".to_string(),
            stimulus: "frame 20 input Open".to_string(),
            expected_events: vec!["door_opened".to_string(), "score_incremented".to_string()],
            primary_state_hash: hash.to_string(),
            secondary_render_digest: None,
            tolerance: "2D bit-exact state hash".to_string(),
            provenance_refs: vec!["source-notes/v111-intent.md".to_string()],
            status: CapturedOracleStatus::Captured,
            ported_claim_allowed: false,
        }
    }

    fn handoff(hash: &str) -> VerificationHandoff {
        VerificationHandoff {
            unit_id: "unit.v111".to_string(),
            oracle_ref: "oracle.unit.v111".to_string(),
            primary_state_hash: hash.to_string(),
            secondary_render_digest: None,
            verification_rule: "2D bit-exact state hash must match captured oracle".to_string(),
            downstream_milestone: "Era R M111 differential verification A/B".to_string(),
        }
    }

    fn gates() -> ReExpressionGateHandoff {
        ReExpressionGateHandoff {
            source_apply_required: true,
            review_gate_required: true,
            rollback_required: true,
            writes_artifacts_directly: false,
            trusted_write_authority: false,
            provenance_refs: vec!["source-notes/v111-intent.md".to_string()],
        }
    }

    fn request(
        hash: &str,
        observed_events: Vec<&str>,
        observed_hash: &str,
    ) -> DifferentialVerificationRequest {
        DifferentialVerificationRequest {
            unit_id: "unit.v111".to_string(),
            candidate_id: "draft.reexpr.unit-v111".to_string(),
            target_dimensionality: ReExpressionTargetDimensionality::TwoD,
            oracle: oracle(hash),
            verification_handoff: handoff(hash),
            gate_handoff: gates(),
            native_observation: NativeBehaviorObservation {
                stimulus: "frame 20 input Open".to_string(),
                observed_events: observed_events.into_iter().map(str::to_string).collect(),
                primary_state_hash: observed_hash.to_string(),
                secondary_render_digest: None,
                nondeterminism_notes: Vec::new(),
                rollback_evidence_ref: Some("evidence/rollback/unit-v111.json".to_string()),
            },
            baseline_evidence_refs: vec!["evidence/baseline/unit-v111.json".to_string()],
            native_evidence_refs: vec!["evidence/native/unit-v111.json".to_string()],
            source_ir_refs: vec!["ir/m108/unit-v111.json".to_string()],
        }
    }

    #[test]
    fn passing_oracle_and_native_outcome_emits_green_handoff_without_port_claim() {
        let report = verify_behavioral_ab(&request(
            "fnv64:aaaaaaaaaaaaaaaa",
            vec!["score_incremented", "door_opened"],
            "fnv64:aaaaaaaaaaaaaaaa",
        ))
        .unwrap();
        assert_eq!(report.status, DifferentialVerificationStatus::Passed);
        assert_eq!(report.fidelity_report.fidelity_grade, FidelityGrade::Green);
        assert_eq!(report.fidelity_report.green_count, 1);
        assert!(!report.fidelity_report.ported_claim_allowed);
        assert!(!report.fidelity_report.studio_trusted_write_authority);
        assert_eq!(report.ab_result.event_status, ComparisonStatus::Match);
        assert_eq!(report.ab_result.state_hash_status, ComparisonStatus::Match);
        assert!(report.semantic_port_handoff.is_some());
        assert!(report.re_derivation_tasks.is_empty());
    }

    #[test]
    fn event_mismatch_is_yellow_repair_with_rollback_evidence() {
        let report = verify_behavioral_ab(&request(
            "fnv64:bbbbbbbbbbbbbbbb",
            vec!["door_opened"],
            "fnv64:bbbbbbbbbbbbbbbb",
        ))
        .unwrap();
        assert_eq!(report.status, DifferentialVerificationStatus::NeedsRepair);
        assert_eq!(report.fidelity_report.fidelity_grade, FidelityGrade::Yellow);
        assert_eq!(report.ab_result.event_status, ComparisonStatus::Mismatch);
        assert!(report.ab_result.rollback_required);
        assert!(report.semantic_port_handoff.is_none());
        assert_eq!(
            report.re_derivation_tasks[0].handoff_state,
            EraRHandoffState::Reexpress
        );
    }

    #[test]
    fn state_hash_mismatch_is_red_and_changes_digest() {
        let first = verify_behavioral_ab(&request(
            "fnv64:cccccccccccccccc",
            vec!["door_opened", "score_incremented"],
            "fnv64:cccccccccccccccc",
        ))
        .unwrap();
        let failed = verify_behavioral_ab(&request(
            "fnv64:cccccccccccccccc",
            vec!["door_opened", "score_incremented"],
            "fnv64:dddddddddddddddd",
        ))
        .unwrap();
        assert_eq!(failed.status, DifferentialVerificationStatus::Failed);
        assert_eq!(failed.fidelity_report.fidelity_grade, FidelityGrade::Red);
        assert_eq!(
            failed.ab_result.state_hash_status,
            ComparisonStatus::Mismatch
        );
        assert!(!failed.fidelity_report.blocked_or_failed.is_empty());
        assert_ne!(first.deterministic_digest, failed.deterministic_digest);
    }

    #[test]
    fn three_d_requires_state_hash_primary_and_render_secondary() {
        let mut req = request(
            "fnv64:eeeeeeeeeeeeeeee",
            vec!["door_opened", "score_incremented"],
            "fnv64:eeeeeeeeeeeeeeee",
        );
        req.target_dimensionality = ReExpressionTargetDimensionality::ThreeD;
        req.oracle.secondary_render_digest = Some("ssim:0.993;pixel-diff:0.001".to_string());
        req.verification_handoff.secondary_render_digest =
            req.oracle.secondary_render_digest.clone();
        req.native_observation.secondary_render_digest = req.oracle.secondary_render_digest.clone();
        let report = verify_behavioral_ab(&req).unwrap();
        assert_eq!(report.status, DifferentialVerificationStatus::Passed);
        assert_eq!(report.ab_result.render_status, ComparisonStatus::Match);
        assert!(report.fidelity_report.deterministic_state_hash_primary);
        assert!(report.fidelity_report.perceptual_render_secondary_only);
    }

    #[test]
    fn blocked_refs_and_trusted_writes_fail_preflight() {
        let mut req = request(
            "fnv64:ffffffffffffffff",
            vec!["door_opened", "score_incremented"],
            "fnv64:ffffffffffffffff",
        );
        req.source_ir_refs = vec!["decompiled/Assembly-CSharp.dll".to_string()];
        let err = verify_behavioral_ab(&req).unwrap_err().to_string();
        assert!(err.contains("source-project/open-text"));

        let mut req = request(
            "fnv64:1111111111111111",
            vec!["door_opened", "score_incremented"],
            "fnv64:1111111111111111",
        );
        req.gate_handoff.trusted_write_authority = true;
        let err = verify_behavioral_ab(&req).unwrap_err().to_string();
        assert!(err.contains("trusted writes are forbidden"));
    }
}

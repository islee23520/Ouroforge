//! Scenario Coverage v93 regression suite for #2234 / Era R M111.

use std::path::{Path, PathBuf};

use ouroforge_core::deterministic_reexpression::{
    ReExpressionGateHandoff, ReExpressionTargetDimensionality, VerificationHandoff,
};
use ouroforge_core::differential_verification::{
    verify_behavioral_ab, ComparisonStatus, DifferentialVerificationRequest,
    DifferentialVerificationStatus, NativeBehaviorObservation,
    DIFFERENTIAL_VERIFICATION_SCHEMA_VERSION,
};
use ouroforge_core::legacy_logic_ingestion::{EraRHandoffState, FidelityGrade};
use ouroforge_core::tacit_oracle_capture::{CapturedOracleStatus, OracleSpec};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn oracle(hash: &str, secondary: Option<&str>) -> OracleSpec {
    OracleSpec {
        id: "oracle.unit.v93".to_string(),
        unit_id: "unit.v93".to_string(),
        stimulus: "fixed tick 42 input Open".to_string(),
        expected_events: vec!["door_opened".to_string(), "score_incremented".to_string()],
        primary_state_hash: hash.to_string(),
        secondary_render_digest: secondary.map(str::to_string),
        tolerance: if secondary.is_some() {
            "state-hash primary; SSIM/pixel-diff render secondary".to_string()
        } else {
            "2D bit-exact state hash".to_string()
        },
        provenance_refs: vec!["source-notes/v93-clean-room-intent.md".to_string()],
        status: CapturedOracleStatus::Captured,
        ported_claim_allowed: false,
    }
}

fn handoff(hash: &str, secondary: Option<&str>) -> VerificationHandoff {
    VerificationHandoff {
        unit_id: "unit.v93".to_string(),
        oracle_ref: "oracle.unit.v93".to_string(),
        primary_state_hash: hash.to_string(),
        secondary_render_digest: secondary.map(str::to_string),
        verification_rule: if secondary.is_some() {
            "3D state-hash primary with SSIM/pixel-diff secondary corroboration".to_string()
        } else {
            "2D bit-exact state hash must match captured oracle".to_string()
        },
        downstream_milestone: "Era R M111 differential verification A/B".to_string(),
    }
}

fn gate_handoff() -> ReExpressionGateHandoff {
    ReExpressionGateHandoff {
        source_apply_required: true,
        review_gate_required: true,
        rollback_required: true,
        writes_artifacts_directly: false,
        trusted_write_authority: false,
        provenance_refs: vec!["source-notes/v93-clean-room-intent.md".to_string()],
    }
}

fn observation(
    events: Vec<&str>,
    hash: &str,
    secondary: Option<&str>,
) -> NativeBehaviorObservation {
    NativeBehaviorObservation {
        stimulus: "fixed tick 42 input Open".to_string(),
        observed_events: events.into_iter().map(str::to_string).collect(),
        primary_state_hash: hash.to_string(),
        secondary_render_digest: secondary.map(str::to_string),
        nondeterminism_notes: Vec::new(),
        rollback_evidence_ref: Some("evidence/v93/rollback.json".to_string()),
    }
}

fn request(
    dimensionality: ReExpressionTargetDimensionality,
    oracle_hash: &str,
    oracle_secondary: Option<&str>,
    native_events: Vec<&str>,
    native_hash: &str,
    native_secondary: Option<&str>,
) -> DifferentialVerificationRequest {
    DifferentialVerificationRequest {
        unit_id: "unit.v93".to_string(),
        candidate_id: "draft.reexpr.unit-v93".to_string(),
        target_dimensionality: dimensionality,
        oracle: oracle(oracle_hash, oracle_secondary),
        verification_handoff: handoff(oracle_hash, oracle_secondary),
        gate_handoff: gate_handoff(),
        native_observation: observation(native_events, native_hash, native_secondary),
        baseline_evidence_refs: vec!["evidence/v93/oracle-baseline.json".to_string()],
        native_evidence_refs: vec!["evidence/v93/native-observation.json".to_string()],
        source_ir_refs: vec!["ir/v93/unit.json".to_string()],
    }
}

#[test]
fn v93_matrix_records_rows_and_boundaries() {
    let matrix = read_json(
        "examples/differential-verification-behavioral-ab-v1/scenario-coverage-v93/matrix.fixture.json",
    );
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v93-differential-verification-v1"
    );
    assert_eq!(matrix["coverageVersion"], 93);
    assert_eq!(matrix["issueRef"], "#2234");

    let rows = matrix["rows"].as_array().unwrap();
    for required in [
        "v93.oracle-and-native-observation-pass-without-port-claim",
        "v93.lossy-outcome-mismatch-not-graded-clean",
        "v93.no-auto-port-without-oracle-fails-preflight",
        "v93.ungated-auto-translated-port-fails",
        "v93.deterministic-state-hash-break-fails",
        "v93.3d-state-hash-primary-render-secondary",
        "v93.clean-room-source-only-and-no-studio-trusted-write",
    ] {
        assert!(
            rows.iter().any(|row| row["id"] == required),
            "missing {required}"
        );
    }
    for row in rows {
        assert_eq!(row["status"], "pass");
        assert!(repo_root()
            .join(row["evidenceRef"].as_str().unwrap())
            .exists());
    }
    let invariants = &matrix["invariants"];
    assert_eq!(invariants["autoPortWithoutOracleAllowed"], false);
    assert_eq!(invariants["lossyImportMayGradeClean"], false);
    assert_eq!(invariants["ungatedAutoTranslatedPortAllowed"], false);
    assert_eq!(invariants["deterministicStateHashRequired"], true);
    assert_eq!(invariants["perceptualRenderSecondaryOnly"], true);
    assert_eq!(invariants["studioTrustedWriteAuthority"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
}

#[test]
fn v93_oracle_and_native_observation_pass_without_port_claim() {
    let report = verify_behavioral_ab(&request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:aaaaaaaaaaaaaaaa",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:aaaaaaaaaaaaaaaa",
        None,
    ))
    .unwrap();

    assert_eq!(
        report.schema_version,
        DIFFERENTIAL_VERIFICATION_SCHEMA_VERSION
    );
    assert_eq!(report.status, DifferentialVerificationStatus::Passed);
    assert_eq!(report.fidelity_report.fidelity_grade, FidelityGrade::Green);
    assert_eq!(report.fidelity_report.green_count, 1);
    assert_eq!(report.fidelity_report.yellow_count, 0);
    assert_eq!(report.fidelity_report.red_count, 0);
    assert_eq!(report.ab_result.event_status, ComparisonStatus::Match);
    assert_eq!(report.ab_result.state_hash_status, ComparisonStatus::Match);
    assert!(!report.fidelity_report.ported_claim_allowed);
    assert!(report.fidelity_report.source_apply_gate_required);
    assert!(!report.fidelity_report.studio_trusted_write_authority);
    assert!(report.semantic_port_handoff.is_some());
    assert!(report.re_derivation_tasks.is_empty());
}

#[test]
fn v93_lossy_outcome_mismatch_not_graded_clean() {
    let report = verify_behavioral_ab(&request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:bbbbbbbbbbbbbbbb",
        None,
        vec!["door_opened"],
        "fnv64:bbbbbbbbbbbbbbbb",
        None,
    ))
    .unwrap();

    assert_eq!(report.status, DifferentialVerificationStatus::NeedsRepair);
    assert_eq!(report.fidelity_report.fidelity_grade, FidelityGrade::Yellow);
    assert_eq!(report.fidelity_report.green_count, 0);
    assert_eq!(report.fidelity_report.yellow_count, 1);
    assert_eq!(report.fidelity_report.red_count, 0);
    assert_eq!(report.ab_result.event_status, ComparisonStatus::Mismatch);
    assert!(report.ab_result.rollback_required);
    assert!(report.semantic_port_handoff.is_none());
    assert!(report
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("observed native events")));
    assert_eq!(
        report.re_derivation_tasks[0].handoff_state,
        EraRHandoffState::Reexpress
    );
}

#[test]
fn v93_no_auto_port_without_oracle_fails_preflight() {
    let mut req = request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:cccccccccccccccc",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:cccccccccccccccc",
        None,
    );
    req.oracle.status = CapturedOracleStatus::Missing;
    let err = verify_behavioral_ab(&req).unwrap_err().to_string();
    assert!(err.contains("captured non-port oracle"));

    let mut req = request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:dddddddddddddddd",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:dddddddddddddddd",
        None,
    );
    req.oracle.ported_claim_allowed = true;
    let err = verify_behavioral_ab(&req).unwrap_err().to_string();
    assert!(err.contains("captured non-port oracle"));
}

#[test]
fn v93_ungated_auto_translated_port_fails_preflight() {
    let mut req = request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:eeeeeeeeeeeeeeee",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:eeeeeeeeeeeeeeee",
        None,
    );
    req.gate_handoff.source_apply_required = false;
    let err = verify_behavioral_ab(&req).unwrap_err().to_string();
    assert!(err.contains("source-apply/review/rollback gates"));

    let mut req = request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:ffffffffffffffff",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:ffffffffffffffff",
        None,
    );
    req.gate_handoff.trusted_write_authority = true;
    let err = verify_behavioral_ab(&req).unwrap_err().to_string();
    assert!(err.contains("trusted writes are forbidden"));

    let mut req = request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:1111111111111111",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:1111111111111111",
        None,
    );
    req.source_ir_refs = vec!["decompiled/Assembly-CSharp/DoorController.cs".to_string()];
    let err = verify_behavioral_ab(&req).unwrap_err().to_string();
    assert!(err.contains("source-project/open-text"));
}

#[test]
fn v93_deterministic_state_hash_break_fails() {
    let passing = verify_behavioral_ab(&request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:2222222222222222",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:2222222222222222",
        None,
    ))
    .unwrap();
    let failed = verify_behavioral_ab(&request(
        ReExpressionTargetDimensionality::TwoD,
        "fnv64:2222222222222222",
        None,
        vec!["score_incremented", "door_opened"],
        "fnv64:3333333333333333",
        None,
    ))
    .unwrap();

    assert_eq!(failed.status, DifferentialVerificationStatus::Failed);
    assert_eq!(failed.fidelity_report.fidelity_grade, FidelityGrade::Red);
    assert_eq!(failed.fidelity_report.red_count, 1);
    assert_eq!(
        failed.ab_result.state_hash_status,
        ComparisonStatus::Mismatch
    );
    assert!(failed.semantic_port_handoff.is_none());
    assert!(failed
        .fidelity_report
        .blocked_or_failed
        .iter()
        .any(|blocked| blocked.contains("state-hash mismatch")));
    assert_ne!(passing.deterministic_digest, failed.deterministic_digest);
}

#[test]
fn v93_3d_state_hash_primary_render_secondary() {
    let report = verify_behavioral_ab(&request(
        ReExpressionTargetDimensionality::ThreeD,
        "fnv64:4444444444444444",
        Some("ssim:0.994;pixel-diff:0.001"),
        vec!["score_incremented", "door_opened"],
        "fnv64:4444444444444444",
        Some("ssim:0.994;pixel-diff:0.001"),
    ))
    .unwrap();
    assert_eq!(report.status, DifferentialVerificationStatus::Passed);
    assert_eq!(report.ab_result.state_hash_status, ComparisonStatus::Match);
    assert_eq!(report.ab_result.render_status, ComparisonStatus::Match);
    assert!(report.fidelity_report.deterministic_state_hash_primary);
    assert!(report.fidelity_report.perceptual_render_secondary_only);

    let missing_render = verify_behavioral_ab(&request(
        ReExpressionTargetDimensionality::ThreeD,
        "fnv64:5555555555555555",
        Some("ssim:0.994;pixel-diff:0.001"),
        vec!["score_incremented", "door_opened"],
        "fnv64:5555555555555555",
        None,
    ))
    .unwrap();
    assert_eq!(
        missing_render.status,
        DifferentialVerificationStatus::NeedsRepair
    );
    assert_eq!(
        missing_render.ab_result.render_status,
        ComparisonStatus::Inconclusive
    );
    assert!(missing_render.semantic_port_handoff.is_none());
}

#[test]
fn v93_clean_room_source_only_and_no_studio_trusted_write() {
    let doc =
        read_text("docs/scenario-coverage-v93-differential-verification.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v93",
        "one-way on-ramp",
        "source-project/open-text",
        "no auto-port without oracle",
        "yellow/red",
        "state-hash primary",
        "rust remains the data plane",
        "no trusted-write",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "missing {required}");
    }
}

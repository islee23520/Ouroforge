//! Scenario Coverage v94 regression suite for #2236 / Era R M112.

use std::path::{Path, PathBuf};

use ouroforge_core::deterministic_reexpression::ReExpressionTargetDimensionality;
use ouroforge_core::legacy_logic_ingestion::{EraRHandoffState, FidelityGrade};
use ouroforge_core::loop_coverage_attribution::{
    LoopCoverageAttributionArtifact, LoopCoverageAttributionSignal, LoopCoverageAttributionStatus,
    LoopCoverageProvenanceClass, LOOP_COVERAGE_ATTRIBUTION_SCHEMA_VERSION,
};
use ouroforge_core::semantic_port_coverage::{
    evaluate_semantic_port_coverage, ConvergencePolicy, SemanticPortConvergenceStatus,
    SemanticPortCoverageRequest, SemanticPortCoverageUnit, SemanticPortUnitStatus,
    SEMANTIC_PORT_COVERAGE_SCHEMA_VERSION,
};
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

fn pending_lossy(unit: &str) -> SemanticPortCoverageUnit {
    SemanticPortCoverageUnit {
        unit_id: unit.to_string(),
        behavioral_unit_ref: format!("ir/m108/{unit}.json"),
        status: SemanticPortUnitStatus::Pending,
        fidelity_grade: FidelityGrade::Yellow,
        oracle_ref: None,
        primary_state_hash: None,
        secondary_render_digest: None,
        evidence_refs: vec![format!("evidence/m111/{unit}-lossy.json")],
        gap_summary: vec![
            "lossy import: missing oracle and observed behavior mismatch remain".to_string(),
        ],
        re_derivation_tasks: Vec::new(),
        loop_attribution: None,
    }
}

fn request(
    dimensionality: ReExpressionTargetDimensionality,
    units: Vec<SemanticPortCoverageUnit>,
) -> SemanticPortCoverageRequest {
    SemanticPortCoverageRequest {
        project_id: "scenario-coverage-v94".to_string(),
        source_project_ref: "source-projects/v94/source.project.json".to_string(),
        target_dimensionality: dimensionality,
        units,
        convergence_policy: ConvergencePolicy {
            current_iteration: 2,
            max_iterations: 5,
            allow_ring2_human_escalation: true,
            stop_when_all_verified: true,
        },
    }
}

#[test]
fn v94_matrix_records_rows_and_boundaries() {
    let matrix =
        read_json("examples/semantic-port-coverage-v1/scenario-coverage-v94/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v94-semantic-port-coverage-v1"
    );
    assert_eq!(matrix["coverageVersion"], 94);
    assert_eq!(matrix["issueRef"], "#2236");

    let rows = matrix["rows"].as_array().unwrap();
    for required in [
        "v94.coverage-complete-without-ported-claim",
        "v94.lossy-import-not-graded-clean",
        "v94.no-auto-port-without-oracle-fails",
        "v94.ungated-auto-translated-port-fails",
        "v94.deterministic-state-hash-break-fails",
        "v94.3d-state-hash-primary-render-secondary",
        "v94.clean-room-source-only-and-no-studio-trusted-write",
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
    assert_eq!(invariants["fullyPortedClaimAllowed"], false);
    assert_eq!(invariants["foreignRuntimeBridgeAllowed"], false);
}

#[test]
fn v94_coverage_complete_without_ported_claim() {
    let report = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![verified("unit.v94-door", "fnv64:aaaaaaaaaaaaaaaa")],
    ))
    .unwrap();

    assert_eq!(report.schema_version, SEMANTIC_PORT_COVERAGE_SCHEMA_VERSION);
    assert_eq!(
        report.convergence_status,
        SemanticPortConvergenceStatus::Passed
    );
    assert!(report.summary.semantic_coverage_complete);
    assert_eq!(report.summary.verified_basis_points, 10_000);
    assert!(!report.summary.ported_claim_allowed);
    assert!(!report.summary.fully_ported_claim_allowed);
    assert!(report.summary.source_apply_gate_required);
    assert!(!report.summary.studio_trusted_write_authority);
    assert!(report.residual_backlog.is_empty());
}

#[test]
fn v94_lossy_import_not_graded_clean() {
    let report = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![
            verified("unit.v94-door", "fnv64:bbbbbbbbbbbbbbbb"),
            pending_lossy("unit.v94-hazard"),
        ],
    ))
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
        report.residual_backlog[0].fidelity_grade,
        FidelityGrade::Yellow
    );
    assert_eq!(
        report.residual_backlog[0].next_handoff_state,
        EraRHandoffState::Reexpress
    );
    assert!(report.residual_backlog[0]
        .reasons
        .iter()
        .any(|reason| reason.contains("lossy import")));
}

#[test]
fn v94_no_auto_port_without_oracle_fails() {
    let mut unit = verified("unit.v94-door", "fnv64:cccccccccccccccc");
    unit.oracle_ref = None;
    let err = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit],
    ))
    .unwrap_err()
    .to_string();
    assert!(err.contains("captured oracle ref"));

    let mut unit = verified("unit.v94-door", "fnv64:dddddddddddddddd");
    unit.evidence_refs.clear();
    let err = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit],
    ))
    .unwrap_err()
    .to_string();
    assert!(err.contains("evidence refs"));
}

#[test]
fn v94_ungated_auto_translated_port_fails() {
    for forbidden_ref in [
        "decompiled/Assembly-CSharp/DoorController.cs",
        "ripped/shipped-build/assets.bin",
        "runtime/foreign-runtime/live-bridge.json",
        "vendor/vendored_unity_runtime/player.dll",
    ] {
        let mut unit = pending_lossy("unit.v94-hazard");
        unit.behavioral_unit_ref = forbidden_ref.to_string();
        let err = evaluate_semantic_port_coverage(&request(
            ReExpressionTargetDimensionality::TwoD,
            vec![unit],
        ))
        .unwrap_err()
        .to_string();
        assert!(
            err.contains("source-project/open-text"),
            "{forbidden_ref} unexpectedly passed: {err}"
        );
    }
}

#[test]
fn v94_deterministic_state_hash_break_fails() {
    let mut invalid = verified("unit.v94-door", "sha256:not-a-state-hash");
    invalid.primary_state_hash = Some("sha256:not-a-state-hash".to_string());
    let err = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![invalid],
    ))
    .unwrap_err()
    .to_string();
    assert!(err.contains("primary state hash"));

    let first = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![verified("unit.v94-door", "fnv64:eeeeeeeeeeeeeeee")],
    ))
    .unwrap();
    let second = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![verified("unit.v94-door", "fnv64:ffffffffffffffff")],
    ))
    .unwrap();
    assert_ne!(first.deterministic_digest, second.deterministic_digest);
}

#[test]
fn v94_3d_state_hash_primary_render_secondary() {
    let mut unit = verified("unit.v94-camera", "fnv64:1111111111111111");
    let err = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::ThreeD,
        vec![unit.clone()],
    ))
    .unwrap_err()
    .to_string();
    assert!(err.contains("secondary render corroboration"));

    unit.secondary_render_digest = Some("ssim:0.995;pixel-diff:0.002".to_string());
    let report = evaluate_semantic_port_coverage(&request(
        ReExpressionTargetDimensionality::ThreeD,
        vec![unit],
    ))
    .unwrap();
    assert_eq!(
        report.convergence_status,
        SemanticPortConvergenceStatus::Passed
    );
    assert!(report.summary.deterministic_state_hash_primary);
    assert!(report.summary.perceptual_render_secondary_only);
}

#[test]
fn v94_clean_room_source_only_and_no_studio_trusted_write() {
    let doc =
        read_text("docs/scenario-coverage-v94-semantic-port-coverage.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v94",
        "one-way on-ramp",
        "source-project/open-text",
        "no auto-port without oracle",
        "yellow/red",
        "state-hash primary",
        "rust remains the data plane",
        "no trusted-write",
        "fullyportedclaimallowed",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "missing {required}");
    }
}

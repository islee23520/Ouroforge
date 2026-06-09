//! Scenario Coverage v92 regression suite for #2231 / Era R M110.

use std::path::{Path, PathBuf};

use ouroforge_core::deterministic_reexpression::{
    reexpress_deterministic_behaviors, ReExpressionRequest, ReExpressionTargetDimensionality,
    DETERMINISTIC_REEXPRESSION_SCHEMA_VERSION,
};
use ouroforge_core::legacy_logic_ingestion::{
    BehavioralUnitRecord, EngineCouplingKind, EraRHandoffState, FidelityGrade, OracleStatus,
};
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

fn captured_unit() -> BehavioralUnitRecord {
    BehavioralUnitRecord {
        id: "unit.v92-open-door".to_string(),
        name: "V92OpenDoor".to_string(),
        source_path: "Assets/Scripts/V92Door.cs".to_string(),
        provenance_node_ids: vec!["ir.method.v92-door.update".to_string()],
        stimuli: vec!["input action Open on fixed tick".to_string()],
        observed_outcomes: vec!["door_opened event emitted exactly once".to_string()],
        engine_couplings: vec![EngineCouplingKind::Input],
        oracle_status: OracleStatus::Captured,
        fidelity_grade: FidelityGrade::Green,
        handoff_state: EraRHandoffState::Reexpress,
        ported_claim_allowed: false,
        gaps: Vec::new(),
    }
}

fn lossy_unit() -> BehavioralUnitRecord {
    BehavioralUnitRecord {
        id: "unit.v92-particle-feel".to_string(),
        name: "V92ParticleFeel".to_string(),
        source_path: "Assets/Scripts/V92Particles.cs".to_string(),
        provenance_node_ids: vec!["ir.method.v92-particles.burst".to_string()],
        stimuli: vec!["collision event".to_string()],
        observed_outcomes: vec!["particle burst appears with authored feel".to_string()],
        engine_couplings: vec![EngineCouplingKind::Rendering],
        oracle_status: OracleStatus::Missing,
        fidelity_grade: FidelityGrade::Yellow,
        handoff_state: EraRHandoffState::CaptureOracle,
        ported_claim_allowed: false,
        gaps: vec!["perceptual particle feel lacks captured oracle".to_string()],
    }
}

fn oracle(unit_id: &str, hash: &str, secondary: Option<&str>) -> OracleSpec {
    OracleSpec {
        id: format!("oracle.{unit_id}"),
        unit_id: unit_id.to_string(),
        stimulus: "frame 20 fixed input".to_string(),
        expected_events: vec!["door_opened".to_string()],
        primary_state_hash: hash.to_string(),
        secondary_render_digest: secondary.map(str::to_string),
        tolerance: if secondary.is_some() {
            "state-hash primary; SSIM/pixel-diff render secondary".to_string()
        } else {
            "2D bit-exact state hash".to_string()
        },
        provenance_refs: vec!["source-notes/v92-clean-room-intent.md".to_string()],
        status: CapturedOracleStatus::Captured,
        ported_claim_allowed: false,
    }
}

fn request(
    dimensionality: ReExpressionTargetDimensionality,
    units: Vec<BehavioralUnitRecord>,
    oracles: Vec<OracleSpec>,
) -> ReExpressionRequest {
    ReExpressionRequest {
        project_id: "scenario-coverage-v92".to_string(),
        scene_path: "scenes/v92-door.scene.json".to_string(),
        scene_hash: "sha256:0123456789abcdef".to_string(),
        target_dimensionality: dimensionality,
        units,
        oracle_specs: oracles,
        skeleton_refs: vec!["Assets/Scenes/V92Door.unity".to_string()],
    }
}

#[test]
fn v92_matrix_records_rows_and_boundaries() {
    let matrix = read_json(
        "examples/deterministic-reexpression-demo-v1/scenario-coverage-v92/matrix.fixture.json",
    );
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v92-deterministic-reexpression-v1"
    );
    assert_eq!(matrix["coverageVersion"], 92);
    assert_eq!(matrix["issueRef"], "#2231");

    let rows = matrix["rows"].as_array().unwrap();
    for required in [
        "v92.captured-oracle-reexpresses-gated-draft-not-port",
        "v92.lossy-import-not-graded-clean",
        "v92.ungated-auto-translation-port-fails",
        "v92.deterministic-state-hash-break-fails",
        "v92.3d-state-hash-primary-render-secondary",
        "v92.clean-room-source-only-and-no-studio-trusted-write",
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
}

#[test]
fn v92_captured_oracle_reexpresses_gated_draft_not_port() {
    let unit = captured_unit();
    let report = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit.clone()],
        vec![oracle(&unit.id, "fnv64:aaaaaaaaaaaaaaaa", None)],
    ))
    .unwrap();

    assert_eq!(
        report.schema_version,
        DETERMINISTIC_REEXPRESSION_SCHEMA_VERSION
    );
    assert_eq!(report.fidelity_report.green_count, 1);
    assert_eq!(report.fidelity_report.yellow_count, 0);
    assert_eq!(report.fidelity_report.red_count, 0);
    assert_eq!(report.behavior_drafts.len(), 1);
    assert_eq!(report.verification_handoffs.len(), 1);
    assert!(report.fidelity_report.no_oracle_not_ported);
    assert!(report.fidelity_report.source_apply_gate_required);
    assert!(!report.fidelity_report.studio_trusted_write_authority);
    assert!(report.plans.iter().all(|plan| !plan.ported_claim_allowed));
    assert!(report.plans.iter().all(|plan| {
        plan.gate_handoff.source_apply_required
            && plan.gate_handoff.review_gate_required
            && plan.gate_handoff.rollback_required
            && !plan.gate_handoff.writes_artifacts_directly
            && !plan.gate_handoff.trusted_write_authority
    }));
    assert!(report.behavior_drafts[0]
        .rationale
        .contains("not a ported claim"));
}

#[test]
fn v92_lossy_import_not_graded_clean() {
    let captured = captured_unit();
    let report = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![captured.clone(), lossy_unit()],
        vec![oracle(&captured.id, "fnv64:bbbbbbbbbbbbbbbb", None)],
    ))
    .unwrap();

    assert_eq!(report.fidelity_report.green_count, 1);
    assert_eq!(report.fidelity_report.yellow_count, 1);
    assert_eq!(report.fidelity_report.red_count, 0);
    assert_eq!(report.behavior_drafts.len(), 1);
    assert_eq!(report.re_derivation_tasks.len(), 1);
    assert_eq!(
        report.re_derivation_tasks[0].task,
        "capture_or_repair_oracle_before_reexpression"
    );
    assert!(report
        .fidelity_report
        .gap_summary
        .iter()
        .any(|gap| gap.contains("perceptual particle feel")));
    assert!(report
        .plans
        .iter()
        .find(|plan| plan.unit_id == "unit.v92-particle-feel")
        .is_some_and(|plan| plan.fidelity_grade == FidelityGrade::Yellow
            && plan.handoff_state == EraRHandoffState::CaptureOracle
            && !plan.ported_claim_allowed));
}

#[test]
fn v92_ungated_auto_translation_port_fails_red() {
    let mut unit = captured_unit();
    unit.ported_claim_allowed = true;
    unit.oracle_status = OracleStatus::Passing;
    unit.handoff_state = EraRHandoffState::Reexpress;
    let report = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit.clone()],
        vec![oracle(&unit.id, "fnv64:cccccccccccccccc", None)],
    ))
    .unwrap();

    assert_eq!(report.fidelity_report.green_count, 0);
    assert_eq!(report.fidelity_report.red_count, 1);
    assert!(report.behavior_drafts.is_empty());
    assert!(report.verification_handoffs.is_empty());
    assert!(report.fidelity_report.blocked_or_unsupported[0].contains("ported claim"));
    assert_eq!(
        report.re_derivation_tasks[0].task,
        "reject_or_defer_reexpression"
    );
    assert!(report.plans.iter().all(|plan| !plan.ported_claim_allowed));
}

#[test]
fn v92_deterministic_state_hash_break_fails() {
    let unit = captured_unit();
    let first = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit.clone()],
        vec![oracle(&unit.id, "fnv64:dddddddddddddddd", None)],
    ))
    .unwrap();
    let same = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit.clone()],
        vec![oracle(&unit.id, "fnv64:dddddddddddddddd", None)],
    ))
    .unwrap();
    let changed = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit.clone()],
        vec![oracle(&unit.id, "fnv64:eeeeeeeeeeeeeeee", None)],
    ))
    .unwrap();

    assert_eq!(first.deterministic_digest, same.deterministic_digest);
    assert_ne!(first.deterministic_digest, changed.deterministic_digest);
    assert_eq!(
        first.verification_handoffs[0].primary_state_hash,
        "fnv64:dddddddddddddddd"
    );
}

#[test]
fn v92_3d_state_hash_primary_render_secondary() {
    let unit = captured_unit();
    let report = reexpress_deterministic_behaviors(&request(
        ReExpressionTargetDimensionality::ThreeD,
        vec![unit.clone()],
        vec![oracle(
            &unit.id,
            "fnv64:ffffffffffffffff",
            Some("ssim:0.992;pixel-diff:0.001"),
        )],
    ))
    .unwrap();

    let handoff = &report.verification_handoffs[0];
    assert_eq!(handoff.primary_state_hash, "fnv64:ffffffffffffffff");
    assert_eq!(
        handoff.secondary_render_digest.as_deref(),
        Some("ssim:0.992;pixel-diff:0.001")
    );
    assert!(handoff.verification_rule.contains("state-hash primary"));
    assert!(handoff
        .verification_rule
        .contains("SSIM/pixel-diff secondary"));
    assert!(report.plans[0]
        .deterministic_constraints
        .iter()
        .any(|constraint| constraint.contains("state-hash primary")));
}

#[test]
fn v92_clean_room_source_only_and_no_studio_trusted_write() {
    let unit = captured_unit();
    let mut request = request(
        ReExpressionTargetDimensionality::TwoD,
        vec![unit.clone()],
        vec![oracle(&unit.id, "fnv64:1111111111111111", None)],
    );
    request.skeleton_refs = vec!["decompiled/Assembly-CSharp/DoorController.cs".to_string()];
    let err = reexpress_deterministic_behaviors(&request)
        .unwrap_err()
        .to_string();
    assert!(err.contains("source-project/open-text"));

    let doc =
        read_text("docs/scenario-coverage-v92-deterministic-reexpression.md").to_ascii_lowercase();
    for required in [
        "scenario coverage v92",
        "one-way on-ramp",
        "source-project/open-text",
        "no auto-port",
        "yellow/red",
        "state-hash primary",
        "rust remains the data plane",
        "no trusted-write",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "missing {required}");
    }
}

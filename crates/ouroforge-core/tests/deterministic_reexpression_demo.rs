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
        id: "unit.demo-open-door".to_string(),
        name: "DemoOpenDoor".to_string(),
        source_path: "Assets/Scripts/DemoDoor.cs".to_string(),
        provenance_node_ids: vec!["ir.method.demo-door.update".to_string()],
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

fn missing_oracle_unit() -> BehavioralUnitRecord {
    BehavioralUnitRecord {
        id: "unit.demo-particle-feel".to_string(),
        name: "DemoParticleFeel".to_string(),
        source_path: "Assets/Scripts/DemoParticles.cs".to_string(),
        provenance_node_ids: vec!["ir.method.demo-particles.burst".to_string()],
        stimuli: vec!["collision event".to_string()],
        observed_outcomes: vec!["particle burst appears".to_string()],
        engine_couplings: vec![EngineCouplingKind::Rendering],
        oracle_status: OracleStatus::Missing,
        fidelity_grade: FidelityGrade::Yellow,
        handoff_state: EraRHandoffState::CaptureOracle,
        ported_claim_allowed: false,
        gaps: vec!["perceptual particle feel needs human oracle".to_string()],
    }
}

fn oracle(hash: &str) -> OracleSpec {
    OracleSpec {
        id: "oracle.demo-open-door".to_string(),
        unit_id: "unit.demo-open-door".to_string(),
        stimulus: "frame 20 input Open".to_string(),
        expected_events: vec!["door_opened".to_string()],
        primary_state_hash: hash.to_string(),
        secondary_render_digest: None,
        tolerance: "2D bit-exact state hash".to_string(),
        provenance_refs: vec!["source-notes/demo-intent.md".to_string()],
        status: CapturedOracleStatus::Captured,
        ported_claim_allowed: false,
    }
}

fn request(hash: &str) -> ReExpressionRequest {
    ReExpressionRequest {
        project_id: "deterministic-reexpression-demo".to_string(),
        scene_path: "scenes/demo-door.scene.json".to_string(),
        scene_hash: "sha256:feedfacecafebeef".to_string(),
        target_dimensionality: ReExpressionTargetDimensionality::TwoD,
        units: vec![captured_unit(), missing_oracle_unit()],
        oracle_specs: vec![oracle(hash)],
        skeleton_refs: vec!["Assets/Scenes/DemoDoor.unity".to_string()],
    }
}

#[test]
fn demo_manifest_and_docs_record_honest_boundaries() {
    let manifest = read_json("examples/deterministic-reexpression-demo-v1/manifest.fixture.json");
    assert_eq!(
        manifest["schemaVersion"],
        "deterministic-reexpression-demo-v1"
    );
    assert_eq!(manifest["issueRef"], "#2230");
    assert_eq!(manifest["summary"]["portedClaimAllowed"], false);
    assert_eq!(manifest["summary"]["studioTrustedWriteAuthority"], false);
    assert_eq!(manifest["summary"]["sourceApplyRequired"], true);
    assert_eq!(manifest["expectedFidelity"]["noOracleNotPorted"], true);
    assert!(repo_root()
        .join(manifest["contractRef"].as_str().unwrap())
        .exists());
    assert!(repo_root()
        .join(manifest["seedRef"].as_str().unwrap())
        .exists());

    let doc = read_text("docs/deterministic-reexpression-demo-v1.md");
    for token in [
        "not a finished port claim",
        "source-apply",
        "State-hash determinism is primary",
        "no trusted-write",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(token), "missing {token}");
    }
}

#[test]
fn demo_reexpresses_captured_unit_and_keeps_oracle_less_unit_as_task() {
    let report = reexpress_deterministic_behaviors(&request("fnv64:1234567890abcdef")).unwrap();
    assert_eq!(
        report.schema_version,
        DETERMINISTIC_REEXPRESSION_SCHEMA_VERSION
    );
    assert_eq!(report.fidelity_report.green_count, 1);
    assert_eq!(report.fidelity_report.yellow_count, 1);
    assert_eq!(report.fidelity_report.red_count, 0);
    assert_eq!(report.behavior_drafts.len(), 1);
    assert_eq!(report.verification_handoffs.len(), 1);
    assert_eq!(report.re_derivation_tasks.len(), 1);
    assert!(report.fidelity_report.no_oracle_not_ported);
    assert!(report.fidelity_report.clean_room_source_only);
    assert!(report.fidelity_report.deterministic_reexpression);
    assert!(!report.fidelity_report.studio_trusted_write_authority);
    assert!(report.plans.iter().all(|plan| !plan.ported_claim_allowed));
    assert!(report
        .plans
        .iter()
        .all(|plan| plan.gate_handoff.source_apply_required
            && !plan.gate_handoff.writes_artifacts_directly));
}

#[test]
fn demo_state_hash_determinism_changes_report_digest() {
    let first = reexpress_deterministic_behaviors(&request("fnv64:1234567890abcdef")).unwrap();
    let same = reexpress_deterministic_behaviors(&request("fnv64:1234567890abcdef")).unwrap();
    let changed = reexpress_deterministic_behaviors(&request("fnv64:fedcba0987654321")).unwrap();

    assert_eq!(first.deterministic_digest, same.deterministic_digest);
    assert_ne!(first.deterministic_digest, changed.deterministic_digest);
    assert_eq!(
        first.verification_handoffs[0].primary_state_hash,
        "fnv64:1234567890abcdef"
    );
}

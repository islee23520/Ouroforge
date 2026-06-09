use ouroforge_core::human_artifact_intake::{
    validate_human_artifact_intake_json, HumanArtifactIntakeArtifact, HumanArtifactIntakeGateKind,
    HumanArtifactIntakeGateResult, HumanArtifactIntakeGateStatus, HumanArtifactIntakeStatus,
    HumanArtifactIntakeSurface, HumanArtifactKind, HUMAN_ARTIFACT_INTAKE_BOUNDARY,
    HUMAN_ARTIFACT_INTAKE_SCHEMA_VERSION,
};

fn gate(
    kind: HumanArtifactIntakeGateKind,
    status: HumanArtifactIntakeGateStatus,
) -> HumanArtifactIntakeGateResult {
    let key = format!("{kind:?}").to_ascii_lowercase();
    HumanArtifactIntakeGateResult {
        kind,
        status,
        evidence_ref: format!("runs/v67/evidence/{key}.json"),
        before_ref: format!("runs/v67/before/{key}.json"),
        after_ref: format!("runs/v67/after/{key}.json"),
    }
}

fn green_artifact() -> HumanArtifactIntakeArtifact {
    HumanArtifactIntakeArtifact {
        schema_version: HUMAN_ARTIFACT_INTAKE_SCHEMA_VERSION.to_string(),
        intake_id: "scenario-v67-human-intake-001".to_string(),
        artifact_id: "scenario-v67-card-human-001".to_string(),
        artifact_kind: HumanArtifactKind::Card,
        captured_via: HumanArtifactIntakeSurface::StudioPhoenixLiveView,
        author: "human:local-author".to_string(),
        author_provenance_ref: "runs/v67/provenance/human-author.json".to_string(),
        human_provenance: true,
        original_artifact_ref: "runs/v67/intake/original-card.json".to_string(),
        normalized_candidate_ref: "runs/v67/intake/normalized-card.json".to_string(),
        target_ref: "projects/demo/cards/card-spark.json".to_string(),
        target_base_ref: "hash:v67-card-base-before".to_string(),
        validation_report_ref: "runs/v67/validation/card-spark.report.json".to_string(),
        review_apply_ref: "runs/v67/review/card-spark.decision.json".to_string(),
        gate_results: vec![
            gate(
                HumanArtifactIntakeGateKind::ReviewApply,
                HumanArtifactIntakeGateStatus::Passed,
            ),
            gate(
                HumanArtifactIntakeGateKind::SceneSourceApply,
                HumanArtifactIntakeGateStatus::Passed,
            ),
            gate(
                HumanArtifactIntakeGateKind::Evaluator,
                HumanArtifactIntakeGateStatus::Passed,
            ),
            gate(
                HumanArtifactIntakeGateKind::EvidenceProvenance,
                HumanArtifactIntakeGateStatus::Passed,
            ),
        ],
        status: HumanArtifactIntakeStatus::ReadyForReviewApply,
        intervention_as_evidence: true,
        read_gated_write: true,
        raw_bypass_requested: false,
        direct_artifact_write: false,
        studio_trusted_write_authority: false,
        human_required_for_autonomous_loop: false,
        cli_fallback_supported: true,
        boundary: HUMAN_ARTIFACT_INTAKE_BOUNDARY.to_string(),
    }
}

#[test]
fn v67_human_artifact_is_ready_only_after_existing_gates_pass() {
    let artifact = green_artifact();
    artifact.validate().expect("v67 green artifact validates");
    assert!(artifact.ready_for_review_apply());

    let json = serde_json::to_string(&artifact).expect("serializes");
    let read_model = validate_human_artifact_intake_json(&json).expect("json validates");
    assert!(read_model.ready_for_review_apply);
    assert_eq!(read_model.gate_count, 4);
    assert_eq!(read_model.passed_gate_count, 4);
    assert!(read_model.blocked_reasons.is_empty());
    assert_eq!(read_model.author, "human:local-author");
    assert_eq!(
        read_model.author_provenance_ref,
        "runs/v67/provenance/human-author.json"
    );
}

#[test]
fn v67_rejects_no_raw_bypass_and_presentation_plane_write_regressions() {
    let mut raw_flag = green_artifact();
    raw_flag.raw_bypass_requested = true;
    assert!(raw_flag.validate().is_err());
    assert!(!raw_flag.ready_for_review_apply());

    let mut direct = green_artifact();
    direct.direct_artifact_write = true;
    assert!(direct.validate().is_err());
    assert!(!direct.ready_for_review_apply());

    let mut trusted_studio = green_artifact();
    trusted_studio.studio_trusted_write_authority = true;
    assert!(trusted_studio.validate().is_err());
    assert!(!trusted_studio.ready_for_review_apply());

    let mut bypass_ref = green_artifact();
    bypass_ref.validation_report_ref = "runs/v67/raw_write_bypass.json".to_string();
    let err = bypass_ref
        .validate()
        .expect_err("raw bypass refs are forbidden");
    assert!(format!("{err:#}").contains("raw bypass"));
}

#[test]
fn v67_failed_or_missing_gate_blocks_review_apply_readiness() {
    let mut failed_gate = green_artifact();
    failed_gate.gate_results[2].status = HumanArtifactIntakeGateStatus::Failed;
    assert!(failed_gate.validate().is_err());
    assert!(!failed_gate.ready_for_review_apply());

    failed_gate.status = HumanArtifactIntakeStatus::Rejected;
    failed_gate
        .validate()
        .expect("rejected intake preserves failed gate evidence");
    assert!(!failed_gate.ready_for_review_apply());

    let mut missing_gate = green_artifact();
    missing_gate
        .gate_results
        .retain(|gate| gate.kind != HumanArtifactIntakeGateKind::SceneSourceApply);
    let err = missing_gate
        .validate()
        .expect_err("missing existing gate is a v67 regression");
    assert!(format!("{err:#}").contains("missing required gate"));
}

#[test]
fn v67_loop_completes_without_human_and_cli_fallback_survives() {
    let artifact = green_artifact();
    assert!(!artifact.human_required_for_autonomous_loop);
    assert!(artifact.cli_fallback_supported);

    let mut human_required = green_artifact();
    human_required.human_required_for_autonomous_loop = true;
    let err = human_required
        .validate()
        .expect_err("mandatory human dependency is forbidden");
    assert!(format!("{err:#}").contains("mandatory humans"));

    let mut no_cli = green_artifact();
    no_cli.cli_fallback_supported = false;
    let err = no_cli
        .validate()
        .expect_err("CLI fallback must keep working");
    assert!(format!("{err:#}").contains("CLI fallback"));
}

#[test]
fn v67_coverage_record_contains_required_boundary_tokens() {
    let doc = include_str!("../../../docs/scenario-coverage-v67-human-artifact-intake.md");
    for token in [
        "Human-Authored Artifact Intake",
        "intervention-as-evidence",
        "author=human provenance",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "read + gated-write",
        "no raw bypass",
        "loop completes with zero human input",
        "Rust remains the data plane",
        "Studio/Phoenix captures and renders local control-plane requests only",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(token), "v67 doc missing token: {token}");
    }
}

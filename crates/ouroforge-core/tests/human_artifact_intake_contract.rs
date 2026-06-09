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
        evidence_ref: format!("runs/m76/evidence/{key}.json"),
        before_ref: format!("runs/m76/before/{key}.json"),
        after_ref: format!("runs/m76/after/{key}.json"),
    }
}

fn ready_artifact() -> HumanArtifactIntakeArtifact {
    HumanArtifactIntakeArtifact {
        schema_version: HUMAN_ARTIFACT_INTAKE_SCHEMA_VERSION.to_string(),
        intake_id: "human-intake-m76-001".to_string(),
        artifact_id: "card-spark-human-001".to_string(),
        artifact_kind: HumanArtifactKind::Card,
        captured_via: HumanArtifactIntakeSurface::StudioPhoenixLiveView,
        author: "human:local-author".to_string(),
        author_provenance_ref: "runs/m76/provenance/human-author.json".to_string(),
        human_provenance: true,
        original_artifact_ref: "runs/m76/intake/original-card.json".to_string(),
        normalized_candidate_ref: "runs/m76/intake/normalized-card.json".to_string(),
        target_ref: "projects/demo/cards/card-spark.json".to_string(),
        target_base_ref: "hash:card-base-before".to_string(),
        validation_report_ref: "runs/m76/validation/card-spark.report.json".to_string(),
        review_apply_ref: "runs/m76/review/card-spark.decision.json".to_string(),
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
fn human_authored_artifact_passes_same_gates_with_human_provenance() {
    let artifact = ready_artifact();
    artifact.validate().expect("valid human intake");
    assert!(artifact.ready_for_review_apply());

    let json = serde_json::to_string_pretty(&artifact).expect("serializes");
    assert!(json.contains("human:local-author"));
    let read = validate_human_artifact_intake_json(&json).expect("json validates");
    assert!(read.ready_for_review_apply);
    assert_eq!(read.passed_gate_count, 4);
    assert_eq!(read.author, "human:local-author");
    assert_eq!(
        read.author_provenance_ref,
        "runs/m76/provenance/human-author.json"
    );
}

#[test]
fn invalid_or_missing_gate_rejects_same_as_agent_output() {
    let mut failed = ready_artifact();
    failed.gate_results[2].status = HumanArtifactIntakeGateStatus::Failed;
    let err = failed
        .validate()
        .expect_err("ready intake cannot include failed evaluator gate");
    assert!(format!("{err:#}").contains("requires review/apply"));

    failed.status = HumanArtifactIntakeStatus::Rejected;
    failed
        .validate()
        .expect("rejected intake keeps failed gate evidence visible");
    assert!(!failed.ready_for_review_apply());

    let mut missing = ready_artifact();
    missing
        .gate_results
        .retain(|gate| gate.kind != HumanArtifactIntakeGateKind::SceneSourceApply);
    let err = missing
        .validate()
        .expect_err("missing scene/source apply gate blocks");
    assert!(format!("{err:#}").contains("missing required gate"));
}

#[test]
fn raw_bypass_trusted_studio_write_and_mandatory_human_are_rejected() {
    let mut raw = ready_artifact();
    raw.raw_bypass_requested = true;
    assert!(raw.validate().is_err());

    let mut direct = ready_artifact();
    direct.direct_artifact_write = true;
    assert!(direct.validate().is_err());

    let mut studio = ready_artifact();
    studio.studio_trusted_write_authority = true;
    assert!(studio.validate().is_err());

    let mut human_required = ready_artifact();
    human_required.human_required_for_autonomous_loop = true;
    let err = human_required
        .validate()
        .expect_err("human intake cannot be mandatory");
    assert!(format!("{err:#}").contains("mandatory humans"));
}

#[test]
fn intake_requires_human_author_and_normalized_candidate() {
    let mut non_human = ready_artifact();
    non_human.author = "agent".to_string();
    let err = non_human
        .validate()
        .expect_err("author must be human provenance");
    assert!(format!("{err:#}").contains("author=human"));

    let mut no_normalization = ready_artifact();
    no_normalization.normalized_candidate_ref = no_normalization.original_artifact_ref.clone();
    let err = no_normalization
        .validate()
        .expect_err("raw original cannot become trusted candidate directly");
    assert!(format!("{err:#}").contains("normalizedCandidateRef"));

    let mut bypass_ref = ready_artifact();
    bypass_ref.original_artifact_ref = "runs/m76/raw_write_bypass.json".to_string();
    let err = bypass_ref
        .validate()
        .expect_err("bypass token in refs is rejected");
    assert!(format!("{err:#}").contains("raw bypass"));
}

#[test]
fn boundary_preserves_two_plane_local_first_and_open_anchors() {
    let mut artifact = ready_artifact();
    artifact.boundary = "read + gated-write only".to_string();
    let err = artifact.validate().expect_err("thin boundary is rejected");
    assert!(format!("{err:#}").contains("human-authored artifact intake"));

    for token in [
        "intervention-as-evidence",
        "read + gated-write",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "local-first CLI fallback",
        "loop completes without human",
        "#1 and #23 remain open",
    ] {
        assert!(HUMAN_ARTIFACT_INTAKE_BOUNDARY.contains(token));
    }
}

use ouroforge_core::proposal_amendment::{
    validate_proposal_amendment_json, ProposalAmendmentArtifact, ProposalAmendmentCaptureSurface,
    ProposalAmendmentGateKind, ProposalAmendmentGateResult, ProposalAmendmentGateStatus,
    ProposalAmendmentStatus, PROPOSAL_AMENDMENT_BOUNDARY, PROPOSAL_AMENDMENT_SCHEMA_VERSION,
};

fn gate(
    kind: ProposalAmendmentGateKind,
    status: ProposalAmendmentGateStatus,
) -> ProposalAmendmentGateResult {
    let key = format!("{kind:?}").to_ascii_lowercase();
    ProposalAmendmentGateResult {
        kind,
        status,
        evidence_ref: format!("runs/m75/evidence/{key}.json"),
        before_ref: format!("runs/m75/before/{key}.json"),
        after_ref: format!("runs/m75/after/{key}.json"),
    }
}

fn ready_artifact() -> ProposalAmendmentArtifact {
    ProposalAmendmentArtifact {
        schema_version: PROPOSAL_AMENDMENT_SCHEMA_VERSION.to_string(),
        amendment_id: "amendment-m75-001".to_string(),
        proposal_id: "proposal-agent-001".to_string(),
        base_proposal_ref: "runs/m75/proposals/proposal-agent-001.before.json".to_string(),
        amended_proposal_ref: "runs/m75/proposals/proposal-agent-001.amended.json".to_string(),
        human_actor: "local-human".to_string(),
        edit_summary: "Adjust proposed balance config before approval.".to_string(),
        captured_via: ProposalAmendmentCaptureSurface::StudioPhoenixLiveView,
        intervention_as_evidence: true,
        before_evidence_refs: vec!["runs/m75/evidence/before-verdict.json".to_string()],
        after_evidence_refs: vec!["runs/m75/evidence/after-verdict.json".to_string()],
        provenance_refs: vec!["runs/m75/evidence/human-amendment-provenance.json".to_string()],
        gate_results: vec![
            gate(
                ProposalAmendmentGateKind::ReviewApply,
                ProposalAmendmentGateStatus::Passed,
            ),
            gate(
                ProposalAmendmentGateKind::SceneSourceApply,
                ProposalAmendmentGateStatus::Passed,
            ),
            gate(
                ProposalAmendmentGateKind::Evaluator,
                ProposalAmendmentGateStatus::Passed,
            ),
            gate(
                ProposalAmendmentGateKind::DesignIntegrity,
                ProposalAmendmentGateStatus::Passed,
            ),
        ],
        status: ProposalAmendmentStatus::ReadyForReviewApply,
        review_apply_ref: "runs/m75/review-apply/amendment-m75-001.decision.json".to_string(),
        auto_apply_performed: false,
        raw_bypass_requested: false,
        studio_trusted_write_authority: false,
        human_required_for_autonomous_loop: false,
        cli_fallback_supported: true,
        boundary: PROPOSAL_AMENDMENT_BOUNDARY.to_string(),
    }
}

#[test]
fn amended_proposal_requires_full_green_gate_set_before_review_apply() {
    let artifact = ready_artifact();
    artifact
        .validate()
        .expect("ready amended proposal validates");
    assert!(artifact.ready_for_review_apply());

    let read = artifact.read_model();
    assert!(read.ready_for_review_apply);
    assert_eq!(read.passed_gate_count, 4);
    assert!(read.blocked_reasons.is_empty());
    assert_eq!(
        read.review_apply_ref,
        "runs/m75/review-apply/amendment-m75-001.decision.json"
    );
    assert!(read.boundary.contains("intervention-as-evidence"));
    assert!(read.boundary.contains("read + gated-write"));
}

#[test]
fn amended_proposal_records_before_after_provenance_and_round_trips_json() {
    let artifact = ready_artifact();
    let json = serde_json::to_string_pretty(&artifact).expect("serializes");
    assert!(json.contains("human-amendment-provenance"));
    let read = validate_proposal_amendment_json(&json).expect("json validates");
    assert_eq!(
        read.before_evidence_refs,
        vec!["runs/m75/evidence/before-verdict.json"]
    );
    assert_eq!(
        read.after_evidence_refs,
        vec!["runs/m75/evidence/after-verdict.json"]
    );
    assert_eq!(
        read.provenance_refs,
        vec!["runs/m75/evidence/human-amendment-provenance.json"]
    );
}

#[test]
fn missing_or_failed_gate_blocks_review_apply() {
    let mut artifact = ready_artifact();
    artifact
        .gate_results
        .retain(|gate| gate.kind != ProposalAmendmentGateKind::DesignIntegrity);
    let err = artifact
        .validate()
        .expect_err("missing design-integrity gate blocks");
    assert!(format!("{err:#}").contains("missing required gate"));

    let mut failed = ready_artifact();
    failed.gate_results[2].status = ProposalAmendmentGateStatus::Failed;
    let err = failed
        .validate()
        .expect_err("ready cannot include failed evaluator gate");
    assert!(format!("{err:#}").contains("requires review/apply"));

    failed.status = ProposalAmendmentStatus::Rejected;
    failed
        .validate()
        .expect("rejected amendment keeps failed gate visible");
    assert!(!failed.ready_for_review_apply());
}

#[test]
fn stale_gate_requires_visible_stale_status() {
    let mut artifact = ready_artifact();
    artifact.status = ProposalAmendmentStatus::Stale;
    let err = artifact
        .validate()
        .expect_err("stale without stale gate is invalid");
    assert!(format!("{err:#}").contains("stale gate"));

    artifact.gate_results[0].status = ProposalAmendmentGateStatus::Stale;
    artifact
        .validate()
        .expect("stale gate evidence supports stale state");
}

#[test]
fn raw_bypass_or_studio_trusted_write_authority_is_rejected() {
    let mut raw = ready_artifact();
    raw.raw_bypass_requested = true;
    let err = raw.validate().expect_err("raw bypass is forbidden");
    assert!(format!("{err:#}").contains("raw bypass"));

    let mut studio_write = ready_artifact();
    studio_write.studio_trusted_write_authority = true;
    let err = studio_write
        .validate()
        .expect_err("Studio cannot own trusted writes");
    assert!(format!("{err:#}").contains("trusted writes"));

    let mut human_required = ready_artifact();
    human_required.human_required_for_autonomous_loop = true;
    let err = human_required
        .validate()
        .expect_err("human amendment cannot be required");
    assert!(format!("{err:#}").contains("require humans"));
}

#[test]
fn boundary_must_preserve_two_plane_and_local_first_terms() {
    let mut artifact = ready_artifact();
    artifact.boundary = "read + gated-write only".to_string();
    let err = artifact.validate().expect_err("thin boundary is rejected");
    assert!(format!("{err:#}").contains("intervention-as-evidence"));

    let mut bypass_ref = ready_artifact();
    bypass_ref.edit_summary = "please use raw_write_bypass".to_string();
    let err = bypass_ref.validate().expect_err("bypass refs are rejected");
    assert!(format!("{err:#}").contains("raw bypass"));
}

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
        evidence_ref: format!("runs/v66/evidence/{key}.json"),
        before_ref: format!("runs/v66/before/{key}.json"),
        after_ref: format!("runs/v66/after/{key}.json"),
    }
}

fn green_artifact() -> ProposalAmendmentArtifact {
    ProposalAmendmentArtifact {
        schema_version: PROPOSAL_AMENDMENT_SCHEMA_VERSION.to_string(),
        amendment_id: "scenario-v66-amendment-001".to_string(),
        proposal_id: "scenario-v66-proposal-001".to_string(),
        base_proposal_ref: "runs/v66/proposals/proposal.before.json".to_string(),
        amended_proposal_ref: "runs/v66/proposals/proposal.amended.json".to_string(),
        human_actor: "local-human".to_string(),
        edit_summary: "Tune the proposal before approval while preserving agent-first defaults."
            .to_string(),
        captured_via: ProposalAmendmentCaptureSurface::StudioPhoenixLiveView,
        intervention_as_evidence: true,
        before_evidence_refs: vec!["runs/v66/evidence/before-verdict.json".to_string()],
        after_evidence_refs: vec!["runs/v66/evidence/after-verdict.json".to_string()],
        provenance_refs: vec!["runs/v66/evidence/intervention-provenance.json".to_string()],
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
        review_apply_ref: "runs/v66/review-apply/decision.json".to_string(),
        auto_apply_performed: false,
        raw_bypass_requested: false,
        studio_trusted_write_authority: false,
        human_required_for_autonomous_loop: false,
        cli_fallback_supported: true,
        boundary: PROPOSAL_AMENDMENT_BOUNDARY.to_string(),
    }
}

#[test]
fn v66_green_amendment_is_ready_only_after_all_existing_gates_pass() {
    let artifact = green_artifact();

    artifact.validate().expect("all gates passed");
    assert!(artifact.ready_for_review_apply());

    let json = serde_json::to_string(&artifact).expect("serializes");
    let read_model = validate_proposal_amendment_json(&json).expect("json validates");
    assert!(read_model.ready_for_review_apply);
    assert_eq!(read_model.gate_count, 4);
    assert_eq!(read_model.passed_gate_count, 4);
    assert!(read_model.blocked_reasons.is_empty());
}

#[test]
fn v66_rejects_raw_bypass_studio_trusted_write_and_auto_apply() {
    let mut raw_bypass = green_artifact();
    raw_bypass.raw_bypass_requested = true;
    assert!(raw_bypass.validate().is_err());
    assert!(!raw_bypass.ready_for_review_apply());

    let mut trusted_write = green_artifact();
    trusted_write.studio_trusted_write_authority = true;
    assert!(trusted_write.validate().is_err());
    assert!(!trusted_write.ready_for_review_apply());

    let mut auto_apply = green_artifact();
    auto_apply.auto_apply_performed = true;
    assert!(auto_apply.validate().is_err());
    assert!(!auto_apply.ready_for_review_apply());

    let mut bypass_token = green_artifact();
    bypass_token.after_evidence_refs = vec!["runs/v66/raw_write_bypass.json".to_string()];
    let err = bypass_token
        .validate()
        .expect_err("raw bypass evidence references are forbidden");
    assert!(format!("{err:#}").contains("raw bypass"));
}

#[test]
fn v66_failed_or_missing_reverify_gate_blocks_review_apply_readiness() {
    let mut failed_gate = green_artifact();
    failed_gate.gate_results[2].status = ProposalAmendmentGateStatus::Failed;
    assert!(failed_gate.validate().is_err());
    assert!(!failed_gate.ready_for_review_apply());

    failed_gate.status = ProposalAmendmentStatus::Rejected;
    failed_gate
        .validate()
        .expect("rejected status preserves failed gate evidence");
    assert!(!failed_gate.ready_for_review_apply());

    let mut missing_gate = green_artifact();
    missing_gate
        .gate_results
        .retain(|gate| gate.kind != ProposalAmendmentGateKind::SceneSourceApply);
    let err = missing_gate
        .validate()
        .expect_err("missing existing gate is a v66 regression");
    assert!(format!("{err:#}").contains("missing required gate"));
}

#[test]
fn v66_human_intervention_remains_optional_and_cli_fallback_survives() {
    let artifact = green_artifact();
    assert!(!artifact.human_required_for_autonomous_loop);
    assert!(artifact.cli_fallback_supported);

    let mut human_required = green_artifact();
    human_required.human_required_for_autonomous_loop = true;
    let err = human_required
        .validate()
        .expect_err("mandatory human dependency is forbidden");
    assert!(format!("{err:#}").contains("require humans"));

    let mut no_cli = green_artifact();
    no_cli.cli_fallback_supported = false;
    let err = no_cli
        .validate()
        .expect_err("CLI fallback must keep working");
    assert!(format!("{err:#}").contains("CLI fallback"));
}

#[test]
fn v66_coverage_record_contains_required_boundary_tokens() {
    let doc = include_str!("../../../docs/scenario-coverage-v66.md");
    for token in [
        "Proposal Amendment and Re-Verify",
        "no raw-bypass",
        "loop completes without human",
        "read + gated-write",
        "intervention-as-evidence",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(token), "v66 doc missing token: {token}");
    }
}

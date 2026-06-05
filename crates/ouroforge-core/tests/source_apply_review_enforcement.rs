use ouroforge_core::{
    SourceApplyReviewDecisionState, SourceApplyReviewEnforcement,
    SourceApplyReviewEnforcementStatus, SourceApplyReviewTargetCoverage,
    SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION,
};

fn target(path: &str) -> SourceApplyReviewTargetCoverage {
    SourceApplyReviewTargetCoverage {
        path: path.to_string(),
        before_hash: format!("before-{path}"),
        after_hash: format!("after-{path}"),
    }
}

/// A fully matching, accepted, independent decision — the only ready case.
fn accepted_fixture() -> SourceApplyReviewEnforcement {
    SourceApplyReviewEnforcement {
        schema_version: SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-704-1".to_string(),
        patch_preview_id: "patch-preview-704-1".to_string(),
        expected_diff_hash: "diffhash-704-1".to_string(),
        transaction_base_revision: "base-rev-704-1".to_string(),
        expected_targets: vec![
            target("examples/source-apply-v1/sample-a.rs"),
            target("examples/source-apply-v1/sample-b.rs"),
        ],
        proposer_id: "agent-proposer".to_string(),
        reviewer_id: "agent-reviewer".to_string(),
        decision_state: SourceApplyReviewDecisionState::Accepted,
        decision_transaction_id: "apply-txn-704-1".to_string(),
        decision_preview_id: "patch-preview-704-1".to_string(),
        decision_diff_hash: "diffhash-704-1".to_string(),
        decision_base_revision: "base-rev-704-1".to_string(),
        decision_targets: vec![
            target("examples/source-apply-v1/sample-a.rs"),
            target("examples/source-apply-v1/sample-b.rs"),
        ],
        guardrails: vec![
            "review enforcement does not apply patches or merge branches".to_string(),
            "dashboard and Studio display this evaluation read-only".to_string(),
        ],
    }
}

#[test]
fn accepted_independent_exact_decision_is_ready() {
    let enforcement = accepted_fixture();
    enforcement
        .validate()
        .expect("structural validation passes");
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Ready,
        "blocked: {:?}",
        evaluation.blocked_reasons
    );
    assert!(evaluation.blocked_reasons.is_empty());
    assert!(enforcement.is_ready());
}

#[test]
fn ready_evaluation_has_no_apply_or_merge_authority() {
    let evaluation = accepted_fixture().evaluate();
    let value = serde_json::to_value(&evaluation).expect("evaluation serializes");
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("mergeCommand").is_none());
    for forbidden in [
        "apply_patch",
        "merge_branch",
        "execute_command",
        "self_approve",
    ] {
        assert!(
            evaluation
                .forbidden_actions
                .iter()
                .any(|action| action == forbidden),
            "missing forbidden action {forbidden}"
        );
    }
}

#[test]
fn missing_decision_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    enforcement.decision_state = SourceApplyReviewDecisionState::Missing;
    enforcement.reviewer_id = String::new();
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("not accepted")));
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("no recorded independent reviewer")));
}

#[test]
fn rejected_deferred_and_withdrawn_decisions_block_readiness() {
    for state in [
        SourceApplyReviewDecisionState::Rejected,
        SourceApplyReviewDecisionState::Deferred,
        SourceApplyReviewDecisionState::Withdrawn,
    ] {
        let mut enforcement = accepted_fixture();
        enforcement.decision_state = state;
        let evaluation = enforcement.evaluate();
        assert_eq!(
            evaluation.status,
            SourceApplyReviewEnforcementStatus::Blocked,
            "state {state:?} must block"
        );
        assert!(evaluation
            .blocked_reasons
            .iter()
            .any(|reason| reason.contains("not accepted")));
    }
}

#[test]
fn self_review_is_impossible() {
    let mut enforcement = accepted_fixture();
    enforcement.reviewer_id = enforcement.proposer_id.clone();
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("self-review/self-approval")));
}

#[test]
fn mismatched_transaction_id_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    enforcement.decision_transaction_id = "apply-txn-other".to_string();
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("does not match the apply transaction id")));
}

#[test]
fn mismatched_diff_hash_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    enforcement.decision_diff_hash = "diffhash-tampered".to_string();
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("diff hash does not match")));
}

#[test]
fn stale_decision_against_other_base_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    enforcement.decision_base_revision = "base-rev-old".to_string();
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("stale")));
}

#[test]
fn partial_coverage_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    // Decision only covers one of the two expected targets.
    enforcement.decision_targets = vec![target("examples/source-apply-v1/sample-a.rs")];
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("partial coverage")));
}

#[test]
fn unexpected_extra_target_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    enforcement
        .decision_targets
        .push(target("examples/source-apply-v1/sample-extra.rs"));
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("unexpected file target")));
}

#[test]
fn hash_mismatch_for_covered_target_blocks_readiness() {
    let mut enforcement = accepted_fixture();
    enforcement.decision_targets[0].after_hash = "after-tampered".to_string();
    let evaluation = enforcement.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyReviewEnforcementStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("hash coverage")));
}

#[test]
fn json_round_trip_preserves_artifact() {
    let enforcement = accepted_fixture();
    let json = serde_json::to_string_pretty(&enforcement).expect("serializes");
    let parsed = SourceApplyReviewEnforcement::from_json_str(&json).expect("parses and validates");
    assert_eq!(parsed, enforcement);
}

#[test]
fn validation_rejects_wrong_schema_version() {
    let mut enforcement = accepted_fixture();
    enforcement.schema_version = "wrong-schema".to_string();
    assert!(enforcement.validate().is_err());
}

#[test]
fn validation_rejects_path_traversal_target() {
    let mut enforcement = accepted_fixture();
    enforcement.expected_targets[0].path = "../../etc/passwd".to_string();
    assert!(enforcement.validate().is_err());
}

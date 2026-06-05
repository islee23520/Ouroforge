use ouroforge_core::{
    SourceApplySandboxCleanupState, SourceApplySandboxCommandEvidence, SourceApplySandboxPromotion,
    SourceApplySandboxPromotionStatus, SourceApplySandboxReportState,
    SourceApplySandboxTargetExpectation, SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION,
};

fn matching_target(path: &str) -> SourceApplySandboxTargetExpectation {
    SourceApplySandboxTargetExpectation {
        path: path.to_string(),
        trusted_before_hash: format!("before-{path}"),
        expected_after_hash: format!("after-{path}"),
        sandbox_before_hash: format!("before-{path}"),
        sandbox_after_hash: format!("after-{path}"),
    }
}

fn passing_command(command: &str) -> SourceApplySandboxCommandEvidence {
    SourceApplySandboxCommandEvidence {
        command: command.to_string(),
        allowlist_policy_id: "source-patch-preview-safe-local-checks-v1".to_string(),
        status: "passed".to_string(),
    }
}

fn valid_fixture() -> SourceApplySandboxPromotion {
    SourceApplySandboxPromotion {
        schema_version: SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION.to_string(),
        patch_preview_id: "patch-preview-705-1".to_string(),
        apply_transaction_id: "apply-txn-705-1".to_string(),
        expected_diff_hash: "diffhash-705-1".to_string(),
        sandbox_diff_hash: "diffhash-705-1".to_string(),
        transaction_base_revision: "base-rev-705-1".to_string(),
        sandbox_base_revision: "base-rev-705-1".to_string(),
        report_state: SourceApplySandboxReportState::Passed,
        targets: vec![
            matching_target("examples/source-apply-v1/sample-a.rs"),
            matching_target("examples/source-apply-v1/sample-b.rs"),
        ],
        allowlisted_commands: vec![
            passing_command("cargo fmt --check"),
            passing_command("cargo test -p ouroforge-core"),
        ],
        forbidden_commands_observed: Vec::new(),
        cleanup_state: SourceApplySandboxCleanupState::Complete,
        generated_state_isolated: true,
        guardrails: vec![
            "sandbox promotion does not apply patches or guarantee a secure sandbox".to_string(),
            "dashboard and Studio display this evaluation read-only".to_string(),
        ],
    }
}

#[test]
fn valid_sandbox_promotion_is_ready() {
    let promotion = valid_fixture();
    promotion.validate().expect("structural validation passes");
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Ready,
        "blocked: {:?}",
        evaluation.blocked_reasons
    );
    assert!(promotion.is_ready());
    for forbidden in ["apply_patch", "merge_branch", "execute_command"] {
        assert!(evaluation
            .forbidden_actions
            .iter()
            .any(|action| action == forbidden));
    }
}

#[test]
fn missing_sandbox_report_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.report_state = SourceApplySandboxReportState::Missing;
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("missing")));
}

#[test]
fn failed_sandbox_report_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.report_state = SourceApplySandboxReportState::Failed;
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("did not pass")));
}

#[test]
fn stale_sandbox_report_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.sandbox_base_revision = "base-rev-old".to_string();
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("stale")));
}

#[test]
fn mismatched_diff_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.sandbox_diff_hash = "diffhash-other".to_string();
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("different diff")));
}

#[test]
fn forbidden_command_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.forbidden_commands_observed = vec!["curl https://example.com/install.sh".to_string()];
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("forbidden command observed")));
}

#[test]
fn failed_allowlisted_command_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.allowlisted_commands[0].status = "failed".to_string();
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("did not pass in the sandbox")));
}

#[test]
fn missing_cleanup_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.cleanup_state = SourceApplySandboxCleanupState::Missing;
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("cleanup")));
}

#[test]
fn target_mismatch_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.targets[0].sandbox_after_hash = "after-tampered".to_string();
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("after-state does not match")));
}

#[test]
fn before_state_mismatch_blocks_promotion() {
    let mut promotion = valid_fixture();
    promotion.targets[0].sandbox_before_hash = "before-drifted".to_string();
    let evaluation = promotion.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplySandboxPromotionStatus::Blocked
    );
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("before-state mismatch")));
}

#[test]
fn json_round_trip_preserves_artifact() {
    let promotion = valid_fixture();
    let json = serde_json::to_string_pretty(&promotion).expect("serializes");
    let parsed = SourceApplySandboxPromotion::from_json_str(&json).expect("parses and validates");
    assert_eq!(parsed, promotion);
}

#[test]
fn validation_rejects_wrong_schema_version() {
    let mut promotion = valid_fixture();
    promotion.schema_version = "wrong".to_string();
    assert!(promotion.validate().is_err());
}

//! Scenario Coverage v107: controlled failure to reviewed fix flow (#2379).

use ouroforge_core::source_apply_controlled_failure_flow::{
    SourceApplyControlledFailureFlow, SourceApplyControlledFailureFlowStatus,
    SOURCE_APPLY_CONTROLLED_FAILURE_FLOW_SCHEMA_VERSION,
};
use ouroforge_core::source_apply_post_apply_rerun::{
    SourceApplyComparisonDimension, SourceApplyComparisonState, SourceApplyRerunComparison,
    SourceApplyRerunStatus, SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION,
};
use ouroforge_core::source_apply_review_enforcement::{
    SourceApplyReviewDecisionState, SourceApplyReviewEnforcement,
    SourceApplyReviewEnforcementStatus, SourceApplyReviewTargetCoverage,
    SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION,
};
use ouroforge_core::source_apply_sandbox_promotion::{
    SourceApplySandboxCleanupState, SourceApplySandboxCommandEvidence, SourceApplySandboxPromotion,
    SourceApplySandboxReportState, SourceApplySandboxTargetExpectation,
    SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION,
};
use std::fs;

fn review_target() -> SourceApplyReviewTargetCoverage {
    SourceApplyReviewTargetCoverage {
        path: "examples/collect-and-exit/scene.json".to_string(),
        before_hash: "before-real-file".to_string(),
        after_hash: "after-real-file".to_string(),
    }
}

fn review(proposer: &str, reviewer: &str) -> SourceApplyReviewEnforcement {
    SourceApplyReviewEnforcement {
        schema_version: SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "txn-v107".to_string(),
        patch_preview_id: "preview-v107".to_string(),
        expected_diff_hash: "diff-v107".to_string(),
        transaction_base_revision: "base-v107".to_string(),
        expected_targets: vec![review_target()],
        proposer_id: proposer.to_string(),
        reviewer_id: reviewer.to_string(),
        decision_state: SourceApplyReviewDecisionState::Accepted,
        decision_transaction_id: "txn-v107".to_string(),
        decision_preview_id: "preview-v107".to_string(),
        decision_diff_hash: "diff-v107".to_string(),
        decision_base_revision: "base-v107".to_string(),
        decision_targets: vec![review_target()],
        guardrails: vec!["review before apply; no self-approval".to_string()],
    }
}

fn sandbox() -> SourceApplySandboxPromotion {
    SourceApplySandboxPromotion {
        schema_version: SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION.to_string(),
        patch_preview_id: "preview-v107".to_string(),
        apply_transaction_id: "txn-v107".to_string(),
        expected_diff_hash: "diff-v107".to_string(),
        sandbox_diff_hash: "diff-v107".to_string(),
        transaction_base_revision: "base-v107".to_string(),
        sandbox_base_revision: "base-v107".to_string(),
        report_state: SourceApplySandboxReportState::Passed,
        targets: vec![SourceApplySandboxTargetExpectation {
            path: "examples/collect-and-exit/scene.json".to_string(),
            trusted_before_hash: "before-real-file".to_string(),
            expected_after_hash: "after-real-file".to_string(),
            sandbox_before_hash: "before-real-file".to_string(),
            sandbox_after_hash: "after-real-file".to_string(),
        }],
        allowlisted_commands: vec![SourceApplySandboxCommandEvidence {
            command: "cargo test -p ouroforge-core scenario_coverage_v107".to_string(),
            allowlist_policy_id: "safe-local-checks-v1".to_string(),
            status: "passed".to_string(),
        }],
        forbidden_commands_observed: vec![],
        cleanup_state: SourceApplySandboxCleanupState::Complete,
        generated_state_isolated: true,
        guardrails: vec!["sandbox only; main worktree unchanged".to_string()],
    }
}

fn comparison(state: SourceApplyComparisonState) -> SourceApplyRerunComparison {
    SourceApplyRerunComparison {
        schema_version: SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "txn-v107".to_string(),
        review_decision_ref: "evidence/review-v107.json".to_string(),
        verification_log_ref: "runs/v107/verification.json".to_string(),
        rollback_snapshot_ref: "evidence/rollback-v107.json".to_string(),
        audit_ledger_ref: "evidence/audit-v107.json".to_string(),
        qa_backlog_refs: vec![],
        rerun_status: SourceApplyRerunStatus::Completed,
        before_run_id: Some("run-before-v107".to_string()),
        after_run_id: Some("run-after-v107".to_string()),
        generated_output_root: "runs/v107".to_string(),
        dimensions: vec![SourceApplyComparisonDimension {
            name: "gameplay-objective".to_string(),
            state,
            detail: "exit flag flips after reviewed sandbox fix".to_string(),
        }],
        guardrails: vec!["comparison links before and after bundles".to_string()],
    }
}

fn flow() -> SourceApplyControlledFailureFlow {
    SourceApplyControlledFailureFlow {
        schema_version: SOURCE_APPLY_CONTROLLED_FAILURE_FLOW_SCHEMA_VERSION.to_string(),
        flow_id: "flow-v107".to_string(),
        apply_transaction_id: "txn-v107".to_string(),
        sandbox_failure_file: "scene.json".to_string(),
        before_bundle_ref: "runs/v107/before/manifest.json".to_string(),
        after_bundle_ref: "runs/v107/after/manifest.json".to_string(),
        comparison_artifact_ref: "runs/v107/comparison.json".to_string(),
        review_artifact_ref: "evidence/review-v107.json".to_string(),
        sandbox_artifact_ref: "runs/v107/sandbox-report.json".to_string(),
        main_worktree_status_ref: "runs/v107/main-worktree-status.txt".to_string(),
        self_approval_attempted: true,
        self_approval_blocked: true,
        review: review("agent-proposer", "agent-reviewer"),
        sandbox: sandbox(),
        comparison: comparison(SourceApplyComparisonState::Improved),
        guardrails: vec!["no trusted worktree mutation from the browser".to_string()],
    }
}

#[test]
fn controlled_failure_flow_uses_real_sandbox_file_and_links_before_after_comparison() {
    let sandbox_root = std::env::temp_dir().join(format!("ouroforge-v107-{}", std::process::id()));
    let _ = fs::remove_dir_all(&sandbox_root);
    fs::create_dir_all(&sandbox_root).expect("sandbox root");
    fs::write(
        sandbox_root.join("scene.json"),
        r#"{"player":{"x":0},"exit":false}"#,
    )
    .expect("real sandbox failure file");

    let evaluation = flow().evaluate(&sandbox_root);
    assert_eq!(
        evaluation.status,
        SourceApplyControlledFailureFlowStatus::ProductObservedReady
    );
    assert!(evaluation
        .evidence_summary
        .iter()
        .any(|line| line.contains("beforeBundle:runs/v107/before/manifest.json")));
    assert!(evaluation
        .evidence_summary
        .iter()
        .any(|line| line.contains("comparison:runs/v107/comparison.json")));
    assert!(evaluation
        .forbidden_actions
        .contains(&"trusted_worktree_apply".to_string()));

    fs::remove_dir_all(&sandbox_root).ok();
}

#[test]
fn self_approval_rejection_path_is_exercised_before_flow_passes() {
    let self_review = review("same-agent", "same-agent");
    let evaluation = self_review.evaluate();
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
fn missing_real_file_or_regression_blocks_product_observed_ready_claim() {
    let root = std::env::temp_dir().join("ouroforge-v107-missing");
    let mut missing = flow();
    let evaluation = missing.evaluate(&root);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("real sandbox file")));

    let root =
        std::env::temp_dir().join(format!("ouroforge-v107-regressed-{}", std::process::id()));
    fs::create_dir_all(&root).expect("sandbox root");
    fs::write(root.join("scene.json"), "{}").expect("real file");
    missing.comparison = comparison(SourceApplyComparisonState::Regressed);
    let evaluation = missing.evaluate(&root);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("regression")));
    fs::remove_dir_all(&root).ok();
}

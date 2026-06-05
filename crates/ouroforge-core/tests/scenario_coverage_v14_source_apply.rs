//! Scenario Coverage v14: Source Apply regression suite (#714).
//!
//! This file composes the existing source-apply safety gates — high-risk
//! blocker, review enforcement, sandbox promotion, rollback snapshot,
//! verification runner, rerun comparison, emergency hold, audit ledger, and
//! evidence bundle — into a single fixture-scoped scenario matrix so a critical
//! failure cannot hide inside one demo. It is read-only: it validates and
//! evaluates the gate contracts and adds no apply/merge/publish authority.

use ouroforge_core::{
    SourceApplyAuditApplyStatus, SourceApplyAuditEntry, SourceApplyAuditLedger,
    SourceApplyBundleComponentStatus, SourceApplyBundleStatus, SourceApplyComparisonDimension,
    SourceApplyComparisonState, SourceApplyEvidenceBundle, SourceApplyHighRiskBlocker,
    SourceApplyHighRiskStatus, SourceApplyHold, SourceApplyHoldQuery, SourceApplyHoldScope,
    SourceApplyHoldScopeKind, SourceApplyRerunComparison, SourceApplyRerunStatus,
    SourceApplyReviewDecisionState, SourceApplyReviewEnforcement,
    SourceApplyReviewEnforcementStatus, SourceApplyReviewTargetCoverage,
    SourceApplyRollbackSnapshot, SourceApplyRollbackStatus, SourceApplyRollbackTarget,
    SourceApplySandboxCleanupState, SourceApplySandboxCommandEvidence, SourceApplySandboxPromotion,
    SourceApplySandboxPromotionStatus, SourceApplySandboxReportState,
    SourceApplySandboxTargetExpectation, SourceApplyVerificationCommand,
    SourceApplyVerificationCommandStatus, SourceApplyVerificationPolicy,
    SourceApplyVerificationRun, SourceApplyVerificationStatus,
    SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION, SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION,
    SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION, SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION,
    SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION, SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION,
    SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION, SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION,
    SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION,
};
use serde_json::Value;

const COVERAGE_DOC: &str = include_str!("../../../docs/scenario-coverage-v14-source-apply.md");
const COVERAGE_FIXTURE: &str =
    include_str!("../../../examples/source-apply-regression-v14/coverage-matrix.fixture.json");

const SCENARIO_IDS: &[&str] = &[
    "SA14.success-valid-apply-transaction",
    "SA14.block-missing-review",
    "SA14.block-self-review",
    "SA14.block-stale-target",
    "SA14.block-sandbox-mismatch",
    "SA14.block-forbidden-file-class",
    "SA14.block-missing-rollback",
    "SA14.record-verification-failure",
    "SA14.record-rerun-regression",
    "SA14.block-emergency-hold",
    "SA14.success-audit-append-only",
    "SA14.success-evidence-bundle",
    "SA14.block-malformed-evidence",
    "SA14.success-read-only-inspection",
    "SA14.block-malformed-bundle-ref",
];

// ----- gate builders (typed, mirroring the per-gate contract tests) -----

fn review_target(path: &str) -> SourceApplyReviewTargetCoverage {
    SourceApplyReviewTargetCoverage {
        path: path.to_string(),
        before_hash: format!("before-{path}"),
        after_hash: format!("after-{path}"),
    }
}

fn accepted_review() -> SourceApplyReviewEnforcement {
    SourceApplyReviewEnforcement {
        schema_version: SOURCE_APPLY_REVIEW_ENFORCEMENT_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        patch_preview_id: "patch-preview-714-1".to_string(),
        expected_diff_hash: "diffhash-714-1".to_string(),
        transaction_base_revision: "base-rev-714-1".to_string(),
        expected_targets: vec![review_target("examples/source-apply-v1/sample-a.rs")],
        proposer_id: "agent-proposer".to_string(),
        reviewer_id: "agent-reviewer".to_string(),
        decision_state: SourceApplyReviewDecisionState::Accepted,
        decision_transaction_id: "apply-txn-714-1".to_string(),
        decision_preview_id: "patch-preview-714-1".to_string(),
        decision_diff_hash: "diffhash-714-1".to_string(),
        decision_base_revision: "base-rev-714-1".to_string(),
        decision_targets: vec![review_target("examples/source-apply-v1/sample-a.rs")],
        guardrails: vec![
            "review enforcement does not apply patches or merge branches".to_string(),
            "dashboard and Studio display this evaluation read-only".to_string(),
        ],
    }
}

fn sandbox_target(path: &str) -> SourceApplySandboxTargetExpectation {
    SourceApplySandboxTargetExpectation {
        path: path.to_string(),
        trusted_before_hash: format!("before-{path}"),
        expected_after_hash: format!("after-{path}"),
        sandbox_before_hash: format!("before-{path}"),
        sandbox_after_hash: format!("after-{path}"),
    }
}

fn sandbox_command(command: &str) -> SourceApplySandboxCommandEvidence {
    SourceApplySandboxCommandEvidence {
        command: command.to_string(),
        allowlist_policy_id: "source-patch-preview-safe-local-checks-v1".to_string(),
        status: "passed".to_string(),
    }
}

fn ready_sandbox() -> SourceApplySandboxPromotion {
    SourceApplySandboxPromotion {
        schema_version: SOURCE_APPLY_SANDBOX_PROMOTION_SCHEMA_VERSION.to_string(),
        patch_preview_id: "patch-preview-714-1".to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        expected_diff_hash: "diffhash-714-1".to_string(),
        sandbox_diff_hash: "diffhash-714-1".to_string(),
        transaction_base_revision: "base-rev-714-1".to_string(),
        sandbox_base_revision: "base-rev-714-1".to_string(),
        report_state: SourceApplySandboxReportState::Passed,
        targets: vec![sandbox_target("examples/source-apply-v1/sample-a.rs")],
        allowlisted_commands: vec![sandbox_command("cargo fmt --check")],
        forbidden_commands_observed: Vec::new(),
        cleanup_state: SourceApplySandboxCleanupState::Complete,
        generated_state_isolated: true,
        guardrails: vec![
            "sandbox promotion does not apply patches or guarantee a secure sandbox".to_string(),
            "dashboard and Studio display this evaluation read-only".to_string(),
        ],
    }
}

fn rollback_target(path: &str, recoverable: bool) -> SourceApplyRollbackTarget {
    SourceApplyRollbackTarget {
        path: path.to_string(),
        before_content_hash: format!("before-{path}"),
        expected_after_hash: format!("after-{path}"),
        before_content_ref: recoverable.then(|| format!("rollback/{path}.before")),
        reverse_patch_ref: None,
    }
}

fn rollback_snapshot(recoverable: bool) -> SourceApplyRollbackSnapshot {
    SourceApplyRollbackSnapshot {
        schema_version: SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        transaction_base_revision: "base-rev-714-1".to_string(),
        snapshot_base_revision: "base-rev-714-1".to_string(),
        actor: "agent-applier".to_string(),
        recorded_at: "2026-06-06T00:00:00Z".to_string(),
        targets: vec![rollback_target(
            "examples/source-apply-v1/sample-a.rs",
            recoverable,
        )],
        recovery_guidance: vec![
            "copy rollback/<path>.before back over the target to restore before-state".to_string(),
        ],
        guardrails: vec![
            "rollback snapshot records metadata only and does not auto-restore".to_string(),
            "dashboard and Studio display this snapshot read-only".to_string(),
        ],
    }
}

fn verification_policy() -> SourceApplyVerificationPolicy {
    SourceApplyVerificationPolicy {
        max_commands: 8,
        timeout_seconds: 600,
        max_output_bytes: 1_000_000,
        log_root: "runs/source-apply-verification".to_string(),
    }
}

fn verification_command(
    argv: &[&str],
    status: SourceApplyVerificationCommandStatus,
) -> SourceApplyVerificationCommand {
    SourceApplyVerificationCommand {
        argv: argv.iter().map(|a| a.to_string()).collect(),
        allowlist_policy_id: "source-patch-preview-safe-local-checks-v1".to_string(),
        status,
        duration_seconds: 30,
        output_bytes: 2048,
        log_ref: "runs/source-apply-verification/log-1.txt".to_string(),
    }
}

fn verification_run(commands: Vec<SourceApplyVerificationCommand>) -> SourceApplyVerificationRun {
    SourceApplyVerificationRun {
        schema_version: SOURCE_APPLY_VERIFICATION_RUN_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        policy: verification_policy(),
        commands,
        guardrails: vec![
            "verification runner is allowlisted, bounded, and not a general command bridge"
                .to_string(),
        ],
    }
}

fn rerun_dimension(
    name: &str,
    state: SourceApplyComparisonState,
) -> SourceApplyComparisonDimension {
    SourceApplyComparisonDimension {
        name: name.to_string(),
        state,
        detail: format!("{name} dimension detail"),
    }
}

fn rerun_comparison(states: Vec<SourceApplyComparisonState>) -> SourceApplyRerunComparison {
    SourceApplyRerunComparison {
        schema_version: SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        review_decision_ref: "evidence/review-decision.json".to_string(),
        verification_log_ref: "runs/verification/log.json".to_string(),
        rollback_snapshot_ref: "evidence/rollback-snapshot.json".to_string(),
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        qa_backlog_refs: vec!["evidence/qa-backlog/item-1.json".to_string()],
        rerun_status: SourceApplyRerunStatus::Completed,
        before_run_id: Some("run-before-1".to_string()),
        after_run_id: Some("run-after-1".to_string()),
        generated_output_root: "runs/post-apply".to_string(),
        dimensions: states
            .into_iter()
            .enumerate()
            .map(|(i, state)| rerun_dimension(&format!("dimension-{i}"), state))
            .collect(),
        guardrails: vec![
            "rerun comparison evaluates evidence and never applies or runs commands".to_string(),
            "promotion claims require evidence".to_string(),
        ],
    }
}

fn audit_entry(attempt: &str, status: SourceApplyAuditApplyStatus) -> SourceApplyAuditEntry {
    SourceApplyAuditEntry {
        attempt_id: attempt.to_string(),
        transaction_id: "apply-txn-714-1".to_string(),
        actor: "agent-applier".to_string(),
        recorded_at: "2026-06-06T00:00:00Z".to_string(),
        review_decision_ref: "evidence/review-decision.json".to_string(),
        sandbox_report_ref: "sandbox/report.json".to_string(),
        stale_guard_ref: "evidence/stale-guard.json".to_string(),
        rollback_snapshot_ref: "evidence/rollback-snapshot.json".to_string(),
        verification_log_ref: "runs/verification/log.json".to_string(),
        rerun_comparison_ref: None,
        apply_status: status,
        failure_reason: None,
        blocked_reasons: match status {
            SourceApplyAuditApplyStatus::Blocked | SourceApplyAuditApplyStatus::Held => {
                vec!["precondition not met".to_string()]
            }
            _ => Vec::new(),
        },
    }
}

fn audit_ledger(entries: Vec<SourceApplyAuditEntry>) -> SourceApplyAuditLedger {
    SourceApplyAuditLedger {
        schema_version: SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION.to_string(),
        ledger_id: "audit-ledger-714".to_string(),
        entries,
        guardrails: vec![
            "audit ledger records attempts and never applies or rolls back".to_string(),
            "recorded history is append-only".to_string(),
        ],
    }
}

fn evidence_component(name: &str, status: &str) -> SourceApplyBundleComponentStatus {
    SourceApplyBundleComponentStatus {
        component: name.to_string(),
        status: status.to_string(),
    }
}

fn evidence_bundle(
    rollback_resolved: bool,
    regression_resolved: bool,
    component_statuses: Vec<SourceApplyBundleComponentStatus>,
    declared_final_status: SourceApplyBundleStatus,
) -> SourceApplyEvidenceBundle {
    SourceApplyEvidenceBundle {
        schema_version: SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION.to_string(),
        bundle_id: "bundle-714-1".to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        preview_ref: "mutation/source-patch-previews/preview.json".to_string(),
        file_class_report_ref: "evidence/file-class-report.json".to_string(),
        diff_integrity_report_ref: "evidence/diff-integrity.json".to_string(),
        sandbox_report_ref: "sandbox/report.json".to_string(),
        review_decision_ref: "evidence/review-decision.json".to_string(),
        apply_transaction_ref: "evidence/apply-transaction.json".to_string(),
        worktree_context_ref: "evidence/worktree-context.json".to_string(),
        stale_guard_ref: "evidence/stale-guard.json".to_string(),
        sandbox_promotion_ref: "evidence/sandbox-promotion.json".to_string(),
        rollback_snapshot_ref: "evidence/rollback-snapshot.json".to_string(),
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        verification_log_refs: vec!["runs/verification/log.json".to_string()],
        rerun_comparison_ref: Some("evidence/rerun-comparison.json".to_string()),
        blocker_evidence_ref: None,
        rollback_resolved,
        regression_resolved,
        component_statuses,
        declared_final_status,
        guardrails: vec![
            "evidence bundle aggregates read-only and applies nothing".to_string(),
            "dashboard and Studio inspect this bundle read-only".to_string(),
        ],
    }
}

// ----- matrix / governance -----

fn coverage_fixture() -> Value {
    serde_json::from_str(COVERAGE_FIXTURE).expect("coverage fixture parses")
}

#[test]
fn scenario_coverage_v14_matrix_declares_every_gate_scenario() {
    let fixture = coverage_fixture();
    assert_eq!(
        fixture["schemaVersion"],
        "scenario-coverage-v14-source-apply-v1"
    );
    assert_eq!(fixture["issue"], 714);
    let scenarios = fixture["scenarios"]
        .as_array()
        .expect("scenarios are an array");
    let ids: Vec<&str> = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("scenario id"))
        .collect();
    for required in SCENARIO_IDS {
        assert!(
            ids.contains(required),
            "fixture missing scenario {required}"
        );
        assert!(COVERAGE_DOC.contains(required), "doc missing {required}");
    }
    let success = scenarios.iter().filter(|s| s["kind"] == "success").count();
    let blocked = scenarios.iter().filter(|s| s["kind"] == "blocked").count();
    let record = scenarios.iter().filter(|s| s["kind"] == "record").count();
    assert!(success >= 4, "expected success scenarios, got {success}");
    assert!(blocked >= 8, "expected blocked scenarios, got {blocked}");
    assert!(record >= 2, "expected recorded scenarios, got {record}");
}

#[test]
fn scenario_coverage_v14_documents_boundaries_and_governance() {
    let lower_doc = COVERAGE_DOC.to_ascii_lowercase();
    let lower_fixture = COVERAGE_FIXTURE.to_ascii_lowercase();
    for term in [
        "read-only",
        "fixture-scoped",
        "generated",
        "command bridge",
        "credential",
        "production-ready",
        "godot replacement",
        "deploy",
        "sign",
        "upload",
    ] {
        assert!(lower_doc.contains(term), "doc missing guardrail {term}");
        assert!(
            lower_fixture.contains(term),
            "fixture missing guardrail {term}"
        );
    }
    assert!(COVERAGE_DOC.contains("#1 remains open"));
    assert!(COVERAGE_DOC.contains("#23 remains open"));
}

#[test]
fn sa14_success_valid_apply_transaction() {
    let blocker = SourceApplyHighRiskBlocker {
        schema_version: SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        candidate_targets: vec!["examples/source-apply-v1/sample-a.rs".to_string()],
        guardrails: vec!["block high-risk classes".to_string()],
    };
    blocker.validate().expect("blocker validates");
    assert!(blocker.is_allowed());
    assert_eq!(
        blocker.evaluate().status,
        SourceApplyHighRiskStatus::Allowed
    );

    let review = accepted_review();
    review.validate().expect("review validates");
    assert_eq!(
        review.evaluate().status,
        SourceApplyReviewEnforcementStatus::Ready
    );
    assert!(review.is_ready());

    let sandbox = ready_sandbox();
    sandbox.validate().expect("sandbox validates");
    assert_eq!(
        sandbox.evaluate().status,
        SourceApplySandboxPromotionStatus::Ready
    );
}

#[test]
fn sa14_block_missing_and_self_review() {
    let mut missing = accepted_review();
    missing.decision_state = SourceApplyReviewDecisionState::Missing;
    assert!(!missing.is_ready(), "missing review must block apply");

    let mut self_review = accepted_review();
    self_review.reviewer_id = self_review.proposer_id.clone();
    assert!(!self_review.is_ready(), "self review must block apply");
}

#[test]
fn sa14_block_stale_target_and_sandbox_mismatch() {
    let mut stale = ready_sandbox();
    stale.sandbox_base_revision = "base-rev-DRIFTED".to_string();
    assert_ne!(
        stale.evaluate().status,
        SourceApplySandboxPromotionStatus::Ready,
        "stale base revision must block promotion"
    );

    let mut mismatch = ready_sandbox();
    mismatch.targets[0].sandbox_after_hash = "after-DIFFERENT".to_string();
    assert_ne!(
        mismatch.evaluate().status,
        SourceApplySandboxPromotionStatus::Ready,
        "sandbox/expected hash mismatch must block promotion"
    );
}

#[test]
fn sa14_block_forbidden_file_class() {
    let blocker = SourceApplyHighRiskBlocker {
        schema_version: SOURCE_APPLY_HIGH_RISK_BLOCKER_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-714-1".to_string(),
        candidate_targets: vec!["Cargo.lock".to_string()],
        guardrails: vec!["block high-risk classes".to_string()],
    };
    blocker.validate().expect("blocker validates structurally");
    assert!(!blocker.is_allowed(), "lockfile is a forbidden class");
    let evaluation = blocker.evaluate();
    assert_eq!(evaluation.status, SourceApplyHighRiskStatus::Blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("lockfile")));
}

#[test]
fn sa14_block_missing_rollback() {
    let complete = rollback_snapshot(true);
    complete.validate().expect("rollback validates");
    assert_eq!(
        complete.evaluate().status,
        SourceApplyRollbackStatus::Complete
    );
    assert!(complete.is_complete());

    let missing = rollback_snapshot(false);
    assert!(
        !missing.is_complete(),
        "no recovery reference must leave a recovery gap"
    );
}

#[test]
fn sa14_record_verification_failure() {
    let passing = verification_run(vec![verification_command(
        &["cargo", "fmt", "--check"],
        SourceApplyVerificationCommandStatus::Passed,
    )]);
    passing.validate().expect("run validates");
    assert_eq!(
        passing.evaluate().status,
        SourceApplyVerificationStatus::Passed
    );
    assert!(passing.is_passed());

    let failing = verification_run(vec![
        verification_command(
            &["cargo", "fmt", "--check"],
            SourceApplyVerificationCommandStatus::Passed,
        ),
        verification_command(
            &["cargo", "test", "-p", "ouroforge-core"],
            SourceApplyVerificationCommandStatus::Failed,
        ),
    ]);
    assert!(
        !failing.is_passed(),
        "a failed allowlisted command must be recorded, not hidden"
    );
}

#[test]
fn sa14_record_rerun_regression() {
    let improved = rerun_comparison(vec![
        SourceApplyComparisonState::Improved,
        SourceApplyComparisonState::Unchanged,
    ]);
    improved.validate().expect("rerun validates");
    assert!(improved.promotion_claim_allowed());

    let regressed = rerun_comparison(vec![
        SourceApplyComparisonState::Improved,
        SourceApplyComparisonState::Regressed,
    ]);
    assert!(
        !regressed.promotion_claim_allowed(),
        "a regression must block a promotion claim"
    );
}

#[test]
fn sa14_block_emergency_hold() {
    let hold = SourceApplyHold {
        schema_version: SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION.to_string(),
        hold_id: "hold-714-1".to_string(),
        disabled: true,
        reason: "risk detected in source apply pipeline".to_string(),
        actor: "agent-operator".to_string(),
        recorded_at: "2026-06-06T00:00:00Z".to_string(),
        expires_at: None,
        scopes: vec![SourceApplyHoldScope {
            kind: SourceApplyHoldScopeKind::Global,
            value: String::new(),
        }],
        requires_review_to_lift: true,
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        guardrails: vec![
            "hold blocks source apply locally and cannot be bypassed".to_string(),
            "no remote kill switch or cloud control plane".to_string(),
        ],
    };
    hold.validate().expect("hold validates");
    let query = SourceApplyHoldQuery {
        now: "2026-06-06T01:00:00Z".to_string(),
        transaction_id: "apply-txn-714-1".to_string(),
        file_classes: vec!["rust-source".to_string()],
        force_apply: false,
    };
    let evaluation = hold.evaluate_against(&query);
    assert!(evaluation.active);
    assert!(evaluation.apply_blocked, "a global hold must block apply");
}

#[test]
fn sa14_success_audit_append_only() {
    let base = audit_ledger(vec![audit_entry(
        "attempt-1",
        SourceApplyAuditApplyStatus::Applied,
    )]);
    base.validate().expect("ledger validates");
    let appended = base
        .append_entry(audit_entry(
            "attempt-2",
            SourceApplyAuditApplyStatus::Blocked,
        ))
        .expect("append succeeds");
    appended
        .validate_is_append_of(&base)
        .expect("valid append of base");
    assert!(appended.read_model().append_only);

    let rewritten = audit_ledger(vec![
        audit_entry("attempt-rewritten", SourceApplyAuditApplyStatus::Applied),
        audit_entry("attempt-2", SourceApplyAuditApplyStatus::Blocked),
    ]);
    assert!(
        rewritten.validate_is_append_of(&base).is_err(),
        "rewritten history must fail closed"
    );
}

#[test]
fn sa14_success_evidence_bundle_and_malformed() {
    let complete = evidence_bundle(
        true,
        true,
        vec![
            evidence_component("review", "ok"),
            evidence_component("sandbox", "ok"),
        ],
        SourceApplyBundleStatus::Complete,
    );
    complete.validate().expect("bundle validates");
    assert!(complete.is_complete());

    let malformed = evidence_bundle(
        false,
        false,
        vec![
            evidence_component("review", "ok"),
            evidence_component("verification", "failed"),
        ],
        SourceApplyBundleStatus::Complete,
    );
    assert!(
        !malformed.is_complete(),
        "unresolved rollback and failed component must not be complete"
    );

    let mut missing_ref = evidence_bundle(
        true,
        true,
        vec![evidence_component("review", "ok")],
        SourceApplyBundleStatus::Complete,
    );
    missing_ref.review_decision_ref = String::new();
    assert!(
        missing_ref.validate().is_err(),
        "a missing required reference must fail structural validation"
    );
}

#[test]
fn sa14_success_read_only_inspection_has_no_apply_authority() {
    let review_eval =
        serde_json::to_value(accepted_review().evaluate()).expect("review evaluation serializes");
    assert!(review_eval.get("applyCommand").is_none());
    assert!(review_eval.get("mergeCommand").is_none());

    let rollback_eval = rollback_snapshot(true).evaluate();
    for forbidden in [
        "apply_patch",
        "merge_branch",
        "execute_command",
        "auto_restore",
    ] {
        assert!(
            rollback_eval
                .forbidden_actions
                .iter()
                .any(|action| action == forbidden),
            "rollback evaluation must forbid {forbidden}"
        );
    }
}

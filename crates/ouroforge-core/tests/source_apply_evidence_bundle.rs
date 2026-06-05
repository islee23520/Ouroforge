use ouroforge_core::{
    SourceApplyBundleComponentStatus, SourceApplyBundleStatus, SourceApplyEvidenceBundle,
    SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION,
};

fn component(name: &str, status: &str) -> SourceApplyBundleComponentStatus {
    SourceApplyBundleComponentStatus {
        component: name.to_string(),
        status: status.to_string(),
    }
}

fn complete_fixture() -> SourceApplyEvidenceBundle {
    SourceApplyEvidenceBundle {
        schema_version: SOURCE_APPLY_EVIDENCE_BUNDLE_SCHEMA_VERSION.to_string(),
        bundle_id: "bundle-711-1".to_string(),
        apply_transaction_id: "apply-txn-711-1".to_string(),
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
        rollback_resolved: true,
        regression_resolved: true,
        component_statuses: vec![component("review", "ok"), component("sandbox", "ok")],
        declared_final_status: SourceApplyBundleStatus::Complete,
        guardrails: vec![
            "evidence bundle aggregates read-only and applies nothing".to_string(),
            "dashboard and Studio inspect this bundle read-only".to_string(),
        ],
    }
}

#[test]
fn complete_bundle_is_auditable_end_to_end() {
    let bundle = complete_fixture();
    bundle.validate().expect("valid");
    let evaluation = bundle.evaluate();
    assert_eq!(
        evaluation.computed_status,
        SourceApplyBundleStatus::Complete
    );
    assert!(evaluation.status_consistent);
    assert!(evaluation.issues.is_empty());
    assert!(bundle.is_complete());
    assert!(evaluation
        .forbidden_actions
        .iter()
        .any(|a| a == "apply_patch"));
}

#[test]
fn missing_verification_is_surfaced() {
    let mut bundle = complete_fixture();
    bundle.verification_log_refs.clear();
    bundle.declared_final_status = SourceApplyBundleStatus::Partial;
    let evaluation = bundle.evaluate();
    assert!(evaluation
        .issues
        .iter()
        .any(|i| i.contains("missing verification")));
    assert_ne!(
        evaluation.computed_status,
        SourceApplyBundleStatus::Complete
    );
}

#[test]
fn unresolved_rollback_gap_blocks_bundle() {
    let mut bundle = complete_fixture();
    bundle.rollback_resolved = false;
    bundle.declared_final_status = SourceApplyBundleStatus::Blocked;
    let evaluation = bundle.evaluate();
    assert_eq!(evaluation.computed_status, SourceApplyBundleStatus::Blocked);
    assert!(evaluation
        .issues
        .iter()
        .any(|i| i.contains("unresolved rollback")));
}

#[test]
fn unresolved_regression_blocks_bundle() {
    let mut bundle = complete_fixture();
    bundle.regression_resolved = false;
    bundle.declared_final_status = SourceApplyBundleStatus::Blocked;
    let evaluation = bundle.evaluate();
    assert!(evaluation
        .issues
        .iter()
        .any(|i| i.contains("unresolved regression")));
}

#[test]
fn stale_component_is_flagged() {
    let mut bundle = complete_fixture();
    bundle.component_statuses = vec![component("sandbox", "stale")];
    bundle.declared_final_status = SourceApplyBundleStatus::Blocked;
    let evaluation = bundle.evaluate();
    assert!(evaluation
        .issues
        .iter()
        .any(|i| i.contains("stale artifact")));
}

#[test]
fn failed_component_makes_bundle_failed() {
    let mut bundle = complete_fixture();
    bundle.component_statuses = vec![component("verification", "failed")];
    bundle.declared_final_status = SourceApplyBundleStatus::Failed;
    let evaluation = bundle.evaluate();
    assert_eq!(evaluation.computed_status, SourceApplyBundleStatus::Failed);
}

#[test]
fn declared_status_mismatch_is_inconsistent() {
    let mut bundle = complete_fixture();
    bundle.rollback_resolved = false; // implies blocked
    bundle.declared_final_status = SourceApplyBundleStatus::Complete; // but declares complete
    let evaluation = bundle.evaluate();
    assert!(!evaluation.status_consistent);
    assert!(evaluation
        .issues
        .iter()
        .any(|i| i.contains("inconsistent status")));
}

#[test]
fn missing_required_ref_fails_validation() {
    let mut bundle = complete_fixture();
    bundle.sandbox_report_ref = String::new();
    assert!(bundle.validate().is_err());
}

#[test]
fn malformed_bundle_json_is_rejected() {
    let malformed = r#"{"schemaVersion":"source-apply-evidence-bundle-v1","unexpected":1}"#;
    assert!(SourceApplyEvidenceBundle::from_json_str(malformed).is_err());
}

#[test]
fn json_round_trip_preserves_bundle() {
    let bundle = complete_fixture();
    let json = serde_json::to_string_pretty(&bundle).expect("serializes");
    let parsed = SourceApplyEvidenceBundle::from_json_str(&json).expect("parses");
    assert_eq!(parsed, bundle);
}

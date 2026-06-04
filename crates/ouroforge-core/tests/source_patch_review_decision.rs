use ouroforge_core::{
    inspect_source_patch_review_decision_link, source_patch_review_read_model,
    validate_source_patch_review_decision_link, SourcePatchPreviewEvidenceRef,
    SourcePatchPreviewRequiredTest, SourcePatchReviewDecisionLink, SourcePatchReviewStatus,
    SOURCE_PATCH_REVIEW_DECISION_SCHEMA_VERSION,
};

fn required_test() -> SourcePatchPreviewRequiredTest {
    SourcePatchPreviewRequiredTest {
        command: "cargo test -p ouroforge-core source_patch_review_decision".to_string(),
        argv: vec![
            "cargo".to_string(),
            "test".to_string(),
            "-p".to_string(),
            "ouroforge-core".to_string(),
            "source_patch_review_decision".to_string(),
        ],
        allowlist_policy_id: Some("source-patch-preview-safe-local-checks-v1".to_string()),
        execution_authority: "copyable_only_not_executed_by_preview".to_string(),
    }
}

fn fixture_link(status: SourcePatchReviewStatus) -> SourcePatchReviewDecisionLink {
    SourcePatchReviewDecisionLink {
        schema_version: SOURCE_PATCH_REVIEW_DECISION_SCHEMA_VERSION.to_string(),
        review_decision_id: "review-decision-361-smp1-7-1".to_string(),
        patch_preview_id: "patch-preview-361-smp1-7-1".to_string(),
        status,
        linked_evidence: vec![SourcePatchPreviewEvidenceRef {
            kind: "preview".to_string(),
            path: "mutation/source-patch-previews/patch-preview-361-smp1-7-1.json".to_string(),
        }],
        file_class_report_ref: "evidence/source-file-class-report.json".to_string(),
        diff_integrity_report_ref: "evidence/patch-diff-integrity-report.json".to_string(),
        sandbox_report_ref: "sandbox/patch-preview-361-smp1-7-1/evidence/report.json".to_string(),
        required_tests: vec![required_test()],
        blocked_reasons: Vec::new(),
        guardrails: vec![
            "review decisions do not apply source patches or merge branches".to_string(),
            "dashboard and Studio display this linkage read-only".to_string(),
        ],
    }
}

#[test]
fn source_patch_review_statuses_round_trip_without_apply_authority() {
    let statuses = [
        (SourcePatchReviewStatus::Proposed, "proposed"),
        (SourcePatchReviewStatus::Blocked, "blocked"),
        (SourcePatchReviewStatus::Reviewed, "reviewed"),
        (SourcePatchReviewStatus::Rejected, "rejected"),
        (SourcePatchReviewStatus::Deferred, "deferred"),
    ];

    for (status, expected) in statuses {
        let mut link = fixture_link(status);
        if expected == "blocked" {
            link.blocked_reasons = vec!["waiting for complete sandbox evidence".to_string()];
        }
        let value = serde_json::to_value(&link).expect("link serializes");
        assert_eq!(value["status"], expected);
        assert!(value.get("applyCommand").is_none());
        assert!(value.get("mergeCommand").is_none());
        validate_source_patch_review_decision_link(&link).expect("valid review link passes");
    }
}

#[test]
fn source_patch_review_link_requires_all_evidence_refs() {
    let mut link = fixture_link(SourcePatchReviewStatus::Reviewed);
    link.diff_integrity_report_ref.clear();
    link.required_tests.clear();

    let validation = inspect_source_patch_review_decision_link(&link);
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("diffIntegrityReportRef")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("requiredTests")));
}

#[test]
fn source_patch_review_link_blocks_apply_or_command_authority_drift() {
    let mut link = fixture_link(SourcePatchReviewStatus::Reviewed);
    link.guardrails = vec!["review accepted".to_string()];
    link.required_tests[0].command = "git apply patch.diff".to_string();
    link.required_tests[0].argv = vec![
        "git".to_string(),
        "apply".to_string(),
        "patch.diff".to_string(),
    ];

    let error = validate_source_patch_review_decision_link(&link)
        .expect_err("apply-like review linkage must be blocked");
    let message = error.to_string();
    assert!(message.contains("source patch apply"));
    assert!(message.contains("do not apply"));
}

#[test]
fn source_patch_review_read_model_is_display_only() {
    let link = fixture_link(SourcePatchReviewStatus::Reviewed);
    let model = source_patch_review_read_model(&link);
    assert_eq!(model.schema_version, "source-patch-review-read-model-v1");
    assert_eq!(model.status, "passed");
    assert!(model
        .evidence_summary
        .iter()
        .any(|entry| entry.starts_with("file-class:")));
    assert!(model
        .allowed_actions
        .iter()
        .any(|action| action == "inspect_review_evidence"));
    assert!(model
        .forbidden_actions
        .iter()
        .any(|action| action == "apply_patch"));
    assert!(model
        .forbidden_actions
        .iter()
        .any(|action| action == "merge_branch"));
    assert!(model
        .forbidden_actions
        .iter()
        .any(|action| action == "execute_command"));
}

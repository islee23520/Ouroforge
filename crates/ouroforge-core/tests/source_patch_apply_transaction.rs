use ouroforge_core::{
    inspect_source_patch_apply_transaction_artifact,
    validate_source_patch_apply_transaction_artifact, SourcePatchApplyTransactionArtifact,
    SourcePatchApplyTransactionStatus, SOURCE_PATCH_APPLY_TRANSACTION_SCHEMA_VERSION,
};
use serde_json::json;

fn fixture() -> SourcePatchApplyTransactionArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/source-patch-apply-transaction-v1/apply-transaction.sample.json"
    ))
    .expect("fixture deserializes")
}

#[test]
fn source_patch_apply_transaction_round_trips_fixture_without_apply_authority() {
    let artifact = fixture();

    assert_eq!(
        artifact.schema_version,
        SOURCE_PATCH_APPLY_TRANSACTION_SCHEMA_VERSION
    );
    assert_eq!(
        artifact.status,
        SourcePatchApplyTransactionStatus::ReadyForTrustedApply
    );
    assert_eq!(
        artifact.evidence.patch_preview_id,
        "patch-preview-702-sa15-4-1"
    );
    assert_eq!(artifact.targets.len(), 1);
    assert_eq!(artifact.verification_commands.len(), 1);
    assert!(artifact
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("do not apply patches")));

    let value = serde_json::to_value(&artifact).expect("artifact serializes");
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("mergeCommand").is_none());
    assert!(value.get("browserCommandBridge").is_none());
    assert_eq!(
        value["verificationCommands"][0]["executionAuthority"],
        "copyable_not_executed_by_transaction_artifact"
    );

    let round_trip: SourcePatchApplyTransactionArtifact =
        serde_json::from_value(value).expect("serialized artifact shape remains valid");
    assert_eq!(round_trip, artifact);
}

#[test]
fn source_patch_apply_transaction_schema_rejects_unknown_apply_command_fields() {
    let mut value = serde_json::to_value(fixture()).expect("fixture serializes");
    value["applyCommand"] = json!("git apply patch.diff");

    let error = serde_json::from_value::<SourcePatchApplyTransactionArtifact>(value)
        .expect_err("unknown apply authority field must be rejected by schema");
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn source_patch_apply_transaction_models_required_evidence_links() {
    let artifact = fixture();

    assert!(!artifact.evidence.patch_preview_ref.is_empty());
    assert!(!artifact.evidence.sandbox_report_ref.is_empty());
    assert!(!artifact.evidence.review_decision_ref.is_empty());
    assert!(!artifact.evidence.file_class_report_ref.is_empty());
    assert!(!artifact.evidence.diff_integrity_report_ref.is_empty());
    assert_eq!(
        artifact.diff_summary.diff_integrity_report_ref,
        artifact.evidence.diff_integrity_report_ref
    );
    assert_eq!(
        artifact.rollback_ref.target_before_hashes[0].before_hash,
        artifact.targets[0].before_hash
    );
}

#[test]
fn source_patch_apply_transaction_validation_passes_ready_fixture() {
    let artifact = fixture();

    let validation = validate_source_patch_apply_transaction_artifact(&artifact)
        .expect("complete ready transaction validates");

    assert_eq!(validation.status, "passed");
    assert_eq!(validation.target_count, 1);
    assert_eq!(validation.verification_command_count, 1);
    assert!(validation.blocked_reasons.is_empty());
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("does not apply patches")));
}

#[test]
fn source_patch_apply_transaction_validation_blocks_missing_core_evidence_refs() {
    let mut artifact = fixture();
    artifact.evidence.patch_preview_ref.clear();
    artifact.evidence.review_decision_ref.clear();
    artifact.evidence.sandbox_report_ref.clear();

    let validation = inspect_source_patch_apply_transaction_artifact(&artifact);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("evidence.patchPreviewRef")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("evidence.reviewDecisionRef")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("evidence.sandboxReportRef")));
}

#[test]
fn source_patch_apply_transaction_validation_blocks_unsafe_duplicate_and_unsupported_targets() {
    let mut artifact = fixture();
    artifact.targets.push(artifact.targets[0].clone());
    artifact.targets[0].path = "../Cargo.lock".to_string();
    artifact.targets[0].file_class = "dependency_manifest".to_string();
    artifact.targets[1].path = "../Cargo.lock".to_string();
    artifact.rollback_ref.target_before_hashes[0].path = "../Cargo.lock".to_string();
    artifact.diff_summary.files_changed = artifact.targets.len();

    let validation = inspect_source_patch_apply_transaction_artifact(&artifact);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("must not escape")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("unsupported file class decision")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("duplicate target path")));
}

#[test]
fn source_patch_apply_transaction_validation_blocks_malformed_verification_commands() {
    let mut artifact = fixture();
    artifact.verification_commands[0].command = "git apply patch.diff".to_string();
    artifact.verification_commands[0].argv = vec![
        "git".to_string(),
        "apply".to_string(),
        "patch.diff".to_string(),
    ];
    artifact.verification_commands[0].execution_authority =
        "execute_in_trusted_worktree".to_string();

    let validation = inspect_source_patch_apply_transaction_artifact(&artifact);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("executionAuthority")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("source patch apply")));
}

#[test]
fn source_patch_apply_transaction_validation_blocks_stale_and_rollback_mismatch() {
    let mut artifact = fixture();
    artifact.base_ref.stale_target_policy = "apply immediately".to_string();
    artifact.rollback_ref.pre_apply_commit = "different-commit".to_string();
    artifact.rollback_ref.target_before_hashes[0].before_hash =
        "sha256:3333333333333333333333333333333333333333333333333333333333333333".to_string();

    let validation = inspect_source_patch_apply_transaction_artifact(&artifact);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("staleTargetPolicy")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("preApplyCommit")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("must match target beforeHash")));
}

#[test]
fn source_patch_apply_transaction_validation_reports_blocked_transactions() {
    let mut artifact = fixture();
    artifact.status = SourcePatchApplyTransactionStatus::Blocked;
    artifact.blocked_reasons = vec!["waiting for independent review".to_string()];

    let error = validate_source_patch_apply_transaction_artifact(&artifact)
        .expect_err("blocked transactions are not trusted-apply ready");

    assert!(error.to_string().contains("transaction status is blocked"));
}

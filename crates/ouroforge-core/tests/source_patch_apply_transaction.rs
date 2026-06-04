use ouroforge_core::{
    inspect_source_patch_apply_transaction_artifact,
    inspect_source_patch_apply_transaction_artifact_with_evidence_root,
    source_patch_apply_transaction_read_model, validate_source_patch_apply_transaction_artifact,
    write_source_patch_apply_transaction_artifact, SourcePatchApplyTransactionArtifact,
    SourcePatchApplyTransactionStatus, SOURCE_PATCH_APPLY_TRANSACTION_SCHEMA_VERSION,
};
use serde_json::json;

fn fixture() -> SourcePatchApplyTransactionArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/source-patch-apply-transaction-v1/apply-transaction.sample.json"
    ))
    .expect("fixture deserializes")
}

fn unique_run_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::create_dir_all(dir.join("evidence")).expect("create evidence dir");
    dir
}

fn write_json_at(root: &std::path::Path, relative: &str, value: serde_json::Value) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().expect("fixture path parent")).expect("create parent");
    std::fs::write(
        path,
        serde_json::to_string_pretty(&value).expect("serialize fixture"),
    )
    .expect("write fixture json");
}

fn write_linked_evidence_fixtures(
    root: &std::path::Path,
    artifact: &SourcePatchApplyTransactionArtifact,
) {
    write_json_at(
        root,
        &artifact.evidence.patch_preview_ref,
        json!({"patchPreviewId": artifact.evidence.patch_preview_id, "status": "passed"}),
    );
    write_json_at(
        root,
        &artifact.evidence.sandbox_report_ref,
        json!({"sandboxReportId": artifact.evidence.sandbox_report_id, "status": "passed"}),
    );
    write_json_at(
        root,
        &artifact.evidence.review_decision_ref,
        json!({"reviewDecisionId": artifact.evidence.review_decision_id, "status": "accepted"}),
    );
    write_json_at(
        root,
        &artifact.evidence.file_class_report_ref,
        json!({"fileClassReportId": artifact.evidence.file_class_report_id, "status": "passed"}),
    );
    write_json_at(
        root,
        &artifact.evidence.diff_integrity_report_ref,
        json!({"diffIntegrityReportId": artifact.evidence.diff_integrity_report_id, "status": "passed"}),
    );
}

fn write_minimal_dashboard_run(run_dir: &std::path::Path) {
    std::fs::write(
        run_dir.join("run.json"),
        serde_json::json!({"id":"run-source-patch-apply-transaction","created_at_unix_ms":1})
            .to_string(),
    )
    .expect("write run");
    std::fs::write(
        run_dir.join("verdict.json"),
        serde_json::json!({"status":"passed"}).to_string(),
    )
    .expect("write verdict");
    std::fs::create_dir_all(run_dir.join("evidence")).expect("evidence dir");
    std::fs::write(
        run_dir.join("evidence/index.json"),
        serde_json::json!({"artifacts":[]}).to_string(),
    )
    .expect("write evidence index");
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

    assert_eq!(validation.status, "shape_valid_pending_linked_evidence");
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

#[test]
fn source_patch_apply_transaction_read_model_is_display_only() {
    let artifact = fixture();

    let model = source_patch_apply_transaction_read_model(&artifact);

    assert_eq!(
        model.schema_version,
        "source-patch-apply-transaction-read-model-v1"
    );
    assert_eq!(model.status, "shape_valid_pending_linked_evidence");
    assert_eq!(
        model.readiness_label,
        "shape_valid_pending_linked_evidence_no_apply_authority"
    );
    assert!(model
        .evidence_summary
        .iter()
        .any(|entry| entry.starts_with("sandbox:")));
    assert!(model
        .target_summaries
        .iter()
        .any(|entry| entry.contains("scenario_regression_fixture")));
    assert!(model
        .allowed_actions
        .iter()
        .any(|action| action == "inspect_transaction_evidence"));
    for forbidden in [
        "apply_patch",
        "merge_branch",
        "execute_command",
        "write_trusted_file",
        "browser_command_bridge",
    ] {
        assert!(model
            .forbidden_actions
            .iter()
            .any(|action| action == forbidden));
    }
}

#[test]
fn source_patch_apply_transaction_strictly_rejects_malformed_hashes() {
    let mut artifact = fixture();
    artifact.targets[0].before_hash = "sha256:not-a-hash".to_string();
    artifact.targets[0].expected_after_hash = "sha256:1234".to_string();
    artifact.rollback_ref.target_before_hashes[0].before_hash = "sha256:not-a-hash".to_string();

    let validation = inspect_source_patch_apply_transaction_artifact(&artifact);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("64 hex characters")));
}

#[test]
fn source_patch_apply_transaction_linked_evidence_validation_requires_existing_matching_evidence() {
    let artifact = fixture();
    let missing_root = unique_run_dir("source-patch-apply-missing-linked-evidence");

    let missing = inspect_source_patch_apply_transaction_artifact_with_evidence_root(
        &artifact,
        &missing_root,
    );
    assert_eq!(missing.status, "blocked");
    assert!(missing
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("must exist and be readable JSON")));

    let ready_root = unique_run_dir("source-patch-apply-linked-evidence");
    write_linked_evidence_fixtures(&ready_root, &artifact);
    let ready =
        inspect_source_patch_apply_transaction_artifact_with_evidence_root(&artifact, &ready_root);

    assert_eq!(ready.status, "linked_evidence_ready_no_apply_authority");
    assert!(ready.blocked_reasons.is_empty());
}

#[test]
fn source_patch_apply_transaction_exports_generated_dashboard_artifact_read_only() {
    use ouroforge_core::read_dashboard_run;

    let run_dir = unique_run_dir("source-patch-apply-transaction-dashboard");
    write_minimal_dashboard_run(&run_dir);
    let artifact = fixture();
    let path = write_source_patch_apply_transaction_artifact(&run_dir, &artifact)
        .expect("transaction writes under mutation generated state");
    assert!(path.ends_with("mutation/source-patch-apply-transaction.json"));

    let dashboard = read_dashboard_run(&run_dir).expect("dashboard reads generated transaction");
    let exported = dashboard
        .mutation_artifacts
        .iter()
        .find(|artifact| artifact.id == "source-patch-apply-transaction")
        .expect("transaction exported as mutation artifact");
    assert_eq!(
        exported.path,
        "mutation/source-patch-apply-transaction.json"
    );
    assert_eq!(exported.metadata["read_only"], true);
    let value = exported
        .value
        .as_ref()
        .expect("transaction value is readable");
    assert_eq!(value["transactionId"], artifact.transaction_id);
    assert_eq!(
        value["readModel"]["readinessLabel"],
        "shape_valid_pending_linked_evidence_no_apply_authority"
    );
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("mergeCommand").is_none());
}

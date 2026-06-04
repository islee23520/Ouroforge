use ouroforge_core::{
    SourcePatchApplyTransactionArtifact, SourcePatchApplyTransactionStatus,
    SOURCE_PATCH_APPLY_TRANSACTION_SCHEMA_VERSION,
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
    assert_eq!(artifact.status, SourcePatchApplyTransactionStatus::Blocked);
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

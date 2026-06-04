use ouroforge_core::{
    inspect_source_patch_stale_target_guard_artifact,
    inspect_source_patch_stale_target_guard_artifact_with_roots,
    SourcePatchStaleTargetGuardArtifact, SourcePatchStaleTargetGuardStatus,
    SOURCE_PATCH_STALE_TARGET_GUARD_SCHEMA_VERSION,
};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture() -> SourcePatchStaleTargetGuardArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/source-patch-stale-target-guard-v1/stale-target-guard.sample.json"
    ))
    .expect("stale target guard fixture deserializes")
}

const EMPTY_SHA256: &str =
    "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn unique_temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "ouroforge-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).expect("temp dir created");
    dir
}

fn write_json(root: &Path, rel: &str, value: serde_json::Value) {
    let path = root.join(rel);
    fs::create_dir_all(path.parent().unwrap()).expect("parent dir created");
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).expect("json written");
}

fn write_empty_target(root: &Path, artifact: &SourcePatchStaleTargetGuardArtifact) {
    let path = root.join(&artifact.targets[0].path);
    fs::create_dir_all(path.parent().unwrap()).expect("target parent created");
    fs::write(path, "").expect("target written");
}

fn evidence_root_for(artifact: &SourcePatchStaleTargetGuardArtifact) -> PathBuf {
    let root = unique_temp_dir("stale-guard-evidence");
    for rel in [
        artifact.evidence_freshness.patch_preview_ref.as_str(),
        artifact.evidence_freshness.file_class_report_ref.as_str(),
        artifact
            .evidence_freshness
            .diff_integrity_report_ref
            .as_str(),
        artifact.evidence_freshness.sandbox_report_ref.as_str(),
        artifact.evidence_freshness.review_decision_ref.as_str(),
        artifact.evidence_freshness.apply_transaction_ref.as_str(),
        artifact.worktree_context_ref.as_str(),
    ] {
        write_json(&root, rel, json!({ "status": "passed" }));
    }
    write_json(
        &root,
        &artifact.evidence_freshness.review_decision_ref,
        json!({ "decision": "accepted" }),
    );
    write_json(
        &root,
        &artifact.evidence_freshness.apply_transaction_ref,
        json!({ "status": "ready_for_trusted_apply" }),
    );
    root
}

fn fresh_validation_fixture() -> SourcePatchStaleTargetGuardArtifact {
    let mut artifact = fixture();
    artifact.targets[0].expected_before_hash = EMPTY_SHA256.to_string();
    artifact.targets[0].observed_hash = EMPTY_SHA256.to_string();
    artifact
}

#[test]
fn source_patch_stale_target_guard_round_trips_fixture_without_apply_authority() {
    let artifact = fixture();

    assert_eq!(
        artifact.schema_version,
        SOURCE_PATCH_STALE_TARGET_GUARD_SCHEMA_VERSION
    );
    assert_eq!(artifact.status, SourcePatchStaleTargetGuardStatus::Fresh);
    assert_eq!(artifact.targets.len(), 1);
    assert_eq!(artifact.guard_results.len(), 2);
    assert!(artifact
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("do not apply patches")));

    let value = serde_json::to_value(&artifact).expect("guard serializes");
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("mergeCommand").is_none());
    assert!(value.get("browserCommandBridge").is_none());
    assert_eq!(
        value["targets"][0]["expectedBeforeHash"],
        value["targets"][0]["observedHash"]
    );

    let round_trip: SourcePatchStaleTargetGuardArtifact =
        serde_json::from_value(value).expect("serialized guard shape remains valid");
    assert_eq!(round_trip, artifact);
}

#[test]
fn source_patch_stale_target_guard_schema_rejects_unknown_apply_command_fields() {
    let mut value = serde_json::to_value(fixture()).expect("fixture serializes");
    value["applyCommand"] = json!("git apply patch.diff");

    let error = serde_json::from_value::<SourcePatchStaleTargetGuardArtifact>(value)
        .expect_err("unknown apply authority field must be rejected by schema");
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn source_patch_stale_target_guard_models_stale_and_blocked_evidence() {
    let mut artifact = fixture();
    artifact.status = SourcePatchStaleTargetGuardStatus::Stale;
    artifact.evidence_freshness.stale_reasons =
        vec!["review decision predates transaction".to_string()];
    artifact.targets[0].observed_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    artifact.targets[0].file_status = "changed_since_preview".to_string();
    artifact.blocked_reasons = vec!["target hash mismatch".to_string()];

    let value = serde_json::to_value(&artifact).expect("stale guard serializes");
    assert_eq!(value["status"], "stale");
    assert_eq!(value["targets"][0]["fileStatus"], "changed_since_preview");
    assert!(value["evidenceFreshness"]["staleReasons"][0]
        .as_str()
        .unwrap()
        .contains("review decision"));
    assert!(value["blockedReasons"][0]
        .as_str()
        .unwrap()
        .contains("target hash mismatch"));
}

#[test]
fn source_patch_stale_target_guard_validates_fresh_current_target_and_linked_evidence() {
    let artifact = fresh_validation_fixture();
    let evidence_root = evidence_root_for(&artifact);
    let worktree_root = unique_temp_dir("stale-guard-worktree");
    write_empty_target(&worktree_root, &artifact);

    let validation = inspect_source_patch_stale_target_guard_artifact_with_roots(
        &artifact,
        &evidence_root,
        &worktree_root,
    );

    assert_eq!(
        validation.status,
        "fresh_current_targets_and_linked_evidence_no_apply_authority"
    );
    assert!(validation.blocked_reasons.is_empty());
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("does not apply patches")));
}

#[test]
fn source_patch_stale_target_guard_blocks_stale_recorded_hash_and_branch_head_mismatch() {
    let mut artifact = fresh_validation_fixture();
    artifact.targets[0].observed_hash =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_string();
    artifact.base_ref.observed_head = "6ceebf77f0753f0b73707776c2776bb59beeb7e4".to_string();
    artifact.base_ref.head_status = "stale_branch_head".to_string();

    let validation = inspect_source_patch_stale_target_guard_artifact(&artifact);

    assert!(validation.is_blocked());
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("observedHash must match expectedBeforeHash")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("observedHead must match expectedHead")));
}

#[test]
fn source_patch_stale_target_guard_blocks_missing_and_changed_current_targets() {
    let artifact = fresh_validation_fixture();
    let evidence_root = evidence_root_for(&artifact);
    let missing_worktree_root = unique_temp_dir("stale-guard-missing-worktree");

    let missing_validation = inspect_source_patch_stale_target_guard_artifact_with_roots(
        &artifact,
        &evidence_root,
        &missing_worktree_root,
    );
    assert!(missing_validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("must exist as a regular file")));

    let changed_worktree_root = unique_temp_dir("stale-guard-changed-worktree");
    let changed_path = changed_worktree_root.join(&artifact.targets[0].path);
    fs::create_dir_all(changed_path.parent().unwrap()).expect("target parent created");
    fs::write(changed_path, "changed after preview").expect("changed target written");

    let changed_validation = inspect_source_patch_stale_target_guard_artifact_with_roots(
        &artifact,
        &evidence_root,
        &changed_worktree_root,
    );
    assert!(changed_validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("current hash") && reason.contains("does not match")));
}

#[test]
fn source_patch_stale_target_guard_blocks_stale_linked_reports_and_target_statuses() {
    let mut artifact = fresh_validation_fixture();
    artifact.targets[0].file_status = "missing_target".to_string();
    artifact.targets[0].mode_status = "mode_mismatch".to_string();
    let evidence_root = evidence_root_for(&artifact);
    write_json(
        &evidence_root,
        &artifact.evidence_freshness.review_decision_ref,
        json!({ "decision": "rejected" }),
    );
    write_json(
        &evidence_root,
        &artifact.evidence_freshness.sandbox_report_ref,
        json!({ "status": "failed" }),
    );
    write_json(
        &evidence_root,
        &artifact.evidence_freshness.file_class_report_ref,
        json!({ "status": "stale" }),
    );
    let worktree_root = unique_temp_dir("stale-guard-stale-linked-worktree");
    write_empty_target(&worktree_root, &artifact);

    let validation = inspect_source_patch_stale_target_guard_artifact_with_roots(
        &artifact,
        &evidence_root,
        &worktree_root,
    );

    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("fileStatus records missing target")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("modeStatus records file mode mismatch")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("reviewDecisionRef linked evidence must record")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("sandboxReportRef linked evidence must record")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("fileClassReportRef linked evidence must record")));
}

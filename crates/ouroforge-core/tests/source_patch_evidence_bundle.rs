use ouroforge_core::{
    inspect_source_patch_evidence_bundle, validate_source_patch_evidence_bundle,
    SourcePatchEvidenceBundleArtifact, SourcePatchEvidenceBundleStatus,
    SOURCE_PATCH_EVIDENCE_BUNDLE_SCHEMA_VERSION,
};

fn fixture_bundle() -> SourcePatchEvidenceBundleArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/source-patch-evidence-bundle-v1/source-patch-evidence-bundle.sample.json"
    ))
    .expect("source patch evidence bundle fixture parses")
}

#[test]
fn source_patch_evidence_bundle_round_trips_complete_fixture_without_apply_authority() {
    let bundle = fixture_bundle();
    assert_eq!(
        bundle.schema_version,
        SOURCE_PATCH_EVIDENCE_BUNDLE_SCHEMA_VERSION
    );
    assert_eq!(bundle.status, SourcePatchEvidenceBundleStatus::Complete);
    assert_eq!(bundle.patch_preview_id, "patch-preview-demo-001");
    assert_eq!(bundle.patch_summary.target_count, 2);
    assert_eq!(bundle.file_class_summary.highest_risk, "review_held");
    assert_eq!(
        bundle.risk_ids,
        vec!["source_patch_preview", "review_held_target"]
    );
    assert_eq!(bundle.linked_evidence.len(), 6);
    assert_eq!(bundle.dry_run_summary.status, "passed");
    assert_eq!(bundle.required_test_summary.total, 2);
    assert_eq!(bundle.review_summary.status, "reviewed");
    assert!(bundle
        .forbidden_action_notices
        .iter()
        .any(|notice| notice.action == "apply_patch"));
    assert!(bundle
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("read-only")));
    let validation =
        validate_source_patch_evidence_bundle(&bundle).expect("complete fixture should validate");
    assert_eq!(validation.status, "passed");
    assert_eq!(validation.artifact_count, 6);
    let value = serde_json::to_value(&bundle).expect("bundle serializes");
    assert!(value.get("applyCommand").is_none());
    assert!(value.get("mergeCommand").is_none());
}

#[test]
fn source_patch_evidence_bundle_supports_partial_blocked_and_stale_states() {
    let mut partial = fixture_bundle();
    partial.status = SourcePatchEvidenceBundleStatus::Partial;
    partial.sandbox_report_ref = None;
    partial.test_summary_ref = None;
    partial.review_decision_ref = None;
    partial.dry_run_summary.report_ref = None;
    partial.review_summary.decision_ref = None;
    validate_source_patch_evidence_bundle(&partial).expect("partial bundle may omit later refs");

    let mut blocked = partial.clone();
    blocked.status = SourcePatchEvidenceBundleStatus::Blocked;
    blocked.blocked_reasons = vec!["sandbox evidence missing".to_string()];
    validate_source_patch_evidence_bundle(&blocked).expect("blocked bundle with reason validates");

    let mut stale = partial;
    stale.status = SourcePatchEvidenceBundleStatus::Stale;
    stale.blocked_reasons = vec!["preview base commit is stale".to_string()];
    validate_source_patch_evidence_bundle(&stale).expect("stale bundle with reason validates");
}

#[test]
fn source_patch_evidence_bundle_blocks_missing_refs_and_forbidden_notices() {
    let mut bundle = fixture_bundle();
    bundle.sandbox_report_ref = None;
    bundle
        .forbidden_action_notices
        .retain(|notice| notice.action != "merge_branch");
    bundle.guardrails = vec!["audit data".to_string()];
    bundle.patch_summary.target_count = 0;
    bundle.risk_ids.clear();
    bundle.linked_evidence.clear();
    bundle.required_test_summary.commands = vec!["git merge feature".to_string()];

    let validation = inspect_source_patch_evidence_bundle(&bundle);
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("complete bundle requires")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("merge_branch")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("read-only")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("patchSummary.targetCount")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("riskIds")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("linkedEvidence")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("source patch apply, merge")));
}

#[test]
fn source_patch_evidence_bundle_rejects_escaping_paths_and_unknown_fields() {
    let mut value = serde_json::to_value(fixture_bundle()).expect("fixture serializes");
    value["previewRef"]["path"] = serde_json::json!("../outside.json");
    value["applyCommand"] = serde_json::json!("git apply patch.diff");

    let error = serde_json::from_value::<SourcePatchEvidenceBundleArtifact>(value)
        .expect_err("unknown apply-like fields must not parse");
    assert!(error.to_string().contains("unknown field"));

    let mut bundle = fixture_bundle();
    bundle.preview_ref.path = "../outside.json".to_string();
    let validation = inspect_source_patch_evidence_bundle(&bundle);
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("previewRef.path")));
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

fn write_minimal_dashboard_run(run_dir: &std::path::Path) {
    std::fs::write(
        run_dir.join("run.json"),
        serde_json::json!({"id":"run-source-patch-bundle","created_at_unix_ms":1}).to_string(),
    )
    .expect("write run");
    std::fs::write(
        run_dir.join("verdict.json"),
        serde_json::json!({"status":"passed"}).to_string(),
    )
    .expect("write verdict");
    std::fs::write(
        run_dir.join("evidence/index.json"),
        serde_json::json!({"artifacts":[]}).to_string(),
    )
    .expect("write evidence index");
}

#[test]
fn source_patch_evidence_bundle_writes_generated_artifact_and_exports_to_dashboard() {
    use ouroforge_core::{read_dashboard_run, write_source_patch_evidence_bundle};

    let run_dir = unique_run_dir("source-patch-bundle-dashboard");
    write_minimal_dashboard_run(&run_dir);
    let bundle = fixture_bundle();
    let path = write_source_patch_evidence_bundle(&run_dir, &bundle)
        .expect("bundle writes under mutation generated state");
    assert!(path.ends_with("mutation/source-patch-evidence-bundle.json"));

    let dashboard = read_dashboard_run(&run_dir).expect("dashboard reads generated bundle");
    let artifact = dashboard
        .mutation_artifacts
        .iter()
        .find(|artifact| artifact.id == "source-patch-evidence-bundle")
        .expect("bundle exported as mutation artifact");
    assert_eq!(artifact.path, "mutation/source-patch-evidence-bundle.json");
    assert_eq!(artifact.metadata["read_only"], true);
    assert_eq!(
        artifact.value.as_ref().unwrap()["bundleId"],
        bundle.bundle_id
    );
    assert_eq!(
        artifact.value.as_ref().unwrap()["patchSummary"]["targetCount"],
        bundle.patch_summary.target_count
    );
    assert_eq!(
        artifact.value.as_ref().unwrap()["reviewSummary"]["status"],
        bundle.review_summary.status
    );
    assert!(artifact
        .value
        .as_ref()
        .unwrap()
        .get("applyCommand")
        .is_none());
}

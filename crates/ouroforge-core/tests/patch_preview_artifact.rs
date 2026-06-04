use ouroforge_core::{
    inspect_source_patch_preview_artifact, validate_source_patch_preview_artifact,
    PatchDiffIntegrityLimits, SourcePatchPreviewApplyStatus, SourcePatchPreviewArtifact,
    SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
};

#[test]
fn source_patch_preview_artifact_round_trips_fixture_without_apply_authority() {
    let fixture =
        include_str!("../../../examples/patch-preview-artifact-v1/patch-preview.sample.json");
    let artifact: SourcePatchPreviewArtifact =
        serde_json::from_str(fixture).expect("patch preview fixture parses");

    assert_eq!(artifact.schema_version, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION);
    assert_eq!(artifact.patch_preview_id, "patch-preview-demo-001");
    assert!(artifact.apply_is_blocked());
    assert_eq!(
        artifact.source_mutation_apply_status,
        SourcePatchPreviewApplyStatus::Blocked
    );
    assert_eq!(artifact.targets.len(), 1);
    assert_eq!(
        artifact.targets[0].path,
        "examples/playable-demo-v2/collect-and-exit/scenes/game.scene.json"
    );
    assert_eq!(artifact.diff_summary.diff_stats.files_changed, 1);
    assert_eq!(artifact.diff_summary.diff_stats.additions, 1);
    assert_eq!(artifact.diff_summary.diff_stats.deletions, 1);
    assert!(artifact
        .diff_summary
        .diff_text
        .as_deref()
        .expect("fixture includes inert diff text")
        .contains("diff --git"));
    assert!(artifact.artifact_hash.starts_with("sha256:"));
    assert!(artifact
        .required_tests
        .iter()
        .all(|test| test.execution_authority == "copyable_only_not_executed_by_preview"));
    assert!(artifact
        .read_model_prototype
        .as_ref()
        .expect("fixture carries read-only display prototype")
        .forbidden_actions
        .iter()
        .any(|action| action == "apply_patch"));

    let value = serde_json::to_value(&artifact).expect("artifact serializes");
    assert_eq!(value["schemaVersion"], SOURCE_PATCH_PREVIEW_SCHEMA_VERSION);
    assert_eq!(value["sourceMutationApplyStatus"], "blocked");
    assert_eq!(
        value["artifactHash"],
        "sha256:fixture-preview-artifact-hash"
    );
    assert_eq!(value["diffSummary"]["diffStats"]["binaryOrOpaque"], false);
}

#[test]
fn source_patch_preview_artifact_schema_rejects_unknown_fields() {
    let fixture =
        include_str!("../../../examples/patch-preview-artifact-v1/patch-preview.sample.json");
    let mut value: serde_json::Value = serde_json::from_str(fixture).expect("fixture json parses");
    value.as_object_mut().expect("fixture is object").insert(
        "applyCommand".to_string(),
        serde_json::json!("git apply patch.diff"),
    );

    let error = serde_json::from_value::<SourcePatchPreviewArtifact>(value)
        .expect_err("unknown apply-like fields must not parse in schema fixture tests");
    assert!(error.to_string().contains("unknown field"));
}

fn fixture_artifact() -> SourcePatchPreviewArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/patch-preview-artifact-v1/patch-preview.sample.json"
    ))
    .expect("patch preview fixture parses")
}

#[test]
fn source_patch_preview_validation_passes_fixture_with_diff_and_file_class_evidence() {
    let artifact = fixture_artifact();
    let validation =
        validate_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default())
            .expect("fixture should pass preview validation");

    assert_eq!(
        validation.schema_version,
        "source-patch-preview-validation-v1"
    );
    assert_eq!(validation.status, "passed");
    assert!(validation.blocked_reasons.is_empty());
    let diff_validation = validation
        .diff_integrity_validation
        .as_ref()
        .expect("validation includes diff integrity evidence");
    assert_eq!(diff_validation.status, "passed");
    assert_eq!(diff_validation.report.file_count, 1);
    assert_eq!(diff_validation.report.counts.added, 1);
    assert_eq!(diff_validation.report.counts.removed, 1);
    assert_eq!(validation.file_class_validation.targets.len(), 1);
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("no source patch apply")));
}

#[test]
fn source_patch_preview_validation_checks_required_test_allowlist_metadata() {
    let artifact = fixture_artifact();
    let required_test = &artifact.required_tests[0];
    assert_eq!(
        required_test.allowlist_policy_id.as_deref(),
        Some("source-patch-preview-safe-local-checks-v1")
    );
    assert_eq!(
        required_test.command,
        "cargo test -p ouroforge-core validates_engine_expressiveness_v2_regression_seed_and_pack -- --nocapture"
    );
    assert!(!required_test.argv.is_empty());

    validate_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default())
        .expect("fixture requiredTests metadata is allowlisted");
}

#[test]
fn source_patch_preview_validation_blocks_forbidden_required_test_metadata() {
    let mut artifact = fixture_artifact();
    artifact.required_tests[0].command = "curl https://example.invalid".to_string();
    artifact.required_tests[0].argv =
        vec!["curl".to_string(), "https://example.invalid".to_string()];
    artifact.required_tests[0].allowlist_policy_id =
        Some("source-patch-preview-safe-local-checks-v1".to_string());

    let validation =
        inspect_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default());
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("requiredTests command forbidden")
            && reason.contains("network")));
}

#[test]
fn source_patch_preview_validation_blocks_required_test_command_argv_drift() {
    let mut artifact = fixture_artifact();
    artifact.required_tests[0].command = "cargo test".to_string();

    let validation =
        inspect_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default());
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("must match normalized argv")));
}

#[test]
fn source_patch_preview_validation_blocks_missing_evidence_and_tests() {
    let mut artifact = fixture_artifact();
    artifact.linked_evidence.clear();
    artifact.required_tests.clear();

    let validation =
        inspect_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default());
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("linkedEvidence")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("requiredTests")));
    let error =
        validate_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default())
            .expect_err("blocked preview should reject");
    assert!(error
        .to_string()
        .contains("source patch preview validation blocked"));
}

#[test]
fn source_patch_preview_validation_blocks_duplicate_targets_and_stat_drift() {
    let mut artifact = fixture_artifact();
    artifact.targets.push(artifact.targets[0].clone());
    artifact.diff_summary.diff_stats.additions = 99;

    let validation =
        inspect_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default());
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("duplicate target file")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("diffStats.additions")));
}

#[test]
fn source_patch_preview_validation_blocks_unsafe_diff_targets_before_preview() {
    let mut artifact = fixture_artifact();
    artifact.targets[0].path = "runs/source-patch-previews/generated.json".to_string();
    artifact.targets[0].blocked_reasons = vec!["generated local state".to_string()];
    artifact.diff_summary.hunks[0].path = artifact.targets[0].path.clone();
    artifact.diff_summary.diff_text = Some(
        "diff --git a/runs/source-patch-previews/generated.json b/runs/source-patch-previews/generated.json\n--- a/runs/source-patch-previews/generated.json\n+++ b/runs/source-patch-previews/generated.json\n@@ -1 +1 @@\n-old\n+new\n"
            .to_string(),
    );

    let validation =
        inspect_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default());
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("file class blocked")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("diff integrity blocked")));
}

#[test]
fn source_mutation_preview_demo_fixture_validates_without_apply_authority() {
    let artifact: SourcePatchPreviewArtifact = serde_json::from_str(include_str!(
        "../../../examples/source-mutation-preview-demo-v1/patch-preview-demo.sample.json"
    ))
    .expect("demo patch preview fixture parses");

    assert_eq!(artifact.patch_preview_id, "smp1-10-demo-preview-001");
    assert_eq!(
        artifact.source_mutation_apply_status,
        SourcePatchPreviewApplyStatus::Blocked
    );
    assert_eq!(
        artifact.risk_level,
        ouroforge_core::SourcePatchPreviewRiskLevel::Medium
    );
    assert_eq!(artifact.targets.len(), 1);
    assert_eq!(
        artifact.targets[0].path,
        "examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json"
    );
    assert!(artifact
        .diff_summary
        .diff_text
        .as_deref()
        .expect("demo diff text present")
        .contains("generated sandbox"));
    assert!(artifact
        .read_model_prototype
        .as_ref()
        .expect("read model prototype")
        .forbidden_actions
        .contains(&"apply_patch".to_string()));

    let validation =
        validate_source_patch_preview_artifact(&artifact, PatchDiffIntegrityLimits::default())
            .expect("demo preview fixture should validate");
    assert_eq!(validation.status, "passed");
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("no source patch apply")));

    let demo_note =
        include_str!("../../../examples/source-mutation-preview-demo-v1/demo-behavior-copy.md");
    assert_eq!(demo_note.trim(), "Preview demo copy stays unchanged.");

    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("core crate lives under crates/ouroforge-core");
    let trusted_target = std::fs::read_to_string(workspace_root.join(&artifact.targets[0].path))
        .expect("classified preview target remains readable");
    assert!(
        !trusted_target.contains("Preview demo copy would change only inside a generated sandbox."),
        "preview after-text must not be applied to the trusted target"
    );
}

use ouroforge_core::{
    SourcePatchPreviewApplyStatus, SourcePatchPreviewArtifact, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
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

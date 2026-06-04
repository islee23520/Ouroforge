use ouroforge_core::{
    inspect_source_patch_test_command_allowlist, normalize_source_patch_test_command,
    validate_source_patch_test_command_allowlist, SourcePatchTestCommandAllowlistArtifact,
    SOURCE_PATCH_TEST_COMMAND_ALLOWLIST_SCHEMA_VERSION,
};

fn fixture_policy() -> SourcePatchTestCommandAllowlistArtifact {
    serde_json::from_str(include_str!(
        "../../../examples/source-patch-command-allowlist-v1/allowed-commands.sample.json"
    ))
    .expect("allowlist fixture parses")
}

#[test]
fn source_patch_test_command_allowlist_accepts_safe_fixture_without_execution_authority() {
    let fixture = include_str!(
        "../../../examples/source-patch-command-allowlist-v1/allowed-commands.sample.json"
    );
    let artifact: SourcePatchTestCommandAllowlistArtifact =
        serde_json::from_str(fixture).expect("allowlist fixture parses");

    assert_eq!(
        artifact.schema_version,
        SOURCE_PATCH_TEST_COMMAND_ALLOWLIST_SCHEMA_VERSION
    );
    assert!(artifact
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("does not execute commands")));
    assert!(artifact
        .commands
        .iter()
        .all(|command| !command.may_write_generated_artifacts));
    assert!(artifact
        .commands
        .iter()
        .all(|command| command.working_directory == "."));

    let validation = validate_source_patch_test_command_allowlist(&artifact)
        .expect("safe allowlist fixture validates");
    assert_eq!(
        validation.schema_version,
        "source-patch-test-command-allowlist-validation-v1"
    );
    assert_eq!(validation.status, "passed");
    assert_eq!(validation.allowed_command_count, artifact.commands.len());
    assert!(validation.blocked_reasons.is_empty());
    assert!(validation
        .normalized_commands
        .iter()
        .any(|command| command == "cargo fmt --check"));
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("does not execute commands")));
}

#[test]
fn source_patch_test_command_allowlist_normalizes_from_argv_not_shell_text() {
    let artifact = fixture_policy();
    let focused = artifact
        .commands
        .iter()
        .find(|command| command.id == "core-patch-preview-tests")
        .expect("focused command exists");

    assert_eq!(
        normalize_source_patch_test_command(&focused.argv),
        "cargo test -p ouroforge-core --test patch_preview_artifact -- --nocapture"
    );
}

#[test]
fn source_patch_test_command_allowlist_blocks_schema_drift_and_command_text_mismatch() {
    let mut artifact = fixture_policy();
    artifact.schema_version = "source-patch-test-command-allowlist-v0".to_string();
    artifact.commands[0].command = "cargo fmt".to_string();

    let validation = inspect_source_patch_test_command_allowlist(&artifact);
    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("schemaVersion")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("must match normalized argv")));
    let error = validate_source_patch_test_command_allowlist(&artifact)
        .expect_err("blocked allowlist should fail closed");
    assert!(error
        .to_string()
        .contains("source patch test command allowlist blocked"));
}

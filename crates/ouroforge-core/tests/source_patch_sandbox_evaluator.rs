use ouroforge_core::{
    apply_source_patch_preview_in_sandbox, inspect_source_patch_sandbox_evaluator_plan,
    reject_generated_artifact_source_collision, run_source_patch_sandbox_allowlisted_tests,
    validate_source_patch_sandbox_evaluator_plan, SourcePatchPreviewApplyStatus,
    SourcePatchPreviewArtifact, SourcePatchPreviewRequiredTest, SourcePatchSandboxCleanupPolicy,
    SourcePatchSandboxEvaluationInputs, SourcePatchSandboxEvaluatorPlan,
    SourcePatchSandboxLayoutPolicy, SOURCE_PATCH_SANDBOX_EVALUATOR_PLAN_SCHEMA_VERSION,
};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture_plan() -> SourcePatchSandboxEvaluatorPlan {
    SourcePatchSandboxEvaluatorPlan {
        schema_version: SOURCE_PATCH_SANDBOX_EVALUATOR_PLAN_SCHEMA_VERSION.to_string(),
        evaluation_id: "patch-preview-1".to_string(),
        created_at: "2026-06-04T00:00:00Z".to_string(),
        source_mutation_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        inputs: SourcePatchSandboxEvaluationInputs {
            patch_preview_id: "patch-preview-1".to_string(),
            file_class_validation_ref: "evidence/source-file-class-validation.json".to_string(),
            diff_integrity_validation_ref: "evidence/patch-diff-integrity.json".to_string(),
            allowlist_policy_id: "source-patch-preview-safe-local-checks-v1".to_string(),
        },
        layout: SourcePatchSandboxLayoutPolicy {
            sandbox_id: "patch-preview-1".to_string(),
            sandbox_root: "sandbox/patch-preview-1".to_string(),
            worktree_path: "sandbox/patch-preview-1/worktree".to_string(),
            evidence_path: "sandbox/patch-preview-1/evidence".to_string(),
            report_path: "sandbox/patch-preview-1/evidence/report.json".to_string(),
            cleanup_path: "sandbox/patch-preview-1/cleanup.json".to_string(),
        },
        cleanup: SourcePatchSandboxCleanupPolicy {
            cleanup_required: true,
            preserve_evidence: true,
            on_success: "remove sandbox worktree after report capture".to_string(),
            on_failure: "preserve evidence and cleanup metadata for review".to_string(),
            generated_output_roots: vec![
                "sandbox/patch-preview-1/evidence".to_string(),
                "sandbox/patch-preview-1/worktree/target".to_string(),
            ],
        },
        required_tests: vec![SourcePatchPreviewRequiredTest {
            command: "cargo test -p ouroforge-core --test patch_preview_artifact -- --nocapture"
                .to_string(),
            argv: vec![
                "cargo".to_string(),
                "test".to_string(),
                "-p".to_string(),
                "ouroforge-core".to_string(),
                "--test".to_string(),
                "patch_preview_artifact".to_string(),
                "--".to_string(),
                "--nocapture".to_string(),
            ],
            allowlist_policy_id: Some("source-patch-preview-safe-local-checks-v1".to_string()),
            execution_authority: "copyable_not_executed_metadata".to_string(),
        }],
        guardrails: vec![
            "sandbox evaluator plan is inert metadata; it does not execute commands".to_string(),
            "trusted main worktree is not modified by this setup/cleanup model".to_string(),
            "source patch apply remains blocked outside a future sandbox-only evaluator"
                .to_string(),
        ],
    }
}

#[test]
fn source_patch_sandbox_evaluator_plan_models_inert_setup_and_cleanup() {
    let plan = fixture_plan();

    let validation = validate_source_patch_sandbox_evaluator_plan(&plan)
        .expect("safe sandbox evaluator setup model validates");

    assert_eq!(validation.status, "passed");
    assert!(validation.blocked_reasons.is_empty());
    assert!(validation
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("does not execute commands")));
    assert_eq!(plan.layout.sandbox_root, "sandbox/patch-preview-1");
    assert!(plan.cleanup.cleanup_required);
    assert!(plan.cleanup.preserve_evidence);
}

#[test]
fn source_patch_sandbox_evaluator_plan_rejects_paths_outside_sandbox_root() {
    let mut plan = fixture_plan();
    plan.layout.worktree_path = "../trusted-worktree".to_string();
    plan.cleanup.generated_output_roots = vec!["runs/outside".to_string()];

    let validation = inspect_source_patch_sandbox_evaluator_plan(&plan);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("layout.worktreePath")
            && reason.contains("must stay under sandbox")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("cleanup.generatedOutputRoots[0]")
            && reason.contains("sandbox")));
}

#[test]
fn source_patch_sandbox_evaluator_plan_rejects_execution_or_forbidden_commands() {
    let mut plan = fixture_plan();
    plan.required_tests[0].command = "curl https://example.invalid".to_string();
    plan.required_tests[0].argv = vec!["curl".to_string(), "https://example.invalid".to_string()];
    plan.required_tests[0].execution_authority = "execute_in_trusted_worktree".to_string();

    let validation = inspect_source_patch_sandbox_evaluator_plan(&plan);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("executionAuthority")));
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("forbidden") && reason.contains("network")));
}

#[test]
fn source_patch_sandbox_evaluator_plan_requires_validation_refs_and_apply_block() {
    let mut plan = fixture_plan();
    plan.inputs.file_class_validation_ref = "".to_string();
    plan.inputs.diff_integrity_validation_ref = "/tmp/diff.json".to_string();
    plan.guardrails.clear();

    let validation = inspect_source_patch_sandbox_evaluator_plan(&plan);

    assert_eq!(validation.status, "blocked");
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("fileClassValidationRef")));
    assert!(
        validation
            .blocked_reasons
            .iter()
            .any(|reason| reason.contains("diffIntegrityValidationRef")
                && reason.contains("relative"))
    );
    assert!(validation
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("does not execute")));
}

#[test]
fn source_patch_sandbox_apply_writes_only_sandbox_worktree_and_report() {
    let temp = sandbox_test_dir("apply-writes-only-sandbox");
    let repo_root = temp.join("repo");
    let run_dir = temp.join("run");
    let target_rel = "crates/ouroforge-core/src/lib.rs";
    let trusted_target = repo_root.join(target_rel);
    fs::create_dir_all(trusted_target.parent().expect("target parent")).expect("create repo");
    fs::create_dir_all(&run_dir).expect("create run dir");
    fs::write(repo_root.join("Cargo.toml"), "[workspace]\n").expect("write manifest");
    fs::write(repo_root.join("README.md"), "trusted readme\n").expect("write copied source");
    fs::create_dir_all(repo_root.join("target/debug")).expect("create skipped target");
    fs::write(repo_root.join("target/debug/build.log"), "generated\n")
        .expect("write skipped generated file");
    fs::write(&trusted_target, "pub fn demo() {\n    1\n}\n").expect("write trusted target");

    let snapshot_hash = test_file_hash(&trusted_target);
    let before_hash = "sha256:26487e8eab103e0b035c5075ec7a50319e7e65b92c843c5e44d66b2e35d7b3d4";
    let artifact = fixture_apply_artifact(target_rel, before_hash);
    let plan = fixture_plan();

    let result = apply_source_patch_preview_in_sandbox(&artifact, &plan, &repo_root, &run_dir)
        .expect("sandbox apply succeeds");

    assert_eq!(result.status, "passed");
    assert!(result.commands_run.is_empty());
    assert_eq!(
        fs::read_to_string(&trusted_target).expect("read trusted"),
        "pub fn demo() {\n    1\n}\n"
    );
    let sandbox_target = run_dir
        .join("sandbox/patch-preview-1/worktree")
        .join(target_rel);
    assert_eq!(
        fs::read_to_string(sandbox_target).expect("read sandbox"),
        "pub fn demo() {\n    2\n}\n"
    );
    assert_eq!(
        fs::read_to_string(run_dir.join("sandbox/patch-preview-1/worktree/Cargo.toml"))
            .expect("sandbox manifest copied"),
        "[workspace]\n"
    );
    assert_eq!(
        fs::read_to_string(run_dir.join("sandbox/patch-preview-1/worktree/README.md"))
            .expect("sandbox source file copied"),
        "trusted readme\n"
    );
    assert!(!run_dir
        .join("sandbox/patch-preview-1/worktree/target/debug/build.log")
        .exists());
    let report: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_dir.join("sandbox/patch-preview-1/evidence/report.json"))
            .expect("report written"),
    )
    .expect("report json parses");
    assert_eq!(report["commandsRun"], json!([]));
    assert!(report["guardrails"]
        .as_array()
        .expect("guardrails array")
        .iter()
        .any(|guardrail| guardrail.as_str().unwrap_or_default().contains("no shell")));
    assert_eq!(
        result.target_snapshots[0].trusted_before_hash,
        snapshot_hash
    );
    assert_eq!(result.target_snapshots[0].trusted_after_hash, snapshot_hash);

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_apply_rejects_stale_target_without_trusted_write() {
    let temp = sandbox_test_dir("apply-rejects-stale-target");
    let repo_root = temp.join("repo");
    let run_dir = temp.join("run");
    let target_rel = "crates/ouroforge-core/src/lib.rs";
    let trusted_target = repo_root.join(target_rel);
    fs::create_dir_all(trusted_target.parent().expect("target parent")).expect("create repo");
    fs::create_dir_all(&run_dir).expect("create run dir");
    fs::write(&trusted_target, "pub fn demo() {\n    1\n}\n").expect("write trusted target");

    let mut artifact = fixture_apply_artifact(
        target_rel,
        "sha256:0000000000000000000000000000000000000000000000000000000000000000",
    );
    artifact.targets[0].before_hash =
        "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string();
    let error =
        apply_source_patch_preview_in_sandbox(&artifact, &fixture_plan(), &repo_root, &run_dir)
            .expect_err("stale target is rejected before sandbox write");

    assert!(error.to_string().contains("stale source patch target"));
    assert_eq!(
        fs::read_to_string(&trusted_target).expect("trusted unchanged"),
        "pub fn demo() {\n    1\n}\n"
    );
    assert!(!run_dir
        .join("sandbox/patch-preview-1/worktree")
        .join(target_rel)
        .exists());

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_allowlisted_tests_run_in_sandbox_and_capture_report() {
    let temp = sandbox_test_dir("allowlisted-tests-capture-report");
    let run_dir = temp.join("run");
    let worktree_file =
        run_dir.join("sandbox/patch-preview-1/worktree/examples/evidence-dashboard/dashboard.js");
    fs::create_dir_all(worktree_file.parent().expect("worktree parent")).expect("create worktree");
    fs::write(&worktree_file, "const dashboard = 'sandbox-only';\n").expect("write sandbox js");

    let required_tests = vec![SourcePatchPreviewRequiredTest {
        command: "node --check examples/evidence-dashboard/dashboard.js".to_string(),
        argv: vec![
            "node".to_string(),
            "--check".to_string(),
            "examples/evidence-dashboard/dashboard.js".to_string(),
        ],
        allowlist_policy_id: Some("source-patch-preview-safe-local-checks-v1".to_string()),
        execution_authority: "sandbox_allowlisted_execution".to_string(),
    }];

    let report =
        run_source_patch_sandbox_allowlisted_tests(&fixture_plan(), &required_tests, &run_dir)
            .expect("allowlisted sandbox test passes");

    assert_eq!(report.status, "passed");
    assert_eq!(
        report.commands_run,
        vec!["node --check examples/evidence-dashboard/dashboard.js"]
    );
    assert_eq!(
        report.tests[0].allowlist_policy_id,
        "node-check-known-examples"
    );
    assert_eq!(report.tests[0].status, "passed");
    assert!(report
        .guardrails
        .iter()
        .any(|guardrail| guardrail.contains("current_dir")));
    let report_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(
            run_dir.join("sandbox/patch-preview-1/evidence/test-execution-report.json"),
        )
        .expect("test execution report written"),
    )
    .expect("test execution report parses");
    assert_eq!(
        report_json["commandsRun"][0],
        "node --check examples/evidence-dashboard/dashboard.js"
    );

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_allowlisted_tests_reject_forbidden_commands_before_execution() {
    let temp = sandbox_test_dir("allowlisted-tests-reject-forbidden");
    let run_dir = temp.join("run");
    fs::create_dir_all(run_dir.join("sandbox/patch-preview-1/worktree")).expect("create worktree");

    let required_tests = vec![SourcePatchPreviewRequiredTest {
        command: "curl https://example.invalid".to_string(),
        argv: vec!["curl".to_string(), "https://example.invalid".to_string()],
        allowlist_policy_id: Some("source-patch-preview-safe-local-checks-v1".to_string()),
        execution_authority: "sandbox_allowlisted_execution".to_string(),
    }];

    let error =
        run_source_patch_sandbox_allowlisted_tests(&fixture_plan(), &required_tests, &run_dir)
            .expect_err("forbidden command is blocked");

    assert!(error.to_string().contains("network"));
    let report_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(
            run_dir.join("sandbox/patch-preview-1/evidence/test-execution-report.json"),
        )
        .expect("blocked report written"),
    )
    .expect("blocked report parses");
    assert_eq!(report_json["commandsRun"], json!([]));
    assert!(report_json["blockedReasons"][0]
        .as_str()
        .expect("blocked reason")
        .contains("network"));

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_apply_requires_passed_preview_validation() {
    let temp = sandbox_test_dir("apply-requires-validation");
    let repo_root = temp.join("repo");
    let run_dir = temp.join("run");
    let target_rel = "target/debug/generated.rs";
    let trusted_target = repo_root.join(target_rel);
    fs::create_dir_all(trusted_target.parent().expect("target parent")).expect("create repo");
    fs::create_dir_all(&run_dir).expect("create run dir");
    fs::write(&trusted_target, "pub fn demo() {\n    1\n}\n").expect("write trusted target");

    let mut artifact = fixture_apply_artifact(
        target_rel,
        "sha256:26487e8eab103e0b035c5075ec7a50319e7e65b92c843c5e44d66b2e35d7b3d4",
    );
    artifact.targets[0].file_class = "generated_local_state".to_string();
    artifact.targets[0].classification_status = "blocked_by_design".to_string();
    artifact.targets[0].blocked_reasons = vec!["generated target".to_string()];

    let error =
        apply_source_patch_preview_in_sandbox(&artifact, &fixture_plan(), &repo_root, &run_dir)
            .expect_err("blocked preview validation prevents sandbox apply");

    assert!(error
        .to_string()
        .contains("source patch preview validation blocked"));
    assert!(!run_dir
        .join("sandbox/patch-preview-1/worktree")
        .join(target_rel)
        .exists());

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_cargo_tests_require_sandbox_manifest_not_parent_workspace() {
    let temp = sandbox_test_dir("cargo-requires-sandbox-manifest");
    let repo_root = temp.join("repo");
    let run_dir = repo_root.join("sandbox-parent-leak");
    fs::create_dir_all(run_dir.join("sandbox/patch-preview-1/worktree")).expect("create worktree");
    fs::write(repo_root.join("Cargo.toml"), "[workspace]\n").expect("write parent manifest");
    let artifact = fixture_apply_artifact(
        "crates/ouroforge-core/src/lib.rs",
        "sha256:26487e8eab103e0b035c5075ec7a50319e7e65b92c843c5e44d66b2e35d7b3d4",
    );

    let error = run_source_patch_sandbox_allowlisted_tests(
        &fixture_plan(),
        &artifact.required_tests,
        &run_dir,
    )
    .expect_err("cargo command is blocked without sandbox manifest");

    assert!(error
        .to_string()
        .contains("requires sandbox worktree Cargo.toml"));
    let report_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(
            run_dir.join("sandbox/patch-preview-1/evidence/test-execution-report.json"),
        )
        .expect("blocked report written"),
    )
    .expect("blocked report parses");
    assert!(report_json["blockedReasons"][0]
        .as_str()
        .expect("blocked reason")
        .contains("requires sandbox worktree Cargo.toml"));

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_cargo_tests_reject_explicit_parent_manifest_override() {
    let temp = sandbox_test_dir("cargo-rejects-parent-manifest-override");
    let repo_root = temp.join("repo");
    let run_dir = repo_root.join("sandbox-parent-manifest-override");
    let worktree = run_dir.join("sandbox/patch-preview-1/worktree");
    fs::create_dir_all(&worktree).expect("create worktree");
    fs::write(repo_root.join("Cargo.toml"), "[workspace]\n").expect("write parent manifest");
    fs::write(worktree.join("Cargo.toml"), "[workspace]\n").expect("write sandbox manifest");

    let required_tests = vec![SourcePatchPreviewRequiredTest {
        command: "cargo test -p ouroforge-core --manifest-path ../../../../Cargo.toml --test patch_preview_artifact -- --nocapture".to_string(),
        argv: vec![
            "cargo".to_string(),
            "test".to_string(),
            "-p".to_string(),
            "ouroforge-core".to_string(),
            "--manifest-path".to_string(),
            "../../../../Cargo.toml".to_string(),
            "--test".to_string(),
            "patch_preview_artifact".to_string(),
            "--".to_string(),
            "--nocapture".to_string(),
        ],
        allowlist_policy_id: Some("source-patch-preview-safe-local-checks-v1".to_string()),
        execution_authority: "sandbox_allowlisted_execution".to_string(),
    }];

    let error =
        run_source_patch_sandbox_allowlisted_tests(&fixture_plan(), &required_tests, &run_dir)
            .expect_err("explicit parent manifest override is blocked before execution");

    assert!(error.to_string().contains("may not override workspace"));
    let report_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(
            run_dir.join("sandbox/patch-preview-1/evidence/test-execution-report.json"),
        )
        .expect("blocked report written"),
    )
    .expect("blocked report parses");
    assert_eq!(report_json["commandsRun"], json!([]));
    assert!(report_json["blockedReasons"][0]
        .as_str()
        .expect("blocked reason")
        .contains("may not override workspace"));

    fs::remove_dir_all(temp).expect("cleanup temp");
}

#[test]
fn source_patch_sandbox_root_is_documented_generated_state() {
    reject_generated_artifact_source_collision(
        Path::new("sandbox/patch-preview-1/worktree/crates/ouroforge-core/src/lib.rs"),
        "sandbox preview",
    )
    .expect("sandbox root is generated state even for copied source-like paths");
}

fn fixture_apply_artifact(target_rel: &str, before_hash: &str) -> SourcePatchPreviewArtifact {
    serde_json::from_value(json!({
        "schemaVersion": "patch-preview.v1",
        "patchPreviewId": "patch-preview-1",
        "proposalId": "proposal-1",
        "createdAt": "2026-06-04T00:00:00Z",
        "producer": {
            "name": "test",
            "version": "1",
            "trustedBoundary": "unit-test"
        },
        "sourceMutationApplyStatus": "blocked",
        "baseRef": {
            "branch": "main",
            "commit": "fixture",
            "targetFreshness": "checked"
        },
        "staleTargetPolicy": "reject before sandbox copy",
        "artifactHash": "sha256:1111111111111111111111111111111111111111111111111111111111111111",
        "targets": [{
            "path": target_rel,
            "beforeHash": before_hash,
            "fileClass": "rust_trust_boundary",
            "reviewLevel": "Separate governance approval",
            "classificationStatus": "restricted_separate_approval",
            "classificationRationale": "unit test fixture Rust trust boundary target",
            "blockedReasons": ["Rust trust-boundary code requires separate governance approval"]
        }],
        "diffSummary": {
            "summary": "Change demo return value in sandbox copy only",
            "diffText": format!("diff --git a/{target_rel} b/{target_rel}\n--- a/{target_rel}\n+++ b/{target_rel}\n@@ -1,3 +1,3 @@\n pub fn demo() {{\n-    1\n+    2\n }}\n"),
            "diffStats": {
                "filesChanged": 1,
                "additions": 1,
                "deletions": 1,
                "binaryOrOpaque": false,
                "generatedOrigin": false,
                "truncated": false
            },
            "hunks": [{
                "path": target_rel,
                "summary": "demo body changes from 1 to 2"
            }]
        },
        "riskLevel": "low",
        "riskIds": ["STM-01"],
        "linkedEvidence": [{"kind": "unit-test", "path": "evidence/source-file-class-validation.json"}],
        "expectedBehaviorChange": "sandbox copy changes only",
        "requiredTests": [{
            "command": "cargo test -p ouroforge-core --test patch_preview_artifact -- --nocapture",
            "argv": ["cargo", "test", "-p", "ouroforge-core", "--test", "patch_preview_artifact", "--", "--nocapture"],
            "allowlistPolicyId": "source-patch-preview-safe-local-checks-v1",
            "executionAuthority": "copyable_not_executed_metadata"
        }],
        "reviewerChecklist": ["confirm sandbox-only write"],
        "rollbackExpectations": {
            "requiredBeforeApply": true,
            "minimumFields": ["beforeHash"]
        },
        "readModelPrototype": {
            "status": "blocked",
            "displayLabel": "sandbox apply fixture",
            "fileClassSummary": "rust source",
            "riskSummary": "low",
            "primaryBlockedReason": "trusted apply remains blocked",
            "allowedActions": ["sandbox preview"],
            "forbiddenActions": ["trusted apply"]
        }
    }))
    .expect("apply fixture parses")
}

fn sandbox_test_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "ouroforge-source-patch-sandbox-{name}-{}-{unique}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("remove stale temp dir");
    }
    path
}

fn test_file_hash(path: &Path) -> String {
    let bytes = fs::read(path).expect("read file for test hash");
    format!("fnv1a64-file-v1:{:016x}", test_fnv1a64(&bytes))
}

fn test_fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

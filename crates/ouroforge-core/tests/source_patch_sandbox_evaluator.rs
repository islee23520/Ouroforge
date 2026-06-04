use ouroforge_core::{
    inspect_source_patch_sandbox_evaluator_plan, validate_source_patch_sandbox_evaluator_plan,
    SourcePatchPreviewApplyStatus, SourcePatchPreviewRequiredTest, SourcePatchSandboxCleanupPolicy,
    SourcePatchSandboxEvaluationInputs, SourcePatchSandboxEvaluatorPlan,
    SourcePatchSandboxLayoutPolicy, SOURCE_PATCH_SANDBOX_EVALUATOR_PLAN_SCHEMA_VERSION,
};

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

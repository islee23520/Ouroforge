//! Scenario Coverage v62 regression suite for #2036 / Era L M70.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    attribute_self_audit_bottlenecks, generate_self_diagnosis_record,
    generate_source_apply_fix_proposal, run_self_diagnosis_fix_proposal_demo,
    self_audit_bottleneck_input_from_json_str, source_patch_sandbox_sha256_hex,
    validate_source_patch_preview_artifact, PatchDiffIntegrityLimits, SelfAuditAttributionContract,
    SelfDiagnosisFixProposalContract, SelfDiagnosisFixProposalDemoInput,
    SelfDiagnosisGeneratorInput, SelfFixProposalGeneratorInput, SourcePatchPreviewApplyStatus,
    SourcePatchPreviewRiskLevel, SELF_DIAGNOSIS_FIX_PROPOSAL_DEMO_SCHEMA_VERSION,
    SELF_DIAGNOSIS_GENERATOR_INPUT_SCHEMA_VERSION,
    SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn attribution_contract() -> SelfAuditAttributionContract {
    SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("attribution contract validates")
}

fn bottleneck_input() -> ouroforge_core::SelfAuditBottleneckInput {
    self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("bottleneck input validates")
}

fn bottleneck_report() -> ouroforge_core::SelfAuditBottleneckReport {
    attribute_self_audit_bottlenecks(&attribution_contract(), &bottleneck_input())
        .expect("bottleneck attribution succeeds")
}

fn diagnosis_input() -> SelfDiagnosisGeneratorInput {
    SelfDiagnosisGeneratorInput {
        schema_version: SELF_DIAGNOSIS_GENERATOR_INPUT_SCHEMA_VERSION.to_string(),
        title_id: "era-i-engine-builder-deckbuilder".to_string(),
        verdict_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
        verdict_json: read_text("examples/real-title-dogfood-v1/run/verdict.json"),
        journal_ref: "examples/real-title-dogfood-v1/run/journal.md".to_string(),
        journal_markdown: read_text("examples/real-title-dogfood-v1/run/journal.md"),
        ledger_ref: "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
        ledger_jsonl: read_text("examples/real-title-dogfood-v1/run/ledger.jsonl"),
        loop_coverage_attribution_ref:
            "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json".to_string(),
        source_apply_ref: "source-apply:patch-preview.v1".to_string(),
        trust_gradient_ref: "trust-gradient:risk-tier-v1".to_string(),
        no_human_input: true,
    }
}

fn fix_proposal_input() -> SelfFixProposalGeneratorInput {
    let target_path = "crates/ouroforge-core/src/self_diagnosis_fix_proposal_contract.rs";
    let target_bytes = read_text(target_path);
    SelfFixProposalGeneratorInput {
        schema_version: SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION.to_string(),
        patch_preview_id: "m70-v62-generated-fix-preview-001".to_string(),
        proposal_id: "m70-v62-generated-engine-fix-proposal-001".to_string(),
        created_at: "2026-06-09T00:00:00Z".to_string(),
        base_branch: "main".to_string(),
        base_commit: "origin-main-fixture".to_string(),
        target_path: target_path.to_string(),
        target_before_hash: format!(
            "sha256:{}",
            source_patch_sandbox_sha256_hex(target_bytes.as_bytes())
        ),
        diff_text: format!(
            "diff --git a/{target_path} b/{target_path}\n--- a/{target_path}\n+++ b/{target_path}\n@@ -1,1 +1,2 @@\n //! Self-diagnosis and fix-proposal contract v1 (#2033 / Era L M70).\n+//! Scenario Coverage v62 keeps generated source fixes blocked until go/no-go.\n"
        ),
        expected_behavior_change: "Generated engine fix proposals remain source-apply previews that can be reviewed but not applied autonomously.".to_string(),
        no_self_apply: true,
    }
}

fn demo_input() -> SelfDiagnosisFixProposalDemoInput {
    SelfDiagnosisFixProposalDemoInput {
        title_id: "era-i-engine-builder-deckbuilder".to_string(),
        attribution_contract_json: read_text(
            "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
        ),
        bottleneck_input_json: read_text(
            "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
        ),
        diagnosis_input: diagnosis_input(),
        fix_proposal_input: fix_proposal_input(),
    }
}

#[test]
fn v62_matrix_records_diagnosis_and_proposal_regression_rows() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v62/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v62-self-diagnosis-fix-proposal-v1"
    );
    assert_eq!(matrix["coverageVersion"], 62);
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "planted-defect-diagnosis-root-cause",
        "diagnosis-existing-pipeline-only",
        "source-apply-proposal-scoping",
        "high-risk-proposal-blocked",
        "autonomous-demo-no-human",
        "regression-fail-closed",
        "coverage-v62-boundaries",
    ] {
        assert!(ids.contains(required), "missing v62 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(
            row["locks"].as_str().expect("locks").len() > 40,
            "row must explain locked behavior"
        );
    }

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "openchrome",
        "scenario verdicts",
        "four gates plus design-integrity",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "evolve",
        "source-apply",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "never auto-applied",
        "human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn v62_locks_planted_defect_diagnosis_correctness() {
    let report = bottleneck_report();
    let diagnosis = generate_self_diagnosis_record(&diagnosis_input(), &report)
        .expect("diagnosis generator succeeds");

    assert_eq!(diagnosis.attributed_milestone_id, "m68-real-title-run");
    assert_eq!(diagnosis.attributed_issue_ref, "#2025");
    let based_on = diagnosis.based_on_refs.join("\n").to_ascii_lowercase();
    for required in [
        "verdict.json",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "self-audit-bottleneck-attribution",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(based_on.contains(required), "diagnosis missing {required}");
    }

    let root = diagnosis
        .root_cause_hypotheses
        .first()
        .expect("root cause hypothesis");
    assert_eq!(
        root.hypothesis_id,
        "m68-real-title-run-four-gates-root-cause"
    );
    assert_eq!(root.confidence, "high");
    assert!(root
        .proposed_fix_scope
        .contains("Rust kernel/evaluator/source-apply"));
    assert!(root
        .proposed_fix_scope
        .contains("no Elixir executor change"));
    let chain = root.causal_chain.join("\n").to_ascii_lowercase();
    for stage in ["detect:", "explain:", "trace:", "attribute:", "propose:"] {
        assert!(chain.contains(stage), "causal chain missing {stage}");
    }
    assert!(chain.contains("without human input"));
    assert!(chain.contains("no self-application"));
}

#[test]
fn v62_locks_source_apply_proposal_scope_classification_and_blocking() {
    let diagnosis = generate_self_diagnosis_record(&diagnosis_input(), &bottleneck_report())
        .expect("diagnosis generator succeeds");
    let proposal = generate_source_apply_fix_proposal(&diagnosis, &fix_proposal_input())
        .expect("fix proposal generator succeeds");

    assert_eq!(proposal.schema_version, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION);
    assert_eq!(
        proposal.source_mutation_apply_status,
        SourcePatchPreviewApplyStatus::Blocked
    );
    assert_eq!(proposal.risk_level, SourcePatchPreviewRiskLevel::High);
    assert!(proposal.risk_ids.iter().any(|risk| risk == "STM-01"));
    assert!(proposal.risk_ids.iter().any(|risk| risk == "STM-04"));
    assert_eq!(proposal.diff_summary.diff_stats.files_changed, 1);
    assert!(proposal.targets.iter().all(|target| {
        target.path.starts_with("crates/")
            && target.file_class.contains("rust_trust_boundary")
            && target.classification_status == "restricted_separate_approval"
            && target
                .blocked_reasons
                .join("\n")
                .to_ascii_lowercase()
                .contains("human go/no-go")
    }));

    let evidence = proposal
        .linked_evidence
        .iter()
        .map(|evidence| evidence.path.as_str())
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase();
    for required in [
        "verdict.json",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(
            evidence.contains(required),
            "proposal evidence missing {required}"
        );
    }

    let read_model = proposal.read_model_prototype.as_ref().expect("read model");
    assert_eq!(read_model.status, "blocked");
    let forbidden = read_model.forbidden_actions.join("\n");
    assert!(forbidden.contains("auto_apply"));
    assert!(forbidden.contains("merge"));
    assert!(read_model
        .allowed_actions
        .iter()
        .any(|action| action == "read"));
    assert!(read_model
        .allowed_actions
        .iter()
        .any(|action| action == "review"));
    assert!(proposal
        .reviewer_checklist
        .join("\n")
        .to_ascii_lowercase()
        .contains("human go/no-go"));

    validate_source_patch_preview_artifact(&proposal, PatchDiffIntegrityLimits::default())
        .expect("source-apply preview remains valid");
}

#[test]
fn v62_demo_runs_autonomously_without_apply_authority() {
    let demo = run_self_diagnosis_fix_proposal_demo(&demo_input())
        .expect("demo chains attribution diagnosis and proposal");

    assert_eq!(
        demo.schema_version,
        SELF_DIAGNOSIS_FIX_PROPOSAL_DEMO_SCHEMA_VERSION
    );
    assert!(demo.autonomous_path_completed_without_human);
    assert!(!demo.source_mutation_applied);
    assert!(!demo.high_risk_auto_applied);
    assert_eq!(
        demo.source_apply_proposal.source_mutation_apply_status,
        SourcePatchPreviewApplyStatus::Blocked
    );
    assert_eq!(
        demo.source_apply_proposal.risk_level,
        SourcePatchPreviewRiskLevel::High
    );
    let boundary = demo.boundary.to_ascii_lowercase();
    for required in [
        "no human input",
        "no new verification engine",
        "no new data plane",
        "no self-apply",
        "high-risk",
        "human go/no-go",
        "human ring 2",
    ] {
        assert!(
            boundary.contains(required),
            "demo boundary missing {required}"
        );
    }
}

#[test]
fn v62_regression_inputs_fail_closed_before_apply_or_parallel_verifier_drift() {
    let report = bottleneck_report();

    let mut hidden_human = diagnosis_input();
    hidden_human.no_human_input = false;
    let error = generate_self_diagnosis_record(&hidden_human, &report)
        .expect_err("hidden human input rejected");
    assert!(error.to_string().contains("noHumanInput=true"));

    let mut auto_apply = diagnosis_input();
    auto_apply.ledger_jsonl = auto_apply
        .ledger_jsonl
        .replace("\"highRiskAutoApply\":false", "\"highRiskAutoApply\":true");
    let error = generate_self_diagnosis_record(&auto_apply, &report)
        .expect_err("high-risk auto-apply drift rejected");
    assert!(error.to_string().contains("high-risk"));

    let diagnosis = generate_self_diagnosis_record(&diagnosis_input(), &report)
        .expect("diagnosis generator succeeds");
    let mut non_source = fix_proposal_input();
    non_source.target_path = "examples/real-title-dogfood-v1/run/verdict.json".to_string();
    non_source.diff_text = "diff --git a/examples/real-title-dogfood-v1/run/verdict.json b/examples/real-title-dogfood-v1/run/verdict.json\n--- a/examples/real-title-dogfood-v1/run/verdict.json\n+++ b/examples/real-title-dogfood-v1/run/verdict.json\n@@ -1,1 +1,2 @@\n {\n+  \"drift\": true,\n".to_string();
    let error = generate_source_apply_fix_proposal(&diagnosis, &non_source)
        .expect_err("non-source target rejected");
    assert!(error.to_string().contains("crates/"));

    let mut contract = SelfDiagnosisFixProposalContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-diagnosis-fix-proposal-v1/contract.fixture.json",
    ))
    .expect("contract fixture validates");
    contract.source_apply_proposal.reviewer_checklist = vec![
        "Confirm target freshness before review.".to_string(),
        "Confirm evidence refs.".to_string(),
    ];
    let error = contract
        .validate()
        .expect_err("missing human go/no-go review drift rejected");
    assert!(error.to_string().contains("human go/no-go"));
}

#[test]
fn v62_docs_preserve_test_only_autonomy_boundaries() {
    let doc = read_text("docs/scenario-coverage-v62-self-diagnosis-fix-proposal.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "scenario coverage v62",
        "cargo test --workspace --jobs 2",
        "test-only rust coverage",
        "openchrome",
        "scenario verdicts",
        "four gates plus",
        "design-integrity",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "evolve",
        "source-apply",
        "trust-gradient",
        "does not introduce a verification engine",
        "persistent store",
        "data plane",
        "zero human input",
        "never\nauto-applied",
        "thin human go/no-go",
        "fun/taste and\nrelease go/no-go remain human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
}

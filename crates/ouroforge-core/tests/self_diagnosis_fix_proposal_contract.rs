//! Self-diagnosis and fix-proposal contract tests for #2033 / Era L M70.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    attribute_self_audit_bottlenecks, generate_self_diagnosis_record,
    generate_source_apply_fix_proposal, self_audit_bottleneck_input_from_json_str,
    source_patch_sandbox_sha256_hex, validate_source_patch_preview_artifact,
    PatchDiffIntegrityLimits, SelfAuditAttributionContract, SelfDiagnosisFixProposalContract,
    SelfDiagnosisGeneratorInput, SelfFixProposalGeneratorInput, SourcePatchPreviewApplyStatus,
    SourcePatchPreviewRiskLevel, SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION,
    SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION,
    SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION,
    SELF_DIAGNOSIS_GENERATOR_INPUT_SCHEMA_VERSION,
    SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn generator_input() -> SelfDiagnosisGeneratorInput {
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

fn bottleneck_report() -> ouroforge_core::SelfAuditBottleneckReport {
    let attribution = SelfAuditAttributionContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json",
    ))
    .expect("attribution contract validates");
    assert_eq!(
        attribution.schema_version,
        SELF_AUDIT_ATTRIBUTION_CONTRACT_SCHEMA_VERSION
    );

    let bottleneck_input = self_audit_bottleneck_input_from_json_str(&read_text(
        "examples/real-title-dogfood-v1/bottleneck-attribution-v1/planted-failure.fixture.json",
    ))
    .expect("bottleneck input validates");
    assert_eq!(
        bottleneck_input.schema_version,
        SELF_AUDIT_BOTTLENECK_ATTRIBUTION_SCHEMA_VERSION
    );
    attribute_self_audit_bottlenecks(&attribution, &bottleneck_input)
        .expect("bottleneck attribution succeeds")
}

fn fix_proposal_input() -> SelfFixProposalGeneratorInput {
    let target_path = "crates/ouroforge-core/src/self_diagnosis_fix_proposal_contract.rs";
    let target_bytes = read_text(target_path);
    SelfFixProposalGeneratorInput {
        schema_version: SELF_FIX_PROPOSAL_GENERATOR_INPUT_SCHEMA_VERSION.to_string(),
        patch_preview_id: "m70-generated-fix-preview-001".to_string(),
        proposal_id: "m70-generated-engine-fix-proposal-001".to_string(),
        created_at: "2026-06-09T00:00:00Z".to_string(),
        base_branch: "main".to_string(),
        base_commit: "origin-main-fixture".to_string(),
        target_path: target_path.to_string(),
        target_before_hash: format!(
            "sha256:{}",
            source_patch_sandbox_sha256_hex(target_bytes.as_bytes())
        ),
        diff_text: format!(
            "diff --git a/{target_path} b/{target_path}\n--- a/{target_path}\n+++ b/{target_path}\n@@ -1,1 +1,2 @@\n //! Self-diagnosis and fix-proposal contract v1 (#2033 / Era L M70).\n+//! Generated M70 fix proposals stay blocked in source-apply until human go/no-go.\n"
        ),
        expected_behavior_change: "Engine diagnosis proposals carry an explicit source-apply blocked preview for the attributed root cause without mutating sources.".to_string(),
        no_self_apply: true,
    }
}

fn contract() -> SelfDiagnosisFixProposalContract {
    SelfDiagnosisFixProposalContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-diagnosis-fix-proposal-v1/contract.fixture.json",
    ))
    .expect("contract fixture validates")
}

#[test]
fn contract_fixture_links_diagnosis_to_existing_pipeline_evidence() {
    let contract = contract();
    assert_eq!(
        contract.schema_version,
        SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(contract.title_id, "era-i-engine-builder-deckbuilder");
    assert_eq!(
        contract.diagnosis.attributed_milestone_id,
        "m68-real-title-run"
    );
    assert_eq!(contract.diagnosis.attributed_issue_ref, "#2025");

    let refs = contract
        .diagnosis
        .based_on_refs
        .join("\n")
        .to_ascii_lowercase();
    for required in [
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "self-audit-bottleneck-attribution",
        "source-apply",
        "trust-gradient",
    ] {
        assert!(refs.contains(required), "missing diagnosis ref {required}");
    }

    for hypothesis in &contract.diagnosis.root_cause_hypotheses {
        assert!(hypothesis.causal_chain.len() >= 3);
        assert!(!hypothesis.evidence_refs.is_empty());
        assert!(hypothesis.proposed_fix_scope.contains("Rust"));
    }
}

#[test]
fn fix_proposal_reuses_source_apply_preview_and_blocks_application() {
    let contract = contract();
    let proposal = &contract.source_apply_proposal;
    assert_eq!(proposal.schema_version, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION);
    assert_eq!(
        proposal.source_mutation_apply_status,
        SourcePatchPreviewApplyStatus::Blocked
    );
    assert_eq!(proposal.risk_level, SourcePatchPreviewRiskLevel::High);
    assert!(proposal
        .targets
        .iter()
        .any(|target| target.file_class.contains("rust")
            && target.classification_status.contains("review_held")));
    assert!(proposal
        .reviewer_checklist
        .join("\n")
        .to_ascii_lowercase()
        .contains("human go/no-go"));
    let forbidden = proposal
        .read_model_prototype
        .as_ref()
        .expect("read model")
        .forbidden_actions
        .join("\n");
    assert!(forbidden.contains("auto_apply"));
    assert!(forbidden.contains("merge"));
}

#[test]
fn high_risk_or_parallel_verifier_drift_fails_closed() {
    let mut missing_human_gate = contract();
    missing_human_gate.source_apply_proposal.reviewer_checklist = vec![
        "Confirm target freshness before review.".to_string(),
        "Confirm evidence refs.".to_string(),
    ];
    let error = missing_human_gate
        .validate()
        .expect_err("high-risk checklist drift rejected");
    assert!(error.to_string().contains("human go/no-go"));

    let mut boundary_drift = contract();
    boundary_drift.boundary = boundary_drift.boundary.replace(
        "no new verification engine",
        "new verification engine allowed",
    );
    let error = boundary_drift
        .validate()
        .expect_err("boundary drift rejected");
    assert!(error.to_string().contains("no new verification engine"));
}

#[test]
fn docs_preserve_m70_scope_and_anchor_boundaries() {
    let doc = read_text("docs/self-diagnosis-fix-proposal-contract-v1.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "patch-preview.v1",
        "sourcemutationapplystatus",
        "blocked",
        "ledger.jsonl",
        "journal.md",
        "verdict.json",
        "loop-coverage attribution",
        "source-apply",
        "trust-gradient",
        "does not apply fixes",
        "no new verification engine",
        "no new data plane",
        "elixir executor remains the control plane",
        "fun/taste and release go/no-go stay human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
}

#[test]
fn generator_produces_evidence_linked_root_cause_for_planted_defect() {
    let record = generate_self_diagnosis_record(&generator_input(), &bottleneck_report())
        .expect("diagnosis generator succeeds without human input");

    assert_eq!(record.attributed_milestone_id, "m68-real-title-run");
    assert_eq!(record.attributed_issue_ref, "#2025");
    assert!(record
        .based_on_refs
        .iter()
        .any(|reference| reference.contains("ledger.jsonl")));
    assert!(record
        .based_on_refs
        .iter()
        .any(|reference| reference.contains("journal.md")));
    assert!(record
        .based_on_refs
        .iter()
        .any(|reference| reference.contains("loop-coverage")));

    let first = &record.root_cause_hypotheses[0];
    assert_eq!(
        first.hypothesis_id,
        "m68-real-title-run-four-gates-root-cause"
    );
    let chain = first.causal_chain.join("\n").to_ascii_lowercase();
    assert!(chain.contains("provenance-incomplete"));
    assert!(chain.contains("source-apply"));
    assert!(chain.contains("trust-gradient"));
    assert!(chain.contains("no self-application"));
    assert!(first
        .evidence_refs
        .iter()
        .any(|reference| reference.contains("verdict.json")));
    assert!(first
        .evidence_refs
        .iter()
        .any(|reference| reference.contains("release-provenance.complete.json")));
    assert!(first.proposed_fix_scope.contains("Rust"));
}

#[test]
fn generator_fails_closed_on_hidden_human_or_new_store_drift() {
    let report = bottleneck_report();

    let mut human_input = generator_input();
    human_input.no_human_input = false;
    let error = generate_self_diagnosis_record(&human_input, &report)
        .expect_err("human input drift rejected");
    assert!(error.to_string().contains("noHumanInput=true"));

    let mut new_store = generator_input();
    new_store.ledger_jsonl = new_store
        .ledger_jsonl
        .replace("\"highRiskAutoApply\":false", "\"highRiskAutoApply\":true");
    let error = generate_self_diagnosis_record(&new_store, &report)
        .expect_err("high-risk auto-apply drift rejected");
    assert!(error.to_string().contains("high-risk"));
}

#[test]
fn fix_proposal_generator_emits_blocked_source_apply_preview_from_diagnosis() {
    let diagnosis = generate_self_diagnosis_record(&generator_input(), &bottleneck_report())
        .expect("diagnosis generator succeeds");
    let proposal = generate_source_apply_fix_proposal(&diagnosis, &fix_proposal_input())
        .expect("fix proposal generator succeeds");

    assert_eq!(proposal.schema_version, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION);
    assert_eq!(
        proposal.source_mutation_apply_status,
        SourcePatchPreviewApplyStatus::Blocked
    );
    assert_eq!(proposal.risk_level, SourcePatchPreviewRiskLevel::High);
    assert_eq!(
        proposal.targets[0].classification_status,
        "restricted_separate_approval"
    );
    assert!(proposal.targets[0]
        .file_class
        .contains("rust_trust_boundary"));
    assert!(proposal.targets[0]
        .blocked_reasons
        .join("\n")
        .to_ascii_lowercase()
        .contains("human go/no-go"));
    assert!(proposal
        .linked_evidence
        .iter()
        .any(|evidence| evidence.path.contains("verdict.json")));
    assert!(proposal
        .linked_evidence
        .iter()
        .any(|evidence| evidence.path.contains("journal.md")));
    assert!(proposal
        .linked_evidence
        .iter()
        .any(|evidence| evidence.path.contains("ledger.jsonl")));
    assert!(proposal
        .linked_evidence
        .iter()
        .any(|evidence| evidence.path.contains("loop-coverage")));
    let forbidden = proposal
        .read_model_prototype
        .as_ref()
        .expect("read model")
        .forbidden_actions
        .join("\n");
    assert!(forbidden.contains("auto_apply"));
    assert!(forbidden.contains("merge"));

    validate_source_patch_preview_artifact(&proposal, PatchDiffIntegrityLimits::default())
        .expect("generated proposal satisfies source-apply preview validation");
}

#[test]
fn fix_proposal_generator_fails_closed_on_apply_or_non_source_target_drift() {
    let diagnosis = generate_self_diagnosis_record(&generator_input(), &bottleneck_report())
        .expect("diagnosis generator succeeds");

    let mut self_apply = fix_proposal_input();
    self_apply.no_self_apply = false;
    let error = generate_source_apply_fix_proposal(&diagnosis, &self_apply)
        .expect_err("self-apply drift rejected");
    assert!(error.to_string().contains("noSelfApply=true"));

    let mut non_source = fix_proposal_input();
    non_source.target_path = "examples/real-title-dogfood-v1/run/verdict.json".to_string();
    non_source.diff_text = "diff --git a/examples/real-title-dogfood-v1/run/verdict.json b/examples/real-title-dogfood-v1/run/verdict.json\n--- a/examples/real-title-dogfood-v1/run/verdict.json\n+++ b/examples/real-title-dogfood-v1/run/verdict.json\n@@ -1,1 +1,2 @@\n {\n+  \"drift\": true,\n".to_string();
    let error = generate_source_apply_fix_proposal(&diagnosis, &non_source)
        .expect_err("non-source engine target rejected");
    assert!(error.to_string().contains("crates/"));
}

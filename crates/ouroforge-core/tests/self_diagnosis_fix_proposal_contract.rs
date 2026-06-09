//! Self-diagnosis and fix-proposal contract tests for #2033 / Era L M70.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    SelfDiagnosisFixProposalContract, SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel,
    SELF_DIAGNOSIS_FIX_PROPOSAL_CONTRACT_SCHEMA_VERSION, SOURCE_PATCH_PREVIEW_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
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

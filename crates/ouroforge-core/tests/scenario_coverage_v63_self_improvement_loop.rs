//! Scenario Coverage v63 regression suite for #2041 / Era L M71.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    queue_self_improvement_human_go_no_go, run_self_improvement_reverify_apply_loop,
    trust_gradient_auto_apply::AutoApplyOutcome, SelfImprovementApplyLoopInput,
    SelfImprovementApplyLoopOutcome, SelfImprovementHumanGoNoGoQueueInput,
    SelfImprovementHumanGoNoGoQueueStatus, SelfImprovementReverifyEvidence, SelfImprovementRoute,
    SelfImprovementRoutingInput, SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel,
    SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION,
    SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION,
    SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read_text(path)).unwrap_or_else(|err| panic!("parse {path}: {err}"))
}

fn refs() -> Vec<String> {
    vec![
        "openchrome:seeds/dogfood-deckbuilder.yaml".to_string(),
        "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
        "examples/real-title-dogfood-v1/run/journal.md".to_string(),
        "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
        "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json".to_string(),
        "source-apply:patch-preview.v1".to_string(),
        "trust-gradient:risk-tier-v1:auto-apply-v1".to_string(),
        "source-apply:rollback-snapshot-v1".to_string(),
        "trust-gradient:kill-switch-v1".to_string(),
    ]
}

fn reverify_evidence() -> SelfImprovementReverifyEvidence {
    SelfImprovementReverifyEvidence {
        openchrome_rerun_ref: "openchrome:seeds/dogfood-deckbuilder.yaml".to_string(),
        verdict_ref: "examples/real-title-dogfood-v1/run/verdict.json".to_string(),
        journal_ref: "examples/real-title-dogfood-v1/run/journal.md".to_string(),
        ledger_ref: "examples/real-title-dogfood-v1/run/ledger.jsonl".to_string(),
        loop_coverage_attribution_ref:
            "examples/real-title-dogfood-v1/refs/loop-coverage-attribution.json".to_string(),
        source_apply_ref: "source-apply:patch-preview.v1".to_string(),
        trust_gradient_ref: "trust-gradient:risk-tier-v1:auto-apply-v1".to_string(),
        rollback_ref: "source-apply:rollback-snapshot-v1".to_string(),
        kill_switch_ref: "trust-gradient:kill-switch-v1".to_string(),
        mechanical_gate_passed: true,
        runtime_gate_passed: true,
        visual_gate_passed: true,
        semantic_gate_passed: true,
        design_integrity_passed: true,
        no_human_input: true,
        no_new_verification_engine: true,
        no_new_data_plane: true,
    }
}

fn routing_input(
    proposal_ref: &str,
    risk_level: SourcePatchPreviewRiskLevel,
    source_affecting: bool,
    reversible: bool,
    trust_gradient_outcome: AutoApplyOutcome,
) -> SelfImprovementRoutingInput {
    SelfImprovementRoutingInput {
        schema_version: SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: proposal_ref.to_string(),
        source_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        risk_level,
        source_affecting,
        reversible,
        trust_gradient_outcome,
        kill_switch_engaged: false,
        reverify_evidence: reverify_evidence(),
    }
}

fn apply_input(
    proposal_ref: &str,
    risk_level: SourcePatchPreviewRiskLevel,
    source_affecting: bool,
    reversible: bool,
    trust_gradient_outcome: AutoApplyOutcome,
) -> SelfImprovementApplyLoopInput {
    SelfImprovementApplyLoopInput {
        schema_version: SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: proposal_ref.to_string(),
        attributed_milestone_id: "m71-self-improvement-loop".to_string(),
        routing_input: routing_input(
            proposal_ref,
            risk_level,
            source_affecting,
            reversible,
            trust_gradient_outcome,
        ),
        before_evidence_refs: refs(),
        after_evidence_refs: refs(),
        before_evidence_score: 7,
        after_evidence_score: 9,
        regression_detected: false,
        no_human_input: true,
        no_new_data_plane: true,
    }
}

#[test]
fn v63_matrix_records_self_improvement_regression_rows() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v63/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v63-self-improvement-loop-v1"
    );
    assert_eq!(matrix["coverageVersion"], 63);
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "reverify-gates-design-integrity",
        "low-risk-auto-apply-improves-evidence",
        "regression-rolls-back",
        "high-risk-queues-thin-human-gate",
        "unverified-or-irreversible-auto-apply-blocked",
        "full-loop-demo-two-risk-paths",
        "coverage-v63-boundaries",
    ] {
        assert!(ids.contains(required), "missing v63 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 60);
    }
}

#[test]
fn v63_low_risk_auto_apply_requires_reverify_trust_and_evidence_improvement() {
    let report = run_self_improvement_reverify_apply_loop(&apply_input(
        "source-apply:patch-preview.v1#v63-low-risk",
        SourcePatchPreviewRiskLevel::Low,
        false,
        true,
        AutoApplyOutcome::AutoApplied,
    ))
    .expect("low-risk path routes");

    assert_eq!(report.outcome, SelfImprovementApplyLoopOutcome::AutoApplied);
    assert_eq!(
        report.routing_decision.route,
        SelfImprovementRoute::AutoApplyEligible
    );
    assert!(report.improved_attributed_milestone_evidence);
    assert!(report.source_mutation_applied);
    assert!(!report.human_input_required);
}

#[test]
fn v63_regression_rejects_and_rolls_back_instead_of_promoting() {
    let mut input = apply_input(
        "source-apply:patch-preview.v1#v63-regression",
        SourcePatchPreviewRiskLevel::Low,
        false,
        true,
        AutoApplyOutcome::AutoApplied,
    );
    input.after_evidence_score = 5;
    input.regression_detected = true;

    let report = run_self_improvement_reverify_apply_loop(&input)
        .expect("regression produces rollback report");
    assert_eq!(
        report.outcome,
        SelfImprovementApplyLoopOutcome::RejectedRolledBack
    );
    assert!(!report.source_mutation_applied);
    assert!(report
        .rollback_command
        .as_ref()
        .is_some_and(|cmd| cmd.contains("rollback")));
}

#[test]
fn v63_high_risk_source_affecting_fix_queues_not_auto_applies() {
    let queue = queue_self_improvement_human_go_no_go(&SelfImprovementHumanGoNoGoQueueInput {
        schema_version: SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION.to_string(),
        apply_loop_input: apply_input(
            "source-apply:patch-preview.v1#v63-high-risk",
            SourcePatchPreviewRiskLevel::High,
            true,
            true,
            AutoApplyOutcome::ManualFallback,
        ),
        queue_provenance_ref: "ledger.jsonl:v63-high-risk-queued".to_string(),
        unrelated_loop_work_refs: refs(),
        verification_strength_score: 91,
        broadening_threshold_score: 90,
        one_click_only: true,
        no_debugging_session: true,
        no_new_data_plane: true,
    })
    .expect("verified high-risk path queues");

    assert_eq!(
        queue.status,
        SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision
    );
    assert!(queue.verified_reversible_high_risk);
    assert!(!queue.blocks_unrelated_loop_work);
    assert_eq!(queue.one_click_actions, ["go", "no-go"]);
}

#[test]
fn v63_unverified_irreversible_or_hidden_human_auto_apply_fails_closed() {
    let mut failed_gate = apply_input(
        "source-apply:patch-preview.v1#v63-failed-gate",
        SourcePatchPreviewRiskLevel::Low,
        false,
        true,
        AutoApplyOutcome::AutoApplied,
    );
    failed_gate
        .routing_input
        .reverify_evidence
        .design_integrity_passed = false;
    let report = run_self_improvement_reverify_apply_loop(&failed_gate)
        .expect("failed gate is a blocked report");
    assert_eq!(report.outcome, SelfImprovementApplyLoopOutcome::Blocked);
    assert!(!report.source_mutation_applied);

    let irreversible = run_self_improvement_reverify_apply_loop(&apply_input(
        "source-apply:patch-preview.v1#v63-irreversible",
        SourcePatchPreviewRiskLevel::Low,
        false,
        false,
        AutoApplyOutcome::AutoApplied,
    ))
    .expect("irreversible path routes away from auto-apply");
    assert_eq!(
        irreversible.outcome,
        SelfImprovementApplyLoopOutcome::HumanGoNoGoQueued
    );
    assert!(!irreversible.source_mutation_applied);

    let mut hidden_human = apply_input(
        "source-apply:patch-preview.v1#v63-hidden-human",
        SourcePatchPreviewRiskLevel::Low,
        false,
        true,
        AutoApplyOutcome::AutoApplied,
    );
    hidden_human.no_human_input = false;
    let error = run_self_improvement_reverify_apply_loop(&hidden_human)
        .expect_err("hidden human drift rejected");
    assert!(error.to_string().contains("no human input"));
}

#[test]
fn v63_docs_preserve_boundaries_and_verification_command() {
    let doc = read_text("docs/scenario-coverage-v63-self-improvement-loop.md").to_ascii_lowercase();
    for required in [
        "coverage v63",
        "cargo test --workspace --jobs 2",
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
        "rollback",
        "kill-switch",
        "zero human input",
        "never auto-applied",
        "one-click human go/no-go",
        "does not introduce a verification engine",
        "data plane",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

//! Re-verify/apply loop tests for #2038 / Era L M71.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    run_self_improvement_reverify_apply_loop, trust_gradient_auto_apply::AutoApplyOutcome,
    SelfImprovementApplyLoopInput, SelfImprovementApplyLoopOutcome,
    SelfImprovementReverifyEvidence, SelfImprovementRoute, SelfImprovementRoutingInput,
    SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel,
    SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION,
    SELF_IMPROVEMENT_APPLY_LOOP_REPORT_SCHEMA_VERSION,
    SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
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

fn routing_input() -> SelfImprovementRoutingInput {
    SelfImprovementRoutingInput {
        schema_version: SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: "source-apply:patch-preview.v1#m71-low-risk-reverify".to_string(),
        source_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        risk_level: SourcePatchPreviewRiskLevel::Low,
        source_affecting: false,
        reversible: true,
        trust_gradient_outcome: AutoApplyOutcome::AutoApplied,
        kill_switch_engaged: false,
        reverify_evidence: reverify_evidence(),
    }
}

fn loop_input() -> SelfImprovementApplyLoopInput {
    SelfImprovementApplyLoopInput {
        schema_version: SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: "source-apply:patch-preview.v1#m71-low-risk-reverify".to_string(),
        attributed_milestone_id: "m68-real-title-run".to_string(),
        routing_input: routing_input(),
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
fn reversible_low_risk_fix_auto_applies_without_human_and_improves_evidence() {
    let report =
        run_self_improvement_reverify_apply_loop(&loop_input()).expect("low-risk fix auto-routes");

    assert_eq!(
        report.schema_version,
        SELF_IMPROVEMENT_APPLY_LOOP_REPORT_SCHEMA_VERSION
    );
    assert_eq!(report.outcome, SelfImprovementApplyLoopOutcome::AutoApplied);
    assert_eq!(
        report.routing_decision.route,
        SelfImprovementRoute::AutoApplyEligible
    );
    assert!(report.source_mutation_applied);
    assert!(!report.human_input_required);
    assert!(report.improved_attributed_milestone_evidence);
    assert_eq!(report.attributed_milestone_id, "m68-real-title-run");
}

#[test]
fn regressing_fix_is_rejected_and_rolled_back() {
    let mut input = loop_input();
    input.after_evidence_score = 4;
    input.regression_detected = true;

    let report = run_self_improvement_reverify_apply_loop(&input)
        .expect("regression produces rollback report");
    assert_eq!(
        report.outcome,
        SelfImprovementApplyLoopOutcome::RejectedRolledBack
    );
    assert!(!report.source_mutation_applied);
    assert!(report.rollback_command.as_ref().is_some_and(|command| {
        command.contains("ouroforge rollback") && command.contains("rollback-snapshot")
    }));
    assert!(!report.improved_attributed_milestone_evidence);
}

#[test]
fn high_risk_source_affecting_fix_queues_human_go_no_go_without_auto_apply() {
    let mut input = loop_input();
    input.proposal_ref = "source-apply:patch-preview.v1#m71-source-tail".to_string();
    input.routing_input.proposal_ref = input.proposal_ref.clone();
    input.routing_input.risk_level = SourcePatchPreviewRiskLevel::High;
    input.routing_input.source_affecting = true;
    input.routing_input.trust_gradient_outcome = AutoApplyOutcome::ManualFallback;

    let report = run_self_improvement_reverify_apply_loop(&input)
        .expect("high-risk tail routes to human go/no-go");
    assert_eq!(
        report.outcome,
        SelfImprovementApplyLoopOutcome::HumanGoNoGoQueued
    );
    assert_eq!(
        report.routing_decision.route,
        SelfImprovementRoute::HumanGoNoGo
    );
    assert!(!report.source_mutation_applied);
    assert!(report.human_input_required);
}

#[test]
fn failed_gate_blocks_apply_even_for_low_risk_fix() {
    let mut input = loop_input();
    input.routing_input.reverify_evidence.runtime_gate_passed = false;

    let report =
        run_self_improvement_reverify_apply_loop(&input).expect("failed gate blocks route");
    assert_eq!(report.outcome, SelfImprovementApplyLoopOutcome::Blocked);
    assert!(!report.source_mutation_applied);
    assert!(!report.human_input_required);
}

#[test]
fn hidden_human_or_new_store_drift_fails_closed() {
    let mut input = loop_input();
    input.no_human_input = false;
    let error = run_self_improvement_reverify_apply_loop(&input)
        .expect_err("hidden human dependency rejected");
    assert!(error.to_string().contains("no human input"));

    let mut input = loop_input();
    input.no_new_data_plane = false;
    let error =
        run_self_improvement_reverify_apply_loop(&input).expect_err("new data plane rejected");
    assert!(error.to_string().contains("no new data plane"));
}

#[test]
fn docs_record_reverify_apply_loop_without_new_store() {
    let doc = read_text("docs/self-improvement-loop-contract-v1.md").to_ascii_lowercase();
    for required in [
        "re-verify-then-apply loop",
        "zero human input",
        "rejected and rolled back",
        "thin human go/no-go",
        "does not create a new store",
        "verification engine",
        "data plane",
        "ledger.jsonl",
        "journal.md",
        "verdict",
        "loop-coverage attribution",
        "source-apply",
        "trust-gradient",
        "rollback",
        "kill-switch",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

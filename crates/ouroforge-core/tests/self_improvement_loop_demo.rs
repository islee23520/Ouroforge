//! End-to-end self-improvement loop demo tests for #2040 / Era L M71.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    queue_self_improvement_human_go_no_go, run_self_improvement_reverify_apply_loop,
    trust_gradient_auto_apply::AutoApplyOutcome, SelfImprovementApplyLoopInput,
    SelfImprovementApplyLoopOutcome, SelfImprovementHumanGoNoGoQueueInput,
    SelfImprovementHumanGoNoGoQueueStatus, SelfImprovementReverifyEvidence,
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

fn demo_fixture() -> Value {
    serde_json::from_str(&read_text(
        "examples/real-title-dogfood-v1/self-improvement-loop-demo-v1/demo.fixture.json",
    ))
    .expect("demo fixture parses")
}

fn refs() -> Vec<String> {
    demo_fixture()["beforeAfterEvidenceRefs"]
        .as_array()
        .expect("refs array")
        .iter()
        .map(|value| value.as_str().expect("ref string").to_string())
        .collect()
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
    trust_gradient_outcome: AutoApplyOutcome,
) -> SelfImprovementRoutingInput {
    SelfImprovementRoutingInput {
        schema_version: SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: proposal_ref.to_string(),
        source_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        risk_level,
        source_affecting,
        reversible: true,
        trust_gradient_outcome,
        kill_switch_engaged: false,
        reverify_evidence: reverify_evidence(),
    }
}

fn apply_input(
    proposal_ref: &str,
    milestone_id: &str,
    risk_level: SourcePatchPreviewRiskLevel,
    source_affecting: bool,
    trust_gradient_outcome: AutoApplyOutcome,
) -> SelfImprovementApplyLoopInput {
    SelfImprovementApplyLoopInput {
        schema_version: SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: proposal_ref.to_string(),
        attributed_milestone_id: milestone_id.to_string(),
        routing_input: routing_input(
            proposal_ref,
            risk_level,
            source_affecting,
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
fn demo_fixture_chains_detect_explain_trace_attribute_propose_reverify_apply_and_queue() {
    let fixture = demo_fixture();
    assert_eq!(fixture["schemaVersion"], "self-improvement-loop-demo-v1");
    assert_eq!(fixture["titleId"], "era-i-engine-builder-deckbuilder");
    assert_eq!(fixture["seedPath"], "seeds/dogfood-deckbuilder.yaml");

    let stages: Vec<_> = fixture["stageChain"]
        .as_array()
        .expect("stage chain")
        .iter()
        .map(|stage| stage["stage"].as_str().expect("stage name"))
        .collect();
    assert_eq!(
        stages,
        [
            "detect",
            "explain",
            "trace",
            "attribute",
            "propose",
            "re-verify",
            "apply",
            "queue"
        ]
    );
}

#[test]
fn low_risk_path_auto_applies_after_before_after_evidence_improves() {
    let report = run_self_improvement_reverify_apply_loop(&apply_input(
        "source-apply:patch-preview.v1#m71-low-risk-demo",
        "m68-real-title-run",
        SourcePatchPreviewRiskLevel::Low,
        false,
        AutoApplyOutcome::AutoApplied,
    ))
    .expect("low-risk demo path auto-applies");

    assert_eq!(report.outcome, SelfImprovementApplyLoopOutcome::AutoApplied);
    assert!(report.improved_attributed_milestone_evidence);
    assert!(report.source_mutation_applied);
    assert!(!report.human_input_required);
}

#[test]
fn high_risk_path_is_verified_reversible_and_queued_without_auto_apply() {
    let queue = queue_self_improvement_human_go_no_go(&SelfImprovementHumanGoNoGoQueueInput {
        schema_version: SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION.to_string(),
        apply_loop_input: apply_input(
            "source-apply:patch-preview.v1#m71-high-risk-demo",
            "m71-high-risk-tail",
            SourcePatchPreviewRiskLevel::High,
            true,
            AutoApplyOutcome::ManualFallback,
        ),
        queue_provenance_ref: "ledger.jsonl:m71-high-risk-demo-queued".to_string(),
        unrelated_loop_work_refs: refs(),
        verification_strength_score: 95,
        broadening_threshold_score: 90,
        one_click_only: true,
        no_debugging_session: true,
        no_new_data_plane: true,
    })
    .expect("high-risk demo path queues");

    assert_eq!(
        queue.status,
        SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision
    );
    assert!(queue.verified_reversible_high_risk);
    assert_eq!(queue.one_click_actions, ["go", "no-go"]);
    assert!(!queue.blocks_unrelated_loop_work);
    assert!(queue.decision_provenance_ref.is_none());
}

#[test]
fn demo_fixture_preserves_autonomy_invariants_and_existing_pipeline_only() {
    let fixture = demo_fixture();
    let invariants = &fixture["autonomyInvariants"];
    assert_eq!(invariants["noHumanInputOnAutonomousPath"], true);
    assert_eq!(invariants["noNewVerificationEngine"], true);
    assert_eq!(invariants["noNewDataPlane"], true);
    assert_eq!(invariants["highRiskAutoApplied"], false);
    assert_eq!(invariants["unrelatedLoopWorkBlocksOnHumanGate"], false);

    let boundary = fixture["boundary"]
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
        "rollback",
        "kill-switch",
        "no new verification engine",
        "no new data plane",
        "no new store",
        "human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
}

#[test]
fn demo_docs_record_real_title_command_and_two_risk_paths() {
    let doc = read_text("docs/self-improvement-loop-demo-v1.md").to_ascii_lowercase();
    for required in [
        "engine-builder deckbuilder",
        "detect",
        "explain",
        "trace",
        "attribute",
        "propose",
        "re-verify",
        "apply the low-risk reversible fix",
        "queue the high-risk/source-affecting reversible fix",
        "cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2",
        "does not create a new store",
        "verification engine",
        "data plane",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

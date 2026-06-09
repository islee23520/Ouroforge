//! Thin high-risk human go/no-go queue tests for #2039 / Era L M71.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    queue_self_improvement_human_go_no_go, record_self_improvement_human_go_no_go_decision,
    trust_gradient_auto_apply::AutoApplyOutcome, SelfImprovementApplyLoopInput,
    SelfImprovementHumanGoNoGoDecision, SelfImprovementHumanGoNoGoQueueInput,
    SelfImprovementHumanGoNoGoQueueStatus, SelfImprovementReverifyEvidence,
    SelfImprovementRoutingInput, SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel,
    SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION,
    SELF_IMPROVEMENT_HUMAN_GO_NO_GO_DECISION_SCHEMA_VERSION,
    SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION,
    SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_ITEM_SCHEMA_VERSION,
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

fn high_risk_routing_input() -> SelfImprovementRoutingInput {
    SelfImprovementRoutingInput {
        schema_version: SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: "source-apply:patch-preview.v1#m71-high-risk-tail".to_string(),
        source_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        risk_level: SourcePatchPreviewRiskLevel::High,
        source_affecting: true,
        reversible: true,
        trust_gradient_outcome: AutoApplyOutcome::ManualFallback,
        kill_switch_engaged: false,
        reverify_evidence: reverify_evidence(),
    }
}

fn apply_loop_input() -> SelfImprovementApplyLoopInput {
    SelfImprovementApplyLoopInput {
        schema_version: SELF_IMPROVEMENT_APPLY_LOOP_INPUT_SCHEMA_VERSION.to_string(),
        proposal_ref: "source-apply:patch-preview.v1#m71-high-risk-tail".to_string(),
        attributed_milestone_id: "m71-high-risk-tail".to_string(),
        routing_input: high_risk_routing_input(),
        before_evidence_refs: refs(),
        after_evidence_refs: refs(),
        before_evidence_score: 7,
        after_evidence_score: 9,
        regression_detected: false,
        no_human_input: true,
        no_new_data_plane: true,
    }
}

fn queue_input() -> SelfImprovementHumanGoNoGoQueueInput {
    SelfImprovementHumanGoNoGoQueueInput {
        schema_version: SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION.to_string(),
        apply_loop_input: apply_loop_input(),
        queue_provenance_ref: "ledger.jsonl:m71-human-gate-queued".to_string(),
        unrelated_loop_work_refs: refs(),
        verification_strength_score: 95,
        broadening_threshold_score: 90,
        one_click_only: true,
        no_debugging_session: true,
        no_new_data_plane: true,
    }
}

#[test]
fn verified_reversible_high_risk_fix_queues_as_one_click_gate() {
    let item = queue_self_improvement_human_go_no_go(&queue_input())
        .expect("verified high-risk tail queues");

    assert_eq!(
        item.schema_version,
        SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_ITEM_SCHEMA_VERSION
    );
    assert_eq!(
        item.status,
        SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision
    );
    assert!(item.verified_reversible_high_risk);
    assert_eq!(item.one_click_actions, ["go", "no-go"]);
    assert!(!item.blocks_unrelated_loop_work);
    assert_eq!(
        item.queue_provenance_ref,
        "ledger.jsonl:m71-human-gate-queued"
    );
}

#[test]
fn decision_records_one_click_provenance_not_debugging_session() {
    let item = queue_self_improvement_human_go_no_go(&queue_input())
        .expect("verified high-risk tail queues");
    let record = record_self_improvement_human_go_no_go_decision(
        &item,
        SelfImprovementHumanGoNoGoDecision::Go,
        "journal.md:m71-human-go-click",
    )
    .expect("one-click decision records provenance");

    assert_eq!(
        record.schema_version,
        SELF_IMPROVEMENT_HUMAN_GO_NO_GO_DECISION_SCHEMA_VERSION
    );
    assert_eq!(record.decision, SelfImprovementHumanGoNoGoDecision::Go);
    assert_eq!(
        record.queue_status,
        SelfImprovementHumanGoNoGoQueueStatus::Accepted
    );
    assert!(record.one_click_only);
    assert!(record.no_debugging_session);
    assert!(!record.blocks_unrelated_loop_work);
}

#[test]
fn autonomous_loop_can_continue_unrelated_work_while_queue_waits() {
    let item = queue_self_improvement_human_go_no_go(&queue_input())
        .expect("verified high-risk tail queues");

    assert_eq!(
        item.status,
        SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision
    );
    assert!(!item.blocks_unrelated_loop_work);
    assert!(item
        .unrelated_loop_work_refs
        .iter()
        .any(|r| r.contains("openchrome")));
    assert!(item
        .unrelated_loop_work_refs
        .iter()
        .any(|r| r.contains("ledger.jsonl")));
}

#[test]
fn broadening_hook_records_threshold_without_bypassing_current_gate() {
    let item = queue_self_improvement_human_go_no_go(&queue_input())
        .expect("verified high-risk tail queues");

    assert!(item.broadening_hook.eligible_to_narrow_human_gate);
    assert!(item.broadening_hook.note.contains("M44 broadening hook"));
    assert_eq!(
        item.status,
        SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision
    );
}

#[test]
fn non_reversible_or_unverified_high_risk_fix_fails_closed() {
    let mut input = queue_input();
    input.apply_loop_input.routing_input.reversible = false;
    let error = queue_self_improvement_human_go_no_go(&input)
        .expect_err("non-reversible high-risk proposal is not queued");
    assert!(error.to_string().contains("verified, reversible high-risk"));

    let mut input = queue_input();
    input
        .apply_loop_input
        .routing_input
        .reverify_evidence
        .runtime_gate_passed = false;
    let error = queue_self_improvement_human_go_no_go(&input)
        .expect_err("failed gate high-risk proposal is not queued");
    assert!(error.to_string().contains("verified, reversible high-risk"));
}

#[test]
fn queue_rejects_debugging_session_or_new_data_plane_drift() {
    let mut input = queue_input();
    input.no_debugging_session = false;
    let error = queue_self_improvement_human_go_no_go(&input)
        .expect_err("debugging session drift rejected");
    assert!(error.to_string().contains("not a debugging session"));

    let mut input = queue_input();
    input.no_new_data_plane = false;
    let error =
        queue_self_improvement_human_go_no_go(&input).expect_err("new data plane drift rejected");
    assert!(error.to_string().contains("new data plane"));
}

#[test]
fn docs_record_thin_gate_without_new_store() {
    let doc = read_text("docs/self-improvement-loop-contract-v1.md").to_ascii_lowercase();
    for required in [
        "high-risk go/no-go queue",
        "one-click",
        "provenance",
        "not a debugging session",
        "continues unrelated autonomous work",
        "broadening hook",
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

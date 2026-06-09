//! Scenario Coverage v65 regression suite for #2046 / Era L M73.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use ouroforge_core::{
    queue_self_improvement_human_go_no_go, run_optional_human_channel_demo,
    run_self_improvement_reverify_apply_loop, trust_gradient_auto_apply::AutoApplyOutcome,
    OptionalHumanChannelDemoInput, SelfImprovementApplyLoopInput, SelfImprovementApplyLoopOutcome,
    SelfImprovementHumanGoNoGoQueueInput, SelfImprovementHumanGoNoGoQueueStatus,
    SelfImprovementReverifyEvidence, SelfImprovementRoute, SelfImprovementRoutingInput,
    SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel,
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
        attributed_milestone_id: "m73-v65-e2e".to_string(),
        routing_input: SelfImprovementRoutingInput {
            schema_version: SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION.to_string(),
            proposal_ref: proposal_ref.to_string(),
            source_apply_status: SourcePatchPreviewApplyStatus::Blocked,
            risk_level,
            source_affecting,
            reversible,
            trust_gradient_outcome,
            kill_switch_engaged: false,
            reverify_evidence: reverify_evidence(),
        },
        before_evidence_refs: refs(),
        after_evidence_refs: refs(),
        before_evidence_score: 7,
        after_evidence_score: 10,
        regression_detected: false,
        no_human_input: true,
        no_new_data_plane: true,
    }
}

fn optional_demo_input() -> OptionalHumanChannelDemoInput {
    OptionalHumanChannelDemoInput::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/optional-human-channel-v1/demo.fixture.json",
    ))
    .expect("optional human demo fixture validates")
}

#[test]
fn v65_matrix_records_end_to_end_rows_and_boundaries() {
    let matrix =
        read_json("examples/real-title-dogfood-v1/scenario-coverage-v65/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v65-autonomous-self-improvement-e2e-v1"
    );
    assert_eq!(matrix["coverageVersion"], 65);
    assert_eq!(matrix["titleId"], "era-i-engine-builder-deckbuilder");

    let ids: BTreeSet<_> = matrix["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| row["id"].as_str().expect("row id"))
        .collect();
    for required in [
        "m68-dogfood-run-detects-real-title-evidence",
        "m69-self-audit-attributes-bottleneck",
        "m70-diagnose-and-propose-source-apply-preview",
        "m71-reverify-low-risk-auto-apply-no-human",
        "m71-high-risk-never-auto-applies",
        "m72-optional-human-channel-non-blocking",
        "coverage-v65-boundaries",
    ] {
        assert!(ids.contains(required), "missing v65 row {required}");
    }

    for row in matrix["rows"].as_array().expect("rows") {
        assert_eq!(row["status"], "pass", "row did not pass: {row:#?}");
        let evidence_ref = row["evidenceRef"].as_str().expect("evidenceRef");
        assert!(
            repo_root().join(evidence_ref).is_file(),
            "missing evidence {evidence_ref}"
        );
        assert!(row["locks"].as_str().expect("locks").len() > 90);
    }

    let stage_chain: Vec<_> = matrix["stageChain"]
        .as_array()
        .expect("stageChain")
        .iter()
        .map(|stage| stage.as_str().expect("stage"))
        .collect();
    assert_eq!(
        stage_chain,
        [
            "dogfood-run",
            "self-audit",
            "diagnose",
            "propose",
            "re-verify",
            "auto-apply-low-risk",
            "queue-high-risk",
            "optional-human-non-blocking"
        ]
    );
}

#[test]
fn v65_existing_demo_fixture_chains_detect_to_apply_without_human_for_low_risk() {
    let demo =
        read_json("examples/real-title-dogfood-v1/self-improvement-loop-demo-v1/demo.fixture.json");
    let stages: Vec<_> = demo["stageChain"]
        .as_array()
        .expect("stageChain")
        .iter()
        .map(|stage| stage["stage"].as_str().expect("stage"))
        .collect();
    for required in [
        "detect",
        "explain",
        "trace",
        "attribute",
        "propose",
        "re-verify",
        "apply",
    ] {
        assert!(stages.contains(&required), "missing stage {required}");
    }
    assert_eq!(demo["lowRiskPath"]["outcome"], "auto-applied");
    assert_eq!(demo["lowRiskPath"]["humanInputRequired"], false);
    assert_eq!(
        demo["autonomyInvariants"]["noHumanInputOnAutonomousPath"],
        true
    );
    assert_eq!(demo["autonomyInvariants"]["noNewVerificationEngine"], true);
    assert_eq!(demo["autonomyInvariants"]["noNewDataPlane"], true);
}

#[test]
fn v65_low_risk_reverify_auto_applies_after_evidence_improves() {
    let report = run_self_improvement_reverify_apply_loop(&apply_input(
        "source-apply:patch-preview.v1#v65-low-risk-e2e",
        SourcePatchPreviewRiskLevel::Low,
        false,
        true,
        AutoApplyOutcome::AutoApplied,
    ))
    .expect("low-risk e2e path routes");

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
fn v65_high_risk_source_affecting_tail_never_auto_applies() {
    let report = run_self_improvement_reverify_apply_loop(&apply_input(
        "source-apply:patch-preview.v1#v65-high-risk-e2e",
        SourcePatchPreviewRiskLevel::High,
        true,
        true,
        AutoApplyOutcome::ManualFallback,
    ))
    .expect("high-risk e2e path routes");
    assert_eq!(
        report.outcome,
        SelfImprovementApplyLoopOutcome::HumanGoNoGoQueued
    );
    assert!(!report.source_mutation_applied);
    assert!(report.human_input_required);

    let queue = queue_self_improvement_human_go_no_go(&SelfImprovementHumanGoNoGoQueueInput {
        schema_version: SELF_IMPROVEMENT_HUMAN_GO_NO_GO_QUEUE_INPUT_SCHEMA_VERSION.to_string(),
        apply_loop_input: apply_input(
            "source-apply:patch-preview.v1#v65-high-risk-e2e",
            SourcePatchPreviewRiskLevel::High,
            true,
            true,
            AutoApplyOutcome::ManualFallback,
        ),
        queue_provenance_ref: "ledger.jsonl:v65-high-risk-queued".to_string(),
        unrelated_loop_work_refs: refs(),
        verification_strength_score: 95,
        broadening_threshold_score: 90,
        one_click_only: true,
        no_debugging_session: true,
        no_new_data_plane: true,
    })
    .expect("verified high-risk source tail queues");
    assert_eq!(
        queue.status,
        SelfImprovementHumanGoNoGoQueueStatus::AwaitingDecision
    );
    assert!(!queue.blocks_unrelated_loop_work);
}

#[test]
fn v65_optional_human_channel_does_not_block_end_to_end_loop() {
    let report = run_optional_human_channel_demo(&optional_demo_input())
        .expect("optional channel demo report");
    assert!(report.both_runs_completed);
    assert!(report.loop_never_waited_for_human);
    assert!(report.human_input_optional);
    assert!(!report.trusted_writes_from_surface);
    assert!(!report.taste_feedback_record.auto_applied);
}

#[test]
fn v65_docs_preserve_e2e_autonomy_and_boundary_claims() {
    let doc = read_text("docs/scenario-coverage-v65-autonomous-self-improvement-e2e.md")
        .to_ascii_lowercase();
    for required in [
        "coverage v65",
        "dogfood run",
        "self-audit",
        "diagnose",
        "source-apply patch",
        "re-verify",
        "low-risk reversible fixes auto-apply only after",
        "zero human input",
        "high-risk/source-affecting fixes are never auto-applied",
        "optional human channel",
        "never block",
        "no new verification engine",
        "no new data plane",
        "no new persistent store",
        "rust remains the data plane",
        "elixir executor remains unchanged",
        "#1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "doc missing {required}");
    }
}

//! Self-improvement loop contract tests for #2037 / Era L M71.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    route_self_improvement_fix, trust_gradient_auto_apply::AutoApplyOutcome,
    SelfImprovementLoopContract, SelfImprovementReverifyEvidence, SelfImprovementRoute,
    SelfImprovementRoutingInput, SourcePatchPreviewApplyStatus, SourcePatchPreviewRiskLevel,
    SELF_IMPROVEMENT_LOOP_CONTRACT_SCHEMA_VERSION, SELF_IMPROVEMENT_ROUTING_INPUT_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path))
        .unwrap_or_else(|err| panic!("read {path}: {err}"))
}

fn contract() -> SelfImprovementLoopContract {
    SelfImprovementLoopContract::from_json_str(&read_text(
        "examples/real-title-dogfood-v1/self-improvement-loop-v1/contract.fixture.json",
    ))
    .expect("contract fixture validates")
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
        proposal_ref: "source-apply:patch-preview.v1#m71-low-risk".to_string(),
        source_apply_status: SourcePatchPreviewApplyStatus::Blocked,
        risk_level: SourcePatchPreviewRiskLevel::Low,
        source_affecting: false,
        reversible: true,
        trust_gradient_outcome: AutoApplyOutcome::AutoApplied,
        kill_switch_engaged: false,
        reverify_evidence: reverify_evidence(),
    }
}

#[test]
fn contract_fixture_specifies_reverify_and_routing_without_new_engine() {
    let contract = contract();
    assert_eq!(
        contract.schema_version,
        SELF_IMPROVEMENT_LOOP_CONTRACT_SCHEMA_VERSION
    );
    assert_eq!(contract.title_id, "era-i-engine-builder-deckbuilder");
    assert!(contract
        .reverify_contract
        .openchrome_run_command
        .contains("dogfood-deckbuilder.yaml"));

    let refs = contract
        .required_pipeline_refs
        .join("\n")
        .to_ascii_lowercase();
    for required in [
        "openchrome",
        "verdict.json",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "rollback",
        "kill-switch",
    ] {
        assert!(refs.contains(required), "missing pipeline ref {required}");
    }

    let routes: Vec<_> = contract
        .routing_rules
        .iter()
        .map(|rule| rule.route)
        .collect();
    assert!(routes.contains(&SelfImprovementRoute::AutoApplyEligible));
    assert!(routes.contains(&SelfImprovementRoute::HumanGoNoGo));
    assert!(routes.contains(&SelfImprovementRoute::Blocked));
}

#[test]
fn low_risk_reversible_passed_reverify_routes_to_auto_apply_eligible() {
    let decision = route_self_improvement_fix(&routing_input()).expect("routing succeeds");
    assert_eq!(decision.route, SelfImprovementRoute::AutoApplyEligible);
    assert!(decision
        .allowed_actions
        .contains(&"route_to_existing_source_apply_transaction".to_string()));
    let forbidden = decision.forbidden_actions.join("\n");
    assert!(forbidden.contains("auto_merge"));
    assert!(forbidden.contains("create_new_verification_engine"));
    assert!(forbidden.contains("create_new_data_plane"));
}

#[test]
fn high_risk_or_source_affecting_tail_routes_to_human_go_no_go() {
    let mut input = routing_input();
    input.proposal_ref = "source-apply:patch-preview.v1#m71-rust-fix".to_string();
    input.risk_level = SourcePatchPreviewRiskLevel::High;
    input.source_affecting = true;
    input.trust_gradient_outcome = AutoApplyOutcome::ManualFallback;

    let decision = route_self_improvement_fix(&input).expect("routing succeeds");
    assert_eq!(decision.route, SelfImprovementRoute::HumanGoNoGo);
    assert!(decision
        .reasons
        .join("\n")
        .to_ascii_lowercase()
        .contains("human go/no-go"));
    assert!(decision
        .allowed_actions
        .contains(&"queue_thin_human_go_no_go".to_string()));
}

#[test]
fn failed_reverify_or_kill_switch_blocks_any_apply_route() {
    let mut failed_gate = routing_input();
    failed_gate.reverify_evidence.visual_gate_passed = false;
    let decision = route_self_improvement_fix(&failed_gate).expect("routing succeeds");
    assert_eq!(decision.route, SelfImprovementRoute::Blocked);

    let mut killed = routing_input();
    killed.kill_switch_engaged = true;
    let decision = route_self_improvement_fix(&killed).expect("routing succeeds");
    assert_eq!(decision.route, SelfImprovementRoute::Blocked);
    assert!(decision
        .reasons
        .join("\n")
        .to_ascii_lowercase()
        .contains("kill-switch"));
}

#[test]
fn hidden_human_or_parallel_data_plane_drift_fails_closed() {
    let mut input = routing_input();
    input.reverify_evidence.no_human_input = false;
    let error = route_self_improvement_fix(&input).expect_err("hidden human input rejected");
    assert!(error.to_string().contains("no human input"));

    let mut input = routing_input();
    input.reverify_evidence.no_new_data_plane = false;
    let error = route_self_improvement_fix(&input).expect_err("data plane drift rejected");
    assert!(error.to_string().contains("no new data plane"));

    let mut bad_contract = contract();
    bad_contract.boundary = bad_contract
        .boundary
        .replace("no new verification engine", "parallel verifier allowed");
    let error = bad_contract
        .validate()
        .expect_err("parallel verifier drift rejected");
    assert!(error.to_string().contains("no new verification engine"));
}

#[test]
fn docs_preserve_m71_boundaries_and_verification_block() {
    let doc = read_text("docs/self-improvement-loop-contract-v1.md");
    let lower = doc.to_ascii_lowercase();
    for required in [
        "self-improvement loop contract v1",
        "cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2",
        "scenario verdicts",
        "four gates plus design-integrity",
        "verdict.json",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage attribution",
        "source-apply",
        "trust-gradient",
        "rollback",
        "kill-switch",
        "auto-apply eligible",
        "human go/no-go",
        "does not execute openchrome",
        "create a verifier",
        "persistent store",
        "new data plane",
        "elixir\nexecutor remains the control plane",
        "fun/taste and release go/no-go\nremain human ring 2",
        "#1 and #23 remain open",
    ] {
        assert!(lower.contains(required), "doc missing {required}");
    }
}

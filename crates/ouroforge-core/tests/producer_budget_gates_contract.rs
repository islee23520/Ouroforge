use ouroforge_core::{
    producer_budget_gates::{
        evaluate_producer_budget_gates, ProducerBudgetGatePolicy, ProducerBudgetGateStatus,
        ProducerGateStatus, ProducerStopReason, PRODUCER_BUDGET_GATES_SCHEMA_VERSION,
    },
    producer_orchestration::{start_producer_orchestration, ProducerOrchestrationState},
    producer_plan::{derive_producer_plan, ProducerDesignIntent},
};
use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> ProducerBudgetGatePolicy {
    let path = repo_root()
        .join("examples/producer-budget-gates-v1")
        .join(name);
    let input = fs::read_to_string(path).expect(name);
    ProducerBudgetGatePolicy::from_json_str(&input).expect(name)
}

fn orchestration() -> ProducerOrchestrationState {
    let intent_text = fs::read_to_string(
        repo_root().join("examples/producer-plan-v1/design-intent.valid.fixture.json"),
    )
    .expect("producer plan fixture exists");
    let intent = ProducerDesignIntent::from_json_str(&intent_text).expect("valid intent parses");
    let plan = derive_producer_plan(&intent).expect("valid plan derives");
    start_producer_orchestration(
        "collect-and-exit-whole-game",
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
        plan,
    )
    .expect("orchestration starts")
}

#[test]
fn budget_exhaustion_halts_with_diagnosis() {
    let policy = fixture("budget-halt.fixture.json");
    let read =
        evaluate_producer_budget_gates(&orchestration(), &policy).expect("budget halt evaluates");

    assert_eq!(read.schema_version, PRODUCER_BUDGET_GATES_SCHEMA_VERSION);
    assert_eq!(read.status, ProducerBudgetGateStatus::HaltedBudgetExhausted);
    assert_eq!(read.stop_reason, Some(ProducerStopReason::BudgetExhausted));
    assert_eq!(read.condition_id.as_deref(), Some("stop-budget"));
    assert!(read.diagnosis.contains("budget exhausted"));
    assert!(read.diagnosis.contains("diagnosis evidence"));
}

#[test]
fn human_gate_block_fails_closed_until_mandatory_approval_exists() {
    let policy = fixture("human-gate.fixture.json");
    let read =
        evaluate_producer_budget_gates(&orchestration(), &policy).expect("human gate evaluates");

    assert_eq!(read.status, ProducerBudgetGateStatus::BlockedHumanGate);
    assert_eq!(
        read.stop_reason,
        Some(ProducerStopReason::HumanApprovalRequired)
    );
    assert_eq!(read.condition_id.as_deref(), Some("stop-human"));
    assert_eq!(read.pending_human_gate_ids, vec!["gate-vision"]);
    assert!(read.diagnosis.contains("human approval required"));
}

#[test]
fn no_progress_stop_uses_reused_window_with_diagnosis() {
    let policy = fixture("no-progress.fixture.json");
    let read =
        evaluate_producer_budget_gates(&orchestration(), &policy).expect("no progress evaluates");

    assert_eq!(read.status, ProducerBudgetGateStatus::StoppedNoProgress);
    assert_eq!(read.stop_reason, Some(ProducerStopReason::NoProgress));
    assert_eq!(read.condition_id.as_deref(), Some("stop-no-progress"));
    assert_eq!(read.no_progress_steps, read.no_progress_window);
    assert!(read.diagnosis.contains("no progress"));
}

#[test]
fn approved_gates_inside_budget_can_continue_read_only() {
    let mut policy = fixture("no-progress.fixture.json");
    policy.policy_id = "collect-and-exit-continue".to_string();
    policy.usage.iteration_count = 1;
    policy.usage.cost_units = 25;
    policy.usage.no_progress_steps = 0;
    for gate in &mut policy.human_approval_gates {
        gate.status = ProducerGateStatus::Approved;
    }

    let read =
        evaluate_producer_budget_gates(&orchestration(), &policy).expect("continue evaluates");
    assert_eq!(read.status, ProducerBudgetGateStatus::Continue);
    assert_eq!(read.stop_reason, None);
    assert!(read.pending_human_gate_ids.is_empty());
    assert!(read.diagnosis.contains("within budget"));
}

#[test]
fn missing_mandatory_human_gate_fixture_fails_closed() {
    let mut policy = fixture("human-gate.fixture.json");
    policy
        .human_approval_gates
        .retain(|gate| gate.gate_kind != "legal");
    let error = policy
        .validate()
        .expect_err("missing mandatory legal gate must fail");
    assert!(error
        .to_string()
        .contains("missing mandatory human approval gate `legal`"));
}

#[test]
fn docs_fixtures_and_boundaries_preserve_governance_and_conservative_wording() {
    let docs = fs::read_to_string(repo_root().join("docs/producer-budget-gates-v1.md"))
        .expect("docs exist");
    assert!(docs.contains("Issue: #1685"));
    assert!(docs.contains("evolve-campaign/fuzz budget and stop-condition shape"));
    assert!(docs.contains("human approval gates"));
    assert!(docs.contains("local evidence own validation"));
    assert!(docs.contains("Issues #1 and #23 remain open"));

    for name in [
        "budget-halt.fixture.json",
        "human-gate.fixture.json",
        "no-progress.fixture.json",
    ] {
        let policy = fixture(name);
        assert_eq!(policy.schema_version, PRODUCER_BUDGET_GATES_SCHEMA_VERSION);
        assert!(policy.generated_state_policy.contains("untracked"));
        assert!(policy.generated_state_policy.contains("fixture-scoped"));
    }

    let combined =
        format!("{}\n{}", docs, fixture("budget-halt.fixture.json").boundary).to_ascii_lowercase();
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "production-ready engine",
        "godot replacement enabled",
        "godot parity enabled",
        "quality/fun guaranteed",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}

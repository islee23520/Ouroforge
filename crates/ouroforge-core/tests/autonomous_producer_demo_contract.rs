use ouroforge_core::{
    producer_budget_gates::{
        evaluate_producer_budget_gates, ProducerBudgetGatePolicy, ProducerBudgetGateStatus,
        ProducerStopReason,
    },
    producer_orchestration::{complete_current_dispatch, start_producer_orchestration},
    producer_plan::{derive_producer_plan, ProducerDesignIntent},
};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn demo_value(name: &str) -> Value {
    let path = repo_root()
        .join("examples/autonomous-producer-v1/demo")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).expect(name)).expect(name)
}

fn demo_policy(name: &str) -> ProducerBudgetGatePolicy {
    let path = repo_root()
        .join("examples/autonomous-producer-v1/demo")
        .join(name);
    ProducerBudgetGatePolicy::from_json_str(&fs::read_to_string(path).expect(name)).expect(name)
}

fn derived_orchestration() -> ouroforge_core::producer_orchestration::ProducerOrchestrationState {
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
fn deterministic_demo_replays_intent_to_release_candidate_audit_trail() {
    let demo = demo_value("demo.fixture.json");
    assert_eq!(demo["schemaVersion"], "autonomous-producer-demo-v1");
    assert_eq!(
        demo["intentRef"],
        "examples/producer-plan-v1/design-intent.valid.fixture.json"
    );

    let stages = demo["stages"].as_array().expect("stages");
    let audit = demo["auditTrail"].as_array().expect("audit trail");
    assert_eq!(stages.len(), 6);
    assert_eq!(audit.len(), stages.len());
    for (stage, entry) in stages.iter().zip(audit) {
        assert_eq!(entry["stage"], *stage);
        let evidence = entry["evidenceRef"].as_str().expect("evidence ref");
        assert!(evidence.starts_with("examples/autonomous-producer-v1/demo/evidence/"));
        assert!(!evidence.contains(".."));
        assert!(!entry["summary"].as_str().unwrap().trim().is_empty());
    }

    let mut orchestration = derived_orchestration();
    let mut completed = 0;
    while orchestration.current_dispatch.is_some() {
        let evidence =
            format!("examples/autonomous-producer-v1/demo/evidence/dispatch-{completed}.json");
        orchestration = complete_current_dispatch(&orchestration, &evidence)
            .expect("demo dispatch completes deterministically");
        completed += 1;
    }
    let read = orchestration.read_model();
    assert_eq!(read.status, "complete");
    assert_eq!(read.completed_task_count, read.task_count);
    assert!(read
        .validation_summary
        .iter()
        .any(|line| line.contains("deterministic order")));
}

#[test]
fn release_candidate_halts_at_human_release_gate() {
    let orchestration = derived_orchestration();
    let policy = demo_policy("release-gate.policy.json");
    let read = evaluate_producer_budget_gates(&orchestration, &policy)
        .expect("release gate policy evaluates");

    assert_eq!(read.status, ProducerBudgetGateStatus::BlockedHumanGate);
    assert_eq!(
        read.stop_reason,
        Some(ProducerStopReason::HumanApprovalRequired)
    );
    assert_eq!(read.condition_id.as_deref(), Some("stop-human"));
    assert_eq!(read.pending_human_gate_ids, vec!["gate-release"]);
    assert!(read.diagnosis.contains("human approval required"));

    let demo = demo_value("demo.fixture.json");
    assert_eq!(
        demo["releaseCandidate"]["status"],
        "blocked-at-human-release-gate"
    );
    assert_eq!(demo["releaseCandidate"]["humanGateId"], "gate-release");
}

#[test]
fn budget_halt_case_is_safe_and_diagnostic() {
    let orchestration = derived_orchestration();
    let policy = demo_policy("budget-halt.policy.json");
    let read = evaluate_producer_budget_gates(&orchestration, &policy)
        .expect("budget halt policy evaluates");

    assert_eq!(read.status, ProducerBudgetGateStatus::HaltedBudgetExhausted);
    assert_eq!(read.stop_reason, Some(ProducerStopReason::BudgetExhausted));
    assert_eq!(read.condition_id.as_deref(), Some("stop-budget"));
    assert_eq!(read.iteration_count, read.max_iterations);
    assert_eq!(read.cost_units, read.max_cost_units);
    assert!(read.diagnosis.contains("budget exhausted"));
}

#[test]
fn docs_and_fixtures_preserve_generated_state_and_conservative_boundaries() {
    let docs = fs::read_to_string(repo_root().join("docs/autonomous-producer-v1-demo.md"))
        .expect("docs exist");
    assert!(docs.contains("Issue: #1686"));
    assert!(docs.contains("no network or live browser"));
    assert!(docs.contains("Issues #1 and #23 remain open"));

    let demo = demo_value("demo.fixture.json");
    let combined = format!(
        "{}\n{}\n{}",
        docs,
        demo["generatedStatePolicy"].as_str().unwrap(),
        demo["boundary"].as_str().unwrap()
    )
    .to_ascii_lowercase();
    for required in [
        "untracked unless explicitly fixture-scoped",
        "browser/studio read-only",
        "proposal-only",
        "human gates",
        "#1 and #23 remain open",
    ] {
        assert!(combined.contains(required), "missing boundary: {required}");
    }
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

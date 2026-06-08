//! Scenario Coverage v40 — Autonomous Producer Regression Suite (#1687).
//!
//! Locks Autonomous Producer and Whole-Game Orchestration v1 behavior: producer
//! plan decomposition (#1683), orchestration progression/resume (#1684),
//! budget/stop/human-gate handling (#1685), autonomous producer demo release
//! gating (#1686), plus the backward-compatibility guarantee that the existing
//! single-artifact evolve campaign remains valid. State/shape assertions only —
//! no flaky or timing-based checks — so a breaking change fails CI.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::evolve_campaign::{read_evolve_campaign_artifact, validate_evolve_campaign};
use ouroforge_core::producer_budget_gates::{
    evaluate_producer_budget_gates, ProducerBudgetGatePolicy,
};
use ouroforge_core::producer_orchestration::{
    complete_current_dispatch, resume_producer_orchestration, start_producer_orchestration,
};
use ouroforge_core::producer_plan::{derive_producer_plan, ProducerDesignIntent};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn read_text(relative: &str) -> String {
    let path = repo_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("parse {relative}: {error}"))
}

fn v40(name: &str) -> String {
    read_text(&format!(
        "examples/autonomous-producer-v1/scenario-coverage-v40/{name}"
    ))
}

fn v40_json(name: &str) -> Value {
    serde_json::from_str(&v40(name)).unwrap_or_else(|error| panic!("parse {name}: {error}"))
}

fn valid_intent() -> ProducerDesignIntent {
    ProducerDesignIntent::from_json_str(&read_text(
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
    ))
    .expect("valid producer intent parses")
}

fn valid_plan() -> ouroforge_core::producer_plan::ProducerPlanArtifact {
    derive_producer_plan(&valid_intent()).expect("valid producer plan derives")
}

fn orchestration() -> ouroforge_core::producer_orchestration::ProducerOrchestrationState {
    start_producer_orchestration(
        "collect-and-exit-whole-game",
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
        valid_plan(),
    )
    .expect("producer orchestration starts")
}

const SYSTEMS: [&str; 5] = ["plan", "orchestration", "budget", "backcompat", "doc"];

#[test]
fn v40_matrix_enumerates_autonomous_producer_regressions() {
    let matrix = v40_json("matrix.fixture.json");
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v40");
    assert_eq!(matrix["issue"], 1687);
    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "rust/local owns validation",
        "browser/studio read-only",
        "state/shape assertions only",
        "proposal-only",
        "no network/live browser",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "missing boundary: {required}");
    }

    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 12,
        "v40 enumerates plan/orchestration/budget/backcompat coverage"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(scenario["system"].as_str().expect("system").to_string());
        assert!(scenario["kind"].is_string(), "{id} has kind");
        assert!(scenario["fixtureRef"].is_string(), "{id} has fixture ref");
        assert!(scenario["expect"].is_string(), "{id} has expectation");
    }
    for system in SYSTEMS {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("v40-backcompat-single-artifact-evolve"));
    assert!(ids.contains("v40-demo-release-gate"));
}

#[test]
fn v40_plan_decomposition_and_fail_closed_shape_are_locked() {
    let spec = v40_json("plan-regression.fixture.json");
    assert_eq!(
        spec["schemaVersion"],
        "scenario-coverage-v40-plan-regression"
    );

    let intent =
        ProducerDesignIntent::from_json_str(&read_text(spec["intentRef"].as_str().unwrap()))
            .expect("valid intent parses");
    let plan = derive_producer_plan(&intent).expect("plan derives");
    assert_eq!(plan.plan_id, spec["expectedPlanId"]);
    let expected_task_order = spec["expectedTaskOrder"]
        .as_array()
        .expect("expected task order")
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        plan.tasks
            .iter()
            .map(|task| task.function_agent.clone())
            .collect::<Vec<_>>(),
        expected_task_order
    );
    assert!(plan.tasks.iter().all(|task| task.proposal_only));
    assert_eq!(plan.tasks.last().unwrap().function_agent, "review");
    plan.validate().expect("derived plan validates");

    let read = plan.read_model();
    for (role, count) in spec["expectedRoles"].as_object().expect("roles") {
        assert_eq!(read.role_counts[role], count.as_u64().unwrap() as usize);
    }
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("browser/Studio surfaces remain read-only")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("not a new generator")));

    let mut subset = intent;
    subset.requested_functions = spec["subsetRequestedFunctions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();
    let subset_plan = derive_producer_plan(&subset).expect("subset derives");
    assert_eq!(
        subset_plan.tasks.last().unwrap().function_agent,
        spec["expectedSubsetTerminalAgent"].as_str().unwrap()
    );

    let error =
        ProducerDesignIntent::from_json_str(&read_text(spec["invalidIntentRef"].as_str().unwrap()))
            .expect_err("malformed intent fails closed");
    assert!(error.to_string().contains("unsupported function"));
}

#[test]
fn v40_orchestration_progression_resume_and_terminal_shape_are_locked() {
    let spec = v40_json("orchestration-regression.fixture.json");
    assert_eq!(
        spec["schemaVersion"],
        "scenario-coverage-v40-orchestration-regression"
    );

    let mut state = start_producer_orchestration(
        spec["orchestrationId"].as_str().unwrap(),
        spec["planRef"].as_str().unwrap(),
        valid_plan(),
    )
    .expect("orchestration starts");
    assert_eq!(state.read_model().status, spec["expectedInitialStatus"]);
    assert_eq!(state.completed_task_ids.len(), 0);
    assert!(state.current_dispatch.as_ref().unwrap().proposal_only);

    for evidence in spec["completionEvidenceRefs"].as_array().unwrap() {
        state = complete_current_dispatch(&state, evidence.as_str().unwrap())
            .expect("dispatch completes");
        state.validate().expect("intermediate state validates");
    }
    assert_eq!(
        state.completed_task_ids.len(),
        spec["expectedCompletedAfterThree"].as_u64().unwrap() as usize
    );
    assert_eq!(
        state.current_dispatch.as_ref().unwrap().function_agent,
        spec["expectedCurrentAfterThree"].as_str().unwrap()
    );

    let serialized = serde_json::to_string_pretty(&state).expect("state serializes");
    let resumed = resume_producer_orchestration(&serialized).expect("state resumes");
    assert_eq!(resumed, state);
    assert_eq!(
        resumed.resume_token,
        "collect-and-exit-whole-game:completed-3"
    );

    let mut terminal = resumed;
    let mut step = terminal.completed_task_ids.len();
    while terminal.current_dispatch.is_some() {
        terminal = complete_current_dispatch(
            &terminal,
            &format!("examples/autonomous-producer-v1/demo/evidence/v40-dispatch-{step}.json"),
        )
        .expect("terminal dispatch completes");
        step += 1;
    }
    let read = terminal.read_model();
    assert_eq!(read.status, spec["expectedTerminalStatus"]);
    assert_eq!(read.completed_task_count, read.task_count);
    assert!(read
        .validation_summary
        .iter()
        .any(|line| line.contains("deterministic order")));
}

#[test]
fn v40_budget_stop_and_human_gate_states_are_locked() {
    let spec = v40_json("budget-regression.fixture.json");
    assert_eq!(
        spec["schemaVersion"],
        "scenario-coverage-v40-budget-regression"
    );
    assert_eq!(spec["orchestrationRef"], "collect-and-exit-whole-game");

    for policy_spec in spec["policies"].as_array().expect("policies") {
        let id = policy_spec["id"].as_str().expect("policy id");
        let policy = ProducerBudgetGatePolicy::from_json_str(&read_text(
            policy_spec["ref"].as_str().unwrap(),
        ))
        .unwrap_or_else(|error| panic!("{id} policy parses: {error:#}"));
        let read = evaluate_producer_budget_gates(&orchestration(), &policy)
            .unwrap_or_else(|error| panic!("{id} evaluates: {error:#}"));
        assert_eq!(
            serde_json::to_value(read.status).unwrap(),
            policy_spec["expectedStatus"],
            "{id} status"
        );
        assert_eq!(
            read.stop_reason
                .as_ref()
                .map(|reason| serde_json::to_value(reason).unwrap()),
            Some(policy_spec["expectedStopReason"].clone()),
            "{id} stop reason"
        );
        assert_eq!(
            read.condition_id.as_deref(),
            policy_spec["expectedConditionId"].as_str(),
            "{id} condition"
        );
        if let Some(expected) = policy_spec
            .get("expectedPendingGates")
            .and_then(|v| v.as_array())
        {
            let expected = expected
                .iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect::<Vec<_>>();
            assert_eq!(read.pending_human_gate_ids, expected, "{id} pending gates");
        }
        assert!(
            read.diagnosis.contains("budget")
                || read.diagnosis.contains("human approval")
                || read.diagnosis.contains("no progress"),
            "{id} has actionable diagnosis"
        );
    }
}

#[test]
fn v40_single_artifact_evolve_campaign_backcompat_remains_valid() {
    let golden = v40_json("backcompat.single-artifact.golden.json");
    assert_eq!(golden["schemaVersion"], "scenario-coverage-v40-backcompat");
    let references = golden["references"].as_array().expect("references");
    assert_eq!(references.len(), 1, "v40 pins the single-artifact golden");
    let reference = &references[0];
    assert_eq!(reference["id"], "backcompat-single-artifact-evolve");
    assert_eq!(reference["contract"], "evolve-campaign-v1");
    assert_eq!(reference["expectValid"], true);

    let artifact =
        read_evolve_campaign_artifact(repo_root().join(reference["ref"].as_str().unwrap()))
            .expect("single-artifact evolve campaign reads");
    validate_evolve_campaign(&artifact)
        .unwrap_or_else(|error| panic!("single-artifact evolve campaign regressed: {error:#}"));
}

#[test]
fn v40_docs_fixtures_and_public_wording_preserve_governance() {
    let doc = read_text("docs/scenario-coverage-v40.md");
    assert!(doc.contains("Scenario Coverage v40"));
    assert!(doc.contains("#1687"));
    assert!(doc.contains("state/shape"));
    assert!(doc.contains("backward-compatibility"));
    assert!(doc.contains("Rust/local owned"));
    assert!(doc.contains("no network or live browser"));
    assert!(doc.contains("Issues #1 and #23 remain open"));

    let combined = format!(
        "{}\n{}\n{}",
        doc,
        v40("matrix.fixture.json"),
        read_json("examples/autonomous-producer-v1/demo/demo.fixture.json")
    )
    .to_ascii_lowercase();
    for required in [
        "browser/studio read-only",
        "proposal-only",
        "untracked unless fixture-scoped",
        "no network/live browser",
        "#1 and #23 remain open",
    ] {
        assert!(combined.contains(required), "missing wording: {required}");
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

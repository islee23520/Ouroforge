use ouroforge_core::{
    producer_orchestration::{
        complete_current_dispatch, resume_producer_orchestration, start_producer_orchestration,
        PRODUCER_ORCHESTRATION_SCHEMA_VERSION,
    },
    producer_plan::{derive_producer_plan, ProducerDesignIntent},
};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> Value {
    let path = repo_root()
        .join("examples/producer-orchestration-v1")
        .join(name);
    serde_json::from_str(&fs::read_to_string(path).expect(name)).expect(name)
}

fn valid_plan() -> ouroforge_core::producer_plan::ProducerPlanArtifact {
    let intent_text = fs::read_to_string(
        repo_root().join("examples/producer-plan-v1/design-intent.valid.fixture.json"),
    )
    .expect("#1683 producer plan fixture exists");
    let intent = ProducerDesignIntent::from_json_str(&intent_text).expect("valid intent parses");
    derive_producer_plan(&intent).expect("valid plan derives")
}

#[test]
fn plan_progression_drives_role_agents_in_plan_order() {
    let spec = fixture("plan-progression.fixture.json");
    let state = start_producer_orchestration(
        spec["orchestrationId"].as_str().unwrap(),
        spec["planRef"].as_str().unwrap(),
        valid_plan(),
    )
    .expect("orchestration starts");

    assert_eq!(state.schema_version, PRODUCER_ORCHESTRATION_SCHEMA_VERSION);
    assert_eq!(state.read_model().status, spec["expectedInitialStatus"]);
    assert_eq!(state.completed_task_ids.len(), 0);
    assert_eq!(state.dispatches.len(), 1);

    let advanced =
        complete_current_dispatch(&state, spec["completionEvidenceRef"].as_str().unwrap())
            .expect("first dispatch completes");
    assert_eq!(
        advanced.completed_task_ids.len(),
        spec["expectedCompletedAfterOne"].as_u64().unwrap() as usize
    );
    assert_eq!(
        advanced.current_dispatch.as_ref().unwrap().function_agent,
        "requirements"
    );
    advanced.validate().expect("advanced state validates");
}

#[test]
fn role_dispatch_is_proposal_only_and_matches_current_plan_task() {
    let spec = fixture("role-dispatch.fixture.json");
    let state = start_producer_orchestration(
        "collect-and-exit-whole-game",
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
        valid_plan(),
    )
    .expect("orchestration starts");
    let dispatch = state.current_dispatch.as_ref().unwrap();

    assert_eq!(dispatch.task_id, spec["expectedFirstTaskId"]);
    assert_eq!(dispatch.function_agent, spec["expectedFunctionAgent"]);
    assert_eq!(dispatch.role, spec["expectedRole"]);
    assert_eq!(dispatch.proposal_only, spec["expectedProposalOnly"]);
    assert_eq!(
        serde_json::to_value(&dispatch.status).unwrap(),
        spec["expectedStatus"]
    );
    assert_eq!(dispatch.inputs, state.plan.tasks[0].inputs);
    assert_eq!(dispatch.outputs, state.plan.tasks[0].outputs);
}

#[test]
fn resumable_long_horizon_state_round_trips_after_multiple_steps() {
    let spec = fixture("resume-state.fixture.json");
    let mut state = start_producer_orchestration(
        "collect-and-exit-whole-game",
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
        valid_plan(),
    )
    .expect("orchestration starts");
    for evidence in spec["completionEvidenceRefs"].as_array().unwrap() {
        state = complete_current_dispatch(&state, evidence.as_str().unwrap())
            .expect("dispatch completes");
    }

    let json = serde_json::to_string_pretty(&state).expect("state serializes");
    let resumed = resume_producer_orchestration(&json).expect("state resumes");
    assert_eq!(resumed, state);
    assert_eq!(
        resumed.completed_task_ids.len(),
        spec["expectedCompletedTaskCount"].as_u64().unwrap() as usize
    );
    assert_eq!(resumed.resume_token, spec["expectedResumeToken"]);
    assert_eq!(
        resumed.current_dispatch.as_ref().unwrap().function_agent,
        spec["expectedCurrentFunctionAgent"].as_str().unwrap()
    );
}

#[test]
fn orchestration_fails_closed_on_dispatch_drift_and_completed_prefix_drift() {
    let _drift_fixture = fixture("invalid/dispatch-drift.fixture.json");
    let state = start_producer_orchestration(
        "collect-and-exit-whole-game",
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
        valid_plan(),
    )
    .expect("orchestration starts");

    let mut direct_write = state.clone();
    direct_write
        .current_dispatch
        .as_mut()
        .unwrap()
        .proposal_only = false;
    *direct_write.dispatches.last_mut().unwrap() = direct_write.current_dispatch.clone().unwrap();
    let error = direct_write
        .validate()
        .expect_err("direct write dispatch must fail");
    assert!(error.to_string().contains("proposalOnly must be true"));

    let mut prefix_drift = state;
    prefix_drift.completed_task_ids = vec!["not-the-first-task".to_string()];
    let error = prefix_drift.validate().expect_err("prefix drift must fail");
    assert!(error.to_string().contains("prefix of the plan task order"));
}

#[test]
fn read_model_docs_and_fixtures_keep_governance_and_compatibility_boundaries() {
    let state = start_producer_orchestration(
        "collect-and-exit-whole-game",
        "examples/producer-plan-v1/design-intent.valid.fixture.json",
        valid_plan(),
    )
    .expect("orchestration starts");
    let read = state.read_model();
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| { note.contains("Milestone 23") && note.contains("Milestone 42") }));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("browser/Studio surfaces remain read-only")));

    let docs = fs::read_to_string(repo_root().join("docs/producer-orchestration-v1.md"))
        .expect("docs exist");
    assert!(docs.contains("Issue: #1684"));
    assert!(docs.contains("Milestone 23 campaign state"));
    assert!(docs.contains("Milestone 42 production pipeline"));
    assert!(docs.contains("Generated orchestration state, runs, assets, content, coverage"));
    assert!(docs.contains("Issues #1 and #23 remain open"));

    let combined = format!("{}\n{}", docs, state.boundary).to_ascii_lowercase();
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

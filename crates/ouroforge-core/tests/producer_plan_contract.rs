use ouroforge_core::producer_plan::{
    derive_producer_plan, ProducerDesignIntent, ProducerPlanArtifact, PRODUCER_PLAN_SCHEMA_VERSION,
};
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/producer-plan-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_design_intent_decomposes_into_expected_production_plan() {
    let intent =
        ProducerDesignIntent::from_json_str(&read_fixture("design-intent.valid.fixture.json"))
            .expect("valid design intent parses");
    let plan = derive_producer_plan(&intent).expect("valid intent derives plan");

    assert_eq!(plan.schema_version, PRODUCER_PLAN_SCHEMA_VERSION);
    assert_eq!(
        plan.plan_id,
        "collect-and-exit-producer-intent-production-plan"
    );
    assert_eq!(plan.intent_id, intent.intent_id);
    assert_eq!(
        plan.gdd_ref,
        "examples/gdd-design-brief-v1/design-brief.valid.fixture.json"
    );
    assert_eq!(plan.tasks.len(), 10);
    assert_eq!(plan.tasks.first().unwrap().function_agent, "design-brief");
    assert_eq!(plan.tasks.last().unwrap().function_agent, "review");
    assert!(plan.tasks.iter().all(|task| task.proposal_only));
    assert_eq!(
        plan.tasks
            .iter()
            .map(|task| task.function_agent.as_str())
            .collect::<Vec<_>>(),
        vec![
            "design-brief",
            "requirements",
            "mechanics",
            "scaffold",
            "scene-level",
            "behavior",
            "assets",
            "scenarios",
            "qa",
            "review"
        ]
    );
    plan.validate().expect("derived plan validates");
}

#[test]
fn malformed_intent_is_rejected_fail_closed() {
    let error = ProducerDesignIntent::from_json_str(&read_fixture(
        "invalid/design-intent.malformed.fixture.json",
    ))
    .expect_err("unsupported function must reject intent");
    assert!(
        error.to_string().contains("unsupported function"),
        "unexpected error: {error:#}"
    );
}

#[test]
fn decomposition_is_deterministic_and_order_independent_for_requested_functions() {
    let mut first =
        ProducerDesignIntent::from_json_str(&read_fixture("design-intent.valid.fixture.json"))
            .expect("valid design intent parses");
    let mut second = first.clone();
    first.requested_functions.reverse();
    second.requested_functions.rotate_left(3);

    let first_plan = derive_producer_plan(&first).expect("first derives");
    let second_plan = derive_producer_plan(&second).expect("second derives");
    assert_eq!(first_plan, second_plan);
    assert_eq!(
        first_plan.read_model_json().expect("read model json"),
        second_plan.read_model_json().expect("read model json")
    );
}

#[test]
fn plan_validation_rejects_trusted_write_and_dependency_drift() {
    let intent =
        ProducerDesignIntent::from_json_str(&read_fixture("design-intent.valid.fixture.json"))
            .expect("valid design intent parses");
    let plan = derive_producer_plan(&intent).expect("valid intent derives plan");
    let plan_json = serde_json::to_string_pretty(&plan).expect("serialize plan fixture-like json");
    let mut plan = ProducerPlanArtifact::from_json_str(&plan_json).expect("plan reparses");

    plan.tasks[0].proposal_only = false;
    let error = plan.validate().expect_err("trusted write must fail");
    assert!(error.to_string().contains("proposalOnly must be true"));

    let mut plan = ProducerPlanArtifact::from_json_str(&plan_json).expect("plan reparses");
    plan.tasks[1].depends_on = vec!["missing-upstream".to_string()];
    let error = plan.validate().expect_err("missing dependency must fail");
    assert!(error
        .to_string()
        .contains("missing or out-of-order dependency"));
}

#[test]
fn read_model_preserves_browser_read_only_and_review_boundaries() {
    let intent =
        ProducerDesignIntent::from_json_str(&read_fixture("design-intent.valid.fixture.json"))
            .expect("valid design intent parses");
    let plan = derive_producer_plan(&intent).expect("valid intent derives plan");
    let read = plan.read_model();
    assert_eq!(read.schema_version, PRODUCER_PLAN_SCHEMA_VERSION);
    assert_eq!(read.task_count, 10);
    assert_eq!(read.role_counts["designer"], 2);
    assert!(read
        .validation_summary
        .iter()
        .any(|note| note.contains("deterministic production plan")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("browser/Studio surfaces remain read-only")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("not a new generator")));
}

#[test]
fn docs_and_fixtures_keep_generated_state_wording_and_governance_conservative() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/producer-plan-v1.md"),
    )
    .expect("docs exist");
    assert!(docs.contains("Issue: #1683"));
    assert!(docs
        .contains("Generated runs, assets, content, coverage, and other artifacts stay untracked"));
    assert!(docs.contains("Issues #1 and #23 remain open"));
    assert!(docs.contains("Browser"));
    assert!(docs.contains("read model only"));

    let combined = format!(
        "{}\n{}",
        docs,
        read_fixture("design-intent.valid.fixture.json")
    )
    .to_ascii_lowercase();
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

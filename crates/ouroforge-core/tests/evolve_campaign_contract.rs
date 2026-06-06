use ouroforge_core::{
    evolve_campaign_cli_summary, iteration_reaches_acceptance, validate_evolve_campaign,
    EvolveCampaignArtifact, EvolveCampaignStopReason, EvolveGateStatus, EvolveGateVerdict,
    EVOLVE_CAMPAIGN_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> EvolveCampaignArtifact {
    let path = workspace_path(&format!("examples/evolve-campaign-v1/contract/{name}"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} exists: {err}", path.display()));
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn accepts_each_termination_mode() {
    let cases = [
        (
            "acceptance-reached.fixture.json",
            EvolveCampaignStopReason::AcceptanceReached,
        ),
        (
            "budget-exhausted.fixture.json",
            EvolveCampaignStopReason::BudgetExhausted,
        ),
        (
            "no-progress.fixture.json",
            EvolveCampaignStopReason::NoProgress,
        ),
    ];
    for (name, reason) in cases {
        let read_model = validate_evolve_campaign(&fixture(name)).expect(name);
        assert_eq!(read_model.schema_version, EVOLVE_CAMPAIGN_SCHEMA_VERSION);
        assert_eq!(read_model.stop_reason, reason, "{name}");
        assert!(read_model.boundary.contains("descriptive"), "{name}");
        assert!(read_model.iteration_count >= 1, "{name}");
        // cli summary renders for every termination mode.
        let summary = evolve_campaign_cli_summary(&fixture(name)).expect(name);
        assert!(summary.contains("descriptive-read-only"), "{name}");
    }
}

#[test]
fn acceptance_records_passing_iteration_and_no_diagnosis() {
    let read_model =
        validate_evolve_campaign(&fixture("acceptance-reached.fixture.json")).expect("valid");
    assert_eq!(read_model.accepted_iteration, Some(2));
    assert_eq!(read_model.diagnosis, None);
}

#[test]
fn non_converged_terminations_carry_a_diagnosis() {
    for name in ["budget-exhausted.fixture.json", "no-progress.fixture.json"] {
        let read_model = validate_evolve_campaign(&fixture(name)).expect(name);
        assert_eq!(read_model.accepted_iteration, None, "{name}");
        assert!(
            read_model.diagnosis.is_some_and(|text| !text.is_empty()),
            "{name} carries a diagnosis"
        );
    }
}

#[test]
fn iteration_reaches_acceptance_requires_every_targeted_gate() {
    let artifact = fixture("acceptance-reached.fixture.json");
    let target = artifact.acceptance_target.clone();
    assert!(!iteration_reaches_acceptance(
        &artifact.iterations[0],
        &target
    ));
    assert!(!iteration_reaches_acceptance(
        &artifact.iterations[1],
        &target
    ));
    assert!(iteration_reaches_acceptance(
        &artifact.iterations[2],
        &target
    ));
}

#[test]
fn rejects_missing_stop_conditions() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.stop_conditions.clear();
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("stop condition"), "{err}");
}

#[test]
fn rejects_campaign_without_acceptance_stop_condition() {
    let mut artifact = fixture("budget-exhausted.fixture.json");
    artifact
        .stop_conditions
        .retain(|condition| condition.reason != EvolveCampaignStopReason::AcceptanceReached);
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("acceptance-reached"), "{err}");
}

#[test]
fn rejects_malformed_budget() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.budget.max_iterations = 0;
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("maxIterations"), "{err}");

    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.budget.max_cost_units = 0;
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("maxCostUnits"), "{err}");
}

#[test]
fn rejects_zero_iteration_campaign() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.iterations.clear();
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("at least one iteration"), "{err}");
}

#[test]
fn rejects_stale_or_unsafe_refs() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.iterations[0].mutation_ref = "../escape/mutation.json".to_string();
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("safe repo-relative"), "{err}");
}

#[test]
fn rejects_iteration_missing_a_gate() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.iterations[2].four_gate = vec![EvolveGateVerdict {
        gate: "mechanical".to_string(),
        status: EvolveGateStatus::Pass,
    }];
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("all four gates"), "{err}");
}

#[test]
fn rejects_acceptance_without_a_passing_final_iteration() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    // Break the final iteration's semantic gate so acceptance is not actually met.
    if let Some(verdict) = artifact.iterations[2]
        .four_gate
        .iter_mut()
        .find(|verdict| verdict.gate == "semantic")
    {
        verdict.status = EvolveGateStatus::Fail;
    }
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("acceptance gate"), "{err}");
}

#[test]
fn rejects_termination_referencing_undeclared_condition() {
    let mut artifact = fixture("acceptance-reached.fixture.json");
    artifact.termination.condition_id = "stop-unknown".to_string();
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("declared stop condition"), "{err}");
}

#[test]
fn rejects_budget_exhausted_when_budget_not_reached() {
    let mut artifact = fixture("budget-exhausted.fixture.json");
    // Raise the caps so neither the iteration nor cost budget is actually reached.
    artifact.budget.max_iterations = 10;
    artifact.budget.max_cost_units = 1000;
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("budget to be reached"), "{err}");
}

#[test]
fn rejects_no_progress_when_a_gate_improved_in_window() {
    let mut artifact = fixture("no-progress.fixture.json");
    // Improve the visual gate in the final iteration, contradicting no-progress.
    if let Some(verdict) = artifact
        .iterations
        .last_mut()
        .and_then(|iteration| iteration.four_gate.iter_mut().find(|v| v.gate == "visual"))
    {
        verdict.status = EvolveGateStatus::Pass;
    }
    let err = validate_evolve_campaign(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("gate improved"), "{err}");
}

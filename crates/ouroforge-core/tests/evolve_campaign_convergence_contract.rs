use ouroforge_core::{
    compute_evolve_campaign_outcome, evolve_campaign_outcome_cli_summary, EvolveCampaignArtifact,
    EvolveCampaignOutcomeState, EvolveCampaignStopReason, EvolveGateTransition,
    EVOLVE_CAMPAIGN_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> EvolveCampaignArtifact {
    let path = workspace_path(&format!("examples/evolve-campaign-v1/convergence/{name}"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} exists: {err}", path.display()));
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn converged_campaign_reports_passing_iteration_and_no_diagnosis() {
    let outcome = compute_evolve_campaign_outcome(&fixture("converged.fixture.json")).expect("ok");
    assert_eq!(outcome.schema_version, EVOLVE_CAMPAIGN_SCHEMA_VERSION);
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::Converged);
    assert_eq!(
        outcome.stop_reason,
        EvolveCampaignStopReason::AcceptanceReached
    );
    assert_eq!(outcome.converged_iteration, Some(2));
    assert_eq!(outcome.diagnosis, None);
    assert_eq!(outcome.iteration_deltas.len(), 3);
    assert!(outcome.iteration_deltas[0].is_baseline);
    assert!(!outcome.iteration_deltas[1].is_baseline);
    assert!(outcome.iteration_deltas[2].reaches_acceptance);
    assert!(outcome.boundary.contains("descriptive"));
}

#[test]
fn converged_deltas_record_per_iteration_improvement() {
    let outcome = compute_evolve_campaign_outcome(&fixture("converged.fixture.json")).expect("ok");
    // baseline iteration has no improving transition.
    assert_eq!(outcome.iteration_deltas[0].improved_gates, 0);
    // iteration 1 fixes mechanical + runtime versus the baseline.
    assert_eq!(outcome.iteration_deltas[1].improved_gates, 2);
    assert_eq!(outcome.iteration_deltas[1].regressed_gates, 0);
    // iteration 2 fixes visual + semantic.
    assert_eq!(outcome.iteration_deltas[2].improved_gates, 2);
}

#[test]
fn not_converged_at_budget_reports_diagnosis_and_last_deltas() {
    let outcome =
        compute_evolve_campaign_outcome(&fixture("not-converged-budget.fixture.json")).expect("ok");
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::NotConverged);
    assert_eq!(
        outcome.stop_reason,
        EvolveCampaignStopReason::BudgetExhausted
    );
    assert_eq!(outcome.converged_iteration, None);
    assert!(outcome.diagnosis.is_some_and(|text| !text.is_empty()));
    assert_eq!(outcome.last_deltas.len(), 4);
}

#[test]
fn no_progress_reports_not_converged() {
    let outcome =
        compute_evolve_campaign_outcome(&fixture("no-progress.fixture.json")).expect("ok");
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::NotConverged);
    assert_eq!(outcome.stop_reason, EvolveCampaignStopReason::NoProgress);
    assert!(outcome.diagnosis.is_some());
}

#[test]
fn non_monotonic_noise_is_tracked_without_misreporting_convergence() {
    let outcome =
        compute_evolve_campaign_outcome(&fixture("non-monotonic.fixture.json")).expect("ok");
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::Converged);
    assert_eq!(outcome.converged_iteration, Some(3));
    // Iteration 2 regresses the mechanical gate while fixing the visual gate.
    let noisy = &outcome.iteration_deltas[2];
    assert!(noisy.regressed_gates >= 1, "regression recorded");
    assert!(noisy.improved_gates >= 1, "improvement recorded");
    let mechanical = noisy
        .gate_deltas
        .iter()
        .find(|delta| delta.gate == "mechanical")
        .expect("mechanical delta");
    assert_eq!(mechanical.transition, EvolveGateTransition::Regressed);
    // The final iteration restores every gate to pass.
    assert!(outcome.iteration_deltas[3].reaches_acceptance);
}

#[test]
fn rejects_budget_overflow() {
    let mut artifact = fixture("converged.fixture.json");
    artifact.budget.max_cost_units = 5; // below the summed iteration cost.
    let err = compute_evolve_campaign_outcome(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("maxCostUnits"), "{err}");
}

#[test]
fn rejects_missing_baseline_iteration() {
    let mut artifact = fixture("converged.fixture.json");
    artifact.iterations.clear();
    let err = compute_evolve_campaign_outcome(&artifact).expect_err("must reject");
    assert!(err.to_string().contains("at least one iteration"), "{err}");
}

#[test]
fn outcome_cli_summary_renders_for_every_state() {
    for name in [
        "converged.fixture.json",
        "not-converged-budget.fixture.json",
        "no-progress.fixture.json",
    ] {
        let summary = evolve_campaign_outcome_cli_summary(&fixture(name)).expect(name);
        assert!(summary.contains("descriptive-read-only"), "{name}");
        assert!(summary.contains("state="), "{name}");
    }
}

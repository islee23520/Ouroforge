//! Multi-Iteration Evolve Campaign Demo v1 (#1490).
//!
//! A deterministic, fixture-scoped smoke test for the campaign demo. It asserts
//! that the converging case reaches passing acceptance over N bounded iterations
//! with a full per-iteration audit trail, and that the non-converging case stops
//! safely at the iteration budget with an evidence-linked diagnosis. No network,
//! no live browser, no timing assertions — states and shapes only.

use ouroforge_core::{
    build_evolve_campaign_journal, compute_evolve_campaign_outcome, validate_evolve_campaign,
    EvolveCampaignArtifact, EvolveCampaignOutcomeState, EvolveCampaignStopReason,
    EVOLVE_CAMPAIGN_FOUR_GATES, EVOLVE_CAMPAIGN_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> EvolveCampaignArtifact {
    let path = workspace_path(&format!("examples/evolve-campaign-v1/demo/{name}"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("demo fixture {} exists: {err}", path.display()));
    serde_json::from_str(&text).expect("demo fixture parses")
}

#[test]
fn converging_demo_reaches_acceptance_with_audit_trail() {
    let artifact = fixture("converging.fixture.json");

    // The campaign is bounded and validates.
    let read_model = validate_evolve_campaign(&artifact).expect("converging demo validates");
    assert_eq!(read_model.schema_version, EVOLVE_CAMPAIGN_SCHEMA_VERSION);
    assert_eq!(
        read_model.stop_reason,
        EvolveCampaignStopReason::AcceptanceReached
    );
    assert!(
        read_model.iteration_count >= 2,
        "demo converges over multiple bounded iterations"
    );
    assert!(
        artifact.iterations.len() as u32 <= artifact.budget.max_iterations,
        "iterations stay within the iteration budget"
    );

    // Converged outcome with a recorded converged iteration and no diagnosis.
    let outcome = compute_evolve_campaign_outcome(&artifact).expect("converging demo outcome");
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::Converged);
    assert_eq!(
        outcome.converged_iteration, read_model.accepted_iteration,
        "converged iteration matches the accepted iteration"
    );
    assert!(outcome.diagnosis.is_none());

    // Full audit trail: a delta per iteration, the baseline starts failing, and
    // the final iteration passes every acceptance gate.
    assert_eq!(outcome.iteration_deltas.len(), outcome.iteration_count);
    assert!(outcome.iteration_deltas[0].is_baseline);
    assert!(
        !outcome.iteration_deltas[0].reaches_acceptance,
        "the demo starts from a failing baseline"
    );
    assert!(
        outcome
            .iteration_deltas
            .last()
            .expect("final delta")
            .reaches_acceptance,
        "the final iteration reaches acceptance"
    );

    // The journal narrates every iteration and links justifying evidence.
    let journal = build_evolve_campaign_journal(&artifact).expect("converging demo journal");
    assert_eq!(journal.entries.len(), journal.summary.iteration_count);
    assert_eq!(journal.summary.state, EvolveCampaignOutcomeState::Converged);
    for entry in &journal.entries {
        assert_eq!(entry.four_gate.len(), EVOLVE_CAMPAIGN_FOUR_GATES.len());
        assert!(
            !entry.evidence_refs.is_empty(),
            "each audit-trail entry links evidence"
        );
        assert!(!entry.mutation_ref.is_empty());
    }
}

#[test]
fn non_converging_demo_stops_safely_at_budget() {
    let artifact = fixture("non-converging.fixture.json");

    // Bounded: the campaign stops at the iteration budget, never unbounded.
    let read_model = validate_evolve_campaign(&artifact).expect("non-converging demo validates");
    assert_eq!(
        read_model.stop_reason,
        EvolveCampaignStopReason::BudgetExhausted
    );
    assert_eq!(
        artifact.iterations.len() as u32,
        artifact.budget.max_iterations,
        "the demo exhausts the declared iteration budget"
    );
    assert!(
        read_model.accepted_iteration.is_none(),
        "a non-converging campaign records no accepted iteration"
    );

    // Not-converged outcome carries an evidence-linked diagnosis.
    let outcome = compute_evolve_campaign_outcome(&artifact).expect("non-converging demo outcome");
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::NotConverged);
    assert!(outcome.converged_iteration.is_none());
    assert!(
        outcome
            .diagnosis
            .as_deref()
            .is_some_and(|text| text.contains("evidence")),
        "the diagnosis links justifying evidence"
    );

    // The final iteration genuinely did not reach acceptance.
    assert!(
        !outcome
            .iteration_deltas
            .last()
            .expect("final delta")
            .reaches_acceptance,
        "the demo stops without reaching acceptance"
    );

    // The journal records the safe non-convergence summary.
    let journal = build_evolve_campaign_journal(&artifact).expect("non-converging demo journal");
    assert_eq!(
        journal.summary.state,
        EvolveCampaignOutcomeState::NotConverged
    );
    assert!(journal.summary.diagnosis.is_some());
}

#[test]
fn demo_manifest_enumerates_both_cases() {
    let path = workspace_path("examples/evolve-campaign-v1/demo/manifest.json");
    let text = std::fs::read_to_string(&path).expect("demo manifest exists");
    let manifest: serde_json::Value = serde_json::from_str(&text).expect("demo manifest parses");

    assert_eq!(manifest["schemaVersion"], "evolve-campaign-demo-v1");
    assert_eq!(manifest["issue"], 1490);
    assert_eq!(manifest["deterministic"], true);
    assert_eq!(manifest["fixtureScoped"], true);

    let boundary = manifest["boundary"].as_str().expect("boundary string");
    assert!(boundary.contains("descriptive read-only"));
    assert!(boundary.contains("no auto-fix, no auto-apply, no auto-merge"));
    assert!(boundary.contains("no network or live browser"));

    let cases = manifest["cases"].as_array().expect("cases array");
    let stop_reasons: Vec<&str> = cases
        .iter()
        .map(|case| case["expectStopReason"].as_str().expect("stop reason"))
        .collect();
    assert!(stop_reasons.contains(&"acceptance-reached"));
    assert!(stop_reasons.contains(&"budget-exhausted"));

    let outcomes: Vec<&str> = cases
        .iter()
        .map(|case| case["expectOutcome"].as_str().expect("outcome"))
        .collect();
    assert!(outcomes.contains(&"converged"));
    assert!(outcomes.contains(&"not-converged"));

    // Every enumerated case fixture exists, validates, and matches the manifest.
    for case in cases {
        let name = case["fixture"].as_str().expect("fixture name");
        let artifact = fixture(name);
        let read_model = validate_evolve_campaign(&artifact).expect("manifest case validates");
        assert_eq!(
            read_model.stop_reason.as_str(),
            case["expectStopReason"].as_str().unwrap(),
            "{name} stop reason matches manifest"
        );
    }
}

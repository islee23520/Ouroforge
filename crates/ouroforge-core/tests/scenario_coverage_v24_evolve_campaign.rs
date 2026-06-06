//! Scenario Coverage v24: Evolve Campaign Regression Suite (#1491).
//!
//! Locks Multi-Iteration Evolve Campaigns v1 behavior: campaign termination
//! (acceptance / budget / no-progress), convergence outcomes
//! (converged / not-converged), the journal narrative, and the single-shot
//! evolve backward-compatibility golden. Asserts states and shapes only — no
//! flaky or timing assertions.

use ouroforge_core::{
    build_evolve_campaign_journal, compute_evolve_campaign_outcome, validate_evolve_campaign,
    EvolveCampaignArtifact, EvolveCampaignOutcomeState, EvolveCampaignStopReason,
    EVOLVE_CAMPAIGN_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> EvolveCampaignArtifact {
    let path = workspace_path(&format!(
        "examples/evolve-campaign-v1/scenario-coverage-v24/{name}"
    ));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} exists: {err}", path.display()));
    serde_json::from_str(&text).expect("fixture parses")
}

struct Case {
    name: &'static str,
    stop_reason: EvolveCampaignStopReason,
    outcome: EvolveCampaignOutcomeState,
}

const TERMINATION_CASES: &[Case] = &[
    Case {
        name: "acceptance.fixture.json",
        stop_reason: EvolveCampaignStopReason::AcceptanceReached,
        outcome: EvolveCampaignOutcomeState::Converged,
    },
    Case {
        name: "budget.fixture.json",
        stop_reason: EvolveCampaignStopReason::BudgetExhausted,
        outcome: EvolveCampaignOutcomeState::NotConverged,
    },
    Case {
        name: "no-progress.fixture.json",
        stop_reason: EvolveCampaignStopReason::NoProgress,
        outcome: EvolveCampaignOutcomeState::NotConverged,
    },
];

#[test]
fn termination_modes_are_locked() {
    for case in TERMINATION_CASES {
        let artifact = fixture(case.name);
        let read_model = validate_evolve_campaign(&artifact).expect(case.name);
        assert_eq!(read_model.schema_version, EVOLVE_CAMPAIGN_SCHEMA_VERSION);
        assert_eq!(read_model.stop_reason, case.stop_reason, "{}", case.name);
    }
}

#[test]
fn convergence_outcomes_are_locked() {
    for case in TERMINATION_CASES {
        let outcome = compute_evolve_campaign_outcome(&fixture(case.name)).expect(case.name);
        assert_eq!(outcome.state, case.outcome, "{}", case.name);
        match case.outcome {
            EvolveCampaignOutcomeState::Converged => {
                assert!(outcome.converged_iteration.is_some(), "{}", case.name);
                assert!(outcome.diagnosis.is_none(), "{}", case.name);
            }
            EvolveCampaignOutcomeState::NotConverged => {
                assert!(outcome.converged_iteration.is_none(), "{}", case.name);
                assert!(
                    outcome.diagnosis.is_some_and(|text| !text.is_empty()),
                    "{}",
                    case.name
                );
            }
        }
        // The four-gate delta trajectory is recorded for every iteration.
        assert_eq!(
            outcome.iteration_deltas.len(),
            outcome.iteration_count,
            "{}",
            case.name
        );
    }
}

#[test]
fn journal_narratives_are_locked() {
    for case in TERMINATION_CASES {
        let journal = build_evolve_campaign_journal(&fixture(case.name)).expect(case.name);
        assert_eq!(
            journal.entries.len(),
            journal.summary.iteration_count,
            "{}",
            case.name
        );
        assert!(journal.entries[0].is_baseline, "{}", case.name);
        assert_eq!(journal.summary.state, case.outcome, "{}", case.name);
        // Every entry links justifying evidence.
        for entry in &journal.entries {
            assert!(!entry.evidence_refs.is_empty(), "{}", case.name);
        }
    }
}

#[test]
fn single_shot_evolve_remains_valid() {
    let artifact = fixture("single-shot.fixture.json");
    assert_eq!(artifact.iterations.len(), 1, "single-shot is one iteration");
    let read_model = validate_evolve_campaign(&artifact).expect("single-shot validates");
    assert_eq!(read_model.iteration_count, 1);
    assert_eq!(
        read_model.stop_reason,
        EvolveCampaignStopReason::AcceptanceReached
    );

    let outcome = compute_evolve_campaign_outcome(&artifact).expect("single-shot outcome");
    assert_eq!(outcome.state, EvolveCampaignOutcomeState::Converged);
    assert_eq!(outcome.converged_iteration, Some(0));
    // The baseline iteration is the converged iteration for a single-shot run.
    assert!(outcome.iteration_deltas[0].is_baseline);
    assert!(outcome.iteration_deltas[0].reaches_acceptance);

    // The single-shot campaign also produces a valid (baseline-only) narrative.
    let journal = build_evolve_campaign_journal(&artifact).expect("single-shot journal");
    assert_eq!(journal.entries.len(), 1);
}

#[test]
fn coverage_matrix_enumerates_required_cases() {
    let path = workspace_path("examples/evolve-campaign-v1/scenario-coverage-v24/matrix.json");
    let text = std::fs::read_to_string(&path).expect("matrix exists");
    let matrix: serde_json::Value = serde_json::from_str(&text).expect("matrix parses");

    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v24-evolve-campaign-v1"
    );
    assert_eq!(matrix["issue"], 1491);
    assert_eq!(matrix["fixtureScoped"], true);
    let boundary = matrix["boundary"].as_str().expect("boundary string");
    assert!(boundary.contains("no auto-fix, no auto-apply, no auto-merge"));
    assert!(boundary.contains("descriptive read-only"));

    let stop_reasons: Vec<&str> = matrix["terminationCases"]
        .as_array()
        .expect("termination cases")
        .iter()
        .map(|case| case["expectStopReason"].as_str().expect("stop reason"))
        .collect();
    for reason in ["acceptance-reached", "budget-exhausted", "no-progress"] {
        assert!(stop_reasons.contains(&reason), "matrix covers {reason}");
    }

    let outcomes: Vec<&str> = matrix["terminationCases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["expectOutcome"].as_str().expect("outcome"))
        .collect();
    assert!(outcomes.contains(&"converged"));
    assert!(outcomes.contains(&"not-converged"));

    assert!(
        !matrix["backwardCompatCases"]
            .as_array()
            .expect("backward-compat cases")
            .is_empty(),
        "matrix includes a single-shot backward-compat case"
    );
}

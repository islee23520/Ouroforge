// Consolidated evolve contract tests.
// Source files: evolve_campaign_contract.rs, evolve_campaign_convergence_contract.rs,
//   evolve_campaign_demo_contract.rs, evolve_campaign_journal_contract.rs,
//   evolve_proposal_evidence_contract.rs, evolve_proposal_selection_contract.rs,
//   evolve_rerun_comparison_contract.rs

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

// ---------------------------------------------------------------------------
// evolve_campaign_contract
// ---------------------------------------------------------------------------

mod evolve_campaign_contract {
    use ouroforge_core::{
        evolve_campaign_cli_summary, iteration_reaches_acceptance, validate_evolve_campaign,
        EvolveCampaignArtifact, EvolveCampaignStopReason, EvolveGateStatus, EvolveGateVerdict,
        EVOLVE_CAMPAIGN_SCHEMA_VERSION,
    };

    fn fixture(name: &str) -> EvolveCampaignArtifact {
        let path = super::workspace_path(&format!("examples/evolve-campaign-v1/contract/{name}"));
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
    fn rejects_missing_repo_relative_seed_ref() {
        let mut artifact = fixture("acceptance-reached.fixture.json");
        artifact.seed_ref = "examples/evolve-campaign-v1/contract/missing-seed.yaml".to_string();
        let err = validate_evolve_campaign(&artifact).expect_err("must reject");
        assert!(
            err.to_string().contains(
                "missing repo-relative ref `examples/evolve-campaign-v1/contract/missing-seed.yaml`"
            ),
            "{err}"
        );
    }

    #[test]
    fn rejects_missing_repo_relative_mutation_ref() {
        let mut artifact = fixture("acceptance-reached.fixture.json");
        artifact.iterations[0].mutation_ref =
            "examples/evolve-campaign-v1/contract/missing-mutation.json".to_string();
        let err = validate_evolve_campaign(&artifact).expect_err("must reject");
        assert!(
            err.to_string()
                .contains("missing repo-relative ref `examples/evolve-campaign-v1/contract/missing-mutation.json`"),
            "{err}"
        );
    }

    #[test]
    fn rejects_missing_repo_relative_evidence_ref() {
        let mut artifact = fixture("acceptance-reached.fixture.json");
        artifact.iterations[0].evidence_refs =
            vec!["examples/evolve-campaign-v1/contract/missing-evidence.json".to_string()];
        let err = validate_evolve_campaign(&artifact).expect_err("must reject");
        assert!(
            err.to_string()
                .contains("missing repo-relative ref `examples/evolve-campaign-v1/contract/missing-evidence.json`"),
            "{err}"
        );
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
}

// ---------------------------------------------------------------------------
// evolve_campaign_convergence_contract
// ---------------------------------------------------------------------------

mod evolve_campaign_convergence_contract {
    use ouroforge_core::{
        compute_evolve_campaign_outcome, evolve_campaign_outcome_cli_summary,
        EvolveCampaignArtifact, EvolveCampaignOutcomeState, EvolveCampaignStopReason,
        EvolveGateTransition, EVOLVE_CAMPAIGN_SCHEMA_VERSION,
    };

    fn fixture(name: &str) -> EvolveCampaignArtifact {
        let path = super::workspace_path(&format!("examples/evolve-campaign-v1/convergence/{name}"));
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
}

// ---------------------------------------------------------------------------
// evolve_campaign_demo_contract
// ---------------------------------------------------------------------------

mod evolve_campaign_demo_contract {
    //! Multi-Iteration Evolve Campaign Demo v1 (#1490).
    //!
    //! A deterministic, fixture-scoped smoke test for the campaign demo. It asserts
    //! that the converging case reaches passing acceptance over N bounded iterations
    //! with a full per-iteration audit trail, and that the non-converging case stops
    //! safely at the iteration budget with an evidence-linked diagnosis. No network,
    //! no live browser, no timing assertions -- states and shapes only.

    use ouroforge_core::{
        build_evolve_campaign_journal, compute_evolve_campaign_outcome, validate_evolve_campaign,
        EvolveCampaignArtifact, EvolveCampaignOutcomeState, EvolveCampaignStopReason,
        EVOLVE_CAMPAIGN_FOUR_GATES, EVOLVE_CAMPAIGN_SCHEMA_VERSION,
    };

    fn fixture(name: &str) -> EvolveCampaignArtifact {
        let path = super::workspace_path(&format!("examples/evolve-campaign-v1/demo/{name}"));
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
        let path = super::workspace_path("examples/evolve-campaign-v1/demo/manifest.json");
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
}

// ---------------------------------------------------------------------------
// evolve_campaign_journal_contract
// ---------------------------------------------------------------------------

mod evolve_campaign_journal_contract {
    use ouroforge_core::{
        build_evolve_campaign_journal, render_evolve_campaign_journal_markdown,
        EvolveCampaignArtifact, EvolveCampaignOutcomeState, EvolveCampaignStopReason,
        EVOLVE_CAMPAIGN_SCHEMA_VERSION,
    };

    fn fixture(name: &str) -> EvolveCampaignArtifact {
        let path = super::workspace_path(&format!("examples/evolve-campaign-v1/journal/{name}"));
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} exists: {err}", path.display()));
        serde_json::from_str(&text).expect("fixture parses")
    }

    #[test]
    fn multi_iteration_narrative_links_deltas_and_evidence() {
        let journal =
            build_evolve_campaign_journal(&fixture("converged-narrative.fixture.json")).expect("ok");
        assert_eq!(journal.schema_version, EVOLVE_CAMPAIGN_SCHEMA_VERSION);
        assert_eq!(journal.entries.len(), 3);

        let baseline = &journal.entries[0];
        assert!(baseline.is_baseline);
        assert_eq!(baseline.failing_gates.len(), 4);

        // Iteration 1 fixes mechanical + runtime; visual + semantic still fail.
        let second = &journal.entries[1];
        assert!(!second.is_baseline);
        assert_eq!(second.failing_gates, vec!["visual", "semantic"]);
        let improved: Vec<&str> = second
            .gate_deltas
            .iter()
            .filter(|delta| delta.transition == ouroforge_core::EvolveGateTransition::Improved)
            .map(|delta| delta.gate.as_str())
            .collect();
        assert!(improved.contains(&"mechanical"));
        assert!(improved.contains(&"runtime"));
        // Each entry links its justifying evidence.
        assert!(!second.evidence_refs.is_empty());

        // Final iteration reaches acceptance.
        assert!(journal.entries[2].reaches_acceptance);
        assert_eq!(journal.summary.state, EvolveCampaignOutcomeState::Converged);
        assert_eq!(journal.summary.converged_iteration, Some(2));
        assert_eq!(journal.summary.diagnosis, None);
    }

    #[test]
    fn not_converged_summary_carries_diagnosis() {
        let journal = build_evolve_campaign_journal(&fixture("not-converged-narrative.fixture.json"))
            .expect("ok");
        assert_eq!(
            journal.summary.state,
            EvolveCampaignOutcomeState::NotConverged
        );
        assert_eq!(
            journal.summary.stop_reason,
            EvolveCampaignStopReason::BudgetExhausted
        );
        assert_eq!(journal.summary.converged_iteration, None);
        assert!(journal
            .summary
            .diagnosis
            .as_ref()
            .is_some_and(|text| !text.is_empty()));
    }

    #[test]
    fn markdown_narrative_renders_iterations_and_summary() {
        let markdown =
            render_evolve_campaign_journal_markdown(&fixture("converged-narrative.fixture.json"))
                .expect("ok");
        assert!(markdown.contains("# Evolve Campaign Journal: campaign-journal-converged"));
        assert!(markdown.contains("## Iteration 0 (baseline)"));
        assert!(markdown.contains("## Iteration 1"));
        assert!(markdown.contains("Rerun delta:"));
        assert!(markdown.contains("mechanical improved (fail -> pass)"));
        assert!(markdown.contains("## Campaign summary"));
        assert!(markdown.contains("Outcome: converged"));
        // Evidence artifacts are linked in the narrative.
        assert!(markdown.contains("`runs/campaign/iteration-2/verdict.json`"));
    }

    #[test]
    fn not_converged_markdown_renders_diagnosis_line() {
        let markdown =
            render_evolve_campaign_journal_markdown(&fixture("not-converged-narrative.fixture.json"))
                .expect("ok");
        assert!(markdown.contains("Outcome: not-converged"));
        assert!(markdown.contains("Diagnosis:"));
        assert!(markdown.contains("Converged at iteration: none"));
    }

    #[test]
    fn rejects_iteration_missing_evidence() {
        let mut artifact = fixture("converged-narrative.fixture.json");
        artifact.iterations[1].evidence_refs.clear();
        let err = build_evolve_campaign_journal(&artifact).expect_err("must reject");
        assert!(err.to_string().contains("missing linked evidence"), "{err}");
    }

    #[test]
    fn rejects_gap_in_iteration_sequence() {
        let mut artifact = fixture("converged-narrative.fixture.json");
        // Introduce a gap: relabel the middle iteration's index.
        artifact.iterations[1].index = 5;
        let err = build_evolve_campaign_journal(&artifact).expect_err("must reject");
        let message = err.to_string();
        assert!(
            message.contains("position") || message.contains("sequence"),
            "{message}"
        );
    }

    #[test]
    fn rejects_stale_refs() {
        let mut artifact = fixture("converged-narrative.fixture.json");
        artifact.iterations[0].evidence_refs = vec!["../escape/verdict.json".to_string()];
        let err = build_evolve_campaign_journal(&artifact).expect_err("must reject");
        assert!(err.to_string().contains("safe repo-relative"), "{err}");
    }
}

// ---------------------------------------------------------------------------
// evolve_proposal_evidence_contract
// ---------------------------------------------------------------------------

mod evolve_proposal_evidence_contract {
    use ouroforge_core::{
        add_evidence_artifact, create_run, evolve_run, list_mutation_proposals,
        MutationProposalBoundedMutationType, MutationProposalEvidenceState,
        MutationProposalGateCategory, MutationProposalRationaleConfidence,
    };
    use serde_json::{json, Value};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    const SEED: &str = r#"
id: evolve.proposal.evidence.contract
title: Evolve Proposal Evidence Contract
goal: Prove failed evidence creates bounded, evidence-linked mutation proposals.
constraints:
  target: local-fixture
acceptance:
  - Mutation proposal cites a failing gate and evidence.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed failure scenario.
"#;

    #[test]
    fn evolve_proposal_cites_gate_and_evidence_for_each_four_gate_failure() {
        for case in [
            GateCase {
                name: "mechanical",
                failure: json!({
                    "kind": "scenario_failed",
                    "path": "evidence/scenarios/collect-and-exit/scenario-result.json"
                }),
                evidence_id: "scenario-result",
                evidence_path: "evidence/scenarios/collect-and-exit/scenario-result.json",
                artifact_metadata: json!({"artifact":"scenario_result"}),
                expected_gate: MutationProposalGateCategory::Mechanical,
                expected_bounded_type: MutationProposalBoundedMutationType::Scenario,
                expected_confidence: MutationProposalRationaleConfidence::High,
            },
            GateCase {
                name: "runtime",
                failure: json!({
                    "kind": "behavior_assertion_failed",
                    "evidence_ref": "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"
                }),
                evidence_id: "runtime-probe",
                evidence_path: "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                artifact_metadata: json!({"artifact":"runtime_probe"}),
                expected_gate: MutationProposalGateCategory::Runtime,
                expected_bounded_type: MutationProposalBoundedMutationType::Data,
                expected_confidence: MutationProposalRationaleConfidence::High,
            },
            GateCase {
                name: "visual",
                failure: json!({
                    "kind": "visual_gate_failed",
                    "state": "fail",
                    "path": "evidence/scenarios/collect-and-exit/visual/visual-comparison.json"
                }),
                evidence_id: "visual-comparison",
                evidence_path: "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
                artifact_metadata: json!({"artifact":"visual_comparison_evidence", "gate":"visual", "declaredAcceptance": true}),
                expected_gate: MutationProposalGateCategory::Visual,
                expected_bounded_type: MutationProposalBoundedMutationType::Scene,
                expected_confidence: MutationProposalRationaleConfidence::High,
            },
            GateCase {
                name: "semantic",
                failure: json!({
                    "kind": "semantic_gate_failed",
                    "state": "fail",
                    "model_ref": "evidence/scenarios/collect-and-exit/semantic/runtime-invariant-model.json",
                    "world_state_ref": "evidence/scenarios/collect-and-exit/world-state.json"
                }),
                evidence_id: "semantic-model",
                evidence_path:
                    "evidence/scenarios/collect-and-exit/semantic/runtime-invariant-model.json",
                artifact_metadata: json!({"artifact":"runtime_invariant_model", "gate":"semantic", "declaredAcceptance": true}),
                expected_gate: MutationProposalGateCategory::Semantic,
                expected_bounded_type: MutationProposalBoundedMutationType::Data,
                expected_confidence: MutationProposalRationaleConfidence::High,
            },
        ] {
            let (root, run_dir) = create_fixture_run(&format!("evolve-proposal-{}", case.name));
            write_indexed_evidence(
                &run_dir,
                "scenario-result",
                "evidence/scenarios/collect-and-exit/scenario-result.json",
                json!({"artifact":"scenario_result"}),
            );
            if case.evidence_id != "scenario-result" {
                write_indexed_evidence(
                    &run_dir,
                    case.evidence_id,
                    case.evidence_path,
                    case.artifact_metadata.clone(),
                );
            }
            write_failed_verdict(
                &run_dir,
                case.failure.clone(),
                vec![
                    "evidence/scenarios/collect-and-exit/scenario-result.json",
                    case.evidence_path,
                ],
            );

            let summary = evolve_run(&run_dir).expect("evolve creates proposal");
            assert_eq!(summary.status, "proposed", "{}", case.name);
            assert_eq!(summary.proposals_created, 1, "{}", case.name);
            let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
            let rationale = proposals[0].rationale.as_ref().expect("rationale");

            assert_eq!(proposals[0].evidence_id, case.evidence_id, "{}", case.name);
            assert_eq!(
                rationale.failing_gate_category,
                Some(case.expected_gate),
                "{}",
                case.name
            );
            assert_eq!(
                rationale.justifying_evidence_ref.as_deref(),
                Some(case.evidence_path),
                "{}",
                case.name
            );
            assert_eq!(
                rationale.evidence_state,
                Some(MutationProposalEvidenceState::Linked),
                "{}",
                case.name
            );
            assert_eq!(
                rationale.bounded_mutation_type,
                Some(case.expected_bounded_type),
                "{}",
                case.name
            );
            assert_eq!(
                rationale.confidence, case.expected_confidence,
                "{}",
                case.name
            );
            assert!(
                rationale.reasoning_summary.contains("evidence"),
                "{}",
                case.name
            );

            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn evolve_proposal_fails_closed_when_justifying_evidence_is_missing() {
        let (root, run_dir) = create_fixture_run("evolve-proposal-missing-evidence");
        write_indexed_evidence(
            &run_dir,
            "unrelated",
            "evidence/scenarios/collect-and-exit/unrelated.json",
            json!({"artifact":"unrelated"}),
        );
        write_failed_verdict(
            &run_dir,
            json!({
                "kind": "visual_gate_failed",
                "state": "fail",
                "path": "evidence/scenarios/collect-and-exit/visual/missing-comparison.json"
            }),
            vec!["evidence/scenarios/collect-and-exit/visual/missing-comparison.json"],
        );

        let summary = evolve_run(&run_dir).expect("missing evidence is fail-closed summary");

        assert_eq!(summary.status, "missing-evidence");
        assert_eq!(summary.proposals_created, 0);
        assert!(summary
            .reason
            .contains("no mutation proposal was fabricated"));
        assert!(list_mutation_proposals(&run_dir)
            .expect("proposal list")
            .is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn evolve_proposal_fails_closed_when_justifying_evidence_is_stale() {
        let (root, run_dir) = create_fixture_run("evolve-proposal-stale-evidence");
        write_indexed_evidence(
            &run_dir,
            "unrelated",
            "evidence/scenarios/collect-and-exit/unrelated.json",
            json!({"artifact":"unrelated"}),
        );
        write_failed_verdict(
            &run_dir,
            json!({
                "kind": "visual_gate_failed",
                "state": "stale-ref",
                "path": "evidence/scenarios/collect-and-exit/visual/stale-comparison.json"
            }),
            vec!["evidence/scenarios/collect-and-exit/visual/stale-comparison.json"],
        );

        let summary = evolve_run(&run_dir).expect("stale evidence is fail-closed summary");

        assert_eq!(summary.status, "stale-ref");
        assert_eq!(summary.proposals_created, 0);
        assert!(summary
            .reason
            .contains("no mutation proposal was fabricated"));
        assert!(list_mutation_proposals(&run_dir)
            .expect("proposal list")
            .is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn evolve_proposal_fails_closed_for_unsupported_gate_even_with_linked_evidence() {
        let (root, run_dir) = create_fixture_run("evolve-proposal-unsupported-gate");
        write_indexed_evidence(
            &run_dir,
            "source-patch-request",
            "evidence/scenarios/collect-and-exit/source-patch-request.json",
            json!({"artifact":"source_patch_request"}),
        );
        write_failed_verdict(
            &run_dir,
            json!({
                "kind": "source_patch_requested",
                "gate_category": "source_patch",
                "path": "evidence/scenarios/collect-and-exit/source-patch-request.json"
            }),
            vec!["evidence/scenarios/collect-and-exit/source-patch-request.json"],
        );

        let summary = evolve_run(&run_dir).expect("unsupported gate is fail-closed summary");

        assert_eq!(summary.status, "unsupported");
        assert_eq!(summary.proposals_created, 0);
        assert!(summary
            .reason
            .contains("no mutation proposal was fabricated"));
        assert!(list_mutation_proposals(&run_dir)
            .expect("proposal list")
            .is_empty());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn mutation_proposal_rejects_unsupported_gate_category_and_bounded_type_drift() {
        let (root, run_dir) = create_fixture_run("evolve-proposal-invalid-rationale");
        fs::create_dir_all(run_dir.join("mutation")).unwrap();
        fs::write(
            run_dir.join("mutation/proposals.json"),
            serde_json::to_vec_pretty(&json!({
                "proposals": [{
                    "id": "mutation-1",
                    "reason": "invalid broad source mutation",
                    "evidence_id": "evidence-1",
                    "target": "src/lib.rs",
                    "path": "source",
                    "from": "old",
                    "to": "new",
                    "confidence": "medium",
                    "status": "proposed",
                    "verdict_status": "failed",
                    "created_at_unix_ms": 1,
                    "rationale": {
                        "schema_version": "1",
                        "failure_classification": "semantic_root_cause",
                        "evidence_artifact_ids": ["evidence-1"],
                        "scenario_result_refs": ["evidence/scenarios/collect-and-exit/scenario-result.json"],
                        "verdict_refs": ["verdict.json"],
                        "expected_effect": "invalid source patch",
                        "confidence": "medium",
                        "reasoning_summary": "invalid",
                        "allowed_mutation_type": "data_only",
                        "failing_gate_category": "source_patch",
                        "justifying_evidence_ref": "evidence/source.json",
                        "evidence_state": "linked",
                        "bounded_mutation_type": "source"
                    }
                }]
            }))
            .unwrap(),
        )
        .unwrap();

        let error = list_mutation_proposals(&run_dir).expect_err("unsupported enums fail");

        assert!(error
            .to_string()
            .contains("failed to parse mutation proposals"));
        fs::remove_dir_all(root).unwrap();
    }

    struct GateCase {
        name: &'static str,
        failure: Value,
        evidence_id: &'static str,
        evidence_path: &'static str,
        artifact_metadata: Value,
        expected_gate: MutationProposalGateCategory,
        expected_bounded_type: MutationProposalBoundedMutationType,
        expected_confidence: MutationProposalRationaleConfidence,
    }

    fn create_fixture_run(prefix: &str) -> (PathBuf, PathBuf) {
        let root = std::env::temp_dir().join(format!("ouroforge-{prefix}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, SEED).unwrap();
        let artifacts = create_run(&seed_path, root.join("runs")).unwrap();
        (root, artifacts.run_dir)
    }

    fn write_indexed_evidence(run_dir: &Path, id: &str, rel: &str, metadata: Value) {
        let path = run_dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "{}\n").unwrap();
        add_evidence_artifact(run_dir, id, "application/json", rel, metadata).unwrap();
    }

    fn write_failed_verdict(run_dir: &Path, failure: Value, evidence_refs: Vec<&str>) {
        fs::write(
            run_dir.join("verdict.json"),
            serde_json::to_vec_pretty(&json!({
                "status": "failed",
                "summary": "fixture failed with evidence-linked gate",
                "failures": [failure],
                "evidence_refs": evidence_refs,
                "metadata": {}
            }))
            .unwrap(),
        )
        .unwrap();
    }
}

// ---------------------------------------------------------------------------
// evolve_proposal_selection_contract
// ---------------------------------------------------------------------------

mod evolve_proposal_selection_contract {
    use ouroforge_core::{
        add_evidence_artifact, create_run, evolve_run, list_mutation_proposals,
        write_mutation_backlog_artifact, MutationBacklogArtifact, MutationBacklogItem,
        MutationBacklogSeverity, MutationClassificationCategory, MutationProposalBoundedMutationType,
    };
    use serde_json::{json, Value};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    const SEED: &str = r#"
id: evolve.proposal.selection.contract
title: Evolve Proposal Selection Contract
goal: Prove classified failures select bounded proposals from a read-only backlog.
constraints:
  target: local-fixture
acceptance:
  - Classification-driven proposal selection is bounded and review-only.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed failure scenario.
"#;

    #[test]
    fn classification_taxonomy_maps_to_bounded_proposal_types() {
        for case in mapped_cases() {
            let (root, run_dir) = create_fixture_run(&format!("evolve-selection-{}", case.name));
            write_indexed_evidence(
                &run_dir,
                "failure-evidence",
                case.evidence_path,
                case.metadata,
            );
            write_failed_verdict(&run_dir, case.failure, vec![case.evidence_path]);
            write_backlog_item(
                &run_dir,
                "backlog-1",
                "classification-1",
                case.category.clone(),
                case.expected_type.clone(),
                MutationBacklogSeverity::Medium,
                vec![case.evidence_path],
            );

            let summary = evolve_run(&run_dir).expect("mapped class creates proposal");

            assert_eq!(summary.status, "proposed", "{}", case.name);
            assert_eq!(summary.proposals_created, 1, "{}", case.name);
            let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
            let rationale = proposals[0].rationale.as_ref().expect("rationale");
            assert_eq!(
                rationale.bounded_mutation_type,
                Some(case.expected_type.clone()),
                "{}",
                case.name
            );
            assert_eq!(
                rationale.failure_classification, case.expected_label,
                "{}",
                case.name
            );
            assert_eq!(
                rationale.selection_backlog_item_id.as_deref(),
                Some("backlog-1"),
                "{}",
                case.name
            );
            assert_eq!(
                rationale.selection_source.as_deref(),
                Some("mutation/backlog.json")
            );
            assert_eq!(rationale.backlog_read_only, Some(true));

            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn backlog_only_classes_do_not_fabricate_proposals() {
        for (name, marker, category, label) in [
            (
                "flaky",
                "flaky rerun marker",
                MutationClassificationCategory::Flaky,
                "flaky",
            ),
            (
                "unsupported",
                "unsupported mechanic marker",
                MutationClassificationCategory::Unsupported,
                "unsupported",
            ),
            (
                "unknown",
                "opaque marker",
                MutationClassificationCategory::Unknown,
                "unknown",
            ),
        ] {
            let (root, run_dir) = create_fixture_run(&format!("evolve-selection-{name}"));
            write_indexed_evidence(
                &run_dir,
                "failure-evidence",
                "evidence/scenarios/collect-and-exit/scenario-result.json",
                json!({"artifact":"scenario_result", "marker": marker}),
            );
            write_failed_verdict(
                &run_dir,
                json!({
                    "kind": "classified_failure",
                    "classification": label,
                    "summary": marker,
                    "path": "evidence/scenarios/collect-and-exit/scenario-result.json"
                }),
                vec!["evidence/scenarios/collect-and-exit/scenario-result.json"],
            );
            if category != MutationClassificationCategory::Unknown {
                write_backlog_item(
                    &run_dir,
                    "backlog-only",
                    "classification-1",
                    category,
                    MutationProposalBoundedMutationType::Data,
                    MutationBacklogSeverity::High,
                    vec!["evidence/scenarios/collect-and-exit/scenario-result.json"],
                );
            }

            let summary = evolve_run(&run_dir).expect("backlog-only class is handled");

            assert_eq!(summary.status, "backlog-only", "{name}");
            assert_eq!(summary.proposals_created, 0, "{name}");
            assert!(list_mutation_proposals(&run_dir)
                .expect("proposal list")
                .is_empty());
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn selection_validates_missing_stale_and_bounded_backlog_refs() {
        for case in [
            InvalidBacklogCase {
                name: "missing-backlog-ref",
                classification_id: "classification-missing",
                backlog_category: MutationClassificationCategory::GameplayLogic,
                backlog_type: MutationProposalBoundedMutationType::Data,
                evidence_refs: vec![
                    "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
                ],
                expected_error: "missing-backlog-ref",
            },
            InvalidBacklogCase {
                name: "stale-ref",
                classification_id: "classification-1",
                backlog_category: MutationClassificationCategory::VisualMismatch,
                backlog_type: MutationProposalBoundedMutationType::Scene,
                evidence_refs: vec!["evidence/scenarios/collect-and-exit/visual/stale.json"],
                expected_error: "stale-ref",
            },
            InvalidBacklogCase {
                name: "missing-classification",
                classification_id: "classification-missing",
                backlog_category: MutationClassificationCategory::VisualMismatch,
                backlog_type: MutationProposalBoundedMutationType::Scene,
                evidence_refs: vec![
                    "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
                ],
                expected_error: "missing-classification",
            },
            InvalidBacklogCase {
                name: "bounded-type-violation",
                classification_id: "classification-1",
                backlog_category: MutationClassificationCategory::VisualMismatch,
                backlog_type: MutationProposalBoundedMutationType::Data,
                evidence_refs: vec![
                    "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
                ],
                expected_error: "bounded-type-violation",
            },
        ] {
            let (root, run_dir) = create_fixture_run(&format!("evolve-selection-{}", case.name));
            write_indexed_evidence(
                &run_dir,
                "visual-evidence",
                "evidence/scenarios/collect-and-exit/visual/visual-comparison.json",
                json!({"artifact":"visual_comparison_evidence", "gate":"visual"}),
            );
            write_failed_verdict(
                &run_dir,
                json!({
                    "kind": "visual_gate_failed",
                    "path": "evidence/scenarios/collect-and-exit/visual/visual-comparison.json"
                }),
                vec!["evidence/scenarios/collect-and-exit/visual/visual-comparison.json"],
            );
            write_backlog_item(
                &run_dir,
                "bad-backlog",
                case.classification_id,
                case.backlog_category.clone(),
                case.backlog_type.clone(),
                MutationBacklogSeverity::Critical,
                case.evidence_refs,
            );

            let error = evolve_run(&run_dir).expect_err("invalid backlog blocks proposal");

            assert!(
                error.to_string().contains(case.expected_error),
                "{:#}",
                error
            );
            assert!(list_mutation_proposals(&run_dir)
                .expect("proposal list")
                .is_empty());
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn backlog_selection_is_read_only_and_prefers_severity_with_repro_context() {
        let (root, run_dir) = create_fixture_run("evolve-selection-read-only-backlog");
        write_indexed_evidence(
            &run_dir,
            "runtime-evidence",
            "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
            json!({"artifact":"runtime_probe"}),
        );
        write_indexed_evidence(
            &run_dir,
            "scenario-result",
            "evidence/scenarios/collect-and-exit/scenario-result.json",
            json!({"artifact":"scenario_result"}),
        );
        write_failed_verdict(
            &run_dir,
            json!({
                "kind": "runtime_probe_failed",
                "path": "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"
            }),
            vec![
                "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                "evidence/scenarios/collect-and-exit/scenario-result.json",
            ],
        );
        write_backlog_items(
            &run_dir,
            vec![
                backlog_item(
                    "medium-item",
                    "classification-1",
                    MutationClassificationCategory::ProbeFailure,
                    MutationProposalBoundedMutationType::Data,
                    MutationBacklogSeverity::Medium,
                    vec![
                        "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                        "evidence/scenarios/collect-and-exit/scenario-result.json",
                    ],
                ),
                backlog_item(
                    "critical-item",
                    "classification-1",
                    MutationClassificationCategory::ProbeFailure,
                    MutationProposalBoundedMutationType::Data,
                    MutationBacklogSeverity::Critical,
                    vec![
                        "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                        "evidence/scenarios/collect-and-exit/scenario-result.json",
                    ],
                ),
            ],
        );
        let before = fs::read(run_dir.join("mutation/backlog.json")).unwrap();

        let summary = evolve_run(&run_dir).expect("runtime backlog selects proposal");

        assert_eq!(summary.status, "proposed");
        let after = fs::read(run_dir.join("mutation/backlog.json")).unwrap();
        assert_eq!(before, after, "selection must consume backlog read-only");
        let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
        let rationale = proposals[0].rationale.as_ref().expect("rationale");
        assert_eq!(
            rationale.selection_backlog_item_id.as_deref(),
            Some("critical-item")
        );
        assert_eq!(rationale.backlog_read_only, Some(true));
        assert!(rationale
            .selection_reason
            .as_deref()
            .unwrap_or_default()
            .contains("without mutating backlog state"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn backlog_selection_prefers_higher_severity_across_later_classifications() {
        let (root, run_dir) = create_fixture_run("evolve-selection-global-severity");
        write_indexed_evidence(
            &run_dir,
            "probe-evidence",
            "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
            json!({"artifact":"runtime_probe"}),
        );
        write_indexed_evidence(
            &run_dir,
            "gameplay-evidence",
            "evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json",
            json!({"artifact":"gameplay_failure"}),
        );
        write_indexed_evidence(
            &run_dir,
            "scenario-result",
            "evidence/scenarios/collect-and-exit/scenario-result.json",
            json!({"artifact":"scenario_result"}),
        );
        // A verdict with TWO failures yields classification-1 (probe) and classification-2
        // (gameplay), in that order.
        fs::write(
            run_dir.join("verdict.json"),
            serde_json::to_vec_pretty(&json!({
                "status": "failed",
                "summary": "two distinct classified failures",
                "failures": [
                    {
                        "kind": "probe_failed",
                        "classification_category": "probe_failure",
                        "path": "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"
                    },
                    {
                        "kind": "gameplay_logic_failed",
                        "classification_category": "gameplay_logic",
                        "path": "evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json"
                    }
                ],
                "evidence_refs": [
                    "evidence/scenarios/collect-and-exit/runtime/runtime-probe.json",
                    "evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json"
                ],
                "metadata": {}
            }))
            .unwrap(),
        )
        .unwrap();
        // classification-1 (probe) carries only a Medium backlog item, while the LATER
        // classification-2 (gameplay) carries a higher-severity Critical item.
        write_backlog_items(
            &run_dir,
            vec![
                backlog_item(
                    "probe-medium",
                    "classification-1",
                    MutationClassificationCategory::ProbeFailure,
                    MutationProposalBoundedMutationType::Data,
                    MutationBacklogSeverity::Medium,
                    vec!["evidence/scenarios/collect-and-exit/runtime/runtime-probe.json"],
                ),
                backlog_item(
                    "gameplay-critical",
                    "classification-2",
                    MutationClassificationCategory::GameplayLogic,
                    MutationProposalBoundedMutationType::Data,
                    MutationBacklogSeverity::Critical,
                    vec!["evidence/scenarios/collect-and-exit/gameplay/gameplay-failure.json"],
                ),
            ],
        );

        let summary = evolve_run(&run_dir).expect("multi-classification backlog selects a proposal");
        assert_eq!(summary.status, "proposed");

        let proposals = list_mutation_proposals(&run_dir).expect("proposal list");
        let rationale = proposals[0].rationale.as_ref().expect("rationale");
        // The globally highest-severity backlog item must win even though it belongs to the
        // later classification-2 rather than the first classification.
        assert_eq!(
            rationale.selection_backlog_item_id.as_deref(),
            Some("gameplay-critical")
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[derive(Clone)]
    struct MappedCase {
        name: &'static str,
        failure: Value,
        evidence_path: &'static str,
        metadata: Value,
        category: MutationClassificationCategory,
        expected_label: &'static str,
        expected_type: MutationProposalBoundedMutationType,
    }

    fn mapped_cases() -> Vec<MappedCase> {
        vec![
            mapped(
                "gameplay",
                "gameplay logic assertion",
                MutationClassificationCategory::GameplayLogic,
                "gameplay_logic",
                MutationProposalBoundedMutationType::Data,
            ),
            mapped(
                "level",
                "level design route",
                MutationClassificationCategory::LevelDesign,
                "level_design",
                MutationProposalBoundedMutationType::Scene,
            ),
            mapped(
                "asset",
                "asset sprite missing",
                MutationClassificationCategory::Asset,
                "asset",
                MutationProposalBoundedMutationType::Data,
            ),
            mapped(
                "physics",
                "physics/collision overlap",
                MutationClassificationCategory::PhysicsCollision,
                "physics_collision",
                MutationProposalBoundedMutationType::Data,
            ),
            mapped(
                "input",
                "input control dropped",
                MutationClassificationCategory::Input,
                "input",
                MutationProposalBoundedMutationType::Scenario,
            ),
            mapped(
                "performance",
                "performance metric over budget",
                MutationClassificationCategory::PerformanceRegression,
                "performance_regression",
                MutationProposalBoundedMutationType::Data,
            ),
            mapped(
                "visual",
                "visual screenshot mismatch",
                MutationClassificationCategory::VisualMismatch,
                "visual_mismatch",
                MutationProposalBoundedMutationType::Scene,
            ),
            mapped(
                "runtime-crash",
                "runtime crash stacktrace",
                MutationClassificationCategory::RuntimeCrash,
                "runtime_crash",
                MutationProposalBoundedMutationType::Data,
            ),
            mapped(
                "console",
                "console error emitted",
                MutationClassificationCategory::ConsoleError,
                "console_error",
                MutationProposalBoundedMutationType::Data,
            ),
            mapped(
                "probe",
                "probe failure evidence",
                MutationClassificationCategory::ProbeFailure,
                "probe_failure",
                MutationProposalBoundedMutationType::Data,
            ),
        ]
    }

    fn mapped(
        name: &'static str,
        summary: &'static str,
        category: MutationClassificationCategory,
        expected_label: &'static str,
        expected_type: MutationProposalBoundedMutationType,
    ) -> MappedCase {
        MappedCase {
            name,
            failure: json!({
                "kind": "classified_failure",
                "classification": expected_label,
                "summary": summary,
                "path": "evidence/scenarios/collect-and-exit/scenario-result.json"
            }),
            evidence_path: "evidence/scenarios/collect-and-exit/scenario-result.json",
            metadata: json!({"artifact":"scenario_result", "summary": summary}),
            category,
            expected_label,
            expected_type,
        }
    }

    struct InvalidBacklogCase {
        name: &'static str,
        classification_id: &'static str,
        backlog_category: MutationClassificationCategory,
        backlog_type: MutationProposalBoundedMutationType,
        evidence_refs: Vec<&'static str>,
        expected_error: &'static str,
    }

    fn create_fixture_run(prefix: &str) -> (PathBuf, PathBuf) {
        let root = std::env::temp_dir().join(format!("ouroforge-{prefix}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, SEED).unwrap();
        let artifacts = create_run(&seed_path, root.join("runs")).unwrap();
        (root, artifacts.run_dir)
    }

    fn write_indexed_evidence(run_dir: &Path, id: &str, rel: &str, metadata: Value) {
        let path = run_dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "{}\n").unwrap();
        add_evidence_artifact(run_dir, id, "application/json", rel, metadata).unwrap();
    }

    fn write_failed_verdict(run_dir: &Path, failure: Value, evidence_refs: Vec<&str>) {
        fs::write(
            run_dir.join("verdict.json"),
            serde_json::to_vec_pretty(&json!({
                "status": "failed",
                "summary": failure.get("summary").and_then(Value::as_str).unwrap_or("fixture failed"),
                "failures": [failure],
                "evidence_refs": evidence_refs,
                "metadata": {}
            }))
            .unwrap(),
        )
        .unwrap();
    }

    fn write_backlog_item(
        run_dir: &Path,
        id: &str,
        classification_id: &str,
        category: MutationClassificationCategory,
        bounded_type: MutationProposalBoundedMutationType,
        severity: MutationBacklogSeverity,
        evidence_refs: Vec<&str>,
    ) {
        write_backlog_items(
            run_dir,
            vec![backlog_item(
                id,
                classification_id,
                category,
                bounded_type,
                severity,
                evidence_refs,
            )],
        );
    }

    fn write_backlog_items(run_dir: &Path, items: Vec<MutationBacklogItem>) {
        let artifact = MutationBacklogArtifact {
            schema_version: "1".to_string(),
            run_id: "run-selection-fixture".to_string(),
            items,
        };
        write_mutation_backlog_artifact(run_dir, &artifact).unwrap();
    }

    fn backlog_item(
        id: &str,
        classification_id: &str,
        category: MutationClassificationCategory,
        bounded_type: MutationProposalBoundedMutationType,
        severity: MutationBacklogSeverity,
        evidence_refs: Vec<&str>,
    ) -> MutationBacklogItem {
        MutationBacklogItem {
            id: id.to_string(),
            classification_id: classification_id.to_string(),
            failure_class: category,
            bounded_mutation_type: bounded_type,
            severity,
            reproduction_context: "scenario collect-and-exit reproduces locally".to_string(),
            evidence_refs: evidence_refs.into_iter().map(str::to_string).collect(),
            suggested_next_investigation: "review linked evidence before any manual mutation"
                .to_string(),
            owner_lane: "evolve-depth".to_string(),
            review_status: "open".to_string(),
            blocked_reasons: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// evolve_rerun_comparison_contract
// ---------------------------------------------------------------------------

mod evolve_rerun_comparison_contract {
    use ouroforge_core::{
        add_evidence_artifact, compare_runs, create_run, evolve_run, update_journal,
        write_run_comparison_artifact,
    };
    use serde_json::{json, Value};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    const SEED: &str = r#"
id: evolve.rerun.comparison.contract
title: Evolve Rerun Comparison Contract
goal: Prove four-gate before/after deltas and journal mutation-loop summaries.
constraints:
  target: local-fixture
acceptance:
  - Rerun comparisons link gate deltas to evidence and proposal context.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed rerun comparison scenario.
"#;

    #[test]
    fn rerun_comparison_records_four_gate_deltas_and_journal_summary() {
        let (root, before_run) = create_fixture_run("evolve-rerun-before");
        let after_root = root.join("after-root");
        fs::create_dir_all(&after_root).unwrap();
        let (_, after_run) = create_fixture_run_under(&after_root, "evolve-rerun-after");

        write_scenario_result(&before_run, "failed");
        write_gate_evidence(&before_run, "visual", "visual-before", "fail");
        write_gate_evidence(&before_run, "semantic", "semantic-before", "fail");
        write_failed_verdict(&before_run);
        let summary = evolve_run(&before_run).expect("failed run creates mutation proposal");
        assert_eq!(summary.status, "proposed");

        write_scenario_result(&after_run, "passed");
        write_gate_evidence(&after_run, "visual", "visual-after", "pass");
        write_gate_evidence(&after_run, "semantic", "semantic-after", "pass");
        write_passed_verdict(&after_run);

        let comparison = compare_runs(&before_run, &after_run).expect("comparison");
        assert_eq!(comparison.classification, "improved");
        assert_eq!(comparison.comparability.state, "comparable");
        assert_gate_delta(
            &comparison.four_gate_deltas,
            "visual",
            "fail",
            "pass",
            "fail_to_pass",
            "visual-comparison.json",
        );
        assert_gate_delta(
            &comparison.four_gate_deltas,
            "semantic",
            "fail",
            "pass",
            "fail_to_pass",
            "runtime-invariant-model.json",
        );
        assert_gate_delta(
            &comparison.four_gate_deltas,
            "mechanical",
            "fail",
            "pass",
            "fail_to_pass",
            "scenario-result.json",
        );

        let comparison_path =
            write_run_comparison_artifact(&before_run, &after_run, before_run.join("mutation"))
                .expect("comparison writes");
        let comparison_json: Value =
            serde_json::from_str(&fs::read_to_string(&comparison_path).unwrap())
                .expect("comparison json parses");
        assert_eq!(
            comparison_json["fourGateDeltas"].as_array().unwrap().len(),
            4
        );
        assert_eq!(comparison_json["comparability"]["state"], "comparable");

        let journal = update_journal(&before_run).expect("journal updates");
        assert!(journal.contains("Next-step hypothesis"));
        assert!(journal.contains("Evidence-linked gate: `visual`"));
        assert!(journal.contains("proposal `mutation-1`"));
        assert!(journal.contains("rerun delta `visual`: `fail_to_pass`"));
        assert!(journal.contains("rerun delta `semantic`: `fail_to_pass`"));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn rerun_comparison_fails_for_missing_required_after_evidence() {
        let (root, before_run) = create_fixture_run("evolve-rerun-missing-before");
        let after_root = root.join("after-root");
        fs::create_dir_all(&after_root).unwrap();
        let (_, after_run) = create_fixture_run_under(&after_root, "evolve-rerun-missing-after");
        write_scenario_result(&before_run, "failed");
        write_failed_verdict(&before_run);
        fs::remove_file(after_run.join("evidence/index.json")).expect("remove required evidence index");

        let error = write_run_comparison_artifact(&before_run, &after_run, before_run.join("mutation"))
            .expect_err("missing after evidence blocks comparison");

        assert!(error
            .to_string()
            .contains("after run is missing required artifact"));
        fs::remove_dir_all(root).ok();
    }

    fn assert_gate_delta(
        deltas: &[ouroforge_core::RunGateDelta],
        gate: &str,
        before: &str,
        after: &str,
        transition: &str,
        evidence_substring: &str,
    ) {
        let delta = deltas
            .iter()
            .find(|delta| delta.gate == gate)
            .unwrap_or_else(|| panic!("missing gate {gate}"));
        assert_eq!(delta.before_status, before);
        assert_eq!(delta.after_status, after);
        assert_eq!(delta.transition, transition);
        assert!(delta
            .before_evidence_refs
            .iter()
            .chain(delta.after_evidence_refs.iter())
            .any(|value| value.contains(evidence_substring)));
    }

    fn create_fixture_run(prefix: &str) -> (PathBuf, PathBuf) {
        let root = std::env::temp_dir().join(format!("ouroforge-{prefix}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        create_fixture_run_under(&root, prefix)
    }

    fn create_fixture_run_under(root: &Path, _prefix: &str) -> (PathBuf, PathBuf) {
        fs::create_dir_all(root).unwrap();
        let seed_path = root.join("seed.yaml");
        fs::write(&seed_path, SEED).unwrap();
        let artifacts = create_run(&seed_path, root.join("runs")).unwrap();
        (root.to_path_buf(), artifacts.run_dir)
    }

    fn write_scenario_result(run_dir: &Path, status: &str) {
        let rel = "evidence/scenarios/collect-and-exit/scenario-result.json";
        fs::create_dir_all(run_dir.join("evidence/scenarios/collect-and-exit")).unwrap();
        fs::write(
            run_dir.join(rel),
            serde_json::to_vec_pretty(&json!({
                "scenario_id": "collect-and-exit",
                "status": status,
                "assertions": [{ "kind": "fixture", "passed": status == "passed" }],
                "evidence": {}
            }))
            .unwrap(),
        )
        .unwrap();
        add_evidence_artifact(
            run_dir,
            "scenario-result",
            "application/json",
            rel,
            json!({"artifact":"scenario_result"}),
        )
        .ok();
    }

    fn write_gate_evidence(run_dir: &Path, gate: &str, id: &str, state: &str) -> String {
        let file_name = if gate == "visual" {
            "visual-comparison.json"
        } else {
            "runtime-invariant-model.json"
        };
        let rel = format!("evidence/scenarios/collect-and-exit/{gate}/{id}/{file_name}");
        fs::create_dir_all(run_dir.join(&rel).parent().unwrap()).unwrap();
        fs::write(
            run_dir.join(&rel),
            serde_json::to_vec_pretty(&json!({
                "id": id,
                "gate": gate,
                "state": state,
                "artifact": if gate == "visual" { "visual_comparison_evidence" } else { "runtime_invariant_model" }
            }))
            .unwrap(),
        )
        .unwrap();
        add_evidence_artifact(
            run_dir,
            id,
            "application/json",
            &rel,
            json!({"artifact": if gate == "visual" { "visual_comparison_evidence" } else { "runtime_invariant_model" }, "gate": gate}),
        )
        .ok();
        rel
    }

    fn write_failed_verdict(run_dir: &Path) {
        let visual = "evidence/scenarios/collect-and-exit/visual/visual-before/visual-comparison.json";
        let semantic =
            "evidence/scenarios/collect-and-exit/semantic/semantic-before/runtime-invariant-model.json";
        write_verdict(run_dir, "failed", "fail", "fail", visual, semantic);
    }

    fn write_passed_verdict(run_dir: &Path) {
        let visual = "evidence/scenarios/collect-and-exit/visual/visual-after/visual-comparison.json";
        let semantic =
            "evidence/scenarios/collect-and-exit/semantic/semantic-after/runtime-invariant-model.json";
        write_verdict(run_dir, "passed", "pass", "pass", visual, semantic);
    }

    fn write_verdict(
        run_dir: &Path,
        status: &str,
        visual_state: &str,
        semantic_state: &str,
        visual_ref: &str,
        semantic_ref: &str,
    ) {
        let scenario = "evidence/scenarios/collect-and-exit/scenario-result.json";
        fs::write(
            run_dir.join("verdict.json"),
            serde_json::to_vec_pretty(&json!({
                "status": status,
                "summary": "fixture four-gate verdict",
                "failures": if status == "failed" { vec![json!({"kind":"visual_gate_failed", "path": visual_ref})] } else { Vec::new() },
                "evidence_refs": [scenario, visual_ref, semantic_ref],
                "gateCategories": {
                    "aggregation": { "operator": "declared-gate-and", "undeclaredGatePolicy": "neutral" },
                    "mechanical": { "declared": true, "status": if status == "passed" { "pass" } else { "fail" } },
                    "runtime": { "declared": false, "status": "pass" },
                    "visual": { "declared": true, "status": visual_state },
                    "semantic": { "declared": true, "status": semantic_state }
                },
                "visual": [{ "state": visual_state, "comparison_ref": visual_ref, "reason": "fixture visual gate" }],
                "semantic": [{ "state": semantic_state, "model_ref": semantic_ref, "evidence_refs": [scenario], "invariant_id": "health-non-negative" }],
                "metadata": {}
            }))
            .unwrap(),
        )
        .unwrap();
    }
}

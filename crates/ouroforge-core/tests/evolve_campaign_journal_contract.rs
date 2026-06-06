use ouroforge_core::{
    build_evolve_campaign_journal, render_evolve_campaign_journal_markdown, EvolveCampaignArtifact,
    EvolveCampaignOutcomeState, EvolveCampaignStopReason, EVOLVE_CAMPAIGN_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> EvolveCampaignArtifact {
    let path = workspace_path(&format!("examples/evolve-campaign-v1/journal/{name}"));
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

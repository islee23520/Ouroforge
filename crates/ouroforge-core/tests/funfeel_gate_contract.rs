//! Contract test for Fun-Feel Evaluation Gate v1 (#1859).
//!
//! The gate is a human-in-the-loop release-readiness precondition. It records
//! who approved the scoped evidence and never computes or accepts an automated
//! fun score.

use std::path::PathBuf;

use ouroforge_core::funfeel_gate::{
    FunFeelGateInput, FunFeelReadiness, FUNFEEL_GATE_BOUNDARY, FUNFEEL_GATE_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/playtest-funfeel-gate-v1")
            .join(name),
    )
    .unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn fixture(name: &str) -> FunFeelGateInput {
    FunFeelGateInput::from_json_str(&fixture_text(name)).expect("fixture parses")
}

#[test]
fn no_human_verdict_blocks_release_readiness() {
    let gate = fixture("funfeel-gate-no-verdict-v1.json");
    let decision = gate.evaluate();

    assert_eq!(gate.schema_version, FUNFEEL_GATE_SCHEMA_VERSION);
    assert_eq!(gate.boundary, FUNFEEL_GATE_BOUNDARY);
    assert_eq!(decision.readiness, FunFeelReadiness::NeedsHumanReview);
    assert_eq!(decision.readiness.as_str(), "needs-human-review");
    assert!(!decision.release_ready);
    assert!(decision.reason.contains("missing human fun/feel verdict"));
    assert!(decision.decided_by.is_none());
}

#[test]
fn recorded_human_verdict_passes_release_readiness_precondition() {
    let gate = fixture("funfeel-gate-recorded-verdict-v1.json");
    let decision = gate.evaluate();

    assert_eq!(decision.readiness, FunFeelReadiness::ApprovedByHuman);
    assert_eq!(decision.readiness.as_str(), "approved-by-human");
    assert!(decision.release_ready);
    assert_eq!(
        decision.decided_by.as_deref(),
        Some("human-reviewer-local-001")
    );
    assert!(decision.reason.contains("human fun/feel verdict approved"));
    assert!(gate.browser_studio_mode.contains("read-only"));
    assert!(!gate.trusted_write_requested);
    assert!(!gate.release_button_requested);
}

#[test]
fn automated_metric_or_non_human_actor_never_auto_scores_fun() {
    let gate = fixture("funfeel-gate-no-auto-score-v1.json");
    let decision = gate.evaluate();

    assert_eq!(decision.readiness, FunFeelReadiness::Blocked);
    assert!(!decision.release_ready);
    assert!(
        decision.reason.contains("humanConfirmed must be true")
            || decision.reason.contains("actor role")
            || decision
                .reason
                .contains("automated metrics cannot decide fun"),
        "unexpected reason: {}",
        decision.reason
    );
}

#[test]
fn unknown_automated_fun_score_field_is_rejected_by_schema() {
    let mut value: serde_json::Value =
        serde_json::from_str(&fixture_text("funfeel-gate-recorded-verdict-v1.json"))
            .expect("json parses");
    value["automatedFunScore"] = serde_json::json!(0.99);
    let text = serde_json::to_string(&value).expect("json serializes");
    let error = FunFeelGateInput::from_json_str(&text)
        .expect_err("unknown automated fun score field must be rejected");
    assert!(error.to_string().contains("automatedFunScore"));
}

#[test]
fn stale_candidate_verdict_blocks_release_readiness() {
    let mut gate = fixture("funfeel-gate-recorded-verdict-v1.json");
    gate.candidate_refs = vec!["examples/other-candidate.json#v2".to_string()];

    let decision = gate.evaluate();
    assert_eq!(decision.readiness, FunFeelReadiness::Blocked);
    assert!(!decision.release_ready);
    assert!(decision.reason.contains("candidateRefs"));
}

#[test]
fn scope_doc_keeps_gate_human_owned_and_read_only() {
    let doc = std::fs::read_to_string(repo_root().join("docs/playtest-funfeel-gate-v1.md"))
        .expect("scope doc exists");
    assert!(doc.contains("Fun-feel evaluation gate contract"));
    assert!(doc.contains("Human sign-off required"));
    assert!(doc.contains("Release-readiness is blocked"));
    assert!(doc.contains("never replace it"));
    assert!(doc.contains("read-only"));
}

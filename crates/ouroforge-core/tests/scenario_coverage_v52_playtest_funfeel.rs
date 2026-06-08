//! Scenario Coverage v52: Playtest and Fun-Feel Gate Regression Suite (#1861).
//!
//! State/shape-only coverage for #1858/#1859/#1860 plus existing evaluator gate
//! aggregation back-compat. Local deterministic fixtures only: no network, live
//! browser, timing, trusted writes, auto-apply, auto-merge, self-approval, or
//! automated fun/release claim.

use std::path::{Path, PathBuf};

use ouroforge_core::funfeel_gate::{FunFeelGateInput, FunFeelReadiness};
use ouroforge_core::playtest_capture::PlaytestSessionCapture;
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

fn read_capture(relative: &str) -> PlaytestSessionCapture {
    PlaytestSessionCapture::from_json_str(&read_text(relative)).expect(relative)
}

fn read_gate(relative: &str) -> FunFeelGateInput {
    FunFeelGateInput::from_json_str(&read_text(relative)).expect(relative)
}

#[test]
fn v52_matrix_enumerates_required_rows_and_boundaries() {
    let matrix =
        read_json("examples/playtest-funfeel-v1/scenario-coverage-v52/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v52.playtest-funfeel.v1"
    );
    assert_eq!(matrix["issue"], "1861");
    let boundary = matrix["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local",
        "browser/Studio surfaces are read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "state/shape checks only",
        "no automated fun score",
        "no auto-apply",
        "no auto-merge",
        "no self-approval",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(phrase), "missing boundary {phrase}");
    }
    let rows = matrix["rows"].as_array().unwrap();
    let ids = rows
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "V52.capture.shape",
            "V52.gate.no_verdict_blocks",
            "V52.gate.recorded_verdict_unblocks",
            "V52.gate.no_auto_score",
            "V52.demo.smoke",
            "V52.evaluator_aggregation.backcompat",
        ]
    );
    for row in rows {
        let fixture = row["fixture"].as_str().unwrap();
        assert!(
            repo_root().join(fixture).exists(),
            "missing fixture {fixture}"
        );
    }
}

#[test]
fn v52_capture_and_human_gate_states_are_locked() {
    let capture = read_capture("examples/playtest-funfeel-v1/demo/playtest-session-demo-v1.json");
    capture.validate().expect("capture validates");
    assert_eq!(capture.capture_id, "playtest-funfeel-demo-capture-v1");
    assert_eq!(capture.signals.one_more_run, "yes");
    assert!(capture.actor.human_confirmed);
    assert!(!capture.trusted_write_requested);
    assert!(!capture.release_authority);
    assert!(capture.boundary.contains("evidence-only"));

    let no_verdict =
        read_gate("examples/playtest-funfeel-v1/demo/funfeel-gate-no-verdict-demo-v1.json");
    let blocked = no_verdict.evaluate();
    assert_eq!(blocked.readiness, FunFeelReadiness::NeedsHumanReview);
    assert_eq!(blocked.readiness.as_str(), "needs-human-review");
    assert!(!blocked.release_ready);
    assert!(blocked.reason.contains("missing human fun/feel verdict"));

    let recorded =
        read_gate("examples/playtest-funfeel-v1/demo/funfeel-gate-recorded-verdict-demo-v1.json");
    let approved = recorded.evaluate();
    assert_eq!(approved.readiness, FunFeelReadiness::ApprovedByHuman);
    assert_eq!(approved.readiness.as_str(), "approved-by-human");
    assert!(approved.release_ready);
    assert_eq!(
        approved.decided_by.as_deref(),
        Some("human-reviewer-demo-001")
    );
}

#[test]
fn v52_no_auto_score_and_demo_sequence_are_locked() {
    let auto_score_attempt =
        read_gate("examples/playtest-funfeel-gate-v1/funfeel-gate-no-auto-score-v1.json");
    let decision = auto_score_attempt.evaluate();
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

    let manifest =
        read_json("examples/playtest-funfeel-v1/demo/playtest-funfeel-demo-manifest-v1.json");
    assert_eq!(
        manifest["expectedReadinessSequence"],
        serde_json::json!(["capture-valid", "needs-human-review", "approved-by-human"])
    );
    let joined = [
        read_text("examples/playtest-funfeel-v1/demo/playtest-session-demo-v1.json"),
        read_text("examples/playtest-funfeel-v1/demo/funfeel-gate-no-verdict-demo-v1.json"),
        read_text("examples/playtest-funfeel-v1/demo/funfeel-gate-recorded-verdict-demo-v1.json"),
        read_text("examples/playtest-funfeel-v1/demo/playtest-funfeel-demo-manifest-v1.json"),
    ]
    .join("\n");
    assert!(!joined.contains("automatedFunScore"));
    assert!(!joined.contains("funScore"));
}

#[test]
fn v52_existing_evaluator_aggregation_backcompat_is_preserved() {
    let evaluator_contract =
        read_text("crates/ouroforge-evaluator/tests/verdict_schema_contract.rs");
    for required in [
        "verdict_schema_preserves_existing_gate_field_names",
        "evaluator_crate_owns_top_level_run_verdict_writer",
        "gateCategories",
        "visual",
        "semantic",
        "scenarioId",
        "checkpointId",
        "modelId",
        "invariantId",
    ] {
        assert!(
            evaluator_contract.contains(required),
            "missing evaluator backcompat token {required}"
        );
    }
}

#[test]
fn v52_doc_records_conservative_state_shape_scope() {
    let doc = read_text("docs/scenario-coverage-v52.md");
    for required in [
        "state/shape checks only",
        "backward-compatibility golden",
        "existing evaluator gate aggregation",
        "Generated runs/artifacts remain",
        "Issues #1 and\n#23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v52_playtest_funfeel",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

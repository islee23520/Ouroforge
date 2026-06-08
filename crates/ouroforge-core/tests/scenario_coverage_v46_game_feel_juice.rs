//! Scenario Coverage v46: Game-Feel and Juice Regression Suite (#1823).
//!
//! State/shape-only coverage for #1819/#1820/#1821/#1822 plus existing runtime
//! feedback back-compat. Local deterministic fixtures only: no network, live
//! browser, timing, trusted writes, auto-apply, auto-merge, or fun claim.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    score_cascade_feedback_trace, verify_responsiveness, CardRogueliteConfig,
    ResponsivenessEvidence, ResponsivenessStatus,
};
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

#[test]
fn v46_matrix_enumerates_required_rows_and_boundaries() {
    let matrix = read_json("examples/game-feel-juice-v1/scenario-coverage-v46/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v46.game-feel-juice.v1"
    );
    assert_eq!(matrix["issue"], "1823");
    let boundary = matrix["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local",
        "browser/Studio read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "no timing flakes",
        "no auto-apply",
        "no auto-merge",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(phrase), "missing boundary {phrase}");
    }
    let ids = matrix["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "V46.juice.primitives",
            "V46.cascade.order",
            "V46.responsiveness.pass",
            "V46.responsiveness.fail",
            "V46.demo.smoke",
            "V46.runtime.backcompat",
        ]
    );
}

#[test]
fn v46_juice_cascade_and_responsiveness_states_are_locked() {
    let juice = read_json("examples/game-runtime/juice-scene-v1.json");
    let kinds = juice["juice"]["primitives"]
        .as_array()
        .unwrap()
        .iter()
        .map(|primitive| primitive["kind"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(kinds, vec!["tween", "shake", "hit-stop", "sfx"]);
    assert_eq!(juice["metadata"]["scenarioId"], "juice-primitives-v1");
    assert!(juice["metadata"]["boundary"]
        .as_str()
        .unwrap()
        .contains("human Era J owns feel/fun judgment"));

    let cascade_fixture = read_json("examples/game-runtime/score-cascade-feedback-v1.json");
    let config: CardRogueliteConfig =
        serde_json::from_value(cascade_fixture["config"].clone()).expect("cascade config parses");
    let trace = score_cascade_feedback_trace(&config).expect("cascade trace recomputes");
    let phases = trace
        .events
        .iter()
        .map(|event| event.phase.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        phases,
        vec![
            "base",
            "modifier",
            "modifier",
            "card-total",
            "base",
            "modifier",
            "card-total",
            "cascade-complete",
        ]
    );
    assert_eq!(trace.final_score, 24);
    assert!(trace
        .events
        .iter()
        .all(|event| event.juice_trigger == "score_cascade" && event.read_only_evidence));

    let responsiveness = read_json("examples/game-runtime/responsiveness-v1.json");
    let results = responsiveness["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let evidence: ResponsivenessEvidence =
                serde_json::from_value(case.clone()).expect("responsiveness case parses");
            let report = verify_responsiveness(&evidence).expect("responsiveness report");
            (
                report.scenario_id,
                match report.status {
                    ResponsivenessStatus::Pass => "pass",
                    ResponsivenessStatus::Fail => "fail",
                },
                report.max_latency_ms,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        results,
        vec![
            ("responsiveness-within-budget".to_string(), "pass", 80),
            ("responsiveness-over-budget".to_string(), "fail", 112),
        ]
    );
}

#[test]
fn v46_demo_and_runtime_backcompat_golden_remain_valid() {
    let demo = read_json("examples/game-feel-juice-v1/demo/game-feel-juice-demo-v1.json");
    assert_eq!(demo["schemaVersion"], "ouroforge.game-feel-juice-demo.v1");
    assert_eq!(demo["expected"]["finalScore"], 24);
    assert_eq!(
        demo["expected"]["responsivenessStatuses"],
        serde_json::json!(["pass", "fail"])
    );
    assert!(demo["boundary"]
        .as_str()
        .unwrap()
        .contains("not browser trusted authority"));
    assert_eq!(demo["governanceAudit"]["issue1MustRemainOpen"], true);
    assert_eq!(demo["governanceAudit"]["issue23MustRemainOpen"], true);

    let golden = read_json(
        "examples/game-feel-juice-v1/scenario-coverage-v46/runtime-feedback-backcompat-golden.json",
    );
    let index = read_text("examples/game-runtime/index.html");
    let runtime = read_text("examples/game-runtime/runtime.js");
    let juice_js = read_text("examples/game-runtime/juice.js");
    assert!(index.contains(golden["expected"]["runtimeIndexScript"].as_str().unwrap()));
    assert!(runtime.contains(golden["expected"]["runtimeFeedbackEvent"].as_str().unwrap()));
    assert!(runtime.contains(
        golden["expected"]["runtimeFeedbackUpdateEvent"]
            .as_str()
            .unwrap()
    ));
    assert!(runtime.contains(golden["expected"]["worldStateField"].as_str().unwrap()));
    assert!(runtime.contains(golden["expected"]["probeNamespace"].as_str().unwrap()));
    assert!(juice_js.contains(golden["expected"]["juiceSchemaVersion"].as_str().unwrap()));
    for kind in golden["expected"]["requiredPrimitiveKinds"]
        .as_array()
        .unwrap()
    {
        assert!(
            juice_js.contains(kind.as_str().unwrap()),
            "missing primitive kind {kind}"
        );
    }
}

#[test]
fn v46_doc_records_conservative_state_shape_scope() {
    let doc = read_text("docs/scenario-coverage-v46.md");
    for required in [
        "state/shape checks only",
        "existing runtime feedback backward-compatibility golden",
        "Generated runs/artifacts remain untracked unless fixture-scoped",
        "Issues #1 and #23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v46_game_feel_juice",
        "feel/fun judgments",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

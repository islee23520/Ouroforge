//! Scenario Coverage v47: Deckbuilder UI Regression Suite (#1830).
//!
//! State/shape-only coverage for #1826/#1827/#1828/#1829 plus existing runtime
//! UI/probe back-compat. Local deterministic fixtures only: no network, live
//! browser, timing, trusted writes, auto-apply, auto-merge, self-approval, or
//! automated fun/release claim.

use std::path::{Path, PathBuf};

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
fn v47_matrix_enumerates_required_rows_and_boundaries() {
    let matrix = read_json("examples/deckbuilder-ui-v1/scenario-coverage-v47/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v47.deckbuilder-ui.v1"
    );
    assert_eq!(matrix["issue"], "1830");
    let boundary = matrix["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local",
        "browser/Studio read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "no timing flakes",
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
            "V47.hand.pipeline",
            "V47.shop.run_map",
            "V47.score.cascade_display",
            "V47.demo.smoke",
            "V47.runtime.backcompat",
            "V47.boundary.negative_shapes",
        ]
    );
    for row in rows {
        assert!(
            row["expectedShape"].as_str().unwrap().len() > 40,
            "row records a concrete expected shape: {row}"
        );
    }
}

#[test]
fn v47_hand_pipeline_shop_and_run_map_shapes_are_locked() {
    let scene = read_json("examples/game-runtime/deckbuilder-ui-scene-v1.json");
    assert_eq!(
        scene["deckbuilderUi"]["schemaVersion"],
        "ouroforge.deckbuilder-ui.v1"
    );
    assert_eq!(
        scene["deckRoguelike"]["schemaVersion"],
        "ouroforge.deck-roguelike.v1"
    );

    let deck = scene["deckRoguelike"]["deck"].as_array().unwrap();
    let cards = scene["deckRoguelike"]["cards"].as_object().unwrap();
    assert!(!deck.is_empty());
    assert!(!cards.is_empty());
    for card in deck {
        assert!(cards.contains_key(card.as_str().unwrap()));
    }

    let slots = scene["deckbuilderUi"]["pipelineSlots"].as_array().unwrap();
    assert_eq!(
        slots
            .iter()
            .map(|slot| slot["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["intent", "modifier", "commit"]
    );

    let shop = &scene["deckbuilderUi"]["shop"];
    assert_eq!(shop["id"], "act1-shop");
    assert_eq!(shop["currency"], "gold");
    assert_eq!(shop["balance"], 45);
    let offers = shop["offers"].as_array().unwrap();
    assert_eq!(
        offers
            .iter()
            .map(|offer| offer["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["offer-strike-plus", "offer-vigor-charm"]
    );
    assert_eq!(offers[0]["available"], true);
    assert_eq!(offers[1]["available"], false);
    assert_eq!(offers[1]["unavailableReason"], "insufficient gold");

    let run_map = &scene["deckbuilderUi"]["runMap"];
    assert_eq!(run_map["currentNodeId"], "start");
    let nodes = run_map["nodes"].as_array().unwrap();
    assert_eq!(
        nodes
            .iter()
            .map(|node| node["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["start", "shop", "elite", "boss"]
    );
    assert_eq!(nodes[2]["status"], "blocked");
    let edges = run_map["edges"].as_array().unwrap();
    assert!(edges
        .iter()
        .any(|edge| edge["from"] == "start" && edge["to"] == "shop"));
    assert!(edges
        .iter()
        .any(|edge| edge["from"] == "start" && edge["to"] == "elite" && edge["blocked"] == true));
}

#[test]
fn v47_score_display_and_demo_expected_state_shapes_are_locked() {
    let scene = read_json("examples/game-runtime/deckbuilder-ui-scene-v1.json");
    let score_display = &scene["deckbuilderUi"]["scoreDisplay"];
    assert_eq!(score_display["id"], "score-display-v1");
    assert_eq!(
        score_display["sourceSchemaVersion"],
        "ouroforge.score-cascade-feedback.v1"
    );
    assert_eq!(score_display["finalScore"], 24);
    assert_eq!(score_display["authoritativeScore"], 24);
    let events = score_display["events"].as_array().unwrap();
    assert_eq!(
        events
            .iter()
            .map(|event| event["phase"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec![
            "base",
            "modifier",
            "modifier",
            "card-total",
            "cascade-complete"
        ]
    );
    assert_eq!(
        events
            .iter()
            .map(|event| event["stepIndex"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 4]
    );
    assert!(events.iter().all(|event| event["readOnlyEvidence"] == true));

    let manifest = read_json("examples/deckbuilder-ui-v1/demo/demo-manifest.json");
    assert_eq!(
        manifest["schemaVersion"],
        "ouroforge.deckbuilder-ui-demo.v1"
    );
    assert_eq!(manifest["issue"], 1829);
    assert_eq!(manifest["determinism"]["network"], "disabled");
    assert_eq!(manifest["determinism"]["liveBrowser"], "not required");
    assert_eq!(
        manifest["expected"]["schemaVersion"],
        "ouroforge.deckbuilder-ui-state.v1"
    );
    assert_eq!(manifest["expected"]["formattedFinalScore"], "24");
    assert_eq!(manifest["governance"]["anchors"], "#1 and #23 remain open");
}

#[test]
fn v47_runtime_ui_probe_backcompat_golden_remains_valid() {
    let golden = read_json(
        "examples/deckbuilder-ui-v1/scenario-coverage-v47/runtime-ui-probe-backcompat-golden.json",
    );
    let index = read_text("examples/game-runtime/index.html");
    let runtime = read_text("examples/game-runtime/runtime.js");
    let module = read_text("examples/game-runtime/deckbuilder-ui.js");
    let runtime_test = read_text("examples/game-runtime/deckbuilder-ui.test.cjs");
    let docs = read_text("docs/deckbuilder-ui-v1.md");
    let combined = format!("{runtime}\n{module}\n{runtime_test}\n{docs}");

    assert!(index.contains(golden["expected"]["runtimeIndexScript"].as_str().unwrap()));
    assert!(runtime.contains(golden["expected"]["probeNamespace"].as_str().unwrap()));
    assert!(module.contains(golden["expected"]["stateSchemaVersion"].as_str().unwrap()));
    assert!(module.contains(golden["expected"]["renderSchemaVersion"].as_str().unwrap()));
    assert!(module.contains(golden["expected"]["moduleGlobal"].as_str().unwrap()));

    for method in golden["expected"]["requiredProbeMethods"]
        .as_array()
        .unwrap()
    {
        assert!(
            combined.contains(method.as_str().unwrap()),
            "missing method {method}"
        );
    }
    for event in golden["expected"]["requiredRuntimeEvents"]
        .as_array()
        .unwrap()
    {
        assert!(
            combined.contains(event.as_str().unwrap()),
            "missing runtime event {event}"
        );
    }
    for signal in golden["expected"]["draftOnlySignals"].as_array().unwrap() {
        assert!(
            combined.contains(signal.as_str().unwrap()),
            "missing signal {signal}"
        );
    }
    for action in golden["expected"]["disallowedActions"].as_array().unwrap() {
        assert!(
            combined
                .to_ascii_lowercase()
                .contains(action.as_str().unwrap()),
            "missing disallowed action {action}"
        );
    }
}

#[test]
fn v47_docs_record_conservative_state_shape_scope() {
    let doc = read_text("docs/scenario-coverage-v47.md");
    for required in [
        "state/shape checks only",
        "hand/pipeline UI",
        "shop/run-map UI",
        "score-cascade display",
        "runtime UI/probe backward-compatibility golden",
        "Generated runs/artifacts remain untracked unless fixture-scoped",
        "Issues #1 and #23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v47_deckbuilder_ui",
        "feel/fun judgments",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

//! Scenario Coverage v45: Engine-Builder Balance Regression Suite (#1816).
//!
//! State/shape-only coverage for #1812/#1813/#1814/#1815 plus Milestone 32
//! synthetic balance back-compat. Local deterministic fixtures only: no network,
//! live browser, timing, trusted writes, auto-apply, auto-merge, or fun claim.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_combo_detector::{detect_degenerate_builds, ComboTelemetryFixture};
use ouroforge_core::balance_dominant_build::{
    analyze_dominant_builds, DominantBuildTelemetryFixture,
};
use ouroforge_core::balance_fairness::{verify_fairness, FairnessFixture, SeedVerdict};
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

fn read_typed<T: serde::de::DeserializeOwned>(relative: &str) -> T {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

#[test]
fn v45_matrix_enumerates_required_rows_and_boundaries() {
    let matrix =
        read_json("examples/engine-builder-balance-v1/scenario-coverage-v45/matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "ouroforge.scenario-coverage.v45.engine-builder-balance.v1"
    );
    assert_eq!(matrix["issue"], "1816");
    let boundary = matrix["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local",
        "browser/Studio read-only",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "no auto-apply",
        "no auto-merge",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(phrase), "missing boundary {phrase}");
    }
    let rows = matrix["rows"].as_array().unwrap();
    let ids = rows
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "V45.combo.detect",
            "V45.combo.balanced",
            "V45.dominant.detect",
            "V45.dominant.balanced",
            "V45.fairness.unfair",
            "V45.demo.smoke",
            "V45.m32.backcompat",
        ]
    );
}

#[test]
fn v45_combo_dominant_fairness_and_demo_states_are_locked() {
    let combo: ComboTelemetryFixture = read_typed(
        "examples/engine-builder-balance-v1/combo-detector/degenerate-combo.fixture.json",
    );
    let combo_report = detect_degenerate_builds(&combo).unwrap();
    assert!(combo_report
        .findings
        .iter()
        .any(|f| f.cards == ["overcharger", "reactor-loop"] && f.replay_deck_seed == 310));

    let balanced_combo: ComboTelemetryFixture =
        read_typed("examples/engine-builder-balance-v1/combo-detector/balanced-build.fixture.json");
    assert!(detect_degenerate_builds(&balanced_combo)
        .unwrap()
        .findings
        .is_empty());

    let dominant: DominantBuildTelemetryFixture =
        read_typed("examples/engine-builder-balance-v1/dominant-build/dominant-build.fixture.json");
    let dominant_report = analyze_dominant_builds(&dominant).unwrap();
    assert!(dominant_report
        .dominant_builds
        .iter()
        .any(|b| b.build_id == "loop-engine" && b.replay_deck_seed == 510));
    assert!(dominant_report
        .dead_modifiers
        .iter()
        .any(|m| m.modifier == "rusty-bearing"));

    let balanced_dominant: DominantBuildTelemetryFixture = read_typed(
        "examples/engine-builder-balance-v1/dominant-build/balanced-builds.fixture.json",
    );
    let balanced_report = analyze_dominant_builds(&balanced_dominant).unwrap();
    assert!(balanced_report.dominant_builds.is_empty());
    assert!(balanced_report.dead_modifiers.is_empty());

    let fairness: FairnessFixture =
        read_typed("examples/engine-builder-balance-v1/fairness/fairness.fixture.json");
    let fairness_report = verify_fairness(&fairness).unwrap();
    assert_eq!(fairness_report.daily_seed_failures, vec![7202]);
    assert_eq!(
        fairness_report
            .results
            .iter()
            .find(|r| r.seed == 7201)
            .unwrap()
            .verdict,
        SeedVerdict::Pass
    );
    assert_eq!(
        fairness_report
            .results
            .iter()
            .find(|r| r.seed == 7202)
            .unwrap()
            .verdict,
        SeedVerdict::Unfair
    );

    let demo = read_json("examples/engine-builder-balance-v1/demo/demo-manifest.json");
    assert_eq!(
        demo["schemaVersion"],
        "ouroforge.engine-builder-balance-demo.v1"
    );
    assert_eq!(demo["expected"]["unfairSeed"], 7202);
}

#[test]
fn v45_preserves_milestone_32_synthetic_balance_report_shape() {
    let report = read_json("examples/game-runtime/balance-telemetry-report-v1.json");
    assert_eq!(report["schemaVersion"], "ouroforge.balance-report.v1");
    assert!(report["degenerateCombos"]
        .as_array()
        .unwrap()
        .iter()
        .any(|combo| {
            combo["cards"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c == "smite")
                && combo["replay"]["deckSeed"].as_u64().is_some()
        }));
    assert!(report["deadItems"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["card"] == "brick"));
    assert!(report["digest"]
        .as_str()
        .unwrap()
        .starts_with("report|scene="));
}

#[test]
fn v45_doc_records_conservative_state_shape_scope() {
    let doc = read_text("docs/scenario-coverage-v45.md");
    for required in [
        "state/shape checks only",
        "Milestone 32 synthetic-balance backward-compatibility",
        "Generated runs/artifacts remain untracked unless",
        "Issues #1 and #23 remain open",
        "cargo test -p ouroforge-core --test scenario_coverage_v45_engine_builder_balance",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

//! Contract test for the Difficulty Metric Artifact v1 (#1582).
//!
//! Exercises `ouroforge_core::puzzle_difficulty_metric` over the shared
//! `ouroforge.grid-puzzle.v1` fixtures and committed solver/detector evidence.
//! It machine-checks that the metrics are *computed* from evidence (not
//! asserted) on a known level, and that stale or missing evidence fails closed.
//! The metric reuses the #1580 solver search and #1581 intent; no new analysis
//! engine.

use std::path::{Path, PathBuf};

use ouroforge_core::puzzle_difficulty_metric::{self, EVIDENCE_SCHEMA};
use ouroforge_core::puzzle_solver::SolveBudget;
use serde_json::{json, Value};

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

const SCENE: &str = "examples/game-runtime/grid-puzzle-scene-v1.json";
const EVIDENCE: &str = "examples/game-runtime/grid-puzzle-difficulty-evidence-v1.json";
const STALE_EVIDENCE: &str = "examples/game-runtime/grid-puzzle-difficulty-evidence-stale-v1.json";

fn scene_spec() -> Value {
    read_json(SCENE)["gridPuzzle"].clone()
}

#[test]
fn difficulty_metric_is_computed_from_evidence_on_a_known_level() {
    let spec = scene_spec();
    let evidence = read_json(EVIDENCE);

    let metric =
        puzzle_difficulty_metric::compute_difficulty(&spec, &evidence, SolveBudget::default())
            .expect("valid spec + evidence");

    // Solution length comes from the recorded solver witness.
    assert_eq!(metric.solution_length, 4);
    // Mechanics introduced along the intended path, in first-use order.
    assert_eq!(metric.mechanic_introduction_order, vec!["push", "move"]);
    // Descriptive measurements over the reachable state space.
    assert!(
        metric.reachable_states > 0,
        "reachable space must be measured"
    );
    assert!(
        metric.branching_factor > 0.0 && metric.branching_factor <= 4.0,
        "branching factor must be a per-state average over up to 4 moves: {}",
        metric.branching_factor
    );
    assert!(
        (0.0..1.0).contains(&metric.dead_end_density),
        "dead-end density must be a fraction: {}",
        metric.dead_end_density
    );
}

#[test]
fn difficulty_metric_is_deterministic() {
    let spec = scene_spec();
    let evidence = read_json(EVIDENCE);
    let first =
        puzzle_difficulty_metric::compute_difficulty(&spec, &evidence, SolveBudget::default())
            .expect("valid");
    let second =
        puzzle_difficulty_metric::compute_difficulty(&spec, &evidence, SolveBudget::default())
            .expect("valid");
    assert_eq!(first, second, "identical inputs yield identical metrics");
}

#[test]
fn stale_solver_evidence_fails_closed() {
    let spec = scene_spec();
    let stale = read_json(STALE_EVIDENCE);
    let error = puzzle_difficulty_metric::compute_difficulty(&spec, &stale, SolveBudget::default())
        .expect_err("stale evidence must fail closed");
    assert_eq!(
        error,
        "solver witness evidence is stale: it does not solve the current level"
    );
}

#[test]
fn missing_witness_evidence_fails_closed() {
    let spec = scene_spec();
    let evidence = json!({
        "schemaVersion": EVIDENCE_SCHEMA,
        "intendedSolution": ["left", "down", "left", "up"],
    });
    let error =
        puzzle_difficulty_metric::compute_difficulty(&spec, &evidence, SolveBudget::default())
            .expect_err("missing witness must fail closed");
    assert_eq!(
        error,
        "difficulty metric requires a non-empty solver witness"
    );
}

#[test]
fn wrong_evidence_schema_fails_closed() {
    let spec = scene_spec();
    let evidence = json!({
        "schemaVersion": "ouroforge.something-else.v1",
        "witness": ["left", "down", "left", "up"],
        "intendedSolution": ["left", "down", "left", "up"],
    });
    let error =
        puzzle_difficulty_metric::compute_difficulty(&spec, &evidence, SolveBudget::default())
            .expect_err("wrong schema must fail closed");
    assert_eq!(
        error,
        format!("difficulty metric requires {EVIDENCE_SCHEMA} solver evidence")
    );
}

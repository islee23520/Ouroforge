//! Deterministic demo smoke test for Puzzle Solver and Over-Solution Detection
//! v1 (#1584).
//!
//! Reproduces, from a fresh clone and with no network or live browser, the
//! end-to-end moat behavior over fixture-scoped levels in
//! `examples/puzzle-solver-oversolution-v1/demo/`:
//!
//! - the solver (#1580) proves both levels solvable with a replayable witness;
//! - the over-solution detector (#1581) surfaces a strictly shorter unintended
//!   solution on the dirty level as a replayable counterexample trace, and none
//!   on the clean level;
//! - the design-integrity gate verdict — defined by the scope contract (#1579)
//!   as *intent satisfied AND no unintended over-solution* — FAILS the dirty
//!   level and PASSES the clean level.
//!
//! The gate verdict here is composed from the merged solver/detector surfaces to
//! demonstrate the gate's semantics end to end; the formal evaluator
//! `declared-gate-and` integration is tracked separately by #1583.

use std::path::{Path, PathBuf};

use ouroforge_core::puzzle_oversolution::{self, OverSolutionReport};
use ouroforge_core::puzzle_solver::{self, SolveBudget, SolveOutcome};
use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_scene(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("demo fixture exists");
    serde_json::from_str(&text).expect("demo fixture parses")
}

const DIRTY: &str = "examples/puzzle-solver-oversolution-v1/demo/dirty-level.json";
const CLEAN: &str = "examples/puzzle-solver-oversolution-v1/demo/clean-level.json";

/// The design-integrity gate verdict per the scope contract (#1579): a level
/// passes only when the intent is captured and satisfied AND the over-solution
/// detector finds no unintended over-solution. Invalid/missing intent fails
/// closed.
fn design_integrity_passes(spec: &Value, intent: &Value) -> bool {
    match puzzle_oversolution::detect_oversolutions(spec, intent, SolveBudget::default()) {
        Ok(report) => !report.has_oversolution(),
        Err(_) => false,
    }
}

fn detect(spec: &Value, intent: &Value) -> OverSolutionReport {
    puzzle_oversolution::detect_oversolutions(spec, intent, SolveBudget::default())
        .expect("valid spec + intent")
}

#[test]
fn dirty_level_is_solvable_but_caught_by_the_gate_with_a_replayable_trace() {
    let scene = read_scene(DIRTY);
    let spec = scene["gridPuzzle"].clone();
    let intent = scene["intent"].clone();

    // Solvability is table stakes.
    let outcome = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    let witness = match &outcome {
        SolveOutcome::Solvable { witness, .. } => witness.clone(),
        other => panic!("dirty demo level must be solvable, got {other:?}"),
    };
    assert!(puzzle_solver::replay(&spec, &witness)
        .expect("witness replays")
        .is_won());

    // The detector surfaces a strictly shorter unintended solution as a trace.
    let report = detect(&spec, &intent);
    assert!(
        report.has_oversolution(),
        "dirty level must have an over-solution"
    );
    let bypass = &report.counterexamples[0];
    assert!(bypass.length < report.intended_length);
    assert!(puzzle_solver::replay(&spec, &bypass.trace)
        .expect("counterexample replays")
        .is_won());

    // The design-integrity gate FAILS the dirty level.
    assert!(
        !design_integrity_passes(&spec, &intent),
        "the gate must fail the dirty level"
    );
}

#[test]
fn clean_level_is_solvable_and_passes_the_gate() {
    let scene = read_scene(CLEAN);
    let spec = scene["gridPuzzle"].clone();
    let intent = scene["intent"].clone();

    let outcome = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    assert!(outcome.is_solvable(), "clean demo level must be solvable");

    // The intended solution is the unique shortest, so no over-solution exists.
    let report = detect(&spec, &intent);
    assert!(
        !report.has_oversolution(),
        "clean level must have no over-solution: {:?}",
        report.counterexamples
    );

    // The design-integrity gate PASSES the clean level.
    assert!(
        design_integrity_passes(&spec, &intent),
        "the gate must pass the clean level"
    );
}

#[test]
fn demo_is_deterministic_across_runs() {
    for fixture in [DIRTY, CLEAN] {
        let scene = read_scene(fixture);
        let spec = scene["gridPuzzle"].clone();
        let intent = scene["intent"].clone();
        let first = detect(&spec, &intent);
        let second = detect(&spec, &intent);
        assert_eq!(
            first, second,
            "{fixture}: detector report must be deterministic"
        );
    }
}

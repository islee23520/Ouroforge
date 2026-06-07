//! Contract test for the Deterministic Grid-Puzzle Solver v1 (#1580).
//!
//! Exercises the trusted Rust solver (`ouroforge_core::puzzle_solver`) over the
//! shared `ouroforge.grid-puzzle.v1` fixtures from the Grid-Puzzle Game Class
//! (#1574). It machine-checks the solver's three required verdicts — solvable
//! with a replayable witness, unsolvable, and bounded-search exhaustion — plus
//! determinism and fail-closed validation. No new runtime or game model is
//! introduced; the solver operates over the existing state model.

use std::path::{Path, PathBuf};

use ouroforge_core::puzzle_solver::{self, SolveBudget, SolveOutcome};
use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn grid_puzzle_spec(scene_fixture: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(scene_fixture)).expect("fixture exists");
    let scene: Value = serde_json::from_str(&text).expect("fixture parses");
    scene["gridPuzzle"].clone()
}

const SOLVABLE: &str = "examples/game-runtime/grid-puzzle-scene-v1.json";
const UNSOLVABLE: &str = "examples/game-runtime/grid-puzzle-unsolvable-v1.json";
const MALFORMED: &str = "examples/game-runtime/grid-puzzle-invalid-malformed-grid.json";

#[test]
fn solvable_level_returns_a_replayable_witness() {
    let spec = grid_puzzle_spec(SOLVABLE);
    let outcome = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");

    let witness = match &outcome {
        SolveOutcome::Solvable { witness, .. } => witness.clone(),
        other => panic!("expected Solvable, got {other:?}"),
    };
    assert!(!witness.is_empty(), "a non-trivial level needs a witness");

    // The witness must actually replay to the win state on the same stepper.
    let final_state = puzzle_solver::replay(&spec, &witness).expect("witness replays");
    assert!(final_state.is_won(), "witness must reach the win state");
    assert_eq!(final_state.status(), "won");
}

#[test]
fn witness_is_at_least_as_short_as_the_declared_intended_solution() {
    let spec = grid_puzzle_spec(SOLVABLE);
    let intended = spec["intendedSolution"]
        .as_array()
        .expect("declared intended solution")
        .len();
    let outcome = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    let witness = outcome.witness().expect("solvable").to_vec();
    // BFS returns a shortest witness, so it can never exceed the authored path.
    assert!(
        witness.len() <= intended,
        "shortest witness {} must not exceed intended {}",
        witness.len(),
        intended
    );
}

#[test]
fn solver_is_deterministic_across_runs() {
    let spec = grid_puzzle_spec(SOLVABLE);
    let first = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    let second = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    assert_eq!(
        first, second,
        "identical inputs yield identical verdict + witness"
    );
}

#[test]
fn unsolvable_level_is_reported_after_full_exploration() {
    let spec = grid_puzzle_spec(UNSOLVABLE);
    let outcome = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    match outcome {
        SolveOutcome::Unsolvable { explored } => {
            assert!(explored > 0, "the reachable space must be explored");
        }
        other => panic!("expected Unsolvable, got {other:?}"),
    }
}

#[test]
fn bounded_search_exhaustion_is_explicit_not_a_false_negative() {
    let spec = grid_puzzle_spec(SOLVABLE);
    // A bound too small to reach the solution must report Exhausted, never
    // Unsolvable: the same level is decided Solvable under a generous bound.
    let tight = puzzle_solver::solve(&spec, SolveBudget { max_states: 2 }).expect("valid spec");
    match tight {
        SolveOutcome::Exhausted { explored, budget } => {
            assert_eq!(budget, 2);
            assert_eq!(explored, 2, "exhaustion is reported at the declared bound");
        }
        other => panic!("expected Exhausted under a tight bound, got {other:?}"),
    }

    let generous = puzzle_solver::solve(&spec, SolveBudget::default()).expect("valid spec");
    assert!(
        generous.is_solvable(),
        "the same level is solvable under a generous bound — exhaustion was not a false negative"
    );
}

#[test]
fn malformed_spec_fails_closed() {
    let spec = grid_puzzle_spec(MALFORMED);
    let error = puzzle_solver::solve(&spec, SolveBudget::default())
        .expect_err("malformed grid must be rejected before search");
    assert_eq!(error, "row 2 must be a string of length 6");
}

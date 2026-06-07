//! Contract test for Designer Intent Capture and the Over-Solution Detector v1
//! (#1581).
//!
//! Exercises `ouroforge_core::puzzle_oversolution` over the shared
//! `ouroforge.grid-puzzle.v1` fixtures. It machine-checks the three required
//! behaviors — an over-solution is surfaced as a replayable counterexample
//! trace, a single-solution level yields no false positive, and missing intent
//! fails closed — plus intent validation and determinism. The detector reuses
//! the #1580 solver and the existing state model; no new search engine.

use std::path::{Path, PathBuf};

use ouroforge_core::puzzle_oversolution::{self, OverSolutionReport};
use ouroforge_core::puzzle_solver::{self, SolveBudget};
use serde_json::{json, Value};

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_scene(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

const SINGLE_SOLUTION: &str = "examples/game-runtime/grid-puzzle-scene-v1.json";
const OVER_SOLUTION: &str = "examples/game-runtime/grid-puzzle-oversolution-v1.json";
const COUNTEREXAMPLE: &str =
    "examples/game-runtime/grid-puzzle-oversolution-counterexample-v1.json";

#[test]
fn over_solution_is_surfaced_as_a_replayable_counterexample_trace() {
    let scene = read_scene(OVER_SOLUTION);
    let spec = scene["gridPuzzle"].clone();
    let intent = scene["intent"].clone();

    let report = puzzle_oversolution::detect_oversolutions(&spec, &intent, SolveBudget::default())
        .expect("valid spec + intent");

    assert!(
        report.has_oversolution(),
        "a strictly shorter solution exists"
    );
    assert!(
        !report.exhausted,
        "the shorter-solution space fits in budget"
    );

    // The counterexample trace must actually replay to the win state.
    let bypass = &report.counterexamples[0];
    assert_eq!(bypass.kind, "shorter-than-intended");
    assert!(
        bypass.length < report.intended_length,
        "a counterexample must be strictly shorter than intended"
    );
    let replayed = puzzle_solver::replay(&spec, &bypass.trace).expect("trace replays");
    assert!(
        replayed.is_won(),
        "the counterexample trace reaches the win state"
    );
    assert_eq!(bypass.exercises_mechanic, Some(true));

    // The detector output matches the committed sample counterexample artifact.
    let sample = read_scene(COUNTEREXAMPLE);
    assert_eq!(
        sample["intendedLength"].as_u64().unwrap() as usize,
        report.intended_length
    );
    let sample_traces: Vec<Vec<String>> = sample["counterexamples"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            c["trace"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap().to_string())
                .collect()
        })
        .collect();
    let detected_traces: Vec<Vec<String>> = report
        .counterexamples
        .iter()
        .map(|c| c.trace.clone())
        .collect();
    assert_eq!(
        detected_traces, sample_traces,
        "detector matches the sample trace"
    );
}

#[test]
fn single_solution_level_yields_no_false_positive() {
    let scene = read_scene(SINGLE_SOLUTION);
    let spec = scene["gridPuzzle"].clone();
    // Capture intent from the level's own declared intended solution.
    let intent = json!({
        "intendedSolution": spec["intendedSolution"].clone(),
        "taughtMechanic": "push",
    });

    let report = puzzle_oversolution::detect_oversolutions(&spec, &intent, SolveBudget::default())
        .expect("valid spec + intent");

    assert!(
        !report.has_oversolution(),
        "the unique intended solution must not be flagged: {:?}",
        report.counterexamples
    );
    assert!(!report.exhausted);
}

#[test]
fn missing_intent_fails_closed() {
    let scene = read_scene(OVER_SOLUTION);
    let spec = scene["gridPuzzle"].clone();
    let error = puzzle_oversolution::capture_intent(&spec, &json!({}))
        .expect_err("missing intent must fail closed");
    assert_eq!(
        error,
        "intent capture requires a non-empty intendedSolution"
    );
}

#[test]
fn intent_that_does_not_win_fails_closed() {
    let scene = read_scene(OVER_SOLUTION);
    let spec = scene["gridPuzzle"].clone();
    let intent = json!({ "intendedSolution": ["down"] });
    let error = puzzle_oversolution::capture_intent(&spec, &intent)
        .expect_err("a non-winning intended path must be rejected");
    assert_eq!(
        error,
        "captured intended solution does not reach the win state"
    );
}

#[test]
fn unknown_taught_mechanic_fails_closed() {
    let scene = read_scene(OVER_SOLUTION);
    let spec = scene["gridPuzzle"].clone();
    let intent = json!({
        "intendedSolution": ["down", "up", "left"],
        "taughtMechanic": "teleport",
    });
    let error = puzzle_oversolution::capture_intent(&spec, &intent)
        .expect_err("an unknown mechanic must be rejected");
    assert_eq!(error, "unknown taught mechanic \"teleport\"");
}

#[test]
fn distinct_shorter_traces_to_the_same_board_are_all_surfaced() {
    // Two different player routes reach the same final board (crate on target,
    // player at the push origin) via different action sequences. The detector
    // must surface each as its own trace, not collapse them by final state.
    let spec = json!({
        "schemaVersion": "ouroforge.grid-puzzle.v1",
        "id": "sokoban-two-routes",
        "width": 5,
        "height": 5,
        "objects": {
            "floor": { "role": "background" },
            "wall": { "role": "solid" },
            "player": { "role": "player" },
            "crate": { "role": "pushable" },
            "target": { "role": "target" }
        },
        "legend": {
            "#": ["floor", "wall"],
            ".": ["floor"],
            "@": ["floor", "target"],
            "*": ["floor", "crate"],
            "P": ["floor", "player"]
        },
        "rows": ["#####", "#...#", "#@*.#", "#P..#", "#####"],
        "win": { "type": "all-targets-covered" },
        "lose": { "type": "none" }
    });
    // A long (8-step) declared intent so the shorter routes both qualify.
    let intent = json!({
        "intendedSolution": ["up", "up", "right", "right", "down", "up", "down", "left"],
        "taughtMechanic": "push"
    });

    let report = puzzle_oversolution::detect_oversolutions(&spec, &intent, SolveBudget::default())
        .expect("valid spec + intent");
    assert!(
        report.counterexamples.len() >= 2,
        "expected multiple shorter routes"
    );

    // Find two counterexamples with different traces but the same final board.
    struct Outcome {
        trace: Vec<String>,
        player: (usize, usize),
        crates: Vec<(usize, usize)>,
    }
    let outcomes: Vec<Outcome> = report
        .counterexamples
        .iter()
        .map(|c| {
            let state = puzzle_solver::replay(&spec, &c.trace).expect("trace replays");
            assert!(state.is_won(), "every counterexample reaches the win state");
            Outcome {
                trace: c.trace.clone(),
                player: state.player(),
                crates: state.pushable_positions(),
            }
        })
        .collect();

    let mut found_same_board_distinct_trace = false;
    for i in 0..outcomes.len() {
        for j in (i + 1)..outcomes.len() {
            let same_board = outcomes[i].player == outcomes[j].player
                && outcomes[i].crates == outcomes[j].crates;
            if outcomes[i].trace != outcomes[j].trace && same_board {
                found_same_board_distinct_trace = true;
            }
        }
    }
    assert!(
        found_same_board_distinct_trace,
        "two distinct traces to the same final board must both be surfaced: {:?}",
        report.counterexamples
    );
}

#[test]
fn detector_is_deterministic_across_runs() {
    let scene = read_scene(OVER_SOLUTION);
    let spec = scene["gridPuzzle"].clone();
    let intent = scene["intent"].clone();
    let first: OverSolutionReport =
        puzzle_oversolution::detect_oversolutions(&spec, &intent, SolveBudget::default()).unwrap();
    let second: OverSolutionReport =
        puzzle_oversolution::detect_oversolutions(&spec, &intent, SolveBudget::default()).unwrap();
    assert_eq!(first, second, "identical inputs yield an identical report");
}

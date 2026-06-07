//! Scenario Coverage v27 — Grid-Puzzle Game Class regression suite (#1577).

use std::path::{Path, PathBuf};

use ouroforge_core::grid_puzzle_dsl_ingest::ingest_puzzlescript;
use ouroforge_core::puzzle_solver::{self, SolveBudget, SolveOutcome};
use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(workspace_path(relative)).expect("read"))
        .expect("json")
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(workspace_path(relative)).expect("read")
}

const MATRIX: &str = "examples/grid-puzzle-game-class-v1/scenario-coverage-v27/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v27.md";

#[test]
fn v27_matrix_header_is_pinned() {
    let m = read_json(MATRIX);
    assert_eq!(
        m["schemaVersion"],
        "scenario-coverage-v27-grid-puzzle-matrix-v1"
    );
    assert_eq!(m["issue"], 1577);
}

#[test]
fn v27_dsl_ingest_valid_malformed_unsupported() {
    let m = read_json(MATRIX);
    let f = &m["dslFixtures"];
    ingest_puzzlescript(&read_text(f["valid"].as_str().unwrap())).expect("valid ingest");
    assert!(ingest_puzzlescript(&read_text(f["malformed"].as_str().unwrap())).is_err());
    assert!(ingest_puzzlescript(&read_text(f["unsupported"].as_str().unwrap())).is_err());
}

#[test]
fn v27_grid_puzzle_solves_and_replays() {
    let m = read_json(MATRIX);
    let spec = read_json(m["gridScenes"]["solvable"].as_str().unwrap())["gridPuzzle"].clone();
    let solution: Vec<String> = m["gridScenes"]["solution"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    let non_winning: Vec<String> = m["gridScenes"]["nonWinningReplay"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    let outcome = puzzle_solver::solve(
        &spec,
        SolveBudget {
            max_states: 200_000,
        },
    )
    .expect("solve");
    assert!(matches!(outcome, SolveOutcome::Solvable { .. }));
    assert!(puzzle_solver::replay(&spec, &solution)
        .expect("replay")
        .is_won());
    let blocked = puzzle_solver::replay(&spec, &non_winning).expect("non-winning replay");
    assert!(
        !blocked.is_won(),
        "non-winning replay must not assert a win"
    );
}

#[test]
fn v27_backward_compat_fixtures_preserve_platformer_and_collect_exit_shapes() {
    let m = read_json(MATRIX);
    let scene_path = m["backwardCompat"]["platformer"].as_str().unwrap();
    let scene = read_json(scene_path);
    assert!(
        scene.get("entities").is_some() || scene.get("objects").is_some(),
        "platformer scene shape"
    );
    assert_eq!(
        scene["metadata"]["template"],
        "engine-expansion-v1-one-screen-platformer"
    );

    let collect_path = m["backwardCompat"]["collectAndExit"].as_str().unwrap();
    let collect = read_json(collect_path);
    assert_eq!(collect["id"], "collect-and-exit-scene");
    assert_eq!(
        collect["metadata"]["scenarioId"],
        "collect-and-exit-source-smoke"
    );
    assert_eq!(
        collect["metadata"]["startState"]["checkpointSlot"],
        "demo-start"
    );
    assert!(collect["gameplayRules"]["flags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|flag| flag["id"] == "exit_reached"));
}

#[test]
fn v27_doc_governance() {
    let doc = read_text(DOC);
    assert!(doc.contains("#1") && doc.contains("#23") && doc.contains("fixture-scoped"));
}

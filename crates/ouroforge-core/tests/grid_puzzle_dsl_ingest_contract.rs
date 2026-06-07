//! Contract test for the PuzzleScript-compatible DSL ingest v1 (#1575).
//!
//! Exercises the trusted Rust/local validate-then-load ingest path
//! (`ouroforge_core::grid_puzzle_dsl_ingest`) over fixture-scoped
//! PuzzleScript-compatible documents. It machine-checks the ingest contract
//! from `docs/grid-puzzle-game-class-v1.md` rather than asserting it in prose:
//!
//! - a well-formed PuzzleScript Sokoban document validates, lowers to the exact
//!   expected `ouroforge.grid-puzzle.v1` game-class spec, loads through the
//!   existing game-class validator, and *runs through the loop* (its solution
//!   replays deterministically to a win);
//! - a malformed document (a level glyph absent from the legend) is rejected
//!   fail-closed with a malformed-input diagnostic and does not load;
//! - a well-formed document that uses an out-of-subset construct (a non-push
//!   rule, an unsupported win condition) is rejected with an explicit
//!   unsupported-construct diagnostic;
//! - structural fail-closed cases (missing required sections) are rejected.
//!
//! The loading/stepping reuses the existing `ouroforge_core::puzzle_solver`
//! game-class validator and deterministic stepper — the same logic the runtime
//! (`examples/game-runtime/grid-puzzle.js`) mirrors — so ingest is a front door
//! onto the existing game class, not a parallel engine.

use std::path::{Path, PathBuf};

use ouroforge_core::grid_puzzle_dsl_ingest::ingest_puzzlescript;
use ouroforge_core::puzzle_solver::{self, SolveBudget, SolveOutcome};
use serde_json::{json, Value};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/grid-puzzle-dsl-ingest-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).expect("fixture exists")
}

fn expected_micro_spec() -> Value {
    json!({
        "schemaVersion": "ouroforge.grid-puzzle.v1",
        "id": "sokoban-micro-import",
        "width": 6,
        "height": 5,
        "objects": {
            "background": { "role": "background" },
            "wall": { "role": "solid" },
            "player": { "role": "player" },
            "crate": { "role": "pushable" },
            "target": { "role": "target" }
        },
        "legend": {
            "#": ["background", "wall"],
            "@": ["background", "target"],
            ".": ["background"],
            "*": ["background", "crate"],
            "P": ["background", "player"]
        },
        "rows": [
            "######",
            "#@...#",
            "#.*P.#",
            "#....#",
            "######"
        ],
        "win": { "type": "all-targets-covered" },
        "lose": { "type": "none" }
    })
}

#[test]
fn well_formed_puzzlescript_lowers_to_the_expected_game_class_spec() {
    let source = read_fixture("sokoban-micro.puzzlescript");
    let outcome = ingest_puzzlescript(&source).expect("well-formed import validates and loads");

    // The lowered spec equals the expected game-class spec, byte-for-byte by
    // value (object key order is irrelevant to Value equality).
    assert_eq!(outcome.spec, expected_micro_spec());

    // The report summarizes the deterministic role assignment.
    assert_eq!(outcome.report.id, "sokoban-micro-import");
    assert_eq!(outcome.report.width, 6);
    assert_eq!(outcome.report.height, 5);
    assert!(outcome
        .report
        .roles
        .contains(&("player".to_string(), "player".to_string())));
    assert!(outcome
        .report
        .roles
        .contains(&("crate".to_string(), "pushable".to_string())));
    assert!(outcome
        .report
        .roles
        .contains(&("wall".to_string(), "solid".to_string())));
    assert!(outcome
        .report
        .roles
        .contains(&("target".to_string(), "target".to_string())));
    assert!(outcome
        .report
        .roles
        .contains(&("background".to_string(), "background".to_string())));
}

#[test]
fn imported_game_loads_into_the_game_class_and_runs_to_a_win() {
    let source = read_fixture("sokoban-micro.puzzlescript");
    let outcome = ingest_puzzlescript(&source).expect("import validates");

    // It loads into the existing game-class validator.
    let state = puzzle_solver::validate_spec(&outcome.spec).expect("lowered spec loads");
    assert_eq!(state.status(), "playing");

    // It runs through the loop: the bounded solver finds a witness and the
    // witness replays deterministically to a win state.
    let solved = puzzle_solver::solve(&outcome.spec, SolveBudget::default())
        .expect("solver runs over the lowered spec");
    let witness = match &solved {
        SolveOutcome::Solvable { witness, .. } => witness.clone(),
        other => panic!("imported game should be solvable, got {other:?}"),
    };
    let replayed = puzzle_solver::replay(&outcome.spec, &witness).expect("witness replays");
    assert!(replayed.is_won(), "the witness must reach a win state");

    // The author-faithful intended solution also reaches a win.
    let intended = ["left", "down", "left", "up"].map(String::from);
    let final_state = puzzle_solver::replay(&outcome.spec, &intended).expect("intended replays");
    assert!(
        final_state.is_won(),
        "the canonical Sokoban solution reaches the win state"
    );
}

#[test]
fn malformed_document_is_rejected_fail_closed() {
    let source = read_fixture("malformed-undeclared-glyph.puzzlescript");
    let error = ingest_puzzlescript(&source).expect_err("malformed input must be rejected");
    assert!(
        error.is_malformed(),
        "an undeclared level glyph is a malformed-input rejection, got {error:?}"
    );
    assert!(
        error.message().to_lowercase().contains("glyph"),
        "the diagnostic must name the offending glyph: {error}"
    );
}

#[test]
fn used_object_missing_from_collisionlayers_is_rejected_fail_closed() {
    // Wall is used by the level but omitted from COLLISIONLAYERS, so its layer
    // is undeclared. Ingest must reject it rather than silently assign a layer.
    let source = read_fixture("malformed-missing-collisionlayer.puzzlescript");
    let error = ingest_puzzlescript(&source)
        .expect_err("a used object absent from COLLISIONLAYERS must be rejected");
    assert!(
        error.is_malformed(),
        "an incomplete COLLISIONLAYERS section is a malformed-input rejection, got {error:?}"
    );
    assert!(
        error.message().to_lowercase().contains("collisionlayers"),
        "the diagnostic must name COLLISIONLAYERS: {error}"
    );
}

#[test]
fn unsupported_rule_is_rejected_with_an_unsupported_diagnostic() {
    let source = read_fixture("unsupported-rule.puzzlescript");
    let error = ingest_puzzlescript(&source).expect_err("an out-of-subset rule must be rejected");
    assert!(
        error.is_unsupported(),
        "a non-push rule is an unsupported-construct rejection, got {error:?}"
    );
    assert!(
        error.message().to_lowercase().contains("push subset"),
        "the diagnostic must explain the unsupported rule: {error}"
    );
}

#[test]
fn missing_required_section_is_rejected() {
    // A document with no LEVELS section cannot load.
    let source = "\
title No Levels
========
OBJECTS
========
Background
lightgreen

Player
black

Crate
orange

Target
yellow

Wall
brown
=======
LEGEND
=======
. = Background
# = Wall and Background
P = Player and Background
* = Crate and Background
@ = Target and Background
================
COLLISIONLAYERS
================
Background
Target
Player, Wall, Crate
======
RULES
======
[ > Player | Crate ] -> [ > Player | > Crate ]
==============
WINCONDITIONS
==============
All Target on Crate
";
    let error = ingest_puzzlescript(source).expect_err("a missing LEVELS section must be rejected");
    assert!(error.is_malformed(), "got {error:?}");
    assert!(
        error.message().to_lowercase().contains("levels"),
        "the diagnostic must name the missing section: {error}"
    );
}

#[test]
fn unsupported_win_condition_is_rejected_with_an_unsupported_diagnostic() {
    // A `Some ... on ...` win condition is outside the supported subset.
    let source = "\
title Unsupported Win
========
OBJECTS
========
Background
lightgreen

Player
black

Crate
orange

Target
yellow

Wall
brown
=======
LEGEND
=======
. = Background
# = Wall and Background
P = Player and Background
* = Crate and Background
@ = Target and Background
================
COLLISIONLAYERS
================
Background
Target
Player, Wall, Crate
======
RULES
======
[ > Player | Crate ] -> [ > Player | > Crate ]
==============
WINCONDITIONS
==============
Some Target on Crate
=======
LEVELS
=======
######
#@...#
#.*P.#
#....#
######
";
    let error =
        ingest_puzzlescript(source).expect_err("an unsupported win condition must be rejected");
    assert!(
        error.is_unsupported(),
        "a `Some ... on ...` win condition is unsupported, got {error:?}"
    );
}

#[test]
fn empty_document_is_rejected() {
    let error = ingest_puzzlescript("").expect_err("an empty document must be rejected");
    assert!(error.is_malformed(), "got {error:?}");
}

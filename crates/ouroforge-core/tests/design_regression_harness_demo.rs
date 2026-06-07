//! Deterministic demo smoke test for the Design Regression Harness (#1589).
//!
//! Era F Milestone 29 (`docs/design-regression-harness-v1-demo.md`). From a
//! fresh clone, with no network or live browser, this exercises the committed
//! demo fixtures under `examples/design-regression-harness-v1/demo/` against the
//! merged #1588 harness and asserts the headline behavior:
//!
//! 1. A shared-rule edit that opens a new over-solution **elsewhere** (in a
//!    level other than the edited target) is flagged as a regression, and the
//!    regression carries a **replayable** counterexample trace that genuinely
//!    reaches the win state on the trusted stepper (trace linkage).
//! 2. A clean edit passes with no false regression and no trace.
//!
//! It is a fixture-scoped demo over the existing solver/over-solution/difficulty
//! surfaces; it adds no new engine and asserts behavior/gate states, not
//! subjective quality.

use std::path::{Path, PathBuf};

use ouroforge_core::design_regression_harness::{DesignRegressionHarness, RegressionOutcome};
use ouroforge_core::puzzle_solver;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn load(relative: &str) -> DesignRegressionHarness {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("demo fixture exists");
    DesignRegressionHarness::from_json_str(&text).expect("demo fixture is a valid harness artifact")
}

const REGRESSION_EDIT: &str = "examples/design-regression-harness-v1/demo/regression-edit.json";
const CLEAN_EDIT: &str = "examples/design-regression-harness-v1/demo/clean-edit.json";

#[test]
fn demo_regression_edit_flags_an_over_solution_elsewhere_with_a_replayable_trace() {
    let harness = load(REGRESSION_EDIT);
    let report = harness.run().expect("demo harness runs deterministically");

    // The run is a regression because a level elsewhere broke.
    assert_eq!(report.overall_verdict, "regressed");
    assert_eq!(report.regression_count, 1);
    assert!(report.promotion_blocked(), "a regression blocks promotion");

    // The directly-edited target stays design-clean (no false flag).
    let edited = report
        .levels
        .iter()
        .find(|l| l.level_id == "rule-edited-target")
        .expect("edited target level present");
    assert_eq!(edited.outcome, RegressionOutcome::Unchanged);
    assert!(edited.current.clean);
    assert!(edited.trace.is_none());

    // The shared-rule level ELSEWHERE is the one flagged newly-broken.
    let elsewhere = report
        .levels
        .iter()
        .find(|l| l.level_id == "shared-mechanic-elsewhere")
        .expect("elsewhere level present");
    assert_eq!(elsewhere.outcome, RegressionOutcome::NewlyBroken);
    assert!(
        elsewhere.baseline.clean,
        "elsewhere was clean before the edit"
    );
    assert!(!elsewhere.current.clean, "the edit broke it elsewhere");
    assert_eq!(
        elsewhere.trace_kind.as_deref(),
        Some("shorter-than-intended")
    );

    // Trace linkage: the flagged trace is non-empty and genuinely replays to a
    // win on the trusted stepper — "watch the bypass", not "trust me".
    let trace = elsewhere
        .trace
        .as_ref()
        .expect("the regression carries a replayable trace");
    assert!(!trace.is_empty());
    let spec = harness
        .levels
        .iter()
        .find(|l| l.level_id == "shared-mechanic-elsewhere")
        .expect("level present")
        .spec
        .clone();
    let replayed = puzzle_solver::replay(&spec, trace).expect("trace replays on the stepper");
    assert!(
        replayed.is_won(),
        "the flagged over-solution trace actually reaches the win state"
    );
    // The bypass is the single-push shortcut, strictly shorter than the
    // designer's intended 3-step scenic path.
    assert_eq!(trace, &vec!["left".to_string()]);
}

#[test]
fn demo_clean_edit_passes_with_no_false_regression() {
    let report = load(CLEAN_EDIT)
        .run()
        .expect("demo harness runs deterministically");

    assert_eq!(report.overall_verdict, "clean");
    assert_eq!(report.regression_count, 0);
    assert!(!report.has_regression());
    assert!(
        !report.promotion_blocked(),
        "a clean edit does not block promotion"
    );
    for level in &report.levels {
        assert_ne!(level.outcome, RegressionOutcome::NewlyBroken);
        assert!(level.current.clean, "clean edit keeps every level clean");
        assert!(level.trace.is_none(), "no false trace on a clean edit");
    }
}

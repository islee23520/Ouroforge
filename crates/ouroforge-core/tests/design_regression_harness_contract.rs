//! Contract test for the Design Regression Harness model and diff v1 (#1588).
//!
//! Era F Milestone 29 (`docs/design-regression-harness-v1.md`): design
//! regression as CI for game design. The harness re-runs the existing #1580
//! solver, #1581 over-solution detector, and #1582 difficulty suite over the
//! affected levels, diffs against the recorded baseline, and classifies each
//! level as unchanged / improved / newly-broken — with a replayable trace for
//! every regression. It machine-checks the four required scenarios: an edit
//! that opens a new over-solution (regression), an edit that fixes one
//! (improved), a no-op edit (unchanged), and a stale baseline reference
//! (fails closed as inconclusive). It is orchestration over existing surfaces;
//! no new comparison engine.

use std::path::{Path, PathBuf};

use ouroforge_core::design_regression_harness::{
    DesignRegressionHarness, RegressionOutcome, DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION,
};
use ouroforge_core::puzzle_solver;
use serde_json::{json, Value};

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(workspace_path(relative)).expect("fixture exists")
}

const HARNESS: &str = "examples/game-runtime/design-regression-harness-v1.json";
const STALE_HARNESS: &str = "examples/game-runtime/design-regression-harness-stale-v1.json";

fn load(relative: &str) -> DesignRegressionHarness {
    DesignRegressionHarness::from_json_str(&read_text(relative)).expect("valid harness fixture")
}

fn level<'a>(
    report: &'a ouroforge_core::design_regression_harness::DesignRegressionReport,
    id: &str,
) -> &'a ouroforge_core::design_regression_harness::DesignRegressionLevelResult {
    report
        .levels
        .iter()
        .find(|l| l.level_id == id)
        .unwrap_or_else(|| panic!("level {id} present"))
}

#[test]
fn harness_classifies_the_four_scenarios_and_blocks_promotion_on_regression() {
    let harness = load(HARNESS);
    let report = harness.run().expect("harness runs");

    assert_eq!(
        report.schema_version,
        DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION
    );
    assert_eq!(report.overall_verdict, "regressed");
    assert_eq!(report.regression_count, 1);
    assert!(report.has_regression());
    // A regression blocks any promotion/auto-apply claim.
    assert!(report.promotion_blocked());

    // Scenario 1: an edit that opens a new over-solution elsewhere is a
    // regression carrying a replayable counterexample trace.
    let broken = level(&report, "level-newly-broken");
    assert_eq!(broken.outcome, RegressionOutcome::NewlyBroken);
    assert!(broken.baseline.clean, "level was clean at baseline");
    assert!(!broken.current.clean, "level is broken after the edit");
    assert!(broken.current.oversolution_count >= 1);
    assert_eq!(broken.trace_kind.as_deref(), Some("shorter-than-intended"));
    let trace = broken.trace.as_ref().expect("regression carries a trace");
    assert_eq!(trace, &vec!["left".to_string()]);
    // The trace is genuinely replayable on the trusted stepper and wins.
    let spec = broken_spec(&harness, "level-newly-broken");
    assert!(
        puzzle_solver::replay(&spec, trace)
            .expect("trace replays")
            .is_won(),
        "the flagged bypass actually reaches the win state"
    );

    // Scenario 2: an edit that fixes an over-solution is improved, no trace.
    let improved = level(&report, "level-improved");
    assert_eq!(improved.outcome, RegressionOutcome::Improved);
    assert!(!improved.baseline.clean);
    assert!(improved.current.clean);
    assert_eq!(improved.current.oversolution_count, 0);
    assert!(improved.trace.is_none());
    // Difficulty is computed (descriptive) for the recomputed clean level.
    let difficulty = improved.difficulty.expect("difficulty computed");
    assert!(difficulty.reachable_states > 0);
    assert!(difficulty.branching_factor > 0.0);

    // Scenario 3: a no-op edit leaves a clean level unchanged, no false flag.
    let unchanged = level(&report, "level-unchanged");
    assert_eq!(unchanged.outcome, RegressionOutcome::Unchanged);
    assert!(unchanged.baseline.clean);
    assert!(unchanged.current.clean);
    assert!(unchanged.trace.is_none());
}

fn broken_spec(harness: &DesignRegressionHarness, level_id: &str) -> Value {
    harness
        .levels
        .iter()
        .find(|l| l.level_id == level_id)
        .expect("level present")
        .spec
        .clone()
}

#[test]
fn stale_baseline_reference_fails_closed_as_inconclusive() {
    let harness = load(STALE_HARNESS);
    let report = harness.run().expect("harness runs");

    // Scenario 4: a stale baseline reference cannot be trusted; the comparison
    // fails closed as inconclusive rather than a false improved/unchanged, and
    // promotion is blocked.
    assert_eq!(report.overall_verdict, "inconclusive");
    assert_eq!(report.regression_count, 0);
    assert!(report.promotion_blocked());

    let stale = level(&report, "level-stale-baseline");
    assert_eq!(stale.outcome, RegressionOutcome::Inconclusive);
    assert!(stale.detail.to_lowercase().contains("stale"));
    assert!(stale.trace.is_none());
}

#[test]
fn report_forbids_trusted_writes_and_self_approval() {
    let report = load(HARNESS).run().expect("harness runs");
    for forbidden in [
        "apply_patch",
        "auto_apply_fix",
        "merge_branch",
        "execute_command",
        "promote_without_evidence",
        "self_approve",
    ] {
        assert!(
            report.forbidden_actions.iter().any(|a| a == forbidden),
            "report must forbid {forbidden}"
        );
    }
    // The verdict is serializable for the read-only dashboard surface.
    let serialized = report.to_json().expect("report serializes");
    assert!(serialized.contains("\"overallVerdict\""));
    assert!(serialized.contains("\"traceKind\""));
}

#[test]
fn malformed_artifacts_are_rejected_before_running() {
    // Wrong schema version.
    let wrong_schema = json!({
        "schemaVersion": "not-the-harness",
        "editRef": "edits/x",
        "generatedOutputRoot": "runs/x",
        "levels": [],
        "guardrails": ["g"]
    });
    assert!(DesignRegressionHarness::from_json_str(&wrong_schema.to_string()).is_err());

    // Path-traversal in a local ref is rejected (fail closed).
    let traversal = json!({
        "schemaVersion": DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION,
        "editRef": "../escape",
        "generatedOutputRoot": "runs/x",
        "levels": [{
            "levelId": "l",
            "spec": {},
            "intent": {"intendedSolution": ["left"]},
            "baseline": {"solvable": true, "oversolutionCount": 0}
        }],
        "guardrails": ["g"]
    });
    assert!(DesignRegressionHarness::from_json_str(&traversal.to_string()).is_err());

    // No affected levels is rejected.
    let no_levels = json!({
        "schemaVersion": DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION,
        "editRef": "edits/x",
        "generatedOutputRoot": "runs/x",
        "levels": [],
        "guardrails": ["g"]
    });
    assert!(DesignRegressionHarness::from_json_str(&no_levels.to_string()).is_err());

    // A malformed current spec fails closed into an inconclusive level result
    // (not a panic, not a false verdict).
    let bad_spec = json!({
        "schemaVersion": DESIGN_REGRESSION_HARNESS_SCHEMA_VERSION,
        "editRef": "edits/x",
        "generatedOutputRoot": "runs/x",
        "levels": [{
            "levelId": "bad",
            "spec": {"schemaVersion": "wrong"},
            "intent": {"intendedSolution": ["left"]},
            "baseline": {"solvable": true, "oversolutionCount": 0}
        }],
        "guardrails": ["g"]
    });
    let report = DesignRegressionHarness::from_json_str(&bad_spec.to_string())
        .expect("artifact shape is valid")
        .run()
        .expect("runs");
    assert_eq!(report.overall_verdict, "inconclusive");
    assert_eq!(report.levels[0].outcome, RegressionOutcome::Inconclusive);
}

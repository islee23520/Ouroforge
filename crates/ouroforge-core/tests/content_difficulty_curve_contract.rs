//! Contract test for Whole-Game Difficulty-Curve Verification v1 (#1651).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. These tests machine-check the whole-game curve contract: a
//! monotonic-enough curve passes, a spike and a regression are each detected
//! with evidence, and missing/malformed per-stage evidence fails closed. Each
//! stage difficulty is *derived* from existing Milestone 28 difficulty metrics
//! and Milestone 32 balance reports — not a new engine — and the verdict is
//! descriptive, never a fun/quality/balance guarantee.

use std::path::PathBuf;

use ouroforge_core::content_difficulty_curve::{
    difficulty_from_balance_report, difficulty_from_metric, verify_curve, CurveInput,
    DIFFICULTY_CURVE_REPORT_SCHEMA,
};
use ouroforge_core::puzzle_difficulty_metric::DifficultyMetric;
use serde_json::json;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_input(name: &str) -> CurveInput {
    let path: PathBuf = repo_root()
        .join("examples/generative-front-door")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    CurveInput::from_json_str(&text).expect("fixture curve input parses")
}

#[test]
fn monotonic_enough_curve_passes() {
    let input = read_input("content-difficulty-curve-pass-v1.json");
    let report = verify_curve(&input).expect("curve verifies");

    assert_eq!(report.schema_version, DIFFICULTY_CURVE_REPORT_SCHEMA);
    assert_eq!(report.stage_count, 4);
    assert!(
        report.passed,
        "monotonic-enough curve must pass: {:?}",
        report.findings
    );
    assert!(report.findings.is_empty());
    assert_eq!(report.spikes, 0);
    assert_eq!(report.regressions, 0);

    // Realized curve derived from M28 solution length and M32 win-rate/turns.
    let diffs: Vec<f64> = report.curve.iter().map(|p| p.difficulty).collect();
    assert_eq!(diffs, vec![3.0, 5.0, 6.0, 10.0]);
}

#[test]
fn spike_and_regression_are_detected() {
    let input = read_input("content-difficulty-curve-spike-v1.json");
    let report = verify_curve(&input).expect("curve verifies");

    assert!(!report.passed);
    assert_eq!(report.spikes, 1);
    assert_eq!(report.regressions, 1);
    assert_eq!(report.findings.len(), 2);

    let spike = report.findings.iter().find(|f| f.kind == "spike").unwrap();
    assert_eq!(spike.from_stage, "lvl-1");
    assert_eq!(spike.to_stage, "lvl-2-spike");
    assert!(spike.delta > spike.tolerance);

    let regression = report
        .findings
        .iter()
        .find(|f| f.kind == "regression")
        .unwrap();
    assert_eq!(regression.from_stage, "lvl-2-spike");
    assert_eq!(regression.to_stage, "lvl-3-regression");
    assert!(-regression.delta > regression.tolerance);
}

#[test]
fn missing_evidence_fails_closed() {
    let input = read_input("content-difficulty-curve-missing-evidence-v1.json");
    let error = verify_curve(&input).expect_err("missing stage evidence must fail closed");
    assert!(
        error.to_string().contains("winRate"),
        "unexpected error: {error}"
    );
}

#[test]
fn verification_is_deterministic() {
    let input = read_input("content-difficulty-curve-pass-v1.json");
    let first = verify_curve(&input).expect("first");
    let second = verify_curve(&input).expect("second");
    assert_eq!(first, second);
}

#[test]
fn difficulty_from_metric_uses_solution_length() {
    let metric = DifficultyMetric {
        solution_length: 7,
        branching_factor: 2.0,
        dead_end_density: 0.2,
        mechanic_introduction_order: vec!["push".to_string()],
        reachable_states: 50,
    };
    assert_eq!(difficulty_from_metric(&metric), 7.0);
}

#[test]
fn difficulty_from_balance_report_rewards_hard_stages() {
    // Lower win rate + more turns => higher difficulty.
    let easy = json!({
        "schemaVersion": "ouroforge.balance-report.v1",
        "winRate": { "wins": 5, "total": 5 },
        "difficultyCurve": [{ "turns": 2 }]
    });
    let hard = json!({
        "schemaVersion": "ouroforge.balance-report.v1",
        "winRate": { "wins": 1, "total": 5 },
        "difficultyCurve": [{ "turns": 8 }]
    });
    let easy_d = difficulty_from_balance_report(&easy).expect("easy");
    let hard_d = difficulty_from_balance_report(&hard).expect("hard");
    assert_eq!(easy_d, 2.0);
    assert_eq!(hard_d, 8.0 + 0.8 * 10.0);
    assert!(hard_d > easy_d);

    // A report with the wrong schema fails closed.
    let bad = json!({ "schemaVersion": "wrong", "winRate": { "wins": 1, "total": 1 } });
    assert!(difficulty_from_balance_report(&bad).is_err());
}

#[test]
fn balance_report_with_wins_exceeding_total_fails_closed() {
    // A malformed report where wins > total would produce a win rate above 1.0
    // and a negative loss term, understating difficulty; it must fail closed.
    let malformed = json!({
        "schemaVersion": "ouroforge.balance-report.v1",
        "winRate": { "wins": 6, "total": 5 },
        "difficultyCurve": [{ "turns": 4 }]
    });
    let error = difficulty_from_balance_report(&malformed)
        .expect_err("wins exceeding total must fail closed");
    assert!(
        error.to_string().contains("must not exceed"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_curve_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/content-difficulty-curve-v1.md"))
        .expect("content-difficulty-curve doc exists");
    assert!(
        doc.contains("#1651"),
        "Content-Difficulty-Curve v1 doc records this issue (#1651)"
    );
    assert!(
        doc.contains("spike") && doc.contains("regression"),
        "doc records spike/regression detection"
    );
}

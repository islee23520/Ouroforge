//! Escalating quota/ante run contract (#1806).
//!
//! This suite locks the bounded run shape over the existing card-roguelite
//! substrate. It is Rust/local mechanical evidence only: no browser/Studio
//! trusted writes, no parallel engine, and no automated fun or release claim.

use std::path::PathBuf;

use ouroforge_core::{
    resolve_card_roguelite_run_ante, CardRogueliteConfig, CardRogueliteStatus,
    CARD_ROGUELITE_RUN_ANTE_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("crate lives under crates/ouroforge-core")
        .to_path_buf()
}

fn read_json(relative: &str) -> Value {
    let path = repo_root().join(relative);
    serde_json::from_str(&std::fs::read_to_string(&path).expect("fixture is readable"))
        .expect("fixture parses as json")
}

fn read_config(relative: &str) -> CardRogueliteConfig {
    serde_json::from_value(read_json(relative)).expect("config parses")
}

#[test]
fn escalation_curve_is_bounded_and_rising() {
    let config = read_config("examples/run-shop-v1/fixtures/escalating.win.json");
    let report = resolve_card_roguelite_run_ante(&config).expect("run resolves");

    assert_eq!(
        report.schema_version,
        CARD_ROGUELITE_RUN_ANTE_SCHEMA_VERSION
    );
    assert!(report.bounded);
    assert_eq!(report.max_ante, 3);
    assert_eq!(report.rounds.len(), 3);
    assert_eq!(
        report
            .rounds
            .iter()
            .map(|round| round.quota)
            .collect::<Vec<_>>(),
        vec![40, 64, 84]
    );
    assert!(report
        .rounds
        .windows(2)
        .all(|pair| pair[0].quota <= pair[1].quota));
    assert!(report
        .read_only_inspection
        .disallowed_actions
        .contains(&"trusted writes".to_string()));
}

#[test]
fn win_condition_reaches_terminal_win_after_final_ante() {
    let config = read_config("examples/run-shop-v1/fixtures/escalating.win.json");
    let report = resolve_card_roguelite_run_ante(&config).expect("winning run resolves");

    assert_eq!(report.terminal_status, CardRogueliteStatus::Won);
    assert_eq!(report.total_score, 84);
    assert_eq!(report.final_gold, 47);
    assert!(!report.budget_exhausted);
    assert!(report.rounds.iter().all(|round| round.passed));
    assert_eq!(
        report
            .rounds
            .last()
            .expect("final round")
            .status_after_round,
        CardRogueliteStatus::Won
    );
}

#[test]
fn loss_condition_stops_when_budget_is_exhausted() {
    let config = read_config("examples/run-shop-v1/fixtures/escalating.loss.json");
    let report = resolve_card_roguelite_run_ante(&config).expect("losing run resolves");

    assert_eq!(report.terminal_status, CardRogueliteStatus::Lost);
    assert_eq!(report.total_score, 32);
    assert_eq!(report.rounds.len(), 2);
    assert!(report.rounds[0].passed);
    assert!(!report.rounds[1].passed);
    assert!(report.budget_exhausted);
    assert_eq!(report.final_gold, 13, "only passed ante rewards are paid");
}

#[test]
fn identical_seed_reproduces_run_report_digest() {
    let first = read_config("examples/run-shop-v1/fixtures/escalating.win.json");
    let second = read_config("examples/run-shop-v1/fixtures/escalating.win.json");

    let first_report = resolve_card_roguelite_run_ante(&first).expect("first run resolves");
    let second_report = resolve_card_roguelite_run_ante(&second).expect("second run resolves");

    assert_eq!(first_report.rounds, second_report.rounds);
    assert_eq!(first_report.digest, second_report.digest);
    assert_eq!(first_report.seed_algorithm, "mulberry32");
}

#[test]
fn non_escalating_quota_curve_fails_closed() {
    let mut config = read_config("examples/run-shop-v1/fixtures/escalating.win.json");
    config.run.ante_steps[1].quota = 12;

    let error = resolve_card_roguelite_run_ante(&config).expect_err("curve drift fails closed");
    assert!(error.to_string().contains("non-decreasing quota curve"));
}

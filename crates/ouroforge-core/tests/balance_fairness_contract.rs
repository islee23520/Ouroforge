//! Contract test for Fairness and Daily-Seed Solvability Verifier v1 (#1814).
//!
//! Reuses deterministic seeded-run evidence and solver-style winning witnesses.
//! The verifier is descriptive Rust/local evidence only: no new engine, no
//! trusted browser writes, no auto-apply, no auto-merge, and no fun/quality/
//! production/Godot-parity claim.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_fairness::{verify_fairness, FairnessFixture, SeedVerdict};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_fixture(name: &str) -> FairnessFixture {
    let path = repo_root()
        .join("examples/engine-builder-balance-v1/fairness")
        .join(name);
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn skilled_winnable_seed_passes_and_unfair_daily_seed_is_flagged() {
    let fixture = read_fixture("fairness.fixture.json");
    let report = verify_fairness(&fixture).expect("valid fixture");

    let pass = report.results.iter().find(|r| r.seed == 7201).unwrap();
    assert_eq!(pass.verdict, SeedVerdict::Pass);
    assert!(pass.skilled_winnable);
    assert!(pass.daily_solvable);
    assert!(pass.loss_attribution_supported);
    assert_eq!(pass.replay_persona.as_deref(), Some("skilled-planner"));

    let unfair = report.results.iter().find(|r| r.seed == 7202).unwrap();
    assert_eq!(unfair.verdict, SeedVerdict::Unfair);
    assert!(!unfair.skilled_winnable);
    assert!(!unfair.daily_solvable);
    assert_eq!(report.unfair_seeds, vec![7202]);
    assert_eq!(report.daily_seed_failures, vec![7202]);
    assert!(report.boundary.contains("browser/Studio read-only"));
}

#[test]
fn fairness_report_is_deterministic() {
    let fixture = read_fixture("fairness.fixture.json");
    let first = verify_fairness(&fixture).expect("first report");
    let second = verify_fairness(&fixture).expect("second report");

    assert_eq!(first, second);
    assert_eq!(
        first.digest,
        "fairness|fixture=engine-builder-fairness|7201:Pass:w1:d1:a1,7202:Unfair:w0:d0:a0"
    );
}

#[test]
fn malformed_seed_evidence_fails_closed() {
    let fixture = read_fixture("malformed.fixture.json");
    let err = verify_fairness(&fixture).expect_err("duplicate seed fails closed");
    assert!(err.contains("duplicate seed"));
}

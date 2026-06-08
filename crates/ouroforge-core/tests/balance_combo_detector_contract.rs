//! Contract test for Combo-Explosion and Degenerate-Build Detector v1 (#1812).
//!
//! Reuses Milestone 32 synthetic-player balance telemetry and the Milestone 28
//! over-solution concept of replayable counterexamples. The detector is
//! descriptive Rust/local logic over existing telemetry, not a new analyzer
//! engine, and it performs no trusted browser write, auto-apply, auto-merge, or
//! fun/quality/production/Godot-parity claim.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_combo_detector::{detect_degenerate_builds, ComboTelemetryFixture};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_fixture(name: &str) -> ComboTelemetryFixture {
    let path = repo_root()
        .join("examples/engine-builder-balance-v1/combo-detector")
        .join(name);
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn planted_degenerate_combo_is_detected_with_replayable_seed() {
    let fixture = read_fixture("degenerate-combo.fixture.json");
    let report = detect_degenerate_builds(&fixture).expect("valid fixture");

    assert_eq!(report.total_runs, 5);
    assert_eq!(report.total_winning_runs, 4);
    let finding = report
        .findings
        .iter()
        .find(|finding| finding.cards == ["overcharger", "reactor-loop"])
        .expect("planted combo detected");
    assert_eq!(finding.winning_runs_with_combo, 4);
    assert_eq!(finding.total_winning_runs, 4);
    assert_eq!(finding.combo_win_share_bps, 10_000);
    assert_eq!(finding.combo_win_rate_bps, 10_000);
    assert!(finding.score_ratio_bps >= 15_000);
    assert_eq!(finding.replay_deck_seed, 310);
    assert_eq!(finding.replay_persona, "smith");
    assert!(!finding.replay_actions.is_empty());
    assert!(finding.reason.contains("over-solution"));
    assert!(report.boundary.contains("browser/Studio read-only"));
}

#[test]
fn balanced_builds_do_not_false_positive() {
    let fixture = read_fixture("balanced-build.fixture.json");
    let report = detect_degenerate_builds(&fixture).expect("valid fixture");

    assert_eq!(report.total_runs, 4);
    assert!(
        report.findings.is_empty(),
        "balanced builds should not be flagged: {:?}",
        report.findings
    );
}

#[test]
fn detection_is_deterministic_and_digest_pins_the_report() {
    let fixture = read_fixture("degenerate-combo.fixture.json");
    let first = detect_degenerate_builds(&fixture).expect("first report");
    let second = detect_degenerate_builds(&fixture).expect("second report");

    assert_eq!(first, second);
    assert_eq!(
        first.digest,
        "combo-detector|fixture=planted-degenerate-engine-combo|runs=5|wins=4|findings=overcharger+reactor-loop@4/4:10000:10000:28750:310"
    );
}

#[test]
fn malformed_or_boundary_drift_fails_closed() {
    let fixture = read_fixture("malformed.fixture.json");
    let err = detect_degenerate_builds(&fixture).expect_err("invalid boundary fails closed");
    assert!(err.contains("boundary"));
}

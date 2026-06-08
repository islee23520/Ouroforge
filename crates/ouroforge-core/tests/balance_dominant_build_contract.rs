//! Contract test for Dominant-Build Analyzer v1 (#1813).
//!
//! Extends Milestone 32 pick-rate/win-rate analysis to engine-builder builds.
//! Metrics are computed from seeded run evidence, not asserted, and remain
//! descriptive Rust/local evidence only: no new analyzer engine, no trusted
//! browser writes, no auto-apply, no auto-merge, and no fun/quality/production/
//! Godot-parity claim.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_dominant_build::{
    analyze_dominant_builds, DominantBuildTelemetryFixture,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_fixture(name: &str) -> DominantBuildTelemetryFixture {
    let path = repo_root()
        .join("examples/engine-builder-balance-v1/dominant-build")
        .join(name);
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn planted_dominant_build_and_dead_modifier_are_flagged() {
    let fixture = read_fixture("dominant-build.fixture.json");
    let report = analyze_dominant_builds(&fixture).expect("valid fixture");

    assert_eq!(report.total_runs, 5);
    assert_eq!(report.total_wins, 3);
    let dominant = report
        .dominant_builds
        .iter()
        .find(|metric| metric.build_id == "loop-engine")
        .expect("dominant build flagged");
    assert_eq!(dominant.picks, 3);
    assert_eq!(dominant.wins, 3);
    assert_eq!(dominant.pick_rate_bps, 6000);
    assert_eq!(dominant.win_rate_bps, 10000);
    assert_eq!(dominant.replay_deck_seed, 510);
    assert_eq!(dominant.replay_persona, "smith");

    let dead = report
        .dead_modifiers
        .iter()
        .find(|metric| metric.modifier == "rusty-bearing")
        .expect("dead modifier flagged");
    assert_eq!(dead.picks, 0);
    assert_eq!(dead.pick_rate_bps, 0);
    assert!(report.boundary.contains("browser/Studio read-only"));
}

#[test]
fn balanced_build_mix_does_not_flag_dominance_or_dead_modifiers() {
    let fixture = read_fixture("balanced-builds.fixture.json");
    let report = analyze_dominant_builds(&fixture).expect("valid fixture");

    assert!(report.dominant_builds.is_empty());
    assert!(report.dead_modifiers.is_empty());
}

#[test]
fn aggregation_is_deterministic_and_digest_pins_metrics() {
    let fixture = read_fixture("dominant-build.fixture.json");
    let first = analyze_dominant_builds(&fixture).expect("first report");
    let second = analyze_dominant_builds(&fixture).expect("second report");

    assert_eq!(first, second);
    assert_eq!(
        first.digest,
        "dominant-build|fixture=planted-dominant-build|runs=5|wins=3|metrics=armor-engine:1/0:2000:0:94,loop-engine:3/3:6000:10000:233,starter-engine:1/0:2000:0:88|dominant=loop-engine|dead=rusty-bearing"
    );
}

#[test]
fn malformed_telemetry_fails_closed() {
    let fixture = read_fixture("malformed.fixture.json");
    let err = analyze_dominant_builds(&fixture).expect_err("undeclared modifier fails");
    assert!(err.contains("undeclared modifier"));
}

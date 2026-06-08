//! Smoke test for Engine-Builder Balance Demo v1 (#1815).
//!
//! The demo recomputes #1812/#1813/#1814 facts from fixture-scoped evidence and
//! asserts replay seeds. It is deterministic and local: no network, no live
//! browser, no trusted writes, no auto-apply, no auto-merge, and no fun/quality/
//! production/Godot-parity claim.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_combo_detector::{detect_degenerate_builds, ComboTelemetryFixture};
use ouroforge_core::balance_dominant_build::{
    analyze_dominant_builds, DominantBuildTelemetryFixture,
};
use ouroforge_core::balance_fairness::{verify_fairness, FairnessFixture, SeedVerdict};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

fn read_typed<T: serde::de::DeserializeOwned>(relative: &str) -> T {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

#[test]
fn demo_manifest_recomputes_flags_and_replay_seeds() {
    let manifest = read_json("examples/engine-builder-balance-v1/demo/demo-manifest.json");
    assert_eq!(
        manifest["schemaVersion"],
        "ouroforge.engine-builder-balance-demo.v1"
    );
    assert_eq!(manifest["issue"], "1815");
    let boundary = manifest["boundary"].as_str().unwrap();
    for phrase in [
        "Rust/local owns validation",
        "browser/Studio read-only",
        "no network/live browser",
        "no auto-apply",
        "no auto-merge",
        "#1 and #23 remain open",
    ] {
        assert!(
            boundary.contains(phrase),
            "missing boundary phrase {phrase}"
        );
    }

    let combo: ComboTelemetryFixture = read_typed(manifest["comboFixture"].as_str().unwrap());
    let combo_report = detect_degenerate_builds(&combo).expect("combo report");
    let combo_finding = combo_report
        .findings
        .iter()
        .find(|f| f.cards == ["overcharger", "reactor-loop"])
        .expect("degenerate combo finding");
    assert_eq!(combo_finding.replay_deck_seed, 310);
    assert_eq!(combo_finding.replay_persona, "smith");

    let dominant: DominantBuildTelemetryFixture =
        read_typed(manifest["dominantFixture"].as_str().unwrap());
    let dominant_report = analyze_dominant_builds(&dominant).expect("dominant report");
    let build = dominant_report
        .dominant_builds
        .iter()
        .find(|m| m.build_id == "loop-engine")
        .expect("dominant build finding");
    assert_eq!(build.replay_deck_seed, 510);
    assert_eq!(build.replay_persona, "smith");
    assert!(dominant_report
        .dead_modifiers
        .iter()
        .any(|m| m.modifier == "rusty-bearing"));

    let fairness: FairnessFixture = read_typed(manifest["fairnessFixture"].as_str().unwrap());
    let fairness_report = verify_fairness(&fairness).expect("fairness report");
    let unfair = fairness_report
        .results
        .iter()
        .find(|r| r.seed == 7202)
        .expect("unfair seed result");
    assert_eq!(unfair.verdict, SeedVerdict::Unfair);
    assert_eq!(fairness_report.daily_seed_failures, vec![7202]);
}

#[test]
fn demo_doc_records_boundaries_and_repro_command() {
    let doc = read_text("docs/engine-builder-balance-v1-demo.md");
    for required in [
        "cargo test -p ouroforge-core --test engine_builder_balance_demo_contract",
        "Generated runs/artifacts remain untracked unless fixture-scoped",
        "no auto-apply",
        "no auto-merge",
        "fun/feel verdict remains the human Era J gate",
        "Issues #1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }
}

//! Scoring-Engine Demo v1 smoke contract (#1802).
//!
//! The demo is fixture-scoped and local-only. It recomputes documented scoring
//! composition facts through the existing card-roguelite substrate without a
//! network, live browser, trusted write, auto-apply, auto-merge, or automated
//! fun/quality verdict.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    analyze_card_roguelite_score_composition, validate_card_roguelite_config, CardRogueliteConfig,
};
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

fn read_config(relative: &str) -> CardRogueliteConfig {
    serde_json::from_str(&read_text(relative)).expect(relative)
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|entry| entry.as_str().expect("string").to_string())
        .collect()
}

#[test]
fn demo_manifest_recomputes_composed_score_and_degenerate_combo() {
    let manifest = read_json("examples/scoring-engine-v1/demo/demo.manifest.json");
    assert_eq!(
        manifest["schemaVersion"],
        "ouroforge.scoring-engine-demo.v1"
    );
    assert_eq!(manifest["issue"], "1802");

    let boundary = manifest["boundary"].as_str().expect("boundary");
    for phrase in [
        "Rust/local scoring demo",
        "browser/Studio read-only",
        "no network/live browser",
        "no trusted writes",
        "no auto-apply",
        "no auto-merge",
        "no automated fun or quality verdict",
        "generated runs/artifacts remain untracked unless fixture-scoped",
        "#1 and #23 remain open",
    ] {
        assert!(
            boundary.contains(phrase),
            "missing boundary phrase {phrase}"
        );
    }

    let readable = &manifest["readableComposedScore"];
    let readable_config = read_config(readable["config"].as_str().expect("readable config"));
    validate_card_roguelite_config(&readable_config).expect("readable config validates");
    let readable_report =
        analyze_card_roguelite_score_composition(&readable_config).expect("readable report");
    let readable_finding = readable_report.findings.first().expect("readable finding");
    assert_eq!(readable_report.total_score, 30);
    assert_eq!(readable_report.total_score, readable["expectedTotalScore"]);
    assert_eq!(readable_finding.final_score, readable["expectedFinalScore"]);
    assert_eq!(
        readable_finding.modifier_ids,
        string_array(&readable["expectedModifierOrder"])
    );
    assert_eq!(readable_finding.degenerate, readable["expectedDegenerate"]);

    let degenerate = &manifest["degenerateCombo"];
    let degenerate_config = read_config(degenerate["config"].as_str().expect("degenerate config"));
    validate_card_roguelite_config(&degenerate_config).expect("degenerate config validates");
    assert_eq!(
        degenerate_config.seed,
        degenerate["replaySeed"].as_u64().unwrap() as u32
    );

    let first =
        analyze_card_roguelite_score_composition(&degenerate_config).expect("degenerate report");
    let second =
        analyze_card_roguelite_score_composition(&degenerate_config).expect("degenerate replay");
    let finding = first.findings.first().expect("degenerate finding");

    assert_eq!(
        first, second,
        "combo replay is digest-stable and deterministic"
    );
    assert_eq!(first.total_score, degenerate["expectedTotalScore"]);
    assert_eq!(finding.final_score, degenerate["expectedFinalScore"]);
    assert_eq!(
        finding.modifier_ids,
        string_array(&degenerate["expectedModifierOrder"])
    );
    assert_eq!(finding.degenerate, degenerate["expectedDegenerate"]);
    assert_eq!(finding.base_score, 10);
    assert_eq!(finding.power_delta, 80);
    assert_eq!(finding.multiplicative_count, 2);
}

#[test]
fn demo_doc_records_repro_command_and_boundaries() {
    let doc = read_text("docs/scoring-engine-v1-demo.md");
    for required in [
        "cargo test -p ouroforge-core --test scoring_engine_demo_contract --jobs 2",
        "examples/scoring-engine-v1/demo/demo.manifest.json",
        "(10 + 5) × 2 × 3 = 90",
        "No network, live browser",
        "Generated runs/artifacts remain untracked unless fixture-scoped",
        "fun/feel verdict remains the human Era J gate",
        "Issues #1 and #23 remain open",
    ] {
        assert!(doc.contains(required), "missing doc text {required}");
    }

    for forbidden in ["production-ready", "Godot replacement", "auto-scored fun"] {
        assert!(
            !doc.contains(forbidden),
            "demo doc must not contain forbidden wording {forbidden}"
        );
    }
}

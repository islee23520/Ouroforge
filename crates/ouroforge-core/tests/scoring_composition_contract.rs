//! Scoring composition contract (#1801).
//!
//! This surfaces reproducible modifier combinations over the existing
//! card-roguelite substrate. Individual effects remain readable; aggregate power
//! is a deterministic mechanical signal, not an automated fun or quality score.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    analyze_card_roguelite_score_composition, validate_card_roguelite_config, CardRogueliteConfig,
    CARD_ROGUELITE_SCORE_COMPOSITION_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture_config(relative: &str) -> CardRogueliteConfig {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn single_modifier_baseline_remains_readable_and_not_degenerate() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/composition.baseline.json",
    );
    validate_card_roguelite_config(&config).expect("baseline validates");

    let composition = analyze_card_roguelite_score_composition(&config).expect("analysis resolves");
    let finding = composition.findings.first().expect("finding exists");

    assert_eq!(
        composition.schema_version,
        CARD_ROGUELITE_SCORE_COMPOSITION_SCHEMA_VERSION
    );
    assert_eq!(composition.total_score, 20);
    assert_eq!(finding.base_score, 10);
    assert_eq!(finding.final_score, 20);
    assert_eq!(finding.modifier_count, 1);
    assert!(
        !finding.degenerate,
        "single readable modifier is baseline only"
    );
}

#[test]
fn degenerate_combo_is_reproducible_and_surfaced_without_fun_claim() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/composition.degenerate.json",
    );

    let first = analyze_card_roguelite_score_composition(&config).expect("analysis resolves");
    let second = analyze_card_roguelite_score_composition(&config).expect("analysis repeats");
    let finding = first.findings.first().expect("finding exists");

    assert_eq!(first.digest, second.digest);
    assert_eq!(finding.modifier_ids, ["tuned", "overdrive", "reactor-loop"]);
    assert_eq!(finding.readable_effects.len(), 3);
    assert_eq!(finding.base_score, 10);
    assert_eq!(finding.final_score, 90);
    assert_eq!(finding.power_delta, 80);
    assert_eq!(finding.multiplicative_count, 2);
    assert!(finding.degenerate, "(10 + 5) * 2 * 3 crosses the threshold");
    assert!(first
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "automated fun verdict"));
}

#[test]
fn composition_is_deterministic_for_repeated_cards_and_seeds() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/composition.determinism.json",
    );

    let first = analyze_card_roguelite_score_composition(&config).expect("analysis resolves");
    let second = analyze_card_roguelite_score_composition(&config).expect("analysis repeats");

    assert_eq!(first, second);
    assert_eq!(first.total_score, 64);
    assert_eq!(first.findings.len(), 3);
    assert_eq!(first.digest.value.len(), 16);
}

#[test]
fn composition_reuses_substrate_and_stays_read_only() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/composition.degenerate.json",
    );

    let composition = analyze_card_roguelite_score_composition(&config).expect("analysis resolves");

    assert_eq!(
        composition.read_only_inspection.trusted_emitter,
        "rust-card-roguelite-substrate"
    );
    assert!(composition
        .read_only_inspection
        .browser_studio_mode
        .contains("read-only"));
    assert!(composition
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "trusted writes"));
}

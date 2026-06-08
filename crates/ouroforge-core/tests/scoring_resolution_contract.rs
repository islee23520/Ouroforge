//! Deterministic scoring resolution contract (#1800).
//!
//! This locks chips × multiplier-style scoring as a Rust/local resolution pass
//! over the existing card-roguelite substrate. It reuses FNV1a64 canonical JSON
//! digests and read-only inspection boundaries; it is not a new runtime or a
//! browser/Studio trusted write path.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    resolve_card_roguelite_score_resolution, resolve_card_roguelite_state,
    validate_card_roguelite_config, CardRogueliteConfig,
    CARD_ROGUELITE_SCORE_RESOLUTION_SCHEMA_VERSION, CARD_ROGUELITE_SUBSTRATE_DIGEST_ALGORITHM,
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
fn same_seed_resolution_is_digest_stable_and_matches_state_score() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/resolution.parity.json",
    );
    validate_card_roguelite_config(&config).expect("config validates");

    let first = resolve_card_roguelite_score_resolution(&config).expect("resolution resolves");
    let second = resolve_card_roguelite_score_resolution(&config).expect("resolution repeats");
    let state = resolve_card_roguelite_state(&config).expect("substrate state resolves");

    assert_eq!(
        first.schema_version,
        CARD_ROGUELITE_SCORE_RESOLUTION_SCHEMA_VERSION
    );
    assert_eq!(first.digest, second.digest);
    assert_eq!(
        first.digest.algorithm,
        CARD_ROGUELITE_SUBSTRATE_DIGEST_ALGORITHM
    );
    assert_eq!(first.digest.value.len(), 16);
    assert_eq!(first.total_score, 16);
    assert_eq!(first.total_score, state.score);
    assert_eq!(
        first.read_only_inspection.trusted_emitter,
        "rust-card-roguelite-substrate"
    );
}

#[test]
fn explicit_order_uses_order_then_modifier_id_for_ties() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/resolution.order.json",
    );

    let resolution = resolve_card_roguelite_score_resolution(&config).expect("resolution resolves");
    let trace = resolution.card_scores.first().expect("card trace exists");
    let step_ids = trace
        .steps
        .iter()
        .map(|step| step.modifier_id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(step_ids, ["a-add", "z-double"]);
    assert_eq!(trace.base_score, 10);
    assert_eq!(trace.steps[0].after_multiply_score, 15);
    assert_eq!(trace.steps[1].before_score, 15);
    assert_eq!(
        trace.final_score, 30,
        "(10 + 5) * 2 under explicit tie order"
    );
    assert_eq!(resolution.total_score, 30);
}

#[test]
fn overflow_is_blocked_explicitly_in_resolution_and_state() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/resolution.overflow.json",
    );
    validate_card_roguelite_config(&config).expect("overflow fixture is shape-valid");

    let resolution_error = resolve_card_roguelite_score_resolution(&config)
        .expect_err("overflowing multiplier must fail closed")
        .to_string();
    let state_error = resolve_card_roguelite_state(&config)
        .expect_err("state resolution also fails closed on overflow")
        .to_string();

    assert!(
        resolution_error.contains("overflow") && resolution_error.contains("multiplicative"),
        "unexpected resolution error: {resolution_error}"
    );
    assert!(
        state_error.contains("overflow") && state_error.contains("multiplicative"),
        "unexpected state error: {state_error}"
    );
}

#[test]
fn resolution_trace_is_read_only_and_contains_effect_text_without_authority() {
    let config = fixture_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/resolution.parity.json",
    );

    let resolution = resolve_card_roguelite_score_resolution(&config).expect("resolution resolves");
    let step = &resolution.card_scores[0].steps[0];

    assert_eq!(
        step.effect_text.as_deref(),
        Some("add +3 before multipliers")
    );
    assert!(resolution
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "trusted writes"));
    assert!(resolution
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "automated fun verdict"));
}

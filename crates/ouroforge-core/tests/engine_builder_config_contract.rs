//! Engine-builder deckbuilder substrate config contract (#1794).
//!
//! This proves the first engine-builder deckbuilder variant is only a thin
//! configuration over Card-Roguelite Substrate v1 (#1792), deterministic and
//! distinct from the deck-roguelike config without adding a parallel engine or
//! trusted browser/Studio write path.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    card_roguelite_probe_state, default_deck_roguelike_substrate_config,
    default_engine_builder_deckbuilder_substrate_config, resolve_card_roguelite_state,
    validate_card_roguelite_config, CardRogueliteConfig,
    CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION,
};
use serde_json::json;

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
fn engine_builder_deckbuilder_config_matches_fixture_and_validates() {
    let config = default_engine_builder_deckbuilder_substrate_config(424_242);
    let fixture = fixture_config(
        "examples/card-roguelite-substrate-v1/engine-builder/deckbuilder.config.golden.json",
    );

    assert_eq!(config, fixture, "function output remains fixture-stable");
    validate_card_roguelite_config(&config).expect("engine-builder config validates");
    assert_eq!(config.variant, "engine-builder-deckbuilder");
    assert!(config.cards.contains_key("gear-train"));
    assert!(config.modifiers.contains_key("overdrive"));
}

#[test]
fn engine_builder_deckbuilder_run_is_deterministic_and_probe_observable() {
    let config = default_engine_builder_deckbuilder_substrate_config(424_242);

    let first = resolve_card_roguelite_state(&config).expect("engine-builder resolves");
    let second = resolve_card_roguelite_state(&config).expect("engine-builder repeats");
    let probe = card_roguelite_probe_state(&config).expect("probe resolves");

    assert_eq!(first, second, "same config and seed must be deterministic");
    assert_eq!(
        probe.schema_version,
        CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION
    );
    assert_eq!(probe.digest, first.digest);
    assert_eq!(probe.substrate_state, first);
    assert_eq!(first.config_id, "engine-builder-deckbuilder");
    assert_eq!(first.variant, "engine-builder-deckbuilder");
    assert_eq!(first.seed, 424_242);
    assert_eq!(first.hp, 24);
    assert_eq!(first.gold, 47);
    assert_eq!(first.shop_offers.len(), 4);
    assert_eq!(first.digest.value.len(), 16);
    assert!(first
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "trusted writes"));
    assert!(first
        .read_only_inspection
        .disallowed_actions
        .iter()
        .any(|action| action == "automated fun verdict"));
}

#[test]
fn engine_builder_deckbuilder_is_distinct_from_deck_roguelike_config() {
    let seed = 424_242;
    let engine_builder = default_engine_builder_deckbuilder_substrate_config(seed);
    let deck_roguelike = default_deck_roguelike_substrate_config(seed);

    let engine_state = resolve_card_roguelite_state(&engine_builder).expect("engine resolves");
    let deck_state = resolve_card_roguelite_state(&deck_roguelike).expect("deck resolves");

    assert_ne!(engine_builder.variant, deck_roguelike.variant);
    assert_ne!(engine_builder.config_id, deck_roguelike.config_id);
    assert_ne!(engine_builder.cards, deck_roguelike.cards);
    assert_ne!(engine_builder.starting_deck, deck_roguelike.starting_deck);
    assert_ne!(engine_state.deck, deck_state.deck);
    assert_ne!(engine_state.shop_offers, deck_state.shop_offers);
    assert_ne!(engine_state.digest.value, deck_state.digest.value);
}

#[test]
fn engine_builder_contract_is_not_a_parallel_engine_or_trusted_write_surface() {
    let config = default_engine_builder_deckbuilder_substrate_config(424_242);
    let state = resolve_card_roguelite_state(&config).expect("engine-builder resolves");
    let projection = json!({
        "trustedEmitter": state.read_only_inspection.trusted_emitter,
        "browserStudioMode": state.read_only_inspection.browser_studio_mode,
        "disallowedActions": state.read_only_inspection.disallowed_actions,
    });

    assert_eq!(
        projection["trustedEmitter"],
        "rust-card-roguelite-substrate"
    );
    assert_eq!(
        projection["browserStudioMode"],
        "read-only card-roguelite substrate inspection"
    );
    assert!(projection["disallowedActions"]
        .as_array()
        .expect("array")
        .iter()
        .any(|action| action == "live mutation"));
}

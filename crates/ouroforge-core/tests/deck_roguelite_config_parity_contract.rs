//! Deck-Roguelike-as-Substrate-Config parity contract (#1793).
//!
//! This locks the existing deck-roguelike class (#1601) as a migrated config
//! over Card-Roguelite Substrate v1 (#1792). The migration is additive and must
//! preserve fixture bytes/goldens; it does not rewrite runtime behavior.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    deck_roguelike_spec_to_substrate_config, resolve_card_roguelite_state,
    validate_card_roguelite_config, CardRogueliteConfig,
};
use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

fn canonical_bytes(value: &impl serde::Serialize) -> Vec<u8> {
    serde_json::to_vec_pretty(value).expect("serializes")
}

#[test]
fn migrated_deck_roguelike_config_matches_golden_bytes() {
    let scene = read_json("examples/game-runtime/deck-roguelike-scene-v1.json");
    let migrated = deck_roguelike_spec_to_substrate_config(&scene["deckRoguelike"])
        .expect("existing deck-roguelike fixture migrates");
    let golden: CardRogueliteConfig = serde_json::from_value(read_json(
        "examples/card-roguelite-substrate-v1/parity/deck-roguelike-classic.substrate.golden.json",
    ))
    .expect("golden config parses");

    validate_card_roguelite_config(&migrated).expect("migrated config validates");
    assert_eq!(
        migrated, golden,
        "migration output must match the golden config by value"
    );
    assert_eq!(
        canonical_bytes(&migrated),
        canonical_bytes(&golden),
        "migration output must preserve golden bytes"
    );
}

#[test]
fn migrated_fixture_preserves_existing_opening_hand_golden() {
    let scene = read_json("examples/game-runtime/deck-roguelike-scene-v1.json");
    let migrated = deck_roguelike_spec_to_substrate_config(&scene["deckRoguelike"])
        .expect("existing deck-roguelike fixture migrates");
    let substrate = resolve_card_roguelite_state(&migrated).expect("migrated config resolves");

    // This is the same opening draw pile/hand order locked by v31's JS and Rust
    // deck-roguelike contracts for seed 12345. The substrate config uses the
    // same seeded Fisher-Yates stream, so the migrated fixture remains unchanged.
    assert_eq!(
        &substrate.deck[..5],
        ["strike", "strike", "bash", "defend", "defend"]
    );
    assert_eq!(substrate.seed, 12_345);
    assert_eq!(substrate.variant, "deck-roguelike-classic");
    assert!(substrate
        .digest
        .value
        .chars()
        .all(|ch| ch.is_ascii_hexdigit()));
}

#[test]
fn malformed_existing_fixture_fails_closed_before_substrate_resolution() {
    let scene = read_json("examples/game-runtime/deck-roguelike-invalid-malformed-deck.json");
    let error = deck_roguelike_spec_to_substrate_config(&scene["deckRoguelike"])
        .expect_err("malformed existing fixture must not migrate")
        .to_string();
    assert!(
        error.contains("startingDeck references undeclared card")
            || error.contains("deck references undeclared card")
            || error.contains("cards vocabulary"),
        "unexpected error: {error}"
    );
}

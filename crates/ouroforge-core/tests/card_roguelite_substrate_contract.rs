//! Contract tests for Card-Roguelite Substrate Core Model v1 (#1792).
//!
//! The substrate generalizes the existing deck-roguelike class (#1601) over the
//! seeded RNG contract (#1600). These tests prove deterministic resolution,
//! seed reproducibility, fail-closed malformed config handling, and digest-stable
//! probe observability without introducing a parallel engine or trusted browser
//! write path.

use ouroforge_core::{
    card_roguelite_probe_state, card_roguelite_seed_algorithm,
    default_deck_roguelike_substrate_config, resolve_card_roguelite_state,
    validate_card_roguelite_config, CardRogueliteStatus,
    CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION, SEEDED_RNG_ALGORITHM,
};

use std::path::{Path, PathBuf};

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture_config(relative: &str) -> ouroforge_core::CardRogueliteConfig {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

#[test]
fn substrate_state_is_deterministic_and_probe_observable() {
    let config = fixture_config("examples/card-roguelite-substrate-v1/fixtures/classic.valid.json");

    let first = resolve_card_roguelite_state(&config).expect("valid substrate config");
    let second = resolve_card_roguelite_state(&config).expect("same config resolves");
    let probe = card_roguelite_probe_state(&config).expect("probe state resolves");

    assert_eq!(first, second, "same seed/config is byte-stable by value");
    assert_eq!(
        probe.schema_version,
        CARD_ROGUELITE_SUBSTRATE_PROBE_SCHEMA_VERSION
    );
    assert_eq!(probe.digest, first.digest);
    assert_eq!(probe.substrate_state.digest.value, first.digest.value);
    assert_eq!(first.config_id, "deck-roguelike-classic");
    assert_eq!(first.variant, "deck-roguelike-classic");
    assert_eq!(first.seed, 12_345);
    assert_eq!(first.rng.seed, 12_345);
    assert_eq!(first.digest.algorithm, "fnv1a64-canonical-json-v1");
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
fn seed_reproducibility_diverges_for_different_seeds() {
    let a = resolve_card_roguelite_state(&default_deck_roguelike_substrate_config(1))
        .expect("seed 1 resolves");
    let b = resolve_card_roguelite_state(&default_deck_roguelike_substrate_config(1))
        .expect("seed 1 repeats");
    let c = resolve_card_roguelite_state(&default_deck_roguelike_substrate_config(2))
        .expect("seed 2 resolves");

    assert_eq!(a.digest.value, b.digest.value);
    assert_ne!(a.deck, c.deck, "different seeds shuffle differently");
    assert_ne!(a.digest.value, c.digest.value);
    assert_eq!(card_roguelite_seed_algorithm(), SEEDED_RNG_ALGORITHM);
}

#[test]
fn malformed_config_is_rejected_fail_closed() {
    let config =
        fixture_config("examples/card-roguelite-substrate-v1/fixtures/malformed.missing-card.json");

    let error = validate_card_roguelite_config(&config)
        .expect_err("undeclared card reference must fail closed")
        .to_string();
    assert!(error.contains("startingDeck references undeclared card"));
    assert!(resolve_card_roguelite_state(&config).is_err());
}

#[test]
fn modifier_composition_is_ordered_and_deterministic() {
    let mut config = default_deck_roguelike_substrate_config(99);
    config.cards.get_mut("strike").unwrap().modifier_refs = vec!["double".into(), "plus".into()];
    config.modifiers.insert(
        "double".into(),
        ouroforge_core::CardRogueliteModifier {
            order: 20,
            add_score: 0,
            multiply_score: 2,
        },
    );
    config.modifiers.insert(
        "plus".into(),
        ouroforge_core::CardRogueliteModifier {
            order: 10,
            add_score: 1,
            multiply_score: 1,
        },
    );

    let first = resolve_card_roguelite_state(&config).expect("ordered modifiers resolve");
    let second = resolve_card_roguelite_state(&config).expect("ordered modifiers repeat");

    assert_eq!(first.score, second.score);
    assert_eq!(first.digest, second.digest);
    assert!(matches!(
        first.status,
        CardRogueliteStatus::Ready | CardRogueliteStatus::Won
    ));
}

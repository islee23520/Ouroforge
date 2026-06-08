//! Scenario Coverage v42 — Card-Roguelite Substrate Regression Suite (#1796).
//!
//! Locks Card-Roguelite Substrate v1 behavior with state/shape regressions only:
//! substrate determinism and fail-closed validation, deck-roguelike config golden
//! parity, engine-builder config parity, demo manifest shape, generated-state
//! wording, and the backward-compatibility guarantee that the pre-substrate
//! deck-roguelike v31 golden remains valid. No timing, browser, network, trusted
//! write, or subjective fun assertions are introduced.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::{
    deck_roguelike_spec_to_substrate_config, default_deck_roguelike_substrate_config,
    default_engine_builder_deckbuilder_substrate_config, resolve_card_roguelite_state,
    validate_card_roguelite_config, CardRogueliteConfig,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn read_text(relative: &str) -> String {
    let path = repo_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("parse {relative}: {error}"))
}

fn read_config(relative: &str) -> CardRogueliteConfig {
    serde_json::from_value(read_json(relative)).expect("substrate config parses")
}

fn assert_repo_ref(relative: &str) {
    assert!(
        repo_root().join(relative).is_file(),
        "stale fixture ref: {relative}"
    );
}

const MATRIX: &str =
    "examples/card-roguelite-substrate-v1/scenario-coverage-v42/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v42.md";

#[test]
fn v42_matrix_enumerates_card_roguelite_substrate_regressions() {
    let matrix = read_json(MATRIX);
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v42");
    assert_eq!(matrix["issue"], 1796);
    assert_eq!(matrix["fixtureScoped"], true);

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "rust/local owns substrate validation",
        "browser/studio surfaces are read-only",
        "state/shape assertions only",
        "no timing",
        "no network",
        "no live browser",
        "trusted writes",
        "auto-merge",
        "automated fun score",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "missing boundary: {required}");
    }

    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 8,
        "v42 enumerates all regression surfaces"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(scenario["system"].as_str().expect("system").to_string());
        assert!(scenario["kind"].is_string(), "{id} has kind");
        let fixture_ref = scenario["fixtureRef"].as_str().expect("fixture ref");
        assert_repo_ref(fixture_ref);
        assert!(scenario["expect"].is_string(), "{id} has expectation");
    }
    for system in [
        "substrate",
        "deck-roguelike-config",
        "engine-builder-config",
        "demo",
        "backcompat",
        "governance",
    ] {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("v42-backcompat-pre-substrate-deck-roguelike"));
}

#[test]
fn v42_substrate_determinism_and_fail_closed_shape_are_locked() {
    let config = read_config("examples/card-roguelite-substrate-v1/fixtures/classic.valid.json");
    validate_card_roguelite_config(&config).expect("classic fixture validates");

    let first = resolve_card_roguelite_state(&config).expect("classic resolves");
    let second = resolve_card_roguelite_state(&config).expect("classic resolves again");
    assert_eq!(first, second, "same config and seed are deterministic");
    assert_eq!(
        first.schema_version,
        "ouroforge.card-roguelite-substrate-state.v1"
    );
    assert_eq!(first.config_id, "deck-roguelike-classic");
    assert_eq!(first.variant, "deck-roguelike-classic");
    assert_eq!(first.seed, 12_345);
    assert_eq!(first.shop_offers.len(), 3);
    assert_eq!(first.digest.value.len(), 16);
    assert!(first.digest.value.chars().all(|ch| ch.is_ascii_hexdigit()));
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

    let divergent = resolve_card_roguelite_state(&default_deck_roguelike_substrate_config(12_346))
        .expect("different seed resolves");
    assert_ne!(first.digest.value, divergent.digest.value);
    assert_ne!(first.deck, divergent.deck);

    let malformed =
        read_config("examples/card-roguelite-substrate-v1/fixtures/malformed.missing-card.json");
    let error = validate_card_roguelite_config(&malformed)
        .expect_err("undeclared card ref fails closed")
        .to_string();
    assert!(error.contains("startingDeck references undeclared card"));
    assert!(resolve_card_roguelite_state(&malformed).is_err());
}

#[test]
fn v42_deck_roguelike_and_engine_builder_config_goldens_are_locked() {
    let scene = read_json("examples/game-runtime/deck-roguelike-scene-v1.json");
    let migrated = deck_roguelike_spec_to_substrate_config(&scene["deckRoguelike"])
        .expect("existing deck-roguelike fixture migrates");
    let golden = read_config(
        "examples/card-roguelite-substrate-v1/parity/deck-roguelike-classic.substrate.golden.json",
    );
    assert_eq!(migrated, golden, "migration output remains golden-stable");
    let migrated_state = resolve_card_roguelite_state(&migrated).expect("migrated resolves");
    assert_eq!(
        &migrated_state.deck[..5],
        ["strike", "strike", "bash", "defend", "defend"]
    );
    assert_eq!(migrated_state.shop_offers.len(), 1);

    let engine = default_engine_builder_deckbuilder_substrate_config(424_242);
    let engine_golden = read_config(
        "examples/card-roguelite-substrate-v1/engine-builder/deckbuilder.config.golden.json",
    );
    assert_eq!(
        engine, engine_golden,
        "engine-builder config remains fixture-stable"
    );
    let engine_state = resolve_card_roguelite_state(&engine).expect("engine-builder resolves");
    assert_eq!(engine.variant, "engine-builder-deckbuilder");
    assert_eq!(engine_state.shop_offers.len(), 4);
    assert_ne!(
        engine.cards, migrated.cards,
        "deckbuilder is a distinct config"
    );
    assert_ne!(engine_state.digest.value, migrated_state.digest.value);
}

#[test]
fn v42_demo_manifest_and_pre_substrate_backcompat_golden_remain_valid() {
    let manifest =
        read_json("examples/card-roguelite-substrate-v1/demo/substrate-demo.manifest.json");
    assert_eq!(
        manifest["schemaVersion"],
        "ouroforge.card-roguelite-substrate-demo.v1"
    );
    assert_eq!(manifest["execution"]["network"], "disabled");
    assert_eq!(manifest["execution"]["liveBrowser"], "not-required");
    assert_eq!(manifest["execution"]["trustedWrites"], "none");
    assert_eq!(manifest["execution"]["funVerdict"], "human-era-j-only");
    assert_eq!(
        manifest["governance"]["anchorsRemainOpen"],
        serde_json::json!([1, 23])
    );
    let runs = manifest["runs"].as_array().expect("demo runs");
    assert_eq!(
        runs.len(),
        2,
        "demo enumerates deck and engine-builder configs"
    );
    let demo_root = repo_root().join("examples/card-roguelite-substrate-v1/demo");
    for run in runs {
        let config_ref = run["configRef"].as_str().expect("config ref");
        let config_path = demo_root.join(config_ref);
        assert!(config_path.is_file(), "stale demo config ref: {config_ref}");
        let config: CardRogueliteConfig = serde_json::from_str(
            &fs::read_to_string(&config_path)
                .unwrap_or_else(|error| panic!("read {config_path:?}: {error}")),
        )
        .expect("demo run config parses");
        validate_card_roguelite_config(&config).expect("demo run config validates");
    }

    let backcompat = read_json(
        "examples/deck-roguelike-game-class-v1/scenario-coverage-v31/non-stochastic-digest.golden.json",
    );
    assert_eq!(
        backcompat["schemaVersion"],
        "deck-roguelike-backward-compat-golden-v31"
    );
    assert_eq!(backcompat["fixtureScoped"], true);
    let cases = backcompat["cases"].as_array().expect("backcompat cases");
    assert!(cases.len() >= 2, "pre-substrate golden remains enumerated");
    for case in cases {
        assert_repo_ref(case["sceneRef"].as_str().expect("scene ref"));
        let digest = case["expectedDigest"].as_str().expect("digest");
        assert_eq!(digest.len(), 16);
        assert!(digest.chars().all(|ch| ch.is_ascii_hexdigit()));
    }
}

#[test]
fn v42_doc_preserves_generated_state_wording_and_governance() {
    let doc = read_text(DOC);
    assert!(doc.contains("#1") && doc.contains("#23"));
    let lowered = doc.to_ascii_lowercase();
    for required in [
        "state/shape regressions only",
        "fixture-scoped",
        "no network",
        "no live browser",
        "read-only",
        "no auto-merge",
        "no auto-apply",
        "production-ready claim",
        "godot replacement/parity claim",
        "automated fun score",
        "generated runs",
        "remain open governance anchors",
    ] {
        assert!(lowered.contains(required), "doc missing {required}");
    }
}

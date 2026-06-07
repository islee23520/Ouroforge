//! Scenario Coverage v31 — Deck-Roguelike Game Class regression suite (#1603).
//!
//! CI-gated mirror of the runtime coverage runner
//! `examples/deck-roguelike-game-class-v1/scenario-coverage-v31-deck-roguelike.test.cjs`.
//! It machine-checks that the regression suite enumerates every seeded-determinism
//! dimension, re-derives the seeded opening-hand goldens in trusted Rust (the same
//! `mulberry32` stream as the runtime, #1600), validates the non-stochastic
//! backward-compatibility golden, and confirms the coverage doc keeps conservative
//! wording and the #1/#23 governance anchors.
//!
//! Asserts states/shapes and digest goldens only; no flaky or timing assertions.

use std::path::{Path, PathBuf};

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

fn assert_repo_ref(relative: &str) {
    assert!(
        workspace_path(relative).is_file(),
        "stale fixture ref: {relative}"
    );
}

const CASES: &str =
    "examples/deck-roguelike-game-class-v1/scenario-coverage-v31/cases.fixture.json";
const GOLDEN: &str =
    "examples/deck-roguelike-game-class-v1/scenario-coverage-v31/non-stochastic-digest.golden.json";
const DOC: &str = "docs/scenario-coverage-v31.md";

// --- Seeded mulberry32 stream + Fisher-Yates (identical to the runtime) -----------

const RNG_INCREMENT: u32 = 0x6d2b79f5;

fn next_raw(state: &mut u32) -> u32 {
    *state = state.wrapping_add(RNG_INCREMENT);
    let mut t = *state;
    t = (t ^ (t >> 15)).wrapping_mul(1 | t);
    t = (t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t))) ^ t;
    t ^ (t >> 14)
}

fn shuffled_deck(deck: &[String], seed: u32) -> Vec<String> {
    let mut result = deck.to_vec();
    let mut state = seed;
    let mut i = result.len();
    while i > 1 {
        i -= 1;
        let bound = (i + 1) as u32;
        let j = (next_raw(&mut state) % bound) as usize;
        result.swap(i, j);
    }
    result
}

/// The seeded opening hand is the first `handSize` cards of the shuffled deck,
/// mirroring the runtime module's create-state draw.
fn opening_hand(scene_ref: &str, seed: u32) -> Vec<String> {
    let scene = read_json(scene_ref);
    let deck = scene["deckRoguelike"]["deck"]
        .as_array()
        .expect("deck array")
        .iter()
        .map(|card| card.as_str().expect("card id").to_string())
        .collect::<Vec<_>>();
    let hand_size = scene["deckRoguelike"]["player"]["handSize"]
        .as_u64()
        .expect("hand size") as usize;
    shuffled_deck(&deck, seed)
        .into_iter()
        .take(hand_size)
        .collect()
}

fn expected_hand(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("hand array")
        .iter()
        .map(|card| card.as_str().expect("card id").to_string())
        .collect()
}

fn cases_of_kind<'a>(matrix: &'a Value, kind: &str) -> Vec<&'a Value> {
    matrix["cases"]
        .as_array()
        .expect("cases array")
        .iter()
        .filter(|case| case["kind"] == kind)
        .collect()
}

#[test]
fn v31_enumerates_every_seeded_determinism_dimension() {
    let matrix = read_json(CASES);
    assert_eq!(
        matrix["schemaVersion"],
        "deck-roguelike-scenario-coverage-v31"
    );
    assert_eq!(matrix["fixtureScoped"], true);
    assert_repo_ref(matrix["sceneRef"].as_str().expect("scene ref"));
    for kind in [
        "seeded-determinism",
        "seeded-divergence",
        "snapshot-restore",
        "run-reproducibility",
    ] {
        assert!(
            !cases_of_kind(&matrix, kind).is_empty(),
            "coverage enumerates a {kind} case"
        );
    }
}

#[test]
fn v31_same_seed_reproduces_the_opening_hand_golden() {
    let matrix = read_json(CASES);
    let scene_ref = matrix["sceneRef"].as_str().expect("scene ref");
    for case in cases_of_kind(&matrix, "seeded-determinism") {
        let seed = case["seed"].as_u64().expect("seed") as u32;
        let expected = expected_hand(&case["expect"]["openingHand"]);
        assert_eq!(
            opening_hand(scene_ref, seed),
            expected,
            "same-seed opening hand is reproducible"
        );
        // Re-derivation is deterministic: a second pass matches the first.
        assert_eq!(opening_hand(scene_ref, seed), opening_hand(scene_ref, seed));
    }
}

#[test]
fn v31_different_seed_shuffles_to_a_divergent_opening_hand() {
    let matrix = read_json(CASES);
    let scene_ref = matrix["sceneRef"].as_str().expect("scene ref");
    for case in cases_of_kind(&matrix, "seeded-divergence") {
        let seed_a = case["seedA"].as_u64().expect("seed a") as u32;
        let seed_b = case["seedB"].as_u64().expect("seed b") as u32;
        let hand_a = opening_hand(scene_ref, seed_a);
        let hand_b = opening_hand(scene_ref, seed_b);
        assert_eq!(hand_a, expected_hand(&case["expect"]["openingHandA"]));
        assert_eq!(hand_b, expected_hand(&case["expect"]["openingHandB"]));
        assert_ne!(hand_a, hand_b, "different seeds shuffle differently");
    }
}

#[test]
fn v31_non_stochastic_backward_compat_golden_is_well_formed_and_additive() {
    let golden = read_json(GOLDEN);
    assert_eq!(
        golden["schemaVersion"],
        "deck-roguelike-backward-compat-golden-v31"
    );
    assert_eq!(golden["fixtureScoped"], true);
    let cases = golden["cases"].as_array().expect("golden cases");
    assert!(
        cases.len() >= 2,
        "golden locks at least two non-stochastic states"
    );
    let mut digests = Vec::new();
    for case in cases {
        assert_repo_ref(case["sceneRef"].as_str().expect("scene ref"));
        let digest = case["expectedDigest"].as_str().expect("digest");
        assert_eq!(digest.len(), 16, "fnv1a64 digest is 16 hex chars");
        assert!(
            digest.chars().all(|c| c.is_ascii_hexdigit()),
            "digest is hex: {digest}"
        );
        digests.push(digest.to_string());
    }
    // Distinct grid states must record distinct digests, so the golden is a
    // meaningful regression rather than a constant.
    digests.sort();
    digests.dedup();
    assert_eq!(
        digests.len(),
        cases.len(),
        "each non-stochastic golden state has a distinct digest"
    );
}

#[test]
fn v31_doc_preserves_generated_state_wording_and_governance() {
    assert_repo_ref(DOC);
    let doc = std::fs::read_to_string(workspace_path(DOC)).expect("doc exists");
    assert!(
        doc.contains("#1") && doc.contains("#23"),
        "doc reconfirms #1/#23 anchors"
    );
    let lowered = doc.to_lowercase();
    // Conservative wording is asserted positively (the boundary explicitly
    // disclaims the overclaims) rather than by forbidding substrings, which
    // would false-positive on the negated disclaimer itself.
    assert!(
        lowered.contains("fixture-scoped"),
        "doc records the fixture-scoped boundary"
    );
    assert!(
        lowered.contains("read-only"),
        "doc records the browser/Studio read-only boundary"
    );
    assert!(
        lowered.contains("no production-ready")
            && lowered.contains("godot-replacement/parity claim"),
        "doc explicitly disclaims production-ready / Godot-replacement-or-parity claims"
    );
}

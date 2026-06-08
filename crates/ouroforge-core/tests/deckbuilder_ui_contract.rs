use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read(relative)).expect(relative)
}

#[test]
fn fixture_declares_card_hand_pipeline_ui_over_existing_deck_runtime() {
    let scene = read_json("examples/game-runtime/deckbuilder-ui-scene-v1.json");
    assert_eq!(scene["metadata"]["issue"], "1826");
    assert_eq!(
        scene["deckRoguelike"]["schemaVersion"],
        "ouroforge.deck-roguelike.v1"
    );
    assert_eq!(
        scene["deckbuilderUi"]["schemaVersion"],
        "ouroforge.deckbuilder-ui.v1"
    );
    assert_eq!(scene["deckbuilderUi"]["id"], "deckbuilder-ui-v1");

    let slots = scene["deckbuilderUi"]["pipelineSlots"]
        .as_array()
        .expect("pipeline slots");
    let ids = slots
        .iter()
        .map(|slot| slot["id"].as_str().expect("slot id"))
        .collect::<Vec<_>>();
    assert_eq!(ids, ["intent", "modifier", "commit"]);

    let deck = scene["deckRoguelike"]["deck"].as_array().expect("deck");
    let cards = scene["deckRoguelike"]["cards"].as_object().expect("cards");
    assert!(!deck.is_empty());
    assert!(!cards.is_empty());
    for card in deck {
        let card_id = card.as_str().expect("card id");
        assert!(
            cards.contains_key(card_id),
            "deck references fixture card {card_id}"
        );
    }
}

#[test]
fn runtime_sources_expose_probe_methods_without_new_trusted_write_surface() {
    let runtime = read("examples/game-runtime/runtime.js");
    let module = read("examples/game-runtime/deckbuilder-ui.js");
    let index = read("examples/game-runtime/index.html");

    for required in [
        "OuroforgeDeckbuilderUi",
        "deckbuilderUiSelectCard",
        "deckbuilderUiQueueSelected",
        "runtime.deckbuilder_ui.select_card",
        "runtime.deckbuilder_ui.queue_selected",
        "syncWithDeck",
    ] {
        assert!(
            runtime.contains(required) || module.contains(required),
            "missing runtime probe surface: {required}"
        );
    }
    assert!(index.contains("deckbuilder-ui.js"));

    let combined = format!("{runtime}\n{module}").to_ascii_lowercase();
    for required in [
        "read-only",
        "draft-only",
        "existing review/apply/trust-gradient path",
        "trustedwrite: false",
        "generated runs/artifacts remain untracked unless explicitly fixture-scoped",
    ] {
        assert!(
            combined.contains(required),
            "missing boundary phrase: {required}"
        );
    }
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "production-ready engine",
        "godot replacement enabled",
        "godot parity enabled",
        "quality/fun guaranteed",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}

#[test]
fn docs_record_generated_state_and_anchor_issue_governance() {
    let docs = read("docs/deckbuilder-ui-v1.md");
    let lower = docs.to_ascii_lowercase();
    for required in [
        "issue: #1826",
        "window.__ouroforge__",
        "generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "browser/studio surfaces remain read-only",
        "draft-only",
        "issues #1 and #23 remain open",
        "no production-ready",
    ] {
        assert!(lower.contains(required), "missing doc boundary: {required}");
    }
}

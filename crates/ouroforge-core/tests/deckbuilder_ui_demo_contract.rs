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
fn demo_manifest_declares_full_deckbuilder_ui_probe_shape() {
    let manifest = read_json("examples/deckbuilder-ui-v1/demo/demo-manifest.json");
    assert_eq!(
        manifest["schemaVersion"],
        "ouroforge.deckbuilder-ui-demo.v1"
    );
    assert_eq!(manifest["issue"], 1829);
    assert_eq!(
        manifest["sceneRef"],
        "examples/deckbuilder-ui-v1/demo/deckbuilder-ui-demo-scene-v1.json"
    );
    assert_eq!(manifest["determinism"]["network"], "disabled");
    assert_eq!(manifest["determinism"]["liveBrowser"], "not required");

    let expected = &manifest["expected"];
    assert_eq!(expected["handCardIds"].as_array().unwrap().len(), 5);
    assert_eq!(expected["pipelineSlotIds"].as_array().unwrap().len(), 3);
    assert_eq!(expected["shopOfferIds"].as_array().unwrap().len(), 2);
    assert_eq!(expected["runMapNodeIds"].as_array().unwrap().len(), 4);
    assert_eq!(expected["scoreDisplayId"], "score-display-v1");
    assert_eq!(expected["formattedFinalScore"], "24");
}

#[test]
fn demo_scene_reuses_existing_runtime_ui_without_new_authority() {
    let scene = read_json("examples/deckbuilder-ui-v1/demo/deckbuilder-ui-demo-scene-v1.json");
    assert_eq!(scene["metadata"]["issue"], "1829");
    assert_eq!(
        scene["deckRoguelike"]["schemaVersion"],
        "ouroforge.deck-roguelike.v1"
    );
    assert_eq!(
        scene["deckbuilderUi"]["schemaVersion"],
        "ouroforge.deckbuilder-ui.v1"
    );
    assert!(scene["deckbuilderUi"]["shop"].is_object());
    assert!(scene["deckbuilderUi"]["runMap"].is_object());
    assert!(scene["deckbuilderUi"]["scoreDisplay"].is_object());
}

#[test]
fn demo_docs_and_smoke_preserve_generated_state_and_governance() {
    let docs = read("docs/deckbuilder-ui-v1-demo.md");
    let smoke = read("examples/deckbuilder-ui-v1/demo/demo-smoke.test.cjs");
    let manifest = read("examples/deckbuilder-ui-v1/demo/demo-manifest.json");
    let combined = format!("{docs}\n{smoke}\n{manifest}").to_ascii_lowercase();

    for required in [
        "issue: #1829",
        "node examples/deckbuilder-ui-v1/demo/demo-smoke.test.cjs",
        "network disabled",
        "no live browser",
        "window.__ouroforge__",
        "trustedwrite: false",
        "generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "#1 and #23 remain open",
        "no production-ready",
    ] {
        assert!(
            combined.contains(required),
            "missing demo boundary: {required}"
        );
    }

    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "production-ready engine",
        "godot replacement enabled",
        "quality/fun guaranteed",
        "requires live browser",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden demo wording: {forbidden}"
        );
    }
}

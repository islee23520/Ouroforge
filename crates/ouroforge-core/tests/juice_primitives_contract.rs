//! Juice Primitives v1 (#1819) contract audit.
//!
//! The trusted Rust side keeps this feature bounded by auditing that the browser
//! runtime exposes deterministic, read-only feedback evidence only. JavaScript
//! owns in-game juice/feedback execution per the language boundary; this test
//! locks the fixture, probe wording, snapshot/restore surface, audio-intent
//! reuse, generated-state policy, and #1/#23 governance wording.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf()
}

fn repo_path(path: &str) -> PathBuf {
    repo_root().join(path)
}

fn read(path: &str) -> String {
    let full = repo_path(path);
    fs::read_to_string(&full)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", full.display()))
}

#[test]
fn fixture_declares_all_juice_primitives_and_conservative_boundaries() {
    let fixture: serde_json::Value =
        serde_json::from_str(&read("examples/game-runtime/juice-scene-v1.json"))
            .expect("juice fixture json parses");

    assert_eq!(fixture["id"], "juice-primitives-v1-fixture");
    assert_eq!(fixture["metadata"]["issue"], 1819);
    assert!(fixture["metadata"]["generatedStatePolicy"]
        .as_str()
        .unwrap()
        .contains("fixture-scoped"));
    assert!(fixture["metadata"]["boundary"]
        .as_str()
        .unwrap()
        .contains("human Era J owns feel/fun judgment"));

    let primitives = fixture["juice"]["primitives"]
        .as_array()
        .expect("fixture has juice primitives");
    let ids: Vec<_> = primitives
        .iter()
        .map(|primitive| primitive["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        ["spawn-pop", "tick-shake", "impact-stop", "pickup-sfx"]
    );
    let kinds: Vec<_> = primitives
        .iter()
        .map(|primitive| primitive["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["tween", "shake", "hit-stop", "sfx"]);
}

#[test]
fn runtime_wires_juice_through_existing_probe_snapshot_and_audio_intent_surfaces() {
    let runtime = read("examples/game-runtime/runtime.js");
    let juice = read("examples/game-runtime/juice.js");
    let index = read("examples/game-runtime/index.html");
    let js_test = read("examples/game-runtime/juice.test.cjs");

    for required in [
        "window.OuroforgeJuice",
        "emitJuiceEvents('scene_loaded')",
        "emitJuiceEvents('tick')",
        "emitJuiceEvents('collision'",
        "state.juice = world.juice ? juice.worldStateView(world.juice) : null",
        "juiceEventCount",
        "sourceFeedbackId",
    ] {
        assert!(
            runtime.contains(required),
            "missing runtime juice wiring: {required}"
        );
    }
    assert!(index.contains("<script src=\"juice.js\"></script>"));
    assert!(juice.contains("ouroforge.runtime-juice-feedback.v1"));
    assert!(juice.contains("mechanical deterministic feedback evidence"));
    assert!(juice.contains("auto-fun verdict"));
    assert!(
        juice.contains("audioIntent"),
        "sfx hooks must reuse audio-intent shape"
    );
    assert!(js_test.contains("snapshot/restore parity"));
    assert!(js_test.contains("playback === 'intent'"));
}

#[test]
fn governance_and_generated_state_audit_stays_explicit() {
    let paths = [
        "examples/game-runtime/juice.js",
        "examples/game-runtime/juice.test.cjs",
        "examples/game-runtime/juice-scene-v1.json",
    ];
    for path in paths {
        assert!(
            repo_path(path).exists(),
            "required #1819 artifact missing: {path}"
        );
    }

    let combined = format!(
        "{}\n{}\n{}",
        read("examples/game-runtime/juice.js"),
        read("examples/game-runtime/juice.test.cjs"),
        read("examples/game-runtime/juice-scene-v1.json")
    );
    for required in [
        "read-only",
        "trusted writes",
        "fixture-scoped",
        "feel/fun judgment remains human",
        "not a fun/quality verdict",
    ] {
        assert!(
            combined.contains(required),
            "missing conservative wording: {required}"
        );
    }
}

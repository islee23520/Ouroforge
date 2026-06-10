//! Scenario Coverage v109: M128 Signal Gate Relay dogfood vertical slice (#2385-#2387).

use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn repo_file(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(rel)
}

fn read_json(rel: &str) -> Value {
    let path = repo_file(rel);
    let text = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

#[test]
fn v109_covers_scaffold_content_polish_and_generated_state_boundaries() {
    let manifest = read_json("examples/playable-demo-v2/signal-gate-dogfood/ouroforge.project.json");
    assert_eq!(manifest["project"]["id"], "signal_gate_relay_dogfood");
    assert_eq!(manifest["governance"]["anchorsRemainOpen"], serde_json::json!([1, 23]));
    let generated = manifest["generated"]["roots"].as_array().expect("generated roots");
    for root in ["runs", "screenshots", "browser-profiles", "dist"] {
        assert!(generated.iter().any(|value| value == root), "missing generated root {root}");
    }

    let levels = read_json("examples/playable-demo-v2/signal-gate-dogfood/levels/signal-gate-relay-level-set.json");
    assert_eq!(levels["schemaVersion"], "signal-gate-level-set-v1");
    assert_eq!(levels["progressionSummary"]["encounterCount"], 3);
    assert_eq!(levels["studioEditSmoke"]["blockedBy2368"], "closed");
    assert_eq!(levels["progression"].as_array().expect("progression").len(), 3);

    let hud = read_json("examples/playable-demo-v2/signal-gate-dogfood/hud-feedback.json");
    let hud_states: Vec<_> = hud["hudStates"].as_array().expect("hud states").iter().map(|v| v["state"].as_str().unwrap()).collect();
    assert!(hud_states.contains(&"fail/blocked"));
    assert!(hud_states.contains(&"win/exit"));
    assert!(hud["boundary"].as_str().unwrap().contains("no automated fun verdict"));

    let rubric = read_json("examples/playable-demo-v2/signal-gate-dogfood/visual-audio-rubric.json");
    assert_eq!(rubric["issue"], 2387);
    assert!(rubric["boundary"].as_str().unwrap().contains("not pixel-perfect proof"));
    assert!(rubric["checks"].as_array().expect("checks").iter().all(|check| check["status"] == "pass"));
}

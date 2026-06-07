//! Scenario Coverage v32 (#1610).
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read_json(rel: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(root().join(rel)).expect("read")).expect("json")
}
fn read_text(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).expect("read")
}
#[test]
fn v32_matrix_and_golden() {
    let m =
        read_json("examples/synthetic-player-balance-v1/scenario-coverage-v32/matrix.fixture.json");
    assert_eq!(
        m["schemaVersion"],
        "scenario-coverage-v32-synthetic-player-balance-matrix-v1"
    );
    assert_eq!(m["issue"], 1610);
    assert!(root()
        .join(m["personaContract"].as_str().unwrap())
        .is_file());
    let g = read_json(m["compareGolden"].as_str().unwrap());
    assert_eq!(g["fixtureScoped"], true);
    assert_eq!(g["runKind"], "non-balance");
    assert_eq!(g["balanceTelemetryRequired"], false);
    assert_eq!(g["status"], "unchanged");
    assert!(g["evidenceRefs"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r == "docs/run-comparison-v2.md"));
}

#[test]
fn v32_persona_telemetry_and_compare_fixtures_are_targeted() {
    let m =
        read_json("examples/synthetic-player-balance-v1/scenario-coverage-v32/matrix.fixture.json");
    let personas = read_json(m["runtimeFixtures"]["personas"].as_str().unwrap());
    assert_eq!(
        personas["schemaVersion"],
        "ouroforge.synthetic-player-personas.v1"
    );
    assert!(personas["personas"]
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p["id"] == m["expectedPersonaDigest"]["personaId"]));
    let scene = read_json(m["runtimeFixtures"]["deckBalanceScene"].as_str().unwrap());
    assert_eq!(
        scene["deckRoguelike"]["schemaVersion"],
        "ouroforge.deck-roguelike.v1"
    );

    let contract = read_text(m["personaContract"].as_str().unwrap());
    assert!(contract.contains(m["expectedPersonaDigest"]["digest"].as_str().unwrap()));
    assert!(contract.contains("budget_exhausted"));
    assert!(contract.contains("reckless novice over-extends and dies"));
    let runtime = read_text("examples/game-runtime/synthetic-player.js");
    assert!(runtime.contains("readOnlyInspection"));
    assert!(runtime.contains("budgetExhausted"));

    let telemetry_doc = read_text(m["telemetryEvidence"]["doc"].as_str().unwrap());
    for signal in m["telemetryEvidence"]["signals"].as_array().unwrap() {
        assert!(telemetry_doc.contains(signal.as_str().unwrap()));
    }
    for phrase in m["telemetryEvidence"]["readOnlyBoundary"]
        .as_array()
        .unwrap()
    {
        assert!(telemetry_doc.contains(phrase.as_str().unwrap()));
    }
}

#[test]
fn v32_js_runner_executes_persona_determinism_and_diff_evidence() {
    let script = root()
        .join("examples/synthetic-player-balance-v1/scenario-coverage-v32-synthetic.test.cjs");
    let output = Command::new("node")
        .arg(&script)
        .current_dir(root())
        .output()
        .expect("node scenario runner executes");
    assert!(
        output.status.success(),
        "node runner failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
#[test]
fn v32_doc() {
    let doc = std::fs::read_to_string(root().join("docs/scenario-coverage-v32.md")).unwrap();
    assert!(doc.contains("#1") && doc.contains("fixture-scoped"));
}

use ouroforge_core::{add_evidence_artifact, create_run, evaluate_run};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: evaluator.semantic.gate.v1
title: Evaluator Semantic Gate Harness
goal: Prove first-class semantic evaluator gate behavior.
constraints:
  target: local-fixture
acceptance:
  - Semantic gate verdict is declared-invariant bounded.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed semantic acceptance scenario.
"#;

#[test]
fn semantic_gate_evaluator_emits_declared_invariant_subverdicts() {
    let (root, run_dir) = create_fixture_run("semantic-gate-states");
    write_world_state(&run_dir);
    write_scenario_result(&run_dir, "collect-and-exit", "passed");
    let run_id = read_run_id(&run_dir);

    for (id, fixture) in [
        (
            "pass-health",
            include_str!("../../../examples/evaluator-depth-v1/semantic/semantic-gate-pass-health.json"),
        ),
        (
            "fail-health",
            include_str!("../../../examples/evaluator-depth-v1/semantic/semantic-gate-fail-health.json"),
        ),
        (
            "missing-target-state",
            include_str!("../../../examples/evaluator-depth-v1/semantic/semantic-gate-missing-target-state.json"),
        ),
        (
            "unsupported-type",
            include_str!("../../../examples/evaluator-depth-v1/semantic/invalid/semantic-gate-unsupported-type.json"),
        ),
        (
            "malformed-invariant",
            include_str!("../../../examples/evaluator-depth-v1/semantic/invalid/semantic-gate-malformed-invariant.json"),
        ),
        (
            "unsafe-expression",
            include_str!("../../../examples/evaluator-depth-v1/semantic/invalid/semantic-gate-unsafe-expression.json"),
        ),
        (
            "stale-ref",
            include_str!("../../../examples/evaluator-depth-v1/semantic/invalid/semantic-gate-stale-ref.json"),
        ),
    ] {
        write_declared_semantic_model(&run_dir, id, fixture, &run_id);
    }

    let verdict = evaluate_run(&run_dir).expect("evaluation completes without panics");
    assert_eq!(verdict.status, "failed");
    assert_eq!(verdict.semantic.len(), 7);
    assert_eq!(state_for(&verdict.semantic, "pass-health"), "pass");
    assert_eq!(state_for(&verdict.semantic, "fail-health"), "fail");
    assert_eq!(
        state_for(&verdict.semantic, "missing-target-state"),
        "missing-target-state"
    );
    assert_eq!(
        state_for(&verdict.semantic, "unsupported-type"),
        "unsupported"
    );
    assert_eq!(
        state_for(&verdict.semantic, "malformed-invariant"),
        "malformed-invariant"
    );
    assert_eq!(
        state_for(&verdict.semantic, "unsafe-expression"),
        "unsafe-expression"
    );
    assert_eq!(state_for(&verdict.semantic, "stale-ref"), "stale-ref");

    let failed = verdict
        .semantic
        .iter()
        .find(|gate| gate.model_ref.contains("fail-health"))
        .unwrap();
    assert_eq!(failed.invariant_id, "health-non-negative");
    assert_eq!(failed.target_path.as_deref(), Some("player.health"));
    assert!(failed.reason.contains("health-non-negative"));
    assert!(failed.reason.contains("player.health"));
    assert!(failed
        .evidence_refs
        .contains(&"evidence/scenarios/collect-and-exit/world-state.json".to_string()));
    assert!(verdict
        .failures
        .iter()
        .any(|failure| failure["kind"] == "semantic_gate_failed"));

    let verdict_json: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("verdict.json")).unwrap()).unwrap();
    assert!(verdict_json["semantic"]
        .as_array()
        .unwrap()
        .iter()
        .any(|gate| gate["state"] == "unsafe-expression"));
    assert_eq!(verdict_json["metadata"]["semantic_gate_results"], 7);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn semantic_gate_is_additive_and_neutral_when_no_semantic_acceptance_declared() {
    let (root, run_dir) = create_fixture_run("semantic-gate-neutral");
    write_world_state(&run_dir);
    write_scenario_result(&run_dir, "collect-and-exit", "passed");

    let verdict =
        evaluate_run(&run_dir).expect("evaluation passes without declared semantic gates");
    assert_eq!(verdict.status, "passed");
    assert!(verdict.semantic.is_empty());
    let verdict_json = fs::read_to_string(run_dir.join("verdict.json")).unwrap();
    assert!(
        !verdict_json.contains("\"semantic\""),
        "empty semantic gate remains byte-compatible via skipped field"
    );

    fs::remove_dir_all(root).unwrap();
}

fn create_fixture_run(prefix: &str) -> (PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!("ouroforge-{prefix}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let seed_path = root.join("seed.yaml");
    fs::write(&seed_path, SEED).unwrap();
    let artifacts = create_run(&seed_path, root.join("runs")).unwrap();
    (root, artifacts.run_dir)
}

fn read_run_id(run_dir: &Path) -> String {
    let run: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run.json")).unwrap()).unwrap();
    run["id"].as_str().unwrap().to_string()
}

fn write_world_state(run_dir: &Path) {
    let scenario_dir = run_dir.join("evidence/scenarios/collect-and-exit");
    fs::create_dir_all(&scenario_dir).unwrap();
    fs::write(
        scenario_dir.join("world-state.json"),
        serde_json::to_string_pretty(&json!({
            "player": {
                "safeHealth": 3,
                "health": -1,
                "transform": { "x": 2, "y": 3 }
            },
            "level": { "bounds": { "minX": 0, "maxX": 10, "minY": 0, "maxY": 10 } },
            "entities": { "player": { "id": "player" } },
            "world": { "impossible": false }
        }))
        .unwrap(),
    )
    .unwrap();
    fs::write(scenario_dir.join("frame-stats.json"), "{}\n").unwrap();
    fs::write(scenario_dir.join("input-replay.json"), "{}\n").unwrap();
}

fn write_scenario_result(run_dir: &Path, scenario_id: &str, status: &str) {
    let result_path = format!("evidence/scenarios/{scenario_id}/scenario-result.json");
    fs::write(
        run_dir.join(&result_path),
        serde_json::to_string_pretty(&json!({
            "scenario_id": scenario_id,
            "status": status,
            "assertions": [],
            "evidence": {
                "world_state": format!("evidence/scenarios/{scenario_id}/world-state.json"),
                "frame_stats": format!("evidence/scenarios/{scenario_id}/frame-stats.json"),
                "input_replays": [format!("evidence/scenarios/{scenario_id}/input-replay.json")]
            }
        }))
        .unwrap(),
    )
    .unwrap();
    add_evidence_artifact(
        run_dir,
        "scenario-result",
        "application/json",
        &result_path,
        json!({"artifact":"scenario_result"}),
    )
    .unwrap();
}

fn write_declared_semantic_model(run_dir: &Path, id: &str, fixture: &str, run_id: &str) {
    let rendered = fixture.replace("__RUN_ID__", run_id);
    let value: Value = serde_json::from_str(&rendered).unwrap();
    let model_id = value["modelId"].as_str().unwrap().to_string();
    let model_path =
        format!("evidence/scenarios/collect-and-exit/semantic/{id}/runtime-invariant-model.json");
    fs::create_dir_all(run_dir.join(Path::new(&model_path).parent().unwrap())).unwrap();
    fs::write(
        run_dir.join(&model_path),
        serde_json::to_string_pretty(&value).unwrap(),
    )
    .unwrap();
    add_evidence_artifact(
        run_dir,
        &format!("semantic-gate-{id}"),
        "application/json",
        &model_path,
        json!({
            "artifact": "runtime_invariant_model",
            "gate": "semantic",
            "declaredAcceptance": true,
            "scenarioId": "collect-and-exit",
            "modelId": model_id
        }),
    )
    .unwrap();
}

fn state_for<T: serde::Serialize>(gates: &[T], model_ref_suffix: &str) -> String {
    gates
        .iter()
        .map(|gate| serde_json::to_value(gate).unwrap())
        .find(|gate| {
            gate["modelRef"]
                .as_str()
                .unwrap()
                .contains(model_ref_suffix)
        })
        .and_then(|gate| gate["state"].as_str().map(str::to_string))
        .unwrap()
}

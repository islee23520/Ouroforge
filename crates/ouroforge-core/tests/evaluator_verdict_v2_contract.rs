use ouroforge_core::{add_evidence_artifact, create_run, evaluate_run, update_journal};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: evaluator.verdict.v2
title: Evaluator Verdict v2 Harness
goal: Prove four-category verdict aggregation.
constraints:
  target: local-fixture
acceptance:
  - Declared gates aggregate without changing legacy runs.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed verdict scenario.
"#;

#[test]
fn legacy_two_gate_verdict_snapshot_stays_byte_compatible() {
    let (root, run_dir) = create_fixture_run("verdict-v2-legacy");
    write_world_state(&run_dir, 3);
    write_scenario_result(&run_dir, "collect-and-exit", "passed");

    let verdict = evaluate_run(&run_dir).expect("legacy evaluation passes");
    assert_eq!(verdict.status, "passed");
    assert!(verdict.visual.is_empty());
    assert!(verdict.semantic.is_empty());
    assert!(verdict.gate_categories.is_none());

    let verdict_json = fs::read_to_string(run_dir.join("verdict.json")).unwrap();
    assert!(!verdict_json.contains("\"visual\""));
    assert!(!verdict_json.contains("\"semantic\""));
    assert!(!verdict_json.contains("\"gateCategories\""));
    assert_eq!(
        serde_json::from_str::<Value>(&verdict_json).unwrap(),
        json!({
            "status": "passed",
            "summary": "1 scenario result(s) passed with consistent evidence.",
            "failures": [],
            "evidence_refs": ["evidence/scenarios/collect-and-exit/scenario-result.json"],
            "metadata": {
                "evaluator": "ouroforge-evaluator-v0",
                "scenario_results": 1,
                "suite_summaries": 0,
                "behavior_evaluator_results": 0,
                "visual_gate_results": 0,
                "semantic_gate_results": 0
            }
        })
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn declared_visual_and_semantic_gates_aggregate_and_journalize_failures() {
    let (root, run_dir) = create_fixture_run("verdict-v2-four-category");
    write_world_state(&run_dir, -1);
    write_scenario_result(&run_dir, "collect-and-exit", "passed");
    write_declared_visual_gate_parse_failure(&run_dir);
    write_declared_semantic_model(&run_dir);

    let verdict = evaluate_run(&run_dir).expect("four-category evaluation completes");
    assert_eq!(verdict.status, "failed");
    let categories = verdict.gate_categories.as_ref().unwrap();
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
    assert_eq!(categories["aggregation"]["undeclaredGatePolicy"], "neutral");
    assert_eq!(categories["mechanical"]["status"], "pass");
    assert_eq!(categories["runtime"]["declared"], false);
    assert_eq!(categories["visual"]["declared"], true);
    assert_eq!(categories["visual"]["status"], "fail");
    assert_eq!(categories["semantic"]["declared"], true);
    assert_eq!(categories["semantic"]["status"], "fail");

    let verdict_json: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("verdict.json")).unwrap()).unwrap();
    assert_eq!(verdict_json["gateCategories"], *categories);

    let journal = update_journal(&run_dir).expect("journal renders gate observations");
    assert!(journal.contains("## Four-Gate Verdict Categories"));
    assert!(journal.contains("- `mechanical`: `pass`"));
    assert!(journal.contains("- `runtime`: `pass` (declared: `false`"));
    assert!(journal.contains("- `visual`: `fail`"));
    assert!(journal.contains("- `semantic`: `fail`"));
    assert!(journal.contains("undeclared gate policy: `neutral`"));
    assert!(journal.contains("## Visual/Semantic Gate Observations"));
    assert!(journal.contains("Visual `"));
    assert!(journal.contains("Semantic `fail`: invariant `health-non-negative`"));
    assert!(journal.contains("Next-step hypothesis"));
    assert!(journal
        .contains("evidence/scenarios/collect-and-exit/visual/broken/visual-comparison.json"));
    assert!(journal.contains(
        "evidence/scenarios/collect-and-exit/semantic/fail/runtime-invariant-model.json"
    ));

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

fn write_world_state(run_dir: &Path, health: i32) {
    let scenario_dir = run_dir.join("evidence/scenarios/collect-and-exit");
    fs::create_dir_all(&scenario_dir).unwrap();
    fs::write(
        scenario_dir.join("world-state.json"),
        serde_json::to_string_pretty(&json!({
            "player": { "health": health, "transform": { "x": 2, "y": 3 } },
            "level": { "bounds": { "minX": 0, "maxX": 10, "minY": 0, "maxY": 10 } }
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

fn write_declared_visual_gate_parse_failure(run_dir: &Path) {
    let path = "evidence/scenarios/collect-and-exit/visual/broken/visual-comparison.json";
    fs::create_dir_all(run_dir.join(Path::new(path).parent().unwrap())).unwrap();
    fs::write(run_dir.join(path), "{}\n").unwrap();
    add_evidence_artifact(
        run_dir,
        "visual-gate-broken",
        "application/json",
        path,
        json!({
            "artifact": "visual_comparison_evidence",
            "gate": "visual",
            "declaredAcceptance": true,
            "scenarioId": "collect-and-exit",
            "checkpointId": "checkpoint-broken"
        }),
    )
    .unwrap();
}

fn write_declared_semantic_model(run_dir: &Path) {
    let run_id = read_run_id(run_dir);
    let path = "evidence/scenarios/collect-and-exit/semantic/fail/runtime-invariant-model.json";
    fs::create_dir_all(run_dir.join(Path::new(path).parent().unwrap())).unwrap();
    fs::write(
        run_dir.join(path),
        serde_json::to_string_pretty(&json!({
            "schemaVersion": "runtime-invariant-model-v1",
            "modelId": "semantic_gate_fail_health",
            "runId": run_id,
            "scenarioId": "collect-and-exit",
            "worldStatePath": "evidence/scenarios/collect-and-exit/world-state.json",
            "scenarioResultPath": "evidence/scenarios/collect-and-exit/scenario-result.json",
            "evidenceIndexPath": "evidence/index.json",
            "invariants": [{
                "invariantId": "health-non-negative",
                "invariantType": "health_non_negative",
                "targetPath": "player.health",
                "evidencePath": "evidence/scenarios/collect-and-exit/world-state.json"
            }]
        }))
        .unwrap(),
    )
    .unwrap();
    add_evidence_artifact(
        run_dir,
        "semantic-gate-fail",
        "application/json",
        path,
        json!({
            "artifact": "runtime_invariant_model",
            "gate": "semantic",
            "declaredAcceptance": true,
            "scenarioId": "collect-and-exit",
            "modelId": "semantic_gate_fail_health"
        }),
    )
    .unwrap();
}

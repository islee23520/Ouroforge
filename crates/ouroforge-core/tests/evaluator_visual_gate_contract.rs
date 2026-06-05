use ouroforge_core::{add_evidence_artifact, create_run, evaluate_run};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: evaluator.visual.gate.v1
title: Evaluator Visual Gate Harness
goal: Prove first-class visual evaluator gate behavior.
constraints:
  target: local-fixture
acceptance:
  - Visual gate verdict is evidence-linked and threshold bounded.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed visual acceptance scenario.
"#;

#[test]
fn visual_gate_evaluator_emits_declared_visual_subverdicts() {
    let (root, run_dir) = create_fixture_run("visual-gate-states");
    write_scenario_result(&run_dir, "collect-and-exit", "passed");

    for (id, fixture) in [
        ("pass-unchanged", include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-pass-unchanged.json")),
        ("fail-changed", include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-fail-changed-over-threshold.json")),
        ("pass-under-threshold", include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-pass-under-threshold.json")),
        ("missing-baseline", include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-missing-baseline.json")),
        ("missing-screenshot", include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-missing-screenshot.json")),
        ("dimension-mismatch", include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-dimension-mismatch.json")),
        ("malformed-screenshot", include_str!("../../../examples/evaluator-depth-v1/visual/invalid/visual-gate-malformed-screenshot.json")),
        ("missing-threshold", include_str!("../../../examples/evaluator-depth-v1/visual/invalid/visual-gate-missing-threshold.json")),
        ("stale-ref", include_str!("../../../examples/evaluator-depth-v1/visual/invalid/visual-gate-stale-ref.json")),
    ] {
        write_declared_visual_comparison(&run_dir, id, fixture, id != "stale-ref");
    }

    let verdict = evaluate_run(&run_dir).expect("evaluation completes without panics");
    assert_eq!(verdict.status, "failed");
    assert_eq!(verdict.visual.len(), 9);
    assert_eq!(state_for(&verdict.visual, "pass-unchanged"), "pass");
    assert_eq!(state_for(&verdict.visual, "fail-changed"), "fail");
    assert_eq!(state_for(&verdict.visual, "pass-under-threshold"), "pass");
    assert_eq!(
        state_for(&verdict.visual, "missing-baseline"),
        "missing-baseline"
    );
    assert_eq!(
        state_for(&verdict.visual, "missing-screenshot"),
        "missing-screenshot"
    );
    assert_eq!(state_for(&verdict.visual, "dimension-mismatch"), "fail");
    assert_eq!(
        state_for(&verdict.visual, "malformed-screenshot"),
        "unsupported-format"
    );
    assert_eq!(
        state_for(&verdict.visual, "missing-threshold"),
        "threshold-not-declared"
    );
    assert_eq!(state_for(&verdict.visual, "stale-ref"), "stale-ref");

    let changed = verdict
        .visual
        .iter()
        .find(|gate| gate.comparison_ref.contains("fail-changed"))
        .unwrap();
    assert!(changed.reason.contains(
        "evidence/scenarios/collect-and-exit/visual/fail-changed/visual-comparison.json"
    ));
    assert!(changed.reason.contains("64 changed pixels"));
    assert!(changed.reason.contains("threshold"));
    assert_eq!(
        changed.output_root,
        "evidence/scenarios/collect-and-exit/visual/fail-changed"
    );
    assert!(verdict
        .failures
        .iter()
        .any(|failure| failure["kind"] == "visual_gate_failed"));

    let verdict_json: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("verdict.json")).unwrap()).unwrap();
    assert!(verdict_json["visual"]
        .as_array()
        .unwrap()
        .iter()
        .any(|gate| gate["state"] == "threshold-not-declared"));
    assert_eq!(verdict_json["metadata"]["visual_gate_results"], 9);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn visual_gate_is_additive_and_neutral_when_no_visual_acceptance_declared() {
    let (root, run_dir) = create_fixture_run("visual-gate-neutral");
    write_scenario_result(&run_dir, "collect-and-exit", "passed");

    let verdict = evaluate_run(&run_dir).expect("evaluation passes without declared visual gates");
    assert_eq!(verdict.status, "passed");
    assert!(verdict.visual.is_empty());
    let verdict_json = fs::read_to_string(run_dir.join("verdict.json")).unwrap();
    assert!(
        !verdict_json.contains("\"visual\""),
        "empty visual gate remains byte-compatible via skipped field"
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

fn write_scenario_result(run_dir: &Path, scenario_id: &str, status: &str) {
    let scenario_dir = run_dir.join(format!("evidence/scenarios/{scenario_id}"));
    fs::create_dir_all(&scenario_dir).unwrap();
    fs::write(scenario_dir.join("world-state.json"), "{}\n").unwrap();
    fs::write(scenario_dir.join("frame-stats.json"), "{}\n").unwrap();
    fs::write(scenario_dir.join("input-replay.json"), "{}\n").unwrap();
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

fn write_declared_visual_comparison(run_dir: &Path, id: &str, fixture: &str, index_refs: bool) {
    let mut value: Value = serde_json::from_str(fixture).unwrap();
    let comparison_id = value["comparisonId"].as_str().unwrap().to_string();
    let base = format!("evidence/scenarios/collect-and-exit/visual/{id}");
    rewrite_visual_refs(&mut value, &base);
    if id == "malformed-screenshot" {
        value["before"]["screenshotRef"] = json!(format!("{base}/before.gif"));
        value["evidenceRefs"] = json!([format!("{base}/before.gif"), format!("{base}/after.png")]);
    }
    if index_refs {
        index_visual_refs(run_dir, &value);
    }
    let comparison_path = format!("{base}/visual-comparison.json");
    fs::create_dir_all(run_dir.join(&base)).unwrap();
    fs::write(
        run_dir.join(&comparison_path),
        serde_json::to_string_pretty(&value).unwrap(),
    )
    .unwrap();
    add_evidence_artifact(
        run_dir,
        &format!("visual-gate-{id}"),
        "application/json",
        &comparison_path,
        json!({
            "artifact": "visual_comparison_evidence",
            "gate": "visual",
            "declaredAcceptance": true,
            "scenarioId": "collect-and-exit",
            "checkpointId": "goal-checkpoint",
            "comparisonId": comparison_id
        }),
    )
    .unwrap();
}

fn rewrite_visual_refs(value: &mut Value, base: &str) {
    for side in ["before", "after"] {
        if let Some(obj) = value[side].as_object_mut() {
            if obj.get("screenshotRef").is_some() {
                obj.insert(
                    "screenshotRef".to_string(),
                    json!(format!("{base}/{side}.png")),
                );
            }
            if obj.get("metadataRef").is_some() {
                obj.insert(
                    "metadataRef".to_string(),
                    json!(format!("{base}/{side}.metadata.json")),
                );
            }
        }
    }
    let mut refs = Vec::new();
    for side in ["before", "after"] {
        if let Some(screenshot) = value[side]["screenshotRef"].as_str() {
            refs.push(json!(screenshot));
        }
    }
    value["evidenceRefs"] = Value::Array(refs);
    let mut metadata = Vec::new();
    for side in ["before", "after"] {
        if let Some(metadata_ref) = value[side]["metadataRef"].as_str() {
            metadata.push(json!(metadata_ref));
        }
    }
    value["metadataRefs"] = Value::Array(metadata);
}

fn index_visual_refs(run_dir: &Path, value: &Value) {
    let comparison_id = value["comparisonId"].as_str().unwrap();
    let run_id = value["runId"].as_str().unwrap();
    let scenario_id = value["scenarioId"].as_str().unwrap();
    let checkpoint_id = value["checkpointId"].as_str().unwrap();
    let mut refs = Vec::new();
    for key in ["evidenceRefs", "metadataRefs"] {
        if let Some(values) = value[key].as_array() {
            refs.extend(values.iter().filter_map(|v| v.as_str()).map(str::to_string));
        }
    }
    refs.sort();
    refs.dedup();
    for reference in refs {
        fs::create_dir_all(run_dir.join(Path::new(&reference).parent().unwrap())).unwrap();
        if reference.ends_with(".json") {
            let side = if reference.contains("before") || reference.contains("baseline") {
                "before"
            } else {
                "after"
            };
            let width = value[side]["width"].as_u64().unwrap_or(320);
            let height = value[side]["height"].as_u64().unwrap_or(180);
            let format = value[side]["format"].as_str().unwrap_or("png");
            fs::write(
                run_dir.join(&reference),
                serde_json::to_string_pretty(&json!({
                    "runId": run_id,
                    "scenarioId": scenario_id,
                    "checkpointId": checkpoint_id,
                    "width": width,
                    "height": height,
                    "format": format
                }))
                .unwrap(),
            )
            .unwrap();
        } else {
            fs::write(run_dir.join(&reference), test_png_bytes(320, 180)).unwrap();
        }
        add_evidence_artifact(
            run_dir,
            &format!("visual-ref-{comparison_id}-{}", reference.replace('/', "-")),
            if reference.ends_with(".json") {
                "application/json"
            } else {
                "image/png"
            },
            &reference,
            json!({"artifact":"visual_gate_fixture_ref"}),
        )
        .unwrap();
    }
}

fn state_for(gates: &[ouroforge_core::VisualGateVerdict], comparison_id: &str) -> String {
    let gate = gates
        .iter()
        .find(|gate| gate.comparison_ref.contains(comparison_id))
        .unwrap();
    serde_json::to_value(gate.state)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn test_png_bytes(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"\x89PNG\r\n\x1a\n");
    bytes.extend_from_slice(&13u32.to_be_bytes());
    bytes.extend_from_slice(b"IHDR");
    bytes.extend_from_slice(&width.to_be_bytes());
    bytes.extend_from_slice(&height.to_be_bytes());
    bytes.extend_from_slice(&[8, 6, 0, 0, 0]);
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes
}

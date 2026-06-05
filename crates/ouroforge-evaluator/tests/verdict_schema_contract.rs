use ouroforge_evaluator::{
    EvaluationVerdict, RuntimeInvariantType, SemanticGateState, SemanticGateVerdict,
    VisualGateState, VisualGateVerdict,
};
use serde_json::json;

#[test]
fn verdict_schema_preserves_existing_gate_field_names() {
    let verdict = EvaluationVerdict {
        status: "failed".to_string(),
        summary: "fixture".to_string(),
        failures: vec![json!({"kind":"visual_gate_failed"})],
        evidence_refs: vec!["evidence/index.json".to_string()],
        metadata: json!({"evaluator":"ouroforge-evaluator-v0"}),
        gate_categories: Some(json!({"visual":{"declared":true,"status":"fail"}})),
        visual: vec![VisualGateVerdict {
            scenario_id: "scenario".to_string(),
            checkpoint_id: "checkpoint".to_string(),
            state: VisualGateState::Fail,
            reason: "changed".to_string(),
            comparison_ref: "evidence/scenarios/scenario/visual/checkpoint/visual-comparison.json"
                .to_string(),
            changed_pixels: Some(7),
            changed_percent_x1000: Some(11),
            changed_region_count: 1,
            threshold_summary: vec!["pixels <= 1".to_string()],
            evidence_refs: vec![
                "evidence/scenarios/scenario/visual/checkpoint/visual-comparison.json".to_string(),
            ],
            output_root: "evidence/scenarios/scenario/visual/checkpoint".to_string(),
        }],
        semantic: vec![SemanticGateVerdict {
            scenario_id: "scenario".to_string(),
            model_id: "model".to_string(),
            invariant_id: "health".to_string(),
            invariant_type: Some(RuntimeInvariantType::HealthNonNegative),
            state: SemanticGateState::Fail,
            reason: "negative health".to_string(),
            model_ref: "evidence/scenarios/scenario/semantic/model/runtime-invariant-model.json"
                .to_string(),
            world_state_ref: Some("evidence/scenarios/scenario/world-state.json".to_string()),
            target_path: Some("player.health".to_string()),
            evidence_refs: vec!["evidence/scenarios/scenario/world-state.json".to_string()],
        }],
    };

    let value = serde_json::to_value(verdict).expect("verdict serializes");
    assert_eq!(value["gateCategories"]["visual"]["status"], "fail");
    assert_eq!(value["visual"][0]["scenarioId"], "scenario");
    assert_eq!(value["visual"][0]["checkpointId"], "checkpoint");
    assert_eq!(value["visual"][0]["state"], "fail");
    assert_eq!(
        value["visual"][0]["comparisonRef"],
        "evidence/scenarios/scenario/visual/checkpoint/visual-comparison.json"
    );
    assert_eq!(value["visual"][0]["changedPixels"], 7);
    assert_eq!(value["visual"][0]["changedPercentX1000"], 11);
    assert_eq!(value["visual"][0]["changedRegionCount"], 1);
    assert_eq!(value["visual"][0]["thresholdSummary"][0], "pixels <= 1");
    assert_eq!(
        value["visual"][0]["evidenceRefs"][0],
        "evidence/scenarios/scenario/visual/checkpoint/visual-comparison.json"
    );
    assert_eq!(
        value["visual"][0]["outputRoot"],
        "evidence/scenarios/scenario/visual/checkpoint"
    );
    assert_eq!(value["semantic"][0]["modelId"], "model");
    assert_eq!(value["semantic"][0]["invariantId"], "health");
    assert_eq!(value["semantic"][0]["invariantType"], "health_non_negative");
    assert_eq!(value["semantic"][0]["state"], "fail");
    assert_eq!(
        value["semantic"][0]["modelRef"],
        "evidence/scenarios/scenario/semantic/model/runtime-invariant-model.json"
    );
    assert_eq!(
        value["semantic"][0]["worldStateRef"],
        "evidence/scenarios/scenario/world-state.json"
    );
    assert_eq!(value["semantic"][0]["targetPath"], "player.health");
}

#[test]
fn evaluator_crate_owns_top_level_run_verdict_writer() {
    let run_dir = std::env::temp_dir().join(format!(
        "ouroforge-evaluator-run-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(run_dir.join("evidence/scenarios/bootstrap"))
        .expect("run evidence dir writes");
    std::fs::write(
        run_dir.join("evidence/scenarios/bootstrap/world-state.json"),
        "{}\n",
    )
    .expect("world state writes");
    std::fs::write(
        run_dir.join("evidence/scenarios/bootstrap/frame-stats.json"),
        "{}\n",
    )
    .expect("frame stats writes");
    std::fs::write(
        run_dir.join("evidence/scenarios/bootstrap/scenario-result.json"),
        serde_json::to_string_pretty(&json!({
            "scenario_id": "bootstrap",
            "status": "passed",
            "assertions": [],
            "evidence": {
                "world_state": "evidence/scenarios/bootstrap/world-state.json",
                "frame_stats": "evidence/scenarios/bootstrap/frame-stats.json"
            }
        }))
        .expect("scenario serializes"),
    )
    .expect("scenario writes");
    std::fs::write(
        run_dir.join("evidence/index.json"),
        serde_json::to_string_pretty(&json!({
            "artifacts": [{
                "id": "bootstrap-result",
                "kind": "application/json",
                "path": "evidence/scenarios/bootstrap/scenario-result.json",
                "metadata": { "artifact": "scenario_result" },
                "added_at_unix_ms": 1
            }]
        }))
        .expect("index serializes"),
    )
    .expect("index writes");

    let verdict = ouroforge_evaluator::evaluate_run_with_config(&run_dir, None)
        .expect("evaluator crate evaluates run");

    assert_eq!(verdict.status, "passed");
    assert_eq!(verdict.metadata["evaluator"], "ouroforge-evaluator-v0");
    assert!(run_dir.join("verdict.json").is_file());
    std::fs::remove_dir_all(run_dir).expect("temp run cleanup");
}

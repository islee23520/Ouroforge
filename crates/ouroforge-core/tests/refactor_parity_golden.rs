//! Refactor Parity Golden Baseline (#1302, Foundation Hardening v1 / #1301).
//!
//! Establishes a byte-level golden baseline of representative deterministic
//! demo-run `verdict.json` outputs *before* any crate extraction. Every
//! Foundation Hardening extraction PR (#1303 ledger / #1304 evidence /
//! #1305 evaluator) must keep these snapshots byte-identical, proving the
//! mechanical refactor changed no runtime behavior.
//!
//! Golden snapshots live under `examples/refactor-parity-golden-v1/` and are
//! the only committed, fixture-scoped artifacts. Generated run directories are
//! written under the system temp dir and removed after each case.
//!
//! Determinism guard: each case is generated twice into two independent temp
//! run directories and the two `verdict.json` byte streams must match exactly,
//! proving the output carries no timestamp, absolute path, or process-id leak.
//! Set `OUROFORGE_PARITY_CAPTURE=1` to (re)write the committed golden from the
//! current engine output; the captured bytes are exactly what the engine emits
//! and are never normalized.

use ouroforge_core::{add_evidence_artifact, create_run, evaluate_run};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: refactor.parity.golden.v1
title: Refactor Parity Golden Baseline Harness
goal: Pin deterministic evaluator verdict bytes before crate extraction.
constraints:
  target: local-fixture
acceptance:
  - Verdict bytes are deterministic and fixture-scoped.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed deterministic acceptance scenario.
"#;

/// Representative deterministic demo runs whose `verdict.json` is stable.
const CASES: &[&str] = &["mechanical-pass", "mechanical-fail", "visual-gates"];

#[test]
fn refactor_parity_golden_verdicts_are_deterministic_and_match_baseline() {
    let capture = std::env::var("OUROFORGE_PARITY_CAPTURE").is_ok();
    for case in CASES {
        // Generate the same case into two independent run directories.
        let first = generate_verdict(case, "a");
        let second = generate_verdict(case, "b");
        assert_eq!(
            first, second,
            "case `{case}` verdict.json is not deterministic across runs \
             (timestamp/path/process leak?)"
        );

        let golden_path = golden_path(case);
        if capture {
            fs::write(&golden_path, &first).unwrap();
            continue;
        }

        let golden = fs::read_to_string(&golden_path).unwrap_or_else(|err| {
            panic!(
                "missing golden snapshot {} ({err}); run with \
                 OUROFORGE_PARITY_CAPTURE=1 to create it",
                golden_path.display()
            )
        });
        assert_eq!(
            first,
            golden,
            "case `{case}` verdict.json drifted from the committed golden baseline \
             {}; an extraction PR must not change verdict bytes",
            golden_path.display()
        );
    }
}

fn golden_path(case: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/refactor-parity-golden-v1")
        .join(format!("{case}.verdict.golden.json"))
}

/// Build a deterministic fixture run for `case`, evaluate it, and return the
/// produced `verdict.json` bytes. The run directory is removed before return.
fn generate_verdict(case: &str, slot: &str) -> String {
    let root = std::env::temp_dir().join(format!(
        "ouroforge-parity-{case}-{slot}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let seed_path = root.join("seed.yaml");
    fs::write(&seed_path, SEED).unwrap();
    let run_dir = create_run(&seed_path, root.join("runs")).unwrap().run_dir;

    match case {
        "mechanical-pass" => write_scenario_result(&run_dir, "collect-and-exit", "passed"),
        "mechanical-fail" => write_scenario_result(&run_dir, "collect-and-exit", "failed"),
        "visual-gates" => {
            write_scenario_result(&run_dir, "collect-and-exit", "passed");
            for (id, fixture) in [
                (
                    "pass-unchanged",
                    include_str!(
                        "../../../examples/evaluator-depth-v1/visual/visual-gate-pass-unchanged.json"
                    ),
                ),
                (
                    "fail-changed",
                    include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-fail-changed-over-threshold.json"),
                ),
                (
                    "missing-baseline",
                    include_str!("../../../examples/evaluator-depth-v1/visual/visual-gate-missing-baseline.json"),
                ),
            ] {
                write_declared_visual_comparison(&run_dir, id, fixture);
            }
        }
        other => panic!("unknown parity case `{other}`"),
    }

    let verdict = evaluate_run(&run_dir).expect("evaluation completes without panics");
    // Sanity: the representative set covers both terminal statuses.
    match case {
        "mechanical-pass" => assert_eq!(verdict.status, "passed"),
        _ => assert_eq!(verdict.status, "failed"),
    }
    let bytes = fs::read_to_string(run_dir.join("verdict.json")).unwrap();
    fs::remove_dir_all(&root).unwrap();
    bytes
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

fn write_declared_visual_comparison(run_dir: &Path, id: &str, fixture: &str) {
    let mut value: Value = serde_json::from_str(fixture).unwrap();
    let comparison_id = value["comparisonId"].as_str().unwrap().to_string();
    let base = format!("evidence/scenarios/collect-and-exit/visual/{id}");
    rewrite_visual_refs(&mut value, &base);
    index_visual_refs(run_dir, &value);
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

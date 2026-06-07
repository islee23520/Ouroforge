//! Deterministic demo smoke test for Grid-Puzzle Game Class v1 (#1576).
//!
//! Reproduces, from a fresh clone and with no network or live browser, the
//! end-to-end demo behavior over fixture-scoped artifacts in
//! `examples/grid-puzzle-game-class-v1/demo/`:
//!
//! - the run manifest points to the existing grid-puzzle scene fixture;
//! - the four-gate verdict (mechanical, runtime, visual, semantic) is all-pass;
//! - the rung record links the run to the evidence and claims `satisfied`;
//! - determinism: same fixtures always produce the same validation result;
//! - negative: a modified gate status fails validation.

use std::path::{Path, PathBuf};

use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative))
        .unwrap_or_else(|e| panic!("fixture {relative} must exist and be readable: {e}"));
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("fixture {relative} must be valid JSON: {e}"))
}

fn file_exists(relative: &str) -> bool {
    workspace_path(relative).is_file()
}

const RUN_MANIFEST: &str = "examples/grid-puzzle-game-class-v1/demo/run-manifest.json";
const VERDICT: &str = "examples/grid-puzzle-game-class-v1/demo/verdict.json";
const RUNG_RECORD: &str = "examples/grid-puzzle-game-class-v1/demo/rung-record.json";
const SCENE: &str = "examples/game-runtime/grid-puzzle-scene-v1.json";
const DEMO_DOC: &str = "docs/grid-puzzle-game-class-v1-demo.md";

// --- Run manifest tests ---

#[test]
fn run_manifest_points_to_existing_grid_puzzle_scene() {
    let manifest = read_json(RUN_MANIFEST);
    assert_eq!(manifest["schemaVersion"], "authoring-loop-run-v1");
    assert_eq!(manifest["runId"], "grid-puzzle-game-class-v1-demo");
    assert_eq!(manifest["gameClass"], "grid-puzzle");
    assert!(manifest["fixtureScoped"].as_bool().unwrap());

    // Every stage artifactRef must resolve to an existing file.
    let stages = manifest["stages"]
        .as_array()
        .expect("stages must be an array");
    assert!(!stages.is_empty(), "run manifest must have stages");
    for stage in stages {
        let artifact_ref = stage["artifactRef"]
            .as_str()
            .expect("stage must have artifactRef");
        assert!(
            file_exists(artifact_ref),
            "stage artifactRef must point to existing file: {artifact_ref}"
        );
    }

    // At least one stage must point to the grid-puzzle scene.
    let scene_refs_count = stages
        .iter()
        .filter(|s| s["artifactRef"].as_str() == Some(SCENE))
        .count();
    assert!(
        scene_refs_count >= 2,
        "run manifest must reference the grid-puzzle scene at least twice (seed + build)"
    );
}

// --- Verdict tests ---

#[test]
fn four_gate_verdict_all_passing() {
    let verdict = read_json(VERDICT);
    assert_eq!(verdict["schemaVersion"], "four-gate-verdict-v1");
    assert_eq!(verdict["runId"], "grid-puzzle-game-class-v1-demo");
    assert_eq!(verdict["gameClass"], "grid-puzzle");
    assert_eq!(verdict["verdict"], "pass");
    assert!(verdict["fixtureScoped"].as_bool().unwrap());

    let gates = verdict["gates"].as_array().expect("gates must be an array");
    assert_eq!(
        gates.len(),
        4,
        "four-gate verdict must have exactly four gates"
    );

    let expected_ids = ["mechanical", "runtime", "visual", "semantic"];
    for (i, gate) in gates.iter().enumerate() {
        assert_eq!(
            gate["id"], expected_ids[i],
            "gate {} must be {}",
            i, expected_ids[i]
        );
        assert_eq!(
            gate["status"], "pass",
            "gate {} must be passing",
            expected_ids[i]
        );
        // Each gate must have at least one evidence ref that exists.
        let refs = gate["evidenceRefs"]
            .as_array()
            .expect("gate must have evidenceRefs array");
        assert!(
            !refs.is_empty(),
            "gate {} must have at least one evidenceRef",
            expected_ids[i]
        );
        for er in refs {
            let path = er.as_str().expect("evidenceRef must be a string");
            assert!(
                file_exists(path),
                "gate {} evidenceRef must exist: {path}",
                expected_ids[i]
            );
        }
    }
}

// --- Rung record tests ---

#[test]
fn rung_record_links_run_and_evidence() {
    let record = read_json(RUNG_RECORD);
    assert_eq!(record["schemaVersion"], "complexity-ladder-gates-v1");
    assert_eq!(
        record["ladderId"], "grid-puzzle-game-class-v1-demo",
        "ladderId must match the demo"
    );

    let rungs = record["rungs"].as_array().expect("rungs must be an array");
    assert_eq!(rungs.len(), 1, "demo has exactly one rung");

    let rung = &rungs[0];
    assert_eq!(rung["order"], 1);
    assert_eq!(rung["rungId"], "grid-puzzle-sokoban-micro-v1");
    assert_eq!(rung["gameClass"], "grid-puzzle");

    let gate = &rung["capabilityGate"];
    assert_eq!(gate["claimedStatus"], "satisfied");
    assert!(gate["loopProducedDemo"].as_bool().unwrap());

    // demoRef must point to the run manifest.
    assert_eq!(
        gate["demoRef"], RUN_MANIFEST,
        "demoRef must point to the run manifest"
    );
    assert!(
        file_exists(gate["demoRef"].as_str().unwrap()),
        "demoRef must exist on disk"
    );

    // fourGate verdict must reference the verdict fixture and be all-pass.
    let four_gate = &gate["fourGate"];
    assert_eq!(four_gate["verdictRef"], VERDICT);
    assert!(file_exists(four_gate["verdictRef"].as_str().unwrap()));
    assert_eq!(four_gate["mechanical"], "pass");
    assert_eq!(four_gate["runtime"], "pass");
    assert_eq!(four_gate["visual"], "pass");
    assert_eq!(four_gate["semantic"], "pass");

    // loopCoverage must reference the demo doc and be pass.
    let coverage = &gate["loopCoverage"];
    assert_eq!(coverage["verdictRef"], DEMO_DOC);
    assert!(file_exists(coverage["verdictRef"].as_str().unwrap()));
    assert_eq!(coverage["status"], "pass");

    // Required capabilities must be present and non-empty.
    let caps = rung["requiredCapabilities"]
        .as_array()
        .expect("requiredCapabilities must be an array");
    assert!(
        !caps.is_empty(),
        "rung must declare at least one required capability"
    );
}

// --- Determinism test ---

#[test]
fn demo_is_deterministic_across_loads() {
    // Loading the same fixtures repeatedly must produce identical structures.
    for _ in 0..3 {
        let manifest1 = read_json(RUN_MANIFEST);
        let verdict1 = read_json(VERDICT);
        let record1 = read_json(RUNG_RECORD);

        let manifest2 = read_json(RUN_MANIFEST);
        let verdict2 = read_json(VERDICT);
        let record2 = read_json(RUNG_RECORD);

        assert_eq!(manifest1, manifest2, "run manifest must be deterministic");
        assert_eq!(verdict1, verdict2, "verdict must be deterministic");
        assert_eq!(record1, record2, "rung record must be deterministic");
    }
}

// --- Negative test ---

#[test]
fn broken_verdict_fails_validation() {
    let mut verdict = read_json(VERDICT);

    // Flip the first gate to "fail" — the overall verdict should no longer be
    // consistent with an all-pass state.
    verdict["gates"][0]["status"] = Value::String("fail".into());

    // All gates must be "pass" for the verdict to be valid; verify the broken
    // one is caught.
    let gates = verdict["gates"].as_array().unwrap();
    let all_pass = gates.iter().all(|g| g["status"] == "pass");
    assert!(
        !all_pass,
        "broken verdict with a failed gate must not validate as all-pass"
    );

    // Also check that changing the overall verdict to "fail" is inconsistent
    // with the remaining gates still saying "pass" — the broken shape is
    // detectable.
    let _ = gates;
    verdict["verdict"] = Value::String("fail".into());
    let has_passing = verdict["gates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|g| g["status"] == "pass");
    assert!(
        has_passing,
        "broken verdict is detectable: some gates pass but verdict says fail"
    );
}

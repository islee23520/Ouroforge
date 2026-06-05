use ouroforge_core::{
    add_evidence_artifact, compare_runs, create_run, evolve_run, update_journal,
    write_run_comparison_artifact,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

const SEED: &str = r#"
id: evolve.rerun.comparison.contract
title: Evolve Rerun Comparison Contract
goal: Prove four-gate before/after deltas and journal mutation-loop summaries.
constraints:
  target: local-fixture
acceptance:
  - Rerun comparisons link gate deltas to evidence and proposal context.
scenarios:
  - id: collect-and-exit
    description: Fixture-backed rerun comparison scenario.
"#;

#[test]
fn rerun_comparison_records_four_gate_deltas_and_journal_summary() {
    let (root, before_run) = create_fixture_run("evolve-rerun-before");
    let after_root = root.join("after-root");
    fs::create_dir_all(&after_root).unwrap();
    let (_, after_run) = create_fixture_run_under(&after_root, "evolve-rerun-after");

    write_scenario_result(&before_run, "failed");
    write_gate_evidence(&before_run, "visual", "visual-before", "fail");
    write_gate_evidence(&before_run, "semantic", "semantic-before", "fail");
    write_failed_verdict(&before_run);
    let summary = evolve_run(&before_run).expect("failed run creates mutation proposal");
    assert_eq!(summary.status, "proposed");

    write_scenario_result(&after_run, "passed");
    write_gate_evidence(&after_run, "visual", "visual-after", "pass");
    write_gate_evidence(&after_run, "semantic", "semantic-after", "pass");
    write_passed_verdict(&after_run);

    let comparison = compare_runs(&before_run, &after_run).expect("comparison");
    assert_eq!(comparison.classification, "improved");
    assert_eq!(comparison.comparability.state, "comparable");
    assert_gate_delta(
        &comparison.four_gate_deltas,
        "visual",
        "fail",
        "pass",
        "fail_to_pass",
        "visual-comparison.json",
    );
    assert_gate_delta(
        &comparison.four_gate_deltas,
        "semantic",
        "fail",
        "pass",
        "fail_to_pass",
        "runtime-invariant-model.json",
    );
    assert_gate_delta(
        &comparison.four_gate_deltas,
        "mechanical",
        "fail",
        "pass",
        "fail_to_pass",
        "scenario-result.json",
    );

    let comparison_path =
        write_run_comparison_artifact(&before_run, &after_run, before_run.join("mutation"))
            .expect("comparison writes");
    let comparison_json: Value =
        serde_json::from_str(&fs::read_to_string(&comparison_path).unwrap())
            .expect("comparison json parses");
    assert_eq!(
        comparison_json["fourGateDeltas"].as_array().unwrap().len(),
        4
    );
    assert_eq!(comparison_json["comparability"]["state"], "comparable");

    let journal = update_journal(&before_run).expect("journal updates");
    assert!(journal.contains("Next-step hypothesis"));
    assert!(journal.contains("Evidence-linked gate: `visual`"));
    assert!(journal.contains("proposal `mutation-1`"));
    assert!(journal.contains("rerun delta `visual`: `fail_to_pass`"));
    assert!(journal.contains("rerun delta `semantic`: `fail_to_pass`"));

    fs::remove_dir_all(root).ok();
}

#[test]
fn rerun_comparison_fails_for_missing_required_after_evidence() {
    let (root, before_run) = create_fixture_run("evolve-rerun-missing-before");
    let after_root = root.join("after-root");
    fs::create_dir_all(&after_root).unwrap();
    let (_, after_run) = create_fixture_run_under(&after_root, "evolve-rerun-missing-after");
    write_scenario_result(&before_run, "failed");
    write_failed_verdict(&before_run);
    fs::remove_file(after_run.join("evidence/index.json")).expect("remove required evidence index");

    let error = write_run_comparison_artifact(&before_run, &after_run, before_run.join("mutation"))
        .expect_err("missing after evidence blocks comparison");

    assert!(error
        .to_string()
        .contains("after run is missing required artifact"));
    fs::remove_dir_all(root).ok();
}

fn assert_gate_delta(
    deltas: &[ouroforge_core::RunGateDelta],
    gate: &str,
    before: &str,
    after: &str,
    transition: &str,
    evidence_substring: &str,
) {
    let delta = deltas
        .iter()
        .find(|delta| delta.gate == gate)
        .unwrap_or_else(|| panic!("missing gate {gate}"));
    assert_eq!(delta.before_status, before);
    assert_eq!(delta.after_status, after);
    assert_eq!(delta.transition, transition);
    assert!(delta
        .before_evidence_refs
        .iter()
        .chain(delta.after_evidence_refs.iter())
        .any(|value| value.contains(evidence_substring)));
}

fn create_fixture_run(prefix: &str) -> (PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!("ouroforge-{prefix}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    create_fixture_run_under(&root, prefix)
}

fn create_fixture_run_under(root: &Path, _prefix: &str) -> (PathBuf, PathBuf) {
    fs::create_dir_all(root).unwrap();
    let seed_path = root.join("seed.yaml");
    fs::write(&seed_path, SEED).unwrap();
    let artifacts = create_run(&seed_path, root.join("runs")).unwrap();
    (root.to_path_buf(), artifacts.run_dir)
}

fn write_scenario_result(run_dir: &Path, status: &str) {
    let rel = "evidence/scenarios/collect-and-exit/scenario-result.json";
    fs::create_dir_all(run_dir.join("evidence/scenarios/collect-and-exit")).unwrap();
    fs::write(
        run_dir.join(rel),
        serde_json::to_vec_pretty(&json!({
            "scenario_id": "collect-and-exit",
            "status": status,
            "assertions": [{ "kind": "fixture", "passed": status == "passed" }],
            "evidence": {}
        }))
        .unwrap(),
    )
    .unwrap();
    add_evidence_artifact(
        run_dir,
        "scenario-result",
        "application/json",
        rel,
        json!({"artifact":"scenario_result"}),
    )
    .ok();
}

fn write_gate_evidence(run_dir: &Path, gate: &str, id: &str, state: &str) -> String {
    let file_name = if gate == "visual" {
        "visual-comparison.json"
    } else {
        "runtime-invariant-model.json"
    };
    let rel = format!("evidence/scenarios/collect-and-exit/{gate}/{id}/{file_name}");
    fs::create_dir_all(run_dir.join(&rel).parent().unwrap()).unwrap();
    fs::write(
        run_dir.join(&rel),
        serde_json::to_vec_pretty(&json!({
            "id": id,
            "gate": gate,
            "state": state,
            "artifact": if gate == "visual" { "visual_comparison_evidence" } else { "runtime_invariant_model" }
        }))
        .unwrap(),
    )
    .unwrap();
    add_evidence_artifact(
        run_dir,
        id,
        "application/json",
        &rel,
        json!({"artifact": if gate == "visual" { "visual_comparison_evidence" } else { "runtime_invariant_model" }, "gate": gate}),
    )
    .ok();
    rel
}

fn write_failed_verdict(run_dir: &Path) {
    let visual = "evidence/scenarios/collect-and-exit/visual/visual-before/visual-comparison.json";
    let semantic =
        "evidence/scenarios/collect-and-exit/semantic/semantic-before/runtime-invariant-model.json";
    write_verdict(run_dir, "failed", "fail", "fail", visual, semantic);
}

fn write_passed_verdict(run_dir: &Path) {
    let visual = "evidence/scenarios/collect-and-exit/visual/visual-after/visual-comparison.json";
    let semantic =
        "evidence/scenarios/collect-and-exit/semantic/semantic-after/runtime-invariant-model.json";
    write_verdict(run_dir, "passed", "pass", "pass", visual, semantic);
}

fn write_verdict(
    run_dir: &Path,
    status: &str,
    visual_state: &str,
    semantic_state: &str,
    visual_ref: &str,
    semantic_ref: &str,
) {
    let scenario = "evidence/scenarios/collect-and-exit/scenario-result.json";
    fs::write(
        run_dir.join("verdict.json"),
        serde_json::to_vec_pretty(&json!({
            "status": status,
            "summary": "fixture four-gate verdict",
            "failures": if status == "failed" { vec![json!({"kind":"visual_gate_failed", "path": visual_ref})] } else { Vec::new() },
            "evidence_refs": [scenario, visual_ref, semantic_ref],
            "gateCategories": {
                "aggregation": { "operator": "declared-gate-and", "undeclaredGatePolicy": "neutral" },
                "mechanical": { "declared": true, "status": if status == "passed" { "pass" } else { "fail" } },
                "runtime": { "declared": false, "status": "pass" },
                "visual": { "declared": true, "status": visual_state },
                "semantic": { "declared": true, "status": semantic_state }
            },
            "visual": [{ "state": visual_state, "comparison_ref": visual_ref, "reason": "fixture visual gate" }],
            "semantic": [{ "state": semantic_state, "model_ref": semantic_ref, "evidence_refs": [scenario], "invariant_id": "health-non-negative" }],
            "metadata": {}
        }))
        .unwrap(),
    )
    .unwrap();
}

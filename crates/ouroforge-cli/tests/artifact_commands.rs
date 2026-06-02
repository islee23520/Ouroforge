use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const VALID_SEED: &str = r#"
id: platformer.v0
title: Platformer Harness Seed
goal: Prove the initial Ouroforge run artifact contract.
constraints:
  target: file-harness
acceptance:
  - Validate the seed schema.
scenarios:
  - id: smoke
    description: Create initial run artifacts.
"#;

#[test]
fn ledger_and_evidence_commands_operate_on_run_artifacts() {
    let temp = unique_temp_dir("ouroforge-cli-artifacts-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let seed_path = temp.join("seed.yaml");
    fs::write(&seed_path, VALID_SEED).expect("seed written");

    let run_output = run_cli(&temp, &["run", seed_path.to_str().unwrap()]);
    assert!(run_output.contains("Run created: runs/run-"));
    let run_dir = temp.join(run_output.trim().strip_prefix("Run created: ").unwrap());

    let appended = run_cli(
        &temp,
        &[
            "ledger",
            "append",
            run_dir.to_str().unwrap(),
            "--kind",
            "test.event",
            "--actor",
            "test",
            "--json",
            r#"{"ok":true}"#,
        ],
    );
    assert!(appended.contains(r#""event": "test.event""#));
    assert!(appended.contains(r#""actor": "test""#));

    let ledger = run_cli(&temp, &["ledger", "list", run_dir.to_str().unwrap()]);
    assert!(ledger.contains("run.created"));
    assert!(ledger.contains("test.event"));

    let empty_evidence = run_cli(&temp, &["evidence", "list", run_dir.to_str().unwrap()]);
    assert_eq!(empty_evidence.trim(), "[]");

    let evidence = run_cli(
        &temp,
        &[
            "evidence",
            "add",
            run_dir.to_str().unwrap(),
            "--id",
            "artifact-1",
            "--kind",
            "text/plain",
            "--path",
            "evidence/artifact-1.txt",
            "--json",
            r#"{"source":"integration-test"}"#,
        ],
    );
    assert!(evidence.contains(r#""id": "artifact-1""#));

    let listed_evidence = run_cli(&temp, &["evidence", "list", run_dir.to_str().unwrap()]);
    assert!(listed_evidence.contains("artifact-1"));
    assert!(listed_evidence.contains("integration-test"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn scene_edit_transaction_output_records_success_and_failure() {
    let temp = unique_temp_dir("ouroforge-cli-scene-transaction-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let scene_path = temp.join("scene.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    let success_artifact = temp.join("transactions/success.json");

    let success = run_cli(
        &temp,
        &[
            "scene",
            "edit",
            scene_path.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.transform.x",
            "--value",
            "48",
            "--transaction-output",
            success_artifact.to_str().unwrap(),
        ],
    );
    assert!(success.contains(r#""validationResult""#));
    assert!(success.contains(r#""status": "passed""#));
    let success_json = fs::read_to_string(&success_artifact).expect("success artifact written");
    assert!(success_json.contains(r#""beforeSceneHash""#));
    assert!(success_json.contains(r#""afterSceneHash""#));
    let edited_scene: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).expect("scene read")).unwrap();
    assert_eq!(
        edited_scene.pointer("/entities/0/components/transform/x"),
        Some(&serde_json::json!(48))
    );

    let failure_artifact = temp.join("transactions/failure.json");
    let failure = run_cli_expect_failure(
        &temp,
        &[
            "scene",
            "edit",
            scene_path.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.size.width",
            "--value",
            "0",
            "--transaction-output",
            failure_artifact.to_str().unwrap(),
        ],
    );
    assert!(failure.contains("scene edit transaction failed validation"));
    let failure_json = fs::read_to_string(&failure_artifact).expect("failure artifact written");
    assert!(failure_json.contains(r#""status": "failed""#));
    assert!(!failure_json.contains(r#""afterSceneHash""#));
    let preserved_scene: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).expect("scene read")).unwrap();
    assert_eq!(
        preserved_scene.pointer("/entities/0/components/size/width"),
        Some(&serde_json::json!(16))
    );

    fs::remove_dir_all(temp).ok();
}

#[test]
fn run_command_binds_scene_edit_transaction_to_metadata_ledger_and_journal() {
    let temp = unique_temp_dir("ouroforge-cli-run-transaction-binding-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let seed_path = temp.join("seed.yaml");
    fs::write(&seed_path, VALID_SEED).expect("seed written");
    let scene_path = temp.join("scene.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    let transaction_path = temp.join("transactions/success.json");
    run_cli(
        &temp,
        &[
            "scene",
            "edit",
            scene_path.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.transform.x",
            "--value",
            "48",
            "--transaction-output",
            transaction_path.to_str().unwrap(),
        ],
    );

    let run_output = run_cli(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--transaction",
            transaction_path.to_str().unwrap(),
        ],
    );
    assert!(run_output.contains("Run transaction bound: scene-edit-"));
    let run_dir_line = run_output
        .lines()
        .find(|line| line.starts_with("Run created: "))
        .expect("run created line present");
    let run_dir = temp.join(run_dir_line.strip_prefix("Run created: ").unwrap());
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run.json")).unwrap()).unwrap();
    let provenance = run_json
        .get("transaction_provenance")
        .expect("transaction provenance recorded");
    assert!(provenance["transactionId"]
        .as_str()
        .unwrap()
        .starts_with("scene-edit-"));
    assert_eq!(
        provenance["scenePath"],
        scene_path.to_string_lossy().to_string()
    );

    let ledger = run_cli(&temp, &["ledger", "list", run_dir.to_str().unwrap()]);
    assert!(ledger.contains("run.transaction_bound"));

    run_cli(&temp, &["journal", "update", run_dir.to_str().unwrap()]);
    let journal = run_cli(&temp, &["journal", "show", run_dir.to_str().unwrap()]);
    assert!(journal.contains("## Scene Edit Transaction"));
    assert!(journal.contains("scene-edit-"));

    let failed_transaction_path = temp.join("transactions/failure.json");
    run_cli_expect_failure(
        &temp,
        &[
            "scene",
            "edit",
            scene_path.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.size.width",
            "--value",
            "0",
            "--transaction-output",
            failed_transaction_path.to_str().unwrap(),
        ],
    );
    let failed_run = run_cli_expect_failure(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--transaction",
            failed_transaction_path.to_str().unwrap(),
        ],
    );
    assert!(failed_run.contains("requires a passed transaction"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn compare_command_prints_semantic_reasons_and_writes_artifact() {
    let temp = unique_temp_dir("ouroforge-cli-semantic-compare-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let seed_path = temp.join("seed.yaml");
    fs::write(&seed_path, VALID_SEED).expect("seed written");

    let before_output = run_cli(&temp, &["run", seed_path.to_str().unwrap()]);
    let before_dir = temp.join(
        before_output
            .trim()
            .strip_prefix("Run created: ")
            .expect("before run created"),
    );
    let after_output = run_cli(&temp, &["run", seed_path.to_str().unwrap()]);
    let after_dir = temp.join(
        after_output
            .trim()
            .strip_prefix("Run created: ")
            .expect("after run created"),
    );
    write_cli_scenario_result(&before_dir, "failed");
    write_cli_scenario_result(&after_dir, "passed");
    run_cli(&temp, &["evaluate", before_dir.to_str().unwrap()]);
    run_cli(&temp, &["evaluate", after_dir.to_str().unwrap()]);

    let output_dir = temp.join("comparisons");
    let compare = run_cli(
        &temp,
        &[
            "compare",
            before_dir.to_str().unwrap(),
            after_dir.to_str().unwrap(),
            "--output-dir",
            output_dir.to_str().unwrap(),
        ],
    );

    assert!(compare.contains("Comparison written: "));
    assert!(compare.contains("Semantic reasons:"));
    assert!(compare.contains("[improved] scenario_verdict"));
    assert!(compare.contains("changed from failed to passed"));
    assert!(compare.contains(r#""semantic""#));
    let comparison_artifact = fs::read_dir(&output_dir)
        .expect("comparison output dir")
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("run-comparison-")
        })
        .expect("comparison artifact written");
    let artifact_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(comparison_artifact).expect("artifact reads"))
            .expect("artifact parses");
    assert_eq!(
        artifact_json["semantic"]["schemaVersion"],
        "run-semantic-diff-v1"
    );
    assert_eq!(artifact_json["classification"], "improved");

    fs::remove_dir_all(temp).ok();
}

#[test]
fn mutation_apply_scene_applies_valid_operation_and_rejects_invalid_inputs() {
    let temp = unique_temp_dir("ouroforge-cli-scene-mutation-apply-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let seed_path = temp.join("seed.yaml");
    fs::write(&seed_path, VALID_SEED).expect("seed written");
    let scene_path = temp.join("scene.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    let run_output = run_cli(&temp, &["run", seed_path.to_str().unwrap()]);
    let run_dir = temp.join(
        run_output
            .trim()
            .strip_prefix("Run created: ")
            .expect("run created"),
    );
    run_cli(
        &temp,
        &[
            "evidence",
            "add",
            run_dir.to_str().unwrap(),
            "--id",
            "scene-mutation-evidence",
            "--kind",
            "application/json",
            "--path",
            "evidence/scene-mutation.json",
            "--json",
            r#"{"source":"cli-scene-mutation-test"}"#,
        ],
    );
    let proposal_json = run_cli(
        &temp,
        &[
            "mutation",
            "create",
            run_dir.to_str().unwrap(),
            "--reason",
            "cli scene mutation",
            "--evidence",
            "scene-mutation-evidence",
            "--target",
            scene_path.to_str().unwrap(),
            "--path",
            "components.transform.x",
            "--from",
            "32",
            "--to",
            "48",
        ],
    );
    let proposal: serde_json::Value = serde_json::from_str(&proposal_json).unwrap();
    let proposal_id = proposal["id"].as_str().unwrap();
    let hash_probe_path = temp.join("transactions/hash-probe.json");
    run_cli(
        &temp,
        &[
            "scene",
            "edit",
            scene_path.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.transform.x",
            "--value",
            "32",
            "--transaction-output",
            hash_probe_path.to_str().unwrap(),
        ],
    );
    let hash_probe: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&hash_probe_path).unwrap()).unwrap();
    let before_hash = hash_probe["beforeSceneHash"].clone();

    let success_operation_path = temp.join("operations/success.json");
    write_operation(
        &success_operation_path,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "edit": { "entity_id": "player", "path": "components.transform.x", "value": 48 },
            "expectedBeforeSceneHash": before_hash,
            "validationRequired": true
        }),
    );
    let success_transaction = temp.join("transactions/apply-success.json");
    let apply_output = run_cli(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--operation",
            success_operation_path.to_str().unwrap(),
            "--transaction-output",
            success_transaction.to_str().unwrap(),
        ],
    );
    assert!(apply_output.contains("Scene-only mutation applied: scene-edit-"));
    assert!(apply_output.contains("Next QA command:"));
    assert!(success_transaction.is_file());
    let edited_scene: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).unwrap()).unwrap();
    assert_eq!(
        edited_scene.pointer("/entities/0/components/transform/x"),
        Some(&serde_json::json!(48))
    );
    let ledger = run_cli(&temp, &["ledger", "list", run_dir.to_str().unwrap()]);
    assert!(ledger.contains("mutation.scene_applied"));
    let applications: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_dir.join("mutation/scene-applications.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(applications["applications"][0]["proposalId"], proposal_id);
    assert_eq!(applications["applications"][0]["status"], "applied");

    let updated_hash_probe = temp.join("transactions/hash-after.json");
    run_cli(
        &temp,
        &[
            "scene",
            "edit",
            scene_path.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.transform.x",
            "--value",
            "48",
            "--transaction-output",
            updated_hash_probe.to_str().unwrap(),
        ],
    );
    let updated_hash: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&updated_hash_probe).unwrap()).unwrap();
    let current_hash = updated_hash["beforeSceneHash"].clone();

    let forbidden_operation = temp.join("operations/forbidden.json");
    write_operation(
        &forbidden_operation,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "edit": { "entity_id": "player", "path": "metadata.debug.mode", "value": "unsafe" },
            "expectedBeforeSceneHash": current_hash,
            "validationRequired": true
        }),
    );
    let forbidden_transaction = temp.join("transactions/forbidden.json");
    let forbidden = run_cli_expect_failure(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--operation",
            forbidden_operation.to_str().unwrap(),
            "--transaction-output",
            forbidden_transaction.to_str().unwrap(),
        ],
    );
    assert!(forbidden.contains("edit path is not allowed"));

    let stale_operation = temp.join("operations/stale.json");
    write_operation(
        &stale_operation,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "edit": { "entity_id": "player", "path": "components.transform.x", "value": 64 },
            "expectedBeforeSceneHash": { "algorithm": "fnv1a64-canonical-json-v1", "value": "0000000000000000" },
            "validationRequired": true
        }),
    );
    let stale_transaction = temp.join("transactions/stale.json");
    let stale = run_cli_expect_failure(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--operation",
            stale_operation.to_str().unwrap(),
            "--transaction-output",
            stale_transaction.to_str().unwrap(),
        ],
    );
    assert!(stale.contains("before hash mismatch"));

    let invalid_operation = temp.join("operations/invalid.json");
    write_operation(
        &invalid_operation,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "edit": { "entity_id": "player", "path": "components.size.width", "value": 0 },
            "expectedBeforeSceneHash": current_hash,
            "validationRequired": true
        }),
    );
    let invalid_transaction = temp.join("transactions/invalid.json");
    let invalid = run_cli_expect_failure(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--operation",
            invalid_operation.to_str().unwrap(),
            "--transaction-output",
            invalid_transaction.to_str().unwrap(),
        ],
    );
    assert!(invalid.contains("transaction failed validation"));
    assert!(invalid_transaction.is_file());
    let preserved_scene: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).unwrap()).unwrap();
    assert_eq!(
        preserved_scene.pointer("/entities/0/components/size/width"),
        Some(&serde_json::json!(16))
    );

    fs::remove_dir_all(temp).ok();
}

fn write_operation(path: &Path, value: serde_json::Value) {
    fs::create_dir_all(path.parent().unwrap()).expect("operation dir");
    fs::write(path, serde_json::to_string_pretty(&value).unwrap()).expect("operation written");
}

fn write_cli_scenario_result(run_dir: &Path, status: &str) {
    let scenario_dir = run_dir.join("evidence/scenarios/smoke");
    fs::create_dir_all(&scenario_dir).expect("scenario dir");
    fs::write(
        scenario_dir.join("scenario-result.json"),
        format!("{{\"scenario_id\":\"smoke\",\"status\":\"{status}\",\"assertions\":[]}}"),
    )
    .expect("scenario result written");
    let output = run_cli(
        run_dir.parent().unwrap().parent().unwrap_or(run_dir),
        &[
            "evidence",
            "add",
            run_dir.to_str().unwrap(),
            "--id",
            "scenario-result-smoke",
            "--kind",
            "application/json",
            "--path",
            "evidence/scenarios/smoke/scenario-result.json",
            "--json",
            r#"{"artifact":"scenario_result","scenario_id":"smoke"}"#,
        ],
    );
    assert!(output.contains("scenario-result-smoke"));
}

fn run_cli(current_dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_ouroforge-cli"))
        .current_dir(current_dir)
        .args(args)
        .output()
        .expect("cli runs");
    assert!(
        output.status.success(),
        "command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("stdout is utf8")
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time works")
        .as_millis();
    std::env::temp_dir().join(format!("{prefix}-{}-{millis}", std::process::id()))
}

fn run_cli_expect_failure(current_dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_ouroforge-cli"))
        .current_dir(current_dir)
        .args(args)
        .output()
        .expect("cli runs");
    assert!(
        !output.status.success(),
        "command unexpectedly succeeded\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

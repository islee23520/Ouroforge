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

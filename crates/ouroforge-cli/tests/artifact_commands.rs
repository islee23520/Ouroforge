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
fn loop_handoff_writes_generated_contract_without_executing_allowed_commands() {
    let temp = unique_temp_dir("ouroforge-cli-loop-handoff-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let plan_path = temp.join("loop-plan.json");
    fs::write(
        &plan_path,
        r#"{
  "schemaVersion": "authoring-loop-plan-v1",
  "loopId": "cli-handoff-loop",
  "project": { "projectId": "cli_project", "manifestPath": "ouroforge.project.json" },
  "seed": { "id": "smoke", "path": "seeds/smoke.yaml" },
  "scenarioPack": { "id": "regression", "path": "scenarios/regression.json" },
  "steps": [
    { "id": "run-baseline", "kind": "run-scenario-pack", "status": "completed", "expectedArtifacts": [{ "id": "baseline-run", "path": "dashboard-data/baseline-run.json" }] },
    { "id": "compare-runs", "kind": "compare-runs", "status": "pending", "dependsOn": ["run-baseline"], "inputs": [{ "id": "baseline-run", "path": "dashboard-data/baseline-run.json" }], "expectedArtifacts": [{ "id": "comparison", "path": "runs/comparisons/run-comparison.json" }] }
  ],
  "generatedState": { "roots": ["runs", "target", "dashboard-data"], "trackedFixtureOnly": true }
}"#,
    )
    .expect("plan writes");
    let handoff_path = temp.join("runs/agent-handoffs/cli-handoff-loop/handoff.json");

    let output = run_cli(
        &temp,
        &[
            "loop",
            "handoff",
            plan_path.to_str().unwrap(),
            "--output",
            handoff_path.to_str().unwrap(),
        ],
    );

    assert!(output.contains("Agent handoff written:"));
    let handoff: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&handoff_path).expect("handoff output reads"))
            .expect("handoff parses");
    assert_eq!(handoff["schemaVersion"], "agent-handoff-contract-v1");
    assert_eq!(handoff["currentStep"]["stepId"], "compare-runs");
    assert!(handoff["allowedCommands"]
        .as_array()
        .expect("commands array")
        .iter()
        .all(|command| command["boundary"]
            .as_str()
            .expect("boundary text")
            .contains("inert")));
    assert!(temp
        .join("runs/authoring-loop-ledgers/cli-handoff-loop/ledger.jsonl")
        .is_file());
    let dashboard_output = temp.join("dashboard-data/dashboard-data.json");
    run_cli(
        &temp,
        &[
            "dashboard",
            "export",
            "--runs-root",
            temp.join("runs").to_str().unwrap(),
            "--output",
            dashboard_output.to_str().unwrap(),
        ],
    );
    let dashboard: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&dashboard_output).expect("dashboard output reads"),
    )
    .expect("dashboard parses");
    assert_eq!(dashboard["agent_handoffs"][0]["loopId"], "cli-handoff-loop");
    assert_eq!(
        dashboard["loop_cockpit"]["schemaVersion"],
        "studio-loop-cockpit-v1"
    );
    assert_eq!(
        dashboard["loop_cockpit"]["loops"][0]["loopId"],
        "cli-handoff-loop"
    );
    assert_eq!(
        dashboard["loop_cockpit"]["loops"][0]["currentStep"]["stepId"],
        "compare-runs"
    );
    assert!(dashboard["loop_cockpit"]["boundary"]
        .as_str()
        .expect("cockpit boundary")
        .contains("does not execute commands"));
    assert!(
        !temp.join("runs/comparisons/run-comparison.json").exists(),
        "allowed commands are not executed by handoff generation"
    );

    fs::remove_dir_all(temp).ok();
}

#[test]
fn project_validate_reports_manifest_summary_and_rejects_invalid_manifest() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let valid_root = repo_root.join("examples/project-workspace-fixtures/valid");
    let valid_manifest = valid_root.join("ouroforge.project.json");

    let by_root = run_cli(
        &repo_root,
        &["project", "validate", valid_root.to_str().unwrap()],
    );
    assert!(by_root.contains("Project manifest valid: project_workspace_fixture"));
    assert!(by_root.contains("Source refs: 3"));
    assert!(by_root.contains("Asset roots: 1"));
    assert!(by_root.contains("Scenario packs: 1"));
    assert!(by_root.contains("Runs root: runs"));
    assert!(by_root.contains("Generated roots: runs,target,dashboard-data"));

    let by_file = run_cli(
        &repo_root,
        &["project", "validate", valid_manifest.to_str().unwrap()],
    );
    assert!(by_file.contains("Project manifest valid: project_workspace_fixture"));

    let invalid_manifest = repo_root
        .join("examples/project-workspace-fixtures/invalid/missing-ref/ouroforge.project.json");
    let invalid = run_cli_expect_failure(
        &repo_root,
        &["project", "validate", invalid_manifest.to_str().unwrap()],
    );
    assert!(invalid.contains("missing file"));
    assert!(invalid.contains("scenes/missing.scene.json"));

    let bad_pack = repo_root.join(
        "examples/project-workspace-fixtures/invalid/bad-scenario-pack/ouroforge.project.json",
    );
    let bad_pack_output = run_cli_expect_failure(
        &repo_root,
        &["project", "validate", bad_pack.to_str().unwrap()],
    );
    assert!(bad_pack_output.contains("scenarioPacks ref unsupported failed validation"));
    assert!(bad_pack_output.contains("unknown field"));

    let wrong_name =
        repo_root.join("examples/project-workspace-fixtures/invalid/unsafe-path.project.json");
    let rejected_name = run_cli_expect_failure(
        &repo_root,
        &["project", "validate", wrong_name.to_str().unwrap()],
    );
    assert!(rejected_name.contains("must be named ouroforge.project.json"));
}

#[test]
fn asset_validate_reports_manifest_summary_and_rejects_invalid_assets() {
    let temp = unique_temp_dir("ouroforge-cli-asset-validate-test");
    fs::create_dir_all(temp.join("assets/sprites")).expect("sprite dir");
    fs::create_dir_all(temp.join("runs/previews")).expect("generated dir");
    fs::write(temp.join("assets/sprites/player.png"), b"png fixture").expect("sprite writes");
    fs::write(
        temp.join("runs/previews/player.png"),
        b"generated png fixture",
    )
    .expect("generated writes");
    let sprite_hash = fnv1a64_hex(b"png fixture");
    let generated_hash = fnv1a64_hex(b"generated png fixture");
    let manifest_path = temp.join("asset-manifest.json");
    fs::write(
        &manifest_path,
        format!(
            r#"{{
  "schemaVersion": "asset-manifest-v1",
  "id": "cli_asset_fixture",
  "assets": [
    {{
      "id": "player_sprite",
      "type": "image",
      "path": "assets/sprites/player.png",
      "contentHash": {{ "algorithm": "fnv1a64-file-v1", "value": "{sprite_hash}" }},
      "classification": "source_like",
      "dimensions": {{ "width": 16, "height": 16 }}
    }},
    {{
      "id": "generated_preview",
      "type": "image",
      "path": "runs/previews/player.png",
      "contentHash": {{ "algorithm": "fnv1a64-file-v1", "value": "{generated_hash}" }},
      "classification": "generated",
      "dimensions": {{ "width": 64, "height": 64 }}
    }}
  ]
}}"#
        ),
    )
    .expect("manifest writes");

    let by_root = run_cli(&temp, &["asset", "validate", temp.to_str().unwrap()]);
    assert!(by_root.contains("Asset manifest valid: cli_asset_fixture"));
    assert!(by_root.contains("Assets: 2"));
    assert!(by_root.contains("Source-like assets: 1"));
    assert!(by_root.contains("Generated assets: 1"));
    assert!(by_root.contains("Asset types: image=2"));

    let by_file = run_cli(
        &temp,
        &["asset", "validate", manifest_path.to_str().unwrap()],
    );
    assert!(by_file.contains("Asset manifest valid: cli_asset_fixture"));

    let stale_manifest = temp.join("asset-manifest.stale.json");
    fs::write(
        &stale_manifest,
        r#"{
  "schemaVersion": "asset-manifest-v1",
  "id": "cli_stale_asset_fixture",
  "assets": [{
    "id": "player_sprite",
    "type": "image",
    "path": "assets/sprites/player.png",
    "contentHash": { "algorithm": "fnv1a64-file-v1", "value": "0000000000000000" },
    "classification": "source_like"
  }]
}"#,
    )
    .expect("stale manifest writes");
    let stale = run_cli_expect_failure(
        &temp,
        &["asset", "validate", stale_manifest.to_str().unwrap()],
    );
    assert!(stale.contains("contentHash mismatch"));

    let generated_source_manifest = temp.join("asset-manifest.generated-source.json");
    fs::write(
        &generated_source_manifest,
        format!(
            r#"{{
  "schemaVersion": "asset-manifest-v1",
  "id": "cli_generated_source_fixture",
  "assets": [{{
    "id": "generated_source",
    "type": "image",
    "path": "runs/previews/player.png",
    "contentHash": {{ "algorithm": "fnv1a64-file-v1", "value": "{generated_hash}" }},
    "classification": "source_like"
  }}]
}}"#
        ),
    )
    .expect("generated source manifest writes");
    let generated_source = run_cli_expect_failure(
        &temp,
        &[
            "asset",
            "validate",
            generated_source_manifest.to_str().unwrap(),
        ],
    );
    assert!(generated_source.contains("generated root runs"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn asset_validate_reports_sprite_atlas_read_only_summary() {
    let temp = unique_temp_dir("ouroforge-cli-sprite-atlas-summary-test");
    fs::create_dir_all(temp.join("assets/sprites")).expect("sprite dir");
    fs::create_dir_all(temp.join("assets/atlases")).expect("atlas dir");
    fs::write(
        temp.join("assets/sprites/player-sheet.png"),
        b"sheet fixture",
    )
    .expect("sheet writes");
    fs::write(
        temp.join("assets/atlases/player-sheet.atlas.json"),
        br#"{"frames":["idle_0","idle_1"]}"#,
    )
    .expect("atlas writes");
    let sheet_hash = fnv1a64_hex(b"sheet fixture");
    let atlas_hash = fnv1a64_hex(br#"{"frames":["idle_0","idle_1"]}"#);
    let manifest_path = temp.join("asset-manifest.json");
    fs::write(
        &manifest_path,
        format!(
            r#"{{
  "schemaVersion": "asset-manifest-v1",
  "id": "cli_sprite_atlas_fixture",
  "assets": [
    {{
      "id": "player_sheet_image",
      "type": "image",
      "path": "assets/sprites/player-sheet.png",
      "contentHash": {{ "algorithm": "fnv1a64-file-v1", "value": "{sheet_hash}" }},
      "classification": "source_like",
      "dimensions": {{ "width": 32, "height": 16 }}
    }},
    {{
      "id": "player_sheet_atlas",
      "type": "sprite_atlas",
      "path": "assets/atlases/player-sheet.atlas.json",
      "contentHash": {{ "algorithm": "fnv1a64-file-v1", "value": "{atlas_hash}" }},
      "classification": "source_like",
      "atlas": {{
        "imageAssetId": "player_sheet_image",
        "frames": [
          {{ "id": "idle_0", "rect": {{ "x": 0, "y": 0, "width": 16, "height": 16 }} }},
          {{ "id": "idle_1", "rect": {{ "x": 16, "y": 0, "width": 16, "height": 16 }} }}
        ],
        "animations": [
          {{ "id": "idle", "frames": [
            {{ "frameId": "idle_0", "durationMs": 120 }},
            {{ "frameId": "idle_1", "durationMs": 120 }}
          ]}}
        ]
      }}
    }}
  ]
}}"#
        ),
    )
    .expect("manifest writes");

    let output = run_cli(
        &temp,
        &["asset", "validate", manifest_path.to_str().unwrap()],
    );
    assert!(output.contains("Asset manifest valid: cli_sprite_atlas_fixture"));
    assert!(output.contains("Assets: 2"));
    assert!(output.contains("Sprite atlases: 1"));
    assert!(output.contains("Sprite atlas frames: 2"));
    assert!(output.contains("Sprite atlas animations: 1"));
    assert!(output.contains("Asset types: image=1,sprite_atlas=1"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn project_init_creates_valid_minimal_workspace_and_rejects_unsafe_destinations() {
    let temp = unique_temp_dir("ouroforge-cli-project-init-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let project_dir = temp.join("minimal-project");

    let output = run_cli(
        &temp,
        &[
            "project",
            "init",
            project_dir.to_str().unwrap(),
            "--template",
            "minimal-2d",
        ],
    );
    assert!(output.contains("Project scaffold created:"));
    assert!(output.contains("Template: minimal-2d"));
    assert!(output.contains("Project manifest valid: minimal_2d"));
    assert!(project_dir.join("ouroforge.project.json").is_file());
    assert!(project_dir.join("scenes/main.scene.json").is_file());
    assert!(project_dir.join("seeds/platformer.yaml").is_file());
    assert!(project_dir
        .join("scenarios/smoke.scenario-pack.json")
        .is_file());
    assert!(project_dir.join(".gitignore").is_file());

    let validate = run_cli(
        &temp,
        &["project", "validate", project_dir.to_str().unwrap()],
    );
    assert!(validate.contains("Project manifest valid: minimal_2d"));

    let seed = run_cli(
        &temp,
        &[
            "seed",
            "validate",
            project_dir.join("seeds/platformer.yaml").to_str().unwrap(),
        ],
    );
    assert!(seed.contains("Seed valid: minimal-2d.platformer"));

    let non_empty = run_cli_expect_failure(
        &temp,
        &[
            "project",
            "init",
            project_dir.to_str().unwrap(),
            "--template",
            "minimal-2d",
        ],
    );
    assert!(non_empty.contains("destination must be empty"));

    let traversal = run_cli_expect_failure(
        &temp,
        &["project", "init", "../outside", "--template", "minimal-2d"],
    );
    assert!(traversal.contains("path traversal"));

    let unsupported = run_cli_expect_failure(
        &temp,
        &[
            "project",
            "init",
            temp.join("unsupported").to_str().unwrap(),
            "--template",
            "future-3d",
        ],
    );
    assert!(unsupported.contains("unsupported project template"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn artifact_write_dashboard_export_rejects_project_scene_output_without_writing() {
    let temp = unique_temp_dir("ouroforge-cli-artifact-write-dashboard-scene");
    fs::create_dir_all(temp.join("scenes")).expect("project dirs exist");
    fs::create_dir_all(temp.join("runs")).expect("runs dir exists");
    let scene_path = temp.join("scenes/main.scene.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    let before_contents = fs::read_to_string(&scene_path).expect("scene before reads");

    let failure = run_cli_expect_failure(
        &temp,
        &[
            "dashboard",
            "export",
            "--runs-root",
            temp.join("runs").to_str().unwrap(),
            "--output",
            scene_path.to_str().unwrap(),
        ],
    );

    assert!(
        failure.contains("dashboard export output must not target source-like path"),
        "{failure}"
    );
    assert_eq!(
        fs::read_to_string(&scene_path).expect("scene after reads"),
        before_contents,
        "dashboard export rejection must not clobber the trusted scene"
    );
    serde_json::from_str::<serde_json::Value>(&before_contents).expect("scene remains parseable");
    fs::remove_dir_all(temp).ok();
}

#[test]
fn artifact_write_dashboard_export_allows_generated_dashboard_data_overwrite() {
    let temp = unique_temp_dir("ouroforge-cli-artifact-write-dashboard-generated");
    fs::create_dir_all(temp.join("runs")).expect("runs dir exists");
    let output = temp.join("dashboard-data/dashboard-data.json");
    fs::create_dir_all(output.parent().expect("output parent")).expect("output dir exists");
    fs::write(&output, "old generated dashboard data").expect("existing generated output");

    let exported = run_cli(
        &temp,
        &[
            "dashboard",
            "export",
            "--runs-root",
            temp.join("runs").to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ],
    );

    assert!(exported.contains("Dashboard data exported"));
    let payload: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).expect("dashboard output reads"))
            .expect("dashboard output is json");
    assert_eq!(payload["schema"], "ouroforge-dashboard-v1");
    fs::remove_dir_all(temp).ok();
}

#[test]
fn dashboard_export_includes_regression_run_matrix_read_model() {
    let temp = unique_temp_dir("ouroforge-cli-dashboard-regression-matrix");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let (_project_dir, run_dir) = create_project_bound_run(&temp);
    write_failed_regression_evidence(&temp, &run_dir, "scaffold-smoke", true);
    let output = temp.join("dashboard-data/dashboard-data.json");

    let exported = run_cli(
        &temp,
        &[
            "dashboard",
            "export",
            "--runs-root",
            temp.join("runs").to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ],
    );

    assert!(exported.contains("Dashboard data exported"));
    let payload: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output).expect("dashboard output reads"))
            .expect("dashboard output parses");
    assert_eq!(payload["schema"], "ouroforge-dashboard-v1");
    assert_eq!(
        payload["regression_matrix"]["schemaVersion"],
        "ouroforge-regression-run-matrix-v1"
    );
    assert_eq!(
        payload["regression_matrix"]["projects"][0]["projectId"],
        "minimal_2d"
    );
    let scenarios = payload["regression_matrix"]["projects"][0]["scenarioPacks"][0]["scenarios"]
        .as_array()
        .expect("matrix scenarios array");
    let scaffold = scenarios
        .iter()
        .find(|scenario| scenario["scenarioId"] == "scaffold-smoke")
        .expect("scaffold scenario in matrix");
    assert_eq!(scaffold["currentStatus"], "failed");
    assert_eq!(
        scaffold["lastFail"]["scenarioResultPath"],
        "evidence/scenarios/scaffold-smoke/scenario-result.json"
    );
    assert!(payload["regression_matrix"]["skippedRuns"]
        .as_array()
        .unwrap()
        .is_empty());
    fs::remove_dir_all(temp).ok();
}

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
fn scene_edit_transaction_output_rejects_exact_scene_path_without_writing() {
    let temp = unique_temp_dir("ouroforge-cli-scene-transaction-exact-collision");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let scene_path = temp.join("scene.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    let before_contents = fs::read_to_string(&scene_path).expect("scene before reads");

    let failure = run_cli_expect_failure(
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
            "96",
            "--transaction-output",
            scene_path.to_str().unwrap(),
        ],
    );

    assert!(
        failure.contains("transaction output must not equal target scene path"),
        "{failure}"
    );
    assert_eq!(
        fs::read_to_string(&scene_path).expect("scene after reads"),
        before_contents,
        "exact output rejection must not corrupt the trusted scene"
    );
    serde_json::from_str::<serde_json::Value>(&before_contents).expect("scene remains parseable");
    fs::remove_dir_all(temp).ok();
}

#[test]
fn scene_edit_transaction_output_rejects_hard_link_scene_alias_without_writing() {
    let temp = unique_temp_dir("ouroforge-cli-scene-transaction-hardlink-collision");
    fs::create_dir_all(temp.join("transactions")).expect("temp dirs exist");
    let scene_path = temp.join("scene.json");
    let hard_link_path = temp.join("transactions/scene-hardlink.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    fs::hard_link(&scene_path, &hard_link_path).expect("hard link to scene can be created");
    let before_contents = fs::read_to_string(&scene_path).expect("scene before reads");

    let failure = run_cli_expect_failure(
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
            "96",
            "--transaction-output",
            hard_link_path.to_str().unwrap(),
        ],
    );

    assert!(
        failure.contains("transaction output must not equal target scene path"),
        "{failure}"
    );
    assert_eq!(
        fs::read_to_string(&scene_path).expect("scene after reads"),
        before_contents,
        "hard-link output rejection must not corrupt the trusted scene"
    );
    assert_eq!(
        fs::read_to_string(&hard_link_path).expect("hard link after reads"),
        before_contents,
        "rejection must happen before writing the transaction artifact"
    );
    serde_json::from_str::<serde_json::Value>(&before_contents).expect("scene remains parseable");
    fs::remove_dir_all(temp).ok();
}

#[cfg(unix)]
#[test]
fn scene_edit_transaction_output_rejects_symlink_scene_alias_without_writing() {
    use std::os::unix::fs::symlink;

    let temp = unique_temp_dir("ouroforge-cli-scene-transaction-symlink-collision");
    fs::create_dir_all(temp.join("transactions")).expect("temp dirs exist");
    let scene_path = temp.join("scene.json");
    let symlink_path = temp.join("transactions/scene-symlink.json");
    fs::write(
        &scene_path,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("scene written");
    symlink(&scene_path, &symlink_path).expect("symlink to scene can be created");
    let before_contents = fs::read_to_string(&scene_path).expect("scene before reads");

    let failure = run_cli_expect_failure(
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
            "96",
            "--transaction-output",
            symlink_path.to_str().unwrap(),
        ],
    );

    assert!(
        failure.contains("transaction output must not equal target scene path"),
        "{failure}"
    );
    assert_eq!(
        fs::read_to_string(&scene_path).expect("scene after reads"),
        before_contents,
        "symlink output rejection must not corrupt the trusted scene"
    );
    assert_eq!(
        fs::read_to_string(&symlink_path).expect("symlink after reads"),
        before_contents,
        "rejection must happen before writing the transaction artifact"
    );
    serde_json::from_str::<serde_json::Value>(&before_contents).expect("scene remains parseable");
    fs::remove_dir_all(temp).ok();
}

#[test]
fn run_command_binds_validated_project_metadata_and_preflights_invalid_projects() {
    let temp = unique_temp_dir("ouroforge-cli-project-run-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let project_dir = temp.join("project");
    run_cli(
        &temp,
        &[
            "project",
            "init",
            project_dir.to_str().unwrap(),
            "--template",
            "minimal-2d",
        ],
    );
    let seed_path = project_dir.join("seeds/platformer.yaml");
    let run_output = run_cli(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    assert!(run_output.contains("Run project bound: minimal_2d"));
    let run_dir_line = run_output
        .lines()
        .find(|line| line.starts_with("Run created: "))
        .expect("run created line present");
    let run_dir = temp.join(run_dir_line.strip_prefix("Run created: ").unwrap());
    let run_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run.json")).unwrap()).unwrap();
    let project = run_json.get("project").expect("project metadata recorded");
    assert_eq!(project["id"], "minimal_2d");
    assert_eq!(project["seedPath"], "seeds/platformer.yaml");
    assert_eq!(project["scenarioPack"]["id"], "smoke");
    assert_eq!(project["scenarioPack"]["scenarioIds"][0], "scaffold-smoke");
    assert_eq!(project["scenes"][0]["path"], "scenes/main.scene.json");
    assert!(project["manifestHash"]["value"].as_str().unwrap().len() == 16);
    let command_context = run_json
        .get("run_command_context")
        .expect("run command context recorded");
    assert_eq!(command_context["schemaVersion"], "run-command-context-v1");
    assert_eq!(command_context["workers"], 1);
    assert_eq!(
        command_context["projectRoot"],
        project_dir.to_string_lossy().to_string()
    );
    assert_eq!(command_context["scenarioPackId"], "smoke");
    assert!(command_context["command"]
        .as_str()
        .unwrap()
        .contains("--scenario-pack smoke"));
    let ledger = run_cli(&temp, &["ledger", "list", run_dir.to_str().unwrap()]);
    assert!(ledger.contains("run.project_bound"));
    assert!(ledger.contains("run.command_context_recorded"));

    let invalid_root = temp.join("invalid-preflight");
    fs::create_dir_all(invalid_root.join("assets")).expect("invalid assets");
    fs::write(
        invalid_root.join("ouroforge.project.json"),
        r#"{
  "schemaVersion": "project-manifest-v1",
  "project": { "id": "invalid_preflight", "name": "Invalid Preflight" },
  "scenes": [{ "id": "main", "path": "missing.scene.json" }],
  "seeds": [{ "id": "platformer", "path": "seeds/platformer.yaml" }],
  "scenarioPacks": [],
  "assetRoots": ["assets"],
  "runsRoot": "runs",
  "generated": { "roots": ["runs"] }
}
"#,
    )
    .expect("invalid manifest written");
    let before_runs = fs::read_dir(temp.join("runs")).unwrap().count();
    let invalid = run_cli_expect_failure(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--project",
            invalid_root.to_str().unwrap(),
        ],
    );
    assert!(invalid.contains("missing file"));
    let after_runs = fs::read_dir(temp.join("runs")).unwrap().count();
    assert_eq!(
        after_runs, before_runs,
        "invalid project must fail before run creation"
    );

    let scenario_without_project = run_cli_expect_failure(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    assert!(scenario_without_project.contains("--scenario-pack requires --project"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn scenario_promote_draft_generates_evidence_linked_draft_without_pack_write() {
    let temp = unique_temp_dir("ouroforge-cli-regression-promote-draft-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let (project_dir, run_dir) = create_project_bound_run(&temp);
    write_failed_regression_evidence(&temp, &run_dir, "scaffold-smoke", true);
    let pack_path = project_dir.join("scenarios/smoke.scenario-pack.json");
    let pack_before = fs::read_to_string(&pack_path).expect("pack reads before");
    let output_path = temp.join("runs/drafts/scaffold-smoke-draft.json");

    let output = run_cli(
        &temp,
        &[
            "scenario",
            "promote-draft",
            run_dir.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario",
            "scaffold-smoke",
            "--output",
            output_path.to_str().unwrap(),
        ],
    );

    assert!(output.contains("Regression promotion draft:"));
    assert!(output.contains("Scenario: scaffold-smoke"));
    assert!(output.contains("Replay artifact: evidence/scenarios/scaffold-smoke/input-replay.json"));
    let pack_after = fs::read_to_string(&pack_path).expect("pack reads after");
    assert_eq!(
        pack_after, pack_before,
        "promote-draft must not write scenario pack files"
    );
    let draft: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&output_path).expect("draft reads"))
            .expect("draft parses");
    assert_eq!(draft["schemaVersion"], "regression-promotion-draft-v1");
    assert_eq!(draft["sourceRun"]["verdictPath"], "verdict.json");
    assert_eq!(
        draft["sourceEvidence"]["scenarioResultPath"],
        "evidence/scenarios/scaffold-smoke/scenario-result.json"
    );
    assert_eq!(
        draft["sourceEvidence"]["replayArtifactPath"],
        "evidence/scenarios/scaffold-smoke/input-replay.json"
    );
    assert_eq!(draft["target"]["scenarioPackId"], "smoke");
    assert_eq!(
        draft["target"]["scenarioPackPath"],
        "scenarios/smoke.scenario-pack.json"
    );
    assert_eq!(draft["proposedScenario"]["id"], "scaffold-smoke");
    assert_eq!(
        draft["proposedScenario"]["steps"][0]["input"]["right"],
        true
    );
    assert_eq!(
        draft["proposedScenario"]["assertions"][0]["world_state"]["path"],
        "entities.0.id"
    );
    assert_eq!(
        draft["proposedScenario"]["assertions"][0]["world_state"]["equals"],
        "player"
    );

    fs::remove_dir_all(temp).ok();
}

#[test]
fn scenario_promote_draft_rejects_missing_or_malformed_replay_evidence() {
    let temp = unique_temp_dir("ouroforge-cli-regression-promote-draft-invalid-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let (project_dir, run_dir) = create_project_bound_run(&temp);
    write_failed_regression_evidence(&temp, &run_dir, "scaffold-smoke", false);
    let missing_replay_output = temp.join("runs/drafts/missing-replay.json");

    let missing = run_cli_expect_failure(
        &temp,
        &[
            "scenario",
            "promote-draft",
            run_dir.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario",
            "scaffold-smoke",
            "--output",
            missing_replay_output.to_str().unwrap(),
        ],
    );
    assert!(missing.contains("requires at least one replay evidence artifact"));
    assert!(
        !missing_replay_output.exists(),
        "rejection must happen before writing draft output"
    );

    let malformed_run = create_project_bound_run(&temp).1;
    write_failed_regression_evidence(&temp, &malformed_run, "scaffold-smoke", true);
    fs::write(
        malformed_run.join("evidence/scenarios/scaffold-smoke/input-replay.json"),
        r#"{"schemaVersion":"ouroforge.scenario-input-replay.v1","scenarioId":"scaffold-smoke"}"#,
    )
    .expect("malformed replay written");
    let malformed_output = temp.join("runs/drafts/malformed-replay.json");
    let malformed = run_cli_expect_failure(
        &temp,
        &[
            "scenario",
            "promote-draft",
            malformed_run.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario",
            "scaffold-smoke",
            "--output",
            malformed_output.to_str().unwrap(),
        ],
    );
    assert!(malformed.contains("failed to parse scenario input replay artifact"));
    assert!(
        !malformed_output.exists(),
        "malformed evidence rejection must happen before writing draft output"
    );

    fs::remove_dir_all(temp).ok();
}

#[test]
fn scenario_promote_dry_run_and_apply_modify_only_authorized_pack_with_record() {
    let temp = unique_temp_dir("ouroforge-cli-regression-promote-apply-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let (project_dir, run_dir) = create_project_bound_run(&temp);
    write_failed_regression_evidence(&temp, &run_dir, "promoted-smoke-regression", true);
    let draft_path = temp.join("runs/drafts/promoted-smoke-regression-draft.json");
    run_cli(
        &temp,
        &[
            "scenario",
            "promote-draft",
            run_dir.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario",
            "promoted-smoke-regression",
            "--output",
            draft_path.to_str().unwrap(),
        ],
    );
    let pack_path = project_dir.join("scenarios/smoke.scenario-pack.json");
    let pack_before = fs::read_to_string(&pack_path).expect("pack reads before dry-run");

    let dry_run = run_cli(
        &temp,
        &[
            "scenario",
            "promote",
            draft_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
            "--dry-run",
        ],
    );
    assert!(dry_run.contains("Regression promotion dry-run: promoted-smoke-regression"));
    assert!(dry_run.contains(r#""dryRun": true"#));
    assert_eq!(
        fs::read_to_string(&pack_path).expect("pack reads after dry-run"),
        pack_before,
        "dry-run must not write scenario pack"
    );

    let promoted = run_cli(
        &temp,
        &[
            "scenario",
            "promote",
            draft_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    assert!(promoted.contains("Regression promoted: promoted-smoke-regression"));
    assert!(promoted.contains(r#""dryRun": false"#));
    assert!(promoted.contains("Promotion record: regression-promotions/"));
    let pack_after = fs::read_to_string(&pack_path).expect("pack reads after promote");
    assert_ne!(pack_after, pack_before, "promotion must update target pack");
    let pack_json: serde_json::Value = serde_json::from_str(&pack_after).expect("pack parses");
    let promoted_group = pack_json["scenarioGroups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|group| group["id"] == "promoted-regressions")
        .expect("promoted group created");
    assert_eq!(
        promoted_group["scenarios"][0]["id"],
        "promoted-smoke-regression"
    );
    let result_json_start = promoted.find('{').expect("result json starts");
    let result: serde_json::Value =
        serde_json::from_str(&promoted[result_json_start..]).expect("result parses");
    let record_path = result["recordPath"].as_str().expect("record path recorded");
    let record: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join(record_path)).expect("record reads"))
            .expect("record parses");
    assert_eq!(record["scenarioId"], "promoted-smoke-regression");
    assert_eq!(record["beforeHash"], result["beforeHash"]);
    assert_eq!(record["afterHash"], result["afterHash"]);

    let duplicate = run_cli_expect_failure(
        &temp,
        &[
            "scenario",
            "promote",
            draft_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    assert!(duplicate.contains("scenario id already exists"));
    assert_eq!(
        fs::read_to_string(&pack_path).expect("pack reads after duplicate rejection"),
        pack_after,
        "duplicate rejection must happen before rewriting pack"
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
    let command_context = run_json
        .get("run_command_context")
        .expect("run command context recorded");
    assert_eq!(
        command_context["transactionPath"],
        transaction_path.to_string_lossy().to_string()
    );
    assert!(command_context["command"]
        .as_str()
        .unwrap()
        .contains("--transaction"));

    let ledger = run_cli(&temp, &["ledger", "list", run_dir.to_str().unwrap()]);
    assert!(ledger.contains("run.transaction_bound"));
    assert!(ledger.contains("run.command_context_recorded"));

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
fn compare_command_prints_project_semantic_context_and_writes_artifact() {
    let temp = unique_temp_dir("ouroforge-cli-project-compare-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let project_dir = temp.join("project");
    run_cli(
        &temp,
        &[
            "project",
            "init",
            project_dir.to_str().unwrap(),
            "--template",
            "minimal-2d",
        ],
    );
    let seed_path = project_dir.join("seeds/platformer.yaml");
    let before_output = run_cli(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    let before_dir = run_dir_from_output(&temp, &before_output);

    let scene_path = project_dir.join("scenes/main.scene.json");
    let mut scene: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).expect("scene reads"))
            .expect("scene parses");
    scene["entities"][0]["components"]["transform"]["x"] = serde_json::json!(96);
    fs::write(
        &scene_path,
        serde_json::to_string_pretty(&scene).expect("scene serializes"),
    )
    .expect("scene writes");

    let after_output = run_cli(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    let after_dir = run_dir_from_output(&temp, &after_output);

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

    assert!(compare.contains("Project comparison:"));
    assert!(compare.contains("- relation: same_project"));
    assert!(compare.contains("- changed: true"));
    assert!(compare.contains("[scene_hash] scene hash changed"));
    assert!(compare.contains("[changed] project_context"));
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
        artifact_json["semantic"]["project"]["relation"],
        "same_project"
    );
    assert_eq!(artifact_json["semantic"]["project"]["changed"], true);
    assert!(artifact_json["semantic"]["project"]["changes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|change| change["kind"] == "scene_hash"));

    fs::remove_dir_all(temp).ok();
}

#[test]
fn mutation_review_records_proposal_decision_statuses_without_applying() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let run_output = run_cli(
        &repo_root,
        &["run", "seeds/evolve-v1-demo.yaml", "--workers", "1"],
    );
    let run_dir = run_dir_from_output(&repo_root, &run_output);
    run_cli(&repo_root, &["evolve", run_dir.to_str().unwrap()]);

    let list_output = run_cli(&repo_root, &["mutation", "list", run_dir.to_str().unwrap()]);
    assert!(list_output.contains("proposal"));
    let proposals: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_dir.join("mutation/proposals.json")).expect("proposals read"),
    )
    .expect("proposals json parses");
    let proposal_id = proposals["proposals"][0]["id"]
        .as_str()
        .expect("proposal id")
        .to_string();

    let accepted = run_cli(
        &repo_root,
        &[
            "mutation",
            "review",
            run_dir.to_str().unwrap(),
            "--proposal",
            &proposal_id,
            "--decision",
            "accepted",
            "--reason",
            "manual test",
            "--evidence",
            "mutation/patch-drafts.json",
            "--reviewer-type",
            "agent",
        ],
    );
    let accepted: serde_json::Value = serde_json::from_str(&accepted).expect("decision json");
    assert_eq!(accepted["proposal_id"].as_str(), Some(proposal_id.as_str()));
    assert_eq!(accepted["decision_status"].as_str(), Some("accepted"));
    assert_eq!(accepted["reviewer_type"].as_str(), Some("agent"));

    let deferred = run_cli(
        &repo_root,
        &[
            "mutation",
            "review",
            run_dir.to_str().unwrap(),
            "--defer",
            "--reason",
            "needs another comparison",
            "--evidence",
            "mutation/patch-drafts.json",
        ],
    );
    assert!(deferred.contains("deferred"));

    let invalid = run_cli_expect_failure(
        &repo_root,
        &[
            "mutation",
            "review",
            run_dir.to_str().unwrap(),
            "--proposal",
            "missing-proposal",
            "--decision",
            "rejected",
            "--reason",
            "bad proposal should fail",
            "--evidence",
            "mutation/patch-drafts.json",
        ],
    );
    assert!(invalid.contains("proposal id not found"));
    assert!(!Path::new("seeds/evolve-v1-draft-target.yaml.applied").exists());
    fs::remove_dir_all(run_dir).ok();
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
    let run: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("run.json")).unwrap()).unwrap();
    fs::create_dir_all(run_dir.join("mutation")).expect("mutation dir exists");
    fs::write(
        run_dir.join("mutation/patch-drafts.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": "1",
            "run_id": run["id"].as_str().unwrap_or("run-cli-review-gated"),
            "drafts": [{
                "id": "patch-draft-cli-scene-apply-1",
                "proposal_id": proposal_id,
                "classification_id": "classification-cli-scene-apply-1",
                "lifecycle_state": "drafted",
                "target_path": "scenes/review-gated-apply.scene.json",
                "rationale": "manual scene-only apply review draft",
                "evidence_refs": ["evidence/scene-mutation.json"],
                "draft_text": "scene-only CLI review-gated apply draft; no source patch apply"
            }]
        }))
        .unwrap(),
    )
    .expect("patch drafts written");
    let rejected_decision = run_cli(
        &temp,
        &[
            "mutation",
            "review",
            run_dir.to_str().unwrap(),
            "--proposal",
            proposal_id,
            "--decision",
            "rejected",
            "--reason",
            "manual test rejects before apply",
            "--evidence",
            "evidence/scene-mutation.json",
        ],
    );
    let rejected_decision: serde_json::Value =
        serde_json::from_str(&rejected_decision).expect("rejected decision json");
    let rejected_decision_id = rejected_decision["id"].as_str().unwrap().to_string();
    let accepted_decision = run_cli(
        &temp,
        &[
            "mutation",
            "review",
            run_dir.to_str().unwrap(),
            "--proposal",
            proposal_id,
            "--decision",
            "accepted",
            "--reason",
            "manual test accepts scene-only apply",
            "--evidence",
            "evidence/scene-mutation.json",
        ],
    );
    let accepted_decision: serde_json::Value =
        serde_json::from_str(&accepted_decision).expect("accepted decision json");
    let accepted_decision_id = accepted_decision["id"].as_str().unwrap().to_string();
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
    let rejected_transaction = temp.join("transactions/rejected-review-gated.json");
    let rejected_apply = run_cli_expect_failure(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--operation",
            success_operation_path.to_str().unwrap(),
            "--decision",
            &rejected_decision_id,
            "--transaction-output",
            rejected_transaction.to_str().unwrap(),
        ],
    );
    assert!(rejected_apply.contains("requires an accepted decision"));
    assert!(!rejected_transaction.exists());
    let scene_before_accepted_apply: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).unwrap()).unwrap();
    assert_eq!(
        scene_before_accepted_apply.pointer("/entities/0/components/transform/x"),
        Some(&serde_json::json!(32))
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
            "--decision",
            &accepted_decision_id,
            "--transaction-output",
            success_transaction.to_str().unwrap(),
        ],
    );
    assert!(apply_output.contains("Scene-only mutation applied: scene-edit-"));
    assert!(apply_output.contains(&format!("Review decision: {accepted_decision_id}")));
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
    assert_eq!(
        applications["applications"][0]["reviewDecisionId"],
        accepted_decision_id
    );
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

#[test]
fn mutation_apply_scene_project_flag_records_context_and_rejects_project_drift() {
    let temp = unique_temp_dir("ouroforge-cli-project-mutation-test");
    fs::create_dir_all(&temp).expect("temp dir exists");
    let project_dir = temp.join("project");
    run_cli(
        &temp,
        &[
            "project",
            "init",
            project_dir.to_str().unwrap(),
            "--template",
            "minimal-2d",
        ],
    );
    let manifest_path = project_dir.join("ouroforge.project.json");
    let scene_path = project_dir.join("scenes/main.scene.json");
    let seed_path = project_dir.join("seeds/platformer.yaml");
    let run_output = run_cli(
        &temp,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--project",
            manifest_path.to_str().unwrap(),
        ],
    );
    let run_dir = run_dir_from_output(&temp, &run_output);
    run_cli(
        &temp,
        &[
            "evidence",
            "add",
            run_dir.to_str().unwrap(),
            "--id",
            "project-mutation-evidence",
            "--kind",
            "application/json",
            "--path",
            "evidence/project-mutation.json",
            "--json",
            r#"{"source":"project-mutation-cli-test"}"#,
        ],
    );
    let proposal_json = run_cli(
        &temp,
        &[
            "mutation",
            "create",
            run_dir.to_str().unwrap(),
            "--reason",
            "project-scoped mutation",
            "--evidence",
            "project-mutation-evidence",
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
    let hash_probe_path = temp.join("transactions/project-hash-probe.json");
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

    let operation_path = temp.join("operations/project-success.json");
    write_operation(
        &operation_path,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "edit": { "entityId": "player", "path": "components.transform.x", "value": 48 },
            "expectedBeforeSceneHash": before_hash,
            "validationRequired": true
        }),
    );
    let transaction_path = temp.join("transactions/project-apply.json");
    let apply_output = run_cli(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--project",
            manifest_path.to_str().unwrap(),
            "--operation",
            operation_path.to_str().unwrap(),
            "--transaction-output",
            transaction_path.to_str().unwrap(),
        ],
    );
    assert!(apply_output.contains("Scene-only mutation applied: scene-edit-"));
    let applications: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(run_dir.join("mutation/scene-applications.json")).unwrap(),
    )
    .unwrap();
    let application = &applications["applications"][0];
    assert_eq!(application["proposalId"], proposal_id);
    assert_eq!(application["project"]["projectId"], "minimal_2d");
    assert_eq!(
        application["project"]["scenePath"],
        "scenes/main.scene.json"
    );
    assert_eq!(
        application["project"]["manifestPath"],
        manifest_path.to_string_lossy().to_string()
    );
    assert!(application["rollback"]["strategy"]
        .as_str()
        .unwrap()
        .contains("beforeSceneHash"));
    let ledger = run_cli(&temp, &["ledger", "list", run_dir.to_str().unwrap()]);
    assert!(ledger.contains(r#""project""#));
    assert!(ledger.contains(r#""scenePath": "scenes/main.scene.json""#));

    let updated_hash_probe = temp.join("transactions/project-hash-after.json");
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

    let mut stale_project = application["project"].clone();
    stale_project["manifestHash"]["value"] = serde_json::json!("0000000000000000");
    stale_project["sceneHash"] = current_hash.clone();
    let stale_operation_path = temp.join("operations/project-stale-manifest.json");
    write_operation(
        &stale_operation_path,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": scene_path.to_string_lossy().to_string(),
            "project": stale_project,
            "edit": { "entityId": "player", "path": "components.transform.x", "value": 64 },
            "expectedBeforeSceneHash": current_hash,
            "validationRequired": true
        }),
    );
    let stale_transaction = temp.join("transactions/project-stale.json");
    let stale = run_cli_expect_failure(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--project",
            manifest_path.to_str().unwrap(),
            "--operation",
            stale_operation_path.to_str().unwrap(),
            "--transaction-output",
            stale_transaction.to_str().unwrap(),
        ],
    );
    assert!(stale.contains("--project context does not match"));
    assert!(!stale_transaction.exists());

    let outside_scene = temp.join("outside.scene.json");
    fs::write(
        &outside_scene,
        include_str!("../../../examples/game-runtime/scene.json"),
    )
    .expect("outside scene written");
    let outside_hash_probe = temp.join("transactions/outside-hash.json");
    run_cli(
        &temp,
        &[
            "scene",
            "edit",
            outside_scene.to_str().unwrap(),
            "--entity",
            "player",
            "--path",
            "components.transform.x",
            "--value",
            "32",
            "--transaction-output",
            outside_hash_probe.to_str().unwrap(),
        ],
    );
    let outside_hash: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&outside_hash_probe).unwrap()).unwrap();
    let outside_operation_path = temp.join("operations/project-outside-target.json");
    write_operation(
        &outside_operation_path,
        serde_json::json!({
            "schemaVersion": "scene-only-mutation-v1",
            "proposalId": proposal_id,
            "targetScenePath": outside_scene.to_string_lossy().to_string(),
            "edit": { "entityId": "player", "path": "components.transform.x", "value": 99 },
            "expectedBeforeSceneHash": outside_hash["beforeSceneHash"].clone(),
            "validationRequired": true
        }),
    );
    let outside_transaction = temp.join("transactions/project-outside.json");
    let outside = run_cli_expect_failure(
        &temp,
        &[
            "mutation",
            "apply-scene",
            run_dir.to_str().unwrap(),
            "--project",
            manifest_path.to_str().unwrap(),
            "--operation",
            outside_operation_path.to_str().unwrap(),
            "--transaction-output",
            outside_transaction.to_str().unwrap(),
        ],
    );
    assert!(outside.contains("not declared in project manifest scenes"));
    assert!(!outside_transaction.exists());
    let preserved_scene: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&scene_path).unwrap()).unwrap();
    assert_eq!(
        preserved_scene.pointer("/entities/0/components/transform/x"),
        Some(&serde_json::json!(48))
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

fn create_project_bound_run(root: &Path) -> (PathBuf, PathBuf) {
    let project_dir = root.join(format!(
        "project-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time works")
            .as_nanos()
    ));
    run_cli(
        root,
        &[
            "project",
            "init",
            project_dir.to_str().unwrap(),
            "--template",
            "minimal-2d",
        ],
    );
    let seed_path = project_dir.join("seeds/platformer.yaml");
    let output = run_cli(
        root,
        &[
            "run",
            seed_path.to_str().unwrap(),
            "--project",
            project_dir.to_str().unwrap(),
            "--scenario-pack",
            "smoke",
        ],
    );
    (project_dir, run_dir_from_output(root, &output))
}

fn write_failed_regression_evidence(
    current_dir: &Path,
    run_dir: &Path,
    scenario_id: &str,
    include_replay: bool,
) {
    let scenario_result_rel = format!("evidence/scenarios/{scenario_id}/scenario-result.json");
    let replay_rel = format!("evidence/scenarios/{scenario_id}/input-replay.json");
    fs::create_dir_all(run_dir.join(format!("evidence/scenarios/{scenario_id}")))
        .expect("scenario evidence dir exists");
    fs::write(
        run_dir.join("verdict.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "status": "failed",
            "summary": "controlled regression promotion test failure",
            "failures": [{
                "kind": "scenario_assertion_failure",
                "scenario_id": scenario_id,
                "path": scenario_result_rel,
                "evidence_ref": scenario_result_rel
            }],
            "evidence_refs": [scenario_result_rel]
        }))
        .unwrap(),
    )
    .expect("failed verdict written");
    fs::write(
        run_dir.join(&scenario_result_rel),
        serde_json::to_string_pretty(&serde_json::json!({
            "scenario_id": scenario_id,
            "status": "failed",
            "evidence": {
                "scenario_input_replays": if include_replay {
                    vec![replay_rel.clone()]
                } else {
                    Vec::<String>::new()
                }
            },
            "assertions": [{
                "target": "world_state",
                "path": "entities.0.id",
                "operator": "equals",
                "expected": "player",
                "actual": "enemy",
                "passed": false,
                "evidence_ref": format!("evidence/scenarios/{scenario_id}/world-state.json")
            }]
        }))
        .unwrap(),
    )
    .expect("scenario result written");
    let scenario_result_output = run_cli(
        current_dir,
        &[
            "evidence",
            "add",
            run_dir.to_str().unwrap(),
            "--id",
            &format!("scenario-result-{scenario_id}"),
            "--kind",
            "application/json",
            "--path",
            &scenario_result_rel,
            "--json",
            &format!(
                r#"{{"artifact":"scenario_result","scenario_id":"{scenario_id}","status":"failed"}}"#
            ),
        ],
    );
    assert!(scenario_result_output.contains(&format!("scenario-result-{scenario_id}")));
    if include_replay {
        fs::write(
            run_dir.join(&replay_rel),
            serde_json::to_string_pretty(&serde_json::json!({
                "schemaVersion": "ouroforge.scenario-input-replay.v1",
                "scenarioId": scenario_id,
                "stepIndex": 0,
                "action": { "kind": "scenario_input_step" },
                "frame": 0,
                "tick": 0,
                "input": { "right": true },
                "result": {
                    "scenarioResultPath": scenario_result_rel,
                    "verdictPath": "verdict.json"
                }
            }))
            .unwrap(),
        )
        .expect("replay evidence written");
        let replay_output = run_cli(
            current_dir,
            &[
                "evidence",
                "add",
                run_dir.to_str().unwrap(),
                "--id",
                &format!("scenario-input-replay-{scenario_id}"),
                "--kind",
                "application/json",
                "--path",
                &replay_rel,
                "--json",
                &format!(r#"{{"artifact":"scenario_input_replay","scenario_id":"{scenario_id}"}}"#),
            ],
        );
        assert!(replay_output.contains(&format!("scenario-input-replay-{scenario_id}")));
    }
}

fn run_dir_from_output(root: &Path, output: &str) -> PathBuf {
    let line = output
        .lines()
        .find(|line| line.starts_with("Run created: "))
        .expect("run created line present");
    root.join(line.strip_prefix("Run created: ").unwrap())
}

fn fnv1a64_hex(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn run_cli(current_dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_ouroforge"))
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
    let output = Command::new(env!("CARGO_BIN_EXE_ouroforge"))
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

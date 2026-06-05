//! CLI inspection tests for the plugin registry (#752).
//!
//! Confirms `ouroforge plugin list` and `ouroforge plugin validate` expose
//! read-only plugin status/capabilities/extension points/compatibility/hash and
//! blocked reasons, and that no install/update/run/enable command exists.

use std::path::Path;
use std::process::Command;

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

fn run_cli_expect_failure(current_dir: &Path, args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_ouroforge"))
        .current_dir(current_dir)
        .args(args)
        .output()
        .expect("cli runs");
    assert!(
        !output.status.success(),
        "command unexpectedly succeeded\nstdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn plugin_list_reports_status_and_descriptors() {
    let root = repo_root();
    let stdout = run_cli(
        &root,
        &["plugin", "list", "examples/plugin-fixture-pack-v1"],
    );
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("list output is JSON");
    assert_eq!(json["schemaVersion"], "ouroforge.plugin-cli-list.v1");

    let plugins = json["plugins"].as_array().expect("plugins array");
    let dashboard = plugins
        .iter()
        .find(|p| p["pluginId"] == "fixture-dashboard-panel")
        .expect("dashboard plugin listed");
    assert_eq!(dashboard["validationStatus"], "valid");
    assert_eq!(dashboard["compatibilityStatus"], "compatible");
    assert_eq!(dashboard["declaredCapabilities"][0], "dashboardPanel");
    assert_eq!(dashboard["extensionPoints"][0], "dashboard.panels.readOnly");
    assert!(dashboard["manifestHash"]
        .as_str()
        .unwrap()
        .starts_with("fnv1a64-canonical-json-v1:"));

    // An invalid plugin reports blocked reasons (validationErrors).
    let invalid = plugins
        .iter()
        .find(|p| p["pluginId"] == "fixture-blocked-capability")
        .expect("blocked plugin listed");
    assert_eq!(invalid["validationStatus"], "invalid");
    assert!(!invalid["validationErrors"].as_array().unwrap().is_empty());

    // Read-only guardrail wording is present.
    assert!(json["guardrail"].as_str().unwrap().contains("no install"));
}

#[test]
fn plugin_validate_passes_for_valid_plugins() {
    let root = repo_root();
    let stdout = run_cli(
        &root,
        &[
            "plugin",
            "validate",
            "examples/plugin-fixture-pack-v1/valid",
        ],
    );
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("validate output is JSON");
    assert_eq!(json["status"], "ok");
    assert_eq!(json["summary"]["validCount"], 3);
}

#[test]
fn plugin_validate_fails_for_invalid_pack() {
    let root = repo_root();
    run_cli_expect_failure(
        &root,
        &["plugin", "validate", "examples/plugin-fixture-pack-v1"],
    );
}

#[test]
fn rejects_generated_root_as_discovery_base() {
    // A user must not be able to point discovery at a generated/evidence root,
    // which would otherwise be treated as a clean base and bypass the
    // registry's generated-root guard (#752).
    let root = repo_root();
    for generated in ["evidence", "runs", "dashboard-data", ".omx"] {
        run_cli_expect_failure(&root, &["plugin", "validate", generated]);
        run_cli_expect_failure(&root, &["plugin", "list", generated]);
    }
}

#[test]
fn no_install_or_run_commands_exist() {
    let root = repo_root();
    for forbidden in [
        ["plugin", "install", "x"],
        ["plugin", "run", "x"],
        ["plugin", "enable", "x"],
        ["plugin", "update", "x"],
    ] {
        run_cli_expect_failure(&root, &forbidden);
    }
}

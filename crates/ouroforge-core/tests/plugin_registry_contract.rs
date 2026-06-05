//! Contract tests for local plugin discovery and the registry (#740).
//!
//! Confirms that repository-rooted discovery finds the fixture-scoped plugin
//! tree under an allowlisted directory and reports valid, invalid, and
//! incompatible states. Discovery is read-only: no plugin code is executed and
//! no unsafe paths are followed.

use ouroforge_core::plugin_registry::{
    discover_plugin_registry, discover_plugins_in_dir, PluginRegistryCompatibility,
    PluginRegistryStatus,
};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/plugin-discovery-v1")
}

#[test]
fn repository_discovery_finds_fixture_plugins() {
    let registry = discover_plugin_registry(repo_root()).expect("repo discovery succeeds");
    let model = registry.read_model();
    assert!(model.valid_count >= 1);
    assert!(model.invalid_count >= 1);
    assert!(model.incompatible_count >= 1);
    // Discovered fixture paths are reported relative to the repo root.
    assert!(registry.entries.iter().any(|entry| entry
        .manifest_path
        .starts_with("examples/plugin-discovery-v1/")));
}

#[test]
fn fixture_tree_reports_expected_states() {
    let registry = discover_plugins_in_dir(fixture_root()).expect("fixture discovery succeeds");

    let valid = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "read-only-dashboard-panel")
        .expect("valid fixture present");
    assert_eq!(valid.validation_status, PluginRegistryStatus::Valid);
    assert_eq!(valid.declared_capabilities, ["dashboardPanel"]);
    // Declared permissions are reported in the registry output (#742).
    assert!(valid.permissions.contains(&"read_docs".to_string()));
    assert!(valid
        .permissions
        .contains(&"contribute_dashboard_panel".to_string()));
    assert!(valid
        .manifest_hash
        .starts_with("fnv1a64-canonical-json-v1:"));

    let incompatible = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "legacy-schema-plugin")
        .expect("incompatible fixture present");
    assert_eq!(
        incompatible.validation_status,
        PluginRegistryStatus::Incompatible
    );

    let invalid = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "broken-capability-plugin")
        .expect("invalid fixture present");
    assert_eq!(invalid.validation_status, PluginRegistryStatus::Invalid);
    assert!(!invalid.validation_errors.is_empty());

    // Asset metadata descriptors are reported for read-only inspection (#748).
    let asset = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "read-only-asset-metadata")
        .expect("asset metadata fixture present");
    assert_eq!(asset.validation_status, PluginRegistryStatus::Valid);
    assert!(asset
        .asset_metadata_descriptors
        .contains(&"sprite-pivot-metadata".to_string()));

    // A structurally valid manifest that requires a newer engine is reported as
    // future-version and blocked from extension contribution (#743).
    let future = registry
        .entries
        .iter()
        .find(|entry| entry.plugin_id == "future-engine-plugin")
        .expect("future-version fixture present");
    assert_eq!(future.validation_status, PluginRegistryStatus::Incompatible);
    assert_eq!(
        future.compatibility_status,
        PluginRegistryCompatibility::FutureVersion
    );
    assert!(future.extension_points.is_empty());
    assert!(future
        .validation_errors
        .iter()
        .any(|d| d.contains("upgrade Ouroforge")));
}

#[test]
fn discovery_refuses_generated_or_evidence_scan_root() {
    // A generated/evidence root used as the scan base would strip its own name from
    // every reported manifest path, so a plugin manifest under generated state could be
    // reported as valid/ok. Discovery must fail closed for such roots. (#752)
    let base = std::env::temp_dir().join(format!(
        "ouroforge-plugin-generated-root-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    for generated in ["evidence", "runs", "dashboard-data", ".omx"] {
        let scan_root = base.join(generated);
        let nested = scan_root.join("nested");
        std::fs::create_dir_all(&nested).expect("create nested dir");
        std::fs::write(
            nested.join("sneaky.plugin.json"),
            r#"{"schemaVersion":"ouroforge.plugin-manifest.v1","id":"sneaky","kind":"dashboardPanel"}"#,
        )
        .expect("write manifest");

        let err = discover_plugins_in_dir(&scan_root)
            .expect_err("generated/evidence scan root is rejected");
        assert!(
            err.to_string().contains(generated) && err.to_string().contains("generated"),
            "expected generated-root rejection for `{generated}`, got: {err}"
        );
    }
    std::fs::remove_dir_all(&base).ok();
}

#[test]
fn discovery_refuses_relative_descendant_of_generated_root() {
    // A relative scan base whose first repo-relative component is a generated root
    // (e.g. `evidence/nested`) descends into generated state and must fail closed,
    // even though the base's final component is an ordinary name. The refusal fires
    // before any filesystem access, so no fixture tree is required. (#1378)
    for generated in ["evidence", "runs", "dashboard-data", ".omx"] {
        let scan_root = std::path::Path::new(generated).join("nested");
        let err = discover_plugins_in_dir(&scan_root)
            .expect_err("relative descendant of a generated root is rejected");
        assert!(
            err.to_string().contains(generated) && err.to_string().contains("generated"),
            "expected generated-root rejection for `{generated}/nested`, got: {err}"
        );
    }
}

#[test]
fn discovery_allows_plugins_dir_under_generated_named_ancestor() {
    // Generated roots are defined relative to the repository root, so an unrelated
    // absolute ancestor whose name happens to be `evidence` must not disqualify a
    // legitimate plugins directory nested beneath it. (#752 follow-up)
    let scan_root = std::env::temp_dir()
        .join(format!(
            "ouroforge-evidence-ancestor-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ))
        .join("evidence")
        .join("work")
        .join("plugins");
    std::fs::create_dir_all(&scan_root).expect("create nested plugins dir");
    std::fs::write(
        scan_root.join("panel.plugin.json"),
        r#"{"schemaVersion":"ouroforge.plugin-manifest.v1","id":"panel","kind":"dashboardPanel"}"#,
    )
    .expect("write manifest");

    let registry = discover_plugins_in_dir(&scan_root)
        .expect("plugins dir under a generated-named ancestor is allowed");
    assert_eq!(registry.entries.len(), 1);

    // Clean up the unique top-level temp dir.
    if let Some(top) = scan_root.ancestors().nth(3) {
        std::fs::remove_dir_all(top).ok();
    }
}

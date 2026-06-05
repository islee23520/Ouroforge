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

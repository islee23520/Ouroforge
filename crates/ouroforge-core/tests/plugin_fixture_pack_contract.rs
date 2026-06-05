//! Contract tests for the plugin fixture pack (#749).
//!
//! Runs read-only discovery over the fixture-scoped plugin pack and asserts the
//! registry/validation behavior: valid plugins contribute descriptors only,
//! invalid plugins produce blocked diagnostics, and an incompatible plugin is
//! reported. The read model exposes a read-only boundary for rendering surfaces.

use ouroforge_core::plugin_registry::{discover_plugins_in_dir, PluginRegistryStatus};
use std::path::PathBuf;

fn fixture_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-fixture-pack-v1")
}

#[test]
fn fixture_pack_exercises_valid_and_blocked_plugins() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    let model = registry.read_model();

    // Three valid plugins, three invalid, one incompatible.
    assert_eq!(model.valid_count, 3, "{:?}", model);
    assert_eq!(model.invalid_count, 3, "{:?}", model);
    assert_eq!(model.incompatible_count, 1, "{:?}", model);
    assert_eq!(model.blocked_count, 0);

    // Valid plugins contribute descriptors (capabilities/extension points).
    for entry in registry
        .entries
        .iter()
        .filter(|entry| entry.validation_status == PluginRegistryStatus::Valid)
    {
        assert!(
            !entry.declared_capabilities.is_empty() && !entry.extension_points.is_empty(),
            "valid plugin `{}` must contribute descriptors",
            entry.plugin_id
        );
    }

    // Invalid plugins produce diagnostics and contribute nothing.
    for entry in registry
        .entries
        .iter()
        .filter(|entry| entry.validation_status == PluginRegistryStatus::Invalid)
    {
        assert!(
            !entry.validation_errors.is_empty(),
            "invalid plugin `{}` must report diagnostics",
            entry.plugin_id
        );
        assert!(entry.extension_points.is_empty());
    }

    // Read-only rendering boundary is stated for downstream surfaces.
    let boundary = model.boundary.to_ascii_lowercase();
    assert!(boundary.contains("read-only"));
    assert!(boundary.contains("no plugin execution"));
}

#[test]
fn fixture_pack_capability_and_asset_summaries_are_present() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    let model = registry.read_model();
    assert!(model
        .capability_summary
        .iter()
        .any(|c| c.contains("dashboardPanel")));
    assert!(model
        .asset_metadata_summary
        .iter()
        .any(|a| a.contains("fixture-sprite-metadata")));
}

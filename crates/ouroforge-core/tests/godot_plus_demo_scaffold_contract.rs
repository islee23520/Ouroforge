//! Godot-Plus demo scaffold contract (#781).
//!
//! The collect-and-exit demo scaffold ships an export profile placeholder, a
//! package metadata placeholder, and an inert read-only plugin descriptor. This
//! test proves the scaffold artifacts parse and validate through the existing
//! Rust-trusted contracts (export profile, package metadata, plugin manifest)
//! and stay inside the bounded local boundary. Validation is pure: it executes
//! no commands and writes no artifacts.

use ouroforge_core::export_package_metadata::{
    PackageMetadata, EXPORT_PACKAGE_METADATA_SCHEMA_VERSION,
};
use ouroforge_core::export_profile::{
    ExportProfile, RuntimeProbeMode, EXPORT_PROFILE_SCHEMA_VERSION,
};
use ouroforge_core::plugin_manifest::{PluginManifest, PLUGIN_MANIFEST_SCHEMA_VERSION};

const EXPORT_PROFILE: &str =
    include_str!("../../../examples/playable-demo-v2/collect-and-exit/export/export-profile.json");
const PACKAGE_METADATA: &str = include_str!(
    "../../../examples/playable-demo-v2/collect-and-exit/export/package-metadata.json"
);
const PLUGIN_MANIFEST: &str = include_str!(
    "../../../examples/playable-demo-v2/collect-and-exit/plugins/collect-and-exit-dashboard-panel/ouroforge.plugin.json"
);

#[test]
fn demo_export_profile_validates_as_local_web_target() {
    let profile =
        ExportProfile::from_json_str(EXPORT_PROFILE).expect("demo export profile must validate");
    assert_eq!(profile.schema_version, EXPORT_PROFILE_SCHEMA_VERSION);
    assert_eq!(profile.profile_id, "collect-and-exit-web-local");
    assert_eq!(profile.project_id, "collect_and_exit_demo");
    assert_eq!(profile.export_target, "web-local");
    assert!(
        profile.target_is_allowed(),
        "demo export target must be allowed in v1"
    );
    assert_eq!(profile.runtime_probe_mode, RuntimeProbeMode::Preserve);
    assert!(
        profile.output_dir.starts_with("dist/"),
        "export output must land in an ignored staging root"
    );
    assert!(
        profile
            .verification_scenario_ids
            .iter()
            .any(|id| id == "collect-key-hud-contract"),
        "export profile must reference the demo win-path scenario"
    );
}

#[test]
fn demo_package_metadata_validates() {
    let metadata = PackageMetadata::from_json_str(PACKAGE_METADATA)
        .expect("demo package metadata must validate");
    assert_eq!(
        metadata.schema_version,
        EXPORT_PACKAGE_METADATA_SCHEMA_VERSION
    );
    assert_eq!(metadata.project_id, "collect_and_exit_demo");
    let descriptor = metadata.to_local_descriptor();
    assert_eq!(descriptor.distribution, "local");
}

#[test]
fn demo_plugin_descriptor_is_inert_read_only_panel() {
    let manifest = PluginManifest::from_json_str(PLUGIN_MANIFEST)
        .expect("demo plugin descriptor must validate");
    assert_eq!(manifest.schema_version, PLUGIN_MANIFEST_SCHEMA_VERSION);
    assert_eq!(manifest.plugin_id, "collect-and-exit-dashboard-panel");
    assert!(
        manifest
            .declared_capabilities
            .iter()
            .any(|c| c == "dashboardPanel"),
        "demo plugin must declare the read-only dashboard panel capability"
    );
    assert!(
        manifest
            .extension_points
            .iter()
            .any(|p| p == "dashboard.panels.readOnly"),
        "demo plugin must target the read-only dashboard extension point"
    );
    assert!(
        manifest
            .boundary
            .to_ascii_lowercase()
            .contains("no executable code"),
        "demo plugin must declare an inert no-executable-code boundary"
    );
}

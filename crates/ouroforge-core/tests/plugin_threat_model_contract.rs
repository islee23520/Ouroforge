//! Contract tests for the plugin security / threat model gate (#750).
//!
//! Confirms the threat-model gate passes, the checklist covers the required
//! risks, and the highest-risk paths actually fail closed against the real
//! validators (manifest, permission, extension catalog, and discovery registry).

use ouroforge_core::plugin_extension_catalog::validate_extension_point;
use ouroforge_core::plugin_manifest::PluginManifest;
use ouroforge_core::plugin_permission::validate_permission;
use ouroforge_core::plugin_registry::{discover_plugins_in_dir, PluginRegistryStatus};
use ouroforge_core::plugin_threat_model::{checklist_ids, gate};
use std::path::PathBuf;

fn fixture_pack_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-fixture-pack-v1")
}

#[test]
fn gate_and_checklist_are_complete() {
    gate().expect("threat model gate passes");
    assert_eq!(checklist_ids().len(), 12);
}

#[test]
fn privileged_permissions_fail_closed() {
    for permission in [
        "native_extension",
        "install_dependency",
        "access_credentials",
        "write_source",
        "publish_export",
        "mutate_ci",
        "run_command",
        "network_access",
        "execute_script",
    ] {
        assert!(
            validate_permission(permission).is_err(),
            "permission `{permission}` must fail closed"
        );
    }
}

#[test]
fn privileged_extension_points_fail_closed() {
    for point in [
        "source.write.now",
        "command.exec.run",
        "release.publish.now",
        "ci.workflow.mutate",
        "native.dylib.load",
    ] {
        assert!(
            validate_extension_point(point).is_err(),
            "extension point `{point}` must fail closed"
        );
    }
}

#[test]
fn network_reference_in_manifest_fails_closed() {
    let manifest = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "network-plugin",
        "name": "Network Plugin",
        "version": "1.0.0",
        "description": "fetch from https://example.com/payload",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["dashboardPanel"],
        "extensionPoints": ["dashboard.panels.readOnly"],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    })
    .to_string();
    assert!(PluginManifest::from_json_str(&manifest).is_err());
}

#[test]
fn high_risk_fixtures_are_blocked_in_discovery() {
    let registry = discover_plugins_in_dir(fixture_pack_root()).expect("fixture pack discovery");
    // arbitrary-js, blocked-capability, and unsafe-path are reported invalid;
    // none of them contribute extension points.
    for plugin_id in [
        "fixture-arbitrary-js",
        "fixture-blocked-capability",
        "fixture-unsafe-path",
    ] {
        let entry = registry
            .entries
            .iter()
            .find(|entry| entry.plugin_id == plugin_id)
            .unwrap_or_else(|| panic!("fixture `{plugin_id}` present"));
        assert_eq!(entry.validation_status, PluginRegistryStatus::Invalid);
        assert!(entry.extension_points.is_empty());
        assert!(!entry.validation_errors.is_empty());
    }
}

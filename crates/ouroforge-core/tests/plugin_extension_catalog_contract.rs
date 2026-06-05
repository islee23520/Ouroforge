//! Contract tests for the plugin extension point catalog (#741).
//!
//! Confirms the v1 catalog is explicit and narrow, that unknown and blocked
//! extension points fail closed, and that the catalog is the source of truth
//! consumed by manifest validation. No executable extension point is allowed.

use ouroforge_core::plugin_extension_catalog::{
    is_allowed, validate_extension_point, ALLOWED_EXTENSION_POINT_IDS, CATALOG,
};
use ouroforge_core::plugin_manifest::PluginManifest;

#[test]
fn catalog_is_narrow_and_read_only() {
    assert_eq!(CATALOG.len(), 6);
    for id in ALLOWED_EXTENSION_POINT_IDS {
        assert!(is_allowed(id));
        assert!(id.ends_with(".readOnly"), "catalog point must be read-only");
        validate_extension_point(id).expect("catalog point validates");
    }
}

#[test]
fn unknown_and_blocked_points_fail_closed() {
    assert!(validate_extension_point("dashboard.panels.preview").is_err());
    for blocked in [
        "source.write.now",
        "command.exec.run",
        "release.deploy.now",
        "runtime.script.inject",
        "native.dylib.load",
    ] {
        let err = validate_extension_point(blocked)
            .expect_err("blocked point fails")
            .to_string();
        assert!(err.contains("blocked"), "{err}");
    }
}

#[test]
fn manifest_validation_consumes_catalog() {
    // A manifest declaring an out-of-catalog extension point fails closed.
    let manifest = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "catalog-consumer",
        "name": "Catalog Consumer",
        "version": "1.0.0",
        "description": "Declares an out-of-catalog extension point.",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["dashboardPanel"],
        "extensionPoints": ["dashboard.panels.readOnly", "dashboard.panels.preview"],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    })
    .to_string();
    assert!(PluginManifest::from_json_str(&manifest).is_err());
}

//! Contract tests for the plugin capability/permission model (#742).
//!
//! Confirms that allowed permissions validate, blocked permissions fail closed
//! with clear reasons, and that manifest validation enforces the permission
//! allowlist. Permission validation grants no runtime power.

use ouroforge_core::plugin_manifest::PluginManifest;
use ouroforge_core::plugin_permission::{
    is_allowed, validate_permission, validate_permissions, ALLOWED_PERMISSIONS,
};

#[test]
fn allowed_permissions_are_read_or_contribute_only() {
    assert_eq!(ALLOWED_PERMISSIONS.len(), 7);
    for permission in ALLOWED_PERMISSIONS {
        assert!(is_allowed(permission));
        assert!(
            permission.starts_with("read_") || permission.starts_with("contribute_"),
            "permission `{permission}` must be read- or contribute-only"
        );
        validate_permission(permission).expect("allowed permission validates");
    }
}

#[test]
fn blocked_permissions_fail_closed() {
    for blocked in [
        "write_source",
        "run_command",
        "install_dependency",
        "publish_export",
        "access_credentials",
        "network_access",
        "mutate_ci",
        "native_extension",
        "execute_script",
    ] {
        let err = validate_permission(blocked)
            .expect_err("blocked permission fails")
            .to_string();
        assert!(err.contains("blocked"), "{err}");
    }
}

#[test]
fn empty_and_valid_permission_sets() {
    validate_permissions("permissions", &[]).expect("empty set valid");
    validate_permissions(
        "permissions",
        &["read_docs".to_string(), "read_evidence".to_string()],
    )
    .expect("valid set");
}

#[test]
fn manifest_enforces_permission_allowlist() {
    let base = serde_json::json!({
        "schemaVersion": "ouroforge.plugin-manifest.v1",
        "pluginId": "perm-consumer",
        "name": "Permission Consumer",
        "version": "1.0.0",
        "description": "Declares permissions.",
        "metadata": { "author": "A", "project": "P" },
        "compatibility": { "minOuroforgeVersion": "0.1.0" },
        "declaredCapabilities": ["dashboardPanel"],
        "extensionPoints": ["dashboard.panels.readOnly"],
        "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
    });

    let mut allowed = base.clone();
    allowed["permissions"] = serde_json::json!(["read_docs", "contribute_dashboard_panel"]);
    PluginManifest::from_json_str(&allowed.to_string()).expect("allowed permissions validate");

    let mut blocked = base;
    blocked["permissions"] = serde_json::json!(["read_docs", "execute_script"]);
    assert!(PluginManifest::from_json_str(&blocked.to_string()).is_err());
}

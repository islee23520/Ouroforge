//! Contract tests for plugin version compatibility (#743).
//!
//! Confirms compatible/incompatible/unknown/future-version evaluation against
//! the current engine contract, with actionable diagnostics and no network or
//! package-registry lookup.

use ouroforge_core::plugin_compatibility::{
    evaluate, PluginCompatibilityStatus, CURRENT_OUROFORGE_VERSION,
    SUPPORTED_PLUGIN_SCHEMA_VERSIONS,
};

#[test]
fn current_contract_constants_are_sane() {
    assert!(!CURRENT_OUROFORGE_VERSION.is_empty());
    assert!(SUPPORTED_PLUGIN_SCHEMA_VERSIONS.contains(&"ouroforge.plugin-manifest.v1"));
}

#[test]
fn compatible_plugin_may_contribute() {
    let report = evaluate("ouroforge.plugin-manifest.v1", "0.1.0", "1.0.0");
    assert_eq!(report.status, PluginCompatibilityStatus::Compatible);
    assert!(report.may_contribute());
}

#[test]
fn incompatible_and_future_plugins_blocked_with_diagnostics() {
    let unsupported = evaluate("ouroforge.plugin-manifest.v9", "0.1.0", "");
    assert_eq!(unsupported.status, PluginCompatibilityStatus::Incompatible);
    assert!(!unsupported.may_contribute());
    assert!(!unsupported.diagnostics.is_empty());

    let future = evaluate("ouroforge.plugin-manifest.v1", "9.0.0", "");
    assert_eq!(future.status, PluginCompatibilityStatus::FutureVersion);
    assert!(!future.may_contribute());
    assert!(future.diagnostics[0].contains("upgrade Ouroforge"));

    let too_old = evaluate("ouroforge.plugin-manifest.v1", "0.0.1", "0.0.2");
    assert_eq!(too_old.status, PluginCompatibilityStatus::Incompatible);
    assert!(!too_old.may_contribute());
}

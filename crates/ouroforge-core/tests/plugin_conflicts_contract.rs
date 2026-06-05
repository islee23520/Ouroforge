//! Contract tests for plugin load order and conflict detection (#751).
//!
//! Confirms deterministic registry ordering, detection of duplicate plugin and
//! descriptor ids with a fail/warn policy, and that detection introduces no
//! automatic merge/override/execution ordering.

use ouroforge_core::plugin_conflicts::{
    detect_conflicts, PluginConflictKind, PluginConflictSeverity,
};
use ouroforge_core::plugin_registry::discover_plugins_in_dir;
use std::path::PathBuf;

fn conflict_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-conflict-v1")
}

#[test]
fn registry_ordering_is_deterministic() {
    let registry = discover_plugins_in_dir(conflict_root()).expect("discovery");
    let paths: Vec<_> = registry
        .entries
        .iter()
        .map(|entry| entry.manifest_path.clone())
        .collect();
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted);
}

#[test]
fn duplicate_ids_are_detected_and_fail() {
    let registry = discover_plugins_in_dir(conflict_root()).expect("discovery");
    let report = detect_conflicts(&registry);
    assert!(report.has_failures());

    let plugin_conflict = report
        .conflicts
        .iter()
        .find(|c| c.kind == PluginConflictKind::DuplicatePluginId)
        .expect("duplicate plugin id conflict");
    assert_eq!(plugin_conflict.severity, PluginConflictSeverity::Fail);
    assert_eq!(plugin_conflict.identifier, "fixture-duplicate-plugin");
    assert_eq!(plugin_conflict.plugins.len(), 2);

    let descriptor_conflict = report
        .conflicts
        .iter()
        .find(|c| c.kind == PluginConflictKind::DuplicateDescriptorId)
        .expect("duplicate descriptor id conflict");
    assert_eq!(descriptor_conflict.severity, PluginConflictSeverity::Fail);
    assert_eq!(descriptor_conflict.identifier, "shared-panel-descriptor");
    assert_eq!(
        descriptor_conflict.descriptor_kind.as_deref(),
        Some("dashboardPanel")
    );

    // Detection only reports; the boundary states no resolution/ordering.
    assert!(report.boundary.contains("without resolving"));
}

#[test]
fn clean_registry_has_no_conflicts() {
    // A single-entry discovery (the conflict tree's dup-a alone is not isolable
    // here, so reuse the manifest-v1 valid example which has unique ids).
    let registry = discover_plugins_in_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/plugin-discovery-v1/plugins/read-only-dashboard-panel"),
    )
    .expect("discovery");
    let report = detect_conflicts(&registry);
    assert!(report.is_clean());
}

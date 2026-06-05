//! Plugin load order and conflict detection (#751).
//!
//! Detects plugin id and extension descriptor conflicts deterministically over a
//! discovered registry. This module only *reports* conflicts with a fail/warn
//! policy — it never resolves them, merges descriptors, overrides plugins, or
//! introduces execution ordering. Registry ordering is already deterministic
//! (sorted by manifest path); conflict output is likewise deterministic.

use crate::plugin_registry::PluginRegistry;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginConflictSeverity {
    /// A colliding id that would make resolution ambiguous.
    Fail,
    /// A reuse that is allowed but worth surfacing.
    Warn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginConflictKind {
    DuplicatePluginId,
    DuplicateDescriptorId,
    CrossKindDescriptorId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginConflict {
    pub kind: PluginConflictKind,
    pub severity: PluginConflictSeverity,
    /// The colliding identifier (plugin id or descriptor id).
    pub identifier: String,
    /// The descriptor kind, when the conflict is about a descriptor.
    #[serde(rename = "descriptorKind", skip_serializing_if = "Option::is_none")]
    pub descriptor_kind: Option<String>,
    /// The plugins (by manifest path) participating in the conflict.
    pub plugins: Vec<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginConflictReport {
    pub conflicts: Vec<PluginConflict>,
    pub boundary: String,
}

impl PluginConflictReport {
    pub fn has_failures(&self) -> bool {
        self.conflicts
            .iter()
            .any(|conflict| conflict.severity == PluginConflictSeverity::Fail)
    }

    pub fn is_clean(&self) -> bool {
        self.conflicts.is_empty()
    }
}

/// Detect plugin id and descriptor id conflicts across a registry. Output is
/// deterministic. No conflict is resolved; this is detection only.
pub fn detect_conflicts(registry: &PluginRegistry) -> PluginConflictReport {
    let mut conflicts = Vec::new();

    // Duplicate plugin ids (by manifest path, since the id itself collides).
    let mut by_plugin_id: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for entry in &registry.entries {
        by_plugin_id
            .entry(entry.plugin_id.as_str())
            .or_default()
            .push(entry.manifest_path.clone());
    }
    for (plugin_id, mut paths) in by_plugin_id {
        if paths.len() > 1 {
            paths.sort();
            conflicts.push(PluginConflict {
                kind: PluginConflictKind::DuplicatePluginId,
                severity: PluginConflictSeverity::Fail,
                identifier: plugin_id.to_string(),
                descriptor_kind: None,
                message: format!(
                    "plugin id `{plugin_id}` is declared by {} manifests",
                    paths.len()
                ),
                plugins: paths,
            });
        }
    }

    // Descriptor ids contributed by valid plugins, grouped by (kind, id) for
    // same-kind collisions and by id for cross-kind reuse.
    let mut by_kind_id: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    let mut kinds_by_id: BTreeMap<String, BTreeMap<String, ()>> = BTreeMap::new();
    for entry in &registry.entries {
        for descriptor in &entry.descriptors {
            by_kind_id
                .entry((descriptor.kind.clone(), descriptor.id.clone()))
                .or_default()
                .push(entry.manifest_path.clone());
            kinds_by_id
                .entry(descriptor.id.clone())
                .or_default()
                .insert(descriptor.kind.clone(), ());
        }
    }
    for ((kind, id), mut paths) in by_kind_id {
        if paths.len() > 1 {
            paths.sort();
            paths.dedup();
            if paths.len() > 1 {
                conflicts.push(PluginConflict {
                    kind: PluginConflictKind::DuplicateDescriptorId,
                    severity: PluginConflictSeverity::Fail,
                    identifier: id.clone(),
                    descriptor_kind: Some(kind.clone()),
                    message: format!(
                        "{kind} descriptor id `{id}` is declared by {} plugins",
                        paths.len()
                    ),
                    plugins: paths,
                });
            }
        }
    }
    for (id, kinds) in kinds_by_id {
        if kinds.len() > 1 {
            let kind_list = kinds.keys().cloned().collect::<Vec<_>>().join(", ");
            conflicts.push(PluginConflict {
                kind: PluginConflictKind::CrossKindDescriptorId,
                severity: PluginConflictSeverity::Warn,
                identifier: id.clone(),
                descriptor_kind: None,
                plugins: Vec::new(),
                message: format!("descriptor id `{id}` is reused across kinds: {kind_list}"),
            });
        }
    }

    conflicts.sort_by(|left, right| {
        (
            severity_rank(left.severity),
            format!("{:?}", left.kind),
            left.descriptor_kind.clone(),
            left.identifier.clone(),
        )
            .cmp(&(
                severity_rank(right.severity),
                format!("{:?}", right.kind),
                right.descriptor_kind.clone(),
                right.identifier.clone(),
            ))
    });

    PluginConflictReport {
        conflicts,
        boundary: "Read-only plugin conflict report; detects duplicate plugin/descriptor ids without resolving, merging, overriding, or ordering plugin execution.".to_string(),
    }
}

fn severity_rank(severity: PluginConflictSeverity) -> u8 {
    match severity {
        PluginConflictSeverity::Fail => 0,
        PluginConflictSeverity::Warn => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin_registry::discover_plugins_in_dir;
    use std::path::PathBuf;

    fn conflict_fixture_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/plugin-conflict-v1")
    }

    #[test]
    fn detects_duplicate_plugin_and_descriptor_ids() {
        let registry =
            discover_plugins_in_dir(conflict_fixture_root()).expect("conflict discovery");
        let report = detect_conflicts(&registry);
        assert!(report.has_failures());
        assert!(report
            .conflicts
            .iter()
            .any(|c| c.kind == PluginConflictKind::DuplicatePluginId
                && c.identifier == "fixture-duplicate-plugin"));
        assert!(report
            .conflicts
            .iter()
            .any(|c| c.kind == PluginConflictKind::DuplicateDescriptorId
                && c.identifier == "shared-panel-descriptor"));
    }

    #[test]
    fn detection_is_deterministic() {
        let registry =
            discover_plugins_in_dir(conflict_fixture_root()).expect("conflict discovery");
        let first = detect_conflicts(&registry);
        let second = detect_conflicts(&registry);
        assert_eq!(first, second);
    }
}

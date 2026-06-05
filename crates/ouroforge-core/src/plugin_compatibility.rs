//! Plugin version compatibility (#743).
//!
//! Evaluates whether a declarative plugin manifest is compatible with the
//! current Ouroforge plugin schema and engine contract. Evaluation is pure and
//! local: it compares declared version metadata against built-in supported
//! ranges and never performs a network or package-registry lookup, loads code,
//! or executes anything.
//!
//! The registry (#740) consumes this to report a compatibility status and
//! actionable diagnostics, and to block incompatible or future-version plugins
//! from extension contribution.

use crate::plugin_manifest::SUPPORTED_MANIFEST_SCHEMA_VERSIONS;
use serde::Serialize;

/// The engine/plugin contract version this build implements. Plugins declare a
/// minimum (and optional maximum) Ouroforge version they support; this is the
/// version those ranges are evaluated against.
pub const CURRENT_OUROFORGE_VERSION: &str = "0.1.0";

/// Plugin schema versions this build supports. Re-exported from the manifest
/// schema so compatibility has a single source of truth.
pub const SUPPORTED_PLUGIN_SCHEMA_VERSIONS: &[&str] = SUPPORTED_MANIFEST_SCHEMA_VERSIONS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PluginCompatibilityStatus {
    /// Schema supported and engine version within the declared range.
    Compatible,
    /// Schema unsupported, or current engine newer than the plugin's maximum.
    Incompatible,
    /// Version metadata could not be parsed/compared.
    Unknown,
    /// Plugin requires a newer engine than this build provides.
    FutureVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PluginCompatibilityReport {
    pub status: PluginCompatibilityStatus,
    pub diagnostics: Vec<String>,
}

impl PluginCompatibilityReport {
    /// Whether the plugin may contribute extensions. Only `Compatible` plugins
    /// may; incompatible/future/unknown plugins are reported but blocked.
    pub fn may_contribute(&self) -> bool {
        self.status == PluginCompatibilityStatus::Compatible
    }
}

/// Evaluate compatibility from a plugin's declared `schemaVersion`,
/// `minOuroforgeVersion`, and optional `maxOuroforgeVersion`.
pub fn evaluate(
    schema_version: &str,
    min_ouroforge_version: &str,
    max_ouroforge_version: &str,
) -> PluginCompatibilityReport {
    let mut diagnostics = Vec::new();

    if !SUPPORTED_PLUGIN_SCHEMA_VERSIONS.contains(&schema_version) {
        diagnostics.push(format!(
            "plugin schema version `{schema_version}` is not supported (supported: {SUPPORTED_PLUGIN_SCHEMA_VERSIONS:?})"
        ));
        return PluginCompatibilityReport {
            status: PluginCompatibilityStatus::Incompatible,
            diagnostics,
        };
    }

    let Some(min) = parse_version(min_ouroforge_version) else {
        diagnostics.push(format!(
            "minOuroforgeVersion `{min_ouroforge_version}` could not be parsed"
        ));
        return PluginCompatibilityReport {
            status: PluginCompatibilityStatus::Unknown,
            diagnostics,
        };
    };
    let current = parse_version(CURRENT_OUROFORGE_VERSION)
        .expect("CURRENT_OUROFORGE_VERSION is a valid version");

    if compare(&min, &current) == std::cmp::Ordering::Greater {
        diagnostics.push(format!(
            "plugin requires Ouroforge >= {min_ouroforge_version}, but this build is {CURRENT_OUROFORGE_VERSION}; upgrade Ouroforge to use this plugin"
        ));
        return PluginCompatibilityReport {
            status: PluginCompatibilityStatus::FutureVersion,
            diagnostics,
        };
    }

    if !max_ouroforge_version.is_empty() {
        let Some(max) = parse_version(max_ouroforge_version) else {
            diagnostics.push(format!(
                "maxOuroforgeVersion `{max_ouroforge_version}` could not be parsed"
            ));
            return PluginCompatibilityReport {
                status: PluginCompatibilityStatus::Unknown,
                diagnostics,
            };
        };
        if compare(&current, &max) == std::cmp::Ordering::Greater {
            diagnostics.push(format!(
                "plugin supports Ouroforge <= {max_ouroforge_version}, but this build is {CURRENT_OUROFORGE_VERSION}; the plugin predates this engine"
            ));
            return PluginCompatibilityReport {
                status: PluginCompatibilityStatus::Incompatible,
                diagnostics,
            };
        }
    }

    PluginCompatibilityReport {
        status: PluginCompatibilityStatus::Compatible,
        diagnostics,
    }
}

fn parse_version(value: &str) -> Option<Vec<u64>> {
    if value.trim().is_empty() {
        return None;
    }
    value
        .split('.')
        .map(|segment| segment.parse::<u64>().ok())
        .collect()
}

fn compare(left: &[u64], right: &[u64]) -> std::cmp::Ordering {
    let len = left.len().max(right.len());
    for index in 0..len {
        let l = left.get(index).copied().unwrap_or(0);
        let r = right.get(index).copied().unwrap_or(0);
        match l.cmp(&r) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_when_in_range() {
        let report = evaluate("ouroforge.plugin-manifest.v1", "0.1.0", "1.0.0");
        assert_eq!(report.status, PluginCompatibilityStatus::Compatible);
        assert!(report.may_contribute());
        assert!(report.diagnostics.is_empty());
    }

    #[test]
    fn compatible_with_no_max() {
        let report = evaluate("ouroforge.plugin-manifest.v1", "0.0.1", "");
        assert_eq!(report.status, PluginCompatibilityStatus::Compatible);
    }

    #[test]
    fn unsupported_schema_is_incompatible() {
        let report = evaluate("ouroforge.plugin-manifest.v2", "0.1.0", "");
        assert_eq!(report.status, PluginCompatibilityStatus::Incompatible);
        assert!(!report.may_contribute());
        assert!(report.diagnostics[0].contains("not supported"));
    }

    #[test]
    fn future_minimum_is_future_version() {
        let report = evaluate("ouroforge.plugin-manifest.v1", "1.0.0", "");
        assert_eq!(report.status, PluginCompatibilityStatus::FutureVersion);
        assert!(report.diagnostics[0].contains("upgrade Ouroforge"));
        assert!(!report.may_contribute());
    }

    #[test]
    fn engine_newer_than_max_is_incompatible() {
        let report = evaluate("ouroforge.plugin-manifest.v1", "0.0.1", "0.0.9");
        assert_eq!(report.status, PluginCompatibilityStatus::Incompatible);
        assert!(report.diagnostics[0].contains("predates this engine"));
    }

    #[test]
    fn unparseable_min_is_unknown() {
        let report = evaluate("ouroforge.plugin-manifest.v1", "abc", "");
        assert_eq!(report.status, PluginCompatibilityStatus::Unknown);
    }
}

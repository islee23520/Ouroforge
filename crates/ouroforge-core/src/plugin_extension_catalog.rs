//! Plugin extension point catalog (#741).
//!
//! Defines the small, allowlisted catalog of declarative extension points
//! available to plugins in v1. Each catalog entry is a pure data descriptor:
//! it names a read-only surface a plugin may declare against and, where
//! applicable, the capability that backs it. No catalog entry authorizes
//! executable code, trusted writes, source mutation, command execution,
//! export/publish/deploy, dependency installation, CI mutation, runtime script
//! injection, or native extensions — those categories fail closed.
//!
//! The manifest schema (#739) consumes this catalog so that the allowed
//! extension points have a single source of truth.

use anyhow::{anyhow, Result};

/// A single allowlisted, read-only extension point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginExtensionPointDescriptor {
    /// Stable identifier declared in plugin manifests.
    pub id: &'static str,
    /// Capability that must back this extension point, if any. Capability-less
    /// points (evidence viewer, docs/examples) may be declared standalone.
    pub required_capability: Option<&'static str>,
    /// Allowlisted descriptor kind a plugin may attach, if any.
    pub descriptor_kind: Option<&'static str>,
    /// Human-readable summary of the read-only surface.
    pub summary: &'static str,
}

/// The complete v1 extension point catalog. Narrow and explicit by design.
pub const CATALOG: &[PluginExtensionPointDescriptor] = &[
    PluginExtensionPointDescriptor {
        id: "dashboard.panels.readOnly",
        required_capability: Some("dashboardPanel"),
        descriptor_kind: Some("dashboardPanel"),
        summary: "Read-only dashboard panel descriptor.",
    },
    PluginExtensionPointDescriptor {
        id: "studio.inspector.readOnly",
        required_capability: Some("studioInspectorPanel"),
        descriptor_kind: Some("studioInspectorPanel"),
        summary: "Read-only Studio inspector panel descriptor.",
    },
    PluginExtensionPointDescriptor {
        id: "evidence.viewer.readOnly",
        required_capability: None,
        descriptor_kind: None,
        summary: "Read-only evidence viewer descriptor.",
    },
    PluginExtensionPointDescriptor {
        id: "scenario.templates.readOnly",
        required_capability: Some("scenarioTemplate"),
        descriptor_kind: Some("scenarioTemplate"),
        summary: "Read-only scenario template descriptor.",
    },
    PluginExtensionPointDescriptor {
        id: "assets.metadata.readOnly",
        required_capability: Some("assetMetadataProvider"),
        descriptor_kind: Some("assetMetadataProvider"),
        summary: "Read-only asset metadata descriptor.",
    },
    PluginExtensionPointDescriptor {
        id: "docs.examples.readOnly",
        required_capability: None,
        descriptor_kind: None,
        summary: "Read-only docs/example descriptor.",
    },
];

/// All allowlisted extension point ids, in catalog order. Used by manifest
/// validation as the single source of truth for accepted extension points.
pub const ALLOWED_EXTENSION_POINT_IDS: &[&str] = &[
    "dashboard.panels.readOnly",
    "studio.inspector.readOnly",
    "evidence.viewer.readOnly",
    "scenario.templates.readOnly",
    "assets.metadata.readOnly",
    "docs.examples.readOnly",
];

/// Explicitly blocked extension point categories. Any extension point id whose
/// segments match one of these tokens fails closed with an actionable reason,
/// rather than being silently treated as merely unknown.
const BLOCKED_EXTENSION_POINT_CATEGORIES: &[(&str, &str)] = &[
    (
        "write",
        "trusted file write is not an allowed extension point",
    ),
    (
        "mutate",
        "source mutation is not an allowed extension point",
    ),
    (
        "mutation",
        "source mutation is not an allowed extension point",
    ),
    (
        "source",
        "source mutation is not an allowed extension point",
    ),
    (
        "command",
        "command execution is not an allowed extension point",
    ),
    (
        "exec",
        "command execution is not an allowed extension point",
    ),
    (
        "shell",
        "command execution is not an allowed extension point",
    ),
    ("export", "export is not an allowed extension point"),
    ("publish", "publish is not an allowed extension point"),
    ("deploy", "deploy is not an allowed extension point"),
    (
        "install",
        "dependency installation is not an allowed extension point",
    ),
    (
        "dependency",
        "dependency installation is not an allowed extension point",
    ),
    (
        "ci",
        "CI/workflow mutation is not an allowed extension point",
    ),
    (
        "workflow",
        "CI/workflow mutation is not an allowed extension point",
    ),
    (
        "script",
        "runtime script injection is not an allowed extension point",
    ),
    (
        "runtime",
        "runtime script injection is not an allowed extension point",
    ),
    (
        "native",
        "native extension is not an allowed extension point",
    ),
    (
        "dylib",
        "native extension is not an allowed extension point",
    ),
];

/// Look up a catalog descriptor by extension point id.
pub fn descriptor(id: &str) -> Option<&'static PluginExtensionPointDescriptor> {
    CATALOG.iter().find(|descriptor| descriptor.id == id)
}

/// Returns true if the extension point id is in the v1 catalog.
pub fn is_allowed(id: &str) -> bool {
    descriptor(id).is_some()
}

/// The extension point an allowlisted capability maps to, if any.
pub fn capability_extension_point(capability: &str) -> Option<&'static str> {
    CATALOG
        .iter()
        .find(|descriptor| descriptor.required_capability == Some(capability))
        .map(|descriptor| descriptor.id)
}

/// The capability an extension point requires, if any.
pub fn required_capability(extension_point: &str) -> Option<&'static str> {
    descriptor(extension_point).and_then(|descriptor| descriptor.required_capability)
}

/// Validate a single extension point id. Fails closed for blocked categories
/// (with a specific reason) and for unknown ids.
pub fn validate_extension_point(id: &str) -> Result<()> {
    if is_allowed(id) {
        return Ok(());
    }
    let lower = id.to_ascii_lowercase();
    for (token, reason) in BLOCKED_EXTENSION_POINT_CATEGORIES {
        if lower.contains(token) {
            return Err(anyhow!(
                "plugin extension point `{id}` is blocked: {reason}"
            ));
        }
    }
    Err(anyhow!(
        "plugin extension point `{id}` is not in the v1 extension point catalog"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_and_id_list_agree() {
        let catalog_ids: Vec<&str> = CATALOG.iter().map(|descriptor| descriptor.id).collect();
        assert_eq!(catalog_ids, ALLOWED_EXTENSION_POINT_IDS);
        assert_eq!(CATALOG.len(), 6);
    }

    #[test]
    fn allowed_points_validate() {
        for id in ALLOWED_EXTENSION_POINT_IDS {
            validate_extension_point(id).expect("catalog point validates");
            assert!(is_allowed(id));
        }
    }

    #[test]
    fn capability_round_trips() {
        assert_eq!(
            capability_extension_point("dashboardPanel"),
            Some("dashboard.panels.readOnly")
        );
        assert_eq!(
            required_capability("scenario.templates.readOnly"),
            Some("scenarioTemplate")
        );
        // Capability-less points have no required capability.
        assert_eq!(required_capability("docs.examples.readOnly"), None);
        assert_eq!(required_capability("evidence.viewer.readOnly"), None);
    }

    #[test]
    fn unknown_point_fails_closed() {
        let err = validate_extension_point("dashboard.panels.preview")
            .expect_err("unknown point fails")
            .to_string();
        assert!(
            err.contains("not in the v1 extension point catalog"),
            "{err}"
        );
    }

    #[test]
    fn blocked_categories_fail_with_reason() {
        for (id, needle) in [
            ("source.write.readWrite", "trusted file write"),
            ("project.source.mutate", "source mutation"),
            ("command.exec.run", "command execution"),
            ("store.publish.now", "publish"),
            ("bundle.export.zip", "export"),
            ("release.deploy.now", "deploy"),
            ("dependency.install.npm", "dependency installation"),
            ("ci.workflow.mutate", "source mutation"),
            ("runtime.script.inject", "runtime script injection"),
            ("native.dylib.load", "native extension"),
        ] {
            let err = validate_extension_point(id)
                .expect_err("blocked point fails")
                .to_string();
            assert!(
                err.contains("blocked") && err.contains(needle),
                "id `{id}` expected `{needle}`, got `{err}`"
            );
        }
    }
}

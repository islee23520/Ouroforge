//! Plugin manifest schema (#739).
//!
//! Declares the typed, declarative `ouroforge.plugin.json` manifest that a local
//! plugin author writes. The manifest carries plugin identity, declared
//! capabilities, allowlisted extension points, version compatibility metadata,
//! and optional descriptor/doc/asset references. Parsing and validation are
//! side-effect free: no command execution, no code loading, no network access,
//! and no trusted writes. Deeper version-compatibility resolution lives in
//! follow-up work (#743); registry discovery lives in #740. This module owns the
//! input contract only.

use crate::plugin_evidence::{
    require_allowed_values, require_local_id, require_local_text, require_unique_ids,
    ALLOWED_CAPABILITIES,
};
use crate::plugin_extension_catalog;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

/// Schema version for the manifest format defined by this module.
pub const PLUGIN_MANIFEST_SCHEMA_VERSION: &str = "ouroforge.plugin-manifest.v1";

/// Manifest schema versions this build can validate. v1 only; anything else
/// fails closed as an incompatible schema version.
pub const SUPPORTED_MANIFEST_SCHEMA_VERSIONS: &[&str] = &[PLUGIN_MANIFEST_SCHEMA_VERSION];

/// Allowlisted descriptor kinds a manifest may reference. Mirrors the capability
/// allowlist: a descriptor reference is only meaningful for a declared
/// capability.
pub const ALLOWED_DESCRIPTOR_KINDS: &[&str] = ALLOWED_CAPABILITIES;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub metadata: PluginManifestMetadata,
    pub compatibility: PluginManifestCompatibility,
    #[serde(rename = "declaredCapabilities")]
    pub declared_capabilities: Vec<String>,
    #[serde(rename = "extensionPoints")]
    pub extension_points: Vec<String>,
    #[serde(default)]
    pub paths: PluginManifestPaths,
    #[serde(rename = "descriptorRefs", default)]
    pub descriptor_refs: Vec<PluginManifestDescriptorRef>,
    /// Optional declared permissions, validated against the fail-closed
    /// permission allowlist (#742). Declarations only; granting no runtime power.
    #[serde(default)]
    pub permissions: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginManifestMetadata {
    pub author: String,
    pub project: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginManifestCompatibility {
    #[serde(rename = "minOuroforgeVersion")]
    pub min_ouroforge_version: String,
    #[serde(rename = "maxOuroforgeVersion", default)]
    pub max_ouroforge_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginManifestPaths {
    #[serde(default)]
    pub docs: Vec<String>,
    #[serde(default)]
    pub assets: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PluginManifestDescriptorRef {
    pub id: String,
    pub kind: String,
    pub path: String,
}

/// Read-only inspection summary of a validated manifest. Used by CLI/dashboard
/// surfaces (#752, Studio browser) without re-running validation.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PluginManifestReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "declaredCapabilities")]
    pub declared_capabilities: Vec<String>,
    #[serde(rename = "extensionPoints")]
    pub extension_points: Vec<String>,
    pub permissions: Vec<String>,
    #[serde(rename = "descriptorRefCount")]
    pub descriptor_ref_count: usize,
    #[serde(rename = "docCount")]
    pub doc_count: usize,
    #[serde(rename = "assetCount")]
    pub asset_count: usize,
    #[serde(rename = "exampleCount")]
    pub example_count: usize,
    #[serde(rename = "minOuroforgeVersion")]
    pub min_ouroforge_version: String,
    #[serde(rename = "maxOuroforgeVersion")]
    pub max_ouroforge_version: String,
    pub boundary: String,
}

impl PluginManifest {
    /// Parse and validate a manifest from JSON. Fails closed with an actionable
    /// diagnostic for any invalid id, path, capability, extension point,
    /// incompatible schema version, or executable-code field. Performs no
    /// command execution, code loading, network access, or trusted writes.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let manifest: Self =
            serde_json::from_str(input).context("failed to parse plugin manifest JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        if !SUPPORTED_MANIFEST_SCHEMA_VERSIONS.contains(&self.schema_version.as_str()) {
            return Err(anyhow!(
                "plugin manifest schemaVersion `{}` is not a supported schema version (expected one of {:?})",
                self.schema_version,
                SUPPORTED_MANIFEST_SCHEMA_VERSIONS
            ));
        }
        require_local_id("plugin manifest pluginId", &self.plugin_id)?;
        require_local_text("plugin manifest name", &self.name)?;
        require_version("plugin manifest version", &self.version)?;
        require_local_text("plugin manifest description", &self.description)?;
        self.metadata.validate()?;
        self.compatibility.validate()?;

        require_allowed_values(
            "plugin manifest declaredCapabilities",
            &self.declared_capabilities,
            ALLOWED_CAPABILITIES,
        )?;
        require_allowed_values(
            "plugin manifest extensionPoints",
            &self.extension_points,
            plugin_extension_catalog::ALLOWED_EXTENSION_POINT_IDS,
        )?;
        // Defense in depth: route every declared extension point through the
        // catalog so blocked categories fail closed with an actionable reason.
        for point in &self.extension_points {
            plugin_extension_catalog::validate_extension_point(point)?;
        }
        self.validate_capability_extension_pairs()?;

        require_unique_ids(
            "plugin manifest descriptorRefs.id",
            self.descriptor_refs.iter().map(|d| d.id.as_str()),
        )?;
        for descriptor in &self.descriptor_refs {
            descriptor.validate()?;
            if !self.declared_capabilities.contains(&descriptor.kind) {
                return Err(anyhow!(
                    "plugin manifest descriptorRefs `{}` declares kind `{}` without a matching declared capability",
                    descriptor.id,
                    descriptor.kind
                ));
            }
        }

        self.paths.validate()?;
        crate::plugin_permission::validate_permissions(
            "plugin manifest permissions",
            &self.permissions,
        )?;
        require_manifest_boundary("plugin manifest boundary", &self.boundary)?;
        Ok(())
    }

    /// Enforce consistency between declared capabilities and extension points
    /// using the extension point catalog (#741): every declared capability
    /// requires its mapped extension point, and every capability-backed
    /// extension point requires its capability. Capability-less catalog points
    /// (e.g. evidence viewer, docs/examples) may be declared standalone.
    fn validate_capability_extension_pairs(&self) -> Result<()> {
        for capability in &self.declared_capabilities {
            let expected = plugin_extension_catalog::capability_extension_point(capability)
                .ok_or_else(|| {
                    anyhow!(
                        "plugin manifest capability `{capability}` has no mapped extension point"
                    )
                })?;
            if !self.extension_points.iter().any(|p| p == expected) {
                return Err(anyhow!(
                    "plugin manifest capability `{capability}` requires extension point `{expected}`"
                ));
            }
        }
        for point in &self.extension_points {
            if let Some(expected) = plugin_extension_catalog::required_capability(point) {
                if !self.declared_capabilities.iter().any(|c| c == expected) {
                    return Err(anyhow!(
                        "plugin manifest extension point `{point}` requires declared capability `{expected}`"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn read_model(&self) -> PluginManifestReadModel {
        let mut declared_capabilities = self.declared_capabilities.clone();
        declared_capabilities.sort();
        let mut extension_points = self.extension_points.clone();
        extension_points.sort();
        let mut permissions = self.permissions.clone();
        permissions.sort();
        PluginManifestReadModel {
            schema_version: self.schema_version.clone(),
            plugin_id: self.plugin_id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            declared_capabilities,
            extension_points,
            permissions,
            descriptor_ref_count: self.descriptor_refs.len(),
            doc_count: self.paths.docs.len(),
            asset_count: self.paths.assets.len(),
            example_count: self.paths.examples.len(),
            min_ouroforge_version: self.compatibility.min_ouroforge_version.clone(),
            max_ouroforge_version: self.compatibility.max_ouroforge_version.clone(),
            boundary: "Read-only plugin manifest summary; declarative descriptors only, with no plugin execution, no command execution, no network install, and no trusted writes.".to_string(),
        }
    }
}

impl PluginManifestMetadata {
    fn validate(&self) -> Result<()> {
        require_local_text("plugin manifest metadata.author", &self.author)?;
        require_local_text("plugin manifest metadata.project", &self.project)?;
        if !self.license.is_empty() {
            require_local_id("plugin manifest metadata.license", &self.license)?;
        }
        if !self.summary.is_empty() {
            require_local_text("plugin manifest metadata.summary", &self.summary)?;
        }
        Ok(())
    }
}

impl PluginManifestCompatibility {
    fn validate(&self) -> Result<()> {
        require_version(
            "plugin manifest compatibility.minOuroforgeVersion",
            &self.min_ouroforge_version,
        )?;
        if !self.max_ouroforge_version.is_empty() {
            require_version(
                "plugin manifest compatibility.maxOuroforgeVersion",
                &self.max_ouroforge_version,
            )?;
            if compare_versions(&self.min_ouroforge_version, &self.max_ouroforge_version)
                == std::cmp::Ordering::Greater
            {
                return Err(anyhow!(
                    "plugin manifest compatibility.minOuroforgeVersion `{}` must not be greater than maxOuroforgeVersion `{}`",
                    self.min_ouroforge_version,
                    self.max_ouroforge_version
                ));
            }
        }
        Ok(())
    }
}

impl PluginManifestPaths {
    fn validate(&self) -> Result<()> {
        for (field, values) in [
            ("plugin manifest paths.docs", &self.docs),
            ("plugin manifest paths.assets", &self.assets),
            ("plugin manifest paths.examples", &self.examples),
        ] {
            require_unique_ids(field, values.iter().map(|v| v.as_str()))?;
            for value in values {
                require_relative_path(field, value)?;
            }
        }
        Ok(())
    }
}

impl PluginManifestDescriptorRef {
    fn validate(&self) -> Result<()> {
        require_local_id("plugin manifest descriptorRefs.id", &self.id)?;
        if !ALLOWED_DESCRIPTOR_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!(
                "plugin manifest descriptorRefs.kind `{}` is not in the v1 allowlist",
                self.kind
            ));
        }
        require_relative_path("plugin manifest descriptorRefs.path", &self.path)?;
        if !self.path.ends_with(".json") {
            return Err(anyhow!(
                "plugin manifest descriptorRefs.path `{}` must point at a .json descriptor",
                self.path
            ));
        }
        Ok(())
    }
}

/// A bounded, dotted numeric version like `1.2.3`. Rejects empty, overlong, or
/// non-numeric segments. Richer range resolution is #743.
fn require_version(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if value.len() > 32 {
        return Err(anyhow!("{field} must be at most 32 characters"));
    }
    let segments: Vec<&str> = value.split('.').collect();
    if !(1..=4).contains(&segments.len()) {
        return Err(anyhow!(
            "{field} must have between one and four dot-separated numeric segments"
        ));
    }
    for segment in segments {
        if segment.is_empty() || !segment.chars().all(|ch| ch.is_ascii_digit()) {
            return Err(anyhow!(
                "{field} segments must be non-empty and numeric (got `{value}`)"
            ));
        }
    }
    Ok(())
}

/// Compare two validated dotted-numeric versions segment by segment.
fn compare_versions(left: &str, right: &str) -> std::cmp::Ordering {
    let parse = |value: &str| -> Vec<u64> {
        value
            .split('.')
            .map(|segment| segment.parse::<u64>().unwrap_or(0))
            .collect()
    };
    let left = parse(left);
    let right = parse(right);
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

/// A manifest-relative path inside the plugin tree. Rejects absolute paths,
/// `..` traversal, and backslash separators (the latter explicitly, because
/// `Path::components` ignores backslashes on Unix). Forbidden executable/URL
/// text is rejected via `require_local_text`.
fn require_relative_path(field: &str, value: &str) -> Result<()> {
    require_local_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} `{value}` must stay inside the plugin tree without traversal"
        ));
    }
    Ok(())
}

fn require_manifest_boundary(field: &str, value: &str) -> Result<()> {
    require_local_text(field, value)?;
    let boundary = value.to_ascii_lowercase();
    for required in [
        "declarative",
        "read-only",
        "no executable",
        "no command",
        "no network",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!("{field} must state `{required}`"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_dashboard_manifest() -> &'static str {
        include_str!(
            "../../../examples/plugin-manifest-v1/valid/dashboard-panel-plugin.plugin.json"
        )
    }

    fn valid_scenario_manifest() -> &'static str {
        include_str!(
            "../../../examples/plugin-manifest-v1/valid/scenario-template-plugin.plugin.json"
        )
    }

    #[test]
    fn valid_dashboard_manifest_parses_and_validates() {
        let manifest = PluginManifest::from_json_str(valid_dashboard_manifest())
            .expect("valid dashboard manifest validates");
        assert_eq!(manifest.schema_version, PLUGIN_MANIFEST_SCHEMA_VERSION);
        assert_eq!(manifest.declared_capabilities, ["dashboardPanel"]);
        assert_eq!(manifest.extension_points, ["dashboard.panels.readOnly"]);
        let model = manifest.read_model();
        assert_eq!(model.plugin_id, manifest.plugin_id);
        assert!(model.boundary.contains("no command execution"));
    }

    #[test]
    fn valid_scenario_manifest_with_descriptor_refs_validates() {
        let manifest = PluginManifest::from_json_str(valid_scenario_manifest())
            .expect("valid scenario manifest validates");
        assert_eq!(manifest.descriptor_refs.len(), 1);
        assert_eq!(manifest.descriptor_refs[0].kind, "scenarioTemplate");
        let model = manifest.read_model();
        assert_eq!(model.descriptor_ref_count, 1);
        assert!(model.example_count >= 1);
    }

    fn base_manifest() -> serde_json::Value {
        serde_json::json!({
            "schemaVersion": PLUGIN_MANIFEST_SCHEMA_VERSION,
            "pluginId": "sample-plugin",
            "name": "Sample Plugin",
            "version": "1.0.0",
            "description": "A declarative sample plugin.",
            "metadata": { "author": "Example Author", "project": "Ouroforge Sample" },
            "compatibility": { "minOuroforgeVersion": "0.1.0" },
            "declaredCapabilities": ["dashboardPanel"],
            "extensionPoints": ["dashboard.panels.readOnly"],
            "boundary": "Declarative read-only manifest with no executable code, no command execution, and no network access."
        })
    }

    fn expect_invalid(value: serde_json::Value, needle: &str) {
        let json = value.to_string();
        let err = format!(
            "{:#}",
            PluginManifest::from_json_str(&json).expect_err("manifest must fail closed")
        );
        assert!(
            err.contains(needle),
            "expected error containing `{needle}`, got `{err}`"
        );
    }

    #[test]
    fn rejects_incompatible_schema_version() {
        let mut manifest = base_manifest();
        manifest["schemaVersion"] = serde_json::json!("ouroforge.plugin-manifest.v2");
        expect_invalid(manifest, "not a supported schema version");
    }

    #[test]
    fn rejects_unknown_capability() {
        let mut manifest = base_manifest();
        manifest["declaredCapabilities"] = serde_json::json!(["executeScript"]);
        expect_invalid(manifest, "not in the v1 allowlist");
    }

    #[test]
    fn rejects_unknown_extension_point() {
        let mut manifest = base_manifest();
        manifest["extensionPoints"] = serde_json::json!(["dashboard.panels.writeBack"]);
        expect_invalid(manifest, "not in the v1 allowlist");
    }

    #[test]
    fn rejects_capability_extension_mismatch() {
        let mut manifest = base_manifest();
        manifest["declaredCapabilities"] = serde_json::json!(["dashboardPanel"]);
        manifest["extensionPoints"] = serde_json::json!(["scenario.templates.readOnly"]);
        expect_invalid(manifest, "requires extension point");
    }

    #[test]
    fn accepts_capability_less_catalog_extension_point() {
        // A capability-backed point plus a standalone catalog point (#741).
        let mut manifest = base_manifest();
        manifest["extensionPoints"] =
            serde_json::json!(["dashboard.panels.readOnly", "docs.examples.readOnly"]);
        let json = manifest.to_string();
        let parsed =
            PluginManifest::from_json_str(&json).expect("capability-less catalog point validates");
        assert!(parsed
            .extension_points
            .contains(&"docs.examples.readOnly".to_string()));
    }

    #[test]
    fn rejects_blocked_extension_point_from_catalog() {
        let mut manifest = base_manifest();
        manifest["declaredCapabilities"] = serde_json::json!(["dashboardPanel"]);
        manifest["extensionPoints"] =
            serde_json::json!(["dashboard.panels.readOnly", "source.write.unsafe"]);
        // Blocked categories are rejected by the catalog membership check.
        expect_invalid(manifest, "not in the v1 allowlist");
    }

    #[test]
    fn accepts_allowed_permissions() {
        let mut manifest = base_manifest();
        manifest["permissions"] = serde_json::json!(["read_docs", "read_evidence"]);
        let parsed = PluginManifest::from_json_str(&manifest.to_string())
            .expect("allowed permissions validate");
        assert_eq!(parsed.permissions.len(), 2);
        assert_eq!(
            parsed.read_model().permissions,
            ["read_docs", "read_evidence"]
        );
    }

    #[test]
    fn rejects_blocked_permission() {
        let mut manifest = base_manifest();
        manifest["permissions"] = serde_json::json!(["read_docs", "run_command"]);
        expect_invalid(manifest, "blocked");
    }

    #[test]
    fn rejects_unknown_permission() {
        let mut manifest = base_manifest();
        manifest["permissions"] = serde_json::json!(["super_admin"]);
        expect_invalid(manifest, "not in the v1 permission allowlist");
    }

    #[test]
    fn rejects_unsafe_path_traversal() {
        let mut manifest = base_manifest();
        manifest["paths"] = serde_json::json!({ "docs": ["../escape/readme.md"] });
        expect_invalid(manifest, "without traversal");
    }

    #[test]
    fn rejects_backslash_path() {
        let mut manifest = base_manifest();
        manifest["paths"] = serde_json::json!({ "assets": ["assets\\sprite.png"] });
        expect_invalid(manifest, "without traversal");
    }

    #[test]
    fn rejects_executable_field_via_deny_unknown_fields() {
        let mut manifest = base_manifest();
        manifest["entrypoint"] = serde_json::json!("plugin.js");
        expect_invalid(manifest, "unknown field");
    }

    #[test]
    fn rejects_executable_text_in_description() {
        let mut manifest = base_manifest();
        manifest["description"] = serde_json::json!("runs eval(payload) on load");
        expect_invalid(manifest, "forbidden executable plugin");
    }

    #[test]
    fn rejects_invalid_id() {
        let mut manifest = base_manifest();
        manifest["pluginId"] = serde_json::json!("bad id!");
        expect_invalid(manifest, "bounded local id");
    }

    #[test]
    fn rejects_invalid_version() {
        let mut manifest = base_manifest();
        manifest["version"] = serde_json::json!("1.x");
        expect_invalid(manifest, "numeric");
    }

    #[test]
    fn rejects_min_greater_than_max() {
        let mut manifest = base_manifest();
        manifest["compatibility"] =
            serde_json::json!({ "minOuroforgeVersion": "2.0.0", "maxOuroforgeVersion": "1.0.0" });
        expect_invalid(manifest, "must not be greater than");
    }

    #[test]
    fn rejects_descriptor_ref_without_capability() {
        let mut manifest = base_manifest();
        manifest["descriptorRefs"] = serde_json::json!([{
            "id": "scenario-descriptor",
            "kind": "scenarioTemplate",
            "path": "descriptors/scenario.json"
        }]);
        expect_invalid(manifest, "without a matching declared capability");
    }

    #[test]
    fn rejects_missing_boundary_keywords() {
        let mut manifest = base_manifest();
        manifest["boundary"] = serde_json::json!("A friendly plugin.");
        expect_invalid(manifest, "must state");
    }

    #[test]
    fn validate_performs_no_io() {
        // Validation is pure: it only inspects the in-memory value. This test
        // documents the contract; there is no filesystem or network surface to
        // exercise here.
        let manifest = PluginManifest::from_json_str(&base_manifest().to_string())
            .expect("base manifest validates");
        assert_eq!(manifest.plugin_id, "sample-plugin");
    }
}

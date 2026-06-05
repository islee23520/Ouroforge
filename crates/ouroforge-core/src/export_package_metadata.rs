//! Package Metadata and Local Distribution Descriptor v1 (#730).
//!
//! Descriptive, non-publishing package metadata for a local export, plus a
//! generated local distribution descriptor. Metadata is validated fail closed:
//! required fields must be present, paths must be safe, and any
//! store/release/signing/credential field is rejected (the schema denies unknown
//! fields, so publish-oriented keys never parse).

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Component, Path};

pub const EXPORT_PACKAGE_METADATA_SCHEMA_VERSION: &str = "export-package-metadata-v1";
pub const LOCAL_DISTRIBUTION_DESCRIPTOR_SCHEMA_VERSION: &str = "local-distribution-descriptor-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PackageMetadata {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub title: String,
    pub version: String,
    /// Author/owner label (descriptive only).
    pub author: String,
    pub description: String,
    /// Placeholder icon reference (package-relative path).
    pub icon: String,
    #[serde(rename = "entryScene")]
    pub entry_scene: String,
}

/// Generated local distribution descriptor. Describes a local, non-published
/// package only.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LocalDistributionDescriptor {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub entry: String,
    /// Always `local`: this descriptor never authorizes publishing.
    pub distribution: String,
}

impl PackageMetadata {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let metadata: Self =
            serde_json::from_str(input).context("failed to parse Package Metadata JSON")?;
        metadata.validate()?;
        Ok(metadata)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize package metadata")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EXPORT_PACKAGE_METADATA_SCHEMA_VERSION {
            return Err(anyhow!(
                "package metadata schemaVersion must be {EXPORT_PACKAGE_METADATA_SCHEMA_VERSION}"
            ));
        }
        require_local_id("package metadata projectId", &self.project_id)?;
        require_text("package metadata title", &self.title)?;
        require_version("package metadata version", &self.version)?;
        require_text("package metadata author", &self.author)?;
        require_text("package metadata description", &self.description)?;
        validate_relative_path("package metadata icon", &self.icon)?;
        validate_relative_path("package metadata entryScene", &self.entry_scene)?;
        Ok(())
    }

    /// Generate the local distribution descriptor for this package.
    pub fn to_local_descriptor(&self) -> LocalDistributionDescriptor {
        LocalDistributionDescriptor {
            schema_version: LOCAL_DISTRIBUTION_DESCRIPTOR_SCHEMA_VERSION.to_string(),
            name: self.title.clone(),
            version: self.version.clone(),
            project_id: self.project_id.clone(),
            entry: self.entry_scene.clone(),
            distribution: "local".to_string(),
        }
    }
}

impl LocalDistributionDescriptor {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize distribution descriptor")
    }
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}

fn require_version(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty()
        || trimmed.len() > 32
        || !trimmed
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '.' | '+'))
    {
        return Err(anyhow!(
            "{field} must be a bounded version string using alphanumeric, dot, dash, or plus"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 512 {
        return Err(anyhow!("{field} must be non-empty text up to 512 bytes"));
    }
    let lower = trimmed.to_ascii_lowercase();
    for forbidden in [
        "production-ready",
        "godot replacement",
        "app store",
        "signing key",
        "publish",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} must describe a local package only (forbidden text `{forbidden}`)"
            ));
        }
    }
    Ok(())
}

fn validate_relative_path(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 512 || value.contains('\\') {
        return Err(anyhow!(
            "{field} must be a relative path without backslashes"
        ));
    }
    let path = Path::new(value);
    if path.is_absolute() || value.starts_with('/') {
        return Err(anyhow!("{field} must be relative"));
    }
    for component in path.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(anyhow!("{field} must be a normalized relative path"));
        }
    }
    Ok(())
}

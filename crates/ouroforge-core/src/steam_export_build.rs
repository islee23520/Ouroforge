//! Steam Desktop Export build/depot contract v1 (#1838).
//!
//! This module validates the local, deterministic Steam desktop export build
//! contract authorized by #1837. It models an Electron wrapper over the existing
//! web runtime plus a SteamPipe depot layout and package descriptor. It performs
//! no command execution, no filesystem writes, no credential handling, no code
//! signing, and no release/publish action.

use crate::export_hash::sha256_prefixed;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Component, Path};

pub const STEAM_EXPORT_BUILD_MANIFEST_SCHEMA_VERSION: &str = "steam-export-build-manifest-v1";
pub const STEAM_DEPOT_CONFIG_SCHEMA_VERSION: &str = "steam-depot-config-v1";
pub const STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION: &str = "steam-package-descriptor-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamExportBuildManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "buildId")]
    pub build_id: String,
    #[serde(rename = "issue")]
    pub issue: u32,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub title: String,
    pub version: String,
    #[serde(rename = "sourceWebBuild")]
    pub source_web_build: WebBuildRef,
    #[serde(rename = "electronWrapper")]
    pub electron_wrapper: ElectronWrapperRef,
    #[serde(rename = "windowsArtifact")]
    pub windows_artifact: WindowsArtifactRef,
    #[serde(rename = "depotConfigRef")]
    pub depot_config_ref: String,
    #[serde(rename = "generatedStatePolicy")]
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WebBuildRef {
    #[serde(rename = "runtime")]
    pub runtime: String,
    #[serde(rename = "runtimeReuse")]
    pub runtime_reuse: String,
    #[serde(rename = "entryHtml")]
    pub entry_html: String,
    #[serde(rename = "assetManifest")]
    pub asset_manifest: String,
    #[serde(rename = "probeMode")]
    pub probe_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ElectronWrapperRef {
    pub framework: String,
    #[serde(rename = "steamBridge")]
    pub steam_bridge: String,
    #[serde(rename = "mainProcess")]
    pub main_process: String,
    #[serde(rename = "preload")]
    pub preload: String,
    #[serde(rename = "trustedWritePolicy")]
    pub trusted_write_policy: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WindowsArtifactRef {
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "executablePath")]
    pub executable_path: String,
    #[serde(rename = "artifactRoot")]
    pub artifact_root: String,
    #[serde(rename = "descriptorPath")]
    pub descriptor_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamDepotConfig {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "buildId")]
    pub build_id: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "depotId")]
    pub depot_id: String,
    #[serde(rename = "contentRoot")]
    pub content_root: String,
    #[serde(rename = "installDir")]
    pub install_dir: String,
    #[serde(rename = "fileMappings")]
    pub file_mappings: Vec<DepotFileMapping>,
    #[serde(rename = "uploadMode")]
    pub upload_mode: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DepotFileMapping {
    pub local: String,
    pub depot: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamPackageDescriptor {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "buildId")]
    pub build_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub title: String,
    pub version: String,
    pub platform: String,
    pub wrapper: String,
    #[serde(rename = "steamBridge")]
    pub steam_bridge: String,
    #[serde(rename = "entryHtml")]
    pub entry_html: String,
    #[serde(rename = "executablePath")]
    pub executable_path: String,
    #[serde(rename = "depotConfigHash")]
    pub depot_config_hash: String,
    #[serde(rename = "buildManifestHash")]
    pub build_manifest_hash: String,
    #[serde(rename = "artifactHash")]
    pub artifact_hash: String,
    #[serde(rename = "releaseAuthority")]
    pub release_authority: String,
}

impl SteamExportBuildManifest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let manifest: Self = serde_json::from_str(input)
            .context("failed to parse Steam export build manifest JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAM_EXPORT_BUILD_MANIFEST_SCHEMA_VERSION {
            return Err(anyhow!(
                "steam export build manifest schemaVersion must be {STEAM_EXPORT_BUILD_MANIFEST_SCHEMA_VERSION}"
            ));
        }
        if self.issue != 1838 {
            return Err(anyhow!("steam export build manifest issue must be 1838"));
        }
        require_local_id("steam export buildId", &self.build_id)?;
        require_local_id("steam export projectId", &self.project_id)?;
        require_text("steam export title", &self.title)?;
        require_version("steam export version", &self.version)?;
        self.source_web_build.validate()?;
        self.electron_wrapper.validate()?;
        self.windows_artifact.validate()?;
        validate_relative_path("steam export depotConfigRef", &self.depot_config_ref)?;
        require_contains(
            "steam export generatedStatePolicy",
            &self.generated_state_policy,
            &["untracked", "fixture-scoped"],
        )?;
        validate_boundary("steam export boundary", &self.boundary)?;
        Ok(())
    }
}

impl WebBuildRef {
    fn validate(&self) -> Result<()> {
        if self.runtime != "existing-web-runtime" {
            return Err(anyhow!(
                "steam export sourceWebBuild runtime must reuse existing-web-runtime"
            ));
        }
        require_contains(
            "steam export sourceWebBuild runtimeReuse",
            &self.runtime_reuse,
            &["reuse", "no new runtime", "no new engine"],
        )?;
        validate_relative_path("steam export sourceWebBuild entryHtml", &self.entry_html)?;
        if !self.entry_html.ends_with(".html") {
            return Err(anyhow!(
                "steam export sourceWebBuild entryHtml must point at html"
            ));
        }
        validate_relative_path(
            "steam export sourceWebBuild assetManifest",
            &self.asset_manifest,
        )?;
        if self.probe_mode != "preserve-read-only" {
            return Err(anyhow!(
                "steam export sourceWebBuild probeMode must be preserve-read-only"
            ));
        }
        Ok(())
    }
}

impl ElectronWrapperRef {
    fn validate(&self) -> Result<()> {
        if self.framework != "electron" {
            return Err(anyhow!("steam export wrapper framework must be electron"));
        }
        if self.steam_bridge != "steamworks.js" {
            return Err(anyhow!(
                "steam export wrapper steamBridge must be steamworks.js"
            ));
        }
        validate_relative_path("steam export wrapper mainProcess", &self.main_process)?;
        validate_relative_path("steam export wrapper preload", &self.preload)?;
        require_contains(
            "steam export wrapper trustedWritePolicy",
            &self.trusted_write_policy,
            &["read-only", "Rust/local", "no direct trusted writes"],
        )?;
        Ok(())
    }
}

impl WindowsArtifactRef {
    fn validate(&self) -> Result<()> {
        if self.platform != "windows-x64" {
            return Err(anyhow!(
                "steam export windowsArtifact platform must be windows-x64"
            ));
        }
        validate_generated_path(
            "steam export windowsArtifact executablePath",
            &self.executable_path,
        )?;
        if !self.executable_path.ends_with(".exe") {
            return Err(anyhow!(
                "steam export windowsArtifact executablePath must end with .exe"
            ));
        }
        validate_generated_path(
            "steam export windowsArtifact artifactRoot",
            &self.artifact_root,
        )?;
        validate_generated_path(
            "steam export windowsArtifact descriptorPath",
            &self.descriptor_path,
        )?;
        Ok(())
    }
}

impl SteamDepotConfig {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let config: Self =
            serde_json::from_str(input).context("failed to parse Steam depot config JSON")?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAM_DEPOT_CONFIG_SCHEMA_VERSION {
            return Err(anyhow!(
                "steam depot config schemaVersion must be {STEAM_DEPOT_CONFIG_SCHEMA_VERSION}"
            ));
        }
        require_local_id("steam depot buildId", &self.build_id)?;
        require_numeric_id("steam depot appId", &self.app_id)?;
        require_numeric_id("steam depot depotId", &self.depot_id)?;
        validate_generated_path("steam depot contentRoot", &self.content_root)?;
        require_local_id("steam depot installDir", &self.install_dir)?;
        if self.file_mappings.is_empty() {
            return Err(anyhow!("steam depot fileMappings must not be empty"));
        }
        let mut depots = BTreeSet::new();
        for mapping in &self.file_mappings {
            mapping.validate()?;
            if !depots.insert(mapping.depot.as_str()) {
                return Err(anyhow!(
                    "steam depot fileMappings must not contain duplicate depot path `{}`",
                    mapping.depot
                ));
            }
        }
        if self.upload_mode != "local-dry-run" {
            return Err(anyhow!(
                "steam depot uploadMode must be local-dry-run; credentialed upload is human/Ring-3"
            ));
        }
        validate_boundary("steam depot boundary", &self.boundary)?;
        Ok(())
    }
}

impl DepotFileMapping {
    fn validate(&self) -> Result<()> {
        validate_generated_path("steam depot fileMappings.local", &self.local)?;
        validate_relative_path("steam depot fileMappings.depot", &self.depot)?;
        Ok(())
    }
}

impl SteamPackageDescriptor {
    pub fn from_manifest_and_depot(
        manifest: &SteamExportBuildManifest,
        depot: &SteamDepotConfig,
    ) -> Result<Self> {
        manifest.validate()?;
        depot.validate()?;
        if manifest.build_id != depot.build_id {
            return Err(anyhow!(
                "steam package descriptor requires matching buildId between manifest and depot config"
            ));
        }
        let manifest_json = serde_json::to_string(manifest)
            .context("failed to canonicalize Steam export build manifest")?;
        let depot_json =
            serde_json::to_string(depot).context("failed to canonicalize Steam depot config")?;
        let artifact_seed = format!(
            "{}|{}|{}|{}|{}|{}",
            manifest.build_id,
            manifest.source_web_build.entry_html,
            manifest.electron_wrapper.framework,
            manifest.electron_wrapper.steam_bridge,
            manifest.windows_artifact.executable_path,
            depot.content_root
        );
        let descriptor = Self {
            schema_version: STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION.to_string(),
            build_id: manifest.build_id.clone(),
            project_id: manifest.project_id.clone(),
            title: manifest.title.clone(),
            version: manifest.version.clone(),
            platform: manifest.windows_artifact.platform.clone(),
            wrapper: manifest.electron_wrapper.framework.clone(),
            steam_bridge: manifest.electron_wrapper.steam_bridge.clone(),
            entry_html: manifest.source_web_build.entry_html.clone(),
            executable_path: manifest.windows_artifact.executable_path.clone(),
            depot_config_hash: sha256_prefixed(depot_json.as_bytes()),
            build_manifest_hash: sha256_prefixed(manifest_json.as_bytes()),
            artifact_hash: sha256_prefixed(artifact_seed.as_bytes()),
            release_authority: "human-ring3-required".to_string(),
        };
        descriptor.validate()?;
        Ok(descriptor)
    }

    pub fn from_json_str(input: &str) -> Result<Self> {
        let descriptor: Self =
            serde_json::from_str(input).context("failed to parse Steam package descriptor JSON")?;
        descriptor.validate()?;
        Ok(descriptor)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize Steam package descriptor")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION {
            return Err(anyhow!(
                "steam package descriptor schemaVersion must be {STEAM_PACKAGE_DESCRIPTOR_SCHEMA_VERSION}"
            ));
        }
        require_local_id("steam package descriptor buildId", &self.build_id)?;
        require_local_id("steam package descriptor projectId", &self.project_id)?;
        require_text("steam package descriptor title", &self.title)?;
        require_version("steam package descriptor version", &self.version)?;
        if self.platform != "windows-x64" {
            return Err(anyhow!(
                "steam package descriptor platform must be windows-x64"
            ));
        }
        if self.wrapper != "electron" || self.steam_bridge != "steamworks.js" {
            return Err(anyhow!(
                "steam package descriptor must record electron wrapper with steamworks.js bridge"
            ));
        }
        validate_relative_path("steam package descriptor entryHtml", &self.entry_html)?;
        validate_generated_path(
            "steam package descriptor executablePath",
            &self.executable_path,
        )?;
        require_sha256(
            "steam package descriptor depotConfigHash",
            &self.depot_config_hash,
        )?;
        require_sha256(
            "steam package descriptor buildManifestHash",
            &self.build_manifest_hash,
        )?;
        require_sha256("steam package descriptor artifactHash", &self.artifact_hash)?;
        if self.release_authority != "human-ring3-required" {
            return Err(anyhow!(
                "steam package descriptor releaseAuthority must remain human-ring3-required"
            ));
        }
        Ok(())
    }
}

fn validate_boundary(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for required in [
        "existing web runtime",
        "rust/local",
        "read-only",
        "no direct trusted writes",
        "human/ring-3",
        "no release button",
        "not layer-3",
    ] {
        if !lower.contains(&required.to_ascii_lowercase()) {
            return Err(anyhow!("{field} must state boundary `{required}`"));
        }
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_contains(field: &str, value: &str, required: &[&str]) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for needle in required {
        if !lower.contains(&needle.to_ascii_lowercase()) {
            return Err(anyhow!("{field} must contain `{needle}`"));
        }
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
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
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_numeric_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 16 || !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(anyhow!(
            "{field} must be a bounded numeric Steam id placeholder"
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
    if trimmed.is_empty() || trimmed.len() > 768 {
        return Err(anyhow!("{field} must be non-empty text up to 768 bytes"));
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_sha256(field: &str, value: &str) -> Result<()> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(anyhow!("{field} must start with sha256:"));
    };
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must contain a lowercase sha256 digest"));
    }
    Ok(())
}

fn validate_relative_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('\\') {
        return Err(anyhow!("{field} must not contain backslashes"));
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

fn validate_generated_path(field: &str, value: &str) -> Result<()> {
    validate_relative_path(field, value)?;
    let normalized = value.trim_start_matches("./");
    if ["dist/", "build/", "target/", ".omo/"]
        .iter()
        .any(|root| normalized == root.trim_end_matches('/') || normalized.starts_with(root))
    {
        return Ok(());
    }
    Err(anyhow!(
        "{field} must be under an ignored generated root (dist/, build/, target/, .omo/)"
    ))
}

fn reject_forbidden_wording(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "production-ready",
        "godot replacement",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "release button is automated",
        "market demand is automated",
        "trusted writes are authorized",
        "layer-3 cloud/mobile is go",
        "signing key",
        "credential",
        "password",
        "secret",
        "publish token",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden Steam export wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}

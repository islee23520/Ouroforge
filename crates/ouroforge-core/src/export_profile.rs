//! Export Profile Schema v1 (#721).
//!
//! An export profile is a side-effect-free declaration of how a project should
//! be packaged for an allowed local export target. Parsing and validation never
//! execute commands and never write artifacts; they only accept or reject a
//! profile with actionable diagnostics.
//!
//! The set of allowed targets is the fail-closed source of truth documented in
//! `docs/export-target-matrix-v1.md`: only `web-local` and `web-static-bundle`
//! are allowed in v1. Every other target (blocked, future, or unknown) is
//! rejected.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Component, Path};

pub const EXPORT_PROFILE_SCHEMA_VERSION: &str = "export-profile-v1";

/// Allowed v1 export targets (see `docs/export-target-matrix-v1.md`).
pub const ALLOWED_EXPORT_TARGETS: &[&str] = &["web-local", "web-static-bundle"];

/// Future, design-gated targets. Declared so diagnostics can distinguish a
/// design-gated target from an outright blocked one; both are rejected in v1.
pub const FUTURE_EXPORT_TARGETS: &[&str] = &["desktop-wrapper"];

/// Targets that are blocked by governance regardless of milestone.
pub const BLOCKED_EXPORT_TARGETS: &[&str] = &[
    "mobile",
    "console",
    "app-store",
    "steam",
    "itch",
    "hosted-deploy",
    "signed-release",
    "ci-release",
];

/// Generated/staging roots. Export output must land in one of these so that it
/// stays ignored; source inputs must never be read from one of these.
const STAGING_ROOTS: &[&str] = &["dist/", "build/", "target/", ".omo/"];

/// Roots that hold generated state. Source inputs (entry scene, asset roots)
/// must never reference them.
const GENERATED_ROOTS: &[&str] = &[
    "dist/",
    "build/",
    "target/",
    "runs/",
    ".omo/",
    "node_modules/",
];

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeProbeMode {
    /// Preserve runtime probe hooks for evidence-native QA.
    Preserve,
    /// Preserve probe hooks and additionally emit a probe-compatibility report.
    Report,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExportProfile {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub title: String,
    pub version: String,
    #[serde(rename = "exportTarget")]
    pub export_target: String,
    #[serde(rename = "entryScene")]
    pub entry_scene: String,
    #[serde(rename = "assetRoots")]
    pub asset_roots: Vec<String>,
    #[serde(rename = "outputDir")]
    pub output_dir: String,
    #[serde(rename = "runtimeProbeMode")]
    pub runtime_probe_mode: RuntimeProbeMode,
    #[serde(rename = "verificationScenarioIds")]
    pub verification_scenario_ids: Vec<String>,
    #[serde(rename = "packageMetadataRef")]
    pub package_metadata_ref: String,
    pub boundary: String,
}

impl ExportProfile {
    /// Parse a profile from JSON and validate it. Pure: no command execution,
    /// no filesystem writes.
    pub fn from_json_str(input: &str) -> Result<Self> {
        let profile: Self =
            serde_json::from_str(input).context("failed to parse Export Profile JSON")?;
        profile.validate()?;
        Ok(profile)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EXPORT_PROFILE_SCHEMA_VERSION {
            return Err(anyhow!(
                "export profile schemaVersion must be {EXPORT_PROFILE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("export profile profileId", &self.profile_id)?;
        require_local_id("export profile projectId", &self.project_id)?;
        require_text("export profile title", &self.title)?;
        require_version("export profile version", &self.version)?;

        validate_export_target(&self.export_target)?;

        // Entry scene and asset roots are source inputs: relative, path-safe,
        // and never read from generated state. A missing entry scene fails.
        validate_source_path("export profile entryScene", &self.entry_scene)?;
        require_nonempty("export profile assetRoots", self.asset_roots.len())?;
        for root in &self.asset_roots {
            validate_source_path("export profile assetRoots", root)?;
        }

        // Output goes to an ignored staging root: never overwrites source.
        validate_output_dir("export profile outputDir", &self.output_dir)?;

        require_nonempty(
            "export profile verificationScenarioIds",
            self.verification_scenario_ids.len(),
        )?;
        for id in &self.verification_scenario_ids {
            require_local_id("export profile verificationScenarioIds", id)?;
        }

        validate_source_path(
            "export profile packageMetadataRef",
            &self.package_metadata_ref,
        )?;

        validate_boundary(&self.boundary)?;
        Ok(())
    }

    /// True for targets allowed in v1.
    pub fn target_is_allowed(&self) -> bool {
        ALLOWED_EXPORT_TARGETS.contains(&self.export_target.as_str())
    }
}

fn validate_export_target(value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("export profile exportTarget must not be empty"));
    }
    if ALLOWED_EXPORT_TARGETS.contains(&trimmed) {
        return Ok(());
    }
    if BLOCKED_EXPORT_TARGETS.contains(&trimmed) {
        return Err(anyhow!(
            "export profile exportTarget `{trimmed}` is blocked: publish, deploy, signing, store, mobile, console, and CI release targets are not permitted (see docs/export-target-matrix-v1.md)"
        ));
    }
    if FUTURE_EXPORT_TARGETS.contains(&trimmed) {
        return Err(anyhow!(
            "export profile exportTarget `{trimmed}` is future/design-gated and not implemented in v1; it requires a separate design-gate issue (see docs/export-target-matrix-v1.md)"
        ));
    }
    Err(anyhow!(
        "export profile exportTarget `{trimmed}` is not an allowed v1 target; allowed targets are {ALLOWED_EXPORT_TARGETS:?} (see docs/export-target-matrix-v1.md)"
    ))
}

/// Validate a relative, path-safe component string. Explicitly rejects
/// backslashes, absolute paths, drive prefixes, `.`/`..`, and empty segments so
/// the check is correct on every platform (`Path::components()` alone would
/// treat a backslash as an ordinary character on Unix).
fn validate_relative_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('\\') {
        return Err(anyhow!("{field} must not contain backslashes"));
    }
    let path = Path::new(value);
    if path.is_absolute() || value.starts_with('/') {
        return Err(anyhow!("{field} must be a relative path"));
    }
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::ParentDir => {
                return Err(anyhow!("{field} must not escape the project with `..`"))
            }
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {
                return Err(anyhow!("{field} must be a normalized relative path"))
            }
        }
    }
    Ok(())
}

fn validate_source_path(field: &str, value: &str) -> Result<()> {
    validate_relative_path(field, value)?;
    let normalized = value.trim_start_matches("./");
    for root in GENERATED_ROOTS {
        if normalized == root.trim_end_matches('/') || normalized.starts_with(root) {
            return Err(anyhow!(
                "{field} `{value}` must be a source input, not generated state under `{root}`"
            ));
        }
    }
    Ok(())
}

fn validate_output_dir(field: &str, value: &str) -> Result<()> {
    validate_relative_path(field, value)?;
    let normalized = value.trim_start_matches("./");
    if STAGING_ROOTS
        .iter()
        .any(|root| normalized == root.trim_end_matches('/') || normalized.starts_with(root))
    {
        return Ok(());
    }
    Err(anyhow!(
        "{field} `{value}` must write into an ignored staging root ({STAGING_ROOTS:?}) so generated export output stays out of source control"
    ))
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
        "<script",
        "javascript:",
        "eval(",
        "command bridge",
        "auto-merge",
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!("{field} contains forbidden text `{forbidden}`"));
        }
    }
    Ok(())
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_boundary(value: &str) -> Result<()> {
    require_text("export profile boundary", value)?;
    let boundary = value.to_ascii_lowercase();
    for required in ["local", "evidence", "no publish", "fail closed"] {
        if !boundary.contains(required) {
            return Err(anyhow!("export profile boundary must state `{required}`"));
        }
    }
    Ok(())
}

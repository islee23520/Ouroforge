//! Export Evidence Bundle v1 (#729).
//!
//! Aggregates the export profile, plan, asset manifest, build fingerprint, and
//! verification report into a single evidence bundle with a verdict and links to
//! run/project identifiers. The bundle is written under the approved staging
//! policy (`target/ouroforge/exports/<run-id>/`), is fully validated, and
//! serializes to a dashboard/Studio-readable JSON shape.

use crate::export_asset_manifest::AssetManifest;
use crate::export_fingerprint::BuildFingerprint;
use crate::export_plan::ExportPlan;
use crate::export_profile::ExportProfile;
use crate::export_staging::staging_dir_for_run;
use crate::export_verification::ExportVerificationReport;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const EXPORT_EVIDENCE_SCHEMA_VERSION: &str = "export-evidence-bundle-v1";

/// File name written under the run staging directory.
pub const EXPORT_EVIDENCE_FILE: &str = "export-evidence.json";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ExportVerdict {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExportEvidenceLinks {
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// Optional ledger event reference; empty when no ledger link exists.
    #[serde(rename = "ledgerRef")]
    pub ledger_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExportEvidenceBundle {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub links: ExportEvidenceLinks,
    pub profile: ExportProfile,
    pub plan: ExportPlan,
    #[serde(rename = "assetManifest")]
    pub asset_manifest: AssetManifest,
    pub fingerprint: BuildFingerprint,
    pub verification: ExportVerificationReport,
    /// Package-relative screenshot evidence paths (may be empty).
    pub screenshots: Vec<String>,
    /// Package-relative world-state capture paths (may be empty).
    #[serde(rename = "worldStatePaths")]
    pub world_state_paths: Vec<String>,
    pub verdict: ExportVerdict,
}

/// Compact, dashboard/Studio-readable summary of an evidence bundle.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExportEvidenceReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "exportTarget")]
    pub export_target: String,
    pub verdict: ExportVerdict,
    #[serde(rename = "artifactCount")]
    pub artifact_count: usize,
    #[serde(rename = "assetCount")]
    pub asset_count: usize,
    #[serde(rename = "checkCount")]
    pub check_count: usize,
    #[serde(rename = "screenshotCount")]
    pub screenshot_count: usize,
}

#[allow(clippy::too_many_arguments)]
pub fn build_export_evidence(
    run_id: &str,
    ledger_ref: &str,
    profile: ExportProfile,
    plan: ExportPlan,
    asset_manifest: AssetManifest,
    fingerprint: BuildFingerprint,
    verification: ExportVerificationReport,
    screenshots: Vec<String>,
    world_state_paths: Vec<String>,
) -> Result<ExportEvidenceBundle> {
    let verdict = if verification.passed() {
        ExportVerdict::Pass
    } else {
        ExportVerdict::Fail
    };
    let bundle = ExportEvidenceBundle {
        schema_version: EXPORT_EVIDENCE_SCHEMA_VERSION.to_string(),
        links: ExportEvidenceLinks {
            run_id: run_id.to_string(),
            project_id: profile.project_id.clone(),
            ledger_ref: ledger_ref.to_string(),
        },
        profile,
        plan,
        asset_manifest,
        fingerprint,
        verification,
        screenshots,
        world_state_paths,
        verdict,
    };
    bundle.validate()?;
    Ok(bundle)
}

impl ExportEvidenceBundle {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let bundle: Self =
            serde_json::from_str(input).context("failed to parse Export Evidence Bundle JSON")?;
        bundle.validate()?;
        Ok(bundle)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize export evidence bundle")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EXPORT_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "export evidence schemaVersion must be {EXPORT_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("export evidence runId", &self.links.run_id)?;
        require_local_id("export evidence projectId", &self.links.project_id)?;
        if !self.links.ledger_ref.is_empty() {
            require_local_id("export evidence ledgerRef", &self.links.ledger_ref)?;
        }
        // Sub-artifacts re-validate.
        self.profile.validate()?;
        self.asset_manifest.validate()?;
        self.fingerprint.validate()?;

        // Project linkage is consistent across artifacts.
        if self.links.project_id != self.profile.project_id {
            return Err(anyhow!(
                "export evidence projectId must match the profile projectId"
            ));
        }

        // Verdict is consistent with the verification report.
        let expected = if self.verification.passed() {
            ExportVerdict::Pass
        } else {
            ExportVerdict::Fail
        };
        if self.verdict != expected {
            return Err(anyhow!(
                "export evidence verdict must match the verification verdict"
            ));
        }

        for path in self.screenshots.iter().chain(self.world_state_paths.iter()) {
            validate_relative_path("export evidence evidence path", path)?;
        }
        Ok(())
    }

    pub fn read_model(&self) -> ExportEvidenceReadModel {
        ExportEvidenceReadModel {
            schema_version: EXPORT_EVIDENCE_SCHEMA_VERSION.to_string(),
            run_id: self.links.run_id.clone(),
            project_id: self.links.project_id.clone(),
            export_target: self.profile.export_target.clone(),
            verdict: self.verdict.clone(),
            artifact_count: self.fingerprint.artifact_checksums.len(),
            asset_count: self.asset_manifest.entries.len(),
            check_count: self.verification.checks.len(),
            screenshot_count: self.screenshots.len(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize export evidence read model")
    }
}

/// Write the evidence bundle under the run-scoped staging directory rooted at
/// `repo_root`, returning the written path. The staging directory lives under
/// `target/` and is therefore ignored by default.
pub fn write_export_evidence(bundle: &ExportEvidenceBundle, repo_root: &Path) -> Result<PathBuf> {
    bundle.validate()?;
    let staging_rel = staging_dir_for_run(&bundle.links.run_id)?;
    let staging_dir = repo_root.join(staging_rel);
    fs::create_dir_all(&staging_dir)
        .with_context(|| format!("failed to create staging dir {}", staging_dir.display()))?;
    let path = staging_dir.join(EXPORT_EVIDENCE_FILE);
    fs::write(&path, bundle.to_json()?)
        .with_context(|| format!("failed to write evidence {}", path.display()))?;
    Ok(path)
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

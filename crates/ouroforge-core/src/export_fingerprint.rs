//! Build Fingerprint and Artifact Checksums v1 (#727).
//!
//! A build fingerprint binds an export to the exact inputs that produced it: the
//! canonical export profile, export plan, and asset manifest hashes, a
//! runtime/source version marker, and a checksum for every packaged artifact.
//!
//! Determinism policy: the fingerprint hashes only canonical JSON of the
//! profile/plan/manifest and the packaged artifact bytes (by package-relative
//! path). It never incorporates timestamps, absolute paths, run ids, or other
//! nondeterministic fields, so re-running an export from the same inputs yields
//! an identical fingerprint. Nondeterministic context (when a run happened, on
//! which machine) belongs in the export evidence bundle (#729), not here.

use crate::export_asset_manifest::AssetManifest;
use crate::export_hash::sha256_prefixed;
use crate::export_plan::ExportPlan;
use crate::export_profile::ExportProfile;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const EXPORT_FINGERPRINT_SCHEMA_VERSION: &str = "export-build-fingerprint-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactChecksum {
    #[serde(rename = "packagePath")]
    pub package_path: String,
    #[serde(rename = "contentHash")]
    pub content_hash: String,
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BuildFingerprint {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "profileHash")]
    pub profile_hash: String,
    #[serde(rename = "planHash")]
    pub plan_hash: String,
    #[serde(rename = "manifestHash")]
    pub manifest_hash: String,
    #[serde(rename = "runtimeVersion")]
    pub runtime_version: String,
    #[serde(rename = "artifactChecksums")]
    pub artifact_checksums: Vec<ArtifactChecksum>,
}

impl BuildFingerprint {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let fp: Self =
            serde_json::from_str(input).context("failed to parse Build Fingerprint JSON")?;
        fp.validate()?;
        Ok(fp)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize build fingerprint JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EXPORT_FINGERPRINT_SCHEMA_VERSION {
            return Err(anyhow!(
                "build fingerprint schemaVersion must be {EXPORT_FINGERPRINT_SCHEMA_VERSION}"
            ));
        }
        for (field, value) in [
            ("profileHash", &self.profile_hash),
            ("planHash", &self.plan_hash),
            ("manifestHash", &self.manifest_hash),
        ] {
            require_sha256(field, value)?;
        }
        require_text("runtimeVersion", &self.runtime_version)?;
        for checksum in &self.artifact_checksums {
            require_text("artifactChecksums.packagePath", &checksum.package_path)?;
            require_sha256("artifactChecksums.contentHash", &checksum.content_hash)?;
            if checksum.size == 0 {
                return Err(anyhow!(
                    "build fingerprint artifact `{}` must have non-zero size",
                    checksum.package_path
                ));
            }
        }
        Ok(())
    }
}

/// Build a deterministic fingerprint from the validated profile, plan, manifest,
/// the assembled bundle, and a runtime/source version marker.
pub fn build_fingerprint(
    profile: &ExportProfile,
    plan: &ExportPlan,
    manifest: &AssetManifest,
    bundle_root: &Path,
    runtime_version: &str,
) -> Result<BuildFingerprint> {
    require_text("runtimeVersion", runtime_version)?;
    let profile_hash = sha256_prefixed(
        serde_json::to_string(profile)
            .context("failed to canonicalize profile")?
            .as_bytes(),
    );
    let plan_hash = sha256_prefixed(
        serde_json::to_string(plan)
            .context("failed to canonicalize plan")?
            .as_bytes(),
    );
    let manifest_hash = sha256_prefixed(
        serde_json::to_string(manifest)
            .context("failed to canonicalize manifest")?
            .as_bytes(),
    );

    let mut artifact_checksums = Vec::new();
    for rel in collect_files(bundle_root)? {
        let bytes = fs::read(bundle_root.join(&rel))
            .with_context(|| format!("failed to read artifact `{rel}`"))?;
        artifact_checksums.push(ArtifactChecksum {
            content_hash: sha256_prefixed(&bytes),
            size: bytes.len() as u64,
            package_path: rel,
        });
    }
    artifact_checksums.sort_by(|a, b| a.package_path.cmp(&b.package_path));

    let fingerprint = BuildFingerprint {
        schema_version: EXPORT_FINGERPRINT_SCHEMA_VERSION.to_string(),
        profile_id: profile.profile_id.clone(),
        plan_id: plan.plan_id.clone(),
        profile_hash,
        plan_hash,
        manifest_hash,
        runtime_version: runtime_version.to_string(),
        artifact_checksums,
    };
    fingerprint.validate()?;
    Ok(fingerprint)
}

fn require_sha256(field: &str, value: &str) -> Result<()> {
    let Some(digest) = value.strip_prefix("sha256:") else {
        return Err(anyhow!(
            "build fingerprint {field} must use sha256:<64 hex>"
        ));
    };
    if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(anyhow!(
            "build fingerprint {field} must use sha256:<64 hex>"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 256 {
        return Err(anyhow!(
            "build fingerprint {field} must be non-empty text up to 256 bytes"
        ));
    }
    Ok(())
}

fn collect_files(dir: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    collect_into(dir, dir, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_into(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<()> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("failed to read dir {}", dir.display()))?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::result::Result<_, _>>()?;
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_into(root, &path, out)?;
        } else if path.is_file() {
            let rel = path
                .strip_prefix(root)
                .expect("entry under root")
                .to_string_lossy()
                .replace('\\', "/");
            out.push(rel);
        }
    }
    Ok(())
}

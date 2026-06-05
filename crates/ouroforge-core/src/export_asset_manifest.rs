//! Asset Manifest and Path Rewriting v1 (#724).
//!
//! An asset manifest records, for each packaged asset, its id, source path,
//! package output path, content hash, size, and package-relative URL. The
//! manifest can be parsed and validated from JSON (fixtures) or built from a
//! validated [`ExportPlan`] against the filesystem. Building and validation are
//! fail closed: duplicate ids, output collisions, unsafe traversal, absolute
//! paths, and missing assets are rejected.

use crate::export_hash::sha256_prefixed;
use crate::export_plan::{ExportPlan, PlannedInputKind};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const EXPORT_ASSET_MANIFEST_SCHEMA_VERSION: &str = "export-asset-manifest-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetManifestEntry {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    #[serde(rename = "outputPath")]
    pub output_path: String,
    #[serde(rename = "contentHash")]
    pub content_hash: String,
    pub size: u64,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetManifest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub entries: Vec<AssetManifestEntry>,
}

impl AssetManifest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let manifest: Self =
            serde_json::from_str(input).context("failed to parse Asset Manifest JSON")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize asset manifest JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EXPORT_ASSET_MANIFEST_SCHEMA_VERSION {
            return Err(anyhow!(
                "asset manifest schemaVersion must be {EXPORT_ASSET_MANIFEST_SCHEMA_VERSION}"
            ));
        }
        require_local_id("asset manifest projectId", &self.project_id)?;

        let mut ids = BTreeSet::new();
        let mut output_paths = BTreeSet::new();
        let mut urls = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !ids.insert(entry.asset_id.as_str()) {
                return Err(anyhow!(
                    "asset manifest has duplicate assetId `{}`",
                    entry.asset_id
                ));
            }
            if !output_paths.insert(entry.output_path.as_str()) {
                return Err(anyhow!(
                    "asset manifest has output collision on `{}`",
                    entry.output_path
                ));
            }
            if !urls.insert(entry.url.as_str()) {
                return Err(anyhow!(
                    "asset manifest has output collision on url `{}`",
                    entry.url
                ));
            }
        }
        Ok(())
    }

    /// Rewrite a runtime asset reference to its package-relative URL, where the
    /// reference exactly matches a known source or output path. Returns `None`
    /// for references that are not explicitly mapped (no rewrite is applied).
    pub fn rewrite_reference(&self, reference: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.source_path == reference || e.output_path == reference)
            .map(|e| e.url.as_str())
    }
}

impl AssetManifestEntry {
    fn validate(&self) -> Result<()> {
        require_local_id("asset manifest entry assetId", &self.asset_id)?;
        validate_relative_path("asset manifest entry sourcePath", &self.source_path)?;
        validate_relative_path("asset manifest entry outputPath", &self.output_path)?;
        validate_relative_path("asset manifest entry url", &self.url)?;
        if self.size == 0 {
            return Err(anyhow!(
                "asset manifest entry `{}` must have a non-zero size",
                self.asset_id
            ));
        }
        let Some(digest) = self.content_hash.strip_prefix("sha256:") else {
            return Err(anyhow!(
                "asset manifest entry `{}` contentHash must use sha256:<64 hex>",
                self.asset_id
            ));
        };
        if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(anyhow!(
                "asset manifest entry `{}` contentHash must use sha256:<64 hex>",
                self.asset_id
            ));
        }
        Ok(())
    }
}

/// Build an asset manifest from a validated plan by hashing the packaged assets
/// on disk. Source inputs are resolved relative to `repo_root`.
pub fn build_asset_manifest(
    plan: &ExportPlan,
    repo_root: &Path,
    project_id: &str,
) -> Result<AssetManifest> {
    let mut entries = Vec::new();
    for input in plan
        .source_inputs
        .iter()
        .filter(|i| i.kind == PlannedInputKind::AssetRoot)
    {
        // Re-validate the asset root before joining it to `repo_root`. `build_asset_manifest`
        // accepts any `ExportPlan` value, which may have been constructed or deserialized
        // outside `ExportPlan::from_profile_json` (and thus never passed export-profile
        // validation). Without this guard a forged plan could carry an absolute or `..`
        // path and make manifest generation read and hash files outside the asset roots,
        // or a path under a blocked prefix such as `.git/` or `secrets/`. Fail closed. (#724)
        validate_relative_path("asset manifest asset root", &input.path)?;
        if let Some(blocked) = plan.blocked_files.iter().find(|prefix| {
            input.path == prefix.trim_end_matches('/') || input.path.starts_with(prefix.as_str())
        }) {
            return Err(anyhow!(
                "asset manifest refuses asset root `{}` under blocked prefix `{}`",
                input.path,
                blocked
            ));
        }
        let segment = last_segment(&input.path);
        let source_dir = repo_root.join(&input.path);
        if !source_dir.is_dir() {
            return Err(anyhow!(
                "asset manifest is missing asset root `{}` on disk",
                input.path
            ));
        }
        for rel in collect_files(&source_dir)? {
            if rel.split('/').any(|p| p.starts_with('.')) {
                continue;
            }
            let source_path = format!("{}/{}", input.path.trim_end_matches('/'), rel);
            let output_path = format!("assets/{segment}/{rel}");
            let bytes = fs::read(source_dir.join(&rel))
                .with_context(|| format!("asset manifest cannot read `{source_path}`"))?;
            entries.push(AssetManifestEntry {
                asset_id: derive_asset_id(&output_path),
                content_hash: sha256_prefixed(&bytes),
                size: bytes.len() as u64,
                url: output_path.clone(),
                output_path,
                source_path,
            });
        }
    }
    entries.sort_by(|a, b| a.output_path.cmp(&b.output_path));
    let manifest = AssetManifest {
        schema_version: EXPORT_ASSET_MANIFEST_SCHEMA_VERSION.to_string(),
        project_id: project_id.to_string(),
        entries,
    };
    manifest.validate()?;
    Ok(manifest)
}

fn derive_asset_id(output_path: &str) -> String {
    output_path
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 160
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
    if value.trim().is_empty() || value.len() > 512 {
        return Err(anyhow!("{field} must be non-empty text up to 512 bytes"));
    }
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
            Component::ParentDir => return Err(anyhow!("{field} must not escape with `..`")),
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {
                return Err(anyhow!("{field} must be a normalized relative path"))
            }
        }
    }
    Ok(())
}

fn last_segment(path: &str) -> &str {
    path.trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(path)
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

//! Asset Import and Atlas Path v1 (#1637).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! This is a validate-then-load **import/atlas path**, not a new runtime. It
//! reuses the existing [`crate::ProjectAssetManifest`] loader and its atlas
//! integrity validation (`validate_assets` → `validate_sprite_atlases` →
//! `atlas.validate_integrity`), then builds the runtime-shaped import report the
//! runtime asset loader already understands. It adds **no** second importer,
//! image decoder, or atlas packer.
//!
//! Governance: a generated asset is only imported after it has passed the
//! asset-QA gate (#1636); [`enforce_asset_qa`] requires a passing
//! [`AssetQaGateVerdict`] for every generated asset. Source-like assets are
//! existing project inputs and do not require asset-QA. The path **fails
//! closed**: a malformed manifest, a content-hash mismatch, a missing file, an
//! out-of-bounds atlas frame, or a generated asset lacking a QA pass all block
//! the import.

use crate::{ProjectAssetClassification, ProjectAssetManifest, ProjectAssetType};
use anyhow::{anyhow, Result};
use ouroforge_evaluator::asset_qa_gate::AssetQaGateVerdict;
use serde::Serialize;
use std::path::Path;

/// Schema version for the asset import report.
pub const ASSET_IMPORT_SCHEMA_VERSION: &str = "ouroforge.asset-import.v1";

/// A single asset as loaded into the runtime-shaped import report.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ImportedAsset {
    pub id: String,
    #[serde(rename = "type")]
    pub asset_type: ProjectAssetType,
    pub classification: ProjectAssetClassification,
    #[serde(rename = "atlasFrameCount")]
    pub atlas_frame_count: usize,
    #[serde(rename = "atlasAnimationCount")]
    pub atlas_animation_count: usize,
}

/// The result of importing a validated asset manifest: a runtime-shaped summary
/// the existing asset loader already understands. Additive; it references the
/// manifest, it does not duplicate asset bytes.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AssetImportReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    pub assets: Vec<ImportedAsset>,
    #[serde(rename = "atlasFrameTotal")]
    pub atlas_frame_total: usize,
    #[serde(rename = "atlasAnimationTotal")]
    pub atlas_animation_total: usize,
}

impl AssetImportReport {
    /// The ids of the generated assets in this import (those that must have
    /// passed asset-QA).
    pub fn generated_asset_ids(&self) -> Vec<&str> {
        self.assets
            .iter()
            .filter(|asset| asset.classification == ProjectAssetClassification::Generated)
            .map(|asset| asset.id.as_str())
            .collect()
    }
}

fn build_report(manifest: &ProjectAssetManifest) -> AssetImportReport {
    let mut atlas_frame_total = 0;
    let mut atlas_animation_total = 0;
    let assets = manifest
        .assets
        .iter()
        .map(|asset| {
            let (frame_count, animation_count) = match &asset.atlas {
                Some(atlas) => (atlas.frames.len(), atlas.animations.len()),
                None => (0, 0),
            };
            atlas_frame_total += frame_count;
            atlas_animation_total += animation_count;
            ImportedAsset {
                id: asset.id.clone(),
                asset_type: asset.asset_type,
                classification: asset.classification,
                atlas_frame_count: frame_count,
                atlas_animation_count: animation_count,
            }
        })
        .collect();
    AssetImportReport {
        schema_version: ASSET_IMPORT_SCHEMA_VERSION.to_string(),
        manifest_id: manifest.id.clone(),
        assets,
        atlas_frame_total,
        atlas_animation_total,
    }
}

/// Validate-then-load a parsed manifest against `base_dir`, reusing the existing
/// manifest/loader and atlas-integrity validation, then build the import report.
/// Fails closed on a malformed manifest, a content-hash mismatch, a missing
/// file, or an out-of-bounds atlas frame.
///
/// Private on purpose: it produces an import report **without** the asset-QA
/// gate, so it is never a public entry point. Every public import path
/// ([`import_validated_assets`], [`import_from_json`]) wraps this with
/// [`enforce_asset_qa`] so a generated asset can never be imported without a
/// passing asset-QA verdict.
fn load_validated_manifest(
    manifest: &ProjectAssetManifest,
    base_dir: &Path,
) -> Result<AssetImportReport> {
    // Reuse: schema validation, per-asset integrity/hash, and atlas-frame bounds
    // are all enforced by the existing ProjectAssetManifest validation.
    manifest.validate_assets(base_dir)?;
    Ok(build_report(manifest))
}

/// Validate a parsed manifest against `base_dir` **without** importing it. A
/// validation-only entry point (it returns no import report and promotes
/// nothing): schema, per-asset integrity/hash, and atlas-frame bounds, reusing
/// the existing manifest/loader. Fails closed on any validation error.
pub fn validate_manifest(manifest: &ProjectAssetManifest, base_dir: &Path) -> Result<()> {
    manifest.validate_assets(base_dir)?;
    Ok(())
}

/// Parse, validate-then-load, and asset-QA-gate an asset manifest from JSON
/// against `base_dir`. Like [`import_validated_assets`], every generated asset
/// must have a passing asset-QA verdict (#1636); fails closed otherwise.
pub fn import_from_json(
    manifest_json: &str,
    base_dir: &Path,
    qa: &[AssetQaGateVerdict],
) -> Result<AssetImportReport> {
    let manifest = ProjectAssetManifest::from_json_str(manifest_json)?;
    import_validated_assets(&manifest, base_dir, qa)
}

/// Enforce the asset-QA gate over an import: every generated asset must have a
/// passing asset-QA verdict (#1636). Source-like assets are existing project
/// inputs and do not require asset-QA. Fails closed on the first generated asset
/// without a passing verdict.
pub fn enforce_asset_qa(report: &AssetImportReport, qa: &[AssetQaGateVerdict]) -> Result<()> {
    for asset in &report.assets {
        if asset.classification != ProjectAssetClassification::Generated {
            continue;
        }
        let passed = qa
            .iter()
            .any(|verdict| verdict.asset_id == asset.id && verdict.state.is_pass());
        if !passed {
            return Err(anyhow!(
                "imported generated asset \"{}\" has not passed the asset-QA gate; no unverified asset is imported",
                asset.id
            ));
        }
    }
    Ok(())
}

/// Import a validated manifest and enforce the asset-QA gate over its generated
/// assets. The single entry point for promoting generated assets into the
/// runtime: validate-then-load, then require asset-QA. Fails closed.
pub fn import_validated_assets(
    manifest: &ProjectAssetManifest,
    base_dir: &Path,
    qa: &[AssetQaGateVerdict],
) -> Result<AssetImportReport> {
    let report = load_validated_manifest(manifest, base_dir)?;
    enforce_asset_qa(&report, qa)?;
    Ok(report)
}

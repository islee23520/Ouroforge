//! Contract test for Asset Import and Atlas Path v1 (#1637).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! These tests machine-check the import/atlas path: a validated manifest loads
//! into the runtime-shaped import report; a malformed manifest (content-hash
//! mismatch / missing file) is rejected fail-closed; an out-of-bounds atlas
//! frame is rejected (atlas integrity); and a generated asset is only imported
//! after it has passed the asset-QA gate (#1636).
//!
//! The path reuses the existing `ProjectAssetManifest` loader and atlas
//! validation; these tests reuse the existing Asset Pipeline v1 regression
//! fixtures (real assets and hashes) rather than committing new binaries.

use std::path::PathBuf;

use ouroforge_core::asset_import::{
    enforce_asset_qa, import_from_json, import_validated_assets, validate_manifest,
    AssetImportReport, ImportedAsset, ASSET_IMPORT_SCHEMA_VERSION,
};
use ouroforge_core::{ProjectAssetClassification, ProjectAssetManifest, ProjectAssetType};
use ouroforge_evaluator::asset_qa_gate::{AssetQaGateState, AssetQaGateVerdict};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/asset-pipeline-v1-regression")
}

fn read_manifest(relative: &str) -> ProjectAssetManifest {
    let text = std::fs::read_to_string(fixture_root().join(relative))
        .unwrap_or_else(|_| panic!("fixture exists: {relative}"));
    ProjectAssetManifest::from_json_str(&text).expect("fixture manifest schema parses")
}

#[test]
fn valid_manifest_imports_and_loads() {
    let manifest = read_manifest("asset-manifest.json");
    // The source-like fixture imports with no asset-QA verdicts (no generated
    // assets), exercising the full QA-gated import path.
    let report =
        import_validated_assets(&manifest, &fixture_root(), &[]).expect("valid manifest imports");

    assert_eq!(report.schema_version, ASSET_IMPORT_SCHEMA_VERSION);
    assert_eq!(report.manifest_id, "asset_pipeline_v1_regression_assets");
    // The five top-level assets load: sheet, atlas, tileset, tilemap, audio.
    assert_eq!(report.assets.len(), 5);
    // The sprite atlas contributes frames and at least one animation.
    assert!(report.atlas_frame_total > 0);
    assert!(report.atlas_animation_total >= 1);
    // The fixture is source-like, so it imports with no asset-QA verdicts.
    assert!(report.generated_asset_ids().is_empty());
    enforce_asset_qa(&report, &[]).expect("source-like assets need no asset-QA");
}

#[test]
fn import_from_json_round_trips() {
    let text = std::fs::read_to_string(fixture_root().join("asset-manifest.json")).expect("read");
    let report = import_from_json(&text, &fixture_root(), &[]).expect("valid manifest imports");
    assert_eq!(report.assets.len(), 5);
}

#[test]
fn content_hash_mismatch_is_rejected_fail_closed() {
    let manifest = read_manifest("invalid/hash-mismatch.asset-manifest.json");
    let error = validate_manifest(&manifest, &fixture_root())
        .expect_err("a content-hash mismatch must be rejected fail-closed");
    assert!(
        error.to_string().contains("contentHash mismatch"),
        "unexpected error: {error}"
    );
}

#[test]
fn missing_file_is_rejected_fail_closed() {
    let manifest = read_manifest("invalid/missing-asset.asset-manifest.json");
    let error = validate_manifest(&manifest, &fixture_root())
        .expect_err("a missing asset file must be rejected fail-closed");
    assert!(
        error.to_string().contains("missing file"),
        "unexpected error: {error}"
    );
}

#[test]
fn out_of_bounds_atlas_frame_is_rejected_atlas_integrity() {
    let manifest = read_manifest("invalid/atlas-frame-out-of-bounds.asset-manifest.json");
    let error = validate_manifest(&manifest, &fixture_root())
        .expect_err("an out-of-bounds atlas frame must be rejected");
    assert!(
        error.to_string().contains("outside atlas image bounds"),
        "unexpected error: {error}"
    );
}

fn generated_report(asset_id: &str) -> AssetImportReport {
    AssetImportReport {
        schema_version: ASSET_IMPORT_SCHEMA_VERSION.to_string(),
        manifest_id: "generated-manifest".to_string(),
        assets: vec![ImportedAsset {
            id: asset_id.to_string(),
            asset_type: ProjectAssetType::Image,
            classification: ProjectAssetClassification::Generated,
            atlas_frame_count: 0,
            atlas_animation_count: 0,
        }],
        atlas_frame_total: 0,
        atlas_animation_total: 0,
    }
}

fn qa_verdict(asset_id: &str, state: AssetQaGateState) -> AssetQaGateVerdict {
    AssetQaGateVerdict {
        asset_id: asset_id.to_string(),
        state,
        reason: "fixture verdict".to_string(),
        evidence_refs: vec!["runs/run-1/evidence/asset-qa/asset.json".to_string()],
    }
}

#[test]
fn generated_asset_requires_a_passing_asset_qa_verdict() {
    let report = generated_report("hero-idle");

    // No asset-QA verdict at all: fail closed.
    let error = enforce_asset_qa(&report, &[])
        .expect_err("a generated asset with no asset-QA verdict must be rejected");
    assert!(
        error
            .to_string()
            .contains("has not passed the asset-QA gate"),
        "unexpected error: {error}"
    );

    // A failing asset-QA verdict: fail closed.
    let error = enforce_asset_qa(
        &report,
        &[qa_verdict("hero-idle", AssetQaGateState::StyleInconsistent)],
    )
    .expect_err("a generated asset with a failing asset-QA verdict must be rejected");
    assert!(error.to_string().contains("has not passed"));

    // A passing asset-QA verdict for the right asset: imported.
    enforce_asset_qa(&report, &[qa_verdict("hero-idle", AssetQaGateState::Pass)])
        .expect("a generated asset with a passing asset-QA verdict imports");

    // A passing verdict for a different asset does not satisfy the gate.
    let error = enforce_asset_qa(&report, &[qa_verdict("other", AssetQaGateState::Pass)])
        .expect_err("a passing verdict for a different asset must not satisfy the gate");
    assert!(error.to_string().contains("has not passed"));
}

#[test]
fn import_validated_assets_combines_load_and_asset_qa() {
    // The source-like fixture imports with no asset-QA verdicts because it has
    // no generated assets.
    let manifest = read_manifest("asset-manifest.json");
    let report = import_validated_assets(&manifest, &fixture_root(), &[])
        .expect("validated source-like manifest imports");
    assert_eq!(report.assets.len(), 5);
}

#[test]
fn import_from_json_blocks_a_generated_asset_without_asset_qa() {
    use ouroforge_core::hash_project_asset_file;

    // Build a real, on-disk generated-image manifest so the public JSON import
    // path runs full validation and then the asset-QA gate.
    let dir = std::env::temp_dir().join(format!("ouro-asset-import-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let asset_path = dir.join("hero.png");
    std::fs::write(&asset_path, b"fake-png-bytes-for-import-test").expect("write asset");
    let hash = hash_project_asset_file(&asset_path).expect("hash asset");

    let manifest_json = format!(
        r#"{{
  "schemaVersion": "asset-manifest-v1",
  "id": "generated_assets",
  "assets": [
    {{
      "id": "hero-idle",
      "type": "image",
      "path": "hero.png",
      "contentHash": {{ "algorithm": "{algo}", "value": "{val}" }},
      "classification": "generated",
      "dimensions": {{ "width": 32, "height": 32 }}
    }}
  ]
}}"#,
        algo = hash.algorithm,
        val = hash.value
    );

    // The public JSON import path must block a generated asset with no asset-QA
    // pass — even though the manifest itself is valid and the file exists.
    let error = import_from_json(&manifest_json, &dir, &[])
        .expect_err("a generated asset imported via JSON without asset-QA must be rejected");
    assert!(
        error
            .to_string()
            .contains("has not passed the asset-QA gate"),
        "unexpected error: {error}"
    );

    // With a passing asset-QA verdict the same manifest imports.
    let report = import_from_json(
        &manifest_json,
        &dir,
        &[qa_verdict("hero-idle", AssetQaGateState::Pass)],
    )
    .expect("a generated asset with a passing asset-QA verdict imports via JSON");
    assert_eq!(report.generated_asset_ids(), vec!["hero-idle"]);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn docs_record_the_import_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/asset-pipeline-design.md"))
        .expect("asset pipeline design doc exists");
    assert!(
        doc.contains("#1637"),
        "design gate doc records the import/atlas follow-up (#1637)"
    );
    assert!(
        doc.contains("manifest/loader"),
        "doc records reuse of the existing manifest/loader"
    );
}

//! Asset Manifest and Path Rewriting v1 contract (#724).

use ouroforge_core::export_asset_manifest::{
    build_asset_manifest, AssetManifest, EXPORT_ASSET_MANIFEST_SCHEMA_VERSION,
};
use ouroforge_core::export_plan::ExportPlan;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn manifest_fixture(rel: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/export-asset-manifest-v1")
            .join(rel),
    )
    .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn bundle_profile() -> String {
    std::fs::read_to_string(
        repo_root().join("examples/export-bundle-v1/export-profile.fixture.json"),
    )
    .expect("bundle profile readable")
}

#[test]
fn valid_declared_manifest_parses() {
    let manifest =
        AssetManifest::from_json_str(&manifest_fixture("asset-manifest.valid.fixture.json"))
            .expect("valid manifest parses");
    assert_eq!(
        manifest.schema_version,
        EXPORT_ASSET_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(manifest.entries.len(), 1);
    let entry = &manifest.entries[0];
    assert!(entry.content_hash.starts_with("sha256:"));
    assert_eq!(entry.url, "assets/assets/sprites/hero.json");
}

#[test]
fn duplicate_id_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/duplicate-id.json"))
        .expect_err("duplicate id rejected");
    assert!(err.to_string().contains("duplicate assetId"));
}

#[test]
fn path_traversal_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/path-traversal.json"))
        .expect_err("traversal rejected");
    assert!(err.to_string().contains(".."));
}

#[test]
fn absolute_path_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/absolute-path.json"))
        .expect_err("absolute path rejected");
    assert!(err.to_string().contains("relative"));
}

#[test]
fn output_collision_is_rejected() {
    let err = AssetManifest::from_json_str(&manifest_fixture("invalid/output-collision.json"))
        .expect_err("output collision rejected");
    assert!(err.to_string().contains("output collision"));
}

#[test]
fn builds_manifest_from_plan_with_real_hash_and_size() {
    let plan = ExportPlan::from_profile_json(&bundle_profile()).expect("plan");
    let manifest = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect("manifest builds from disk");
    assert_eq!(manifest.entries.len(), 1);
    let entry = &manifest.entries[0];
    assert_eq!(
        entry.source_path,
        "examples/export-bundle-v1/fixture-game/assets/sprites/hero.json"
    );
    assert_eq!(entry.output_path, "assets/assets/sprites/hero.json");
    assert!(entry.content_hash.starts_with("sha256:"));
    assert_eq!(entry.content_hash.len(), "sha256:".len() + 64);
    assert!(entry.size > 0);
    // Built manifest re-validates and round-trips through JSON.
    let json = manifest.to_json().unwrap();
    assert_eq!(AssetManifest::from_json_str(&json).unwrap(), manifest);
}

#[test]
fn build_is_deterministic() {
    let plan = ExportPlan::from_profile_json(&bundle_profile()).unwrap();
    let a = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture").unwrap();
    let b = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture").unwrap();
    assert_eq!(a, b);
}

#[test]
fn build_fails_closed_on_missing_asset_root() {
    let profile = std::fs::read_to_string(
        repo_root().join("examples/export-asset-manifest-v1/missing-asset-profile.fixture.json"),
    )
    .unwrap();
    let plan = ExportPlan::from_profile_json(&profile).expect("profile plans");
    let err = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect_err("missing asset root rejected");
    assert!(err.to_string().contains("missing asset root"));
}

#[test]
fn build_fails_closed_on_forged_traversal_asset_root() {
    use ouroforge_core::export_plan::PlannedInputKind;
    // A plan can be deserialized/mutated outside ExportPlan::from_profile_json, so the
    // manifest builder must re-validate asset roots instead of trusting the plan. (#724)
    let mut plan = ExportPlan::from_profile_json(&bundle_profile()).expect("plan");
    let root = plan
        .source_inputs
        .iter_mut()
        .find(|input| input.kind == PlannedInputKind::AssetRoot)
        .expect("plan has an asset root");
    root.path = "../../../../../../etc".to_string();
    let err = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect_err("forged traversal asset root rejected");
    assert!(
        err.to_string().contains(".."),
        "expected traversal rejection, got: {err}"
    );
}

#[test]
fn build_fails_closed_on_blocked_prefix_asset_root() {
    use ouroforge_core::export_plan::PlannedInputKind;
    let mut plan = ExportPlan::from_profile_json(&bundle_profile()).expect("plan");
    let root = plan
        .source_inputs
        .iter_mut()
        .find(|input| input.kind == PlannedInputKind::AssetRoot)
        .expect("plan has an asset root");
    // `secrets/` is in the plan's blocked-file policy; a forged root under it must fail.
    root.path = "secrets/leak".to_string();
    let err = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture")
        .expect_err("blocked-prefix asset root rejected");
    assert!(
        err.to_string().contains("blocked prefix"),
        "expected blocked-prefix rejection, got: {err}"
    );
}

#[test]
fn rewrites_only_explicitly_mapped_references() {
    let plan = ExportPlan::from_profile_json(&bundle_profile()).unwrap();
    let manifest = build_asset_manifest(&plan, &repo_root(), "proj_export_bundle_fixture").unwrap();
    // Source path and output path both rewrite to the package URL.
    assert_eq!(
        manifest
            .rewrite_reference("examples/export-bundle-v1/fixture-game/assets/sprites/hero.json"),
        Some("assets/assets/sprites/hero.json")
    );
    assert_eq!(
        manifest.rewrite_reference("assets/assets/sprites/hero.json"),
        Some("assets/assets/sprites/hero.json")
    );
    // Unknown references are not rewritten.
    assert_eq!(manifest.rewrite_reference("assets/unknown.png"), None);
}

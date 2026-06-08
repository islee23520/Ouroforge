//! Scenario Coverage v50: Post-Launch Patch Regression Suite (#1848).

use ouroforge_core::patch_reverify::{reverify_and_repackage, PatchReverifyPlan};
use ouroforge_core::save_migration::{
    incompatible_save_evidence, migrate_save_forward, SaveMigrationStatus,
};
use ouroforge_core::save_profile_scale::SaveStore;
use ouroforge_core::steam_export_build::{
    SteamDepotConfig, SteamExportBuildManifest, SteamPackageDescriptor,
};
use ouroforge_core::ProjectAssetManifest;
use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

#[test]
fn v50_matrix_enumerates_required_regression_cases() {
    let matrix: Value = serde_json::from_str(&read(
        "examples/post-launch-patch-v1/scenario-coverage-v50/matrix.fixture.json",
    ))
    .expect("matrix parses");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v50-post-launch-patch-v1"
    );
    assert_eq!(matrix["issue"], 1848);
    assert_eq!(matrix["assertionMode"], "state-shape-only");
    assert_eq!(matrix["requiresNetwork"], false);
    assert_eq!(matrix["requiresLiveBrowser"], false);
    let ids: Vec<_> = matrix["cases"]
        .as_array()
        .expect("cases array")
        .iter()
        .map(|case| case["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        [
            "v50.patch-reverify.pass",
            "v50.patch-reverify.fail",
            "v50.save-migration.forward",
            "v50.save-migration.incompatible",
            "v50.non-patched-build-save-backcompat"
        ]
    );
}

#[test]
fn v50_patch_reverify_pass_and_fail_states_are_locked() {
    let manifest = SteamExportBuildManifest::from_json_str(&read(
        "examples/steam-export-build-v1/build-manifest.valid.fixture.json",
    ))
    .expect("steam manifest validates");
    let depot = SteamDepotConfig::from_json_str(&read(
        "examples/steam-export-build-v1/depot-config.valid.fixture.json",
    ))
    .expect("steam depot validates");

    let passing = PatchReverifyPlan::from_json_str(&read(
        "examples/post-launch-patch-v1/patch-reverify.pass.fixture.json",
    ))
    .expect("passing patch fixture validates");
    let repackage = reverify_and_repackage(&passing, &manifest, &depot)
        .expect("passing full gate set allows repackage");
    assert_eq!(repackage.status, "repackaged-after-reverify");
    assert_eq!(repackage.release_authority, "human-ring3-required");

    let failing = PatchReverifyPlan::from_json_str(&read(
        "examples/post-launch-patch-v1/invalid/patch-reverify.fail.fixture.json",
    ))
    .expect("failing patch fixture is structurally valid");
    let error = reverify_and_repackage(&failing, &manifest, &depot)
        .expect_err("failing gate blocks repackage");
    assert!(error
        .to_string()
        .contains("cannot re-package before full re-verify passes"));
    assert!(error.to_string().contains("scenario-coverage"));
}

#[test]
fn v50_save_forward_and_incompatible_states_are_locked() {
    let legacy = read("examples/save-migration-v1/legacy-save.v0.fixture.json");
    let (store, evidence) =
        migrate_save_forward("v50-save-forward", &legacy).expect("legacy save migrates forward");
    assert_eq!(evidence.status, SaveMigrationStatus::Migrated);
    assert!(evidence.replay_digests_preserved);
    store.verify_integrity().expect("migrated save is sealed");

    let incompatible = read("examples/save-migration-v1/invalid/incompatible-save.fixture.json");
    let error = migrate_save_forward("v50-save-incompatible", &incompatible)
        .expect_err("incompatible save fails closed");
    assert!(error.to_string().contains("incompatible"));
    let incompatible_evidence = incompatible_save_evidence("v50-save-incompatible", &incompatible)
        .expect("incompatible evidence validates");
    assert_eq!(
        incompatible_evidence.status,
        SaveMigrationStatus::Incompatible
    );
    assert!(incompatible_evidence.migrated_hash.is_none());
}

#[test]
fn v50_non_patched_build_and_save_backcompat_golden_remains_valid() {
    let golden: Value = serde_json::from_str(&read(
        "examples/post-launch-patch-v1/scenario-coverage-v50/non-patched-build-save.golden.json",
    ))
    .expect("golden parses");
    assert_eq!(
        golden["schemaVersion"],
        "scenario-coverage-v50-non-patched-build-save-golden-v1"
    );
    assert_eq!(golden["runtime"], "existing-web-runtime");

    let entry_html = golden["entryHtml"].as_str().expect("entryHtml");
    let asset_manifest = golden["assetManifest"].as_str().expect("assetManifest");
    assert!(read(entry_html).contains("__OUROFORGE__") || read(entry_html).contains("runtime.js"));
    let manifest = ProjectAssetManifest::from_json_str(&read(asset_manifest))
        .expect("existing asset manifest remains valid");
    assert!(!manifest.assets.is_empty());

    let steam_manifest = SteamExportBuildManifest::from_json_str(&read(
        golden["steamBuildManifestRef"].as_str().unwrap(),
    ))
    .expect("steam build manifest remains valid");
    let steam_depot =
        SteamDepotConfig::from_json_str(&read(golden["steamDepotConfigRef"].as_str().unwrap()))
            .expect("steam depot remains valid");
    let descriptor = SteamPackageDescriptor::from_manifest_and_depot(&steam_manifest, &steam_depot)
        .expect("existing descriptor remains derivable");
    assert_eq!(descriptor.release_authority, "human-ring3-required");

    let legacy_save = read(golden["saveProfileRef"].as_str().unwrap());
    let store = SaveStore::from_legacy_v0_json(&legacy_save)
        .expect("legacy save profile remains forward-compatible");
    store
        .verify_integrity()
        .expect("legacy save migrates and seals");
}

#[test]
fn v50_docs_preserve_generated_state_wording_compatibility_and_governance() {
    let combined = format!(
        "{}\n{}\n{}",
        read("docs/scenario-coverage-v50.md"),
        read("examples/post-launch-patch-v1/scenario-coverage-v50/matrix.fixture.json"),
        read("examples/post-launch-patch-v1/scenario-coverage-v50/non-patched-build-save.golden.json")
    );
    for required in [
        "state/shape-only",
        "read-only",
        "no trusted write",
        "fixture-scoped",
        "no auto-merge",
        "#1 and #23 remain open",
    ] {
        assert!(
            combined.contains(required),
            "missing boundary text {required}"
        );
    }
    for forbidden in [
        "production-ready engine",
        "Godot replacement is authorized",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "automated fun score is authorized",
        "quality score is authorized",
        "Layer-3 cloud/mobile is GO",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording leaked {forbidden}"
        );
    }

    // Hermetic by default: the live GitHub anchor check is opt-in so `cargo test`
    // does not depend on network / gh-auth / external issue state. The doc/governance
    // assertions above already run unconditionally. Set
    // OUROFORGE_LIVE_GOVERNANCE_ANCHOR_CHECK=1 to verify issues #1/#23 are OPEN.
    if std::env::var("OUROFORGE_LIVE_GOVERNANCE_ANCHOR_CHECK").is_err() {
        return;
    }
    let root = repo_root();
    for issue in ["1", "23"] {
        let output = std::process::Command::new("gh")
            .args([
                "issue",
                "view",
                issue,
                "--repo",
                "shaun0927/Ouroforge",
                "--json",
                "state",
                "--jq",
                ".state",
            ])
            .current_dir(&root)
            .output()
            .unwrap_or_else(|error| panic!("gh issue {issue}: {error}"));
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "OPEN");
    }
}

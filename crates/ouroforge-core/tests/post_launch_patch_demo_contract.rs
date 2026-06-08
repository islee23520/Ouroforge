//! Smoke tests for Post-Launch Patch Demo v1 (#1847).

use std::path::{Path, PathBuf};

use ouroforge_core::patch_reverify::{reverify_and_repackage, PatchReverifyPlan};
use ouroforge_core::save_migration::{migrate_save_forward, SaveMigrationStatus};
use ouroforge_core::steam_export_build::{SteamDepotConfig, SteamExportBuildManifest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DemoManifest {
    schema_version: String,
    issue: u32,
    demo_id: String,
    patch_reverify_plan_ref: String,
    steam_build_manifest_ref: String,
    steam_depot_config_ref: String,
    legacy_save_ref: String,
    expected_repackage_status: String,
    expected_migration_status: String,
    network_policy: String,
    browser_policy: String,
    generated_state_policy: String,
    boundary: String,
}

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

fn manifest() -> DemoManifest {
    serde_json::from_str(&read(
        "examples/post-launch-patch-v1/demo/manifest.fixture.json",
    ))
    .expect("demo manifest parses")
}

#[test]
fn demo_reverifies_repackages_and_migrates_save_deterministically() {
    let manifest = manifest();
    assert_eq!(manifest.schema_version, "post-launch-patch-demo-v1");
    assert_eq!(manifest.issue, 1847);
    assert_eq!(manifest.demo_id, "post-launch-patch-demo-001");
    assert_eq!(manifest.network_policy, "offline-fixture-only-no-network");
    assert_eq!(manifest.browser_policy, "no-live-browser-required");
    assert!(manifest.generated_state_policy.contains("untracked"));
    for required in [
        "Rust/local",
        "existing patch re-verify",
        "save/restore",
        "replay-digest",
        "read-only",
        "no network",
        "no live browser",
        "no direct trusted writes",
        "#1 and #23 remain open",
    ] {
        assert!(
            manifest.boundary.contains(required),
            "missing boundary text {required}"
        );
    }

    let plan = PatchReverifyPlan::from_json_str(&read(&manifest.patch_reverify_plan_ref))
        .expect("patch re-verify plan validates");
    let steam_manifest =
        SteamExportBuildManifest::from_json_str(&read(&manifest.steam_build_manifest_ref))
            .expect("steam build manifest validates");
    let steam_depot = SteamDepotConfig::from_json_str(&read(&manifest.steam_depot_config_ref))
        .expect("steam depot config validates");
    let first_repackage = reverify_and_repackage(&plan, &steam_manifest, &steam_depot)
        .expect("patch re-verifies and re-packages");
    let second_repackage = reverify_and_repackage(&plan, &steam_manifest, &steam_depot)
        .expect("patch repackage repeats");
    assert_eq!(first_repackage, second_repackage);
    assert_eq!(first_repackage.status, manifest.expected_repackage_status);
    assert_eq!(first_repackage.release_authority, "human-ring3-required");

    let old_save = read(&manifest.legacy_save_ref);
    let (first_store, first_migration) =
        migrate_save_forward("post-launch-patch-demo-save", &old_save)
            .expect("legacy save migrates forward");
    let (second_store, second_migration) =
        migrate_save_forward("post-launch-patch-demo-save", &old_save)
            .expect("legacy save migration repeats");
    assert_eq!(first_store, second_store);
    assert_eq!(first_migration, second_migration);
    assert_eq!(first_migration.status, SaveMigrationStatus::Migrated);
    assert_eq!(manifest.expected_migration_status, "migrated");
    assert!(first_migration.replay_digests_preserved);
    first_store
        .verify_integrity()
        .expect("migrated demo save remains sealed");
}

#[test]
fn demo_doc_records_generated_state_wording_compatibility_and_governance() {
    let doc = read("docs/post-launch-patch-v1-demo.md");
    for required in [
        "no network, no live browser",
        "Rust/local owns trusted validation",
        "Browser/Studio/Electron/Steamworks surfaces remain read-only",
        "no generated run/build artifact is tracked outside the fixture tree",
        "does not automate a Release button",
        "#1 remains open",
        "#23 remains open",
        "cargo test -p ouroforge-core --test post_launch_patch_demo_contract --jobs 2",
    ] {
        assert!(doc.contains(required), "missing demo doc text {required}");
    }
    for forbidden in [
        "production-ready",
        "Godot replacement",
        "automated fun score",
        "quality score",
        "auto-merge is authorized",
        "trusted writes are authorized",
        "Layer-3 cloud/mobile is go",
    ] {
        assert!(
            !doc.contains(forbidden),
            "forbidden demo wording {forbidden}"
        );
    }
}

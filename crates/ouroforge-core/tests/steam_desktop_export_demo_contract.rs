//! Smoke test for Steam Desktop Export Demo v1 (#1841).

use ouroforge_core::steam_export_build::{
    SteamDepotConfig, SteamExportBuildManifest, SteamPackageDescriptor,
};
use ouroforge_core::steam_store_assets::{generate_steam_store_asset_plan, SteamStoreAssetPlan};
use ouroforge_core::steamworks_integration::{
    DailySeedLeaderboardPayload, NoSteamFallback, SteamworksIntegrationWiring,
};
use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

fn demo_fixture(name: &str) -> String {
    fixture(&format!("examples/steam-desktop-export-v1/demo/{name}"))
}

#[test]
fn demo_deterministically_shows_descriptor_wiring_and_store_assets() {
    let index: Value =
        serde_json::from_str(&demo_fixture("demo-index.fixture.json")).expect("demo index parses");
    assert_eq!(index["schemaVersion"], "steam-desktop-export-demo-v1");
    assert_eq!(index["issue"], 1841);
    assert_eq!(index["requiresNetwork"], false);
    assert_eq!(index["requiresLiveBrowser"], false);
    assert_eq!(index["requiresRealSteamConnection"], false);

    let manifest =
        SteamExportBuildManifest::from_json_str(&demo_fixture("build-manifest.fixture.json"))
            .expect("demo build manifest validates");
    let depot = SteamDepotConfig::from_json_str(&demo_fixture("depot-config.fixture.json"))
        .expect("demo depot config validates");
    let first = SteamPackageDescriptor::from_manifest_and_depot(&manifest, &depot)
        .expect("package descriptor builds");
    let second = SteamPackageDescriptor::from_manifest_and_depot(&manifest, &depot)
        .expect("package descriptor rebuilds");
    assert_eq!(first, second);
    assert_eq!(first.wrapper, "electron");
    assert_eq!(first.steam_bridge, "steamworks.js");
    assert_eq!(first.release_authority, "human-ring3-required");
    assert!(first.artifact_hash.starts_with("sha256:"));

    let wiring =
        SteamworksIntegrationWiring::from_json_str(&demo_fixture("steamworks-wiring.fixture.json"))
            .expect("demo Steamworks wiring validates");
    assert_eq!(wiring.runtime, "existing-web-runtime");
    assert_eq!(wiring.bridge, "steamworks.js");
    let feature_names: Vec<_> = wiring.features.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(
        feature_names,
        [
            "overlay",
            "achievements",
            "cloud-saves",
            "daily-seed-leaderboard"
        ]
    );

    let fallback = NoSteamFallback::from_json_str(&demo_fixture("no-steam-fallback.fixture.json"))
        .expect("demo no-Steam fallback validates");
    assert!(!fallback.steam_available);
    assert!(fallback
        .local_only_features
        .iter()
        .any(|f| f == "local-save"));

    let leaderboard = DailySeedLeaderboardPayload::from_json_str(&demo_fixture(
        "daily-seed-leaderboard.fixture.json",
    ))
    .expect("demo leaderboard payload validates");
    assert_eq!(leaderboard.score_source, "trusted-local-run-evidence");
    assert!(leaderboard.replay_digest.starts_with("sha256:"));

    let store_plan = SteamStoreAssetPlan::from_json_str(&demo_fixture("store-assets.fixture.json"))
        .expect("demo store asset plan validates");
    let generated_store_assets = generate_steam_store_asset_plan(&store_plan, 1_841_000)
        .expect("demo store asset proposals generate");
    let slots: Vec<_> = generated_store_assets
        .assets
        .iter()
        .map(|asset| (asset.slot.as_str(), asset.width, asset.height))
        .collect();
    assert_eq!(
        slots,
        [
            ("capsule_main", 616, 353),
            ("screenshot_1080p", 1920, 1080),
            ("trailer_frame_1080p", 1920, 1080)
        ]
    );
    assert!(generated_store_assets.proposal_only);
    assert!(generated_store_assets.human_submission_required);
}

#[test]
fn demo_doc_and_fixtures_preserve_boundaries_and_governance_state() {
    let combined = format!(
        "{}\n{}\n{}",
        fixture("docs/steam-desktop-export-v1-demo.md"),
        demo_fixture("demo-index.fixture.json"),
        demo_fixture("store-assets.fixture.json")
    );
    for required in [
        "deterministic",
        "fixture-scoped",
        "no network",
        "no live browser",
        "no real Steam connection",
        "browser/Studio/Electron/Steamworks surfaces are read-only",
        "human submits to Steam",
        "Milestone 36 asset generation",
    ] {
        assert!(
            combined.contains(required),
            "missing boundary text: {required}"
        );
    }
    for forbidden in [
        "production-ready",
        "Godot replacement",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "automated fun score",
        "Release button is automated",
        "Layer-3 cloud/mobile is GO",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording leaked: {forbidden}"
        );
    }

    let root = repo_root();
    let issue_1 = std::process::Command::new("gh")
        .args([
            "issue",
            "view",
            "1",
            "--repo",
            "shaun0927/Ouroforge",
            "--json",
            "state",
            "--jq",
            ".state",
        ])
        .current_dir(root.clone())
        .output()
        .expect("gh issue 1 runs");
    let issue_23 = std::process::Command::new("gh")
        .args([
            "issue",
            "view",
            "23",
            "--repo",
            "shaun0927/Ouroforge",
            "--json",
            "state",
            "--jq",
            ".state",
        ])
        .current_dir(root)
        .output()
        .expect("gh issue 23 runs");
    assert_eq!(String::from_utf8_lossy(&issue_1.stdout).trim(), "OPEN");
    assert_eq!(String::from_utf8_lossy(&issue_23.stdout).trim(), "OPEN");
}

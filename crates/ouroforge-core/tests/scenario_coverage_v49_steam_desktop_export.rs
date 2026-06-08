//! Scenario Coverage v49: Steam Desktop Export Regression Suite (#1842).

use ouroforge_core::steam_export_build::{
    SteamDepotConfig, SteamExportBuildManifest, SteamPackageDescriptor,
};
use ouroforge_core::steam_store_assets::{generate_steam_store_asset_plan, SteamStoreAssetPlan};
use ouroforge_core::steamworks_integration::{
    DailySeedLeaderboardPayload, NoSteamFallback, SteamworksIntegrationWiring,
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
fn v49_matrix_enumerates_required_regression_cases() {
    let matrix: Value = serde_json::from_str(&read(
        "examples/steam-desktop-export-v1/scenario-coverage-v49/matrix.fixture.json",
    ))
    .expect("matrix parses");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v49-steam-desktop-export-v1"
    );
    assert_eq!(matrix["issue"], 1842);
    assert_eq!(matrix["assertionMode"], "state-shape-only");
    assert_eq!(matrix["requiresNetwork"], false);
    assert_eq!(matrix["requiresLiveBrowser"], false);
    assert_eq!(matrix["requiresRealSteamConnection"], false);
    let cases = matrix["cases"].as_array().expect("cases array");
    let ids: Vec<_> = cases
        .iter()
        .map(|case| case["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        [
            "v49.build-depot.valid",
            "v49.steamworks.valid",
            "v49.store-assets.valid",
            "v49.demo.composed",
            "v49.web-build-standalone-backcompat"
        ]
    );
}

#[test]
fn v49_build_depot_steamworks_and_store_assets_are_locked() {
    let manifest = SteamExportBuildManifest::from_json_str(&read(
        "examples/steam-export-build-v1/build-manifest.valid.fixture.json",
    ))
    .expect("build manifest validates");
    let depot = SteamDepotConfig::from_json_str(&read(
        "examples/steam-export-build-v1/depot-config.valid.fixture.json",
    ))
    .expect("depot validates");
    let descriptor = SteamPackageDescriptor::from_manifest_and_depot(&manifest, &depot)
        .expect("descriptor builds");
    assert_eq!(descriptor.release_authority, "human-ring3-required");
    assert_eq!(descriptor.wrapper, "electron");
    assert_eq!(descriptor.steam_bridge, "steamworks.js");

    let wiring = SteamworksIntegrationWiring::from_json_str(&read(
        "examples/steamworks-integration-v1/wiring.valid.fixture.json",
    ))
    .expect("Steamworks wiring validates");
    let fallback = NoSteamFallback::from_json_str(&read(
        "examples/steamworks-integration-v1/no-steam-fallback.valid.fixture.json",
    ))
    .expect("fallback validates");
    let leaderboard = DailySeedLeaderboardPayload::from_json_str(&read(
        "examples/steamworks-integration-v1/daily-seed-leaderboard.valid.fixture.json",
    ))
    .expect("leaderboard validates");
    assert_eq!(wiring.features.len(), 4);
    assert!(!fallback.steam_available);
    assert_eq!(leaderboard.score_source, "trusted-local-run-evidence");

    let store_plan = SteamStoreAssetPlan::from_json_str(&read(
        "examples/steam-store-assets-v1/store-assets.valid.fixture.json",
    ))
    .expect("store assets validate");
    let generated = generate_steam_store_asset_plan(&store_plan, 1_842_000)
        .expect("store assets generate deterministically");
    assert!(generated.proposal_only);
    assert!(generated.human_submission_required);
    assert_eq!(generated.assets.len(), 3);
}

#[test]
fn v49_demo_composes_without_network_live_browser_or_real_steam() {
    let demo: Value = serde_json::from_str(&read(
        "examples/steam-desktop-export-v1/demo/demo-index.fixture.json",
    ))
    .expect("demo index parses");
    assert_eq!(demo["requiresNetwork"], false);
    assert_eq!(demo["requiresLiveBrowser"], false);
    assert_eq!(demo["requiresRealSteamConnection"], false);
    for expected in [
        "packaged artifact descriptor is deterministic and human-ring3-required",
        "Steamworks features are wired through a mockable steamworks.js bridge over the existing runtime",
        "store assets are proposal-only and generated through the Milestone 36 asset pipeline",
    ] {
        assert!(
            demo["assertions"].as_array().unwrap().iter().any(|value| value.as_str() == Some(expected)),
            "missing demo assertion: {expected}"
        );
    }
}

#[test]
fn v49_web_build_standalone_backcompat_golden_remains_valid() {
    let golden: Value = serde_json::from_str(&read(
        "examples/steam-desktop-export-v1/scenario-coverage-v49/web-build-standalone.golden.json",
    ))
    .expect("golden parses");
    assert_eq!(golden["runtime"], "existing-web-runtime");
    assert_eq!(golden["expectedState"], "valid-standalone-web-build");
    let entry_html = golden["entryHtml"].as_str().expect("entryHtml");
    let asset_manifest = golden["assetManifest"].as_str().expect("assetManifest");
    let html = read(entry_html);
    assert!(html.contains("__OUROFORGE__") || html.contains("runtime.js"));
    let manifest = ProjectAssetManifest::from_json_str(&read(asset_manifest))
        .expect("standalone asset manifest remains valid");
    assert!(!manifest.assets.is_empty());
}

#[test]
fn v49_docs_preserve_generated_state_wording_and_governance() {
    let combined = format!(
        "{}\n{}\n{}",
        read("docs/scenario-coverage-v49.md"),
        read("examples/steam-desktop-export-v1/scenario-coverage-v49/matrix.fixture.json"),
        read("examples/steam-desktop-export-v1/scenario-coverage-v49/web-build-standalone.golden.json")
    );
    for required in [
        "state/shape-only",
        "read-only",
        "no trusted write",
        "no upload",
        "no release authority",
        "fixture-scoped",
        "#1 and #23 remain open",
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

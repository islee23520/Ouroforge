//! Contract tests for Steamworks Integration v1 (#1839).

use ouroforge_core::steamworks_integration::{
    DailySeedLeaderboardPayload, NoSteamFallback, SteamworksIntegrationWiring,
    STEAMWORKS_FALLBACK_SCHEMA_VERSION, STEAMWORKS_LEADERBOARD_PAYLOAD_SCHEMA_VERSION,
    STEAMWORKS_WIRING_SCHEMA_VERSION,
};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn fixture(rel: &str) -> String {
    std::fs::read_to_string(
        repo_root()
            .join("examples/steamworks-integration-v1")
            .join(rel),
    )
    .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

#[test]
fn valid_wiring_covers_steamworks_features_over_existing_runtime() {
    let wiring = SteamworksIntegrationWiring::from_json_str(&fixture("wiring.valid.fixture.json"))
        .expect("valid wiring parses");
    assert_eq!(wiring.schema_version, STEAMWORKS_WIRING_SCHEMA_VERSION);
    assert_eq!(wiring.issue, 1839);
    assert_eq!(wiring.runtime, "existing-web-runtime");
    assert_eq!(wiring.bridge, "steamworks.js");
    let names: Vec<_> = wiring
        .features
        .iter()
        .map(|feature| feature.name.as_str())
        .collect();
    assert_eq!(
        names,
        [
            "overlay",
            "achievements",
            "cloud-saves",
            "daily-seed-leaderboard"
        ]
    );
}

#[test]
fn no_steam_fallback_disables_steam_features_without_blocking_local_play() {
    let fallback = NoSteamFallback::from_json_str(&fixture("no-steam-fallback.valid.fixture.json"))
        .expect("valid fallback parses");
    assert_eq!(fallback.schema_version, STEAMWORKS_FALLBACK_SCHEMA_VERSION);
    assert_eq!(fallback.mode, "no-steam");
    assert!(!fallback.steam_available);
    assert!(fallback
        .local_only_features
        .iter()
        .any(|f| f == "local-save"));
    assert!(fallback
        .disabled_features
        .iter()
        .any(|f| f == "daily-seed-leaderboard"));
}

#[test]
fn daily_seed_leaderboard_payload_uses_trusted_local_evidence() {
    let payload = DailySeedLeaderboardPayload::from_json_str(&fixture(
        "daily-seed-leaderboard.valid.fixture.json",
    ))
    .expect("valid payload parses");
    assert_eq!(
        payload.schema_version,
        STEAMWORKS_LEADERBOARD_PAYLOAD_SCHEMA_VERSION
    );
    assert_eq!(payload.score_source, "trusted-local-run-evidence");
    assert_eq!(payload.submitted_by, "steam-user-id-pseudonymous");
    assert!(payload.replay_digest.starts_with("sha256:"));
}

#[test]
fn invalid_fixtures_fail_closed_for_missing_feature_no_fallback_and_untrusted_score() {
    let missing =
        SteamworksIntegrationWiring::from_json_str(&fixture("invalid/missing-feature.json"))
            .expect_err("missing required features rejected");
    assert!(missing.to_string().contains("missing feature"));

    let no_fallback = NoSteamFallback::from_json_str(&fixture("invalid/no-fallback.json"))
        .expect_err("Steam-required mode rejected");
    assert!(no_fallback.to_string().contains("no-steam"));

    let untrusted =
        DailySeedLeaderboardPayload::from_json_str(&fixture("invalid/untrusted-leaderboard.json"))
            .expect_err("browser direct score rejected");
    assert!(untrusted.to_string().contains("trusted-local-run-evidence"));
}

#[test]
fn fixtures_preserve_generated_state_wording_and_governance_boundaries() {
    let combined = format!(
        "{}\n{}\n{}",
        fixture("wiring.valid.fixture.json"),
        fixture("no-steam-fallback.valid.fixture.json"),
        fixture("daily-seed-leaderboard.valid.fixture.json")
    );
    for required in [
        "existing runtime",
        "Rust/local owns trusted validation",
        "read-only with no direct trusted writes",
        "graceful no-Steam fallback is mandatory",
        "Steam desktop is not Layer-3 cloud/mobile",
        "Steam account/signing/Release button remain human/Ring-3",
    ] {
        assert!(
            combined.contains(required),
            "missing boundary text: {required}"
        );
    }
    for forbidden in [
        "production-ready",
        "Godot replacement",
        "automated fun score",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "trusted writes are authorized",
        "Release button is automated",
        "Layer-3 cloud/mobile is GO",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording leaked: {forbidden}"
        );
    }
}

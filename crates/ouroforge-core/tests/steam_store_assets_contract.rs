use std::path::PathBuf;

use ouroforge_core::steam_store_assets::{
    generate_steam_store_asset_plan, steam_store_asset_spec, SteamStoreAssetPlan,
    STEAM_STORE_ASSETS_BOUNDARY, STEAM_STORE_ASSETS_GENERATOR,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_fixture(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect("fixture exists")
}

#[test]
fn valid_store_assets_generate_steam_specs_through_milestone_36_pipeline() {
    let plan = SteamStoreAssetPlan::from_json_str(&read_fixture(
        "examples/steam-store-assets-v1/store-assets.valid.fixture.json",
    ))
    .expect("valid Steam store asset plan");

    let generated = generate_steam_store_asset_plan(&plan, 1_777_777).expect("generated proposals");

    assert!(generated.proposal_only);
    assert!(generated.human_submission_required);
    assert_eq!(generated.assets.len(), 3);
    for asset in &generated.assets {
        let spec = steam_store_asset_spec(&asset.slot).expect("known Steam slot");
        assert_eq!(asset.kind, spec.kind);
        assert_eq!((asset.width, asset.height), (spec.width, spec.height));
        assert_eq!(asset.proposal.proposal.status, "proposed");
        assert_eq!(asset.proposal.proposal.verdict_status, "pending");
        assert_eq!(asset.proposal.proposal.confidence, "unverified");
        assert_eq!(asset.proposal.provenance.asset_kind, "ui-art");
        assert!(asset.proposal.provenance.proposal_only);
        assert!(asset
            .proposal
            .proposal
            .reason
            .contains("Generated ui-art asset proposal"));
    }
}

#[test]
fn license_and_provenance_are_complete_for_every_store_asset() {
    let plan = SteamStoreAssetPlan::from_json_str(&read_fixture(
        "examples/steam-store-assets-v1/store-assets.valid.fixture.json",
    ))
    .expect("valid plan");
    let generated = generate_steam_store_asset_plan(&plan, 1_777_777).expect("generated proposals");

    let licenses: Vec<_> = generated
        .assets
        .iter()
        .map(|asset| &asset.proposal.provenance.license)
        .collect();
    assert!(licenses
        .iter()
        .any(|license| license.license == "project-owned"));
    assert!(licenses.iter().any(|license| license.license == "CC0-1.0"));
    assert!(licenses.iter().any(|license| {
        license.license == "CC-BY-4.0"
            && license.attribution.as_deref() == Some("Ouroforge fixture artists")
    }));
    for license in licenses {
        assert!(license.allowed_source);
        assert!(!license.source.trim().is_empty());
    }
}

#[test]
fn malformed_specs_and_missing_provenance_fail_closed() {
    for fixture in [
        "examples/steam-store-assets-v1/invalid/malformed-spec.fixture.json",
        "examples/steam-store-assets-v1/invalid/missing-provenance.fixture.json",
        "examples/steam-store-assets-v1/invalid/not-proposal-only.fixture.json",
    ] {
        let err = SteamStoreAssetPlan::from_json_str(&read_fixture(fixture))
            .expect_err("invalid Steam store asset fixture fails closed");
        let message = format!("{err:#}");
        assert!(
            message.contains("steam store asset")
                || message.contains("asset license")
                || message.contains("proposal-only"),
            "unexpected error for {fixture}: {message}"
        );
    }
}

#[test]
fn contracts_preserve_generated_state_wording_and_governance_boundaries() {
    let issue = read_fixture("examples/steam-store-assets-v1/store-assets.valid.fixture.json");
    assert!(issue.contains("proposalOnly"));
    assert!(issue.contains("humanSubmissionRequired"));
    assert!(issue.contains("Milestone 36"));
    assert!(issue.contains("no trusted write"));
    assert!(issue.contains("browser/Studio read-only"));
    assert!(!issue.contains("production-ready"));
    assert!(!issue.contains("Godot replacement"));
    assert!(!issue.contains("auto-merge"));
    assert_eq!(STEAM_STORE_ASSETS_GENERATOR, "steam-store-assets-v1");
    assert!(STEAM_STORE_ASSETS_BOUNDARY.contains("human submits"));

    // Hermetic by default: the live GitHub anchor check is opt-in so `cargo test`
    // does not depend on network / gh-auth / external issue state. The doc/governance
    // assertions above already run unconditionally. Set
    // OUROFORGE_LIVE_GOVERNANCE_ANCHOR_CHECK=1 to verify issues #1/#23 are OPEN.
    if std::env::var("OUROFORGE_LIVE_GOVERNANCE_ANCHOR_CHECK").is_err() {
        return;
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

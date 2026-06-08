//! Contract tests for Update Re-Verify and Re-Package Loop v1 (#1845).

use std::path::{Path, PathBuf};

use ouroforge_core::patch_reverify::{reverify_and_repackage, PatchReverifyPlan};
use ouroforge_core::steam_export_build::{SteamDepotConfig, SteamExportBuildManifest};

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

fn pass_plan() -> PatchReverifyPlan {
    PatchReverifyPlan::from_json_str(&read(
        "examples/post-launch-patch-v1/patch-reverify.pass.fixture.json",
    ))
    .expect("passing patch reverify fixture validates")
}

fn steam_manifest() -> SteamExportBuildManifest {
    SteamExportBuildManifest::from_json_str(&read(
        "examples/steam-export-build-v1/build-manifest.valid.fixture.json",
    ))
    .expect("steam build manifest validates")
}

fn steam_depot() -> SteamDepotConfig {
    SteamDepotConfig::from_json_str(&read(
        "examples/steam-export-build-v1/depot-config.valid.fixture.json",
    ))
    .expect("steam depot config validates")
}

#[test]
fn passing_patch_reverifies_and_repackages_through_existing_steam_export() {
    let evidence = reverify_and_repackage(&pass_plan(), &steam_manifest(), &steam_depot())
        .expect("passing patch re-packages after re-verify");

    assert_eq!(evidence.status, "repackaged-after-reverify");
    assert_eq!(evidence.release_authority, "human-ring3-required");
    assert_eq!(evidence.package_descriptor.wrapper, "electron");
    assert_eq!(evidence.package_descriptor.steam_bridge, "steamworks.js");
    assert!(evidence
        .boundary
        .contains("Rust/local owns trusted validation"));
    assert!(evidence
        .boundary
        .contains("browser/Studio/Electron/Steamworks surfaces are read-only"));
    assert!(evidence
        .provenance_refs
        .iter()
        .any(|reference| reference.contains("package-descriptor.valid.fixture.json")));
}

#[test]
fn failing_patch_is_blocked_from_repackaging() {
    let failing = PatchReverifyPlan::from_json_str(&read(
        "examples/post-launch-patch-v1/invalid/patch-reverify.fail.fixture.json",
    ))
    .expect("failing patch fixture is structurally valid");

    let error = reverify_and_repackage(&failing, &steam_manifest(), &steam_depot())
        .expect_err("failing gate blocks re-package");
    let message = error.to_string();
    assert!(message.contains("cannot re-package before full re-verify passes"));
    assert!(message.contains("scenario-coverage"));
}

#[test]
fn patch_repackage_evidence_is_deterministic() {
    let first = reverify_and_repackage(&pass_plan(), &steam_manifest(), &steam_depot())
        .expect("first repackage evidence");
    let second = reverify_and_repackage(&pass_plan(), &steam_manifest(), &steam_depot())
        .expect("second repackage evidence");

    assert_eq!(first, second);
    assert_eq!(
        first.to_json().expect("serialize first"),
        second.to_json().expect("serialize second")
    );
}

#[test]
fn full_gate_set_and_governance_are_enforced() {
    let plan = pass_plan();
    let kinds = plan
        .gate_set
        .iter()
        .map(|gate| gate.kind.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "rust-tests",
        "clippy",
        "scenario-coverage",
        "evaluator-gates",
        "compare-provenance",
        "save-compatibility",
        "steam-export-package",
    ] {
        assert!(kinds.contains(required), "missing gate {required}");
    }

    let doc = read("docs/post-launch-patch-v1.md");
    for required in [
        "A patch **must re-verify before re-packaging**",
        "#1 remains open",
        "#23 remains open",
        "no new pipeline",
    ] {
        assert!(doc.contains(required), "missing doc boundary {required}");
    }
}

//! Design-gate regression for Steam Desktop Export and Steamworks v1 (#1837).
//!
//! This issue defines contracts only: Steam is a local desktop export path over
//! the existing web runtime, while human/Ring-3 release steps and Layer-3
//! cloud/mobile remain out of scope.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn gate_doc() -> String {
    std::fs::read_to_string(repo_root().join("docs/steam-desktop-export-v1.md"))
        .expect("steam desktop export design gate exists")
}

#[test]
fn design_gate_records_bounded_go_and_defer_decisions() {
    let doc = gate_doc();
    for required in [
        "Issue: **#1837** (#1 Era I Milestone 54)",
        "Status: **Design gate complete — bounded GO for local Steam desktop export preparation; DEFER human/Ring-3 release authority, market demand, and Layer-3 cloud/mobile.**",
        "Web to desktop wrapper | **GO, bounded** | Wrap the existing web release candidate with **Electron + steamworks.js**.",
        "SteamPipe build/depot pipeline | **GO, bounded** | Define local build/depot packaging, checksums, provenance, and dry-run validation.",
        "Steamworks integration | **GO, bounded** | Define proposal/contract surfaces for overlay, achievements, cloud saves, and daily-seed leaderboard.",
        "Store-asset generation | **GO, bounded** | Reuse the Milestone 36 asset pipeline and asset-QA/provenance gates for capsule/header/library assets.",
        "Steam account, code signing, content survey, Release button, market demand | **DEFER / Ring-3**",
        "Layer-3 cloud/mobile | **DEFER** | Steam is local desktop export only",
    ] {
        assert!(doc.contains(required), "missing decision text: {required}");
    }
}

#[test]
fn design_gate_defines_export_steampipe_steamworks_and_asset_contracts() {
    let doc = gate_doc();
    for required in [
        "existing web release candidate → Electron shell → `steamworks.js` bridge → Steam desktop build",
        "The wrapper must reuse the existing web runtime and asset surfaces.",
        "Generate depot layout metadata, checksums, package manifests, and provenance evidence.",
        "Stop before credentialed upload, partner-site mutation, publishing, or the Release button.",
        "Overlay | Enable the Steam overlay from the desktop wrapper when available.",
        "Achievements | Map approved local achievements from Rust/local validated state to Steam achievement IDs.",
        "Cloud saves | Package validated local save/project state for Steam Cloud sync.",
        "Daily-seed leaderboard | Publish or read leaderboard entries for deterministic daily-seed results.",
        "Store assets reuse the Milestone 36 asset pipeline rather than creating a parallel store-art system.",
        "Milestone 36 asset pipeline reuse includes asset manifest/loader/atlas validation, provenance bundle, compare/visual evidence, asset-QA, and the existing review/apply/trust-gradient path.",
    ] {
        assert!(doc.contains(required), "missing contract text: {required}");
    }
}

#[test]
fn design_gate_preserves_human_ring3_layer3_and_governance_boundaries() {
    let doc = gate_doc();
    for required in [
        "Steam account creation, partner onboarding, app IDs, credentials, and account administration.",
        "Code signing certificates, signing identity, notarization decisions, and credential storage.",
        "Steam content survey, store questionnaire, legal/platform attestations, ratings, and policy acceptance.",
        "The Steam **Release** button, upload/publish approval, store launch timing, and release go/no-go.",
        "Market demand, wishlists, user acquisition, discoverability, pricing, discounts, community management, and support obligations.",
        "Steam desktop export is a **local desktop export**, not Layer-3 cloud/mobile.",
        "Browser, Studio, dashboard, cockpit, Electron, and Steamworks surfaces remain read-only for trusted state.",
        "#1837 scope -> #1838 -> #1839 -> #1840 -> #1841 -> #1842 -> #1843",
        "#1842 must continue Scenario Coverage numbering at **v49**",
        "#1 remains open.",
        "#23 remains open.",
    ] {
        assert!(doc.contains(required), "missing boundary text: {required}");
    }

    for forbidden_positive_claim in [
        "autonomous shipping is authorized",
        "Release button is automated",
        "market demand is automated",
        "Layer-3 cloud/mobile is GO",
        "browser/Studio trusted writes are authorized",
        "Steamworks trusted writes are authorized",
        "auto-merge is authorized",
        "production-ready claim is authorized",
        "Godot replacement claim is authorized",
    ] {
        assert!(
            !doc.contains(forbidden_positive_claim),
            "forbidden positive claim leaked: {forbidden_positive_claim}"
        );
    }
}

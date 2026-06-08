//! Governance refresh for Steam Desktop Export and Steamworks v1 (#1843).

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
fn roadmap_records_milestone_54_completion_on_merged_evidence() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "Era I Milestone 54 — Steam Desktop Export and Steamworks v1",
        "complete for Era I Milestone 54",
        "#1837, #1838, #1839, #1840, #1841, #1842, and #1843",
        "docs/steam-desktop-export-v1.md",
        "docs/steam-desktop-export-v1-demo.md",
        "docs/scenario-coverage-v49.md",
        "PRs #2000/#2001/#2007/#2012/#2016/#2051",
    ] {
        assert!(
            roadmap.contains(required),
            "missing roadmap evidence: {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_steam_desktop_boundaries() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "local desktop export only, not Layer-3 cloud/mobile",
        "Steam account creation, partner portal work, code signing, content survey, store submission, release-button action, and market demand remain human/Ring-3",
        "Browser, Studio, dashboard, cockpit, Electron, JavaScript, and Steamworks surfaces remain read-only for trusted state",
        "Rust/local owns validation, provenance, descriptor derivation, persistence, and trusted-write gates",
        "Generated runs/artifacts/builds remain untracked unless fixture-scoped",
        "The human Era J fun/feel/release judgment remains outside this milestone",
    ] {
        assert!(roadmap.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_claims_out_of_milestone_54() {
    let roadmap = read("docs/roadmap.md");
    let section = roadmap
        .split("**Era I Milestone 54 — Steam Desktop Export and Steamworks v1")
        .nth(1)
        .expect("m54 section exists");
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
            !section.contains(forbidden),
            "forbidden wording leaked: {forbidden}"
        );
    }
}

#[test]
fn governance_refresh_keeps_anchor_issues_open_in_wording() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "#1 and #23 remain open governance anchors",
        "are not closed, narrowed, or modified by this milestone",
    ] {
        assert!(
            roadmap.contains(required),
            "missing anchor wording: {required}"
        );
    }
}

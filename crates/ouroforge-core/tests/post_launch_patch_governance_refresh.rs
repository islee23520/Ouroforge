//! Governance refresh tests for Post-Launch Patch v1 / Era I Milestone 55 (#1849).

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
fn roadmap_records_milestone_55_completion_on_merged_evidence() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "Era I Milestone 55 — Post-Launch Patch, Re-Verify, and Save-Migration Loop v1",
        "#1844, #1845, #1846, #1847, and #1848",
        "PRs #2105/#2107/#2109/#2111/#2112",
        "docs/post-launch-patch-v1.md",
        "docs/post-launch-patch-v1-demo.md",
        "docs/scenario-coverage-v50.md",
        "non-patched build/save backward-compatibility golden",
        "complete for Era I Milestone 55",
    ] {
        assert!(
            roadmap.contains(required),
            "missing roadmap evidence {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_post_launch_patch_boundaries() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "a patch must pass the full declared gate set before re-package evidence is derived",
        "old saves migrate forward through existing save/restore and replay-digest validation",
        "incompatible saves fail closed with explicit evidence",
        "not a new persistence system",
        "browser, dashboard, cockpit, Studio, Electron, JavaScript, and Steamworks surfaces remain read-only",
        "Generated patch runs, package descriptors, migrated saves, builds, and evidence remain untracked unless explicitly fixture-scoped",
        "The human Era J fun/feel/release judgment remains outside this milestone",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(roadmap.contains(required), "missing boundary {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_claims_out_of_milestone_55() {
    let roadmap = read("docs/roadmap.md");
    let start = roadmap
        .find("Era I Milestone 55 — Post-Launch Patch")
        .expect("m55 starts");
    let end = roadmap[start..]
        .find("**Era J Milestone 57")
        .map(|offset| start + offset)
        .unwrap_or(roadmap.len());
    let section = &roadmap[start..end];
    for forbidden in [
        "production-ready engine",
        "Godot replacement is authorized",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "trusted writes are authorized",
        "Release button is automated",
        "Layer-3 cloud/mobile is GO",
        "automated fun score is authorized",
        "quality score is authorized",
        "new persistence mechanism is authorized",
    ] {
        assert!(
            !section.contains(forbidden),
            "forbidden wording leaked {forbidden}"
        );
    }
}

#[test]
fn governance_anchors_remain_open() {
    // Hermetic by default: the live GitHub anchor check is opt-in so `cargo test`
    // does not depend on network / gh-auth / external issue state. Set
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

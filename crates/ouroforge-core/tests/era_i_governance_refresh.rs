//! Era I closing governance refresh tests (#1850).

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
fn roadmap_records_era_i_completion_with_milestone_evidence() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "Era I closing governance refresh (Milestones 47-55 / Milestone 56)",
        "complete for Milestones 47-55",
        "M47 Card-Roguelite Substrate v1",
        "M48 Scoring Engine v1",
        "M49 Run/Shop v1",
        "M50 Balance Verification v1",
        "M51 Game-Feel/Juice Toolkit v1",
        "M52 Deckbuilder UI Kit v1",
        "M53 Localization Pipeline v1",
        "M54 Steam Desktop Export and Steamworks v1",
        "M55 Post-Launch Patch, Re-Verify, and Save-Migration Loop v1",
        "Scenario Coverage v50",
    ] {
        assert!(
            roadmap.contains(required),
            "missing Era I evidence {required}"
        );
    }
}

#[test]
fn shippability_assessment_is_descriptive_and_bounded() {
    let combined = format!("{}\n{}", read("README.md"), read("docs/roadmap.md"));
    for required in [
        "verified Steam-shippable desktop artifact shape",
        "loop-produced deckbuilder",
        "not a public release",
        "not release authority",
        "Human/Ring-3 Steam steps",
        "human Era J fun/feel/release verdict remain outside Era I",
        "configuration over the card-roguelite substrate",
        "local desktop package descriptor",
    ] {
        assert!(
            combined.contains(required),
            "missing bounded assessment {required}"
        );
    }
}

#[test]
fn era_i_boundaries_are_reaffirmed_without_new_authority() {
    let roadmap = read("docs/roadmap.md");
    for required in [
        "the deckbuilder remains substrate-as-config",
        "deterministic/seed-stable Rust/local validation owns trusted state",
        "JavaScript/browser/Studio/dashboard/cockpit/Electron/Steamworks surfaces remain deterministic read-only or proposal/draft-only surfaces",
        "Steam desktop export remains local and not Layer-3",
        "Generated runs/artifacts/builds/migrated saves remain untracked unless fixture-scoped",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(roadmap.contains(required), "missing boundary {required}");
    }
}

#[test]
fn era_i_closure_keeps_forbidden_claims_blocked() {
    let roadmap = read("docs/roadmap.md");
    let start = roadmap
        .find("Era I closing governance refresh")
        .expect("era i close starts");
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
        "new capability is authorized by this governance refresh",
    ] {
        assert!(
            !section.contains(forbidden),
            "forbidden wording leaked {forbidden}"
        );
    }
}

#[test]
fn governance_anchors_remain_open() {
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

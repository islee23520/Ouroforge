//! Governance regression for Game-Feel and Juice Toolkit v1 (#1824).
//!
//! This is a documentation-only guard: Milestone 51 is complete on merged
//! evidence, but the completion must preserve mechanical-only/read-only
//! boundaries and leave #1/#23 open.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_51_completion_on_merged_evidence() {
    let doc = roadmap();
    for required in [
        "Era I Milestone 51 — Game-Feel and Juice Toolkit v1",
        "complete for Era I Milestone 51",
        "#1818 (PR #1893)",
        "#1819 (PR #1920)",
        "#1820 (PR #1923)",
        "#1821 (PR #1926)",
        "#1822 (PR #1927)",
        "#1823 (PR #1930)",
        "Scenario Coverage v46",
        "existing runtime feedback backward-compatibility golden",
    ] {
        assert!(
            doc.contains(required),
            "missing roadmap evidence: {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_game_feel_boundaries() {
    let doc = roadmap();
    for required in [
        "mechanical feedback surfaces only",
        "not a new runtime",
        "not a new engine",
        "automated fun score",
        "feel/fun verdict remains human-owned",
        "proposal-only",
        "read-only",
        "Generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(doc.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_claims_out_of_milestone_51() {
    let doc = roadmap();
    let start = doc
        .find("Game-Feel and Juice Toolkit v1 governance refresh")
        .expect("governance refresh section exists");
    let end = doc[start..]
        .find("**Era J Milestone 57")
        .map(|offset| start + offset)
        .unwrap_or(doc.len());
    let section = &doc[start..end];
    for forbidden in [
        "production-ready claim is introduced",
        "Godot replacement claim is introduced",
        "automated fun score is authorized",
        "auto-merge is authorized",
        "browser trusted writes are authorized",
    ] {
        assert!(
            !section.contains(forbidden),
            "forbidden positive claim leaked: {forbidden}"
        );
    }
    assert!(section.contains("No production-ready, quality, fun, shippable"));
    assert!(section.contains("auto-apply, auto-merge, self-approval"));
}

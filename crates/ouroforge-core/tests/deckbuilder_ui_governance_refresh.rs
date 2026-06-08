//! Governance regression for Deckbuilder UI Kit v1 (#1831).
//!
//! This is a documentation-only guard: Milestone 52 is complete on merged
//! evidence, but the completion must preserve in-game JS UI/read-only Studio
//! boundaries and leave #1/#23 open.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_52_completion_on_merged_evidence() {
    let doc = roadmap();
    for required in [
        "Era I Milestone 52 — Deckbuilder UI Kit v1",
        "complete for Era I Milestone 52",
        "#1825 (PR #1895)",
        "#1826 (PR #1894)",
        "#1827 (PR #1961)",
        "#1828 (PR #1966)",
        "#1829 (PR #1969)",
        "#1830 (PR #1977)",
        "Scenario Coverage v47",
        "existing runtime UI/probe backward-compatibility golden",
    ] {
        assert!(
            doc.contains(required),
            "missing roadmap evidence: {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_deckbuilder_ui_boundaries() {
    let doc = roadmap();
    for required in [
        "in-game deterministic JavaScript presentation",
        "draft/proposal surface",
        "not a new UI framework",
        "automated fun score",
        "feel/fun verdict remains human-owned",
        "proposal-only",
        "read-only",
        "draft-only",
        "Generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(doc.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_claims_out_of_milestone_52() {
    let doc = roadmap();
    let start = doc
        .find("Deckbuilder UI Kit v1 governance refresh")
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
        "Studio trusted writes are authorized",
    ] {
        assert!(
            !section.contains(forbidden),
            "forbidden positive claim leaked: {forbidden}"
        );
    }
    assert!(section.contains("No production-ready, quality, fun, shippable"));
    assert!(section.contains("auto-apply, auto-merge, self-approval"));
    assert!(section.contains("in-game UI"));
    assert!(section.contains("Studio surfaces remain **read-only**"));
}

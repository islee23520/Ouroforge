//! Governance regression for Candidate Generation and Curation Cockpit v1 (#1856).
//!
//! This is a documentation-only guard: Milestone 57 is complete on merged
//! evidence, but the completion must preserve proposal-only/read-only boundaries
//! and leave #1/#23 open.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_57_completion_on_merged_evidence() {
    let doc = roadmap();
    for required in [
        "Era J Milestone 57 — Candidate Generation and Curation Cockpit v1",
        "complete for Era J Milestone 57",
        "#1851 (PR #1901)",
        "#1852 (PR #1914)",
        "#1853 (PR #1917)",
        "#1854 (PR #1918)",
        "#1855 (PR #1921)",
        "Scenario Coverage v51",
        "Milestone 30 single-proposal",
    ] {
        assert!(
            doc.contains(required),
            "missing roadmap evidence: {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_curation_cockpit_boundaries() {
    let doc = roadmap();
    for required in [
        "creation as human curation",
        "proposal-only",
        "read-only",
        "not a parallel engine",
        "human fun/feel verdict",
        "Generated runs/assets/builds/artifacts remain ignored unless explicitly fixture-scoped",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(doc.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_claims_out_of_milestone_57() {
    let doc = roadmap();
    let start = doc
        .find("Candidate Generation and Curation Cockpit v1 governance refresh")
        .expect("governance refresh section exists");
    let end = doc[start..]
        .find("## Product direction")
        .map(|offset| start + offset)
        .unwrap_or(doc.len());
    let section = &doc[start..end];
    for forbidden in [
        "production-ready claim is introduced",
        "Godot replacement claim is introduced",
        "automated fun score",
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

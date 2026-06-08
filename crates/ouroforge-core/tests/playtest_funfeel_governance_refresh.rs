//! Governance refresh contract for Human Playtest Harness and Fun-Feel Gate v1 (#1862).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_58_completion_on_merged_evidence() {
    let doc = roadmap();
    assert!(doc.contains("Era J Milestone 58 — Human Playtest Harness and Fun-Feel Gate v1"));
    for required in [
        "#1857 (PR #1905)",
        "#1858 (PR #1924)",
        "#1859 (PR #1928)",
        "#1860 (PR #1931)",
        "#1861 (PR #1952)",
        "complete for Era J Milestone 58",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(
            doc.contains(required),
            "missing milestone evidence {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_human_fun_verdict_boundary() {
    let doc = roadmap();
    for required in [
        "not automated fun",
        "fun verdict remains permanently human-in-the-loop",
        "cannot be release-ready without a human fun/feel verdict",
        "cannot compute, infer, or replace the human fun verdict",
        "no-auto-score drift protection",
    ] {
        assert!(doc.contains(required), "missing human boundary {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_maturity_and_authority_claims_out_of_milestone_58() {
    let doc = roadmap();
    let start = doc
        .find("**Era J Milestone 58 — Human Playtest Harness")
        .expect("milestone 58 section exists");
    let end = doc[start..]
        .find("### Candidate Generation and Curation Cockpit")
        .map(|offset| start + offset)
        .expect("next section exists");
    let section = &doc[start..end];

    for required in [
        "does not say the title is fun, good, shippable, production-ready",
        "does not add a release button",
        "browser/Studio trusted writes",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "reviewer bypass",
        "Godot replacement/parity",
    ] {
        assert!(
            section.contains(required),
            "missing forbidden boundary {required}"
        );
    }
}

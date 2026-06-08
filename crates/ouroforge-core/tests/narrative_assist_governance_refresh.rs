//! Governance refresh contract for Narrative and Theme-Arc Authoring Assist v1 (#1868).

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_59_completion_on_merged_evidence() {
    let doc = roadmap();
    assert!(doc.contains("Era J Milestone 59 — Narrative and Theme-Arc Authoring Assist v1"));
    for required in [
        "#1863 (PR #1908)",
        "#1864 (PR #1965)",
        "#1865 (PR #1975)",
        "#1866 (PR #1989)",
        "#1867 (PR #1990)",
        "complete for Era J Milestone 59",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(
            doc.contains(required),
            "missing milestone evidence {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_human_tone_soul_boundary() {
    let doc = roadmap();
    for required in [
        "tone/soul human",
        "Tone, soul, and fun verdicts remain permanently human-owned",
        "Tone/soul/fun/quality verdicts remain human-owned",
        "cannot decide tone/soul",
        "cannot claim that a candidate is funny",
        "review/apply/trust-gradient",
    ] {
        assert!(
            doc.contains(required),
            "missing human tone boundary {required}"
        );
    }
}

#[test]
fn roadmap_keeps_forbidden_maturity_and_authority_claims_out_of_milestone_59() {
    let doc = roadmap();
    let start = doc
        .find("**Era J Milestone 59 — Narrative and Theme-Arc")
        .expect("milestone 59 section exists");
    let end = doc[start..]
        .find("### Human Playtest Harness")
        .map(|offset| start + offset)
        .expect("next section exists");
    let section = &doc[start..end];

    for required in [
        "proposal-only",
        "not approval or apply authority",
        "browser/Studio trusted-write authority",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "reviewer bypass",
        "parallel narrative engine",
        "Godot replacement/parity",
        "production-ready",
        "shippable",
        "marketable",
    ] {
        assert!(
            section.contains(required),
            "missing forbidden boundary {required}"
        );
    }
}

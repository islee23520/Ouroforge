use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn roadmap() -> String {
    std::fs::read_to_string(repo_root().join("docs/roadmap.md")).expect("roadmap exists")
}

#[test]
fn roadmap_records_milestone_53_completion_on_merged_evidence() {
    let doc = roadmap();
    for required in [
        "Era I Milestone 53 — Localization Pipeline v1",
        "complete for Era I Milestone 53",
        "#1832 (PR #1896)",
        "#1833 (PR #1992)",
        "#1834 (PR #1993)",
        "#1835 (PR #1995)",
        "Scenario Coverage v48",
        "default-locale backward-compatibility golden",
    ] {
        assert!(
            doc.contains(required),
            "missing roadmap evidence: {required}"
        );
    }
}

#[test]
fn roadmap_reaffirms_localization_boundaries() {
    let doc = roadmap();
    for required in [
        "mechanical localization",
        "proposal-only locale generation",
        "not a new runtime",
        "remote translation service",
        "creative tone",
        "translation quality",
        "feel/fun verdict remains human-owned",
        "read-only",
        "draft-only",
        "Generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "#1 and #23 remain open governance anchors",
    ] {
        assert!(doc.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn roadmap_keeps_forbidden_claims_out_of_milestone_53() {
    let doc = roadmap();
    let start = doc
        .find("Localization Pipeline v1 governance refresh")
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
        "translation quality is guaranteed",
    ] {
        assert!(
            !section.contains(forbidden),
            "forbidden positive claim leaked: {forbidden}"
        );
    }
    assert!(section.contains("No production-ready, quality, fun, shippable"));
    assert!(section.contains("auto-apply, auto-merge, self-approval"));
    assert!(section.contains("proposal-only"));
}

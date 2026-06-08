//! Governance regression for Era H closing autonomy assessment (#1698).
//!
//! This is documentation-only: Era H can close on merged evidence while the
//! permanent human-judgment boundary and Layer-3 DEFER remain visible.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_doc(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path)).expect("doc exists")
}

#[test]
fn roadmap_and_readme_record_era_h_completion_on_merged_evidence() {
    let roadmap = read_doc("docs/roadmap.md");
    let readme = read_doc("README.md");
    let combined = format!("{roadmap}\n{readme}");
    for required in [
        "Era H closing governance and final autonomy assessment (Era H Milestone 46)",
        "complete for Era H",
        "PRs #1704/#1790/#1876/#1877/#1878/#1879/#1880",
        "PRs #1701/#1884/#1885/#1888/#1890/\n#1891/#1902",
        "PRs #1702/#1906/#1910/#1956/#1964/#1973/#1987",
        "Milestone 45 #1697 (PR #1997)",
        "docs/era-h-autonomy-assessment.md",
        "Era H (Milestones 42–46) is recorded complete on merged\nevidence",
    ] {
        assert!(
            combined.contains(required),
            "missing Era H evidence: {required}"
        );
    }
}

#[test]
fn autonomy_assessment_names_automated_review_gated_and_human_owned_stages() {
    let assessment = read_doc("docs/era-h-autonomy-assessment.md");
    for required in [
        "Autonomous or agent-coordinated with local verification",
        "Review-gated and never self-certifying",
        "Permanently human",
        "local web release candidate\nwith synthetic and fixture-scoped evidence",
        "The final release decision is **0%\nautomated** and remains human-owned.",
        "Collect-and-exit / grid-puzzle line",
        "Signal Gate / deck-roguelike-to-deckbuilder line",
        "vision, taste/fun/quality judgment, art/audio/UX/\n  narrative direction",
    ] {
        assert!(
            assessment.contains(required),
            "missing assessment text: {required}"
        );
    }
}

#[test]
fn era_h_closure_preserves_human_boundary_layer3_defer_and_open_anchors() {
    let combined = format!(
        "{}\n{}",
        read_doc("docs/roadmap.md"),
        read_doc("docs/era-h-autonomy-assessment.md")
    );
    for required in [
        "#1 and #23 remain open governance anchors",
        "#1 and #23 remain open governance anchors.",
        "High-risk and source-affecting changes never auto-apply.",
        "Browser, Studio, dashboard, and cockpit surfaces remain read-only for trusted\n  state.",
        "No\nauto-merge, self-approval, reviewer bypass, hidden trusted writes",
        "No automated quality/fun/taste verdict",
        "Godot replacement/parity claim",
        "DEFER absent a separate #1508 Layer-3 GO",
        "Distributed/Elixir remains NO-GO for Layer-3 under ADR #92",
        "Generated runs/assets/content/release artifacts remain untracked unless",
    ] {
        assert!(
            combined.contains(required),
            "missing boundary text: {required}"
        );
    }

    for forbidden_positive_claim in [
        "release decision is automated",
        "human go/no-go can be bypassed",
        "auto-merge is authorized",
        "browser/Studio trusted writes are authorized",
        "production-ready claim is authorized",
        "Godot replacement claim is authorized",
        "shipping/liveops implementation is added",
    ] {
        assert!(
            !combined.contains(forbidden_positive_claim),
            "forbidden positive claim leaked: {forbidden_positive_claim}"
        );
    }
}

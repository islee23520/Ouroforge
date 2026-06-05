const HANDOFF: &str = include_str!("../../../docs/gdd-to-prototype-governance-handoff.md");
const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const README: &str = include_str!("../../../README.md");
const GDD_SCOPE: &str = include_str!("../../../docs/gdd-to-playable-prototype-v1.md");

#[test]
fn gdd_to_prototype_governance_marks_milestone_complete_without_closing_anchors() {
    for text in [HANDOFF, ROADMAP, README, GDD_SCOPE] {
        assert!(
            text.contains("GDD-to-Playable Prototype v1")
                || text.contains("GDD to Playable Prototype v1"),
            "missing milestone name"
        );
        assert!(text.contains("complete") || text.contains("Complete"));
    }
    for text in [HANDOFF, GDD_SCOPE] {
        assert!(text.contains("#1 remains open") || text.contains("#1 remains"));
        assert!(text.contains("#23 remains open") || text.contains("#23 remains"));
    }
    for issue in 644..=660 {
        assert!(
            HANDOFF.contains(&format!("#{issue}")),
            "handoff missing issue #{issue}"
        );
    }
}

#[test]
fn gdd_to_prototype_governance_keeps_conservative_boundaries() {
    let combined = format!("{HANDOFF}\n{ROADMAP}\n{README}\n{GDD_SCOPE}").to_ascii_lowercase();
    for required in [
        "bounded",
        "evidence-gated prototype",
        "autonomous unrestricted game creation",
        "arbitrary source mutation",
        "browser trusted writes",
        "command bridges",
        "auto-apply",
        "auto-merge",
        "generated proprietary asset",
        "native export",
        "plugin runtime",
        "hosted/cloud",
        "production readiness",
        "godot replacement",
        "generated prototype drafts",
        "fixture-scoped",
    ] {
        assert!(combined.contains(required), "missing boundary {required}");
    }

    for forbidden in [
        "autonomous unrestricted game creation enabled",
        "browser trusted write enabled",
        "command bridge enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "generated proprietary assets are included",
        "production-ready claim enabled",
        "current godot replacement is implemented",
        "native export enabled",
        "plugin runtime enabled",
        "hosted/cloud service enabled",
        "public launch automation enabled",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}

#[test]
fn gdd_to_prototype_governance_recommends_next_bounded_milestone() {
    let handoff = HANDOFF.replace('\n', " ");
    let roadmap = ROADMAP.replace('\n', " ");
    let readme = README.replace('\n', " ");
    assert!(handoff.contains("Autonomous QA / Playtest Swarm v1"));
    assert!(roadmap.contains("Autonomous QA / Playtest Swarm v1"));
    assert!(readme.contains("Autonomous QA / Playtest Swarm v1"));
    assert!(handoff.contains("#697"));
    assert!(handoff.contains("#698"));
    assert!(handoff.contains("Safe Source Mutation Apply"));
    assert!(handoff.contains("Build/Export/Packaging"));
    assert!(handoff.contains("own scoped issue sequence"));
}

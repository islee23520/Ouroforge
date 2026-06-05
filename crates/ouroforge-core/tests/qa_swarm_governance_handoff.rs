const HANDOFF: &str =
    include_str!("../../../docs/autonomous-qa-playtest-swarm-governance-handoff.md");
const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const README: &str = include_str!("../../../README.md");
const SCOPE: &str = include_str!("../../../docs/autonomous-qa-playtest-swarm-v1.md");
const DOCS_INDEX: &str = include_str!("../../../docs/README.md");

#[test]
fn qa_swarm_governance_marks_milestone_complete_without_closing_anchors() {
    for text in [HANDOFF, ROADMAP, README, SCOPE] {
        assert!(text.contains("Autonomous QA / Playtest Swarm v1"));
        assert!(text.contains("complete") || text.contains("Complete"));
    }
    assert!(HANDOFF.contains("#1 remains open"));
    assert!(HANDOFF.contains("#23 remains open"));
    assert!(DOCS_INDEX.contains("autonomous-qa-playtest-swarm-governance-handoff.md"));
    for issue in 682..=697 {
        assert!(
            HANDOFF.contains(&format!("#{issue}")),
            "handoff missing issue #{issue}"
        );
    }
}

#[test]
fn qa_swarm_governance_keeps_conservative_boundaries() {
    let combined = format!("{HANDOFF}\n{ROADMAP}\n{README}\n{SCOPE}").to_ascii_lowercase();
    for required in [
        "bounded",
        "evidence/backlog",
        "hidden workers",
        "remote/cloud swarm",
        "unbounded spawning",
        "browser command bridges",
        "browser trusted writes",
        "auto-fix",
        "auto-apply",
        "auto-merge",
        "self-approval",
        "quality/fun/market/production safety guarantees",
        "current godot replacement",
        "production-ready",
        "generated runs",
        "fixture-scoped",
    ] {
        assert!(combined.contains(required), "missing boundary {required}");
    }

    for forbidden in [
        "hidden workers enabled",
        "remote/cloud swarm enabled",
        "unbounded spawning enabled",
        "browser command bridge enabled",
        "browser trusted write enabled",
        "auto-fix enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "production qa certification enabled",
        "quality guarantee enabled",
        "current godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(
            !combined.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
    }
}

#[test]
fn qa_swarm_governance_recommends_next_bounded_sequence() {
    let handoff = HANDOFF.replace('\n', " ");
    let roadmap = ROADMAP.replace('\n', " ");
    let readme = README.replace('\n', " ");
    for text in [handoff.as_str(), roadmap.as_str(), readme.as_str()] {
        assert!(text.contains("Safe Source Mutation Apply"));
        assert!(text.contains("Full Studio Editor"));
        assert!(text.contains("Godot-Plus Demo"));
    }
    assert!(handoff.contains("#716"));
    assert!(handoff.contains("#1 remains open"));
    assert!(handoff.contains("#23 remains open"));
}

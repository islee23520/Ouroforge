//! Governance-refresh contract for Grid-Puzzle Game Class v1 / Era F Milestone 27
//! (#1578). Machine-checks that the roadmap records Milestone 27 as complete
//! strictly against merged evidence, reaffirms the Era F boundaries, and keeps
//! #1 and #23 open governance anchors — without affirmatively closing them or
//! overclaiming maturity.

const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const SCOPE: &str = include_str!("../../../docs/grid-puzzle-game-class-v1.md");
const DEMO: &str = include_str!("../../../docs/grid-puzzle-game-class-v1-demo.md");

#[test]
fn grid_puzzle_governance_marks_milestone_complete_without_closing_anchors() {
    assert!(
        ROADMAP.contains("Grid-Puzzle Game Class v1 governance refresh"),
        "roadmap is missing the dedicated grid-puzzle governance refresh section"
    );
    assert!(
        ROADMAP.contains("Grid-Puzzle Game Class v1 (Era F Milestone 27 under #1) is now complete"),
        "roadmap is missing the grid-puzzle completion prose"
    );

    // Completion is recorded only against merged issue + PR evidence.
    for issue in 1573..=1577 {
        assert!(
            ROADMAP.contains(&format!("#{issue}")),
            "missing issue #{issue} from grid-puzzle governance evidence"
        );
    }
    for pr in ["#1621", "#1631", "#1753", "#1748", "#1763"] {
        assert!(
            ROADMAP.contains(pr),
            "missing merged PR {pr} from grid-puzzle governance evidence"
        );
    }

    // Governance anchors must stay open and never be affirmatively closed.
    assert!(ROADMAP.contains("#1 and #23 remain open"));
    let roadmap_lower = ROADMAP.to_ascii_lowercase();
    for affirmative_closure in [
        "#1 is closed",
        "#23 is closed",
        "#1 and #23 are closed",
        "closes #1 and #23",
        "closed #1 and #23",
    ] {
        assert!(
            !roadmap_lower.contains(affirmative_closure),
            "roadmap affirmatively claims anchors closed: {affirmative_closure}"
        );
    }
}

#[test]
fn grid_puzzle_governance_records_completed_declarative_capabilities() {
    let combined = format!("{ROADMAP}\n{SCOPE}\n{DEMO}").to_ascii_lowercase();
    for required in [
        "grid-puzzle",
        "block-pushing",
        "sokoban",
        "puzzlescript",
        "validate-then-load",
        "dsl ingest",
        "four-gate",
        "loop-coverage",
        "ladder rung",
        "scenario coverage v27",
        "fail-closed",
    ] {
        assert!(
            combined.contains(required),
            "missing completed declarative capability: {required}"
        );
    }
}

#[test]
fn grid_puzzle_governance_keeps_forbidden_claims_blocked() {
    let roadmap_lower = ROADMAP.to_ascii_lowercase();
    // The refresh reaffirms the Era F boundaries.
    for boundary in [
        "proposal-only",
        "read-only",
        "no new engine",
        "demand-driven",
        "no-go",
        "#1508",
    ] {
        assert!(
            roadmap_lower.contains(boundary),
            "missing reaffirmed boundary: {boundary}"
        );
    }

    // No affirmative maturity / replacement overclaim is present.
    for forbidden in [
        "production-ready engine shipped",
        "godot replacement is implemented",
        "godot parity achieved",
        "games are good",
        "shippable game",
        "auto-merge enabled",
    ] {
        assert!(
            !roadmap_lower.contains(forbidden),
            "forbidden affirmative overclaim present: {forbidden}"
        );
    }
}

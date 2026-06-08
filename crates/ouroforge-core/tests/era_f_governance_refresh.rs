//! Consolidated governance-refresh contract for Era F (Milestones 27-34) / the
//! Era F closing governance refresh (#1 Milestone 35, #1619). Machine-checks that
//! the roadmap records Era F as complete strictly against merged per-milestone
//! evidence, assesses the extended north-star descriptively, reaffirms every Era F
//! boundary, and keeps #1 and #23 open governance anchors — without affirmatively
//! closing them or overclaiming maturity. This is a documentation regression
//! guard; it adds no executable behavior.

const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const CHARTER: &str = include_str!("../../../docs/oss-trust-charter.md");

#[test]
fn era_f_governance_marks_era_complete_without_closing_anchors() {
    assert!(
        ROADMAP.contains("Era F (Milestones 27–34) governance refresh"),
        "roadmap is missing the consolidated Era F governance refresh section"
    );
    assert!(
        ROADMAP.contains(
            "Era F (**Accessible Authoring and Genre Verticalization**, Milestones 27–34"
        ),
        "roadmap is missing the Era F completion prose"
    );
    let roadmap_lower = ROADMAP.to_ascii_lowercase();
    assert!(
        roadmap_lower.contains("complete on merged evidence"),
        "Era F completion must be recorded only on merged evidence"
    );
    assert!(
        roadmap_lower.contains("marks **no** milestone complete ahead of merged evidence")
            || roadmap_lower.contains("no milestone is marked complete ahead of merged evidence")
            || roadmap_lower.contains("no\nmilestone complete ahead of merged evidence"),
        "Era F refresh must not mark any milestone complete ahead of merged evidence"
    );

    // Completion is recorded only against merged per-milestone issue evidence
    // spanning Milestones 27-34.
    let milestone_issue_ranges = [
        (1573, 1577), // M27 Grid-Puzzle Game Class
        (1579, 1585), // M28 Solver / Over-Solution / Design-Integrity Gate
        (1587, 1590), // M29 Design Regression Harness
        (1592, 1597), // M30 Generative Front Door
        (1599, 1603), // M31 Deck-Roguelike Game Class
        (1605, 1610), // M32 Synthetic Player Balance
        (1612, 1616), // M33 Evidence-Native Marketplace
        (1618, 1618), // M34 OSS Trust Charter & Paid-Cloud Boundary Gate
    ];
    for (lo, hi) in milestone_issue_ranges {
        for issue in lo..=hi {
            assert!(
                ROADMAP.contains(&format!("#{issue}")),
                "missing issue #{issue} from Era F governance evidence"
            );
        }
    }
    // Representative merged PRs proving the load-bearing capabilities landed:
    // the design-integrity gate (M28) and the trust charter (M34).
    for pr in ["#1751", "#1626"] {
        assert!(
            ROADMAP.contains(pr),
            "missing merged PR {pr} from Era F governance evidence"
        );
    }

    // Governance anchors must stay open and never be affirmatively closed.
    assert!(roadmap_lower.contains("#1 and #23 remain open"));
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
fn era_f_governance_assesses_extended_north_star_descriptively() {
    let combined = format!("{ROADMAP}\n{CHARTER}").to_ascii_lowercase();
    // The extended north-star is assessed across all four dimensions.
    for required in [
        "loop coverage × game complexity × trust × accessibility",
        "front door",
        "engine room",
        "layers, not alternatives",
        "two new genre verticals",
        "grid puzzle",
        "deck roguelike",
        "loop-coverage",
        "ladder rung",
        "non-developer",
        "verified-solvable proposal",
        "descriptive",
    ] {
        assert!(
            combined.contains(required),
            "missing north-star assessment element: {required}"
        );
    }
    // Per-milestone scenario coverage continues from Era E (v26) through v33.
    for coverage in [
        "scenario coverage v27",
        "scenario coverage v28",
        "scenario coverage v29",
        "scenario coverage v30",
        "scenario coverage v31",
        "scenario coverage v32",
        "scenario coverage v33",
    ] {
        assert!(
            combined.contains(coverage),
            "missing Era F scenario coverage milestone: {coverage}"
        );
    }
}

#[test]
fn era_f_governance_keeps_forbidden_claims_blocked() {
    let roadmap_lower = ROADMAP.to_ascii_lowercase();
    // The consolidated refresh reaffirms the Era F boundaries.
    for boundary in [
        "proposal-only",
        "read-only",
        "demand-driven",
        "layer-3",
        "#1508",
        "no-go",
        "adr #92",
        "process guarantee",
    ] {
        assert!(
            roadmap_lower.contains(boundary),
            "missing reaffirmed Era F boundary: {boundary}"
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
        "self-approval enabled",
    ] {
        assert!(
            !roadmap_lower.contains(forbidden),
            "forbidden affirmative overclaim present: {forbidden}"
        );
    }
}

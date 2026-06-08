//! Governance-refresh contract for Deck-Roguelike Game Class v1 (#1604).
//!
//! Locks the roadmap completion record for Era F Milestone 31 against merged
//! evidence and reaffirms the Era F boundaries, mirroring the prior
//! `*_governance_refresh.rs` contracts. Documentation/governance only — it
//! changes no runtime behavior; it just makes the roadmap record CI-gated so a
//! later compaction cannot silently drop the milestone record or the #1/#23
//! anchors.

const ROADMAP: &str = include_str!("../../../docs/roadmap.md");
const SCOPE: &str = include_str!("../../../docs/deck-roguelike-game-class-v1.md");
const DEMO: &str = include_str!("../../../docs/deck-roguelike-game-class-v1-demo.md");
const COVERAGE: &str = include_str!("../../../docs/scenario-coverage-v31.md");

#[test]
fn roadmap_records_deck_roguelike_v1_completion_on_merged_evidence() {
    let roadmap = ROADMAP.replace('\n', " ");
    assert!(
        roadmap.contains("Deck-Roguelike Game Class v1 governance refresh"),
        "roadmap is missing the deck-roguelike governance refresh subsection"
    );
    assert!(
        roadmap.contains("complete for Era F Milestone 31"),
        "roadmap does not record Milestone 31 completion"
    );
    // Completion is recorded strictly against the merged evidence chain.
    for evidence in [
        "#1599", "PR #1624", "#1600", "PR #1630", "#1601", "PR #1752", "#1602", "PR #1757",
        "#1603", "PR #1760",
    ] {
        assert!(
            roadmap.contains(evidence),
            "roadmap is missing merged evidence reference: {evidence}"
        );
    }
    assert!(
        roadmap.contains("#1 and #23 remain open governance anchors"),
        "roadmap must reaffirm #1 and #23 as open anchors"
    );
}

#[test]
fn roadmap_reaffirms_deck_roguelike_era_f_boundaries() {
    let roadmap = ROADMAP.replace('\n', " ");
    for boundary in [
        "All randomness is seeded and replay-stable",
        "seeded stochastic state",
        "no new engine, runtime, or writer",
        "proposal-only",
        "browser/Studio surfaces remain read-only",
        "the deck-roguelike digest key is additive",
        "DEFER until a #1508 Layer-3 GO",
        "Elixir remains NO-GO under ADR #92",
    ] {
        assert!(
            roadmap.contains(boundary),
            "roadmap is missing reaffirmed boundary: {boundary}"
        );
    }
    // Conservative wording is asserted positively (the boundary disclaims the
    // overclaims) rather than by forbidding substrings, which would
    // false-positive on the negated disclaimer itself.
    assert!(
        roadmap.contains(
            "no production-readiness, quality, fun, shippable, or current Godot replacement/parity claim is introduced"
        ),
        "roadmap must explicitly disclaim production/quality/fun/Godot claims"
    );
}

#[test]
fn deck_roguelike_governance_docs_remain_present_and_scoped() {
    assert!(
        SCOPE.contains("deck-roguelike") || SCOPE.contains("Deck-Roguelike"),
        "scope doc is missing"
    );
    assert!(
        DEMO.to_lowercase().contains("fixture-scoped"),
        "demo doc must record the fixture-scoped boundary"
    );
    assert!(
        COVERAGE.contains("Scenario Coverage v31"),
        "coverage doc must record Scenario Coverage v31"
    );
    for doc in [DEMO, COVERAGE] {
        assert!(
            doc.contains("#1") && doc.contains("#23"),
            "governance docs must reaffirm the #1/#23 anchors"
        );
    }
}

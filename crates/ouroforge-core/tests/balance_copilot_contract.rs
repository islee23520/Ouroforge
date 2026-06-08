//! Contract test for Balance Tuning Co-Pilot v1 (#1870).
//!
//! Surfaces Milestone 50 dominant-build findings as advisory recommendations
//! that require human approval/tweak and mechanical re-verification. The co-pilot
//! never auto-applies, never performs trusted writes, and never claims automated
//! fun/quality/release readiness.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_copilot::{
    record_balance_copilot_human_approval, reverify_balance_copilot_approval,
    surface_balance_recommendations, BalanceCopilotApprovalInput, BalanceCopilotHumanApproval,
    BalanceCopilotRecommendationSet, BalanceCopilotReverificationReport, BALANCE_COPILOT_BOUNDARY,
    BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED,
};
use ouroforge_core::balance_dominant_build::{
    analyze_dominant_builds, DominantBuildTelemetryFixture,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_balance_fixture(name: &str) -> DominantBuildTelemetryFixture {
    let path = repo_root()
        .join("examples/engine-builder-balance-v1/dominant-build")
        .join(name);
    let text = std::fs::read_to_string(path).expect("balance fixture exists");
    serde_json::from_str(&text).expect("balance fixture parses")
}

fn read_copilot_fixture(name: &str) -> String {
    let path = repo_root().join("examples/balance-copilot-v1").join(name);
    std::fs::read_to_string(path).expect("balance co-pilot fixture exists")
}

#[test]
fn dominant_build_finding_surfaces_advisory_recommendation_fixture() {
    let before = analyze_dominant_builds(&read_balance_fixture("dominant-build.fixture.json"))
        .expect("dominant-build report");
    let generated = surface_balance_recommendations(&before, "balance-copilot-recset-1870")
        .expect("recommendation set");
    let fixture = BalanceCopilotRecommendationSet::from_json_str(&read_copilot_fixture(
        "recommendation-set-v1.json",
    ))
    .expect("fixture parses");

    fixture.validate().expect("fixture validates");
    assert_eq!(generated, fixture);
    assert_eq!(generated.recommendations.len(), 1);
    let recommendation = &generated.recommendations[0];
    assert_eq!(recommendation.target_build_id, "loop-engine");
    assert_eq!(recommendation.pick_rate_bps, 6000);
    assert_eq!(recommendation.win_rate_bps, 10000);
    assert!(recommendation.review_apply_required);
    assert!(!recommendation.auto_apply_allowed);
    assert!(!recommendation.trusted_write_authority);
    assert_eq!(generated.boundary, BALANCE_COPILOT_BOUNDARY);
}

#[test]
fn human_approval_tweak_reverifies_without_auto_apply() {
    let before = analyze_dominant_builds(&read_balance_fixture("dominant-build.fixture.json"))
        .expect("before report");
    let after = analyze_dominant_builds(&read_balance_fixture("balanced-builds.fixture.json"))
        .expect("after report");
    let set = surface_balance_recommendations(&before, "balance-copilot-recset-1870")
        .expect("recommendation set");
    let approval = record_balance_copilot_human_approval(
        &set,
        BalanceCopilotApprovalInput {
            approval_id: "balance-copilot-approval-1870",
            recommendation_id: "balance-rec-dominant-loop-engine",
            decision: "approved-with-tweak",
            human_actor: "era-j-human-gate",
            rationale: "Approve a smaller synergy reduction proposal for review/apply, then re-verify mechanically before any trusted change is accepted.",
            tweaked_action: Some("increase-counterplay-window"),
            recorded_at_unix_ms: 1_870_000_000_000,
        },
    )
    .expect("human approval");
    let approval_fixture =
        BalanceCopilotHumanApproval::from_json_str(&read_copilot_fixture("human-approval-v1.json"))
            .expect("approval fixture parses");

    assert_eq!(approval, approval_fixture);
    let reverification = reverify_balance_copilot_approval(
        &set,
        &approval,
        &before,
        &after,
        "balance-copilot-reverify-1870",
    )
    .expect("reverification report");
    let reverification_fixture = BalanceCopilotReverificationReport::from_json_str(
        &read_copilot_fixture("reverification-v1.json"),
    )
    .expect("reverification fixture parses");

    assert_eq!(reverification, reverification_fixture);
    assert_eq!(
        reverification.status,
        BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED
    );
    assert!(!reverification.dominant_build_still_flagged);
    assert_eq!(reverification.remaining_dominant_build_count, 0);
    assert!(reverification.human_approved);
    assert!(reverification.review_apply_required);
    assert!(!reverification.auto_apply_performed);
    assert!(!reverification.trusted_write_authority);
}

#[test]
fn auto_apply_or_trusted_write_fixture_fails_closed() {
    let err = BalanceCopilotHumanApproval::from_json_str(&read_copilot_fixture(
        "no-auto-apply-violation.json",
    ))
    .and_then(|approval| approval.validate())
    .expect_err("auto-apply/trusted write drift fails closed");

    assert!(
        err.to_string().contains("must require re-verification")
            || err.to_string().contains("auto-apply/trusted writes"),
        "unexpected error: {err}"
    );
}

#[test]
fn balanced_report_does_not_emit_recommendations() {
    let balanced = analyze_dominant_builds(&read_balance_fixture("balanced-builds.fixture.json"))
        .expect("balanced report");
    let err = surface_balance_recommendations(&balanced, "balance-copilot-recset-balanced")
        .expect_err("no dominance means no recommendation set");

    assert!(err
        .to_string()
        .contains("requires at least one dominant build"));
}

#[test]
fn rejected_or_deferred_decisions_do_not_reverify() {
    let before = analyze_dominant_builds(&read_balance_fixture("dominant-build.fixture.json"))
        .expect("before report");
    let after = analyze_dominant_builds(&read_balance_fixture("balanced-builds.fixture.json"))
        .expect("after report");
    let set = surface_balance_recommendations(&before, "balance-copilot-recset-1870")
        .expect("recommendation set");
    let rejected = record_balance_copilot_human_approval(
        &set,
        BalanceCopilotApprovalInput {
            approval_id: "balance-copilot-rejected-1870",
            recommendation_id: "balance-rec-dominant-loop-engine",
            decision: "rejected",
            human_actor: "era-j-human-gate",
            rationale: "Human rejected the proposal; mechanical re-verification must not run as approved evidence.",
            tweaked_action: None,
            recorded_at_unix_ms: 1_870_000_000_002,
        },
    )
    .expect("rejected approval record");

    let err = reverify_balance_copilot_approval(
        &set,
        &rejected,
        &before,
        &after,
        "balance-copilot-reverify-rejected-1870",
    )
    .expect_err("reverify requires approved human decision");

    assert!(err
        .to_string()
        .contains("requires a human approved decision"));
}

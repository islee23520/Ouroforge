//! Smoke contract for Release Readiness Demo v1 (#1872).
//!
//! The demo is fixture-scoped and deterministic: it recomputes a balance
//! co-pilot recommendation, human approval, re-verification, and a release
//! readiness bundle that still requires the separate human go/no-go record.

use std::path::{Path, PathBuf};

use ouroforge_core::balance_copilot::{
    record_balance_copilot_human_approval, reverify_balance_copilot_approval,
    surface_balance_recommendations, BalanceCopilotApprovalInput, BalanceCopilotHumanApproval,
    BalanceCopilotRecommendationSet, BalanceCopilotReverificationReport,
    BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED,
};
use ouroforge_core::balance_dominant_build::{
    analyze_dominant_builds, DominantBuildTelemetryFixture,
};
use ouroforge_core::release_readiness::{
    build_release_readiness_bundle, record_release_go_no_go, ReleaseGoNoGoInput,
    ReleaseGoNoGoRecord, ReleaseReadinessBundle, ReleaseReadinessInput, ReleaseReadinessStatus,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_text(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path)).expect("fixture/doc exists")
}

fn read_balance_fixture(path: &str) -> DominantBuildTelemetryFixture {
    serde_json::from_str(&read_text(path)).expect("balance fixture parses")
}

fn manifest() -> Value {
    serde_json::from_str(&read_text(
        "examples/release-readiness-v1/demo/manifest.json",
    ))
    .expect("demo manifest parses")
}

fn manifest_str<'a>(manifest: &'a Value, pointer: &str) -> &'a str {
    manifest
        .pointer(pointer)
        .and_then(Value::as_str)
        .expect("manifest string field exists")
}

#[test]
fn demo_recomputes_human_approved_balance_reverification() {
    let manifest = manifest();
    assert_eq!(manifest["determinism"]["networkRequired"], false);
    assert_eq!(manifest["determinism"]["liveBrowserRequired"], false);
    assert_eq!(manifest["determinism"]["fixtureScoped"], true);

    let before = analyze_dominant_builds(&read_balance_fixture(manifest_str(
        &manifest,
        "/balanceCopilot/sourceTelemetryFixture",
    )))
    .expect("before report");
    let after = analyze_dominant_builds(&read_balance_fixture(manifest_str(
        &manifest,
        "/balanceCopilot/balancedTelemetryFixture",
    )))
    .expect("after report");
    let generated_set = surface_balance_recommendations(&before, "balance-copilot-recset-1870")
        .expect("recommendation set");
    let fixture_set = BalanceCopilotRecommendationSet::from_json_str(&read_text(manifest_str(
        &manifest,
        "/balanceCopilot/recommendationSetFixture",
    )))
    .expect("recommendation fixture");
    assert_eq!(generated_set, fixture_set);
    let recommendation = generated_set
        .recommendations
        .iter()
        .find(|rec| {
            rec.recommendation_id
                == manifest_str(&manifest, "/balanceCopilot/expectedRecommendationId")
        })
        .expect("expected recommendation");
    assert_eq!(
        recommendation.target_build_id,
        manifest_str(&manifest, "/balanceCopilot/expectedTargetBuildId")
    );

    let approval_fixture = BalanceCopilotHumanApproval::from_json_str(&read_text(manifest_str(
        &manifest,
        "/balanceCopilot/humanApprovalFixture",
    )))
    .expect("approval fixture");
    let generated_approval = record_balance_copilot_human_approval(
        &generated_set,
        BalanceCopilotApprovalInput {
            approval_id: &approval_fixture.approval_id,
            recommendation_id: &approval_fixture.recommendation_id,
            decision: &approval_fixture.decision,
            human_actor: &approval_fixture.human_actor,
            rationale: &approval_fixture.rationale,
            tweaked_action: approval_fixture.tweaked_action.as_deref(),
            recorded_at_unix_ms: approval_fixture.recorded_at_unix_ms,
        },
    )
    .expect("approval records");
    assert_eq!(generated_approval, approval_fixture);
    assert_eq!(
        generated_approval.decision,
        manifest_str(&manifest, "/balanceCopilot/expectedDecision")
    );

    let generated_reverification = reverify_balance_copilot_approval(
        &generated_set,
        &generated_approval,
        &before,
        &after,
        "balance-copilot-reverify-1870",
    )
    .expect("reverification records");
    let reverification_fixture = BalanceCopilotReverificationReport::from_json_str(&read_text(
        manifest_str(&manifest, "/balanceCopilot/reverificationFixture"),
    ))
    .expect("reverification fixture");
    assert_eq!(generated_reverification, reverification_fixture);
    assert_eq!(
        generated_reverification.status,
        BALANCE_COPILOT_STATUS_REVERIFIED_IMPROVED
    );
    assert_eq!(
        generated_reverification.status,
        manifest_str(&manifest, "/balanceCopilot/expectedReverificationStatus")
    );
    assert!(!generated_reverification.auto_apply_performed);
    assert!(!generated_reverification.trusted_write_authority);
}

#[test]
fn demo_bundle_requires_separate_human_go_no_go_record() {
    let manifest = manifest();
    let input = ReleaseReadinessInput::from_json_str(&read_text(manifest_str(
        &manifest,
        "/releaseReadiness/bundleInputFixture",
    )))
    .expect("bundle input");
    let generated_bundle = build_release_readiness_bundle(&input).expect("bundle builds");
    let fixture_bundle = ReleaseReadinessBundle::from_json_str(&read_text(manifest_str(
        &manifest,
        "/releaseReadiness/bundleFixture",
    )))
    .expect("bundle fixture");
    assert_eq!(generated_bundle, fixture_bundle);
    assert_eq!(generated_bundle.status, ReleaseReadinessStatus::Ready);
    assert!(generated_bundle.human_go_no_go_required);
    assert!(!generated_bundle.human_go_no_go_recorded);
    assert!(!generated_bundle.release_authority_granted);
    assert!(!generated_bundle.auto_merge_allowed);
    assert!(!generated_bundle.trusted_write_authority);

    let decision = ReleaseGoNoGoInput::from_json_str(&read_text(manifest_str(
        &manifest,
        "/releaseReadiness/goNoGoInputFixture",
    )))
    .expect("go/no-go input");
    let generated_record =
        record_release_go_no_go(&generated_bundle, &decision).expect("human go/no-go records");
    let fixture_record = ReleaseGoNoGoRecord::from_json_str(&read_text(manifest_str(
        &manifest,
        "/releaseReadiness/goNoGoRecordFixture",
    )))
    .expect("go/no-go fixture");
    assert_eq!(generated_record, fixture_record);
    assert!(generated_record.release_ready);
    assert!(generated_record.human_confirmed);
    assert!(!generated_record.release_authority_granted);
    assert!(!generated_record.auto_merge_allowed);
    assert!(!generated_record.trusted_write_authority);
}

#[test]
fn demo_docs_preserve_determinism_generated_state_and_conservative_wording() {
    let manifest = manifest();
    let boundary = manifest_str(&manifest, "/boundary").to_ascii_lowercase();
    for required in [
        "fixture-scoped",
        "no network/live browser",
        "browser/studio read-only",
        "no auto-apply",
        "no auto-merge",
        "no release authority",
        "no automated fun score",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }

    let doc = read_text("docs/release-readiness-v1-demo.md");
    let lower = doc.to_ascii_lowercase();
    assert!(lower
        .contains("cargo test -p ouroforge-core --test release_readiness_demo_contract --jobs 2"));
    assert!(lower.contains("generated runs/artifacts remain untracked unless fixture-scoped"));
    assert!(lower.contains("#1 and #23 remain open"));
    for forbidden in [
        "godot replacement",
        "production-ready",
        "automated fun score passes",
        "grants auto-merge authority",
        "trusted-write authority granted",
    ] {
        assert!(
            !lower.contains(forbidden),
            "forbidden wording present: {forbidden}"
        );
    }
}

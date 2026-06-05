use serde_json::Value;

const COVERAGE_FIXTURE: &str =
    include_str!("../../../examples/godot-plus-demo-regression-v18/coverage-matrix.fixture.json");

const REQUIRED_SUCCESS: &[&str] = &[
    "GPD18.success-game-start",
    "GPD18.success-player-movement",
    "GPD18.success-objective-update",
    "GPD18.success-enemy-npc-behavior",
    "GPD18.success-level-completion",
    "GPD18.success-lose-restart",
    "GPD18.success-hud-update",
    "GPD18.success-runtime-probe",
    "GPD18.success-qa-pass",
    "GPD18.success-export-package-evidence",
    "GPD18.success-studio-walkthrough-rendering",
    "GPD18.success-plugin-descriptor-validation",
    "GPD18.success-evidence-bundle-existence",
];

const REQUIRED_BLOCKED: &[&str] = &[
    "GPD18.block-broken-objective",
    "GPD18.block-missing-asset",
    "GPD18.block-missing-probe",
    "GPD18.block-invalid-level-metadata",
    "GPD18.block-broken-export",
    "GPD18.block-invalid-plugin",
    "GPD18.block-unclassified-qa-failure",
    "GPD18.block-incomplete-evidence-bundle",
    "GPD18.block-direct-source-apply-attempt",
    "GPD18.block-publish-deploy-attempt",
];

fn coverage_fixture() -> Value {
    serde_json::from_str(COVERAGE_FIXTURE).expect("coverage fixture parses")
}

#[test]
fn scenario_coverage_v18_matrix_declares_required_success_and_blocked_cases() {
    let fixture = coverage_fixture();
    assert_eq!(
        fixture["schemaVersion"],
        "scenario-coverage-v18-godot-plus-demo-regression-v1"
    );
    assert_eq!(fixture["issue"], 796);
    assert_eq!(fixture["status"], "fixture-scoped");
    assert_eq!(fixture["governanceAnchors"]["issue1"], "open");
    assert_eq!(fixture["governanceAnchors"]["issue23"], "open");
    assert_eq!(
        fixture["governanceAnchors"]["closureStatement"],
        "#1 and #23 remain open"
    );

    let scenarios = fixture["scenarios"]
        .as_array()
        .expect("scenarios are an array");
    let ids: Vec<&str> = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("scenario id"))
        .collect();

    for required in REQUIRED_SUCCESS.iter().chain(REQUIRED_BLOCKED.iter()) {
        assert!(ids.contains(required), "missing scenario {required}");
    }

    let success_count = scenarios
        .iter()
        .filter(|scenario| scenario["kind"] == "success")
        .count();
    let blocked_count = scenarios
        .iter()
        .filter(|scenario| scenario["kind"] == "blocked")
        .count();
    assert_eq!(success_count, REQUIRED_SUCCESS.len());
    assert_eq!(blocked_count, REQUIRED_BLOCKED.len());
}

#[test]
fn scenario_coverage_v18_documents_fixture_scope_and_guardrails() {
    let lower_fixture = COVERAGE_FIXTURE.to_ascii_lowercase();
    for term in [
        "fixture-scoped",
        "generated",
        "read-only",
        "draft-only",
        "review-gated",
        "no trusted browser write",
        "no command bridge",
        "no auto-apply",
        "no publish",
        "no deploy",
        "no executable plugin runtime",
        "no marketplace",
        "no commercial release",
        "no godot replacement",
        "#1 and #23 remain open",
    ] {
        assert!(
            lower_fixture.contains(term),
            "fixture missing guardrail {term}"
        );
    }
}

#[test]
fn scenario_coverage_v18_evidence_refs_are_controlled_and_blockers_are_actionable() {
    let fixture = coverage_fixture();
    let scenarios = fixture["scenarios"]
        .as_array()
        .expect("scenarios are an array");
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        let evidence_refs = scenario["evidenceRefs"]
            .as_array()
            .expect("evidence refs are an array");
        assert!(!evidence_refs.is_empty(), "{id} has evidence refs");
        for evidence_ref in evidence_refs {
            let evidence_ref = evidence_ref.as_str().expect("evidence ref string");
            let controlled = evidence_ref.starts_with("examples/")
                || evidence_ref.starts_with("docs/")
                || evidence_ref.starts_with("runs/")
                || evidence_ref.starts_with("target/")
                || evidence_ref.starts_with("dashboard-data/")
                || evidence_ref.starts_with(".openchrome/")
                || evidence_ref.starts_with(".omc/")
                || evidence_ref.starts_with(".omx/")
                || evidence_ref.starts_with(".claude/");
            assert!(
                controlled,
                "{id} has uncontrolled evidence ref {evidence_ref}"
            );
        }

        if scenario["kind"] == "blocked" {
            let expected = scenario["expected"].as_str().expect("blocked expected");
            let diagnostic = scenario["diagnostic"].as_str().expect("blocked diagnostic");
            assert!(
                expected.contains("block")
                    || expected.contains("reject")
                    || expected.contains("fail closed")
                    || expected.contains("diagnostic"),
                "{id} expected outcome is not fail-closed: {expected}"
            );
            assert!(
                diagnostic.contains("Fix")
                    || diagnostic.contains("Provide")
                    || diagnostic.contains("Restore")
                    || diagnostic.contains("Reject")
                    || diagnostic.contains("Classify")
                    || diagnostic.contains("Review"),
                "{id} diagnostic is not actionable: {diagnostic}"
            );
        }
    }
}

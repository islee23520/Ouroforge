use ouroforge_core::release_compliance_gate::{
    evaluate_release_compliance, ComplianceVerdictStatus, ReleaseComplianceGateInput,
    RELEASE_COMPLIANCE_GATE_SCHEMA_VERSION,
};
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_fixture(name: &str) -> String {
    let path = repo_root().join(format!(
        "examples/release-compliance-gate-v1/fixtures/{name}"
    ));
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"))
}

fn input(name: &str) -> ReleaseComplianceGateInput {
    ReleaseComplianceGateInput::from_json_str(&read_fixture(name))
        .unwrap_or_else(|err| panic!("{name}: {err}"))
}

#[test]
fn compliant_release_passes_composed_compliance_gate() {
    let verdict = evaluate_release_compliance(&input("compliant-release.json")).expect("verdict");
    assert_eq!(
        verdict.schema_version,
        RELEASE_COMPLIANCE_GATE_SCHEMA_VERSION
    );
    assert_eq!(verdict.status, ComplianceVerdictStatus::Pass);
    assert!(verdict.reasons.is_empty(), "{verdict:#?}");
    assert!(verdict
        .composes_with
        .iter()
        .any(|gate| gate == "reviewer-gate"));
    assert!(verdict
        .composes_with
        .iter()
        .any(|gate| gate == "evaluator-declared-gate"));
    assert!(verdict.boundary.contains("no release authority"));
    assert!(verdict.boundary.contains("read-only"));
}

#[test]
fn license_policy_and_age_rating_violations_block_release_candidate() {
    let verdict = evaluate_release_compliance(&input("violation-release.json")).expect("verdict");
    assert_eq!(verdict.status, ComplianceVerdictStatus::Blocked);
    assert!(verdict
        .reasons
        .iter()
        .any(|reason| reason.contains("policy violation")));
    assert!(verdict
        .reasons
        .iter()
        .any(|reason| reason.contains("age-rating signals")));
    assert!(verdict
        .reasons
        .iter()
        .any(|reason| reason.contains("missing license")));
    assert!(verdict
        .reasons
        .iter()
        .any(|reason| reason.contains("missing provenance")));
    assert!(verdict.rollback_command_is_not_present_for_humans());
}

trait NoRollbackCommand {
    fn rollback_command_is_not_present_for_humans(&self) -> bool;
}

impl NoRollbackCommand for ouroforge_core::release_compliance_gate::ReleaseComplianceGateVerdict {
    fn rollback_command_is_not_present_for_humans(&self) -> bool {
        self.boundary.contains("no auto-merge") && self.boundary.contains("no release authority")
    }
}

#[test]
fn malformed_compliance_input_fails_closed_before_verdict() {
    let result = ReleaseComplianceGateInput::from_json_str(&read_fixture("malformed-release.json"));
    assert!(result.is_err(), "malformed fixture must fail closed");
}

#[test]
fn docs_and_fixtures_preserve_generated_state_wording_compatibility_and_governance() {
    let docs = std::fs::read_to_string(repo_root().join("docs/release-compliance-gate-v1.md"))
        .expect("docs exist");
    for required in [
        "composes with existing reviewer/evaluator gates",
        "not a new evaluator",
        "Humans retain release go/no-go",
        "browser/Studio surfaces remain read-only",
        "executes no commands",
        "#1 and #23 remain open",
    ] {
        assert!(
            docs.contains(required),
            "docs missing required phrase: {required}"
        );
    }
    for forbidden in [
        "auto-merges nothing",
        "self-approves nothing",
        "production-ready",
        "quality/fun",
        "Godot replacement/parity",
    ] {
        assert!(
            docs.contains(forbidden),
            "docs must explicitly forbid {forbidden}"
        );
    }

    for fixture in ["compliant-release.json", "violation-release.json"] {
        let input = input(fixture);
        assert!(input.generated_state.generated);
        assert!(input.generated_state.tracked);
        assert!(input.generated_state.fixture_scoped);
        assert!(input.boundary.contains("read-only"));
        assert!(input.boundary.contains("no release authority"));
    }
}

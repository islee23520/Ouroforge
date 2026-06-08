use ouroforge_core::release_auto_apply::{
    decide_release_auto_apply, ReleaseAutoApplyRequest, RELEASE_AUTO_APPLY_SCHEMA_VERSION,
};
use ouroforge_core::trust_gradient_audit::AutoApplyAuditLog;
use ouroforge_core::trust_gradient_auto_apply::AutoApplyOutcome;

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_fixture(name: &str) -> String {
    let path = workspace_path(&format!("examples/release-auto-apply-v1/fixtures/{name}"));
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn request(name: &str) -> ReleaseAutoApplyRequest {
    ReleaseAutoApplyRequest::from_json_str(&read_fixture(name))
        .unwrap_or_else(|err| panic!("{name}: {err}"))
}

#[test]
fn eligible_release_scale_auto_applies_with_one_command_rollback() {
    let decision = decide_release_auto_apply(
        &request("eligible-auto-apply.json"),
        &AutoApplyAuditLog::new(),
    )
    .expect("decision succeeds");

    assert_eq!(decision.schema_version, RELEASE_AUTO_APPLY_SCHEMA_VERSION);
    assert_eq!(decision.outcome, AutoApplyOutcome::AutoApplied);
    let rollback = decision.rollback_command.expect("rollback command present");
    assert!(rollback.starts_with("ouroforge rollback --transaction txn-release-eligible-001"));
    assert!(rollback.contains(" --reverse examples/release-auto-apply-v1/fixtures/reverse/"));
    assert_eq!(decision.trust_decision.budget_after.remaining, 3);
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("human release gate remains pending")));
}

#[test]
fn high_risk_source_affecting_release_change_falls_back_to_manual() {
    let decision = decide_release_auto_apply(
        &request("ineligible-high-risk-manual.json"),
        &AutoApplyAuditLog::new(),
    )
    .expect("decision succeeds");

    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision.rollback_command.is_none());
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("not auto-apply eligible")));
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("risk tier is not low")));
}

#[test]
fn missing_game_scale_rollback_blocks_broadened_auto_apply() {
    let decision = decide_release_auto_apply(
        &request("missing-game-scale-rollback.json"),
        &AutoApplyAuditLog::new(),
    )
    .expect("decision succeeds");

    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision.rollback_command.is_none());
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("rollback scope is not game-scale")));
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("does not cover whole-game state")));
}

#[test]
fn kill_switch_halts_release_scale_autonomy() {
    let log = AutoApplyAuditLog::from_json_str(&read_fixture("kill-switch-audit-log.json"))
        .expect("kill-switch fixture validates");
    assert!(log.is_autonomy_halted());

    let decision = decide_release_auto_apply(&request("eligible-auto-apply.json"), &log)
        .expect("decision succeeds");

    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision.rollback_command.is_none());
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("kill switch engaged")));
}

#[test]
fn docs_and_fixtures_preserve_generated_state_wording_compatibility_and_governance() {
    let docs = std::fs::read_to_string(workspace_path("docs/release-auto-apply-v1.md"))
        .expect("docs exist");
    for required in [
        "extends the Milestone 22 trust gradient",
        "not a new writer",
        "browser/Studio surfaces remain read-only",
        "executes no commands",
        "writes no trusted files",
        "#1 and #23 remain open",
    ] {
        assert!(
            docs.contains(required),
            "docs missing required phrase: {required}"
        );
    }
    for forbidden in [
        "production-ready",
        "Godot replacement/parity",
        "auto-merge",
        "self-approval",
    ] {
        assert!(
            docs.contains(forbidden),
            "docs must explicitly forbid {forbidden}"
        );
    }

    for fixture in [
        "eligible-auto-apply.json",
        "ineligible-high-risk-manual.json",
        "missing-game-scale-rollback.json",
    ] {
        let request = request(fixture);
        assert!(request.boundary.contains("proposal-only"));
        assert!(request.boundary.contains("read-only"));
        assert!(request.boundary.contains("no auto-merge"));
        assert_eq!(
            request.verification.human_release_gate_state,
            ouroforge_core::release_auto_apply::HumanReleaseGateState::PendingHumanGoNoGo
        );
    }
}

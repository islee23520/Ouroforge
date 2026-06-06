use ouroforge_core::trust_gradient_auto_apply::{
    decide_auto_apply, AutoApplyOutcome, AutoApplyRequest, RollbackHandle,
    TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn fixture(name: &str) -> AutoApplyRequest {
    let path = workspace_path(&format!("examples/trust-gradient-v1/fixtures/{name}"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    AutoApplyRequest::from_json_str(&text).unwrap_or_else(|err| panic!("{name}: {err}"))
}

#[test]
fn low_risk_eligible_in_budget_auto_applies_reversibly() {
    let request = fixture("auto-apply-success.json");
    let decision = decide_auto_apply(&request).expect("decides");
    assert_eq!(
        decision.schema_version,
        TRUST_GRADIENT_AUTO_APPLY_SCHEMA_VERSION
    );
    assert_eq!(decision.outcome, AutoApplyOutcome::AutoApplied);
    // One-command rollback is guaranteed for any auto-applied change.
    let command = decision.rollback_command.expect("rollback command present");
    assert!(command.contains("txn-scene-001"));
    assert!(command.contains("rollback"));
    // Budget is consumed.
    assert_eq!(decision.budget_after.remaining, 2);
    // Conservative wording boundary travels with the decision.
    assert!(decision.boundary.contains("not auto-merge"));
    assert!(decision.boundary.contains("not self-approval"));
    assert!(decision.boundary.contains("read-only"));
}

#[test]
fn ineligible_proposal_falls_back_to_manual() {
    let request = fixture("auto-apply-ineligible-manual.json");
    let decision = decide_auto_apply(&request).expect("decides");
    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision.rollback_command.is_none());
    // Budget is untouched on fallback.
    assert_eq!(decision.budget_after.remaining, 3);
}

#[test]
fn gate_regression_on_rerun_falls_back_to_manual() {
    let request = fixture("auto-apply-gate-regression.json");
    let decision = decide_auto_apply(&request).expect("decides");
    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("four gates")));
}

#[test]
fn budget_exhaustion_falls_back_to_manual() {
    let request = fixture("auto-apply-budget-exhausted.json");
    let decision = decide_auto_apply(&request).expect("decides");
    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("budget exhausted")));
}

#[test]
fn autonomy_off_is_the_default_and_falls_back_to_manual() {
    let request = fixture("auto-apply-autonomy-disabled.json");
    let decision = decide_auto_apply(&request).expect("decides");
    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("autonomy disabled")));
}

#[test]
fn missing_rollback_handle_refuses_auto_apply() {
    let mut request = fixture("auto-apply-success.json");
    request.rollback_handle = None;
    let decision = decide_auto_apply(&request).expect("decides");
    assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("rollback")));
}

#[test]
fn empty_rollback_handle_refuses_auto_apply() {
    // An empty/whitespace rollback handle is as non-actionable as a missing one:
    // it would emit `ouroforge rollback --transaction  --reverse `, so auto-apply
    // must fall back to manual review (#1478).
    for handle in [
        RollbackHandle {
            apply_transaction_id: String::new(),
            reverse_ref: String::new(),
        },
        RollbackHandle {
            apply_transaction_id: "   ".to_string(),
            reverse_ref: "tx-reverse".to_string(),
        },
        RollbackHandle {
            apply_transaction_id: "tx-apply".to_string(),
            reverse_ref: "  ".to_string(),
        },
    ] {
        let mut request = fixture("auto-apply-success.json");
        request.rollback_handle = Some(handle);
        let decision = decide_auto_apply(&request).expect("decides");
        assert_eq!(decision.outcome, AutoApplyOutcome::ManualFallback);
        assert!(decision.rollback_command.is_none());
        assert!(decision
            .reasons
            .iter()
            .any(|reason| reason.contains("rollback")));
    }
}

#[test]
fn unexpected_schema_version_is_rejected() {
    let request = fixture("auto-apply-success.json");
    let mut json = serde_json::to_value(&request).expect("serializes");
    json["schemaVersion"] = serde_json::Value::String("bogus".to_string());
    let text = serde_json::to_string(&json).expect("serializes");
    assert!(AutoApplyRequest::from_json_str(&text).is_err());
}

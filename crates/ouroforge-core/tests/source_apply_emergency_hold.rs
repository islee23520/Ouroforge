use ouroforge_core::{
    SourceApplyHold, SourceApplyHoldQuery, SourceApplyHoldScope, SourceApplyHoldScopeKind,
    SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION,
};

fn hold(scopes: Vec<SourceApplyHoldScope>) -> SourceApplyHold {
    SourceApplyHold {
        schema_version: SOURCE_APPLY_EMERGENCY_HOLD_SCHEMA_VERSION.to_string(),
        hold_id: "hold-715-1".to_string(),
        disabled: true,
        reason: "risk detected in source apply pipeline".to_string(),
        actor: "agent-operator".to_string(),
        recorded_at: "2026-06-05T00:00:00Z".to_string(),
        expires_at: None,
        scopes,
        requires_review_to_lift: true,
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        guardrails: vec![
            "hold blocks source apply locally and cannot be bypassed".to_string(),
            "no remote kill switch or cloud control plane".to_string(),
        ],
    }
}

fn global_scope() -> SourceApplyHoldScope {
    SourceApplyHoldScope {
        kind: SourceApplyHoldScopeKind::Global,
        value: String::new(),
    }
}

fn query(now: &str) -> SourceApplyHoldQuery {
    SourceApplyHoldQuery {
        now: now.to_string(),
        transaction_id: "apply-txn-715-1".to_string(),
        file_classes: vec!["rust-source".to_string()],
        force_apply: false,
    }
}

#[test]
fn global_hold_blocks_all_apply() {
    let hold = hold(vec![global_scope()]);
    hold.validate().expect("valid");
    let evaluation = hold.evaluate_against(&query("2026-06-05T01:00:00Z"));
    assert!(evaluation.active);
    assert!(evaluation.apply_blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("globally held")));
}

#[test]
fn scoped_file_class_hold_blocks_matching_apply() {
    let hold = hold(vec![SourceApplyHoldScope {
        kind: SourceApplyHoldScopeKind::FileClass,
        value: "rust-source".to_string(),
    }]);
    let evaluation = hold.evaluate_against(&query("2026-06-05T01:00:00Z"));
    assert!(evaluation.apply_blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("file class")));
}

#[test]
fn scoped_hold_does_not_block_unrelated_apply() {
    let hold = hold(vec![SourceApplyHoldScope {
        kind: SourceApplyHoldScopeKind::FileClass,
        value: "shader-source".to_string(),
    }]);
    let evaluation = hold.evaluate_against(&query("2026-06-05T01:00:00Z"));
    assert!(evaluation.active);
    assert!(!evaluation.apply_blocked);
}

#[test]
fn force_apply_cannot_bypass_an_active_hold() {
    let hold = hold(vec![global_scope()]);
    let mut q = query("2026-06-05T01:00:00Z");
    q.force_apply = true;
    let evaluation = hold.evaluate_against(&q);
    assert!(evaluation.apply_blocked);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("cannot bypass")));
}

#[test]
fn stale_expired_hold_does_not_block() {
    let mut hold = hold(vec![global_scope()]);
    hold.expires_at = Some("2026-06-05T00:30:00Z".to_string());
    // now is after expiry -> hold is stale/inactive.
    let evaluation = hold.evaluate_against(&query("2026-06-05T01:00:00Z"));
    assert!(!evaluation.active);
    assert!(!evaluation.apply_blocked);
}

#[test]
fn lifted_hold_does_not_block() {
    let mut hold = hold(vec![global_scope()]);
    hold.disabled = false;
    let evaluation = hold.evaluate_against(&query("2026-06-05T01:00:00Z"));
    assert!(!evaluation.active);
    assert!(!evaluation.apply_blocked);
}

#[test]
fn enabled_hold_requires_a_reason() {
    let mut hold = hold(vec![global_scope()]);
    hold.reason = String::new();
    assert!(hold.validate().is_err());
}

#[test]
fn enabled_hold_requires_a_scope() {
    let hold = hold(Vec::new());
    assert!(hold.validate().is_err());
}

#[test]
fn global_scope_with_value_is_rejected() {
    let hold = hold(vec![SourceApplyHoldScope {
        kind: SourceApplyHoldScopeKind::Global,
        value: "unexpected".to_string(),
    }]);
    assert!(hold.validate().is_err());
}

#[test]
fn file_class_scope_requires_value() {
    let hold = hold(vec![SourceApplyHoldScope {
        kind: SourceApplyHoldScopeKind::FileClass,
        value: String::new(),
    }]);
    assert!(hold.validate().is_err());
}

#[test]
fn json_round_trip_preserves_hold() {
    let hold = hold(vec![global_scope()]);
    let json = serde_json::to_string_pretty(&hold).expect("serializes");
    let parsed = SourceApplyHold::from_json_str(&json).expect("parses");
    assert_eq!(parsed, hold);
}

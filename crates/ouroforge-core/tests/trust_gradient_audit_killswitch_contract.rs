use ouroforge_core::trust_gradient_audit::{
    ApplyResult, AutoApplyAuditEntry, AutoApplyAuditLog, GateOutcome, GateVerdicts, RiskTier,
    RollbackHandle, TRUST_GRADIENT_AUDIT_SCHEMA_VERSION,
};

fn workspace_path(relative: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_fixture(name: &str) -> String {
    let path = workspace_path(&format!("examples/trust-gradient-v1/fixtures/{name}"));
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn entry(sequence: u64) -> AutoApplyAuditEntry {
    AutoApplyAuditEntry {
        sequence,
        proposal_ref: format!("proposal-{sequence}"),
        tier: RiskTier::Low,
        gates: GateVerdicts {
            mechanical: GateOutcome::Pass,
            runtime: GateOutcome::Pass,
            visual: GateOutcome::Pass,
            semantic: GateOutcome::Pass,
        },
        budget_remaining: 2,
        apply_result: ApplyResult::AutoApplied,
        rollback_handle: RollbackHandle {
            apply_transaction_id: format!("txn-{sequence}"),
            reverse_ref: format!("reverse/txn-{sequence}.json"),
        },
    }
}

#[test]
fn complete_audit_log_validates_and_resolves_rollback() {
    let log = AutoApplyAuditLog::from_json_str(&read_fixture("audit-log-complete.json"))
        .expect("complete log validates");
    assert_eq!(log.schema_version, TRUST_GRADIENT_AUDIT_SCHEMA_VERSION);
    assert_eq!(log.entries.len(), 2);
    assert!(!log.is_autonomy_halted());
    assert!(log.rollback_command(1).unwrap().contains("rollback"));
    assert!(log.boundary.contains("not auto-merge"));
    assert!(log.boundary.contains("read-only"));
}

#[test]
fn tampered_sequence_gap_fails_validation() {
    let result = AutoApplyAuditLog::from_json_str(&read_fixture("audit-log-tampered-gap.json"));
    assert!(
        result.is_err(),
        "a gap in the append-only log must be rejected"
    );
}

#[test]
fn broken_rollback_handle_fails_validation() {
    let result = AutoApplyAuditLog::from_json_str(&read_fixture("audit-log-broken-rollback.json"));
    assert!(
        result.is_err(),
        "an incomplete rollback handle must be rejected"
    );
}

#[test]
fn engaged_kill_switch_halts_further_auto_apply() {
    let mut log =
        AutoApplyAuditLog::from_json_str(&read_fixture("audit-log-kill-switch-engaged.json"))
            .expect("engaged log validates");
    assert!(log.is_autonomy_halted());
    // Once halted, no further auto-apply may be recorded.
    assert!(log.append(entry(log.entries.len() as u64)).is_err());
}

#[test]
fn append_enforces_monotonic_sequences() {
    let mut log = AutoApplyAuditLog::new();
    log.append(entry(0)).unwrap();
    // Non-contiguous append is rejected.
    assert!(log.append(entry(5)).is_err());
    // Correct next sequence is accepted.
    log.append(entry(1)).unwrap();
    assert_eq!(log.entries.len(), 2);
}

#[test]
fn kill_switch_can_be_engaged_then_default_no_auto_apply() {
    let mut log = AutoApplyAuditLog::new();
    log.append(entry(0)).unwrap();
    log.engage_kill_switch("operator emergency halt").unwrap();
    log.validate().unwrap();
    assert!(log.is_autonomy_halted());
    assert!(log.append(entry(1)).is_err());
}

#[test]
fn engaged_kill_switch_requires_non_empty_reason() {
    // Engaging with a blank reason fails closed (#1479).
    let mut log = AutoApplyAuditLog::new();
    log.append(entry(0)).unwrap();
    assert!(log.engage_kill_switch("").is_err());
    assert!(log.engage_kill_switch("   ").is_err());
    assert!(!log.is_autonomy_halted());

    // A log deserialized/constructed with an engaged switch but blank reason
    // must not validate.
    let mut json = serde_json::to_value(&log).expect("serializes");
    json["killSwitch"] = serde_json::json!({ "engaged": true, "reason": "  " });
    let tampered: AutoApplyAuditLog =
        serde_json::from_value(json).expect("deserializes engaged-but-blank kill switch");
    assert!(tampered.validate().is_err());
}

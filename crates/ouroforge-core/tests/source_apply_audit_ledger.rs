use ouroforge_core::{
    SourceApplyAuditApplyStatus, SourceApplyAuditEntry, SourceApplyAuditLedger,
    SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION,
};

fn entry(attempt: &str, status: SourceApplyAuditApplyStatus) -> SourceApplyAuditEntry {
    SourceApplyAuditEntry {
        attempt_id: attempt.to_string(),
        transaction_id: "apply-txn-710-1".to_string(),
        actor: "agent-applier".to_string(),
        recorded_at: "2026-06-05T00:00:00Z".to_string(),
        review_decision_ref: "evidence/review-decision.json".to_string(),
        sandbox_report_ref: "sandbox/report.json".to_string(),
        stale_guard_ref: "evidence/stale-guard.json".to_string(),
        rollback_snapshot_ref: "evidence/rollback-snapshot.json".to_string(),
        verification_log_ref: "runs/verification/log.json".to_string(),
        rerun_comparison_ref: None,
        apply_status: status,
        failure_reason: None,
        blocked_reasons: match status {
            SourceApplyAuditApplyStatus::Blocked | SourceApplyAuditApplyStatus::Held => {
                vec!["precondition not met".to_string()]
            }
            _ => Vec::new(),
        },
    }
}

fn ledger(entries: Vec<SourceApplyAuditEntry>) -> SourceApplyAuditLedger {
    SourceApplyAuditLedger {
        schema_version: SOURCE_APPLY_AUDIT_LEDGER_SCHEMA_VERSION.to_string(),
        ledger_id: "audit-ledger-710".to_string(),
        entries,
        guardrails: vec![
            "audit ledger records attempts and never applies or rolls back".to_string(),
            "recorded history is append-only".to_string(),
        ],
    }
}

#[test]
fn ledger_records_applied_and_blocked_attempts() {
    let ledger = ledger(vec![
        entry("attempt-1", SourceApplyAuditApplyStatus::Applied),
        entry("attempt-2", SourceApplyAuditApplyStatus::Blocked),
        entry("attempt-3", SourceApplyAuditApplyStatus::VerificationFailed),
    ]);
    ledger.validate().expect("valid");
    let model = ledger.read_model();
    assert_eq!(model.entry_count, 3);
    assert_eq!(model.applied_count, 1);
    assert_eq!(model.blocked_count, 1);
    assert_eq!(model.failed_count, 1);
    assert!(model.append_only);
    assert!(model
        .forbidden_actions
        .iter()
        .any(|a| a == "rewrite_history"));
}

#[test]
fn duplicate_attempt_id_is_rejected() {
    let ledger = ledger(vec![
        entry("attempt-1", SourceApplyAuditApplyStatus::Applied),
        entry("attempt-1", SourceApplyAuditApplyStatus::ApplyFailed),
    ]);
    assert!(ledger.validate().is_err());
}

#[test]
fn append_entry_enforces_unique_attempt_id() {
    let base = ledger(vec![entry(
        "attempt-1",
        SourceApplyAuditApplyStatus::Applied,
    )]);
    let appended = base
        .append_entry(entry("attempt-2", SourceApplyAuditApplyStatus::Blocked))
        .expect("append succeeds");
    assert_eq!(appended.entries.len(), 2);
    assert!(base
        .append_entry(entry("attempt-1", SourceApplyAuditApplyStatus::ApplyFailed))
        .is_err());
}

#[test]
fn append_only_history_cannot_be_rewritten() {
    let base = ledger(vec![entry(
        "attempt-1",
        SourceApplyAuditApplyStatus::Applied,
    )]);
    let appended = base
        .append_entry(entry("attempt-2", SourceApplyAuditApplyStatus::Blocked))
        .expect("append");
    appended.validate_is_append_of(&base).expect("valid append");

    // Rewriting an existing entry is rejected.
    let mut rewritten = appended.clone();
    rewritten.entries[0].actor = "tampered".to_string();
    assert!(rewritten.validate_is_append_of(&base).is_err());

    // Truncation is rejected.
    let mut truncated = base.clone();
    truncated.entries.clear();
    assert!(truncated.validate_is_append_of(&base).is_err());
}

#[test]
fn blocked_entry_requires_blocked_reasons() {
    let mut bad = entry("attempt-1", SourceApplyAuditApplyStatus::Blocked);
    bad.blocked_reasons.clear();
    let ledger = ledger(vec![bad]);
    assert!(ledger.validate().is_err());
}

#[test]
fn missing_transaction_ref_is_rejected() {
    let mut bad = entry("attempt-1", SourceApplyAuditApplyStatus::Applied);
    bad.transaction_id = String::new();
    let ledger = ledger(vec![bad]);
    assert!(ledger.validate().is_err());
}

#[test]
fn stale_traversal_ref_is_rejected() {
    let mut bad = entry("attempt-1", SourceApplyAuditApplyStatus::Applied);
    bad.review_decision_ref = "../../etc/passwd".to_string();
    let ledger = ledger(vec![bad]);
    assert!(ledger.validate().is_err());
}

#[test]
fn malformed_status_json_is_rejected() {
    let malformed = r#"{
        "schemaVersion":"source-apply-audit-ledger-v1",
        "ledgerId":"audit-ledger-710",
        "entries":[{"attemptId":"a","transactionId":"t","actor":"x","recordedAt":"t",
            "reviewDecisionRef":"a","sandboxReportRef":"b","staleGuardRef":"c",
            "rollbackSnapshotRef":"d","verificationLogRef":"e","applyStatus":"exploded"}],
        "guardrails":["g"]
    }"#;
    assert!(SourceApplyAuditLedger::from_json_str(malformed).is_err());
}

#[test]
fn json_round_trip_preserves_ledger() {
    let ledger = ledger(vec![entry(
        "attempt-1",
        SourceApplyAuditApplyStatus::Applied,
    )]);
    let json = serde_json::to_string_pretty(&ledger).expect("serializes");
    let parsed = SourceApplyAuditLedger::from_json_str(&json).expect("parses");
    assert_eq!(parsed, ledger);
}

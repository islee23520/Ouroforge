use ouroforge_core::{
    SourceApplyRollbackSnapshot, SourceApplyRollbackStatus, SourceApplyRollbackTarget,
    SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION,
};

fn target(path: &str) -> SourceApplyRollbackTarget {
    SourceApplyRollbackTarget {
        path: path.to_string(),
        before_content_hash: format!("before-{path}"),
        expected_after_hash: format!("after-{path}"),
        before_content_ref: Some(format!("rollback/{path}.before")),
        reverse_patch_ref: None,
    }
}

fn complete_fixture() -> SourceApplyRollbackSnapshot {
    SourceApplyRollbackSnapshot {
        schema_version: SOURCE_APPLY_ROLLBACK_SNAPSHOT_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-706-1".to_string(),
        transaction_base_revision: "base-rev-706-1".to_string(),
        snapshot_base_revision: "base-rev-706-1".to_string(),
        actor: "agent-applier".to_string(),
        recorded_at: "2026-06-05T00:00:00Z".to_string(),
        targets: vec![
            target("examples/source-apply-v1/sample-a.rs"),
            target("examples/source-apply-v1/sample-b.rs"),
        ],
        recovery_guidance: vec![
            "copy rollback/<path>.before back over the target to restore before-state".to_string(),
        ],
        guardrails: vec![
            "rollback snapshot records metadata only and does not auto-restore".to_string(),
            "dashboard and Studio display this snapshot read-only".to_string(),
        ],
    }
}

#[test]
fn complete_snapshot_is_apply_ready() {
    let snapshot = complete_fixture();
    snapshot.validate().expect("structural validation passes");
    let evaluation = snapshot.evaluate();
    assert_eq!(
        evaluation.status,
        SourceApplyRollbackStatus::Complete,
        "gaps: {:?}",
        evaluation.recovery_gaps
    );
    assert!(snapshot.is_complete());
    for forbidden in [
        "apply_patch",
        "merge_branch",
        "execute_command",
        "auto_restore",
    ] {
        assert!(evaluation
            .forbidden_actions
            .iter()
            .any(|action| action == forbidden));
    }
}

#[test]
fn reverse_patch_ref_alone_is_sufficient() {
    let mut snapshot = complete_fixture();
    snapshot.targets[0].before_content_ref = None;
    snapshot.targets[0].reverse_patch_ref = Some("rollback/sample-a.reverse.patch".to_string());
    assert!(snapshot.is_complete());
}

#[test]
fn missing_reverse_data_blocks_readiness() {
    let mut snapshot = complete_fixture();
    snapshot.targets[0].before_content_ref = None;
    snapshot.targets[0].reverse_patch_ref = None;
    let evaluation = snapshot.evaluate();
    assert_eq!(evaluation.status, SourceApplyRollbackStatus::Blocked);
    assert!(evaluation
        .recovery_gaps
        .iter()
        .any(|gap| gap.contains("missing reverse data")));
}

#[test]
fn missing_before_hash_blocks_readiness() {
    let mut snapshot = complete_fixture();
    snapshot.targets[0].before_content_hash = "   ".to_string();
    let evaluation = snapshot.evaluate();
    assert_eq!(evaluation.status, SourceApplyRollbackStatus::Blocked);
    assert!(evaluation
        .recovery_gaps
        .iter()
        .any(|gap| gap.contains("before content hash")));
}

#[test]
fn stale_snapshot_blocks_readiness() {
    let mut snapshot = complete_fixture();
    snapshot.snapshot_base_revision = "base-rev-old".to_string();
    let evaluation = snapshot.evaluate();
    assert_eq!(evaluation.status, SourceApplyRollbackStatus::Blocked);
    assert!(evaluation
        .recovery_gaps
        .iter()
        .any(|gap| gap.contains("stale")));
}

#[test]
fn recovery_gap_when_guidance_missing() {
    let mut snapshot = complete_fixture();
    snapshot.recovery_guidance.clear();
    let evaluation = snapshot.evaluate();
    assert_eq!(evaluation.status, SourceApplyRollbackStatus::Blocked);
    assert!(evaluation
        .recovery_gaps
        .iter()
        .any(|gap| gap.contains("recovery gap")));
}

#[test]
fn empty_targets_fail_validation() {
    let mut snapshot = complete_fixture();
    snapshot.targets.clear();
    assert!(snapshot.validate().is_err());
}

#[test]
fn malformed_json_is_rejected() {
    let malformed =
        r#"{"schemaVersion":"source-apply-rollback-snapshot-v1","unexpectedField":true}"#;
    assert!(SourceApplyRollbackSnapshot::from_json_str(malformed).is_err());
}

#[test]
fn path_traversal_target_fails_validation() {
    let mut snapshot = complete_fixture();
    snapshot.targets[0].path = "../../etc/hosts".to_string();
    assert!(snapshot.validate().is_err());
}

#[test]
fn json_round_trip_preserves_artifact() {
    let snapshot = complete_fixture();
    let json = serde_json::to_string_pretty(&snapshot).expect("serializes");
    let parsed = SourceApplyRollbackSnapshot::from_json_str(&json).expect("parses and validates");
    assert_eq!(parsed, snapshot);
}

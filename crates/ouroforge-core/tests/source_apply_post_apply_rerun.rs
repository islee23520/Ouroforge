use ouroforge_core::{
    SourceApplyComparisonDimension, SourceApplyComparisonState, SourceApplyRerunComparison,
    SourceApplyRerunStatus, SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION,
};

fn dimension(name: &str, state: SourceApplyComparisonState) -> SourceApplyComparisonDimension {
    SourceApplyComparisonDimension {
        name: name.to_string(),
        state,
        detail: format!("{name} dimension detail"),
    }
}

fn completed_fixture(states: Vec<SourceApplyComparisonState>) -> SourceApplyRerunComparison {
    SourceApplyRerunComparison {
        schema_version: SOURCE_APPLY_RERUN_COMPARISON_SCHEMA_VERSION.to_string(),
        apply_transaction_id: "apply-txn-708-1".to_string(),
        review_decision_ref: "evidence/review-decision.json".to_string(),
        verification_log_ref: "runs/verification/log.json".to_string(),
        rollback_snapshot_ref: "evidence/rollback-snapshot.json".to_string(),
        audit_ledger_ref: "evidence/audit-ledger.json".to_string(),
        qa_backlog_refs: vec!["evidence/qa-backlog/item-1.json".to_string()],
        rerun_status: SourceApplyRerunStatus::Completed,
        before_run_id: Some("run-before-1".to_string()),
        after_run_id: Some("run-after-1".to_string()),
        generated_output_root: "runs/post-apply".to_string(),
        dimensions: states
            .into_iter()
            .enumerate()
            .map(|(i, state)| dimension(&format!("dimension-{i}"), state))
            .collect(),
        guardrails: vec![
            "rerun comparison evaluates evidence and never applies or runs commands".to_string(),
            "promotion claims require evidence".to_string(),
        ],
    }
}

#[test]
fn improved_comparison_allows_promotion_claim() {
    let comparison = completed_fixture(vec![
        SourceApplyComparisonState::Improved,
        SourceApplyComparisonState::Unchanged,
    ]);
    comparison.validate().expect("valid");
    let evaluation = comparison.evaluate();
    assert_eq!(
        evaluation.overall_state,
        SourceApplyComparisonState::Improved
    );
    assert!(evaluation.promotion_claim_allowed);
    assert!(comparison.promotion_claim_allowed());
}

#[test]
fn regression_blocks_promotion_claim() {
    let comparison = completed_fixture(vec![
        SourceApplyComparisonState::Improved,
        SourceApplyComparisonState::Regressed,
    ]);
    let evaluation = comparison.evaluate();
    assert_eq!(
        evaluation.overall_state,
        SourceApplyComparisonState::Regressed
    );
    assert!(!evaluation.promotion_claim_allowed);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("regression")));
}

#[test]
fn flaky_is_inconclusive_and_blocks_claim() {
    let comparison = completed_fixture(vec![
        SourceApplyComparisonState::Unchanged,
        SourceApplyComparisonState::Flaky,
    ]);
    let evaluation = comparison.evaluate();
    assert_eq!(evaluation.overall_state, SourceApplyComparisonState::Flaky);
    assert!(!evaluation.promotion_claim_allowed);
}

#[test]
fn unsupported_rerun_is_explicit_and_blocks_claim() {
    let mut comparison = completed_fixture(vec![]);
    comparison.rerun_status = SourceApplyRerunStatus::Unsupported;
    comparison.before_run_id = None;
    comparison.after_run_id = None;
    let evaluation = comparison.evaluate();
    assert_eq!(
        evaluation.overall_state,
        SourceApplyComparisonState::Unsupported
    );
    assert!(!evaluation.promotion_claim_allowed);
    assert!(evaluation
        .blocked_reasons
        .iter()
        .any(|r| r.contains("unsupported")));
}

#[test]
fn missing_rerun_is_explicit_and_blocks_claim() {
    let mut comparison = completed_fixture(vec![]);
    comparison.rerun_status = SourceApplyRerunStatus::Missing;
    comparison.before_run_id = None;
    comparison.after_run_id = None;
    let evaluation = comparison.evaluate();
    assert_eq!(
        evaluation.overall_state,
        SourceApplyComparisonState::MissingBefore
    );
    assert!(!evaluation.promotion_claim_allowed);
}

#[test]
fn unchanged_comparison_allows_promotion_claim() {
    let comparison = completed_fixture(vec![SourceApplyComparisonState::Unchanged]);
    let evaluation = comparison.evaluate();
    assert_eq!(
        evaluation.overall_state,
        SourceApplyComparisonState::Unchanged
    );
    assert!(evaluation.promotion_claim_allowed);
    assert!(evaluation
        .forbidden_actions
        .iter()
        .any(|a| a == "claim_promotion_without_evidence"));
}

#[test]
fn completed_rerun_requires_dimensions() {
    let comparison = completed_fixture(vec![]);
    assert!(comparison.validate().is_err());
}

#[test]
fn json_round_trip_preserves_artifact() {
    let comparison = completed_fixture(vec![SourceApplyComparisonState::Improved]);
    let json = serde_json::to_string_pretty(&comparison).expect("serializes");
    let parsed = SourceApplyRerunComparison::from_json_str(&json).expect("parses");
    assert_eq!(parsed, comparison);
}

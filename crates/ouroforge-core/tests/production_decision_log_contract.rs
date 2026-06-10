use ouroforge_core::production_decision_log::{
    ProductionDecisionLog, ProductionDecisionOutcome, ProductionDecisionRecord,
    PRODUCTION_DECISION_LOG_SCHEMA_VERSION,
};

fn record(id: &str, outcome: ProductionDecisionOutcome) -> ProductionDecisionRecord {
    ProductionDecisionRecord {
        decision_id: id.to_string(),
        journal_entry_ref: format!("production-journal:{id}"),
        proposal_ref: format!("runs/proposals/{id}.json"),
        outcome,
        agent_suggested: true,
        human_reviewer: "human-designer".to_string(),
        human_rationale: "human taste and evidence review".to_string(),
        evidence_refs: vec![format!("runs/decisions/{id}/evidence.json")],
        self_approved: false,
    }
}

fn log() -> ProductionDecisionLog {
    ProductionDecisionLog {
        schema_version: PRODUCTION_DECISION_LOG_SCHEMA_VERSION.to_string(),
        log_id: "signal-gate-decision-log".to_string(),
        production_journal_ref: "runs/production-journal/journal.json".to_string(),
        decisions: vec![
            record("accept-hud-copy", ProductionDecisionOutcome::Accepted),
            record("reject-auto-fun", ProductionDecisionOutcome::Rejected),
        ],
    }
}

#[test]
fn decision_log_traces_accepted_and_rejected_agent_proposals() {
    let log = log();
    log.validate().expect("valid decision log");
    assert!(log
        .decisions
        .iter()
        .any(|d| d.outcome == ProductionDecisionOutcome::Accepted));
    assert!(log
        .decisions
        .iter()
        .any(|d| d.outcome == ProductionDecisionOutcome::Rejected));
    assert!(log.decisions.iter().all(|d| !d.human_rationale.is_empty()));
}

#[test]
fn decision_log_rejects_self_approval_and_missing_rejections() {
    let mut bad = log();
    bad.decisions[0].self_approved = true;
    let error = bad
        .validate()
        .expect_err("self approval rejected")
        .to_string();
    assert!(error.contains("self-approved"), "{error}");

    let mut no_reject = log();
    no_reject
        .decisions
        .retain(|d| d.outcome != ProductionDecisionOutcome::Rejected);
    let error = no_reject
        .validate()
        .expect_err("rejection required")
        .to_string();
    assert!(error.contains("rejected proposal"), "{error}");
}

//! Scenario Coverage v110: M129 dogfood production log and playtest gap backlog (#2388-#2390).

use ouroforge_core::evolve_iteration_journal::EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION;
use ouroforge_core::playtest_gap_backlog::{
    PlaytestFinding, PlaytestFindingStatus, PlaytestGapBacklog, PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION,
};
use ouroforge_core::product_gap_taxonomy::{product_gap_category_ids, product_gap_severity_ids};
use ouroforge_core::production_decision_log::{
    ProductionDecisionLog, ProductionDecisionOutcome, ProductionDecisionRecord,
    PRODUCTION_DECISION_LOG_SCHEMA_VERSION,
};
use ouroforge_core::production_journal::{ProductionJournal, PRODUCTION_JOURNAL_SCHEMA_VERSION};
use std::collections::BTreeSet;

fn taxonomy() -> (BTreeSet<String>, BTreeSet<String>) {
    (
        product_gap_category_ids().unwrap().into_iter().collect(),
        product_gap_severity_ids().unwrap().into_iter().collect(),
    )
}

#[test]
fn v110_links_journal_decisions_and_playtest_backlog_without_hiding_gaps() {
    let journal_json = serde_json::json!({
        "schemaVersion": PRODUCTION_JOURNAL_SCHEMA_VERSION,
        "schemaFamily": EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION,
        "journalId": "signal-gate-production-journal-v110",
        "sourceEvolveJournalRef": "runs/evolve/journal.json",
        "entries": [{
            "entryId": "human-feel-v110",
            "kind": "human-fun-feel-decision",
            "summary": "record human pacing note",
            "rationale": "human playtest notes are evidence, not automated pass/fail",
            "issueRefs": ["#2390"],
            "prRefs": ["#v110"],
            "evidenceRefs": ["runs/playtest/v110/evidence.json"],
            "humanDecision": true,
            "unresolvedGap": false,
            "nextAction": "feed backlog proposal candidates"
        }],
        "guardrails": ["reuse M127 family; no second journal family"]
    });
    let journal: ProductionJournal = serde_json::from_value(journal_json).expect("journal json");
    journal.validate().expect("journal validates");

    let decision = ProductionDecisionRecord {
        decision_id: "accept-backlog-handoff".to_string(),
        journal_entry_ref: "production-journal:human-feel-v110".to_string(),
        proposal_ref: "runs/proposals/backlog-handoff.json".to_string(),
        outcome: ProductionDecisionOutcome::Accepted,
        agent_suggested: true,
        human_reviewer: "human-designer".to_string(),
        human_rationale: "accept backlog handoff while preserving deferred blocker visibility"
            .to_string(),
        evidence_refs: vec!["runs/playtest/v110/evidence.json".to_string()],
        self_approved: false,
    };
    let rejected = ProductionDecisionRecord {
        decision_id: "reject-auto-fun-pass".to_string(),
        outcome: ProductionDecisionOutcome::Rejected,
        ..decision.clone()
    };
    let decisions = ProductionDecisionLog {
        schema_version: PRODUCTION_DECISION_LOG_SCHEMA_VERSION.to_string(),
        log_id: "signal-gate-v110-decisions".to_string(),
        production_journal_ref: "runs/production-journal/journal.json".to_string(),
        decisions: vec![decision, rejected],
    };
    decisions.validate().expect("decisions validate");
    assert_eq!(decisions.read_model().rejected_count, 1);

    let (categories, severities) = taxonomy();
    let backlog = PlaytestGapBacklog {
        schema_version: PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION.to_string(),
        backlog_id: "signal-gate-v110-playtest-backlog".to_string(),
        production_journal_ref: "runs/production-journal/journal.json".to_string(),
        findings: vec![PlaytestFinding {
            finding_id: "pacing-blocker".to_string(),
            category: "dogfood_game_quality".to_string(),
            severity: "blocking".to_string(),
            status: PlaytestFindingStatus::Deferred,
            observation: "player wants a clearer second encounter pacing ramp".to_string(),
            human_fun_feel_note: Some("human note preserved for next milestone".to_string()),
            evidence_refs: vec!["runs/playtest/v110/pacing.json".to_string()],
            owner_issue: "#2391".to_string(),
            next_action: "create M130 usability proposal".to_string(),
            blocks_product_observed: true,
        }],
    };
    backlog
        .validate(&categories, &severities)
        .expect("backlog validates");
    let model = backlog.read_model();
    assert!(model.closure_allowed);
    assert_eq!(model.blocking_deferred_count, 1);
    assert_eq!(model.future_proposal_candidate_count, 1);
}

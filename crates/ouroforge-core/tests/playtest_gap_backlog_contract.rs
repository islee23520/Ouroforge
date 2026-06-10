use ouroforge_core::playtest_gap_backlog::{
    PlaytestFinding, PlaytestFindingStatus, PlaytestGapBacklog, PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION,
};
use ouroforge_core::product_gap_taxonomy::{product_gap_category_ids, product_gap_severity_ids};
use std::collections::BTreeSet;

fn taxonomy() -> (BTreeSet<String>, BTreeSet<String>) {
    (
        product_gap_category_ids().unwrap().into_iter().collect(),
        product_gap_severity_ids().unwrap().into_iter().collect(),
    )
}

fn finding(id: &str, status: PlaytestFindingStatus, blocks: bool) -> PlaytestFinding {
    PlaytestFinding {
        finding_id: id.to_string(),
        category: "dogfood_game_quality".to_string(),
        severity: if blocks { "blocking" } else { "polish" }.to_string(),
        status,
        observation: "player understood the objective but wanted clearer pacing".to_string(),
        human_fun_feel_note: Some("human note: promising but needs iteration".to_string()),
        evidence_refs: vec![format!("runs/playtest/{id}/evidence.json")],
        owner_issue: "#2390".to_string(),
        next_action: "create backlog candidate".to_string(),
        blocks_product_observed: blocks,
    }
}

fn backlog() -> PlaytestGapBacklog {
    PlaytestGapBacklog {
        schema_version: PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION.to_string(),
        backlog_id: "signal-gate-playtest-backlog".to_string(),
        production_journal_ref: "runs/production-journal/journal.json".to_string(),
        findings: vec![
            finding("feel-1", PlaytestFindingStatus::Deferred, true),
            finding("polish-1", PlaytestFindingStatus::Open, false),
        ],
    }
}

#[test]
fn playtest_findings_use_m117_taxonomy_and_allow_human_fun_feel_notes() {
    let (categories, severities) = taxonomy();
    let backlog = backlog();
    backlog
        .validate(&categories, &severities)
        .expect("valid backlog");
    assert!(backlog
        .findings
        .iter()
        .any(|finding| finding.human_fun_feel_note.is_some()));
}

#[test]
fn blocking_findings_must_be_deferred_before_product_observed_closure() {
    let (categories, severities) = taxonomy();
    let mut backlog = backlog();
    backlog.findings[0].status = PlaytestFindingStatus::Open;
    let error = backlog
        .validate(&categories, &severities)
        .expect_err("blocking open finding rejected")
        .to_string();
    assert!(error.contains("explicitly deferred"), "{error}");
}

#[test]
fn backlog_read_model_feeds_future_proposals_and_closure_checklist() {
    let (categories, severities) = taxonomy();
    let backlog = backlog();
    backlog
        .validate(&categories, &severities)
        .expect("valid backlog");
    let model = backlog.read_model();
    assert_eq!(model.finding_count, 2);
    assert_eq!(model.blocking_deferred_count, 1);
    assert_eq!(model.open_non_blocking_count, 1);
    assert_eq!(model.future_proposal_candidate_count, 2);
    assert!(model.closure_allowed);
}

#[test]
fn sample_dogfood_report_keeps_deferred_blockers_visible_for_handoff() {
    let (categories, severities) = taxonomy();
    let backlog = backlog();
    backlog
        .validate(&categories, &severities)
        .expect("valid backlog");
    let model = backlog.read_model();
    assert!(
        model.closure_allowed,
        "blocking issue is explicitly deferred, not hidden"
    );
    assert_eq!(model.future_proposal_candidate_count, 2);
    assert!(backlog
        .findings
        .iter()
        .any(|finding| finding.blocks_product_observed
            && finding.status == PlaytestFindingStatus::Deferred));
    assert!(backlog
        .findings
        .iter()
        .all(|finding| !finding.evidence_refs.is_empty()));
}

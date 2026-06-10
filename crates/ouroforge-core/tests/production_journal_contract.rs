use ouroforge_core::evolve_iteration_journal::EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION;
use ouroforge_core::production_journal::{
    ProductionJournal, ProductionJournalEntry, ProductionJournalEntryKind,
    PRODUCTION_JOURNAL_SCHEMA_VERSION,
};

fn entry(id: &str, kind: ProductionJournalEntryKind) -> ProductionJournalEntry {
    let human_decision = matches!(kind, ProductionJournalEntryKind::HumanFunFeelDecision);
    let unresolved_gap = matches!(kind, ProductionJournalEntryKind::UnresolvedGap);
    ProductionJournalEntry {
        entry_id: id.to_string(),
        kind,
        summary: "record dogfood production fact".to_string(),
        rationale: "keep evidence-linked production history".to_string(),
        issue_refs: vec!["#2388".to_string()],
        pr_refs: vec!["#sample".to_string()],
        evidence_refs: vec![format!("runs/production-journal/{id}/evidence.json")],
        proposal_ref: Some(format!("runs/production-journal/{id}/proposal.json")),
        review_ref: Some(format!("runs/production-journal/{id}/review.json")),
        applied_diff_ref: None,
        human_decision,
        unresolved_gap,
        next_action: "continue dogfood production loop".to_string(),
    }
}

fn journal() -> ProductionJournal {
    ProductionJournal {
        schema_version: PRODUCTION_JOURNAL_SCHEMA_VERSION.to_string(),
        schema_family: EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION.to_string(),
        journal_id: "signal-gate-production-journal".to_string(),
        source_evolve_journal_ref: "runs/evolve/journal.json".to_string(),
        entries: vec![
            entry("decision-1", ProductionJournalEntryKind::Decision),
            entry(
                "human-feel-1",
                ProductionJournalEntryKind::HumanFunFeelDecision,
            ),
            entry("gap-1", ProductionJournalEntryKind::UnresolvedGap),
        ],
        guardrails: vec!["reuse M127 family; no second journal family".to_string()],
    }
}

#[test]
fn production_journal_reuses_m127_family_and_validates_samples() {
    let journal = journal();
    journal.validate().expect("valid journal");
    assert_eq!(
        journal.schema_family,
        EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION
    );
    assert!(journal.entries.iter().any(|entry| entry.human_decision));
    assert!(journal.entries.iter().any(|entry| entry.unresolved_gap));
}

#[test]
fn production_journal_is_append_only_and_rejects_second_family() {
    let previous = journal();
    let mut next = previous.clone();
    next.entries
        .push(entry("applied-1", ProductionJournalEntryKind::AppliedDiff));
    next.validate_is_append_of(&previous).expect("append-only");

    let mut second_family = previous.clone();
    second_family.schema_family = "parallel-production-journal-v1".to_string();
    let error = second_family
        .validate()
        .expect_err("second family rejected")
        .to_string();
    assert!(error.contains("reuse M127"), "{error}");
}

#[test]
fn applied_diff_requires_review_and_human_fun_feel_is_human_marked() {
    let mut applied = entry("applied-bad", ProductionJournalEntryKind::AppliedDiff);
    applied.applied_diff_ref = Some("runs/production-journal/applied.diff".to_string());
    applied.review_ref = None;
    let error = applied.validate().expect_err("review required").to_string();
    assert!(error.contains("reviewRef"), "{error}");

    let mut human = entry(
        "human-bad",
        ProductionJournalEntryKind::HumanFunFeelDecision,
    );
    human.human_decision = false;
    let error = human
        .validate()
        .expect_err("human marker required")
        .to_string();
    assert!(error.contains("humanDecision"), "{error}");
}
